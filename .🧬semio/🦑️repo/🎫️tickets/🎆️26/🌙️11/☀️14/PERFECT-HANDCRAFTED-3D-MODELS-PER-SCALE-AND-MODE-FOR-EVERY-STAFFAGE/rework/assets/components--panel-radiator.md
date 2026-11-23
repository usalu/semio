# components/panel-radiator

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts components/panel-radiator --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ panel-radiator (1/1)`.

- Findings before: 1:100 side covers 10 mm, top plate 7.5 mm; 1:200 the whole 1:50 list (22 mm pressed panels and headers, covers, grille rim and plate, nuts, nipples, pipes, five thermal bands). Measured: the nipple joints of 1:50 and 1:100 ended 6.6 / 6.1 mm from the panels (open load path of the front panel), the valve joint 3.3 mm short, the valve set of 1:100 9.5 mm from the radiator.
- Remodelled (`K/rework-panel-radiator-coarse-scales.py`):
  - 1:50: nipple joints through both headers, headers as members, valve joint to the upper nipple (structure only).
  - 1:100: covers and top plate 15 mm; tee body between the lower headers that the valve set meets; structure as 1:50.
  - 1:200: each water panel one flat 30 mm slab over the full height (headers in), 30 mm side covers, one 30 mm top plate with the sixteen slots of the 1:200 plan drawing, nuts and nipples part of the connection block, the two pipes one 30 x 80 mm pair body; thermal front bands as 30 mm slabs.
  - 1:500 (3D only): as 1:200 without covers and slots, connection one block, one bracket rail per side, nipple joints as blocks.
- Sheets opened: 1:200 body, 1:200 thermal cut. Rendered but not opened: 1:500 body, 1:200 structure, 1:100 three-quarter, 2D 1:200.
- Imperfect: the 2D of 1:200 was not redrawn (silhouette rule passes: the outline did not move); the convector fins stay the folded texture sheet of the fine scales at every scale; areas of the flat front bands are about 10 % under the pressed sheet (inside the rule's band, `areaM2` unchanged).
