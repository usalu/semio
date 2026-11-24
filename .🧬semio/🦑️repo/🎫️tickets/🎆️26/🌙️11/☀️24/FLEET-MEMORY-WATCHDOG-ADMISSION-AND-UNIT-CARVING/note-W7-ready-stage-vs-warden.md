# Note from lane W7 (fleet wave 1122-dist), 2026-11-24 08:50: the warden meets the Ready `workspace` stage

Nothing of the warden was edited. Observations at 08:12 to 08:45 (`repo fleet status`, `origins/*/memory.json`):

- Interventions s/r/c: g1-m10 15/13/13, g1-m02 6/5/7, g1-m01 4/4/4, g1-m08 4/2/4, g1-m07 3/2/2, g1-m06 0/0/1, g1-m04 26/26/0.
- The subjects it holds are the READ of the stage itself: `slice-worker` (bun `...🏡️resident/🏗️composition...`, m06 cancelled at pressure critical with 2,200 MB available, m08 suspended and resumed) and `pool-child` (bun `...🕸️graph/📥️inference...`, m01/m02/m10/m04 suspended and resumed 4 to 26 times).
- Consequence: a suspended slice worker shows no new line, the stage stall rule ("nothing new for 10 minutes") stops the whole stage and restarts it from the file inventory. 8 macOS workers had 140+ such stalls in 14 h (`ready-timings.json`), only 3 other failures.
- Landed on the ready side (my area): `readyWardenHoldsV1` in `sidecar/🚦️ready/🌲️process-tree/🟦️.ts`: the stall clock does not run while `currentHostWardenMirrorV1().suspended` names a pid of the stage tree. The hold stays bounded by your `suspendMaximumSeconds`.
- Asks for your side: (1) cancelling a `slice-worker` of a Ready stage costs the whole stage (exit "Slice worker exited with code null"); prefer suspend and a longer `suspendMaximumSeconds` for that kind, or carve the read so that a cancelled slice resumes from the taken owners; (2) the fleet status line shows per worker `last 12 h in this stage: N stopped as stalled` now, so you can see the effect of a change.
