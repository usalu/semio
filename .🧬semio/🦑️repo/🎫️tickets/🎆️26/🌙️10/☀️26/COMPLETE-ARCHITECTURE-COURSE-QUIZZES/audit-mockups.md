# Independent Excel, Grasshopper and Mockup Audit

Read-only source audit; no source edits. Reviewed existing source basis in tools-content.md, both authored quizzes, both language-neutral fixture sets, mockup renderer and CSS, schema-vector verifier, and architecture quiz-run end-to-end source. Commands used repo tools exec bun only.

## Verification

- verify-mockups.ts passed: 21 vectors, 168 projected items, 14 localized renders.
- verify.ts excel-workbook grasshopper-data passed: seven tasks, all 195 items, 40 challenge/rotation runs, owned validator and AJV agreement.
- Excel concrete command inputs independently inspected: SUM 60, AVERAGE 20, COUNT two numeric cells with text excluded, MIN 12, MAX 30, whole-row sort North/East/West, stable filter West/East, transpose 12/18/30, all five source formula operations yielding 24 before Paste Values, and percent display 25% preserving 0.25. Copy-reference dollar placement correctly distinguishes all four categories.
- Grasshopper supplied output trees match stated Flatten/Graft/Simplify transformations for authored nonempty equal-depth branches; list examples correctly repeat last, truncate end, or produce all pairs. Casting avoids rounding and locale ambiguity; integer/numeric text/coordinate conversions and nonnumeric text or missing mesh geometry are appropriately distinguished by the stated direct-cast contract.
- Terminology separates formulas from references via caption, and plugins from individual components via explicit fictional-package caption.
- Renderer uses native tables and row/column headers, explicit learner locale, highlighted announcements, labelled keyboard-focusable overflow regions, textual wire relations, and sibling answer controls. Sheet projections checked by verifier omit grading fields. Mockup rendering introduces no button/select/input answer controls.
- CSS defines capsule parameters versus rectangular components, explicit slider track/thumb/min/value/max, populated panel and plugin package header. Wire geometry and port ids use the same authored endpoints. This is source/SSR review, not a screenshot or native Grasshopper pixel-fidelity certification; parent owns rendered visual verification.
- End-to-end source correctly changes canvas-terminology (zero wires) to tree-operations (one wire) before clicking the Connections summary. Locator is lazy and resolves against the current task. No invalid summary expectation on zero-wire terminology remains.

## Actionable Finding Sent to Parent

German user-authored mockup node labels still contain English at audit time. File: 🎓️teaching/🏛️architecture/🦗️grasshopper/❓️quiz/🔣️.json; task canvas-terminology; field items[id].mockup.nodes[0].label.de. Native application parameter/component type names and fictional package names can intentionally remain proper names. Localize the following authored labels:

| Item | Existing | Suggested |
|---|---|---|
| slider-width | Width 4.0 | Breite 4,0 |
| slider-height | Height 3.0 | Höhe 3,0 |
| slider-bays | Bays 6 | Felder 6 |
| slider-spacing | Spacing 1.2 | Abstand 1,2 |
| slider-angle | Angle 30 | Winkel 30 |
| panel-messages | Ready | Bereit |
| panel-names | West / East | West / Ost |
| tree-floors | Floors | Geschosse |
| tree-rooms | Rooms | Räume |
| tree-facades | Facades | Fassaden |
| tree-panels | Panels | Paneele |
| tree-zones | Zones | Zonen |
| branch-floor | Floor | Geschoss |
| branch-room | Room | Raum |
| branch-facade | Facade | Fassade |
| branch-panel | Panel | Paneel |
| branch-zone | Zone | Zone |

## Limits and Minor Presentation Notes

Parameter capsules retain generic A/R port letters and are teaching abstractions, not exact native Grasshopper capsules. The source code exposes meaningful relations in text, so this does not affect the tested answer semantics. Copying-references includes copy-absolute-4, whose destination D4 receives =$D$4 and would produce a circular reference in actual Excel; its reference classification remains absolute, but replacing this specific source with a non-destination absolute address would avoid the incidental circular-reference distraction.

No blocking semantic, grading-projection or schema issue found in the reviewed task scope. Localization finding requires parent fix and rerun.
