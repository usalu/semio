# `machinery/heat-pump-outdoor-unit`

## 2026-11-22 23:00 to 24:00 — rework of the invalid originals (umbrella lot machinery)

Fan blades declared `sheet` at 1:100, 1:200, 1:500 (`rework-heat-pump-outdoor-unit-blades.py`); five `volumes-connected` findings closed by modelling (`rework-heat-pump-outdoor-unit-joins.py`): pilot valve on the reversing valve, hinge load path, expansion valve bracket, grille wires into the cap, four 30 mm spokes at 1:200. Before `INVALID … validateAsset 3`.

Result: source and catalog file `VALID … schema 0, validateAsset 0, checkVolumes 0` (full proof with `--volumes`), check CLI `✓`, published with `rework-machinery-publish.sh`. Findings before, what was remodelled per scale, rejected attempts, sheets opened and what stays imperfect: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-machinery.md`.
