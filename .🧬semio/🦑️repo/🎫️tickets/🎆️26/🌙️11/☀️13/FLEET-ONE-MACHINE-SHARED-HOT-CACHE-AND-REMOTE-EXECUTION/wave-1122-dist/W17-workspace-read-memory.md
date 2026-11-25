# W17: why g1-m08 and g1-m10 fail the "workspace" Ready stage (2026-11-25, 08:54 to ~09:40)

`O` = `/Users/Shared/semio-store/v1/origins/`, `HC` = `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/`, `A` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/`. Work folder `W17/`: `runs.py` (table of `ready-timings.json`), `fleet-status.json` (`repo fleet status --json`, 08:58), `base/decision.ts` + `base-plug.ts` + `run/graph-plan-base.test.ts` (runs a test file against the exact pre-edit decision code, checkout untouched), `sha-before.txt`. No sub-agent, nothing restarted, no scratch build, one test file at a time, load checked before every heavy step (14 to 27, never above 30).

## Verdict
1. **Memory does not kill the read now.** The slice reports of all five workers I could read show 1.9 to 2.3 M arrays and 7.1 to 8.0 M strings (g1-m01 03:52, g1-m03 01:50, g1-m08 08:53, g1-m10 01:34, g1-m13 07:39). W15 measured 17 to 37 M arrays at 00:41; that is no longer the case. The slice workers are under the 4,096 MB cap (rss 1.5 to 3.9 GB; m08's 1,455 MB rss is lower than its 2.4 GB heap because the worker is swapped out: 7.6 GB compressed, 277 MB/s swap-out). The last two m08 runs peaked at 2,318 and 3,421 MB (stage tree). No memory-cap cancellation of a slice worker appears in the current m08/m10 alarms. I changed no cap and no consumption.
2. **g1-m10: its graph read has waited 26,000 s (about 7 h, since about 01:53) in the host-capacity gate with reason `session-cap`.** It never ran out of memory. The ready tail says: "waiting for host capacity (this agent session already holds its share of the memory reservations); 2 active grants, 1 queued; the gate reports 1,700 to 3,100 MB available above the 2,048 MB floor; 6553 MB requested, minimum 2048 MB".
   - The cap is `sessionCapBasisPoints` 4000, which is 6,553 MB on a 16 GiB worker (`HC📏️decision/🟦️.ts` `hostSessionCapMegabytesV1`). It refuses a request while the same session's reservations plus the request's minimum would pass the cap.
   - On a fleet worker every process the sidecar starts derives the same agent session: the ready stage, its run-many grant, its local units (rustc wasm32 units at 1.5 to 4 GB during the wait) and the graph plan. So the plan waits behind its own command's grants. That is a self-starvation with no end: the heartbeat counts as progress, so the stage is neither stalled nor timed out.
   - The cap exists to share the host between agent sessions. With no second session on the host it protects nobody.
3. **g1-m08 (and g1-m10 at 00:07) get past the read and then fail in the build:**
   - `@semio-tech/norm-din4108:build` fails with "Foreign package output binding preserved and refused" (`A🟦️typescript/🔗️binding/🟦️.ts:33`; the run marks it "another rerun is unlikely to help").
   - **This refusal appears only on m08 and m10.** None of the other 10 workers' failure logs has it. It is the deterministic difference between the two failing workers and the eight that are Ready.
   - The other failures on m08 are peers' transient edits: `process.source.unresolved-module` `dbg` in `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs`, plus `framework-rs:generate` and `framework-actor-rs:typegen`. Then come a restart because a peer edited files ("2 of 6"), a graph service hold of 658 s, and SIGTERM (exit 143) at 3,791 s.
   - The binding's receipt lives in machine storage, but the links (`dist`, `📤️dist/<mode>`) live in the package folder. m08 and m10 are also the only two workers whose `.cache/b/p/m` image is small (14,335 and 9,215 MB; the others are 24,575 to 31,743 MB, m14 13,311). That fits a store recreated under disk pressure, losing the receipt while the link stayed. It is not proven: worker records cannot show the file kind.
4. **m01/m02/m03/m06/m07 did not get Ready through a graph adopted from the fleet.**
   - No worker log has `graph-adopted` or any closure-graph adoption line. The logs are tails, so this is absence of evidence only.
   - m01's slice report at 03:52 shows a real owner read ("reading the owners (round 1)", 44 of 3,600). m01 and m03 *published* the selected nx-graph ("Published the nx-graph of state … to the fleet cache (3 documents, 4 MB)").
   - They became Ready in ordinary reads of 1,001 to 3,070 s (m03 01:11 to 02:03, m06 01:49 to 02:12, m01 03:46 to 04:10, m07 03:47 to 04:06, m12 00:59 to 01:19, m13 07:08 to 08:08, m14 04:06 to 04:20). They succeeded after the array regression stopped and without the binding refusal. m02 has been Ready since 11-24 07:54, before GR22.
   - The coordinator's machine graph exists (`.cache/f/closure-graph`, 10 MB, head 09:01). Whether a fleet copy was adopted is not visible in the records.

## Numbers
| worker | last runs (start to end, s, outcome, stage peak MB) | what stops it |
|---|---|---|
| g1-m08 | 01:12 to 07:30, 22,659 s, failed, 2,318; 07:50 to 08:53, 3,791 s, exit 143, 3,421 | build: foreign binding (norm-din4108) + peer-edit failures, then timeout during the re-read; 12 h: 0 ok / 8 failed / 4 stalled |
| g1-m10 | 00:35 to 01:07, 1,932 s, failed, 4,047; stage running since ~01:27, adopted 07:44 | graph plan queued 26,166 s on `session-cap`; 12 h: 0 ok / 6 failed |
| fleet (08:58) | m08: pressure warn, refusing (no headroom, memory), 905 actions offloaded out; m10: refusing (queue-order), running 3, queued 1, offloadedIn 0 | |

## Changes (both in `T/landings.md`)
1. **Session cap binds only while a second agent session competes** (`HC📏️decision/🟦️.ts`, lock `lock.host-capacity`, ~09:04).
   - `decideHostCapacityV1` builds the `contending` set: the sessions of the round's active grants and waiters, without the sessionless ones. `session-cap` is judged only when that set holds more than one session.
   - Alone on a host, a request is judged by memory, budget and load alone. With a second session holding a grant or waiting, the old rule applies unchanged.
   - The docstring of `hostSessionCapMegabytesV1` and the `sessionCapBasisPoints` description in `HC🧬️schema/🔣️.json` are updated.
   - Python oracle `HC🧪️tests/🛋️reserve/🐍️.py` restates the rule.
   - Fixture changes:
     - 3 new cases: alone on the host, holding 5,000 MB and getting the 2,000 MB read; a waiter of a second session makes the cap bind; a sessionless grant is no second session.
     - 2 cases gained a second session, so they still prove the cap.
     - The test maps null ledger sessions to absent and compares decisions in queue order. The gate orders waiters fairly, and the oracle states one verdict per queued request.
2. **Test arithmetic fixed** in `…/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧮️graph-plan/🟦️.ts`. The case "fair queue ordering even when a plan fits" now holds 12,000 MB instead of 20,000. The interactive reserve leaves only 480 MB of budget beside 20,000 MB, so the 3,000 MB plan could not "fit". The test was red before W17: 11/1 on the exact pre-edit decision code through `W17/run/graph-plan-base.test.ts`.
3. **The foreign-binding refusal names what it found** (`A🟦️typescript/🔗️binding/🟦️.ts` `foreignBindingV1`, lock `lock.artifact-binding`, ~09:10). The message now carries the path, its kind (link, directory or file), the link target, and the receipt's modes and active mode (or "absent"). The next m08/m10 failure log will tell a lost receipt from a foreign output.
   - No protocol change: `existingForeignBinding: refused` stays.
   - `A💾️storage/🧪️tests/📥️physical/🟦️.ts` expects the detailed message.

## Tests (`repo test-file`, one at a time)
- reserve 2/0 (11 oracle cases).
- graph-plan 12/0 (pre-edit code 11/1).
- process host-capacity: main 1/0, progress, upgrade/agents, gate, essential, all green.
- library unsatisfiable green.
- binding plan (Python oracle) green.
- `A💾️storage/🧪️tests/📥️physical/🟦️.ts` is red before and after with the same fixture cause ("Actual artifact repository authority required", 14 of 15 tests). It is not caused by this change and was not fixed here.

## Watch
Watched 09:09 to 09:31. **Neither worker was Ready at the end of the watch.**
- **g1-m08** left back-off at about 09:13 (attempt 19) on the new code.
  - 101 s: file inventory.
  - 430 s: "2620 inventory records and fleet records verified; taken 515 of 609". The fleet take works.
  - 1,040 s: the read started "a new pass", with the inventory back at 0 observations. That matches the tree churn under the read.
  - No memory alarm on its slice worker in the window.
  - It had not yet reached the build, where the new binding message would show.
- **g1-m10** was still on `session-cap` at 27,506 s. The gate reported 1,490 to 4,241 MB available above the floor, enough for the 2,048 MB minimum. Its waiting graph service still runs the old decision module (open item 1).

## Open items
1. **m10 needs a fresh graph service process to use the new rule.** The waiting service on m10 loaded the old decision module. Its reload drains "until `drainStallMilliseconds` without an original work observation", and the reservation heartbeat may count as work, so a reload may never cut it. I restarted nothing. If m10 stays on `session-cap`, the ready supervisor (or a human) has to end that stage.
   - Follow-up for the resident service: a plan that is only waiting in the capacity gate should not count as work in flight for a reload drain.
2. **The m08/m10 package binding needs the facts from the new message.**
   - If it says "link -> …/a/ts/<hash>/<mode>/out; receipt absent", the receipt was lost. The fix then is to adopt a protocol-shaped link of this owner (its target ends in `a/ts/<sha256(owner)[0:32]>/<mode>/out`) when the receipt is missing. That is a change to the binding protocol and needs a working `📥️physical` harness first.
   - If it says "directory", something writes a real `dist` there.
3. Separately on m08: peers' `dbg` module leftovers in `🦀️.rs` sources and failing `framework-rs:generate` / `framework-actor-rs:typegen` keep failing its stage build. They are transient, not this lane's.
4. **The 6.3 h m08 run (01:12 to 07:30) cannot be attributed.** Its log was overwritten; it is probably the same `session-cap` wait as m10, since its peak was 2,318 MB.
