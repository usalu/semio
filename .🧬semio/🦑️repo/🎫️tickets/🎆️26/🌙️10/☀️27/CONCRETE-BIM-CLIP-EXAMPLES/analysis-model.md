# Analysis Model (`C/🧱️model/🧮️analysis/🐍️.py`)

Written by the ANALYSIS-MODEL agent (a5410a3373c26ab4e); saved by the coordinator because the subagent could not write report files.

## Exports

`translate_structure`, `tributary`, `clash_check`, `link_values` (used by the core's `consistency_issues` / `resolve_link` and by the JSON builder). Stdlib only; the core is reached via `importlib.import_module` at call time; the repo root must be on `sys.path`. Self-check: `T/analysis-selfcheck.py` ("ALL OK").

## Decisions

- Mesh: 24 shells, 35 slab nodes at the mid-surface z = 3.375 m (first floor +3.50, slab 0.25 m). The ramp void (x 7.6–10.0, y 2.8–14.5) does not align with mesh lines and is not meshed: analysis gross 408.5 m², void 28.08 m², net 380.42 m².
- `nodes`: 60 = 35 slab + 25 base (`role` slab/base); `counts` gives each number.
- Members: 25 `IfcStructuralCurveMember` (3.375 m to the mid-surface; physical piloti height 3.25 m) and 24 `IfcStructuralSurfaceMember` (one per shell), each with `source_guid`; no IFC relationship named.
- Supports: 25 at base nodes, `condition` string + `label {en,de}`.
- First-floor loads: self-weight 6.25 (25 kN/m³ × 0.25 m) + finishes 1.5 + imposed 2.0 = 9.75 kN/m². Roof reactions enter as `point_loads` at slab nodes.
- Roof load (uniform version): concrete 5.00 + waterproofing 0.20 + screed 1.32 + soil 1.80 + imposed 1.0 = 9.32 kN/m²; demo soil event (0.30 m) → 12.92 kN/m². Densities from materials.
- Tributary cells: midlines and slab edges (west/east cells include the 1.25 m cantilever), clipped to the outline, cut by voids. 4 corner / 12 edge / 9 interior. First floor: B3 15.53 m² / 151.5 kN, A1 8.61 m² / 83.9 kN, total 3709.1 kN. Roof: net 270.21 m², total 2518.4 kN.
- `source_revision` = revision of the structure partial model (soil event → 2; opening event leaves 1).
- Losses: 76 per-element entries from JSON rules + `loss_groups` (12). No ramp handrail in the model → ramp and flights used.
- Assumptions: JSON ones + computed `linear-elastic` and `loads` (localized numbers).
- Clash: exactly one: DP-1 (x 1.40, y 0.35) vs A1, clearance 0.181 m < required 0.25 m (assumed); other downpipes 0.45 m. `include_passed=True` adds nearest passing partners.
- Link values: `slab-load {thickness, area_load, total_reaction}`, `roof-load {area_load, total_reaction, reaction_<tag> ×25}`, `piloti-section {diameter, height, cross_section_area}`. Soil event → `stale-link` + `stale-value`; `resolve_link` clears; `OpeningResized` → none.

## Invariants For The Tests

1. 24 shells, 25 supports, 35 slab + 25 base = 60 nodes, 25 curve + 24 surface members; unique node ids, every referenced node exists; slab nodes z = 3.5 − thickness/2, base z = 0.
2. Shell areas sum to 21.5 × 19; `slab.net_area` = `quantities("SL-1")["area"]`.
3. Every `source_guid` is 22 characters and exists; curve → piloti GlobalId, surface → slab GlobalId.
4. `total = self_weight + finishes + imposed`, `self_weight = density × thickness`.
5. Both levels: cell areas sum to net slab area; `total_reaction = area_load × slab_area`; each reaction = `area_load × area`; polygon area of `pieces` = `area`.
6. A 0.025 m point-sampling grid reproduces every `area` within 0.01 m² (NumPy in tests).
7. Soil 0.10 → 0.30 m raises the roof load by 3.6 kN/m² (uniform version) and the total by 3.6 × 270.21 kN; first-floor tributary unchanged. (Changes if the zoned roof load is adopted — see below.)
8. `clash_check()` returns exactly DP-1/A1 with distance < 0.25; moving DP-1 to x = 1.7 on a copy yields none.
9. `link_values` returns numbers only; a fresh model with filled links has no issues.

## Notes For Scenes

- Roof scenes must call `tributary(model, level="roof")` (default "first").
- Row C is a poor choice for roof arrows (C4 0 kN, C3 22 kN, C5 13 kN; C2 176 kN, D2 191 kN) because the roof terrace is a void of the roof slab. Pick tags from the data.
- Say "24 surface members" and "35 slab nodes". `translate_structure()` ≈ 1.5 s, `tributary` ≈ 0.35 s: cache them in scenes.

## Zoned Roof Load (adopted)

- Zones: `solarium` (SP-SOLARIUM, 70.00 m², concrete 5.00 + waterproofing 0.20 + screed 1.32 + imposed 2.0 = 8.52 kN/m²) and `planted` (200.21 m², + soil 1.80 + imposed 1.0 = 9.32 kN/m²). Roof total 2462.357 kN over 270.21 m², mean 9.1128 kN/m².
- `tributary(model, level)` adds `zones:[{id, name{en,de}, area, area_load, load_breakdown, share, reaction, outline, outlines, holes, pieces}]` and per piloti `zones{id:{area, share, reaction, pieces}}`; top-level `area_load` is the area-weighted mean. `roof-load` link keys: `area_load`, `planted_area_load`, `solarium_area_load`, `total_reaction`, `reaction_<tag>`.
- Invariant 5: piloti reaction = Σ q_zone × zone area. Invariant 7 (replaced): soil 0.10 → 0.30 m (+3.6 kN/m²): planted 9.32 → 12.92, solarium 8.52 unchanged, mean 9.1128 → 11.7801; each reaction changes by 3.6 × its planted cell area; total by 720.76 kN; only C4 unchanged; first floor unchanged. Invariant 11: zone areas sum to the roof net area; outlines − holes = area; point sampling reproduces 200.21 m². Invariant 12: solarium breakdown has no soil; link values numbers only.
- Visible soil effect: rows A and D/E (D3 +41.1, E2/E3/E4/A3/A4 ≈ +40.6 kN); row C small (C1 +8.6, C2 +1.7, C3 +8.5, C4 0, C5 +5.1).
- The JSON `links[].used` must be refilled by re-running `T/model-build-json.py`, otherwise a fresh model reports `stale-value` on `LK-ROOF`.

## Follow-ups

- MODEL: JSON assumption `mesh-grid` text should use `{shells}`, `{slab_nodes}`, `{base_nodes}` placeholders.
- Coordinator decision: adopt the zoned roof load (soil only on the planted roof-garden zone, solarium imposed 2.0 kN/m²) so the soil event changes the reactions of the pilotis under the garden — requested from ANALYSIS-MODEL.
