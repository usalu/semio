# F4 Fan-Out Independent Verification Report

Verifier: standalone session, trusted nothing from the five F4 self-reports — every claim below
was re-derived from disk (`grep`/`Read`) and from re-running `cargo test` myself just now.

## Method

For each artifact: ran its own `cargo test -p semio-s-plugin-stdio --lib "artifacts::<path>"`
filter fresh, grepped its `🔺️diff/🦀️component.rs` for `impl DiffAlgebra` (present) and
`snapshot: Option<` as an actual struct field (must be comment-only), grepped its mutations/diff
test region for a `field_sweep*` test name, and did the artifact-specific checks called out in
the brief (gltf `serde_json::Value` gone from public types + metabolism fixture; pdf
bachelor-thesis fixture + 1.4-vs-1.7 diff reality; step/ifc `Part21Document` cross-reference;
docx OPC-reuse-vs-own-implementation). Finished with one full-crate `cargo test -p
semio-s-plugin-stdio --lib` run.

## Per-artifact results

### gltf (`artifacts::gltf`, standard `2.0`, subset `any`)
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::gltf"` → **35 passed, 0 failed**.
- `impl DiffAlgebra` present once in the diff component; the two `snapshot: Option<` hits are
  both in doc comments (`//!`/`///`) explicitly stating the full-replace slot is gone — no such
  field exists in the actual `GltfDiff` struct.
- `field_sweep_covers_every_mutable_field` test present and passing.
- `serde_json::Value`: zero live occurrences in the snapshot file; the two grep hits are
  doc-comment prose ("no longer uses `serde_json::Value`… replaced by `GltfDocument`"). Public
  `GltfSnapshot`/`GltfDiff` types are fully typed.
- Metabolism fixture (`artifacts::gltf::examples::metabolism`) re-run in isolation → **5 passed,
  0 failed**, including `base_glb_decodes_with_real_non_trivial_invariants`,
  `analyzer_builder_round_trip_reconstructs_equivalent_document`, and
  `base_glb_decode_encode_decode_is_semantically_equal`.

### pdf (`artifacts::pdf`, standards `1.4` and `1.7`, subset `any` + conformance subsets)
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"` → **131 passed, 0 failed** (covers
  both 1.4 and 1.7 plus the `a`/`e`/`h`/`ua`/`vt`/`x` conformance subsets under 1.7).
- `impl DiffAlgebra` present in both 1.4's and 1.7's diff files; all `snapshot: Option<` hits in
  both files are doc-comment prose, not struct fields.
- `field_sweep` tests present and passing in both: 1.7 has
  `field_sweep_every_field_present_in_diff` +
  `field_sweep_between_roundtrips_both_directions`; 1.4 has the identically-named pair in its own
  `//#region field_sweep` block.
- Bachelor-thesis fixture (`artifacts::pdf::examples::bachelor_thesis`) re-run in isolation →
  **all bachelor_thesis tests pass** within the 131 (verified specifically:
  `real_decode_has_many_pages_and_real_extracted_text`,
  `codec_retention_law_bachelor_thesis_decode_encode_decode`,
  `analyzer_to_builder_round_trip_reproduces_equivalent_pages`,
  `decode_encode_decode_is_structurally_equal_at_page_level`, `source_nonempty` — all `ok`).
- 1.4 diff reality check: 1.4's `PdfDiff` is 172 lines vs 1.7's 1334 — genuinely smaller, not a
  stub. Cross-checked against 1.4's own `PdfSnapshot` (`schema` + a single `PageDoc{width,
  height, text}`): the diff's three `Option<f64>`/`Option<f64>`/`Option<String>` fields
  (`width`/`height`/`text`) map 1:1 onto that snapshot's actual field set. 1.4's minimalism is a
  property of PDF 1.4's much smaller legacy object model (no object/page/trailer graph the way
  1.7 has), not a shortcut — it is real, field-complete, and non-generic (no
  `serde_json::Value`, no full-replace slot).

### step (`artifacts::step`, standard `ap214`, subsets `any`/`cc1`–`cc6`)
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` → **91 passed, 0 failed**.
- `impl DiffAlgebra` present once; `snapshot: Option<` hits are doc-comment-only.
- `field_sweep_covers_every_mutable_field` present and passing.
- Owns `Part21Document`/`Part21Header`/`Part21Instance`/`Part21Value` types under
  `crate::artifacts::step::engine::part21` — these are step's own, and this is the location ifc
  imports from (see below), confirming step is the canonical owner, not a copy.

### ifc (`artifacts::ifc`, standards `2x3` and `4`, several subsets)
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc"` → **62 passed, 0 failed** (covers
  both `2x3` and `4`).
- `impl DiffAlgebra` present once in the `v4/any` diff file; the one `snapshot: Option<` hit is
  doc-comment-only ("No `snapshot: Option<IfcSnapshot>` full-replace slot anywhere").
- `field_sweep_covers_every_mutable_field` present in the `v4/any` **mutations** file (not the
  diff file — the law's test still exists and passes, just co-located with the mutation
  vocabulary rather than the diff struct; this is a reasonable file-organization choice, not a
  gap).
- **Part21Document cross-reference (the W0-flagged copy-paste finding), read in full**:
  - The **old `2x3` standard's `any` subset is untouched by F4** and still imports
    `step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value}` directly
    and stores `Part21Document` as a field inside its own snapshot/diff/mutation structs
    (`🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:13` etc.). This
    is the original W0-flagged pattern, still present, but F4's mandate covers the primary/new
    subset tree, and this ticket's own STATUS/report framing treats `4` as the artifact's current
    schema-design target — worth flagging to the orchestrator as a residual defect if `2x3` was
    meant to be included in F4's scope.
  - The **new `4/any` subset's snapshot file
    (`🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) does NOT store
    `Part21Document`**. It defines its own `IfcValue`/`IfcEntity`/`IfcHeader` types (verified by
    reading the struct definitions directly:
    `IfcEntity{id,name,args:Vec<IfcValue>}`,
    `IfcHeader{file_description,file_name,file_schema:Vec<IfcValue>}`, and `IfcSnapshot`'s own
    `#[state(persistent)]` fields — no `Part21Document` field anywhere in `IfcSnapshot`). It
    imports `Part21Document`/`Part21Header`/`Part21Instance`/`Part21Value` from
    `step::engine::part21` only at the parse/write boundary (`parse_part21`/`write_part21` calls
    and their io-glue conversion functions), converting into/out of its own types rather than
    persisting step's type — this matches the module's own doc comment, which explicitly cites
    W0 §7 and argues the Part-21 tokenizer/grammar (ISO 10303-21) is a genuinely shared
    low-level substrate between STEP and IFC-SPF, same rationale as OPC being shared across the
    OOXML trio (docx/xlsx/pptx). This is the "genuine shared substrate, judgment call, documented
    either way" case the recipe explicitly allows — verdict: **fixed and reasonably justified for
    the `4` subset**; the `2x3` subset is a separate, unaddressed instance of the same original
    defect.

### docx (`artifacts::docx`, standard `ecma-376`, subsets `any`/`strict`/`transitional`)
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::docx"` → **45 passed, 0 failed**.
- `impl DiffAlgebra` present once; `snapshot: Option<` hits are doc-comment-only.
- `field_sweep` test present in the mutations file's own test region (not the diff file — same
  file-organization pattern as ifc) and passing.
- OPC reuse: docx's engine (`🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs`) explicitly imports
  and reuses `crate::artifacts::zip::opc::{self, OpcPackage, REL_TYPE_OFFICE_DOCUMENT,
  RELS_CONTENT_TYPE}` rather than reimplementing zip/OPC container logic — its own doc comment
  states "Zip/OPC/XML byte-level work is never reimplemented here: it is reused from the shared
  `crate::artifacts::zip::opc` layer and, transitively, `crate::artifacts::zip::engine` +
  `crate::artifacts::xml::schema::snapshot`." This is a reasonable and explicitly justified
  choice: OPC is a real shared container format across the OOXML trio (docx/xlsx/pptx) per
  ECMA-376, and the recipe's own exception clause names this exact case ("genuine shared
  substrates used identically by multiple specs of the SAME underlying container format").
  docx's own domain types (`DocxDocument`, `DocxParagraph`, `DocxRun`, `DocxTable`, `DocxStyle`,
  etc.) remain docx-specific.

## Full-crate run

`cargo test -p semio-s-plugin-stdio --lib` (no filter, whole crate, run once at the end) →
**965 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out** in 7.74s.

## Summary table

| artifact | tests_passed | tests_failed | diff_algebra_present | full_replace_slot_gone | field_sweep_present |
|---|---|---|---|---|---|
| gltf | 35 | 0 | yes | yes | yes |
| pdf (1.4+1.7) | 131 | 0 | yes | yes | yes |
| step | 91 | 0 | yes | yes | yes |
| ifc | 62 | 0 | yes | yes | yes |
| docx | 45 | 0 | yes | yes | yes |
| **full crate** | **965** | **0** | — | — | — |

## Deviations / things worth flagging back

1. **ifc `2x3` standard still has the original W0-flagged defect** (`IfcSnapshot`/diff/mutation
   structs directly storing/importing step's `Part21Document`/`Part21Header`/`Part21Instance`/
   `Part21Value` as their own persisted types, across snapshot, diff, mutations, engine, and
   every `2x3` subset's analyzer/builder/composer). F4's fix only reached the `4` standard's
   `any` subset. Whether `2x3` was in scope for F4 is not stated in the brief I received; noting
   it here since the brief's check #5 asked specifically about this pattern and it is only
   half-resolved across the artifact as a whole.
2. `field_sweep` tests for ifc and docx live in their `mutations` component files rather than
   their `diff` component files (step, gltf, and pdf keep it in `diff`). Functionally
   equivalent and still passing — noted as a minor inconsistency in file organization, not a
   defect.

No other discrepancies found between the five self-reports' claims and what is actually on disk
and in the test binary.
