# Page Boundaries in the Inspector

Selecting a blank region on a page exposes its MediaBox, CropBox, BleedBox, TrimBox, ArtBox and user-unit scale. The inspector sends unrotated coordinates in left, bottom, right, top order through the existing page action and into typed page-box mutations. Clearing a non-media box restores its PDF default. The media box requires four finite increasing coordinates.

The mounted commit lane compares text with the exact published field value. Use the canonical space-separated spelling displayed by the inspector for the current browser acceptance case. Alternate number spelling and redundant whitespace can normalize to the same PDF geometry but leave the renderer draft in conflict. An explicit numeric component or an authored normalization contract is needed to cover those text inputs without weakening commit receipts.

The shared native-page-edit fixture exercises five boundary edits and a scale of 2.5, then clears CropBox. The authored Rust package test uses the UI action payload, typed mutation, export, import and canvas page geometry. The isolated Rust source test passed for all five controls, invalid inputs and both viewport sizes. The independent MuPDF 1.24.14 script reopened the fixture, verified the saved dictionary entries, and measured 450 × 650 before clearing CropBox and 500 × 750 afterward. The registered package test awaits the governed package run.

- [Shared fixture](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📝️native-page-edit/🔣️.json)
- [Production receipt reproduction](📜️page-boundary-draft-probe.ts)
- [Native source probe](📜️page-boundaries-native.py)
- [MuPDF oracle](📜️page-boundaries-oracle.py)
- [ISO 32000-1, Table 30 and §14.11.2](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)
