# W2b — `presentation` subset — Final Report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/**` only.
No files outside this directory were edited.

## Summary

All 8 completeness items were implemented as real, handcrafted code (no scaffolded `🚧`
placeholders remain in any Rust file under this scope; `🚪️io/🦀️component.rs` intentionally stays
a structure-only stub per the brief — that facet is W4's job).

`cargo check -p semio-s-plugin-stdio --lib` compiles this subset's code with **zero errors and
zero warnings** (confirmed repeatedly, most recently in
`w2b-presentation-cargo-check-final.txt`). **`cargo test` could not be executed** — the crate as a
whole fails to compile due to a persistent, unrelated, foreign compile error in three OTHER
subsets (`document`, `image`, `workflow`), confirmed out of scope and not caused by this work. See
"Blocker" section below for full detail and evidence. All law-test logic was written, and every
non-trivial algorithm (the generic indexed/named triple between/apply/inverse/absorb engine, the
canonical absorb cases, the field-sweep fixtures) was manually hand-traced against the same
algorithm already proven correct and merged in docx's own diff file, which this subset's engine
is a direct structural port of.

## 1. Snapshot (complete-per-spec)

`🧬️schema/📸️snapshot/🦀️component.rs` — `SemioPresentationSnapshot { schema, masters:
Vec<SlideMaster>, layouts: Vec<SlideLayout>, slides: Vec<Slide> }`.

- `SlideMaster{id, shapes}`, `SlideLayout{id, master_id, shapes}` — id-keyed, referenced by id.
- `Slide{id, layout_id: Option<String>, shapes, notes: Vec<DocBlock>}` — index-keyed (presentation
  order is significant, like pdf page order), `id` still carried as its own identity field.
- `SlideShape` (tag `shapeKind`, not `kind` — see below): `TextBox{frame, blocks: Vec<DocBlock>}`,
  `Picture{frame, image: SlidePictureImage}`, `Table{frame, rows: Vec<SlideTableRow>}`,
  `Placeholder{frame, kind: PlaceholderKind}`.
- `SlideFrame{origin: SemioPoint2, width, height}` — reuses the shared `engine::geometry`
  `SemioPoint2` for the position field.
- `PlaceholderKind`, `SlidePictureImage`, `SlideTableRow`, `SlideTableCell` — all owned here.
- **Spec-mandated cross-reuse**: `TextBox.blocks` / table-cell `blocks` / `Slide.notes` all reuse
  `document::DocBlock` directly (not redefined), per `w1b-type-ownership.md`.
- No `serde_json::Value`, no bare tuples, no nested fixed arrays anywhere.

**Real bug found and fixed in my own code**: `SlideShape`/`SlideShapeDiff` were originally tagged
`#[serde(tag = "kind")]`, but the `Placeholder` variant's own field is *also* named `kind` — an
internally-tagged enum's tag must not collide with a variant's own field name, which is a hard
serde constraint (not merely a lint) and made the whole enum silently fail to derive
`Serialize`/`Deserialize`, cascading into ~40 confusing downstream trait-bound errors. Fixed by
renaming the tag to `shapeKind` on both `SlideShape` and `SlideShapeDiff` (documented in both
types' doc comments).

## 2. Diff (sparse, handcrafted)

`🧬️schema/🔺️diff/🦀️component.rs` — `SemioPresentationDiff{masters: Option<NamedTripleDiff<...>>,
layouts: Option<NamedTripleDiff<...>>, slides: Option<IndexedTripleDiff<...>>}`. No
`snapshot: Option<SemioPresentationSnapshot>` full-replace slot anywhere — verified by grep.

- `masters`/`layouts` are name-keyed (`NamedTripleDiff<String, _, _>`); `slides` is index-keyed
  (`IndexedTripleDiff<_, _>`, presentation order matters).
- Every nested collection (`shapes`, table `rows`/`cells`, `notes`, `TextBox.blocks`) is
  index-keyed via the same generic engine.
- **Shared-infra gap found, reported, NOT fixed** (out of my write scope,
  `⚙️engine/🧰️triples/🦀️component.rs`): the shared `IndexedTripleDiff`/`NamedTripleDiff` derive
  omits the `#[serde(bound(...))]` override that docx's own local copy carries. Without it,
  `serde_derive`'s conservative per-field-`#[serde(default)]` bound inference spuriously requires
  `D: Default`/`T: Default` on every instantiation — a real compile blocker here since neither
  `SlideShape` nor (out-of-scope) `document::DocBlock` implements `Default`. Worked around by
  defining a LOCAL copy of `IndexedTripleDiff`/`NamedTripleDiff` (with the correct `bound(...)`)
  in this file, exactly mirroring docx's own precedent (every hand-rolled artifact in this program
  already keeps its own local copy of this small generic engine — this file follows that, not a
  new pattern). Recommend the shared `🧰️triples` module gets the same `bound(...)` fix so future
  W2 agents can actually use the "shared" copy for non-`Default` item types.
- `document::DocBlock` is reused for text content but owned by another subset with no field-level
  diff exposed yet — diffed as a **whole value** (`D = T = DocBlock`, "modified" carries the full
  replacement). Honest per the recipe's weak/strong-entity split (I do not own `DocBlock`'s
  internals). A real, hand-rolled 8-variant encoder (`enc_doc_block`/`dec_doc_block`, covering
  `Paragraph`/`Heading`/`List`/`Table`/`Code`/`Quote`/`Image`/`PageBreak`) was still written for the
  hand-rolled `DiffCodec`/`OpText` wire grammars, since encoding a value you don't own the type of
  is the established precedent (docx's own `enc_xml_node` for `xml::XmlNode`).
- `MutationDiff::{apply, absorb}` + `DiffAlgebra::{between, inverse, is_empty}` all hand-rolled,
  structurally identical to docx's proven algorithm (between_indexed/apply_indexed/inverse_indexed/
  absorb_indexed + the named equivalents), adapted to this subset's own types.
- Hand-rolled `protocol::DiffCodec` (`print_diff`/`parse_diff`/`encode_diff`/`decode_diff`):
  `masters=[...] layouts=[...] slides=[...]` token grammar, binary = text bytes verbatim (same
  simplification docx/gif/svg use).

## 3. Mutations (named-variant, hand-written diff()/inverse())

`🧬️schema/🧬️mutations/🦀️component.rs` — 15-variant `SemioPresentationMutation` (`NoMutation`,
`SetSnapshot`, `InsertSlide`, `RemoveSlide`, `SetSlideLayout`, `SetSlideNotes`, `InsertShape`,
`RemoveShape`, `SetShapeFrame`, `SetTextBoxBlocks`, `InsertMaster`, `RemoveMaster`, `InsertLayout`,
`RemoveLayout`, `SetLayoutMaster`). Every variant's `diff()` calls a dedicated hand-rolled
`diff_*` builder in the diff module (never apply-and-capture — verified: no `snapshot: Option<`
anywhere, no catch-all dispatch arm in `Mutation::diff`'s match). Every variant's `inverse()` is
hand-rolled, index/key-aware, matching the base state at call time.

- Addressing: slides by `index`, shapes by `(slide_index, shape_index)` — no recursive path type
  needed (unlike docx's `DocxBlockPath`), since a shape tree here is exactly two levels deep.
- `📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` triad dir present (thin delegates, matching the
  existing pattern — docx itself only ships this one triad dir despite 13 mutation variants; the
  other variants' `diff()`/`inverse()` logic lives inline in the main `mutations/component.rs`,
  exactly the same shape this file follows).
- Hand-rolled `OpText`/`OpBinary`: `keyword arg=value ...` one-line grammar (docx/gif/svg
  convention), binary = text bytes verbatim.

## 4. Grammar leaves (8 text + 6 binary, per facet)

All 3 facets (`📸️snapshot`, `🔺️diff`, `🧬️mutations`) × 14 leaf files (`.g4`, `.grammar.semio`,
`.graphql`, `.json`, `.ebnf`, `.proto`, `.ts`, `.rs` for text; `.ksy`, `.spicy`,
`.protocol.semio`, `.abnf`, `.ts`, `.rs` for binary) = 42 files, all real content (no
`*OCTET`-catch-all dishonesty where real structure exists):

- **Diff/mutations facets**: real, structured grammar matching the actual hand-rolled
  `print_diff`/`print_op` token grammars (`masters=[...];[...];[...] layouts=...` and
  `keyword arg=value ...` respectively) — genuinely decomposed keyword/token/triple productions,
  not a lazy catch-all.
- **Snapshot facet**: honest envelope+hex-payload grammar (`header NL payload=*OCTET`), matching
  the real `ArtifactDsl`/`ArtifactPack` impl exactly — the payload genuinely IS an opaque
  serde_json blob at this representation level (the structured shape lives in the sibling JSON
  Schema); this is the same convention docx's own (F6-complete, real) snapshot-level grammar
  leaves use, not a shortcut unique to this subset.
- Generated via a scratch Python script (`w2b-presentation-gen-mirrors.py`, this ticket folder,
  not a permanent repo script) to keep 60+ files consistent with the real Rust shapes; every
  `.json` file validated as parseable JSON.

## 5. Builder

`🏗️builder/🦀️component.rs` — real `ArtifactBuilder` impl: `empty`/`from_snapshot`/`from_text`/
`from_binary`/`mutate → (Self, Diff)`/`absorb`/`build`, all delegating to the real snapshot codecs
and `apply_semio_presentation_mutation`. 3 tests added (empty/from_snapshot/build round trip,
from_text/from_binary round trip through a populated snapshot, mutate-then-absorb-matches-
direct-apply).

## 6. Analyzer

`🧐️analyzer/🦀️component.rs` — real `ArtifactAnalyzer`: `sniff()` genuinely inspects the payload
for the schema marker (High/Low, not an always-constant stub), `analyze()` genuinely decodes via
the real `ArtifactDsl`/`ArtifactPack` impls and reports Low confidence + a real diagnostic on
decode failure. 3 tests added.

## 7. Composer

`🎹️composer/🦀️component.rs` — `SemioPresentationComposer` with
`WRITES = Dialect{"s.stdio.semio", "v1", "presentation"}` (const literal matches its own path).
`SemioPresentationValidator` upgraded from decode-only to **real referential-invariant checks**:
`check_presentation_referential_integrity` verifies every `layout.master_id` resolves to a real
master, every (set) `slide.layout_id` resolves to a real layout, and `masters`/`layouts` ids are
unique (both are name-keyed collections — a duplicate id would silently corrupt any future
`between()`/`apply()`). 6 tests added (clean snapshot, dangling-master, dangling-layout,
duplicate-id, no-layout-is-fine, validator-through-pack-payload).
`register_document_codec` id is `"s.stdio.semio.presentation"` — distinct, matches the
`#[artifact_schema(id = ...)]` on the snapshot/diff types, repo-wide unique (only this subset uses
it).

## 8. Test laws

All 8 laws are implemented as real tests (7 in `mutations/component.rs`'s `mod tests`, 1 —
`diff_codec_text_binary_roundtrip_law` — in `diff/component.rs`'s
`mod handcrafted_diff_codec_tests`, mirroring docx's exact placement):
`field_sweep`, `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
`codec_retention_law`, `op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`.
Plus one extra targeted test (`shape_kind_change_produces_replace_and_round_trips`) covering the
`SlideShapeDiff::Replace` fallback, which none of the 8 required laws' fixtures happened to
exercise on their own.

`absorb_law` includes all 4 canonical cases the recipe requires (Insert+Remove-before,
Insert+Insert-same-index, Insert+SetField-patch-into-added, Modify+Remove-annihilates) plus
associativity, run against `slides` (the index-keyed top-level collection).

`field_sweep`'s `sweep_a`/`sweep_b` fixtures differ in every mutable field across `masters`
(removed+modified+added), `layouts` (removed+modified+added, `master_id` change), and `slides`
(removed via `a→b`, added via the reverse `b→a` — an index-keyed collection can't show both in one
direction, same "structural trap" docx's own fixtures document), including the nested shape tree
(modified `TextBox` + added `Picture`), the `document::DocBlock`-reuse notes list, and the
`layout_id` tri-state (`Some(Some(_))` one direction, `Some(None)` the other).

## Blocker: `cargo test` could not be run (foreign, confirmed out of scope)

`cargo check -p semio-s-plugin-stdio --lib` fails to compile the **whole crate** due to 6 real
compile errors, **all three outside this subset**, unchanged across 6+ recompiles over 30+ minutes
of polling (per the ticket's own "foreign unstaged mods → poll 3×10 min, don't chase" guidance):

```
document/🧬️schema/🧬️mutations/🦀️component.rs:632,636 — E0599 no method `print_op`/`parse_op`
image/🧬️schema/🧬️mutations/🦀️component.rs:272,276    — E0599 no method `print_op`/`parse_op`
workflow/🧬️schema/🧬️mutations/🦀️component.rs:264,268  — E0599 no method `print_op`/`parse_op`
```

Root cause (verified by reading the files, not touched): each file's `impl protocol::OpBinary`
block calls `self.print_op()`/`Self::parse_op(...)`, but `document`'s own
`use protocol::{OpBinary, OpText};` is gated `#[cfg(test)]` (visible only to its own test module,
not its main `impl` block) — `image` and `workflow` are missing the import entirely outside their
test modules. `git status` confirms all three files are `M` (actively modified, uncommitted) —
concurrent W2b sibling agents' in-progress work, not mine to touch (write scope is exactly
`✳️presentation/**`). Evidence: `w2b-presentation-cargo-check-final.txt`,
`w2b-presentation-foreign-blocker-gitstatus.txt`.

**This subset's own code is proven error-and-warning-free** by `cargo check` (grep for
`presentation` in the final check log matched only 2 cosmetic warnings, both fixed inline: an
`unnecessary qualification` in `mutations/component.rs`, and a `hidden lifetime parameters are
deprecated` in `composer/🦀️component.rs`'s pre-existing `compose(sources: &[ComposeSource])`
signature — the latter was inherited unchanged from the W1b scaffold, not introduced by this
wave). Re-verified clean with zero presentation-scope errors/warnings after both fixes. Every law's algorithm was
manually hand-traced against docx's own already-`cargo test`-green implementation, which this
file's generic engine is a direct structural port of (same `between_indexed`/`apply_indexed`/
`inverse_indexed`/`absorb_indexed`/`transform_index`/`simulate_mid_origins` logic, same canonical
absorb cases, same field-sweep fixture shape). I could not, however, produce the literal
`cargo test` pass/fail numbers this ticket's exit checklist asks for — doing so requires the
document/image/workflow blocker to clear first. Recommend the wave closer either re-run
`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio.*presentation"` once those three files
compile, or dispatch a 3-line fix to whichever agent owns them.

## Files touched (all within `✳️presentation/**`)

Every file under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/`
was replaced/written with real content (76 files total): `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}`
× {top-level `component.rs`/`.ts`/`.graphql`/`.json`/`.proto`, `📝️text` 8 leaves, `💾️binary` 6
leaves}, `🧬️schema/🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` (untouched, already
correct thin delegates), `🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,
🔣️component.json,🛰️component.proto}` (top-level Artifact facet), `🏗️builder/🦀️component.rs`,
`🧐️analyzer/🦀️component.rs`, `🎹️composer/🦀️component.rs`, `🚪️io/🦀️component.rs` (left as W4's
structure-only stub, per the brief). `🟦️component.ts` triad-leaf files and `🚪️io/🟦️component.ts`
left as-is (already-correct thin TS facades, `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS` convention).

Scratch files (this ticket folder, not the repo tree): `w2b-presentation-gen-mirrors.py` (facet
mirror + grammar leaf generator, kept for provenance), `w2b-presentation-cargo-check-final.txt`,
`w2b-presentation-foreign-blocker-gitstatus.txt`, `w2b-presentation-scope-grep.txt`.
