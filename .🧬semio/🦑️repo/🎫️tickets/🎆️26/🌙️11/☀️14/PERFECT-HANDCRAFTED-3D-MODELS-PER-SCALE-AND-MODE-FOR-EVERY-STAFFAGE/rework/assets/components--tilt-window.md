# components/tilt-window

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts components/tilt-window --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ tilt-window (1/1)`.

- Findings before: 1:100 hinge leaves, stay plate and pin 8 mm, grip 12 mm; 1:200 these plus stops 25 mm, beads 18 mm, hinge knuckle 16 mm.
- Remodelled (`K/rework-tilt-window-coarse-scales.py`):
  - 1:100: each hinge its 16 mm knuckle alone, stays 15 mm plates with 15 mm pins, grip 15 mm.
  - 1:200: stops 30 x 30 mm, knuckles 30 mm, stays plain 30 x 30 mm bars without the slot, stiles and rails plain 70 x 90 mm sections that take the beads in, one 30 mm grip bar on the top rail, the stay pin a joint block of the structure.
  - 1:500 (3D only): as 1:200 without stops, grip and pin joint; mass marker 75 mm.
- Sheets opened: 1:200 body (closed), 1:200 body `fully-tilted` three-quarter. Rendered but not opened: 1:500 body, 1:100 three-quarter, 1:200 thermal cut, 1:200 structure, 2D 1:200.
- Imperfect: the open defects of batch 19 stand (no setting blocks; the EPDM gasket only at 1:50); the reveal lining of 0.30 m depth is the same four boxes at every scale; 2D not redrawn (not needed by the silhouette rule).
