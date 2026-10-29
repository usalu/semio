# Stair Parameter Audit R53

Read-only source audit,2026-10-29. No source changes. Current source was freshly reread after R52.

## Current Truth

ST-CG,ST-GF,ST-FR already use kind compensated_u, registered common centre(6.4071448289,9.1802203723), outer tread radius0.89m and inner radius0.08m. All three have explicit step_parts and structural_parts. Step counts13/18/18 and actual continuous U-waist/tread geometry are qualified as inferred in source evidence. Names and historical evidence already describe compensated U stairs. No stair type reference is present. Guard products use compensated_stair_guard and handrails compensated_stair_handrail, with explicit U names. Therefore this audit does not establish that current geometry remains a full helical stair.

Each stair retains start_angle−90 and turns0.5. These describe a half turn if interpreted numerically; they are not retained full-turn1 values. However they are unused for the actual explicit-part construction.

## Exact Producer and Schema Obligations

🎬️clip/🧱️model/🐍️.py:1517–1518 places actual step_parts. Only the fallback1519–1524 uses start_angle/turns/centre/radii to synthesize evenly spaced circular wedge tread blocks with a fixed0.12m underside. That fallback does not express the current straight trains, compensated return, finite waist, inferred finish bedding or structural_parts. It is obsolete alternate construction and should be removed from the greenfield explicit-part contract. No compatibility branch is needed.

The actual public return at1528 still copies centre/radius/inner_radius. Those parameters retain legitimate meanings as U-return curve controls and cross-check references. Their continued presence is not evidence of a helix. The geometry source is the authored parts; the controls are not used to regenerate or override them.

Schema 🎬️clip/🧱️model/🧬️schema/🔣️.schema.json:2003–2013 requires kind,centre,radius,inner_radius,from_level,to_level,step_count,start_angle,turns,landing_parts. It does not require step_parts or structural_parts in the IfcStair-specific branch, despite actual construction relying on them. Generic property definitions contain step_parts at1598 and structural_parts at1873. The class-specific schema should require explicit parts and remove the obsolete angle/turn fields with the fallback; update global property declarations if unused elsewhere. A constrained compensated_u kind is more descriptive than a loose geometry kind.

## Consumer and Test Impact

No public plan/section/elevation/axon scene was found reading stair start_angle/turns. These routes consume actual steps/parts. The exchange chapter centre access584 is a selected COLUMN mesh, not a stair consumer, so it is irrelevant. Axon view turns rotates the camera in quarter turns and must not be removed by stair cleanup.

Legitimate U-return tests consume the curve controls: suite.py77–80 uses centre and the average tread radii to place the final-train arrival route; details.py92–99 uses the centre/radius to sample straight and return guard faces. Cellar contract checks the registered centre. Those should retain the physical U controls or move them into a clearly named return descriptor; deleting them without replacing the actual test reference would lose useful independent checks.

placements.py36 authors start_angle/turns in a fixture that already has explicit step_parts and landing_parts. Remove those obsolete fixture fields with the explicit-parts contract. Its material-placement purpose should continue to pass unchanged. suite.py100 retains a test name test_helical_tubes_have_connected_physical_sections_and_nominal_volume even though current handrails are compensated U; rename it to match U handrail reality. Core visual material lookup contains an unused helical_handrail key at58; no current product uses that kind. It may be removed if no separately valid product contract owns it; do not replace compensated_stair_handrail with a legacy alias.

## Recommended Portable Regression

Use explicit compensated-U native parts without start_angle/turns; require matching schema validation and identical part volumes, bounds, treads, waist and handrail occupancy. Independently compare union of actual tread polygons with the expected straight-plus-half-return fixture using existing GEOS. Missing step_parts/structural_parts must fail rather than silently manufacture circular wedges. Test that source kind/name/evidence describes U topology while independently preserving centre/radii for the return. No product or test was edited in this audit.

## Complete Portable Fixture Follow-Up

Searched all current model-test Python files and JSON fixtures. The only synthetic IfcStair source construction found is placements.py36: it has step_parts and landing_parts but no structural_parts, with kind cellar_stair and obsolete angle/turn fields. Update it to an explicit structural-parts contract; no fixture was found that intentionally requires circular-wedge fallback. cellar.json86 retains stair_turns0.5 as a fixture datum; remove if unconsumed. parking.json IfcStair records are frozen occupied obstacle entries, not generated source stair definitions, so their class identity alone is not an obsolete fallback dependency.
