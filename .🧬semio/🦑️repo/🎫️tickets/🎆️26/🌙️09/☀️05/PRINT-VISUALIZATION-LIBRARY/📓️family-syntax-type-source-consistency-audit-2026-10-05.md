# Family Syntax/Type Source Consistency Audit

Current schema SHA: 855CDD6B98CBBA96308C3184A32AF39E011779546ABCA1F38B75DEA518DEDCE1.

Bounded scan found 29 contradictions. The attached authored source-contracts.json records every exact family/key, current descriptor, desired primitive type/syntax/reset and actual source setter/consumer evidence. This is read-only source analysis, not a runtime pass.

The eleven numeric root indices require scalar syntax; identifiers are not arbitrary node names here. Six padding controls evaluate one floating-point value for four sides, rather than a text record. Named places/grid/points values require identifier syntax and retain string type. Boolean tier-title/labels/filled/raster retain scalar syntax and require boolean type. Geo mode-specific overrides occur after common reset/parsing and must not be confused with reset defaults.

semio-viz-network.sty — 75B03BE67391F39304F5AB1DEE06013B9B6C23D49F9ECECC5DA11B0E8A82697A
semio-viz-network-graph.sty — E7E70BD06B1FA0C720423C58EFEE3537867394BF9FDC2B28545246C46016F4E7
semio-viz-diagram-architecture.sty — 7D4F4191A32852BCEB324795206289E0932B3E345D5E25EB999A50D51DCB2BD9
semio-viz-geo-map.sty — 01BC451D6FF129FD92F89F3CC18F8E239D917CDC84210B249CF90F5DE79748BB
semio-viz-geo-contours.sty — ED810FF0CF1A7CC3ADF2E0CDDCC8FBB479774C53DD596FA8DACB3F3C39802467
semio-viz-geo-symbols.sty — 72E4EDFE02E5C85C0087C862231E156C84C9ABC2B46B8869BCF2D86A914CE0EF

Bounded extension checks non-scalar classes against actual inherited numeric setters: geo-terrain and spatial-scalar-field cell are fp numbers (expression syntax can remain to preserve TeX arithmetic), three inherited levels are integers, and spatial-scalar-field row is an integer inherited via terrain. Six additional exact rows added to source-contracts.json (35 total). This does not classify legitimate TL expression/empty controls from spelling.

Encoder boundary correction: actual fp/int setters evaluate TeX arithmetic, while scalar encoder rejects arithmetic strings. All numeric desired syntax tags in the authored handoff are therefore expression; primitive number/integer and reset values remain unchanged. Boolean tags remain scalar. Earlier scalar recommendations described numeric primitive identity and were insufficient for the source arithmetic grammar; this correction preserves effective customization.

Reusable direct numeric setter candidate extraction retained as direct-numeric-setter-candidates.json:603 current numeric Scalar descriptors in201 exact direct family namespaces. Balanced keys_define bodies and actual fp_set/int_set declarations are recorded with source SHA/line/variable. This is a candidate source map, not full consumer/default closure. Native owns the905 numeric grammar consolidation; this lane avoids duplicating that scan.

Further exact direct primitive audit found six FP controls currently string/Expression: arch-axonometric scale-z reset1; arch-schematic col-gap/row-gap reset4 via diagram reset; uml-class col-gap reset4; infographic-icon unit-value reset1; state-overview detail-share reset0.5. Setters and actual projection/grid/icon/pane consumers are attached. Desired primitive number preserves Expression syntax. Total exact contract rows41.
