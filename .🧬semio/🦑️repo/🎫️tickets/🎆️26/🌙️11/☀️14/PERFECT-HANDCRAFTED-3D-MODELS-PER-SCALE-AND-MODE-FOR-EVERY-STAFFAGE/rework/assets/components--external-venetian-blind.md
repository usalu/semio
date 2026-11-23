# components/external-venetian-blind

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts components/external-venetian-blind --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ external-venetian-blind (1/1)`.

- Finding: the 12 x 16 mm edge spacer ring of the sealed unit, carried from 1:50 into 1:200 and 1:500.
- Remodelled (`K/rework-external-venetian-blind-coarse-unit.py`): 1:200: no spacer ring; the unit is two 4 mm panes around one cavity body to the glass edge (1.13 x 1.18 m, gas space and edge seal as one). 1:500: no spacer ring; the cavity stays the gas space (1.106 x 1.156 m), the 12 mm rim lies in the frame rebate. Thermal follows (`th-edge-spacer` gone, `th-cavity` per scale); one decision sentence per scale rewritten.
- Sheets opened: 1:200 body, 1:200 thermal cut, 1:200 2D, 1:500 body, 1:500 thermal cut. Rendered but not opened: 1:200 and 1:500 structure (unchanged lists), 1:500 2D.
- Imperfect: 1:200 and 1:500 still carry the 18 slats, the segmented lift tapes and the ladder cords of the fine scales (they pass as `texture` / `slender`); they were not re-authored. The 2D of both scales was not changed (the spacer was not drawn there).
