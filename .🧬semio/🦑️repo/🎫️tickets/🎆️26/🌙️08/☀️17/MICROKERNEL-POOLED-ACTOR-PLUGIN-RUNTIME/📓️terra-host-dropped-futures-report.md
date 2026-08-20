# 📓️ `host-dropped-futures` report — 37 dropped futures in `semio-framework-plugin-host`, all fixed

Packet: `host-dropped-futures`. Path scope: `🔌️plugin/🖥️host/**`.

## Result

Forced-rebuild (`cargo clean -p semio-framework-plugin-host` then fresh `cargo check`) count of the
robust `unused implementer of` pattern in `semio-framework-plugin-host`: **0** (was 37). The instrument
was proven trustworthy on this exact crate/pattern before the fix (37 real positives at the first
forced-rebuild census) and after (0), satisfying R12's "verify the instrument can see a known-positive
before trusting a negative" on both ends, not just the final zero.

| check | result |
|---|---:|
| Forced-rebuild dropped-future census, `-p semio-framework-plugin-host --lib` | **0** (was 37) |
| `cargo check -p semio-framework-plugin-host --lib` | **EXIT 0** (was EXIT 0 before — not regressed) |
| `cargo test -p semio-framework-plugin-host --lib` | **EXIT 101, 919 errors**, all `#[cfg(test)]` residue — see "Test coverage" below |
| `cargo check -p semio-framework-plugin-host --all-targets` | **EXIT 101, 919 errors**, same residue, honestly reported |
| `cargo check -p semio-framework-plugin --lib` | **EXIT 0** |
| `cargo check -p semio-framework-plugin --lib --all-features` | **EXIT 0** |
| `cargo check -p semio-framework-async --lib` | **EXIT 0** |
| `cargo check -p semio-framework-os-kernel-db --lib` | **EXIT 0** |
| `cargo check -p semio-framework-os-kernel --lib` | **EXIT 0** |
| `cargo test -p semio-framework-os-kernel --lib` | **779 passed / 0 failed / 0 ignored** — matches recorded baseline exactly |

All numbers above run by me this session, in the foreground, `CARGO_TARGET_DIR` pointed at the session
scratchpad (`.../scratchpad/target-host`), pasted verbatim from actual command output.

## Census mechanics (R12)

Used `cargo check -p semio-framework-plugin-host --lib --message-format=json` after `cargo clean -p
semio-framework-plugin-host`, and grepped the JSON `message.message` field for the substring `unused
implementer of` (not the narrower `unused implementer of \`std::future::Future\`` the ticket's own R12
text quotes — this rustc/edition renders the type as `` `Future` `` here, matching db-dedyn's identical
finding). This surfaced exactly 37 primary spans, matching the brief's per-file breakdown exactly:
28 in `⚡️effects/🦀️component.rs`, 3 in `🧵️shard/🦀️component.rs`, 3 in `⏳️imports.rs`, 2 in
`🦀️component.rs` (lines 2147/2167 as named), 1 in `🧵️shard/🏃️executor.rs`.

## Pattern groups and treatment

All 37 sites' enclosing function was already `async fn` (or, for group F/J/K/L, the call chain crossed
into a genuinely sync context) — details per group below.

| group | file | sites | shape | fix |
|---|---|---:|---|---|
| A | effects | 1 (:153) | `CapabilityRevocationRegistry::revoke`'s `for token in tokens { token.cancel(); }` | plain `.await` |
| B | effects | 1 (:281) | `ensure_subscribed`'s `self.events.subscribe(...)` | plain `.await` |
| C | effects | 4 (:674,682,684,690) | `suspend`/`resume`/`revoke_capability` lifecycle calls (`park`/`unpark`/`flush`/`capabilities.revoke`) | plain `.await` |
| D | effects | 18 (:737,742,747,752,757,761,765,769,773,778,783,788,793,798,803,808,813,818) | `execute()`'s per-`Effect`-variant dispatch: outer calls to `dispatch_http`/`dispatch_storage`(×3)/`dispatch_set_timer`/`dispatch_publish_event`/`dispatch_send_message`/`events.subscribe`/`events.unsubscribe`/`dispatch_router_effect`(×9) | plain `.await` |
| E | effects | 3 (:846,866,976) | `emit_completed_err(...)` inside the 3 boxed `async move` tasks' `is_cancelled` early-return branch | plain `.await` |
| F | effects | 1 (:916) | `dispatch_set_timer`'s `wheel.disarm(timer_id)` inside its boxed (`HostFuture<()>`, `Send`-required) task | **`resolve_ready` E5 bridge**, not plain `.await` — see "The one non-trivial fix" below |
| G | shard/component.rs | 1 (:84) | `ShardFrame::pack_encode`'s `semio_framework_actor::pack::write_u8(out, self.tag().await)` | plain `.await` |
| H | shard/component.rs | 1 (:267) | `ShardLoop::unregister`'s `self.runtime.drop_instance(instance)` | plain `.await` |
| I | shard/component.rs | 1 (:522) | `Payload::Cancel` handler's `self.unregister(ActorId(actor_id))` | plain `.await` |
| J | shard/executor.rs | 1 (:200) | `impl Drop for ShardExecutor`'s `self.stop()` | **R9: `stop()` reverted to sync** — see below |
| K | imports.rs | 1 (:208) | `impl Drop for CancelOnDrop`'s `self.token.cancel()` | **E5 `block_on` bridge** |
| L | imports.rs | 1 (:232) | `DirectAwaitCapabilityRegistry::revoke`'s `token.cancel()` | **E5 `block_on` bridge** |
| M | imports.rs | 1 (:760) | `http_fetch`'s spawned chunk-pull task's `wake_chunk_shared(&shared_for_task)` | plain `.await` |
| N | component.rs | 2 (:2147,2167) | `walk_io_routes` self-recursive DFS — the recursive call (2147) and `resolve_io_route`'s outer call (2167) | recursive: `Box::pin(...).await`; outer: plain `.await` |
| **total** | | **37** | | |

(1+1+4+18+3+1 = 28 effects; 1+1+1 = 3 shard/component.rs; 1 shard/executor.rs; 1+1+1 = 3 imports.rs;
2 component.rs — matches the 37/28/3/3/2/1 split named in the packet brief exactly.)

## The one non-trivial fix — group F, `wheel.disarm(timer_id)` (E5, not a plain await)

A plain `.await` here does not compile: `dispatch_set_timer`'s timer-firing loop lives inside `Box::pin(async
move { ... })` cast to `HostFuture<()> = Pin<Box<dyn Future<Output=()> + Send>>` (the erased spawn channel
R1/R3 sanction). `TimerWheel::disarm` (`🛎️services/🦀️component.rs:506-508`) does
`self.core.lock().expect(...).disarm(id).await` — holding a `std::sync::MutexGuard<WheelCore>` (not `Send`)
across its OWN internal `.await`. Confirmed by compiling it: `E0277: MutexGuard<WheelCore> cannot be sent
between threads safely`.

That same `impl WheelCore` block already carries an R9 tag on its OWN sibling methods `pop_expired`/
`next_expiry_ms` for the identical reason ("held behind a `std::sync::Mutex` across an async caller ...
`async fn` here would force that `MutexGuard` ... to live across the outer future's await points, breaking
the `HostFuture<()>: Send` bound R3 requires. See R9.") — but `arm`/`disarm`/`armed_count` were missed by
whichever prior packet applied that fix. **This is a real bug in `🛎️services`, outside this packet's
`🔌️plugin/🖥️host` path_scope to fix at the root** (flagged below, not edited).

Fix within scope: added a local `resolve_ready<F: Future>(fut: F) -> F::Output` helper to
`⚡️effects/🦀️component.rs` (mirrors the identical existing private copies in `🛎️services`/`🚪️io`/
`🕸️graph/🗣️dsl` — no shared home for it in `⏳️async` today), and call `resolve_ready(wheel.disarm(timer_id))`
instead of `.await`. Sound because `WheelCore::disarm`'s own body (`entries.get_mut`/`cancelled = true`/a
saturating counter decrement) has zero suspension points — the whole `TimerWheel::disarm` future always
resolves on its first poll, so `resolve_ready` never hits its `unreachable!()` panic branch. Polling
synchronously (rather than `.await`ing) means the non-`Send` `MutexGuard` inside `disarm`'s state never
becomes part of the OUTER `Box::pin` block's own generated (Send-required) state — it lives and dies within
one execution step, exactly like the ORIGINAL un-awaited bug did, except this time the operation actually
runs.

**Reported, not fixed — needs its own packet or a `🛎️services` owner**: `TimerWheel::arm`/`disarm`/
`armed_count` (`🛎️services/🦀️component.rs:494-511`) should get the same R9 treatment their own sibling
methods already have, so callers outside a `Send`-boxed context don't need a `resolve_ready` bridge either.

## R9 fix — group J, `ShardExecutor::stop()` reverted to sync

`impl Drop for ShardExecutor { fn drop(&mut self) { self.stop(); } }` — `Drop::drop` is E1 (external trait,
language-fixed sync), so it cannot `.await`. Unlike group F, this one IS in my path_scope: `stop()`'s own
body (`self.stop.store(true, ...)` + `let _ = handle.join()`) has **zero suspension points** — no `.await`
anywhere inside it, before or after this fix. Grepped every call site repo-wide (`grep -rn "\.stop(\|fn
stop("`): exactly 3, all bare (no `.await`) — the production `Drop::drop` here, plus 2 in `#[cfg(test)]`.
No external crate calls it. Per R9's decision procedure (pure, zero suspension, one consumer language-barred
by E1): reverted `pub async fn stop(&mut self)` → `pub fn stop(&mut self)`, tagged. This is the cleaner fix
than a `block_on` bridge precisely because there is nothing to bridge — the `async` keyword bought nothing.

## E5 bridges — groups K and L, `imports.rs`

Both are `Drop`/sync-constrained contexts calling `CancelToken::cancel()` — which lives in `⏳️async`
(outside this packet's `🔌️plugin/🖥️host` path_scope), so unlike group J it cannot be reverted to sync from
here even though `CancelToken::cancel`'s own body (`self.0.local.store(...)`, confirmed by reading
`⏳️async/🦀️component.rs:147-149`) is equally suspension-free. This crate already has the exact same
E5-bridge shape established and tagged at `🧵️shard/🚚️process-transport/🦀️component.rs:196-206`
(`impl Drop for ProcessTransport`, `block_on(ShardTransport::kill(self))`) and at `⏳️imports.rs:563-564`
(`impl wit_host_async::Host`'s `fn emit`) — both fixes here (`CancelOnDrop::drop`, group K; and
`DirectAwaitCapabilityRegistry::revoke`, group L, whose own sync signature was already an R9 decision from a
prior packet that this fix does not relitigate) use `semio_framework_async::block_on(token.cancel())`,
matching that established idiom, tagged `// 🚫️async: E5 executor bridge` at each site.

## R13/R14 `let _ =` corollary — grepped every host file, found 2 real bugs

Grepped `find "🔌️plugin/🖥️host" -name "*.rs" | xargs grep -n "let _ = " | grep -v '\.await'` (13 candidate
lines). 11 were genuinely non-future: `std::fs`/`JoinHandle::join`/`std::process::Child::kill`/`::wait`/
`mpsc::Sender::send`/`tokio::sync::oneshot::Sender::send` (all sync std/tokio APIs, confirmed by reading each
signature) or plain variable-suppression (`let _ = error;`, `let _ = topic;`). 2 were test-only
(`#[cfg(test)]`, `probe.take_outbound()` at `shard/component.rs:1239,1275` — drained with no assertion
depending on it, left as-is, same class the SDK report already documented as benign). **2 were genuine
dropped futures, both fixed:**

| site | function | what was silently missing |
|---|---|---|
| `🦀️component.rs:916`, `WasmtimeRuntime::compile` | `let _ = store_compiled_component(&component, &cache_path);` | The on-disk compilation-cache write for a freshly-compiled wasm component never ran. `compile` checks `load_compiled_component` for a cache hit at the top, then (on a miss) was SUPPOSED to write the freshly-compiled bytes back via `store_compiled_component` so the NEXT `compile` of the same package hits the cache — but the write's future was dropped, so every single `compile` call was silently recompiling from wasm bytes from scratch, forever, regardless of `load_compiled_component`'s cache-hit path existing at all. Fixed to `let _ = store_compiled_component(...).await;` — the `Result` still discards (best-effort cache write, must not fail `compile` itself), only the future is now actually driven. |
| `⚡️effects/🦀️component.rs:941`, `dispatch_send_message` (`MessageEndpoint::Backbone` arm) | `let _ = self.backbone.send(dispatch.actor, uri, payload);` | `Effect::SendMessage` to a `Backbone` target never actually reached `BackboneRegistry::send` — the capability check (`messaging.backbone:<uri>`), endpoint lookup, and transport dispatch never ran. This module's own doc comment describes `BackboneRegistry` as "replaces the deleted PROCESS-GLOBAL `set_host_backbone_channel`, which left guest↔store sync with NO path at all" — this dropped future left it EXACTLY as broken, just via a different mechanism (silently-dropped future instead of a missing implementation). Fixed to `.send(...).await` — `Result` still discards (no `RequestId` on this effect variant to answer with a completion). |

## Per-site behaviour table — what was actually missing (groups A-N)

| group | function(s) | behaviour that was silently missing |
|---|---|---|
| A | `CapabilityRevocationRegistry::revoke` | Even where the OUTER `revoke_capability` call (group C) ran, each individual child `CancelToken` registered under the revoked capability never actually transitioned to `Cancelled` — so an in-flight operation holding one of those tokens would keep running as if never revoked; `token.cancel()`'s own body (a bare atomic store) never executed. |
| B | `ensure_subscribed` | An actor's `__effect_completions__:<actor>` topic was never actually registered with `EventRouter::subscribe`. Because the guard (`subscribed.insert(actor)`) is a plain `HashSet` checked BEFORE the dropped call, `ensure_subscribed` believed it had subscribed (would skip re-subscribing on every later call) even though the real registration never happened — a permanent, silent loss of that actor's completion delivery path from the very first `complete()`/`flush()` call. |
| C | `suspend`/`resume`/`revoke_capability` | `suspend`: `scope.cancel.park()` dropped — the actor's `CancelToken` never entered `Park`, so the documented "in-flight ops complete, but future completions buffer instead of deliver" contract never engaged; the actor kept behaving as fully `Live`. `resume`: BOTH `unpark()` (leave Park) and `sink.flush(actor)` (deliver buffered completions) dropped — resume did nothing at all. `revoke_capability`: `self.capabilities.revoke(capability)` dropped — `CapabilityRevocationRegistry::revoke`'s whole lookup-and-cancel body (group A) never ran in the first place; this is a "double drop" — both the outer AND inner call needed fixing for revocation to work end-to-end. |
| D | `execute()`'s per-effect dispatch | **The entire effect-dispatch path was a no-op, for every effect kind.** `dispatch_http`/`dispatch_storage`(read/write/delete)/`dispatch_set_timer`/`dispatch_router_effect`(blob-write/blob-load/document-read/document-write/io-compose/cache-derive/cache-read/invoke-extension/dispatch-action) each build a boxed task and call `spawn_scoped` INSIDE their own body — but since the wrapper fn itself was never awaited, that body (spawn included) never ran, so NOTHING was ever actually dispatched to `HttpPool`/`StorageScheduler`/`ComputePool`. `dispatch_publish_event`/`dispatch_send_message` similarly never ran their own internal `EventRouter::publish`/backbone-routing logic. `events.subscribe`/`events.unsubscribe` (the direct, non-wrapped `Effect::Subscribe`/`Unsubscribe` calls) never registered/removed the actor's subscription either. This is the host-side mirror of the SDK packet's group D (`HostAdapter::emit`) — here it's the INBOUND dispatch side: every effect a guest plugin emits would have silently vanished at the host's own dispatch loop, regardless of what the SDK side does. |
| E | `emit_completed_err` (×3, cancelled-before-dispatch branches) | When an operation's `CancelToken` was ALREADY cancelled by the time its boxed task started running (revoked capability, or actor trapped between dispatch and task start), the task was supposed to short-circuit and emit a `capability-revoked` error completion so the guest gets a definite answer. With the future dropped, the task still `return`ed early (correct), but the guest's original request got NO completion at all — a silently hung request, worse than a slow one. |
| F | `dispatch_set_timer`'s `wheel.disarm` | After a timer's loop ends (guest cancelled it, or it fired its last non-repeating shot), the per-plugin quota slot `TimerWheel::arm` reserved for it was never released. Every timer that ever finished (not just ones explicitly cancelled) permanently held its quota slot — a slow, unbounded quota leak that would eventually make `SetTimer` start rejecting a plugin's brand-new, unrelated timer requests with `QuotaExceeded`. |
| G | `ShardFrame::pack_encode` | The `write_u8` call that writes this frame's own type-tag BYTE never executed — every encoded `ShardFrame` (used by the thread AND process shard transports, `design-runtime.md §2`'s "thread-or-process, same wire" promise) would have been missing its leading tag byte, corrupting every frame on the wire and desyncing `pack_decode` on the far end. |
| H | `ShardLoop::unregister`'s `drop_instance` | Releasing an actor's `GuestInstance` (on generation-change restart, or a real unload) never actually told the runtime to reclaim the instance's pooling-allocator slab — a resource leak on every unregister, silent because `unregister`'s OTHER bookkeeping (`running_jobs.retain`/`job_placement.retain`/`pending_completions.remove`) still ran and looked complete. |
| I | `Payload::Cancel` handler's `unregister` | **The most severe of the shard-file findings.** On an actor-level `Cancel` (cancel every running job, then drop the instance outright), the actual `unregister` call — which removes the actor from `self.instances`, releases its allocator slab (group H), and clears its running-job/placement/pending-completion bookkeeping — never ran. The handler then unconditionally sent `ShardOutcome::Cancelled { actor }`, claiming success, while the actor instance stayed fully registered, running, and reachable exactly as before the Cancel — a false-success response to what looked like a working cancellation. |
| J | `ShardExecutor::stop()` (via `Drop`) | A `ShardExecutor` going out of scope was documented to signal-and-join its dedicated OS thread so it "never leaks a running thread" — with the `Drop`-invoked call dropped, that never happened: the stop flag was never set, the thread was never joined, and the doc's own stated guarantee was silently false for every `ShardExecutor` drop in production. |
| K | `CancelOnDrop::drop` | `CancelOnDrop`'s entire documented purpose ("generalises S2's proven shape... the guest cancels the awaiting subtask and wasmtime drops THIS future mid-poll without ever reaching the tail `guard.disarm()`") — propagating a genuine mid-poll cancellation into the child `CancelToken` — silently never fired. Every direct-await import call cancelled mid-flight (not via normal completion) left its child token in whatever state it was already in, never `Cancelled`. |
| L | `DirectAwaitCapabilityRegistry::revoke` | The direct-await-import counterpart to group A: cancelling a capability's tracked tokens for calls made through `⏳️imports.rs`'s "24 async imports" path never actually cancelled anything — same silent no-op as group A, on a parallel registry. |
| M | `wake_chunk_shared` | After pushing a newly-arrived HTTP body chunk (or marking the stream done) into `ChunkShared`, the registered `Waker` (recorded by `ChunkStreamProducer::poll_produce` the last time it returned `Pending`) was never actually invoked. The guest's stream reader would never be re-polled on new data arriving — this is "the one genuinely CHUNKED import" the module's own doc calls out as fixing the WIT doc's "only keeps the FINAL chunk" gap; with this dropped, streaming HTTP reads could stall indefinitely waiting on a wake that never came. |
| N | `walk_io_routes` / `resolve_io_route` | **The entire cross-plugin IO route-resolution algorithm never ran.** `walk_io_routes`'s breadth-bounded, cycle-free DFS is what populates `candidates`; with BOTH the recursive call (this fn calling itself one hop deeper) and `resolve_io_route`'s own outer call to it dropped, `candidates` stayed permanently empty — so `resolve_io_route` (the host's twin of `io::io_mechanism::resolve_route`, absorbing the ≤3-hop cycle-free route-resolution semantics `📌️important.md`'s peer-ticket note describes as load-bearing for cross-plugin artifact conversion) would unconditionally return `Err("no io route from X to Y within N hops")` for EVERY request, regardless of what routes were actually registered in the merged multi-plugin graph. |

## Test coverage — did any of the 37 sit under a passing test?

**No.** `cargo test -p semio-framework-plugin-host --lib` does not compile: **EXIT 101, 919 errors**,
matching the packet brief's own prediction ("expect ~919 `#[cfg(test)]` residue") almost exactly. Root
cause, confirmed by reading the actual errors (not assumed): every `#[test]` in this crate is written as
`#[test] async fn ...` — but plain `#[test]` does not support an `async fn` signature at all; the compiler
says so literally (`error: async functions cannot be used for tests`) at `🧵️shard/🏃️executor.rs:380` and
elsewhere. Downstream of that, every call the test bodies make to a genuinely-async constructor/method
(`MockGuestRuntime::new()`, `ShardExecutor::spawn(...)`, etc.) resolves to `impl Future<Output = T>` instead
of `T`, cascading into hundreds of `E0599: no method named ... found for opaque type impl Future<...>`
errors — including, concretely, `error[E0599]: no method named 'stop' found for opaque type 'impl
Future<Output = executor::ShardExecutor>'` at `🏃️executor.rs:385`. I confirmed this specific error is NOT
caused by my `stop()` signature change (group J): `executor` itself is already an un-awaited `impl Future`
by the time `.stop()` is called on it, so no method resolves regardless of `stop`'s own signature — same
mechanism breaks `.is_running()`, `.compile()`, `.instantiate()`, `.script_turn()` identically in the same
test. `cargo check --all-targets` reproduces the identical 919-error set (confirmed by diffing error
locations — same files, same shape, no new site). **`--lib` never compiles `#[cfg(test)]` code at all
(confirmed: `cargo check --lib` is EXIT 0 with 0 dropped-future warnings even before any of my fixes ran on
the test module), so none of the 37 production sites, nor the test module's own independent instances of
the identical bug, have ever been exercised by a passing test in this crate.** I did not attempt to fix the
919-error residue — explicitly out of scope per this ticket's rule 25 (atomic-adjacent, needs its own
packet, matching the identical class already documented for `sdk-final`/`dispatch-group-split`/`db-dedyn`
on other crates).

## Files touched (all inside the granted `🔌️plugin/🖥️host/**` path_scope)

- `⚡️effects/🦀️component.rs` — groups A-F (28 dropped-future fixes) + the `let _ =` corollary fix
  (group P above) + the new local `resolve_ready` helper.
- `🧵️shard/🦀️component.rs` — groups G-I (3 dropped-future fixes).
- `🧵️shard/🏃️executor.rs` — group J (1 dropped-future fix, R9 signature revert).
- `⏳️imports.rs` — groups K-M (3 dropped-future fixes).
- `🦀️component.rs` — group N (2 dropped-future fixes) + the `let _ =` corollary fix (group O above).

I did **not** touch `⏳️runtime.rs`, `🧪️schema-parity/🦀️component.rs`, or
`🧵️shard/🚚️process-transport/🦀️component.rs` — `git diff --stat` shows changes in those files too, but
that is other sessions' live uncommitted work in this shared tree (confirmed: none of my Edit calls
targeted them), not mine. I am flagging this explicitly per the ticket's own standing lesson about
misattributing a diff-vs-HEAD to the wrong packet.

## Cross-packet finding — NOT fixed, reported for `🛎️services`

`TimerWheel::arm`/`disarm`/`armed_count` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:494-511`)
hold a `std::sync::MutexGuard<WheelCore>` across their own internal `.await`, making their returned futures
non-`Send` — the exact defect their own sibling methods `pop_expired`/`next_expiry_ms` in the same `impl
WheelCore` block are ALREADY R9-tagged against, three methods over. This forced a `resolve_ready` bridge
(group F above) at the one call site inside my path_scope that needed to await `disarm` from within a
`Send`-required `Box::pin` block. `🛎️services` is outside `🔌️plugin/🖥️host`'s path_scope — per this
packet's brief, reporting rather than editing.

## Honest gaps

- Did not fix the 919-error `#[cfg(test)]` residue (`cargo test --lib` / `--all-targets`) — explicitly out
  of scope per rule 25, needs its own dedicated packet.
- Did not fix `🛎️services`'s `TimerWheel::arm`/`disarm`/`armed_count` Send-violating shape at its root —
  outside this packet's path_scope, reported above instead.
- The `let _ =` corollary sweep covered every `.rs` file under `🔌️plugin/🖥️host/**` (13 candidate lines,
  all individually checked), not a repo-wide sweep — repo-wide is out of this packet's path_scope.
- Did not re-verify the 2 test-only `probe.take_outbound()` `let _ =` sites beyond confirming they sit
  inside `#[cfg(test)]` with no assertion depending on their result — left as-is, matching the precedent
  the SDK packet's report already established for the identical shape.
