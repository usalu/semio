# Current Source And API Metadata Audit

Final frozen-source reread: all three reported metadata/API/display findings below are resolved. The chronology describes their discovery; current _bridge_parts includes every dedicated bridge part, lower0/1/2 counts follow actual podium datums without heads/vents, empty drawings are safe, and the podium description is corrected. Fresh galleries17:15–17:17Z were visually accepted; see final-architecture-gate.md. Native replay acceptance remains separate.

Read-only inspection of the shared Nakagin Python model/renderer, both clip binding JSON files, Grasshopper model adapter and GH/Excel scene call sites. No production files changed. Targeted searches replaced an overly broad teaching-tree traversal, which was cancelled without source effects.

## Findings

| Finding | Evidence | Action / status |
|---|---|---|
| Stale pitch prose | Shared JSON items.stagger note still referred to primary2.90 while value/core metadata use2.80 | Coordinator notified and reports note corrected |
| Empty-capsule partial-view edge | Visuals _tower_items sets levels=max(0,count−first_capsule_storey), then crown defaults to t.base_elevation when no capsule is selected; height=crown+roof_clearance | Public tower_axon(storeys=0/1/2) therefore renders a roughly11m core rather than only the requested ground/office count. Current Tower scene always adds podium count to at least one capsule storey, so this is an API edge, not a demonstrated current clip blocker |
| Stale podium description | podium_axon docstring says “simple massing with a ribbon of windows on every storey” | Coordinator notified; actual renderer uses detailed podium assembly with opaque upper bands/recessed strip glazing and recessed ground rooms |

No further actionable stale numeric datum was found in the targeted runtime model/renderer, consumer binding JSON or GH/Excel scene scan. Values7.5/2.75 that occur as animation durations, screen coordinates or core half-widths are not architectural datum defects. The native kit source title legitimately retains its historical2.75m reference pitch. The core logical-floor portable fixture2.9/8.32 tests an independent supplied pitch/base algebraically; it is not implicitly a claim about the current building.

## Verified Semantics

capital_base now returns the explicitly selected Tower.height. A partial Tower is therefore not clamped to the entire building's tallest capsule crown. The renderer computes each nonempty partial crown from actual selected capsule frames plus their nominal shell height, adds the same tower-specific clearance, and supplies that explicit height to the core and head assembly. _capital_items computes its translation from the same selected tower datum, so no remaining extra global-crown translation was found.

core_solid metadata levels now consist of the actual podium ground/office levels, nominal capsule floor datums derived from current podium height/current pitch, and concrete roof top, clipped to the selected height. Its levels are nominal floor datums, not every staggered entry sill. Actual local capsule landings and lift stops use authored centers/sublevels and the explicit floor-finish offset. _normal_lift_stops recovers the phase0 sill even where sparse floor3 contains phase1 entries only.

Grasshopper slot_tree reads podium.levels.2 dynamically and subtracts current storey_height. It uses each floor's actual slot plane/family; candidate facade/item indices provide tree alignment and culling, not a fixed claim that a candidate index always occupies one physical position. Crown-offset rows derive each actual floor's highest capsule roof. make_slot_capsule reads the shared detailed family geometry. aggregate_tree preserves actual ragged branches.

Both clip binding JSON files were inspected at the current roots. GH pitch bindings reference core.storey_height. Excel dimension/program/coordinate consumers remain authored against the shared model rather than hidden old absolute height constants. Scene screen coordinates and pedagogical local core variants are explicit display inputs, not historical building measurements.

## Limits

This is a source/API audit, not a fresh geometry/native-render pass. The final bridge footprint and coordinator datum edits remain owned by other agents. Final exterior/section/detail galleries and TowerEN will be inspected only after the coordinator signals their fresh completion. Earlier native18-scene results establish behavior on their own snapshots and are not silently promoted to geometry validation of later revisions.

## Proposed Public Lower-Storey Contract

storeys counts the two podium storeys plus capsule storeys. For0, emit no selected tower core/head/capsules. For1, terminate the concrete core exactly at shared podium.levels[1] (currently4.0m). For2, terminate it at podium.levels[2] (7.72m). A machinery head begins only when at least one capsule storey is included. The podium flag remains independent, so podium=True may still display the full podium while selected tower components are empty. Counts≥3 retain actual selected capsule crown plus tower-specific clearance; None retains full authored roof datum.

Hidden implementation impacts identified before a fix:
- _core_parts must not be called for count0; core_assembly can divide by selected storeys during shaft segmentation.
- Both ground creation and _compose use np.vstack over items. Empty selection must yield an empty drawing with valid view/semantic groups; a ground plane cannot be sized from absent geometry.
- core_assembly always emits roof vent/curbs above the selected core top, while _core_items LOD retains the vent. Lower-only partials must omit this protrusion to satisfy the exact4.0/7.72 maximum.
- Bridge filtering must see a zero maximum for zero-height selected towers and must produce no bridge until both selected cores reach a bridge level.

Independent portable assertions should obtain expected0/4.0/7.72 directly from authored JSON datums, collect raw mesh vertices with NumPy before screen projection, and compare maximum Z against those independent expected planes. Assert no capsule/head/bridge groups for all three lower counts; count0 with podium=False is empty and render-safe with either ground flag. Count0 with podium=True should reproduce exactly the independently generated podium geometry, not a shortened podium. Count3 should include precisely the authored sparse floor3 capsule frames and its actual highest roof, not a uniform storey-height substitute. A cached-copy render call should preserve these semantic counts and world bounds.

## Additional Bridge Display Omission

visuals._bridge_parts currently selects only roles bridge and bridge_rail from structure_assembly. The detailed bridge_base, bridge_diagonal, bridge_mast, bridge_crossbar and bridge_post geometry is therefore absent from normal tower_axon views. The coordinator was notified to include the full bridge semantic-role set or compose the dedicated bridge assemblies. Geometry tests of bridge_assembly alone cannot demonstrate that its physical details appear in consumer drawings.
