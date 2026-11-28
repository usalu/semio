# H41: compiler image `r` (shard, size, retention, lease pin)

## Cause (measured 11-27)
- `r` fills because the unit collector skipped EVERY unit of a profile whose build lease compilers hold; on a busy worker the shared profiles (`b/p/...`, 2.3 GB per variant of one crate) are never free, so nothing was evictable and the image reached ENOSPC (m12, m11, m13...). Oversize images (40-96 GiB) cannot unmap; reborn ones are capped at 32 GiB.
- Working set of `r` per worker (content = scan bytes less `fixed.dmg`, `🧰️scripts/H41/measure-working-set.py`, scans partial so lower bounds): m01 26.0, m02 4.7, m03 13.9 (37.3 an hour earlier), m06 14.6, m07 7.4, m08 32.3, m10 4.9, m11 12.4, m12 27.4, m13 17.6, m14 23.5, m15 12.6 GB; median 13.9, maximum 32-37 GB. Only m03/m08 peaks exceed one 32 GiB image; most of the set is stale variants.

## Landed (05:05, see landings.md)
- `🧱️units`: a held lease no longer pins the profile. `unitsCoveredV1` = newest 2 variants per crate + units used within 6 h (`activeBuildMilliseconds`, schema). Evidence of use: Cargo `-Z mtime-on-use` touches the fingerprint of every planned unit (fresh ones too; verified in a scratch crate), so last use now reads every fingerprint file; both cargo launch sites set `CARGO_UNSTABLE_MTIME_ON_USE=true`. Uncovered units of a busy profile go under their own unit lock, no exclusive lease; a unit touched between measuring and removing stays. Lane-live profiles stay fully pinned (the proc-macro2 incident test unchanged).
- Per-image retention: `planUnitsByImageV1` / `UnitImagesV1` (need, budget, target per image); `measureUnitImagesV1`, `worstImageNeedBytesV1` in gc; the watcher and the collector act on the fullest image, not only the sum (active as soon as shards exist; today every worker is one image so behaviour = aggregate).
- Sizing: `imageShardPlanV1` (shards = ceil(working set / 87.5 % of the cap), per image aligned, host-free refusal) with the measured corpus.
- Tests (never weakened; 3 old expectations of "held profile pins everything" replaced by the new rule): units 19/0 with Python oracles for plan, cover, images (+ 300-400 seeded random states each); gc 15/0; budget shards 5/0 (oracle for sizing); auto-reclaim 13/0.

## Not done
- `.cargo/config.toml` NOT edited: it is a global graph identity and unit descriptor input (a change would re-read the graph and could re-key units); env var used instead.
- Lane-birth sharding in the pool and retirement of a lane's shard, remount after reboot (`repairNativeImageShardsV1` is unused), relayout while builds run: not wired. The shard rule for `r` is enabled in policy but no worker has a shard; relayout only runs when the image is quiescent. Needs its own lane (lifecycle under live builds cannot be proven without live experiments).
- Residual risk: a cargo launch outside the two patched sites (none found) could lose a non-newest unit idle >6 h that it reuses fresh (E0463, retried).
- Pre-existing flake: `explicit reclaim retires lane units...` in the units test fails when run alone (9/10 on the untouched tree), passes in the file run.
- Typecheck/dangling-imports result and the m11/m12 watch: see the final lines below.
- Landed 05:05; typecheck of product files 0 errors (test file type errors fixed forward, 0 now), dangling-imports 0. Watch 05:15: m11/m12/m13 r free (MB) before: 28831/16104/8773 of 40959 (see ready logs); not yet changed (workers restart cargo on next stage). Baseline for the next look.
