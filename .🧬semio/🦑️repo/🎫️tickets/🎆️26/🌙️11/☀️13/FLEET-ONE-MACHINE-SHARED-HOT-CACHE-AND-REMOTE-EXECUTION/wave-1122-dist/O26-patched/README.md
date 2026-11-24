# O26 patch set for the play clone (COMBINED O24 + O26, one restart)

Clone: `/Users/ueli/.sinc/play`. The clone has patch sets up to O23; **O24-patched is NOT applied and must not be applied separately**. This set contains O24 (component jobs on LOCAL and FLEET slots) plus O26 (the storage gate), so the play chain restarts ONCE.

## Files (`list.tsv`, sha256 pre-image of the clone now -> post-image)
- modified (pre-images checked by `apply.sh`): job-slots schema, job-slots runner, component-pass driver, driver-stages (the O24 files) with the O26 edits in the component-pass driver.
- new: `storage-gate.ts` + `storage-gate-schema.json` (member `component-pass/💾️storage-gate`), its test and Python oracle, and the O24 fleet test and oracle.
- `edit-o24.py` + `runner.part.ts` + `fleet.part.ts` (O24) and `edit-o26.py` (O26) are the anchored edits; `build.sh <clone> <scratch>` regenerates every file, `.patch` and `list.tsv` from the clone's CURRENT files (the clone is only read); `apply.sh <clone>` verifies every pre-image (refuses on any mismatch), writes through `.o26tmp` + rename in list order (schemas and the new member first, the entry file last), idempotent (second run 0 files; verified twice on a scratch copy, all four modified files parse).
- `preflight.sh <clone>`: read-only check that the new run will not refuse release-00-b0 (below).

## What O26 changes (driver loads it once, so it takes effect at the restart)
1. The gate charges a running pass `max(0, predicted - written)`, written = output bytes of its finished units (the image's free space already shows them), re-read every 5 s (the old code woke only when a pass settled and charged the whole prediction for the pass's life).
2. A batch's prediction counts only the units no earlier batch of its lane (profile + target, one shared build directory) accounted; shared dependency units count once.
3. Bytes already on disk from the run this one replaces (unit outputs of the profile in the shared build directory, `deps` and `build/<pkg>/<hash>/out`) are credited against the batches in planned order.
4. Learned growth: every finished batch appends what its compiled units wrote to `components/storage-growth.jsonl`; with two rows the p90 replaces the computed footprint.
5. When nothing runs and the room is still short: collector, then the grow step (law and hook only; see report: no online resize exists). The wait line states needed/charged/free and what releases it, EN + DE.

## Restart cost (O24's numbers) and the pre-flight
- About 24 min of planning before the first rustc is repeated; the shared build directory, the unit cache (1,171 reused units) and the stage cache are reused; units in flight (4 local rustc, a remote final up to 17-40 min) are lost; counters restart at 0/73.
- The old risk was that b0's whole 51.3 GiB prediction plus the 2 GiB reserve does not fit the 45 GiB free of the 80 GiB image, so the new run would refuse b0. With the credit the gate wants predicted less what the lane already holds. Run before restarting:

```
./preflight.sh /Users/ueli/.sinc/play
```

  It printed (14:50): 45,677 MiB free; b0 predicted 52,573 MiB; recorded lane content 30,226 MiB; the gate wants 24,395 MiB (54,621 MiB without the O26 credit): OK. If it says REFUSED, free room first; do not restart.
- Apply: `O26-patched/apply.sh /Users/ueli/.sinc/play` (the play session does this; nothing was written to the clone by lane O26).
