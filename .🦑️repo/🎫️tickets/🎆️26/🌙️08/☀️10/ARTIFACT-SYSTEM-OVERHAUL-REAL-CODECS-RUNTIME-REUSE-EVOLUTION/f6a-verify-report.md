# F6a Verify Report — Independent Verification of 7 Fan-Out Agents

Scope: `ply`, `ifc4`, `txt`, `pdf1.4`, `csv`, `step`, `xlsx`. Everything below was re-derived
from disk and from real `cargo test` runs in this session — nothing here is taken on the
self-reporting agents' word.

Crate under test: `semio-s-plugin-stdio` (single crate, `bun ./📜️script.ts test` ==
`cargo test -p semio-s-plugin-stdio`). Rust toolchain has no `timeout(1)` on this macOS host,
so tests were run directly via `cargo test -p semio-s-plugin-stdio --lib "<module-path-filter>"`.

## Per-artifact results

### ply (standard `1.0`)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `impl protocol::DiffCodec for PlyDiff` at line 937. **Hand-rolled.** No `dsl::DslDiff` in any derive
  list on `PlyDiff` (only referenced in a doc comment explaining the derive fails).
- Mutations file: `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → `impl protocol::OpText for PlyMutation` (line 221) and `impl protocol::OpBinary for PlyMutation`
  (line 231). No `serde_json::to_string`/`serde_json::to_vec` anywhere in either file (stub confirmed gone).
- Tests present: `op_text_binary_roundtrip_law` (mutations, line 267) and
  `diff_codec_text_binary_roundtrip_law` (diff, line 1023) — both real, both ran.
- Test filter run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply::"`
  → **25 passed, 0 failed**, includes both target laws (`ok`).

### ifc4 (standard `4`, i.e. IFC4 — distinct from `2x3` which is out of scope)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `impl protocol::DiffCodec for IfcDiff` at line 850. **Hand-rolled** (doc comment explicitly
  confirms `#[derive(dsl::DslDiff)]` fails to compile on this struct — derive intentionally absent).
- Mutations file: `.../🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → `impl protocol::OpText for IfcMutation` (line 260), `impl protocol::OpBinary for IfcMutation`
  (line 270). `serde_json::to_string`/`to_vec` stub: gone from both files.
- Tests present: `op_text_binary_roundtrip_law` (mutations) and `diff_codec_text_binary_roundtrip_law`
  (diff, inside `handcrafted_diff_codec_tests` module) — both real, both ran.
- Test filter run (scoped to `v4` only, to exclude the unrelated `2x3` standard):
  `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc::standards::v4::"`
  → **19 passed, 0 failed**, includes both target laws.
  (Note: an unscoped `artifacts::ifc::` filter pulls in 64 tests total because it also matches the
  `2x3` standard, which has its own older `op_text_round_trips` test unrelated to this ticket — the
  19-test v4-scoped number is the correct one for this artifact/standard.)

### txt (standard `utf-8`)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]`
  on `TxtLinesDiff` (line 243). **Derived.** No manual `impl protocol::DiffCodec` anywhere in the file
  (grep confirms zero matches) — clean single-path, not a derive+override hybrid.
- Mutations file: `.../🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → `impl protocol::OpText for TxtMutation` (line 125), `impl protocol::OpBinary for TxtMutation`
  (line 148) — hand-written as expected (OpText/OpBinary are always handwritten regardless of diff path).
  `serde_json` stub: gone.
- Tests present: `op_text_binary_roundtrip_law` (mutations, line 237) and
  `diff_codec_text_binary_roundtrip_law` (diff, line 431).
- Test filter run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::txt::"`
  → **21 passed, 0 failed**, includes both target laws.

### pdf1.4 (standard `1.4` — distinct from `1.7` which is out of scope)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]`
  (line 28). **Derived**, confirmed compiling clean per file's own doc comment. No manual
  `impl protocol::DiffCodec` present anywhere in the file.
- Mutations file: `.../🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → `impl protocol::OpText for PdfMutation` (line 71), `impl protocol::OpBinary for PdfMutation`
  (line 91). `serde_json` stub: gone.
- Tests present: `diff_codec_text_binary_roundtrip_law` (diff, line 186) and
  `op_text_binary_roundtrip_law` (mutations, line 149).
- Test filter run (scoped to `v1_4` to exclude the unrelated `1.7` standard):
  `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf::standards::v1_4::"`
  → **23 passed, 0 failed**, includes both target laws.

### csv (standard `rfc4180`)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `impl protocol::DiffCodec for CsvDiff` at line 699. **Hand-rolled** (doc comment explicitly states
  `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` cannot be used because of the
  `Option<Vec<Option<CsvFieldDiff>>>` shape).
- Mutations file: `.../🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → imports `use protocol::{Mutation, MutationDiff, OpText};` then `impl OpText for CsvMutation`
  (line 222, unqualified form — this was a false negative on a naive fully-qualified grep, confirmed
  present on closer inspection) and `impl protocol::OpBinary for CsvMutation` (line 232). Both hand-rolled
  per the file's own doc comment. `serde_json` stub: gone.
- Tests present: `op_text_binary_roundtrip_law` (mutations, line 476, inside region
  `🔖️OpTextBinaryRoundtripLaw`) and `diff_codec_text_binary_roundtrip_law` (diff, line 734, inside
  `handcrafted_diff_codec_tests`).
- Test filter run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::csv::"`
  → **19 passed, 0 failed**, includes both target laws.

### step (standard `ap214`)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `impl protocol::DiffCodec for StepDiff` at line 917. **Hand-rolled** (doc comment: derive fails,
  `StepEntitiesDiff: DslField` unsatisfied, cascading).
- Mutations file: `.../🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → imports `use protocol::{Mutation, MutationDiff, OpText};` then `impl OpText for StepMutation`
  (line 247, unqualified — same pattern as csv, confirmed present) and
  `impl protocol::OpBinary for StepMutation` (line 257). Both hand-rolled. `serde_json` stub: gone.
- Tests present: `op_text_binary_roundtrip_law` (mutations, line 349) and
  `diff_codec_text_binary_roundtrip_law` (diff, line 1184).
- Test filter run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::step::"`
  → **93 passed, 0 failed** (this artifact has many additional cc1-cc6 subset conformance tests
  unrelated to F6 — confirmed via grep that both target laws are among the 93 and both show `ok`).

### xlsx (standard `ecma-376`)
- Diff file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  → `impl protocol::DiffCodec for XlsxDiff` at line 1205. **Hand-rolled** (doc comment references a
  ticket-folder scratch file `f6-xlsx-diff-check2.txt` documenting the derive failure).
- Mutations file: `.../🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  → `impl protocol::OpText for XlsxMutation` (line 279), `impl protocol::OpBinary for XlsxMutation`
  (line 289). `serde_json` stub: gone.
- Tests present: `diff_codec_text_binary_roundtrip_law` (diff, line 1270, inside
  `handcrafted_diff_codec_tests`) and `op_text_binary_roundtrip_law` (mutations, line 826).
- Test filter run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::xlsx::"`
  → **43 passed, 0 failed**, includes both target laws.

## Full crate suite (run once, after all per-artifact filters)

```
cargo test -p semio-s-plugin-stdio --lib
```
→ **1033 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out**, finished in 7.47s.
(Up from the 1019+/0 baseline the F6 recon captured before this fan-out wave landed — consistent
with 7 new hand-rolled/derived codec implementations plus their new tests being added.)

Full raw output saved to (ticket-folder scratch, not deleted per rules):
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6a-full-test-run.txt`

## Cross-cutting observations

1. **Two hand-rolled `OpText` impls use the unqualified form.** `csv` and `step` both write
   `use protocol::{..., OpText};` then `impl OpText for X` rather than `impl protocol::OpText for X`.
   This is functionally identical (same trait, just imported) but means a naive grep for
   `impl protocol::OpText` alone under-reports — verification here re-grepped both files for the
   bare `OpText`/`OpBinary` tokens before drawing conclusions, and confirmed both impls are real and
   present. Anyone doing a similar sweep on this crate should account for both call forms.
2. **All 7 diff-path classifications match what each artifact's own doc comments claim**: `ply`,
   `ifc4`, `csv`, `step`, `xlsx` are hand-rolled `DiffCodec` (each with an explicit doc-comment
   citation of why the derive fails); `txt` and `pdf1.4` are derived via `#[derive(dsl::DslDiff)]`
   with no leftover manual `impl DiffCodec` anywhere in the file (no derive+override hybrids found).
3. **No artifact still uses the old generic `serde_json::to_string`/`serde_json::to_vec` stub** —
   confirmed via `grep` returning zero matches in every diff.rs and mutations.rs file checked.
4. **`ifc4` and `pdf1.4` each share a parent artifact directory with a sibling standard that is out
   of this ticket's scope** (`ifc` also has `2x3`; `pdf` also has `1.7`). An unscoped module-path test
   filter (`artifacts::ifc::` / `artifacts::pdf::`) pulls in the sibling standard's tests too. The
   per-artifact counts reported above are scoped to exactly the standard in question
   (`artifacts::ifc::standards::v4::` / `artifacts::pdf::standards::v1_4::`) so they reflect only
   this ticket's work.
5. No modifications were made to any file during this verification pass — this was a read/grep/test-run
   only session, consistent with the "independent verification" brief.

## Summary table

| artifact | tests_passed | tests_failed | diff_codec_present | op_text_binary_present | serde_json_stub_gone |
|---|---|---|---|---|---|
| ply    | 25 | 0 | true (hand-roll) | true | true |
| ifc4   | 19 | 0 | true (hand-roll) | true | true |
| txt    | 21 | 0 | true (derive)    | true | true |
| pdf1.4 | 23 | 0 | true (derive)    | true | true |
| csv    | 19 | 0 | true (hand-roll) | true | true |
| step   | 93 | 0 | true (hand-roll) | true | true |
| xlsx   | 43 | 0 | true (hand-roll) | true | true |
| **full crate** | **1033** | **0** | | | |
