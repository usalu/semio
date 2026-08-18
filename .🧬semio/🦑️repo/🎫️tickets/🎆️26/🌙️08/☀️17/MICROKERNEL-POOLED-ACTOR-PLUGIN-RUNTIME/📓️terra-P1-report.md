# 📓️ terra — P1-process-shards report

Packet: **P1-process-shards** (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME). Executor: terra.

## 1. What landed

- **`ProcessTransport`** (parent-side `ShardTransport`) and **`StdioTransport`** (child-side `ShardTransport`) — both in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs` (new file).
- **`semio-shard` `[[bin]]`** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs` (new file), registered via a new `[[bin]]` stanza in `🖥️host/📦️packages/🦀️rust/Cargo.toml`. Hosts one real `ShardLoop` + `WasmtimeRuntime` over `StdioTransport`.
- **`ShardRuntimeKind`** (thread/process selection, `SEMIO_SHARD_KIND` env flag) and **`ProcessShardWatchdog`** (native port of `ShardClient.checkHeartbeats`'s 3-missed-window semantics) — same new file.
- **Two-line additive registration** of `pub mod process_transport;` in the shared `🖥️host/🦀️component.rs`, beside the existing `pub mod shard;`.
- Real, running kill→rebuild proof against an actual `wasm32-wasip2 world actor` component (see §5).

## 2. Framing protocol, and why

Length-prefixed: `[tag:u8][len:u32 LE][payload]`. Two tags — `Data` (an `Envelope`, one direction; a `ShardOutcome`, the other) and `Heartbeat` (empty payload). Chosen over a delimiter/newline framing because `Envelope`/`ShardOutcome` bytes are arbitrary binary (pack-encoded / JSON with arbitrary string-field bytes) and can legally contain any byte including `\n` — a delimiter needs escaping, length-prefixing does not. This matches `📓️design-runtime.md`'s own line verbatim: *"`ProcessTransport` (stdio, length-prefixed, last wave)."* The `Heartbeat` tag exists so a periodic liveness signal can interleave on the same pipe as real data without the reader ever mis-decoding one as the other — both `ProcessTransport`'s reader thread and `StdioTransport`'s reader thread branch on the tag before deciding whether to hand bytes to the inbound queue or just update a liveness clock.

One `Mutex`-guarded writer per direction (shared between the "normal" sender and the heartbeat thread on the child side) — a frame is three separate `write_all` calls, and two threads each doing their own `write_all`s without a shared lock would interleave and corrupt frame boundaries. Verified by `a_data_frame_round_trips_arbitrary_bytes_including_zero_and_newline` / `a_heartbeat_frame_carries_no_payload_and_does_not_desync_the_next_frame`.

## 3. Where `ProcessTransport` lives, and why not in `🎭️actor`

`🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs` — the plugin-host crate, NOT `semio-framework-actor`. `🎭️actor/🦀️component.rs`'s own module doc is explicit that the crate is domain-neutral and pure ("no I/O, no clock... no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file") and that its own `ShardTransport` trait doc names `ProcessTransport` as **host-supplied**, exactly like `WorkerTransport`. `ProcessTransport`/`StdioTransport` need `std::process::Command`, `std::thread`, and wall-clock time (`SystemTime`) — all banned in the actor crate. I made **zero edits** to `🎭️actor/🦀️component.rs` (confirmed: `git diff --stat` shows changes there, but they are NOT mine — see `## peer-coexistence`). Purity grep, run after all my other work:

```
grep -nE 'wasm_bindgen|web_sys|winit|tokio|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net' 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs
```
Only hit: the module doc's own sentence *listing* those banned tokens as prose (line 2-4) — zero actual usage. Crate stays pure.

## 4. Heartbeat / kill

- **Heartbeat**: `StdioTransport` (child) runs a background thread that writes a `Heartbeat` frame every `heartbeat_interval_ms` (200 ms in `semio-shard`'s `main.rs`), independent of whether the shard has any real outcome to send — an idle shard must still prove it's alive. `ProcessTransport`'s reader thread (parent) updates a shared `AtomicU64` wall-clock timestamp (`heartbeat_ms`) on EVERY frame it sees, `Data` or `Heartbeat` alike; `ShardTransport::heartbeat()` returns that value.
- **EOF**: `ProcessTransport`'s reader thread treats `Ok(None)` (clean EOF) *and* any read error from `framing::read_frame` identically — the child's write half is gone — and flips an `Arc<AtomicBool> alive` to `false`. `ProcessTransport::is_child_alive()` (a concrete-type-only method, same idiom `ThreadTransport::beat` already uses for its own extra non-trait method) exposes this.
- **`kill()`**: `ProcessTransport::kill()` — trait method — sends the child a hard kill (`Child::kill`, `SIGKILL` on unix) and `wait()`s to reap it (no zombie). `Drop` calls `kill()` if it wasn't already called, so a dropped `ProcessTransport` never leaks a live child. This is the *deliberate* parent-initiated path.
- **`ProcessShardWatchdog`**: the *involuntary*-death detection path, a native port of `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s `ShardClient.checkHeartbeats` — same semantics deliberately: three CONSECUTIVE stale windows (not three raw calls) before a shard counts as lost, with a `last_miss_counted_ms` gate so a flurry of polls inside one window doesn't multi-count. `poll_with_liveness` adds an EOF fast path `is_child_alive() == false` short-circuits immediately, which the web `ShardClient` has no equivalent of (a `Worker` never separately reports "my process is gone").

## 5. Kill→rebuild evidence (real, not simulated)

Test: `component::process_transport::tests::process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy` in the new file, `#[ignore]`d (needs a pre-built wasm component, so it does not run in the default 86-test suite). **I built the real component and ran it** — this is genuine end-to-end runtime evidence, not a description of intent:

1. Built the F1 scale fixture as a REAL `wasm32-wasip2` component (verified: magic+version bytes `00 61 73 6d 0d 00 01 00` — version 0x0d/layer 0x01 is the component-model format, not a bare module):
   ```
   CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo component build -p semio-framework-os-scale-fixture --target wasm32-wasip2 --features component-guest
   ```
   Exit 0. Full log: `terra-p1-fixture-wasm-build1.txt`. Artifact: `<ticket>/🎯️target-p1/wasm32-wasip2/debug/semio_framework_os_scale_fixture.wasm` (741 265 bytes).

2. Built the `semio-shard` bin explicitly (needed so `CARGO_BIN_EXE_semio-shard` could be handed to the test — `cargo test -p ... --lib` alone does not build sibling `[[bin]]` targets, so I built it with `cargo build -p semio-framework-plugin-host --bin semio-shard` first and passed the resulting path via the env var):
   ```
   CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo build -p semio-framework-plugin-host --bin semio-shard
   ```
   Exit 0.

3. Ran the real test:
   ```
   SEMIO_SCALE_FIXTURE_WASM=<...wasm32-wasip2/debug/semio_framework_os_scale_fixture.wasm> \
   CARGO_BIN_EXE_semio-shard=<...debug/semio-shard> \
   CARGO_TARGET_DIR=<ticket>/🎯️target-p1 \
   cargo test -p semio-framework-plugin-host --lib -- --ignored process_shard_kill_is_detected --nocapture
   ```
   **Exit 0.** Output (full log `terra-p1-kill-rebuild1.txt`):
   ```
   [semio-shard] pid=1517 package=scale-fixture-a actor=1 ready
   [semio-shard] pid=1518 package=scale-fixture-b actor=2 ready
   [semio-shard] pid=1538 package=scale-fixture-a actor=3 ready
   test component::process_transport::tests::process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy ... ok
   test result: ok. 1 passed; 0 failed; 0 measured; 86 filtered out; finished in 1.68s
   ```
   What actually happened, in order, all against real child processes and a real `WasmtimeRuntime`:
   - Spawned shard `a` (pid 1517) and shard `b` (pid 1518), each a real `semio-shard` child that compiled+instantiated the real component and registered one real `GuestInstance` on its own `ShardLoop`.
   - Sent each a real `Event::InstanceOpen{config: {"profile":"idle"}}` envelope, pack-encoded, over stdin; both replied with a real `ShardOutcome::Turn` (asserted, not assumed) — proving `execute_turn` actually ran against real wasm.
   - `kill -9 1517` — an **external** OS kill, not via `ProcessTransport::kill()` — the exact scenario the packet asks for.
   - `ProcessShardWatchdog` (500 ms window) polled `heartbeat()`/`is_child_alive()` and reported the shard lost (asserted) — via the EOF fast path, since the pipe closed immediately on SIGKILL.
   - Rebuilt: spawned a fresh child (pid 1538) at the same logical shard slot with a fresh actor id (generation bump, per the packet's own "restart-after-trap addressable without id reuse" convention), sent it `InstanceOpen`, got a real `ShardOutcome::Turn` back — the shard is usable again.
   - Shard `b` (pid 1518, never touched) was still alive throughout and answered a SECOND turn (a `Wake` envelope) AFTER `a`'s kill+rebuild — proving the failure was isolated, not fleet-wide.

This is the actual point of process isolation, proven against a real process, a real kill signal, and a real wasm component — not mocked.

## 6. Wiring into the native host (honest gap)

`ShardRuntimeKind::from_env()` (`SEMIO_SHARD_KIND=process|thread`, default `Thread`) is the selection seam the packet brief asks for. I grepped the repo before writing it: `ThreadTransport::new_pair` has **zero non-test call sites anywhere in this codebase** — no live `Kernel`/`ShardTable`/scheduler constructs a shard of either kind outside `🧵️shard/🦀️component.rs`'s own `#[cfg(test)]` module and `🦀️component.rs`'s wasmtime tests. So "wire it in behind a flag" cannot mean "flip an existing thread-shard call site" — that call site does not exist yet for EITHER transport kind. What I built is the selection primitive a future scheduler wires through (same "land the seam ahead of the caller" shape `GuestRuntime`/`MockGuestRuntime` themselves used before `WasmtimeRuntime` existed), not a switch with a live default-path on the other end. Recording this as a gap rather than fabricating a caller.

## 7. peer-coexistence

- Liveness check before editing `🖥️host/🦀️component.rs`: `git log --date=iso --oneline -5` showed no NEW commit since the ticket's baseline; mtime (15:58) was newer than the last commit, consistent with T1's in-progress uncommitted edit for metrics publishing, as flagged in the brief. Re-read the file immediately before editing; my edit was two lines (`#[path=...] pub mod process_transport;`) directly beside the existing `pub mod shard;` at the top of the file (lines 3-6), nowhere near the Metrics/ShardMetrics regions T1 was working in.
- **Two things landed under me mid-packet, flagged by the coordinator, both re-verified from disk rather than trusted blind**:
  1. `JobStep::Done`/`Failed` are struct variants (`Done { output }`, `Failed { error }`) — I never constructed a `JobStep` myself, so no action needed; confirmed by reading `🖥️host/🦀️component.rs` fresh before writing `👶️child/🦀️main.rs`.
  2. `BudgetLimiter::default()`'s `max_instances` changed 1 → 256 (plus `total_core_instances`/`total_memories`/`total_tables` added to the pooling config) — this is WHY `semio-shard`'s real `WasmtimeRuntime::instantiate` call actually succeeds; had it still been `max_instances: 1`, the child would have failed to instantiate. Not something I changed; just a load-bearing precondition I verified rather than assumed.
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` shows a 73-line diff in `git diff --stat` — **not mine**. I made no edits to that file at all (confirmed by never having run `Edit`/`Write` against it in this session); this is a sibling packet's in-flight, uncommitted work.
- Did not touch: root `📜️script.ts`, root `📋️project.json`, `.vscode/🧩️launch.seed.jsonc`, any `🤖️generated/**`, `Shell/🧊️component.rs`, `ShellHost/🟦️component.tsx` — all registrar-only per `path_scope`, see `## lease-requests`.

## 8. Commands run, every one, with exit code

```
CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo component build -p semio-framework-os-scale-fixture --target wasm32-wasip2 --features component-guest
  → exit 0 (terra-p1-fixture-wasm-build1.txt)

CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo check -p semio-framework-plugin-host --all-targets
  → exit 0, 0 errors, only the pre-existing unrelated semio-framework-os-kernel warning (terra-p1-check1.txt)

CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo test -p semio-framework-plugin-host --lib
  → exit 0, "test result: ok. 86 passed; 0 failed; 1 ignored" (terra-p1-test1.txt)
  (baseline was 74 passed at packet start, re-verified as 86/0 by the coordinator mid-packet after K1's fix landed;
   my 12 new tests — 3 framing, 1 selection, 4 watchdog, 3 ProcessTransport-with-a-real-`cat`-process, 1 ignored
   kill-rebuild — account for exactly 74 + 12 = 86. Zero regressions.)

CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo build -p semio-framework-plugin-host --bin semio-shard
  → exit 0

SEMIO_SCALE_FIXTURE_WASM=<...> CARGO_BIN_EXE_semio-shard=<...> CARGO_TARGET_DIR=<ticket>/🎯️target-p1 \
  cargo test -p semio-framework-plugin-host --lib -- --ignored process_shard_kill_is_detected --nocapture
  → exit 0, "test result: ok. 1 passed; 0 failed" (terra-p1-kill-rebuild1.txt) — the real kill→rebuild proof, §5.

grep -nE 'wasm_bindgen|web_sys|winit|tokio|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net' 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs
  → only the module doc's own prose naming those tokens; zero real usage; crate stays pure.
```

I did **not** touch `semio-framework-actor` (no edits), so its own `cargo check`/`cargo test` were not re-run as a gate for MY changes — the purity grep above is the relevant proof instead.

## 9. Gaps (honest, not glossed over)

- **No live scheduler calls either `ThreadTransport` or `ProcessTransport` yet.** `ShardRuntimeKind` is a seam, not a wired switch — see §6. This is a repo-wide pre-existing gap (confirmed by grep), not something P1 introduced or was supposed to close by itself.
- **`semio-shard`'s CLI bootstraps exactly one starter actor** (`<wasm> <package-id> <actor-id>`), not a multi-actor shard. `ShardLoop::register` supports many actors per shard; nothing stops a future caller from sending more `Envelope`s addressed to OTHER actor ids once a real multi-actor bootstrap protocol exists over the same stdio channel — I didn't invent one, since no design doc specifies the wire shape for "register a second actor on an already-running shard-child" and guessing one felt riskier than flagging it.
- **No restore-from-checkpoint on rebuild.** The kill→rebuild test's rebuilt shard gets a FRESH actor (new generation, no state), which matches `ActorId`'s own "generation makes restart-after-trap addressable without id reuse" design intent, but does not exercise `GuestRuntime::checkpoint`/`restore` across the kill boundary — that mechanism is already proven correct by `🧵️shard/🦀️component.rs`'s K1 suspend/resume tests; wiring "checkpoint before we lose the shard" into the watchdog's rebuild path needs a live scheduler that knows the actor's last checkpoint, which doesn't exist yet (same gap as §6).
- **Lease-request**: the launch entry `🛠️dev🖥️s🧊️wgpu🖥️native🧵️process-shards` (`design-workforce.md`'s own naming) — registrar-owned per `path_scope`. Suggested shape: a `.vscode/🧩️launch.seed.jsonc` entry that runs the wgpu-native dev target with `SEMIO_SHARD_KIND=process` set, once a scheduler actually reads that flag (see §6) — premature to add the entry before there's a live consumer of the env var, flagging rather than adding a dead launch config.

## 10. Files touched

- New: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs`
- New: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` (`[[bin]]` stanza)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (2-line additive `pub mod process_transport;`)
- Ticket scratch: `terra-p1-fixture-wasm-build1.txt`, `terra-p1-check1.txt`, `terra-p1-test1.txt`, `terra-p1-kill-rebuild1.txt`
