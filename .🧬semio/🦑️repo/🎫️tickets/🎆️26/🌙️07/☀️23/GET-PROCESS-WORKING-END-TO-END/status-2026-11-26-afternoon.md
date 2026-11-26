# Status 2026-11-26 Afternoon (14:45)

## Where it stands

| Area | State |
| --- | --- |
| Browser end to end (Firefox, `:6022`) | Last green: hub of 2026-11-25 23:31. Interactions, load-trace and Demo → Viewer → Editor all passed and rendered the workpiece (`e2e-process-v14-*.png`). Cancel-replay failed. The two hubs since then (v15, v16) don't boot: the live-cleanup pump faults with `zero-progress`. The fix is in the tree, and hub rebuild v17 is queued. |
| Cancel replay | Fixed natively. A cancelled cleanup no longer poisons the instance. `folder_reload_route` 10/10 includes the new law. The browser re-check comes with v17. |
| Folder save/reload | `folder_reload_route` 10/10: operation-wire receiving, replay payload sizing, commit publishing, replacement-store owners, implicit trunk. |
| Time travel (`time_travel_tests_1`) | 31/32, up from 0/28 (it didn't compile). The last law (3061-op undo of a finalize) is down to replay memory retention, and an agent is on it. |
| Replication history | 48/48, `fold_work` 3/3, spr fold law, TS twins 3/3. |
| Store test harness | Original-turn driver done: tests-08 41/41. tests-01 compiles (40 pass, 15 fail, 2 hang), and an agent is on the failures. tests-02/03/04 and space-space are being ported. space-collection has 6 sqlite failures, and an agent is on them. |

## Fixed this session (product and infrastructure)

- **Product:**
  - Operation wire: production apps install their persisted receiving and target.
  - Replacement stores use the persisted wire.
  - Commit publishes through the memory backbone.
  - Implicit history trunk, with a trunk-chain accessor.
  - Draft preview shares the Report replay's replacement module.
  - `Withdrawn` drafts are reported in outcomes.
  - Queued time-travel effects: a base move no longer drops a preview or replay.
  - Revert frontier settle.
  - Finalize, undo and redo heads.
  - Pure-lane replay initializer.
  - Report replay: about 190 → 5.5 turns per op. Finalize: about 650 → 4 turns per op.
  - Lease-starvation livelock.
  - Fold cleanup, issuer leak and copy-bytes bugs in replication.
  - Live-cleanup cancel handling.
  - A sealed store type's fields were made private.
- **Infrastructure (fundamental, these slowed every agent):**
  - The memory broker over-estimated `syn`/`libc` (up to 5.9 GB), which refused or deadlocked test builds for hours. Fixed, with laws and a Python oracle.
  - The capacity gate starved large requests behind small ones. Fixed with head-of-line reservation and aging, plus laws and an oracle.
  - The hub link was missing from the warden's carving list (it was killed repeatedly), so I declared it. The thin-LTO link still peaks at about 9 GB and trips the danger level on 16 GB. This ticket's prototype hub build now uses `lto="off"` with a 3 GB grant, and it builds reliably on the first attempt.
- **Cleanup:** removed leftover `[DEBUG]` prints from 26 files. Fixed a stale TS twin and a stale spr test line.

## Problems faced

- Shared tree churn: other sessions' in-flight edits broke the hub build about five times (transient compile errors, `DBG` statics). Each attempt also rebuilds 400–800 crates.
- Memory on 16 GB: the warden cancelled the 9 GB thin-LTO link seven times. Solved with no LTO.
- Weekly model limit and a session restart stopped agents mid-run. They were resumed.
- Agents sharing private target dirs deleted each other's builds, and stale waiter loops piled up. Cleaned up.

## Expected finish

- Hub v17 builds (about 60–90 min including the queue), then the four browser scenarios run (about 25 min). If the stall rule holds, that is green end to end. I'd say early evening; the risk is more boot faults that are only visible in the browser.
- The time-travel long-history law, store tests-01/02/03/04, space-space, sqlite and the digest crates come in parallel, in roughly 1–3 h, each still able to surface product bugs.
- Then the extensions (metal, wood, robotic, concrete), bridge and hub tests. These weren't reached yet and are the largest unknown.
- Close-out: delete `🗑️generated` (about 25 GB of native builds), keep the reports, close the ticket.

## Resource use

- 6 Sonnet agents active (time travel, 4 store crates, sqlite, digests, live-cleanup rule) plus the hub build.
- The machine is saturated: 4 grants running, 7 queued, load about 7, memory at normal pressure, 22 GB disk free.
- The binding constraint is 16 GB of RAM: more agents would only queue in the capacity gate. CPU isn't the limit.
- No Haiku agents right now: the remaining work is all diagnosis and fixes, not exploration.
- The shared work fleet: `repo fleet_status` was not usable for these builds, so everything ran on native tools through `repo cargo` and `repo capacity run`.
