# result-e2e (wave 1027b, lane e2e): course end-to-end verdict on the 10-27 tree

C = the ticket folder. Logs: `C/🗑️generated/e2e/` and `C/🗑️generated/e2e-course-e2e-1027-*.log`. Mirror `e2e-cl` (mine, library subtree) holds the claimsctl law run.

## VERDICT TABLE (10-27, live tree of 17:14-21:00, host load 28-80)

| run | topology | what | result | cause |
|---|---|---|---|---|
| e2e-1027-1d1 (final.sh attempt 1, 17:14, 62 min) | dev | boot 1/1; exclusive stack 12/12 (presence, shortage, away, faults); shared stack 62/64 | RED (2 specs) | two `pets` specs, both flakes of the SPEC at any load (see Findings 2-3), not load timeouts |
| e2e-1027-p1 (focused: the 2 pets specs x6, 3 workers, load 29) | dev | 12/12 passed in 141 s | GREEN | the flake rate is below 1/6 per spec when the host is calm |
| e2e-1027-p4 (focused x40, 4 workers, load 40-60) | dev | 63 of 80 passed; parachute 35/40, click-under-pet 30/42 | RED | reproduces: 17 failures = 12 click-under-pet + 5 parachute |
| e2e-1027-p2 / p3 (focused, 19:31 / 19:34) | dev | died before any spec at graph-plan: `Original lease signal identity required` | RED, LIVE DEFECT | Finding 1 (graph claim port) |
| e2e-1027-r1 (20:19-20:41, 17 min gate) | rehearsal | boot 1/1; exclusive 12/12; shared 62/64 | RED (2 specs) | parachute (Finding 3, the same head flake) + layered-home `nested grid pans` preview 845 vs 839 px (Finding 6) |
| every other shared/exclusive spec of the course (74 + 12 + boot) | dev | passed in 1d1 | GREEN | |

So: dev passes 75 of 77 specs and rehearsal 75 of 77 on today's tree; every red is a spec race (none is a product defect, none a plain load timeout). Not "course e2e green" yet: a green full run needs `petsrace` + `layoutfresh` landed (the two pets specs alone fail a full run about 60 % of the time).

FOR coordinator (ready lines, nothing applied by me):
1. `python3 -I C/e2e-tools/staged/claimsctl/stage.py apply` ; verify: mirror e2e-cl already applied it: `sh C/e2e-tools/staged/claimsctl/verify.sh <mirror root>` (law 4/4, existing claims suite 124/0). Revert: `python3 -I C/e2e-tools/staged/claimsctl/stage.py revert`. LIVE DEFECT, land first: every command that must WAIT for a graph claim while it holds an issued control dies (see Finding 1); the 19:31 and 19:34 e2e starts died of it at graph-plan.
2. `python3 -I C/e2e-tools/staged/petsrace/stage.py apply` (2 spec files); verify `sh C/e2e-tools/staged/petsrace/verify.sh .` (parse check); PROOF = the focused run (command in that verify.sh): base 17/80 failed, expect 0/80. Revert: `python3 -I C/e2e-tools/staged/petsrace/stage.py revert`.
3. `python3 -I C/e2e-tools/staged/runfix/stage.py apply` (retry.sh, focus.sh, specs-finished.pattern, law); verify `sh C/e2e-tools/staged/runfix/verify.sh .` (law 5/5, red on base). Revert: `python3 -I C/e2e-tools/staged/runfix/stage.py revert`.
3b. `python3 -I C/e2e-tools/staged/layoutfresh/stage.py apply` (1 spec file); verify `sh C/e2e-tools/staged/layoutfresh/verify.sh .`; proof after landing: focused `--project desktop --no-deps --repeat-each 12 --workers 4 --grep "nested grid pans"`. Revert: `python3 -I C/e2e-tools/staged/layoutfresh/stage.py revert`.
4. After 1-3b: `sh C/e2e-f-tools/final.sh e2e-1027-<n>` (quiet host: dev about 60 min, rehearsal about 30 min).

## Findings

### 1. LIVE DEFECT: graph claim lease port passes a derived signal next to the issued control (claimsctl)
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs` `createQueuedClaimLeasePortV1`: `request()` passes `signal: signal ?? control.signal` together with `control`. `waitAny` (custody) calls `lease.wait(name, {signal: controller.signal})` with its own AbortController, and the process lease module's `remaining()` refuses `control.signal !== signal` (`Original lease signal identity required`). Any `bun repo` command that has to wait for a claim held by a peer (graph prefetch/refresh "go first") dies at graph-plan after its wait. Same class lockc-c fixed for the cargo custody on 10-23. Seen twice in a row (p2 19:31 after a 62 min capacity wait; p3 19:34 after 70 s); p4 (19:46) got through because it had nothing to wait for.
Fix (staged `claimsctl`): with a control the port always passes `control.signal`; a caller's own abort ends its wait at once (`follow`), a lease granted after that is released exactly once, and the shared waits of one resource share one queued ticket (no ticket per poll). Law `🧪️tests/🎛️control/🟦️.ts` (4 tests, independent oracle = the lease module's own refusal rule copied into the stub): red on base (3 fail), 4/4 on the fix; the existing claims suite 124/0 on the mirror.

### 2. `pets` spec "a control under a pet still receives its click" fails 12 of 42 (29 %) at 4 workers
Traces (`trace.zip` of the kept run directory `🗑️generated/nx/e2e/20261027T185955-96696`): the in-page watch for the pet in front of the card is started by a second harness call AFTER `mouse.up()`; the gap was 21-445 ms (8 of 11 above 150 ms) and the pet is in front of the card for tens of milliseconds only; even with gaps of 21-27 ms 3 failed, because the watch samples once per animation frame and a frame takes 100-350 ms on a busy host (mouse moves + one frame took 350 ms each in the trace). Fix (staged `petsrace`): arm the watch before the hand lets go (`watchFall`, answer in `window.petWalkFall`, the pets' time counts only once the pet left the hand), try again (5 times) when the pet was not seen, assertions unchanged.

### 3. `pets` spec "a pet picked up ... opens its parachute" fails 5 of 40 (12 %)
Four of five failure traces carry the same trail `hang/hand > tumble/air > slide/head > fall/air > land/perch`: the drop column (first of the widest-floor columns, here the 15 px corridor at x=480 between the Badges and Settings cards) lies between two perched pets, the released pet lands on a pet's head, slides off and falls too short for a parachute. Random cast and places, independent of load. Fifth: the pet walked on between `reachable()` and the hand's move (`carry` waited 6 s for the `grab` cursor at a stale position; also 1 of the 12 click specs). Fix (staged `petsrace`): drop column clear of the other pets' sides plus half the pet's width, retry (5 times) when the trail of THIS drop shows `slide/head` before `glide/chute` (any other outcome still fails at once), and `carry` looks the pet up again on every try.

### 4. Tooling: retry.sh / focus.sh turned every red verdict into 8 reruns (runfix)
`retry.sh` looked for `] dev|rehearsal: specs passed|failed` (the sharded gate prints `[e2e] dev-shared: shared failed (1)`) in a log named `e2e-course-${id#e2e-}-...` (the real name is `e2e-course-<run id>-course-test-e2e.log`): a failed wave was always treated as "ended before the specs", slept 60 s and started a new 60 min run (I killed the chain after attempt 1; pids 79993/79999, my own). Fixed in staged `runfix` with a shared `specs-finished.pattern`, the right log name, `RETRY_PAUSE`, and a law (stub driver: a failed wave ends after 1 attempt with its code; a run without a wave is retried; red on base).

### 5. Load, not a defect
The full run 1d1 spent 11 min in graph verification and 20 min in the Bazel test-e2e wait (load 55-79, 3 jobs behind the capacity gate); the capacity ask for the focused runs (8 GB) waited 42 min at 4 GB spare on p2. Nothing in the spec waits is wall-clock based except the 20 s expect timeout (all pets waits are in the pets' own time, `stagePasses` equals the layer's pacer).

### 6. Layered-home spec compares against a stale grid size (layoutfresh)
Rehearsal r1: `🥞️layered-home` "each nested grid pans under glass ... zooms into a child": preview 1440x845 against the grid frame 1440x839 measured earlier in the same test. The footer lists the other learners online and changes its height with them (4 learners share one proctor in the shared stack); `displacement()` already polls both boxes fresh, the size check did not. Fix (staged `layoutfresh`): `sizeGap()` measures both now inside `expect.poll(...).toEqual([0, 0])`; same assertion, retried within the expect timeout.

## Not done / open
- No full run with the staged specs: the gate runs live specs only; the proof of petsrace/layoutfresh/claimsctl is the focused run after the gate lands them (commands above).
- The graph claim defect (Finding 1) also threatens every other lane's `bun repo` while peers refresh the graph; it is the first thing to land.
- Mirror `e2e-cl` (library subtree, 16k files) dropped at the end; the unpacked trace extracts were deleted (the kept run directories `🗑️generated/nx/e2e/20261027T185955-96696`, `...T192338-27531`, `...T163224-82598` hold the original traces).
