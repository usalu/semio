# 🗄️ stdio format artifacts — slice A (24 single-mutation trees)

Handcrafted `set-snapshot` mutation fixtures for the 24 `🗄️stdio` format trees listed as slice A.
Every tree has exactly one mutation leaf (`📄set-snapshot`); every leaf now carries exactly one
test case. Contract D1 / ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`.

## 📋️ Per-tree result

| artifact / standard | case | what the committed diff pins |
| --- | --- | --- |
| `☁️las@1.0` | `lifts-the-second-point-and-stretches-the-z-bound` | `maxZ` header scalar + one `LasPointModified{index:1}` naming only `z`; empty `vlrs` produces no triple |
| `☁️ply@1.0` | `lifts-the-second-vertex-and-appends-a-comment` | whole-value `comments` slot + name-keyed `vertex` element → row 1's `z` cell addressed by PROPERTY NAME |
| `🌐️html@5` | `declares-the-document-language-on-the-root-html-element` | kind-shaped `HtmlNodeDiff::Element` with one `HtmlAttrAdded{index:0}`; body subtree and doctype absent |
| `🌦️epw@energyplus` | `warms-the-second-hour-and-restamps-the-station-city` | whole-substruct `EpwLocation` + a genuinely sparse 1-of-35-column `EpwRecordDiff` |
| `🎒️zip@2.0` | `extends-the-readme-and-adds-a-version-member` | name-keyed `ZipEntryModified` (data only, no rename) + one whole `ZipEntry` added + `comment` scalar |
| `🎞️gif@87a` | `repaints-the-right-pixel-of-the-single-image` | one `GifImageModified` naming only `indices`; GCT untouched |
| `🎞️gif@89a` | `slows-the-second-frame-and-marks-it-do-not-dispose` | GCE-only frame patch (`delayCs` + `disposal`), pixel buffer untouched, `loopCount` absent |
| `🎞️pptx@ecma-376` | `retitles-and-lowers-the-title-placeholder` | slide→shape→paragraph→run index chain + whole-value `PptxTransform`; OPC/xmlParts absent |
| `🎥️mp4@isobmff` | `promotes-the-second-sample-to-a-sync-frame` | tracks→samples chain with a single `sync` scalar; opaque AVCC payload never restated |
| `🎨️svg@1.1` | `recolours-the-circle-fill-to-crimson` | root→children[0] descent to one `SvgAttrModified{fill}`; prolog slots absent |
| `🎵️mp3@mpeg1-layer3` | `retitles-the-id3v2-tit2-frame` | tri-state `id3v2` written as `Some(Some(tag))`; MPEG frame list stays absent |
| `🏗️ifc@2x3` | `renames-the-ifcproject-instance` | id-keyed WHOLE-instance upsert (`upsertedInstances`), `instanceOrder` absent |
| `🏗️ifc@4` | `renames-the-exterior-wall` | id-keyed entity → POSITIONAL `IfcArgsDiff` slot 2; three HEADER slots absent |
| `💬️bcf@2.1` | `closes-the-clash-topic-and-answers-its-comment` | guid-keyed topic → guid-keyed comment, two levels of `NamedTripleDiff` |
| `💾️binary@raw` | `rewrites-the-two-middle-bytes` | one `ByteSplice{offset:1,removeLen:2}` from the common-prefix/suffix scan |
| `📄️pdf@1.4` | `shrinks-the-page-to-a5-and-rewrites-its-text` | all three flat scalars of the deliberately frozen 1.4 model; `schema` has no slot |
| `📄️pdf@1.7` | `rotates-the-plan-page-and-titles-the-document` | whole-record `PdfInfo` beside a sparse one-scalar `PdfPageDiff`; objects/trailer absent |
| `📄txt@utf-8` | `appends-a-third-line-and-switches-to-crlf` | `lineEnding` scalar + one `TxtLineAdded{index:2}`; the two shared lines never appear |
| `📊️csv@rfc4180` | `corrects-the-area-cell-and-quotes-it` | positional `fields:[null, {value,quoted}]` patch — never a remove+add record pair |
| `📐️step@ap214` | `restamps-the-product-long-name` | id-keyed `#1` → positional `StepArgsDiff` slot 1; HEADER triple absent |
| `📑️tsv@iana` | `renames-the-alpha-row-and-switches-to-crlf` | `lineEnding` scalar + positional `fields:[null,"Beta"]` row patch |
| `📕️xlsx@ecma-376` | `widens-the-total-formula-to-a-third-row` | sheet-name key around a `(row,col)` cell key; whole-value `XlsxCellValue` replacement |
| `📜️docx@ecma-376` | `bolds-the-tower-run-of-the-opening-paragraph` | kind-shaped `DocxBlockDiff::Paragraph` → runs[1] `bold`; OPC lane untouched |
| `📝️md@commonmark` | `demotes-the-tower-heading-to-level-3` | `MdBlockDiff::Heading{level:Some(3), inlines:None}` — inlines never dragged along |

Each case carries the full source-of-truth set (`📸️snapshot/⬅️before`, `📸️snapshot/➡️after`,
`🦠️mutation`, `🔺️diff`, `🎯️outcome`, `🦀️component.rs`), 7 tests and 22–30 assertions, every
`🔺️diff/🔣️component.json` transcribed field-by-field from that tree's own `🔺️diff/🦀️component.rs`
`between`/`diff_set_snapshot` oracle (never from the leaf's name or docstring). All outcomes are
`{"status":"applied"}` with no diagnostics: the mutations-root `Mutation::diff` calls
`diff_set_snapshot` directly and never raises the leaf helper's `mutation.no-op` warning for a
payload that genuinely differs, so no rejected case (and therefore no `🚫️component.absent`) arises
in this slice.

## 🔌️ Wiring

Each tree's OWN mutations-root `🧬️mutations/🦀️component.rs` gained one appended, additive
`//#region 🧪️FixtureTests` block:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/<case>/🦀️component.rs"]
    mod tests_set_snapshot_<case_with_underscores>;
}
```

`📦️glue.rs` was NOT touched. Precedent for this exact shape: `📕️norm/📕️din4108`'s own
`🧬️mutations/🦀️component.rs`.

## ✅️ Verification

- `bun ./📜️script.ts fixtures lint --by-tree` (repo-wide): none of the 174 reported errors, and
  none of the `--by-tree` uncovered rows, belong to this slice — every listed row is `🧊️gltf` or a
  `🧿️semio` subset (other agents' slices).
- Because that CLI truncates its error list at 40 rows repo-wide, the lint's own rules
  (`declaredMutations`/`lintArtifact`/`lintCase`, copied verbatim) were re-run scoped to just these
  24 trees via `scoped-lint.mjs` (kept in this session's scratchpad):
  **24 trees · 24 mutations · 24 covered · 0 uncovered · 0 errors · 192 derived-encoding warnings**
  (8 per case = the 4 `.op/.spr/.patch` targets + 2 snapshot sides × 2 derived encodings — expected
  and correct until `fixtures generate` runs).
- Every `include_str!` target resolves (5 per case, 120 total, all present); every committed JSON
  file parses.
- Every `#[path]` in each appended `fixture_tests` block resolves to an existing file.
- `rustfmt --edition 2021 --emit stdout` parses all 24 new test files and 23 of the 24 mutations
  roots. The one failure — `🏗️ifc@2x3`'s `🧬️mutations/🦀️component.rs` — is a PRE-EXISTING,
  not-ours breakage from the concurrent de-async sweep (`pub(crate) async async fn
  demo_mutation_cases`, plus stray `async ` tokens appended to `use`/`}` lines around line 297);
  `git diff` confirms those hunks are not this agent's and they were left untouched per the
  do-not-fix-what-you-did-not-author rule.
- `cargo` was not run (workspace is mid-sweep); no test is claimed to pass.

## ⚠️ Encoding limitations pinned rather than asserted away

- **`Option<Option<T>>` tri-state slots** cannot round-trip through JSON (`Some(None)` serialises to
  `null`, which decodes back as `None`). Present in `HtmlDiff::doctype`, `SvgDiff::declaration`/
  `doctype`, `GifDiff::gct`/`loopCount` (both standards) and `GifImageDiff`/`GifFrameDiff::lct`/
  `transparentIndex`/`plainText`, `LasPointDiff::gpsTime`/`rgb`, `Mp3Diff::id3v2`/`id3v1`,
  `Ifc2x3Diff::edmPreamble`, `BcfCommentDiff::viewpointRef`, `DocxParagraphDiff::style`,
  `PptxRunDiff::fontSize`, `PdfPageDiff::cropBox`, `MdBlockDiff::List.start`/`CodeBlock.info`.
  Every fixture deliberately leaves the tri-state slot ABSENT and asserts that absence in
  `committed_diff_is_canonical`, with the limitation stated in the file's own docstring.
- **Internally tagged newtype variants over non-map payloads are not serde-serializable.**
  `XlsxCellValue::{Number, SharedString, InlineString, Boolean}` and
  `PdfObject::{Bool, Int, Str, Name, Array, Dict}` are `#[serde(tag = "kind")]` newtype variants
  over primitives/sequences, which serde refuses to serialize at runtime. The xlsx fixture is
  therefore built entirely on the struct-variant `Formula` and unit-variant `Empty` arms, and the
  pdf 1.7 fixture keeps `objects`/`trailer` empty and edits only the typed `pages`/`info` lanes.
  Both files document this in their docstrings. (html's `HtmlNode`, md's `MdInline` and json's
  `JsonValue` already carry the same NOTE and were designed around it; ply/ifc4 use adjacent
  tagging and step/xml use external tagging, so they are unaffected.)
- **`Vec<u8>` is a JSON number array, never base64.** `#[dsl(base64)]` governs only the DSL/op
  codec. Asserted explicitly in the binary, zip and mp4 fixtures.
- **`OpcPackage::relationships` is a `HashMap`** whose iteration order is unspecified; the docx,
  xlsx and pptx fixtures keep it empty so the canonical-JSON fixed point cannot depend on map order.
- **Variant `rename_all = "camelCase"` renames variants but NOT struct-variant fields.**
  `Part21Header`'s `file_description`/`file_name`/`file_schema` and `Part21Instance`'s fields stay
  snake_case inside ifc2x3's otherwise camelCase envelope; the fixture encodes them accordingly.

## 📄️ Files

24 × 6 = 144 new files under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/🏅️standards/<std>/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/<case>/`,
plus 24 additive edits to the corresponding `🧬️mutations/🦀️component.rs` files.
