# Source Audit R8

Read-only audit of the integrated 210-element detail source. Read architectural-details-r7.md and detail-completeness-r7.md. No source files changed. Source line references identify the inspected snapshot. CMN architectural claims are inherited from the cited R7 reference inspection; this pass did not independently remeasure or reidentify the original plan.

## Confirmed Integration Defects

1. **Rooflight glazing renders as opaque wall in axon.** `🎨️visuals/🏛️savoye/🧊️axon/🐍️.py:433` chooses only post (stairs/railings) or wall (all details/furnishings). ROOFLIGHT-1, GL-GARAGE-1 and GL-GARAGE-2 are details with kind rooflight and MAT-GLASS, yet all receive wall faces at line 434. Painter line 478 applies glass transparency only to role glass. Plan line 300 and section line 383 already identify rooflights as GLASS. Dispatch rooflight parts to glass role in axon and assert material-role parity; otherwise the two garage skylights are opaque covers despite real slab holes and open planter rings.

2. **Generic detailed-object section/elevation representations return plan XY polygons.** `🧱️model/🐍️.py:854–860` creates rings directly from p.outline, then uses those same XY rings for section/elevation/axis/footprint/plan. FIX-HALL-TABLE-1, new basins, planters and stair guards therefore return no vertical coordinates for section/elevation through the public represent API. Native drawing providers separately project actual z0/z1 correctly. Introduce explicit projection semantics or return correctly projected section/elevation primitives; add a portable table/planter fixture proving vertical bounds. Runtime calls currently succeed but their successful return does not prove correct coordinates.

3. **Stair volume omits now-rendered landing slabs.** `🧱️model/🐍️.py:663` sums steps only, while represent lines 855–859 and axon line 431 include steps plus landings. Actual Python execution gives ST-GF 1.059546125 m³ vs sum of represented solids 1.1297461246459912 m³; ST-FR 1.006744823 vs 1.0769448227533331 m³. Each misses 0.0702 m³. Sum landings under the same quantity policy and test parity against represented occupied solids.

## Executed Evidence

Used ticket python-validation.mjs openValidator with actual CPython/Pyodide, NumPy and Shapely; no shell process or product dependency. All five independent detail tests pass: actual hollow/occupied samples, room/column containment and quantity parity, person/door clearances, connected rising guards/open arrivals, guest basin under rooflight. Checked quantities plus all five plan/section/elevation/3d/body calls on 101 columns/slabs/doors/stairs/part-based objects: zero exceptions. The representation coordinate defect above is semantic, not a thrown error.

## Provider Coverage and Assumptions

- Hollow basins/planters/tables are authored from multiple ordinary prisms, so each provider iterates the real component volumes; no unsupported polygon-with-hole part schema is assumed.
- Stair guards and tubes pass the independent occupied-volume and continuity checks and are included through railing parts in plan/section/axon. Stair landing omission from R4 is fixed in axon.
- Both integrated hall tables and their actual source columns pass detail placement/route checks. Chauffeur room and door approaches pass the new subdivision regression.
- Schema explicitly enumerates all currently used element classes, and element.additionalProperties is false. No unrecognized new detail class was found. Full JSON Schema evaluation was not repeated in this pass.
- Source has SL-1 FLOOR plus SL-R and SL-KIOSK ROOF. Axon chooses one principal ROOF but loops remaining slabs separately, so the kiosk slab is retained; orthographic plan and elevation loop all slabs. Roof planting takes the first roof with zones, which is presently SL-R. This is order dependent but no current omitted kiosk geometry was found.
- Off-grid bearing/footprints are owned by coordinator and were not duplicated here. No claim of native Manim render success is made by this audit.
