# W4 (G6) Report — document↔docx/md/txt/pdf + presentation↔pptx + workflow↔json

Agent: W4 group G6, one of 6 parallel W4 io-leaf agents. Scope: the `s.stdio.semio/v1/document`
subset's bidirectional io bridges to docx (ecma-376), md (commonmark), txt (utf-8), pdf (1.7),
plus `s.stdio.semio/v1/presentation`↔pptx (ecma-376) and `s.stdio.semio/v1/workflow`↔json
(rfc8259).

## What was built

12 new leaf files (deserializer + serializer per pair), all under each subset's own io tree (zero
edits to any format artifact's own tree):

- `✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs` — `SemioDocumentFromDocx`
- `✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs` — `SemioDocumentToDocx`
- `✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs` — `SemioDocumentFromMd`
- `✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs` — `SemioDocumentToMd`
- `✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs` — `SemioDocumentFromTxt`
- `✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs` — `SemioDocumentToTxt`
- `✳️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️component.rs` — `SemioDocumentFromPdf`
- `✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.7/✳️any/🦀️component.rs` — `SemioDocumentToPdf`
- `✳️presentation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs` — `SemioPresentationFromPptx`
- `✳️presentation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs` — `SemioPresentationToPptx`
- `✳️workflow/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — `SemioWorkflowFromJson`
- `✳️workflow/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — `SemioWorkflowToJson`

All under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`. Each is a real
`ArtifactDeserializer`/`ArtifactSerializer` trait impl doing genuine Snapshot-to-Snapshot field
mapping — zero byte-level re-parsing. All 6 formats' own `store::ArtifactPack` codecs (docx's
`engine::encode_docx`/`decode_docx`, pptx's `engine::encode_pptx`/`decode_pptx`, md's
`parse_markdown_blocks`/`render_markdown_blocks`, txt's line-join, pdf 1.7's
`engine::encode_pdf`/`decode_pdf`, json's `parse_json_text`/`write_json_text`) are invoked
transparently by the generic `deserializer_entry_of`/`serializer_entry_of` erasure — never
re-implemented here.

**Existing files edited** (as directed, not new-file scope):
- `✳️document/🎹️composer/🦀️component.rs` — added the 8 io-bridge imports (4 pairs × 2) + a
  `io_entries()` fn (`OnceLock<Vec<ComposerEntry>>`, since `deserializer_entry_of`/
  `serializer_entry_of` aren't `const fn` — cannot build a `static [ComposerEntry; N]` array
  literal directly, so this mirrors the file's own pre-existing `validator_entry()`
  `OnceLock`-memoized pattern) + `register_composer_entries(io_entries())` inside the existing
  `register()`, extending (not replacing) the pre-existing schema-descriptor/document-codec/
  subset-validator registrations. Also extended the existing `#[cfg(test)] mod tests` with 4
  fixture-backed round-trip tests (one per format).
- `✳️presentation/🎹️composer/🦀️component.rs` — same shape, 1 pair (pptx), 1 round-trip test added
  to the existing test module.
- `✳️workflow/🎹️composer/🦀️component.rs` — same shape, 1 pair (json), 1 round-trip test added to
  the existing test module.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — **not edited by this session.** All 6 new
  `pub mod io { pub mod import/export { ... } }` mount blocks for document/presentation/workflow
  (mirroring gltf's own `✳️any/🚪️io` structural template exactly: `artifacts::<fmt>::v_<std>::any`,
  e.g. `docx::v_ecma_376::any`, `pdf::v1_7::any`) were already present in `glue.rs` by the time this
  session ran its first `cargo check` — confirmed via `git status`/`git diff --stat` showing
  `glue.rs` modified (825 insertions) by a concurrent process before this session's first check,
  and cross-checked line-for-line against the exact module-path predictions this session's
  `composer.rs` `use` statements needed (verified: all resolve with zero `E0432`/`E0433`). This
  session's own composer.rs `use super::io::import::deserializers::artifacts::<fmt>::v_<std>::
  any::<Type>;` imports were written against the SAME predicted convention, confirmed correct once
  cross-checked against the live `glue.rs` state.

## Documented real-world impedance mismatches (per pair, never silently fabricated)

- **document↔docx** (richest — closest structural match): docx's block model only has
  `Paragraph`/`Table` (no `Heading`/`List`/`Code`/`Quote`/`Image`/`PageBreak`) — every docx
  paragraph imports as `DocBlock::Paragraph` (never a guessed `Heading`, since inferring heading
  level from a style NAME would be fabrication); on export, `Heading.level` encodes into the
  WordprocessingML-standard `"HeadingN"` style id only when no explicit `style_id` is set,
  `List`/`Quote` flatten in place, `Code`/`Image` degrade to a plain paragraph (text/alt only),
  `PageBreak` drops. `extra_*_properties` (raw XML this docx model doesn't interpret) and `opc`
  media parts have no semio counterpart — `images` is always empty on import, `RunStyle::
  {size,font,color,link}` drop on export.
- **document↔md**: closest semantic match after docx (Heading/Paragraph/List/CodeBlock/BlockQuote
  map directly). `underline`/`size`/`font` have no CommonMark inline construct — dropped on
  import. `styles` always empty (CommonMark has no named-style concept) — dropped on export.
  `Table` has no CommonMark representation in this codec's scope (GFM tables explicitly out of
  scope per `MdBlock`'s own doc comment) — cells flatten to plain paragraphs. Inline md images
  lift out of their paragraph into their own `DocBlock::Image` (bytes/mime always empty — md only
  carries a URL; `image_id`↔`url` round-trips through this pair alone).
- **document↔txt**: simplest pair — honest plain-text extraction/generation, one line per leaf
  block (list items/quote paragraphs recursively flattened, table rows tab-joined). ALL formatting
  and structure (styles, run styling, heading level, code language, list/quote/table shape) is
  dropped — only visible text survives. `PageBreak` drops entirely (never an empty line, which
  would be indistinguishable from a genuinely empty paragraph on the way back).
- **document↔pdf** (1.7): honest best-effort — PDF page-content-stream text extraction is real but
  structurally flat, so each `PdfPage::text` becomes exactly ONE `DocBlock::Paragraph`; page
  BOUNDARIES (the one real structural signal PDF genuinely offers) are modeled via
  `DocBlock::PageBreak` between pages — no fabricated layout fidelity. `PdfInfo` metadata and the
  full `objects`/`trailer` object graph have no semio counterpart and are never walked (this leaf
  only reads/writes the already-resolved `pages` view; `pdf`'s own `engine::encode_pdf` regenerates
  a fresh minimal file from `pages`+`info` alone, so no PDF byte-writing happens here).
- **presentation↔pptx** (should map closely, presentation's W2 design informed directly by pptx):
  `TextBox`/`Picture`/`Placeholder` map closely (position via `SlideFrame`↔`PptxTransform`,
  `PlaceholderKind`↔pptx's `ST_PlaceholderType` string). `masters`/`layouts` always empty (pptx's
  typed model has no `p:sldMaster`/`p:sldLayout` view) — dropped both ways. `Slide::{id,layout_id,
  notes}` are synthesized/dropped (pptx's typed `PptxSlide` has none of the three). `PptxShape::
  Other` (raw retention for charts/tables/SmartArt/groups/connectors) drops on import — recovering
  a typed shape from raw XML would be codec reimplementation. `SlideShape::Table` has no pptx shape
  counterpart at this typed level and drops on export (fabricating well-formed OOXML table markup
  by hand would be codec reimplementation). `Picture::image.{mime,bytes}` never carry real bytes
  either direction — pptx's typed `Picture` only ever holds a relationship id, never raw media
  (media lives in unmodeled `opc` parts on both sides — a genuine, symmetric boundary, not
  fabrication). Non-`Paragraph` blocks nested in a `TextBox` flatten to plain paragraphs (pptx text
  frames only support flat paragraphs of runs).
- **workflow↔json**: near-direct, LOSSLESS structural mapping (`{"nodes":[...],"edges":[...]}`) —
  every `WorkflowNode`/`WorkflowEdge`/`WorkflowParam`/`PortRef`/`SemioPoint2` field has a direct
  JSON member; no documented lossy fields. Malformed/missing members are real errors
  (`store::PackError::Schema`), never silently defaulted.

## Round-trip tests (fixture-backed, per pair)

Design: rather than depending on an unverifiable, not-yet-mounted cross-file `#[path]` reference
between a pair's deserializer and serializer leaf files (both new, sibling directories, no
established codebase convention for leaf-to-leaf `#[path]` mounting — only `glue.rs` centrally
mounts io leaves, confirmed by reading every existing io leaf in the tree), each of the 6 pairs'
true bidirectional round-trip test lives in its OWNING SUBSET'S composer.rs (`document`/
`presentation`/`workflow`), which already needs `use` imports of BOTH the deserializer and
serializer types for `register()`'s `deserializer_entry_of`/`serializer_entry_of` calls anyway —
a sanctioned location per the task's own "the io leaf file itself, or the subset's schema test
region — your call" latitude. Pattern per test: hand-built format-side fixture1 → (deserialize) →
semio1 → (serialize) → format-side fixture2 → (deserialize) → semio2, asserting `semio1 ==
semio2` — proving the composed deserializer/serializer pair is stable at the semio boundary for
content that fully round-trips through the format (documented lossy fields excepted, called out
above; workflow↔json's fixture round-trips exactly since that pair has no documented losses). Each
io leaf ALSO has its own local fixture-backed unit tests (mapping-direction-specific assertions,
e.g. docx's table/heading-via-style mapping, md's inline-image-lift, pdf's page/PageBreak
boundary, pptx's placeholder-kind mapping, json's missing-required-member error path) — 6 pairs ×
2 files × 1-2 tests = ~18 additional unit tests beyond the 6 composer-level round trips.

## Verification

Polled `cargo check -p semio-s-plugin-stdio --lib` / `cargo test -p semio-s-plugin-stdio --lib
"artifacts::semio"` 7× over the course of this session while other concurrent W4 sibling groups'
own in-progress work resolved (foreign blockers observed and NOT fixed here, per hazard-management
convention: drawing's `io::import`/`io::export` not-yet-mounted composer references, image's
`SemioImageSnapshot` missing-`schema`-field test fixtures, brep's missing `ArtifactDeserializer`
trait import in its own test module — full detail below). The FINAL run
(`w4-document-presentation-workflow-test-GREEN.txt`) compiled clean and ran:

```
test result: FAILED. 426 passed; 1 failed; 0 ignored; 0 measured; 1217 filtered out; finished in 0.05s
```

The lone failure is `artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::
artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec` — G4's own
drawing↔pdf leaf (a different W4 group's pair entirely; not `document`/`presentation`/`workflow`
or any of docx/md/txt/pdf/pptx/json). **All 90 tests under `artifacts::semio::standards::v1::
subsets::{document,presentation,workflow}::*` pass, including all 6 fixture-backed round-trip
tests this session added**:

```
test artifacts::semio::standards::v1::subsets::document::composer::tests::docx_round_trip_is_stable ... ok
test artifacts::semio::standards::v1::subsets::document::composer::tests::md_round_trip_is_stable ... ok
test artifacts::semio::standards::v1::subsets::document::composer::tests::txt_round_trip_is_stable ... ok
test artifacts::semio::standards::v1::subsets::document::composer::tests::pdf_round_trip_is_stable ... ok
test artifacts::semio::standards::v1::subsets::presentation::composer::tests::pptx_round_trip_is_stable ... ok
test artifacts::semio::standards::v1::subsets::workflow::composer::tests::json_round_trip_is_stable ... ok
```

(document: 40 tests, presentation: 27 tests, workflow: 23 tests — 90 total, 0 failed; counted via
`grep -c "subsets::<name>::.*\.\.\. ok"` against the raw log.)

### Blocking-state history (for the record, all foreign, none fixed here)

Across the earlier polls (`w4-document-presentation-workflow-check1.txt`, `-check2.txt`,
`-test-final.txt`), the crate was blocked by up to 10 errors at a time, ALWAYS in
`✳️drawing`/`✳️image`/`✳️brep` (G1/G3/G4's own in-progress files, confirmed foreign via `git status`
showing independent uncommitted diffs never touched by this session) — zero errors were ever
observed in `✳️document`, `✳️presentation`, `✳️workflow`, or docx/md/txt/pdf/pptx/json's own trees,
across every single poll. The only mentions of `✳️document`/`✳️presentation`/`✳️workflow` in any
earlier run's output were pre-existing, repo-wide `hidden lifetime parameters`/`unnecessary
qualification` style lints on `fn compose(sources: &[ComposeSource])` and
`impl protocol::OpText for ...Mutation` — byte-identical to the same lints every other subset's
composer/mutations file carries, not introduced by this wave, not touched by this session. Errors
seen at various points and their eventual (self-)resolution: drawing/image io-mount E0433s (×6,
present from the first poll, still present in the final green run's PRECEDING poll, gone by the
green run itself — G4 must have landed its glue.rs mount between those two runs); image
`SemioImageSnapshot` missing-`schema` E0063s (×3, across gif/bmp/tiff export test fixtures, same
resolution pattern); brep `SemioBrepFromStep::deserialize` E0599 (missing trait import in G1's own
test module, seen once then gone). None were fixed by this session — this session's own write
scope never touched `✳️drawing`, `✳️image`, or `✳️brep`.

## Files changed (created/edited) this wave

Created (12, listed above under "What was built").
Edited: `✳️document/🎹️composer/🦀️component.rs`, `✳️presentation/🎹️composer/🦀️component.rs`,
`✳️workflow/🎹️composer/🦀️component.rs` (io-bridge imports + `io_entries()` + `register_composer_
entries()` call + round-trip test each). `📦️glue.rs` was NOT edited by this session (see note
above — already correctly mounted by another concurrent process before this session's first
check).

## Open item for the orchestrator / W4 closer

None from this report's scope — the final poll of this session reached a genuinely green state
for `document`/`presentation`/`workflow` (90/90 tests passing). One FOREIGN failure remains
crate-wide as of this session's last run: `✳️drawing`'s own `pdf` 1.7 export leaf
(`real_byte_round_trip_through_pdf_codec`, a real assertion mismatch — `"hellosemio"` vs
`"hello\nsemio"`, a newline-join bug in G4's own serializer, not this report's file) — squarely
outside this report's write scope (`✳️drawing/**`), left for its owning W4 group / the closer.
