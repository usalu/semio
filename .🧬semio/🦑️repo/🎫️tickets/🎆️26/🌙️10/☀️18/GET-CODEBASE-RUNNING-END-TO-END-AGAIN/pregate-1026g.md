# pregate-1026g (lane pregate-g, wave 1026g): cheap failures shifted left, CHECKPOINT 21:58 (cut by weekly usage 97 %)

Nothing applied to the live tree (every apply ran on mirror `pg-all`, sealed; guard clean). No official run started. Mirrors dropped at 21:58. Tools `C/pregate-g-tools/` (`env.sh` helpers, `rs.py` retargets a lane script to a mirror root, `laws.py` cross-cutting law runner with envelope fallback, `penv.ts` mirror-capable official envelope, `snap.py` CoW clone of a mirror, `tscg.py` tsc --strict over touched files (written, NOT yet run), `cmp.py`). Raw verdicts `C/pregate-g-tools/out/verdicts.txt`, law records `out/base.jsonl` (131 files, 25 red = baseline), `out/laws-g2.jsonl`, `out/laws-g3.jsonl` (partial, 9 of ~135 files when stopped; 0 fail).

Mirror base = live tree at 21:11 (after gate 6b, before gate G1). The live gate landed G1 (quietfc, purge) at 21:22 and plan-once at ~21:36 while I worked, so those three are already live.

## Per-batch apply / verify verdict (all on mirror, in this order)
| batch | apply | own verify | verdict |
|---|---|---|---|
| quietfc (+docs quietfc) | ok | 17 law files 0 fail (host-capacity followup/upgrade/estimates/sampling/gate/phases/handover, library mirrors, python oracles) | GREEN |
| verify2 journal-fix (+docs) | ok | journal 7/0, plan-reuse decision cycle 1/0, no leaked watchers | GREEN |
| verify2 vouch (+docs) | ok | 7/0 9/0 15/0 (the REAL journal law is skipped: no watcher binary in a mirror) | GREEN |
| toolsetup probes (+docs) | ok | witness 8/0, plan-reuse oracle 4/0 | GREEN |
| bench3 observer | ok (needs `rs.py`, script hard-codes the live root) | observer law 0 fail | GREEN |
| purge-g (+docs purge) | ok, 20 files | 9 suites + tickets oracle 18/0; only the 2 documented baseline reds | GREEN |
| dashlock (+docs) | ok, 11 files | digest 4/0, installation 1/0, tsc strict clean; official pin-law leg not exercised | GREEN |
| graphlat plan-once (+docs) | ok (needs `BATCH_ROOT=<root>`, `batch_generic.py` derives the LIVE root from its own location) | plan-once law 3/0, generations 27/0, protocol 47/0, coverage 61/0, yield 7/0 | GREEN |

NOT yet applied/verified (open): oracle2, conv-g, xplat3, xplat4 (platforms/freespace/corpus), obs-g fleet, setup-g, obsrows-g, route2, laws-g build-tools, gram3 index-chunks + churn, gram2 row-table, walk-g walks/footprint, sweep2, nxfix core/nx-patch, nxlock, census2, fdleak guard, store-g, jobsrv, rev3fix, disk-g (rootguard, unitcache, sweep, ledger), keys4 precision-rules, red-g (8 batches, inferfix RISK), the sealed flip, docs3 batches of those. Group law runs for G4 and later and tsc strict were not run.

## Group law verdicts
- G2 (snapshot `pg-g2`, 133 files vs baseline 131): 1396 pass / 59 fail (baseline 1371 / 71), 4 files fixed vs baseline. Two timeouts (leases `status` test 120 s, coverage `queue` tests 60 s) reproduced ONLY in that polluted mirror; the same test passes 8/0 in 3 s on `pg-all` (G2+G3 applied) and on a mirror of base + quietfc + journal-fix + vouch + probes. Cause not proven: suspect stale lease state left in the mirror by the killed tests of the first law run, not a batch. Rule: rerun a red on a PRISTINE clone (`grp` now keeps `pg-gNp`).
- G3: partial, 9 files 66 pass 0 fail (stopped by the checkpoint). consumer-cap after G3: generations/🟨️.mjs 62456 / 65536 (near, 0 over).
- Baseline reds in the law set (25 files, 71 fails: leases wait/fairness/retirement/standing workers, scheduling, unit-cache slow/worker, language corpus, pool-claims worker): all pre-existing on the mirror of the live tree, not batch effects.

## Conflicts and corrections for the landing order (from reading, not yet exercised)
- conv-g vs probes: both edit `🔭️witness/🟨️.mjs` + `.d.mts` docstrings. Probes landing first (as planned) makes 2 conv-g witness files drift; use the merged copies in `toolsetup-g-tools/mirror/afterconv`, or land conv-g first and then `probes-after-conv`. Needs the doc batch `probes` either way.
- gram2 index-chunks is superseded by gram3 index-chunks (gram2's pushes generations/🟨️.mjs to 65887 B over the 65536 B cap); run `rebase-g-tools/consumer-cap.py <root>` after every graph batch. gram3 `churn` after walks, row-table, index-chunks.
- nxfix core before nxlock before testmem2/census2 (census2 SUPERSEDES testmem2; fdleak guard after census2); the build guide: nxfix core replaces the whole guide, so either land it before every doc batch or use docs batch `nxfix-core` instead.
- setup-g before store-g; oracle2 before conv-g; walks before footprint; footprint and conv-g before rev3fix (`apply --accept-drift`); route2 before laws-g build-tools and testmem2; disk-g `unitcache` after jobsrv, disk-g `ledger` after nxfix core/census2/walk/obs-g.
- Scripts that hard-code the live root (bench3, graphlat, verify scripts using `cd /Users/ueli/...`, `batch_generic.py` ROOT from its location, stage.py of verify2/toolsetup ROOT from its location) must be retargeted: `rs.py` plus `BATCH_ROOT`, or copy stage.py into the mirror root.
- flatten-g converts staged trees to `files/fNNN` + manifest.json (same CLI): re-check each tool before use.

## Safe groupings (disjoint file sets, from the audit and the checks above)
G2+G3 as landed are safe together. Proposed next: {obsrows, xplat3, gram2 row-table}; {nxfix core, nx-patch last, conv-g after oracle2}; {route2, gram3 index-chunks, walk walks}; {setup-g, laws-g, walk footprint}; {store-g after setup, nxlock after core, census2}; rev3fix alone, flip alone and last (`flip.py status` after the last batch; conv-g must precede it).

## Exact next commands (nothing is running)
```sh
cd /Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export SEMIO_REPO=$PWD; python3 -I $C/mirror-tools/mirror.py create pg-all --only library process launcher .github .devcontainer   # ~90 s
. $C/pregate-g-tools/env.sh   # then edit M/T paths if the scratchpad moved; helpers: mrun, rs, docs, grp, log
python3 -I $C/pregate-g-tools/snap.py pg-all pg-s0   # baseline clone; laws: python3 -I $C/pregate-g-tools/laws.py <root> out.jsonl -j 3
```
