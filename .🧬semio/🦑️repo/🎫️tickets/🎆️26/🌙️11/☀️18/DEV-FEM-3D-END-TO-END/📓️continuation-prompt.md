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
