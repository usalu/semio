# components/sliding-door

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts components/sliding-door --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ sliding-door (1/1)`.

- Findings before: 38 body solids at every scale; 8 below the feature at 1:100, 27 at 1:200, 30 at 1:500.
- Remodelled (`K/rework-sliding-door-coarse-scales.py`): 1:100 seals, end caps and brushes 15 mm, one 16 mm hanger nut, rosettes 10 mm. 1:200 (22 body solids): seals, caps, brushes and beads belong to jambs, track and leaf frame; one 20 mm floor seal; each wheel pair one 30 mm through roller; each hanger a 20 mm rod into its clevis; handles 20 mm bars without rosettes. 1:500 (9 body solids): jambs, track, leaf frame, glass, and each trolley with its hanger as one 50 mm block; no floor guide, seal, beads, handles.
- Sheets opened: 1:500 body. Rendered but not opened: 1:200 body, 1:200 structure, 1:200 thermal cut, 1:100 three-quarter, 2D 1:200.
- Imperfect: the concept defect named by batch 19 stands (the closed leaf stands in the middle of a 2.9 m opening and closes against nothing); the older decision sentences of 1:200 and 1:500 were kept and one sentence appended, so they still mention parts that are merged now; 2D not redrawn.
