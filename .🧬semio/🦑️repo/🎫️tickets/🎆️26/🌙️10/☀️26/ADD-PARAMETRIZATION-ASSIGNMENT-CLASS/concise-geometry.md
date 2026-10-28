# Concise Worked Geometry

The worked example now shares the exercise matrix implementation: three numbered box headers for (2,1,1), (3,1,1), (3,2,1), followed by three localized parameter rows with values 2/3/3, 1/1/2 and 1/1/1. AssignmentParameterMatrix captures an optional body; an empty body retains the original blank-row branch. AssignmentParameterMatrixRow accepts a label and comma-separated values and uses expandable mapping so alignment groups do not invalidate callbacks.

The new AssignmentWorkedBoxGraph uses the same contrast-aware vertex backing and selected-edge stroke as the K16. Three numbered box vertices lie on a flattened oval with 16 mm vertical radius. A measured horizontal correction fills the text width without vertical scaling. AssignmentWorkedBoxLedger presents the three signed changes in one compact three-column row: 1–2 (+1,0,0), 2–3 (0,+1,0), 1–3 (+1,+1,0).

The existing K16 picture, 32 mm vertical radius, selected twenty edges, dynamic width correction and blank two-column ledger are unchanged.

## Verification

The retained concise-geometry-check.cjs source contract was authored and run before implementation: it failed on the missing AssignmentWorkedBoxTable macro. It passes after implementation. The parent and content agent own actual German/English compilation and independent fixture/PDF checks. This report will be updated with German dark regression evidence if delegated by the parent.

The initial direct Windows write with truncate mode failed with an UNKNOWN open error; edited authored contents were written to a unique generated temporary file and renamed over the target. No original source backup was created.

## Changed Paths

- 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex
- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty
- This report, concise-geometry-check.cjs and edit-concise-geometry.cjs at the ticket root.

## Native Alignment Repair

The first actual German and English compiles exposed a misplaced omit in the filled table. The row command was initially declared with NewDocumentCommand, whose protected argument parser performs assignments before the first multicolumn. It now uses NewExpandableDocumentCommand, and the comma-list map uses the expandable function form. This makes the complete row prefix expandable before TeX starts each alignment cell. The source contract was updated accordingly and passes.

The repaired native German light probe completed successfully (Tectonic exit 0) and produced generated/compiled/parametrization-de.pdf. The parent owns its visual and semantic PDF review. The German dark variant is being independently compiled with the official source derivation, separate output/cache directories, and the same prepared footer resource projection.

## Final German Dark Verification

Registered Tectonic completed with exit 0. The actual dark PDF has three A4 pages, 112,093 bytes, de-DE catalog language, the correct title, zero overfull boxes and zero underfull boxes. Independent pypdf/pdfplumber checks found no text outside any page, exactly three K3 selected edges, and a visible K3 stroke/label envelope of 167.73 by 43.98 mm. The TeX picture bounding box spans the 170 mm text width, including the canvas backing and node padding; vertical geometry remains below 55 mm.

The retained verify-matrix-pdf.py and verify-k16-pdf.py both pass on the dark PDF. Both exercise matrices remain 9×9 with 6.14 mm writing-row pitch, original centered vector artwork, 2.53–2.61 mm minimum insets, and maximum center offset 0.031 mm. The unchanged K16 retains all sixteen canonical vertices, 120 unique straight base edges, sixteen strong ring edges, four strong middle chords, twenty matching blank ledger entries, 32 mm vertical radius and a 170.001 by 84.242 mm outer envelope.

The contact sheet and all three full-resolution page renders were visually inspected. The filled box table, numbered K3 vertices, all three signed change vectors, parameter labels and concise explanatory cases are legible and separated without overlap. Both exercise matrices and the K16 with its ledger remain clean and contained. The new filled rows inherit the existing dark theme correctly. All generated outputs are left for parent cleanup; no further generated files are being accessed.

Retained additional verification input: concise-dark-review.py. No content files, fixtures, AGENTS files or git state were changed by this agent.

## Shared Matrix Format Judgment

The worked table uses the same AssignmentParameterMatrix implementation, header height, writing-row height, padding, label width, grid rules and theme colors as the exercise tables. No extra localized AssignmentTable title was added to the concise example; those wrappers identify the two exercise families and their separate references. This keeps the requested shared transposed format without adding prose.
