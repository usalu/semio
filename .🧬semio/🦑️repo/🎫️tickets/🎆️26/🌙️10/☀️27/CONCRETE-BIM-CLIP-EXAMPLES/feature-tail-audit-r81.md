# Remaining Feature Tail Audit R81

Read-only diagnosis while the112-method owner run continues unchanged. Inspected actual current tail failure records, independent retained fixtures/reports and selected native slab measurements. No producer/source/test edits. Retained short measurement input feature-tail-audit-r81.py and generated areas.

## Wall Fixture Host Selection

Five wall methods fail in shared `_fixture` before exercising geometry because its opening filter directly indexes geometry.wall for every IfcOpeningElement. Current schema legitimately supports wall, slab, curtain-wall and other hosted opening branches. Narrow fixture correction is geometry.get("wall") when selecting only W-S-hosted openings to remove; preserve all unrelated opening host types. Do not delete other-host openings broadly or modify producer schema. The synthetic W-S aperture/material/axis/STEP assertions remain unchanged and need real rerun after fixture repair.

## Facade Region Set

Current W-S/N/E/W each intentionally contain parapet,shelf,upstand,screen; test expects three and omits shelf. This is corroborated by independently retained facade-structure-r23.md and facade-auxiliary-audit-r23.md:20cm lower parapet construction band,6cm RC shelf,14cm upstand, separate shelf footprint and inferred width. Therefore adding shelf to the expected named domain is warranted, with existing thickness/cavity/gross-volume and bounding/occupied inequalities retained. Stronger fixture validation should also retain actual6cm shelf depth and explicit material ownership rather than only count/name acceptance. No evidence of a spurious fourth source region was found.

## Measured Slab Scope

Fresh native measurements:

| Slab | Expected Physical Area | Exclusive Area | Full Area | Shared Area |
|---|---:|---:|---:|---:|
|SL-1|384.059561757115|384.059561757124|384.059561757124|0|
|SL-R|267.205180602007|267.108380602012|267.205180602012|0.0968|

Roof deficit0.0968m² is exactly two0.0484m² solid column-joint footprints, not missing physical roof material. Full volume46.73359966416085m³ matches quantity volume46.733599664; exclusive volume46.71286686416085 differs by shared0.0207328m³. The failing method checks outline-minus-resolved-void physical coverage and full quantity volume against exclusive parts. Use full_parts for its measured physical depths/tops/soffits, area and volume, retaining1e-7sumarea and1e-9containment/pair thresholds; independently conserve owned+shared and ensure only expected declared joints account for difference. Current source reserves are solid10/15; future sleeve holes or partial-height cuts need z-aware expectations. No tolerance adjustment or removal of joint ownership is justified.

## Newly Reached Joinery Failure

The failure artifact now additionally contains `test_source_glazed_joinery_parts_and_quantities_agree_with_geos` expecting exactly8sash pieces for D-ENTRY. Current D-ENTRY uses TY-ENTRY-TIMBER, while this hardcoded tail came from R25 original two glazed entrance sashes. Its preceding source loop selects glazed types and independently verifies quantities/nonoverlap; retain that proof. Do not force opaque/layered timber door back into an obsolete glass-sash count. Independently validate the actual current entrance leaf/frame/skin/material roles, open passage semantics and material conservation against the retained timber-door source evidence/fixture. This newly reached failure was observed while112tail still active; it is not an exhaustive final failure inventory.
