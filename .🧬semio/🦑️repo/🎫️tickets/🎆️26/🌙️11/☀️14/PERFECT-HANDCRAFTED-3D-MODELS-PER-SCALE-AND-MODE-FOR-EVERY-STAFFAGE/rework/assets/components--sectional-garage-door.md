# components/sectional-garage-door

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts components/sectional-garage-door --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ sectional-garage-door (1/1)`.

- Findings before: 1:100 hinge pins 12 mm; 1:200 cable studs 15 mm; 1:500 end stiles 30 mm, bottom rail, seal, top cap, all hinge leaves and pins, roller brackets, cable studs (limit 40 mm, the thinnest drawing of a panel). Measured: at 1:50 each sway brace lay 2.5 mm beside its rear hanger without a fastener (body and structure); at 1:200 the spring shaft group had no load path (bearing plates were no members).
- Remodelled (`K/rework-sectional-garage-door-coarse-scales.py`): 1:50 one M8 bolt per sway brace through both angle legs (body, thermal, structure joint). 1:100 hinge pins 15 mm. 1:200 cable studs 20 mm, bearing plates as structure members. 1:500: each section one 40 mm sandwich body over the full width with its stiles in it, bottom rail and seal one 45 mm shoe, top cap part of the top section, each hinge one 80 x 40 x 80 mm block across the joint that carries the roller stem, roller brackets 40 mm blocks, no cable studs.
- Sheets opened: 1:500 body `half-open` three-quarter. Rendered but not opened: 1:500 body default, 1:500 structure, 1:500 thermal, 2D 1:500. Not rendered: 1:50, 1:100, 1:200 (single small parts changed).
- Imperfect: 1:100 and 1:200 still carry the full hardware of 1:50 (74 body solids; they pass through the thinnest-drawing limit of the panel bones, 18 mm at 1:200); no lift cables; the open list of batch 19 stands (headroom, coaxial hinge pin and roller stem, free-standing frame). The 2D was not touched.
