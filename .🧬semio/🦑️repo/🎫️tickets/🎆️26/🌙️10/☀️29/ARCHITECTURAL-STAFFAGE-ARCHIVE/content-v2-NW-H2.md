# Content v2 report NW-H2 (skateboarder, kneeling-gardener, parent-carrying-toddler, seated-guitarist)

Status at 18:05 (in progress, updated as work lands). Scripts: `🧰️scripts/NW-H2/` (lib.py, human.py, art.py, common_build.py, `<asset>_rig.py`, `<asset>_art.py`, `<asset>_build.py`, watch.ts = long-running suitability watcher). Previews/diagnostics: `🗑️generated/NW-H2/`.

| asset | rig + IK poses | thermal/respiration | drawings | official check |
|---|---|---|---|---|
| skateboarder | done (10 poses, 3 clips) | done | 4 scales drawn, gap/feature fixes in progress | asset JSON written; check run 1: gaps and features failing, force residual fixed by quasi-static structure |
| kneeling-gardener | rig + 3 poses solved | drafted | 1:50 draft drawn | not yet |
| parent-carrying-toddler | rig (toddler as passenger bones) + gait generator | drafted | 1:50 draft drawn | not yet |
| seated-guitarist | rig + 4 poses solved | drafted | 1:50 draft drawn | not yet |

Known toolchain facts: the suitability measure works on visible (hidden-line-removed) outline runs and on white slivers; near-miss outlines between separate shapes (centre distance between pen and pen + 0.15 mm) are the dominant failure, so joints must overlap deeply and detail lines must stay clear of foreign outlines. Machine queue waits of 5 to 10 minutes per bun start; one watcher process re-measures on file change.
