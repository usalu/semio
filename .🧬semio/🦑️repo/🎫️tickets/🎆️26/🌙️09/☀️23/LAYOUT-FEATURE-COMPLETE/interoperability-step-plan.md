# Layout STEP Content Work

The page-only STEP sheet is an open user-visible gap. The next implementation must emit visible layout content, retain it through canonical Part21 and independent readers, and make the editor STEP job progress/cancel without building the whole file in one turn. No general IFC schema work is involved.

## Primary Entity References

STEP Tools publishes the EXPRESS definitions and application-protocol memberships directly. Confirmed AP214 entities: [geometric_curve_set](https://www.steptools.com/stds/stp_aim/html/t_geometric_curve_set.html), [geometrically_bounded_wireframe_shape_representation](https://www.steptools.com/stds/stp_aim/html/t_geometrically_bounded_wireframe_shape_representation.html), [text_literal](https://www.steptools.com/stds/stp_aim/html/t_text_literal.html), [text_literal_with_extent](https://www.steptools.com/stds/stp_aim/html/t_text_literal_with_extent.html), [annotation_text_occurrence](https://www.steptools.com/stds/stp_aim/html/t_annotation_text_occurrence.html), [text_style](https://www.steptools.com/stds/stp_aim/html/t_text_style.html), [text_style_for_defined_font](https://www.steptools.com/stds/stp_aim/html/t_text_style_for_defined_font.html), and [draughting_pre_defined_text_font](https://www.steptools.com/stds/stp_aim/html/t_draughting_pre_defined_text_font.html).

Important constraints: bounded wireframe polylines have more than two points; a straight two-endpoint path therefore needs an intermediate point or a proper trimmed curve. Draughting predefined text font names start with ISO 3098. Text literals carry literal/placement/alignment/path/font; extent adds a real planar extent. Annotation occurrences require presentation style assignments. The merged-schema draughting_model definition notes AP214 differences, so AP214 item constraints must be checked rather than assuming the merged select applies unchanged.

## Landed Foundation

Canonical Part21 writer now exposes owned header/one-instance/footer functions in Rust and TypeScript, sharing existing syntax escaping and value encoding. The neutral stream witness covers Unicode, a complex SI length-unit entity and a polyline. A native Ruststep differential law is authored but unrun. The actual TypeScript source-backed check passes one fragment law with zero target diagnostics against 194 real source files; three generated modules self-clean. Retained input: part21-writer-check.mjs.

## Implementation Sequence (Source Landed For Items1,2,4,5)

1. Schema-first owned draughting primitives in canonical STEP IO: named paths, positioned text annotations and contexts/units. Emit mm coordinates from typographic points with real SI millimetre/radian/steradian units. Retain AP214 page solid plus its representation context.
2. Layout frames resolve parent overrides and layer visibility. Rectangle/image bounds become rotated visible geometry; text becomes actual text literals/annotations. Asset pixels have no chosen native STEP encoding; export must explicitly diagnose or refuse unsupported pixel semantics, while providing meaningful placement geometry and asset identity.
3. Background Drawing geometry maps through its neutral owned Scene: lines/circles/paths and nested transforms. Extend canonical projection to geometric curve sets/polyline/trimmed primitives where necessary, then consume it on Layout import.
4. Bounded job source stage emits one frame/path/text fragment per turn through canonical writer parts, with exact retirement owners and no partial publication. The current STEP Kind::Step encoded bool/65536-byte whole-sheet shortcut must be replaced.
5. Neutral mixed-content fixtures plus Ruststep graph roundtrip and actual Three geometry oracle establish real content, rotation and unit outputs. This remains open until the new content mapping and job path are implemented.

## Current Result And Remaining Work

Items1,2,4,5 now have schema/portable witnesses and real native source: selected-page mm B-rep/context, bounded canonical entity dispatch, parent/body geometry and annotation mappings, editable rectangle/text import, appearance-preserving trace and native direct/session/independent-reader laws. Native execution is unavailable as documented in native-runner.md. Actual TypeScript physical and text/appearance laws pass with targeted strict checks; exact counts are in interoperability.md. The previous whole-sheet encoded shortcut has been removed.

Item3 now uses the actual bounded Drawing lower_step/current_leaf/current_style interface, with numeric ancestry and no complete scene/image/string copy. See interoperability-step-background.md for supported geometry and remaining complex-fill/typography/raster limits. Font substitution, typography flow, raster pixels/alpha and general CAD/PMI/assembly semantics remain explicit limits.
