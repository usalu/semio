# 🔁️ Continuation Prompt (handoff 2026-11-20 ~02:00)

Get dev fem 3d working end to end. In parallel, the shared work fleet mechanism is being developed; if it doesn't work, fall back to native tools. Never get stuck or hold your work. Use the main chat with GPT 6.2 Astra Medium for plan coordination, multiple GPT 6.2 Sol Medium agents for medium- to high-complexity task execution, and multiple GPT 6.1 Terra Medium agents for read-only exploration, audits and low-complexity tasks. Finish the goal as fast as you can with all the resources you can get; never idle, always stay saturated. When you find fundamental issues unrelated to the task, fix them as well. Treat the goal completely end to end. Other agents work on other logical parts, sometimes in the same files. Don't stop; work in conjunction with them and focus on your task.

## Where things stand

- **Ticket:** `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️18\DEV-FEM-3D-END-TO-END\` (manual `🎫️ticket.json`, still open).
  - Read `📓️status.md` first; its rows are chronological.
  - Supporting reports:
    - `📓️set-active-example-no-progress.md`
    - `📓️event-batch-ui-patches.md`
    - `📓️analysis-queued-and-shell-stall.md`
    - `📓️publication-receipt-refusal.md`
    - `📓️familyless-publication-posting.md`
    - `📓️lease-claim-posting.md`
    - `📓️debug-probe-inventory.md`
- **Shared worktree:** `C:\semio`, live-replicated. Re-read files before editing. No mutating git commands.
- **Dev server:** the background shell serving fem3d with `SEMIO_VITE_HMR=0` on http://127.0.0.1:6087. Host TS changes need `bash native-chain.sh 14` (prepare+activate) and then a reload.
- **Build:** run from the ticket folder.
  - `bash native-chain.sh 13` builds the fem component, 50–55 min. Exit 75 means a source edit landed mid-build, and it retries itself.
  - Step 13 is followed by `bash native-chain.sh 14` (about 15 s).
  - A publish shows as a new mtime on `C:\semio\.cache\b\p\m\browser\prototype\modules\🏗️fem\semio_hub_fem_component.core.wasm`.
  - Never run two step-13 loops at once.
  - A transient EPERM rename in `.cache\b\r\m\u\h` is solved by a retry.
  - Kill process trees by PID (`taskkill /PID <winpid> /T /F`), never by image name.
- **Running at handoff:** `rebuild-loop16.txt`, attempt 3, started 01:54:45. It should publish about 02:50.
- **Native cargo checks:** use a private build dir, because the repo config now points `build-dir` at a shared cache.
  ```
  CARGO_BUILD_BUILD_DIR=C:/b/<x>/bd CARGO_TARGET_DIR=C:/b/<x> CARGO_BUILD_RUSTC=rustc RUSTC_WRAPPER= CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 cargo ... -j 4
  ```
  Delete `C:/b/<x>` afterwards.

## Fixed this session (in source)

1. **OpBinary producer** (`🔌️plugin\📜️declaration\👷️job\🎮️command\🧬️op-binary\🦀️.rs`):
   - SEAL advances only on a settled `stepped`.
   - PAGES waits for a pending marker (`self.unit.is_none()&&offset==length`).
   - The `_` arm names the producer state.
2. **Root cause of "producer advanced after wire complete"** (`🧰️framework\🔨️modules\🧵️job\🏭️construction\🦀️.rs`, `admit_original_construction_settled`):
   - The `Ready` arm is now `{if !admission.reset_step(context)?{return Ok(false)}if output.is_some(){return Ok(true)}committed=false}`.
   - Its test is `settled_construction_whose_reset_went_unpaid_still_runs_the_next_outputs_constructor`.
   - 6 job-crate tests fail before and after the change (pre-existing debt).
3. **Receipt ordering in `exchange_generic`** (`⚛️reactor\🔄️turn\⌨️command-ingress-turn\🦀️.rs`): an applied output's receipts are collected in the producing step, before input retirement.
4. **Mounted pump receipt-less arm** (`🧵️instance\🧾️typed-operations\📥️staging\📬️publication\🏃️mounted-run-and-pump\🦀️.rs`): adds the unit's direct posting to `spent`.
5. **Publication window** (`⚛️reactor\🏃️runtime\🧰️ops\🧾️typed-operations\🦀️.rs`): binds a posting into `original_receiving` only when a family receipt is pending (`bind_window_posting`).
   - Lease claims (128 B on wasm32) stay outside the receipt binding.
   - Its test is `a_lease_claim_posting_without_a_family_receipt_owes_no_receiving_and_with_one_binds_that_family`.
6. **Shard worker `replyError`** (`🌐️browser-bundle\🏗️materialization\🧵️shard-worker-source\🔌️actor-loading-and-replies\🟦️.ts`): cycle-safe payload stringify.
   - The host warning still shows `[object Object]`, so the message is built in an outer layer that hasn't been found yet.
7. **Earlier in the ticket:**
   - held command-scope lease
   - reply-scope Drop
   - host event-batch UI patches
   - rustc wrapper response-file retry
   - console probe cleanup

## Update 02:58

Loop 16 attempt 3 published the wasm at 02:56 (177314662 bytes), then ended on exit 75. A source edit landed mid-build, and native-chain's internal retry gave up with exit=1. No build is running now. Run `bash native-chain.sh 14` against the 02:56 wasm. If more edits have landed since, also start a fresh `bash native-chain.sh 13`.

## Open: next steps

1. When loop 16 attempt 3 publishes, run `bash native-chain.sh 14`.
2. Open a **fresh** browser tab on 127.0.0.1:6087. Stale tabs keep old console lines.
3. Read the console right away with narrow substring filters (not regex): `input #`, `semantic failure`, `receipt-refusal`, `owes its enclosing`.
   - The 01:53 wasm still refused seq 7 (`setActiveExample`, `dispatch-failed`).
   - A possibly stale line showed a new failure: "original receiving scope still owes its enclosing semantic receipt". Confirm it, then trace it.
4. Then verify fem3d end to end:
   - Model window renders.
   - Results leave "Analysis queued".
   - No stall banner.
   - The example switch works (demo, concrete-forest, house).
   - Actions dispatch.
5. Unrelated debt worth fixing:
   - `semio-plugin-tests-16` doesn't compile (`plugin-runtime-plugin-builder-contract-part-3`: `PeerPresenceRootRetirement` and others are missing). The regression assertion in `🧪️tests\📤️original-emission` can't run until it does.
   - The exchange snapshot-test failures and the job-crate test failures.
6. Close-out:
   - Remove all `[DEBUG]` probes listed in `📓️debug-probe-inventory.md`, plus my `[DEBUG] publication-unit` probes in `🔁️stall-witness-and-unit-advance\🦀️.rs`. The peer `receipt-refusal` eprintln belongs to someone else; coordinate before removing it.
   - Delete `🗑️generated` and any `C:/b/fem3d-*`.
   - Close the ticket with a summary and the full file list.

## November 20 evening continuation

Read the latest acknowledgment and result-page evidence in [activation verification](./📓️activation-verification.md). The old guest emitted the Demo document and accepted the Effect and UI acknowledgments. It returned a semantic refusal while polling the Terminal acknowledgment. The native actor replay now loads the actual WIT poll budgets and three shell acknowledgment tokens from the recorded browser tape.

The standard component pass retried as loop 20 after loop 19 lost the Windows build broker. The direct Cargo fallback is building the same FEM composition for wasm32-wasip2 under E:/b/fem3d-wasm. The official publication watcher runs activation on a changed core. The ticket browser-smoke script can collect the four requested filters, example choices and model geometry. The playground is served on 6087. Native House, sparse, snapshot and actor replay verification are in progress. Keep this ticket open until fresh browser rendering, completed Results, editing and all three examples work.

## November 21 document handoff checkpoint

The original sequence 7 receipt issue is resolved through terminal ACK in the November 21 linked component. The 6099 exact WIT trace accepted all three pages and continued polling. It exposed the next hold at document load 118 of 118, after SpawnJob and before any visible Model. The native 6098 build reproduced the hold and eventually reported a direct mounted zero-progress refusal. See the [activation report](📓️activation-verification.md) and [archive effect admission](📓️archive-effect-admission.md).

The source now parks pending effects across archive admission and store replacement. Live maintenance retires mounted readers during handoff and the same ladder accepts their snapshot return witnesses. The independent native House reconciler passed with indexed Model and pending Results in both cold and Demo-to-House routes.

The native handoff rebuild uses E:/b/fem3d-wasm and logs to native-handoff-wasm.txt. On success its watcher starts 6100 and the fresh browser waits for solved Demo Results before switching all three examples and verifying Add Node, Undo and Redo. The official November 21 link initially stopped on a stale descriptor helper that rejected --prepare-actor. The helper project rebuild and official retry remain active. The 6087 activation watcher runs steps 14 and 15 on publication, then fresh Model, Results and action checks. Keep the ticket open until those flows are verified and the temporary instrumentation and outputs have been cleaned.

### Current build attempts

The direct handoff Cargo runner is now on native-handoff-wasm-retry-3.txt. Earlier attempts planned before the live replication-causal, reactor-lifecycle and instance-render manifests gained their corresponding direct dependencies. Those manifests now declare them. The current run passed instance-render and reached the STDIO composition crates. The watcher publishes its successful result to the 6100 diagnostic preview. The official retry restarts on source-digest changes. The descriptor rebuild is compiling the helper with the actual artifact writer.

The limited-grant regression verifies that snapshot returns, replacement and mounted retirement can share the archive reserve. The focused maintenance tests passed four tests and 48 assertions. The archive gate and stale cancellation tests passed three tests and 29 assertions. House ambient and Demo-to-House reconciliation measurements are in [house-scene-reconcile.md](house-scene-reconcile.md). The broad native session link was intentionally stopped after successful code generation to free memory for the acceptance builds.
