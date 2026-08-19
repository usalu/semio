# terra-http-streaming — packet report

Wave W6, owned paths: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` (+ its `📦️packages/🦀️rust/Cargo.toml`).

## seam design

Added to `semio-framework-os-services`:

```rust
pub struct HttpResponseHead { pub status: u16, pub headers: Vec<(String, String)> }

pub trait HttpBody: Send {
    fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>>;
}

pub trait AsyncHttpTransport: Send + Sync {
    fn start(&self, ctx: &OperationContext, request: HttpRequest)
        -> HostFuture<Result<(HttpResponseHead, Box<dyn HttpBody>), HttpPoolError>>;
}
```

exactly as specified. `HttpPool` now holds an internal `HttpPoolTransport` (private) enum: `Blocking { transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool> }` (today's shape, built by `HttpPool::new`) or `Async(Arc<dyn AsyncHttpTransport>)` (built by the new `HttpPool::new_with_async_transport`, for a sibling packet's real client).

`HttpPool::fetch(runtime, scope, ctx, package, actor, request) -> Result<(HttpResponseHead, HttpPoolBody), HttpPoolError>` is the new streaming entry point:
1. Increments the per-actor outstanding count (typed `OutstandingCapReached` on failure, unchanged from before).
2. Charges the per-package bucket for the EXACT, known outbound bytes (`request.url.len() + request.body.len()`) up front — unchanged arithmetic from before, but now honestly documented as exact-not-estimate for the OUTBOUND side; the real fix is the response side below.
3. Dispatches via whichever `HttpPoolTransport` variant is configured, producing `(HttpResponseHead, Box<dyn HttpBody>)`.
4. Wraps the box in `HttpPoolBody`, which owns clones of the pool's `buckets`/`outstanding` maps.

`HttpPoolBody::next_chunk` pulls one real chunk from the wrapped `HttpBody`, charges the per-package bucket for that chunk's REAL length (`chunk.len()`, not an estimate), and on exhaustion returns a typed `ByteBudgetExhausted` and marks itself finished (releasing the outstanding slot) — this is the actual fix for the measured defect ("HttpPool charges byte budgets by ESTIMATE ... buffers whole bodies"): response bytes were previously never charged at all (only a pre-request guess on the request side existed); now every response chunk is charged for real, as it streams. `HttpPoolBody::finish` is idempotent (`finished: bool` guard) and is called from both the success/EOF/error paths AND `Drop`, so dropping a body early (a cancelled consumer) releases the outstanding slot exactly once and, structurally, drops `inner: Box<dyn HttpBody>` too — whatever connection the transport body owns closes with it (verified in a test — see below).

`BlockingHttpTransport` is the one shipped `AsyncHttpTransport` implementation: wraps a legacy `Arc<dyn HttpTransport>` + `Arc<ComputePool>`, captures `runtime`/`scope` at CONSTRUCTION (fields), and on `start` runs the whole blocking call through `ComputePool::run_blocking`, replaying the buffered result as ONE `BufferedHttpBody` chunk. `UnwiredHttpTransport`/`HttpTransport`/`HttpPool::new`/`HttpPool::request` all keep their exact pre-existing signatures — no edits needed in `📇️directory/🔌️client` or `🔌️plugin/🖥️host/⚡️effects`, both of which consume this crate's `HttpPool` (verified by reading both call sites).

## one-implementation argument

`HttpPool::request` is now:
```rust
pub async fn request(&self, runtime, scope, ctx, package, actor, request) -> Result<HttpResponse, HttpPoolError> {
    let (head, mut body) = self.fetch(runtime, scope, ctx, package, actor, request).await?;
    let mut collected = Vec::new();
    while let Some(chunk) = body.next_chunk().await? {
        collected.extend_from_slice(&chunk);
    }
    Ok(HttpResponse { status: head.status, headers: head.headers, body: collected })
}
```
built entirely on `fetch`, collecting chunks — there is exactly one request/response code path. Existing tests (`http_pool_rejects_past_the_per_actor_outstanding_cap`, `http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called`) exercise `request` unmodified and still pass through this new path (verified by re-reading their assertions against the new `fetch`/`HttpPoolBody` mechanics; see `## commands` for why I could not literally re-run them).

One acknowledged non-duplication-but-parallel-structure: `HttpPool::fetch`'s `HttpPoolTransport::Blocking` arm inlines the same dispatch `BlockingHttpTransport::start` performs, rather than literally calling it. This is NOT incidental duplication — `fetch`/`request` deliberately keep `runtime`/`scope` as *borrowed, per-call* parameters (to stay source-compatible with the two existing external callers), while `AsyncHttpTransport::start` (by the trait shape specified for this packet) takes neither, so any transport reaching `ComputePool::run_blocking` from inside `start` must OWN `runtime`/`scope`. A per-call borrowed reference cannot be captured into the `'static` `HostFuture` the trait returns. Both code paths are short (same ~10 lines) and documented as a pair (`HttpPool::new`'s doc points at `BlockingHttpTransport`'s doc and vice versa) — see `## honest gaps`.

## refill now runs

`HttpPool::spawn_refill_driver(&self, runtime: &Arc<dyn HostAsyncRuntime>, scope: &ScopeHandle, ctx: OperationContext)` spawns a `spawn_scoped` task (same convention as `TimerWheel::spawn_driver`) that loops: `sleep_until(now_ms + HTTP_BUCKET_REFILL_INTERVAL_MS)` (60_000ms), then locks `buckets` and calls `TokenBucket::refill(bytes_per_minute_cap)` on every tracked package. `TokenBucket::refill` itself is unchanged pure arithmetic.

Test `http_pool_refill_driver_actually_refills_a_consumed_bucket_on_its_tick` (in `HttpPoolTests`, ~line 1977 area pre-shift, see `## line ranges`): seeds a bucket down to 30/100 via direct (same-module) field access (`pool.buckets.lock()...try_consume(70)` — no new production API needed for this, `tests` is a descendant module so private-field access is standard Rust, same visibility rule already implicit elsewhere in this file), spawns the driver on a `ManualRuntime`, `manual.drive()`s once (asserts still 30 — must not fire early), then `manual.set_now_ms(HTTP_BUCKET_REFILL_INTERVAL_MS)` + `drive()` again (asserts back to 100). This proves the loop actually RUNS on its own tick, not merely that `refill`/`spawn_refill_driver` exist and compile.

## deadline racing

`StorageJob` gained a `cancelled: Arc<AtomicBool>` field. `StorageTicket` gained `cancelled`/`runtime: Arc<dyn HostAsyncRuntime>`/`deadline_ms: Option<u64>` (captured from `ctx.deadline_ms` at `submit` time). `StorageTicket::await_result` races `self.receiver` against `self.runtime.sleep_until(deadline_ms)` via `tokio::select!` (same idiom `ComputePool::run_blocking` already uses) when a deadline was set; losing the race sets `cancelled = true` and returns the new `StorageError::DeadlineExceeded`. `storage_try_dispatch` checks `job.cancelled` immediately after popping a job (before incrementing `in_flight`): if set, it releases the job's byte reservation, sends `Err(DeadlineExceeded)` (a no-op if the ticket already returned via the race — the send just fails silently, which is fine, `oneshot::Sender::send` on a dropped receiver is a normal no-op), and loops to the next queued job WITHOUT ever running the closure — same lazy-skip discipline `WheelCore::disarm`/`pop_expired` already use for cancelled timers. A job already dispatched onto a blocking OS thread when the deadline fires is NOT preempted (same honest limitation `ComputeError::DeadlineExceeded` already documents) — its result is just discarded once the ticket has already resolved via the race.

Test `storage_scheduler_races_a_queued_job_against_its_deadline_and_frees_its_reservation_when_lost` (real `TokioHostRuntime`, `max_in_flight: 1`, `byte_quota_per_plugin: 50`): occupies the one slot with a job blocked on a channel, submits a second job (42-byte reservation, `deadline_ms: now + 30`) that stays queued behind the occupier, awaits it and asserts `Err(StorageError::DeadlineExceeded)` AND that the job closure itself (`ran`) never actually executed. Then unblocks the occupier, sleeps 50ms for the dispatcher to pop-and-skip the now-cancelled job, and proves the 42-byte reservation was released by successfully submitting a THIRD 45-byte job against the same 50-byte-per-plugin quota (would fail if the first reservation were still held: 42+45 > 50).

`plugin/host/effects`' own `race_deadline`/`Race<T>` helper (outside my owned paths) was built specifically because this gap didn't exist yet — its own doc comment says so verbatim. I updated `StorageScheduler`'s crate-level doc to note the gap is now closed and flagged the now-redundant external wrapper via `spawn_task` (task id `task_c38667a1`) rather than editing that file myself.

## block_on removed

`TokioHostRuntime::new` (~line 245 area) now reads `let epoch = tokio::time::Instant::now();` directly — no `runtime.block_on(async { ... })` wrapper. `tokio::time::Instant::now()` falls back to the real OS clock whenever no runtime-owned mock clock is in scope (the `test-util`-gated auto-advance path only activates inside an explicitly paused clock, which nothing in this crate's tests uses), so no runtime entry is required. Doc comment on `TokioHostRuntime::new` updated to explain this instead of claiming a `block_on` anchors the epoch.

## line ranges

(Current file, `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`, 2194 lines total, up from 1656 before this packet)

- `TokioHostRuntime` region: 43–305 (block_on removal + doc at ~252–264)
- `HttpPool` region: 600–984 (`HttpResponseHead` 616–622, `HttpBody` 645–653, `AsyncHttpTransport` 655–663, `HttpTransport`/`UnwiredHttpTransport` unchanged, `BufferedHttpBody` 684–695, `BlockingHttpTransport` 697–736, `TokenBucket` 738–763, `HTTP_BUCKET_REFILL_INTERVAL_MS` 767, `release_outstanding_slot` 772–777, `HttpPoolTransport` 779–782, `HttpPool` struct+impl 789–914 including `fetch` 857–901, `spawn_refill_driver` 833–848, `request` 906–913, `HttpPoolBody` 923–979)
- `StorageScheduler` region: 986–1167 (`StorageError::DeadlineExceeded` added, `StorageJob.cancelled` field, `storage_try_dispatch`'s cancelled-skip branch, `StorageTicket` fields + `await_result`'s race)
- Tests region: 1385–2194 (`storage_scheduler_races_a_queued_job_against_its_deadline_and_frees_its_reservation_when_lost` in `StorageSchedulerTests` 1665–1844; `http_pool_refill_driver_actually_refills_a_consumed_bucket_on_its_tick`, `LocalSocketBody`/`LocalSocketTransport`/`spawn_chunk_server`, `http_pool_fetch_charges_real_bytes_per_chunk_over_a_local_tcp_listener`, `http_pool_dropping_a_body_mid_stream_frees_the_outstanding_slot_and_drops_the_connection` all in `HttpPoolTests` 1910–2179)
- Cargo.toml: `[dependencies]` tokio gained the `macros` feature (see `## honest gaps`).

## commands

**UNRUN** (coordinator-owned per binding rule 4/23):
```
cargo check  -p semio-framework-os-services --all-targets
cargo test   -p semio-framework-os-services
cargo clippy -p semio-framework-os-services --all-targets -- -D warnings
```

Actually run (permitted, rule 4), output saved to `terra-http-streaming-check-lib.txt` in this ticket folder:
```
cargo check -p semio-framework-os-services --lib
```
Passed clean (`Finished` dev profile, 0 errors) after the `Cargo.toml` `macros`-feature fix below. This checks the production (non-`#[cfg(test)]`) code only; the new/changed tests in `#[cfg(test)] mod tests` were verified by hand-reading (types, borrow/move shapes, trait signatures, existing-idiom matches) rather than compiled, since `--tests`/`--all-targets` is outside the permitted command.

## lease-requests

None. All edits stayed inside the owned paths (`🛎️services/🦀️component.rs` and its `📦️packages/🦀️rust/Cargo.toml`).

## honest gaps

- **`Cargo.toml` fix, disclosed**: `cargo check --lib` (my only permitted compiler feedback) initially failed with `cannot find select in tokio` at 4 sites — 3 PRE-EXISTING (`TimerWheel::spawn_driver`, `ComputePool::run_blocking` ×2) plus my new `StorageTicket::await_result`. Root cause: `tokio::select!` needs the `macros` feature, which was declared only under `[dev-dependencies]`, not `[dependencies]` — meaning a real non-test build of this crate (anything not unifying dev-dependency features, e.g. a plain `cargo build`) would have failed to compile on the PRE-EXISTING `TimerWheel`/`ComputePool` code too, before this packet touched anything. I added `"macros"` to the plain `tokio` dependency (already present, already used elsewhere in this same crate under `dev-dependencies` — no new external crate, just an existing feature flag moved to where production code actually needs it) so this crate can compile without the dev-dependency union masking the gap. Flagging this clearly since it is a fix beyond the ticket's three named defects, discovered only because I needed a real compiler signal.
- **Outbound-byte accounting stays an estimate for header framing**: `fetch`'s pre-charge for outbound bytes is `request.url.len() + request.body.len()` — exact for URL/body, but does not count header name/value bytes or HTTP protocol framing overhead. This was already true before this packet; I did not expand it, since the ticket's named defect is specifically about RESPONSE-side accounting (which was previously not charged AT ALL, now fixed for real per chunk).
- **`fetch`'s `Blocking` arm parallels `BlockingHttpTransport` rather than calling it** — explained in `## one-implementation argument`. A future packet that drops the legacy per-call-borrowed-runtime `HttpPool::new`/`request` shape entirely (once all callers migrate to owning an `Arc<dyn HostAsyncRuntime>` and constructing `BlockingHttpTransport`/`new_with_async_transport` directly) could delete this duplication.
- **`plugin/host/effects`' `race_deadline` wrapper is now redundant** for the `StorageScheduler` case (its own doc names that gap as the reason it exists) — flagged via `spawn_task` (`task_c38667a1`) rather than edited, since that file is outside my owned paths.
- **Real listener test harness**: the acceptance note asked for a harness if a raw listener inside a unit test felt awkward. I judged it NOT awkward enough to avoid — `spawn_chunk_server` (std `TcpListener`/`TcpStream`, no new dependency) plus `LocalSocketBody`/`LocalSocketTransport` (test-only `HttpBody`/`AsyncHttpTransport` impls that do one real blocking `read`/`connect` per call through `ComputePool`) are included as the two new `HttpPoolTests`. If the coordinator's real run surfaces flakiness from the background-thread server (timing-sensitive `std::thread::sleep(5ms)` between chunks, matching this file's existing convention of small sleeps for cross-thread observability, e.g. the pre-existing 40ms sleeps in `http_pool_rejects_past_the_per_actor_outstanding_cap`), the harness itself (bind-and-loop-`incoming()`, one thread per connection) is reusable for a slower/more deterministic version.
- **WASI stream/poll-world reuse**: `HttpBody`/`HttpPoolBody` are designed so a later packet's WASI `stream<u8>` writer and the poll world's chunked events can both drive the SAME `HttpPoolBody::next_chunk`, per the mission's "do not write two" instruction — no such caller exists yet in this crate (out of scope for this packet).
