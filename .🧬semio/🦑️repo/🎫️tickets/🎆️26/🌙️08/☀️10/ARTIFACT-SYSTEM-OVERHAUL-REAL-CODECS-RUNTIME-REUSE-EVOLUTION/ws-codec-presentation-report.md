# W-S Codec Wave — `stdio.semio.presentation` (`✳️presentation` subset, the 13th and last domain subset)

Real-codec wave for `presentation`, following `ws-codec-workflow-report.md`'s proven pattern and
`ws-codec-document-report.md`'s critical precedent (presentation's `SlideShape::TextBox`/`Table`
cell content embeds `document::DocBlock` directly — document's own real codec is the thing to call
into, never reinvent). Written after real, synchronous, foreground-observed `cargo check`/
`cargo test` runs — every number below was watched, not assumed.

---

## 1. Derive path vs hand-rolled — what actually happened

The derive path was ruled out immediately: `SlideShape` (`#[serde(tag = "shapeKind")]`) and
`PlaceholderKind` (`#[serde(tag = "kind")]`) are both data-carrying tagged enums with heterogeneous
per-variant field sets (`TextBox{frame,blocks}` / `Picture{frame,image}` / `Table{frame,rows}` /
`Placeholder{frame,kind}`; `Title`/`Subtitle`/.../`Other{value}`), and `TextBox`/`Table` transitively
embed `document::DocBlock`, itself the same shape — the
`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` gap brep/document already filed, no
`dsl::DslField`/`DslEnum` impl exists for either. Hand-rolled instead, matching the sibling
`🔺️diff`/`🧬️mutations` facets' pre-existing hand-rolled convention.

**Per-facet pre-wave state check (per the brief's explicit warning — "check both mutations AND diff
carefully, don't assume already-real")**:

- **Snapshot**: `ArtifactDsl::parse_dsl`/`print_dsl` and `ArtifactPack::encode_pack_with`/
  `decode_pack_with` were a hex-of-`serde_json` passthrough (`serde_json::from_slice`/`to_vec`
  called directly) — a real, confirmed policy violation. Fully replaced.
- **Diff**: `print_diff`/`parse_diff` were **already real** hand-rolled hex/bracket text pre-wave
  (confirmed by reading the file directly — this ticket's earlier phase had already landed this).
  `DiffCodec::encode_diff`/`decode_diff` were on the F6 `print_diff().into_bytes()` text-as-binary
  shortcut — needed the binary-frame upgrade only, per the brief's own prediction.
  **A real, separate policy violation was found one level deeper**: the diff facet's own leaf-value
  encoders (`enc_doc_run`/`enc_run_style`/`enc_doc_list_item`/`enc_doc_table_cell`/
  `enc_doc_table_row`/`enc_doc_block`/`dec_doc_block` etc., ~80 lines) were a **local reimplementation**
  of `document::DocBlock`'s codec instead of calling document's own real, already-tested
  `enc_block`/`dec_block` (`document::schema::diff`) — exactly the "presentation reinvents DocBlock
  instead of reusing" trap the brief warned about, confirmed by direct comparison against
  `ws-codec-document-report.md`. **Fixed**: deleted the ~80-line local duplicate, re-exported
  document's real `enc_block`/`dec_block` via `pub(crate) use` in the diff facet (the single import
  point every other facet in this subset already draws its value codecs from), and rewired every
  call site (`enc_table_cell`/`enc_shape`/`enc_slide`/`enc_doc_blocks_diff` and their `dec_*` twins).
- **Mutations**: text codec (`print_presentation_mutation`/`parse_presentation_mutation`) was
  **already real**, 15 variants (`NoMutation` + 14 named — the brief's "14-variant vocabulary" note
  undercounts `NoMutation` itself by one, same undercounting pattern document's own report flagged
  for a sibling subset), confirmed already exercising the diff facet's real value codecs. Only
  `OpBinary::encode_op`/`decode_op` were on the F6 text-as-binary shortcut — binary-frame upgrade
  only, exactly as the brief predicted. (The mutations facet's own `enc_doc_block`/`dec_doc_block`
  imports were transitively fixed by the diff-facet rename below, from `enc_block`/`dec_block`.)

---

## 2. Per-facet checklist (grammar recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 4-line structured body: `schema=<hex>`, `masters=[<master>,...]`, `layouts=[<layout>,...]`,
  `slides=[<slide>,...]`, reusing the diff facet's already-real `enc_str`/`enc_master`/`enc_layout`/
  `enc_slide` value codecs (which themselves reuse document's real `enc_block` for every
  `blocks`/`notes` leaf — snapshot never touches `DocBlock` encoding directly). Replaces the old
  hex-of-`serde_json` passthrough entirely.
- [x] **Real binary pack** — `encode_presentation_snapshot_binary`/`decode_presentation_snapshot_binary`:
  `format u8` + varint-length-prefixed `schema` UTF-8, then varint master/layout/slide counts and
  per-field varint-length-prefixed strings, real 8-byte LE `f64` coordinates, and a real per-variant
  tag byte for `SlideShape`/`PlaceholderKind` (`store::pack_rt`/`store::ByteReader`, no external
  crate, no hand-rolled varint). **Every `Vec<DocBlock>` leaf (`TextBox.blocks`, table cell
  `blocks`, `Slide.notes`) is encoded by calling document's real, already-tested `enc_block`/
  `dec_block` TEXT codec and embedding the result as one length-prefixed UTF-8 blob per block** —
  reuse, not reinvention, satisfying the brief's explicit mandate even at the binary layer (document's
  own `write_block`/`read_block` binary primitives are private to its snapshot module and not
  reachable from presentation, so this is the honest, correct boundary: reuse the real codec that
  IS exposed, rather than hand-duplicating `DocBlock`'s binary shape a second time).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line, tag-prefixed alternation for `shape`/
  `placeholder-kind`), matching `print_presentation_snapshot_body` field-for-field. The `block`
  production family is copied VERBATIM from `document`'s own real, already-conformance-tested
  snapshot grammar (only `row`/`cell` renamed `doc-row`/`doc-cell` to avoid colliding with this
  file's own `row`/`cell` productions for `SlideTableRow`/`SlideTableCell`).
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {format u8}` + real bare `segment schema_len varint` / `segment schema_bytes Array(u8,
  Field(schema_len))` (proven bare form, not the braced form) + one honest opaque `chain payload
  bytes` tail for `masters`/`layouts`/`slides` (`protocol-array-of-records`/
  `protocol-prim-ref-recursion` — `slides` embeds a further recursive `SlideShape`/`DocBlock` union).
  The real Rust encode/decode stays fully structured past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the old ABNF-dialect hex-dump-of-JSON
  placeholder scaffolding to real, descriptive (not test-parsed) mirrors of the new grammar/protocol.
- [x] **Fixtures** — `📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`: genuine
  `print_dsl()`/`encode_pack()` output of a new `demo_semio_presentation_snapshot()` fixture
  (exercises every `SlideShape` variant incl. `Table`, every `PlaceholderKind` variant incl. `Other`,
  and the `document::DocBlock` reuse in `TextBox.blocks`/table cell `blocks`/`Slide.notes`).
  Generated via a temporary `#[test] #[ignore] fn presentation_temp_print_real_fixtures()` in
  `🎹️composer/🦀️component.rs` that wrote the real bytes directly to the fixture files via
  `std::fs::write` (document wave's own safer variant of the recipe's "capture stdout, copy via a
  small script" method — no transcription risk), run once, then **deleted** (confirmed absent in the
  final file).

### Diff (`🔺️diff/`)

- [x] **Text codec leaf values de-duplicated onto document's real codec** — see §1: the ~80-line
  local `enc_doc_run`/`enc_run_style`/`enc_doc_list_item`/`enc_doc_table_cell`/`enc_doc_table_row`/
  `enc_doc_block`/`dec_doc_block` family (and their `dec_*` twins) deleted entirely; every
  `blocks`/`notes` leaf now calls the re-exported `document::schema::diff::{enc_block, dec_block}`
  directly (`pub(crate) use` at the top of the file). Structure (tri-states, collection triples, the
  `Replace` fallback for shape-kind changes) was already real pre-wave and untouched.
- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed pre-wave). Now: `format u8` + `presence u8` (bit0=`masters`, bit1=`layouts`,
  bit2=`slides`) as two real fixed header fields, then 0-3 varint-length-prefixed opaque blobs (the
  same `enc_masters_diff`/`enc_layouts_diff`/`enc_slides_diff` text this facet's own `print_diff`
  already emits, now free of any local `DocBlock` duplication). Same `protocol-cond-cannot-chain`
  rationale as every other semio wave's own diff facet — a second `if`-guard on a field that's
  itself only conditionally decoded hard-errors `eval_cond`.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `master`/`layout`/`slide`/`shape`/`frame`/`image`/`placeholder-kind`/`row`/`cell`/`block` value
  grammars, the tri-state `option-x` pattern for every `Option<T>` diff field (incl. the DOUBLY
  tri-state `slide-diff`'s `layout_id: Option<Option<String>>`), and the collection-triple pattern
  (name-keyed `NamedTripleDiff` for `masters`/`layouts`, index-keyed `IndexedTripleDiff` for
  `slides`/`shapes`/`blocks`/table `rows`/`cells`).
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors — abridged descriptive mirrors (same shape document's own diff
  mirrors use — a generic `'[' .*? ']'` catch-all for nested diff bodies rather than restating the
  full grammar).
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — `snapshot_a()`/
  `snapshot_b()` promoted from the old `mod handcrafted_diff_codec_tests`-local helpers to
  module-scope `#[cfg(test)] pub(crate) fn` (needed by both the composer's conformance laws AND
  the mutations facet's own `demo_mutation_cases()`).

### Mutations (`🧬️mutations/`)

- [x] **Text codec already real pre-wave** — confirmed (see §1): all 15 `SemioPresentationMutation`
  variants already had a genuine `print_presentation_mutation`/`parse_presentation_mutation` keyword
  grammar, reusing the diff facet's real value codecs. Only the import list needed updating
  (`enc_doc_block`/`dec_doc_block` renamed to `enc_block`/`dec_block` to match the diff facet's own
  rename).
- [x] **Binary upgrade** — was on the F6 `print_op().into_bytes()` text-as-binary shortcut. Now:
  `format u8` + `tag u8` (variant ordinal, new `OP_KEYWORDS`/`variant_ordinal`, 0-14 matching
  `parse_presentation_mutation`'s keyword match) as two real fixed fields, then the variant's own
  `key=value ...` argument text as one opaque trailing `bytes` chain (`print_presentation_mutation_args`
  strips the keyword) — reuses the already-real, already-tested text codec, one source of truth.
  `use protocol::{Mutation, OpBinary, OpText};` made unconditional (was `#[cfg(test)]`-gated for
  `OpBinary`) since the real `impl OpBinary` now calls `self.print_op()` in production code.
- [x] Grammar/protocol/mirrors, same treatment as the sibling facets — grammar traced verbatim from
  `print_presentation_mutation`'s real `format!(...)` call sites, never guessed.
- [x] `demo_mutation_cases()` (new module-scope `#[cfg(test)] pub(crate) fn`, one case per variant,
  reusing `snapshot_b()` for `SetSnapshot`) added for the conformance-law tests. The pre-existing
  `mod tests`-local `op_text_binary_roundtrip_law`/`sample_mutations`/`fixture` were left untouched
  (already green, no risk taken).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) written into
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same home every prior semio wave uses (presentation has no per-standard
`⚙️engine/` dir; the shared 14-subset `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` aggregator has no
test module of its own and is out of this wave's `✳️presentation/`-only edit scope).

**All 6 passed on the first real run — no grammar/protocol authoring bugs needed fixing this wave**
(unlike document's `Quote`-bracket bug or drawing's missing `option-bool`), most likely because the
`block`/`row`/`cell` productions were copied verbatim from document's own already-debugged grammar
rather than re-derived from scratch.

### `register_schema_spec` (checklist item, "if unsure, skip and note as follow-up")

**Skipped**, same as every prior hand-rolled semio wave: no derivable `RecordSpec` exists for
`SlideShape`/`PlaceholderKind`'s hand-rolled tagged-enum shape. Filed as a follow-up rather than
fabricated.

### JSON-transfer ban (checklist item 8)

```
$ grep -n "serde_json" 📸️snapshot/🦀️component.rs 🔺️diff/🦀️component.rs 🧬️mutations/🦀️component.rs
📸️snapshot/🦀️component.rs:182:/// copy. Replaces the old hex-of-`serde_json` passthrough.
📸️snapshot/🦀️component.rs:487:/// structured text/binary codecs, replacing the old hex-of-`serde_json` passthrough (both
📸️snapshot/🦀️component.rs:489:/// `serde_json::{to_vec,from_slice}`). The derive path (...) hits the
```
All 3 hits are doc-comment prose describing the OLD, now-replaced shortcut — zero `serde_json::`
calls remain inside any `ArtifactPack`/`OpBinary`/`DiffCodec` impl body. `🔺️diff/🦀️component.rs` and
`🧬️mutations/🦀️component.rs` have **zero** `serde_json` mentions of any kind (not even in comments).

Also ran `bun run ./📜️script.ts policy` and grepped its full breach report for `presentation` —
**zero** JSON-transfer-ban/grammar/protocol/fixture-honesty breaches for this subset; the only two
hits are pre-existing, repo-wide, unrelated patterns already present in all 8 other finished semio
subsets identically (`os-state-authority/item-scope-global` on the composer's `OnceLock` statics —
the same shape workflow/model/brep/cad/drawing/document/image/mesh/object all have at their own
equivalent lines — and one `taxonomy/emoji-prefix` hit on the pre-existing `📄set-snapshot` mutation
subfolder name, unrelated to codecs and pre-dating this wave).

---

## 3. Exact files touched

All paths relative to repo root, base
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/`.

**Snapshot**: `📸️snapshot/🦀️component.rs`, `📸️snapshot/📝️text/📖️component.grammar.semio`,
`📸️snapshot/📝️text/🅰️component.g4`, `📸️snapshot/📝️text/🔤️component.ebnf`,
`📸️snapshot/💾️binary/📡️component.protocol.semio`, `📸️snapshot/💾️binary/🥋️component.ksy`,
`📸️snapshot/💾️binary/🌶️component.spicy`, `📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/📖️component.grammar.semio`,
`🔺️diff/📝️text/🅰️component.g4`, `🔺️diff/📝️text/🔤️component.ebnf`,
`🔺️diff/💾️binary/📡️component.protocol.semio`, `🔺️diff/💾️binary/🥋️component.ksy`,
`🔺️diff/💾️binary/🌶️component.spicy`, `🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `🧬️mutations/🦀️component.rs`, `🧬️mutations/📝️text/📖️component.grammar.semio`,
`🧬️mutations/📝️text/🅰️component.g4`, `🧬️mutations/📝️text/🔤️component.ebnf`,
`🧬️mutations/💾️binary/📡️component.protocol.semio`, `🧬️mutations/💾️binary/🥋️component.ksy`,
`🧬️mutations/💾️binary/🌶️component.spicy`, `🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️presentation/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its
existing `#[cfg(test)] mod tests`; the fixture-generating temp test was added then removed in the
same session — confirmed absent in the final file).

**New example slug** (outside `✳️presentation/`, explicitly permitted by the ticket brief's
deliverable 6, mirroring document's `📚️examples/📄️memo` / workflow's `📚️examples/🌊️pipeline` /
drawing's `📚️examples/🖍️sketch` / brep's `📚️examples/🧊️solid` — none of these are wired into
`📦️glue.rs`/`catalog.json` either, confirmed by grep):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📽️deck/🦀️component.rs`,
`…/📽️deck/🟦️component.ts`, `…/📽️deck/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output,
byte-verified by `fixture_honesty_law`), `…/📽️deck/🖼️assets/🎒️example.pack.semio` (genuine
`encode_pack` bytes, byte-verified).

Nothing outside these was touched — confirmed via `git status --porcelain` scoped to
`✳️presentation/`/`📚️examples/📽️deck/`. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`,
`📦️glue.rs`, `launch.json`, `catalog.json`, `⚙️engine/🧮️geometry`, `⚙️engine/🧰️triples`, and every
other subset (including `✳️document`, which is read-only-referenced here via its `pub(crate)`
`enc_block`/`dec_block` functions — its own files were never touched) were left untouched.

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `masters`/`layouts`/`slides` — `slides` is a homogeneous variable-length repeated record whose own `shapes: Vec<SlideShape>` is a further tagged union. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-prim-ref-recursion` | yes, §5 | `SlideShape::TextBox`/`Table` embed `document::DocBlock`, itself a further recursive tagged union — folded into the same opaque `payload` tail. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `masters`/`layouts`/`slides` — 3 independently-optional segments; used one opaque `chain payload bytes` with a real 3-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled tagged-enum types). |
| `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` | no (brep's own, re-confirmed here) | `SlideShape`/`PlaceholderKind` are data-carrying tagged enums with heterogeneous per-variant field sets — no derive-path route to a matching text grammar production set. |
| **`cross-subset-block-codec-binary-not-exposed`** (NEW — not in recipe's table) | no | `document`'s real DocBlock **binary** primitives (`write_block`/`read_block`, `📸️snapshot/🦀️component.rs`) are private (`fn`, not `pub(crate)`) — only the TEXT codec (`enc_block`/`dec_block`, `🔺️diff/🦀️component.rs`) is `pub(crate)` and cross-subset-reachable. Presentation's own binary pack therefore embeds each `DocBlock` as a length-prefixed UTF-8 blob of document's real TEXT encoding, rather than a byte-for-byte reuse of document's binary tag scheme. This is the honest, correct choice (reuse what's actually exposed, don't duplicate a private implementation) but is worth flagging: any future subset embedding another subset's tagged-enum type in a BINARY pack will hit this identically unless that type's own binary writer/reader is also made `pub(crate)` centrally. |

---

## 5. Verified green — real command output, observed in this session

All commands below were run directly, synchronously, in the foreground in this session, and their
real output was read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** → `Finished` **0 errors** (491 pre-existing warnings,
   none attributable to presentation's own files). Confirmed via a second, `grep -E "^error"`-only
   run — zero matches, both before and after the grammar/protocol/fixture work.

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::presentation"`**
   → **34 passed, 0 failed, 0 ignored**, "finished in 0.17s" (final confirming run). Includes all 6
   conformance-law tests individually confirmed `ok`: `committed_facet_files_parse`,
   `grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
   `protocol_walk_law`, `fixture_honesty_law` — **all 6 passed on the FIRST run**, no
   grammar-authoring bugs needed fixing this wave (attributed in §2 to copying document's already-
   debugged `block` grammar verbatim rather than re-deriving it). Plus every pre-existing
   presentation test — pptx import/export round trips, builder, analyzer, referential-invariant
   validator, diff/mutation algebra laws (`absorb_law`'s associativity check, `field_sweep`,
   `shape_kind_change_produces_replace_and_round_trips` for the `Replace` fallback) — all green.

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) → **1922 passed, 0 failed, 3
   ignored**, "finished in 10.17s" (final confirming run) — **zero regressions anywhere in the
   crate**. One transient, unrelated failure was observed and explicitly NOT chased mid-session,
   exactly per the ticket's own concurrent-development ground rules: a single run showed
   `artifacts::semio::standards::v1::subsets::video::composer::tests::conformance_laws::
   fixture_honesty_law` failing on a literal `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"` string in
   `video`'s own shipped fixture — a different, concurrently-worked sibling subset per the ticket's
   own "sibling agents are concurrently upgrading video/audio/animation" note. A re-run ~20s later
   showed **0 failures** — it self-resolved without any action from this session.

4. **`bun run ./📜️script.ts policy`** — ran and grepped the full breach report for `presentation`;
   zero JSON-transfer-ban/grammar/protocol/fixture-honesty breaches for this subset (§2's own JSON-
   transfer-ban section has the full detail).

**Status: this wave is genuinely proven, fully green for `✳️presentation`'s own scope — the 13th and
last of the semio program's domain subsets to land real codecs.**

---

## 6. Notes for whoever closes out the program

1. **Always check whether a sibling facet's structure is ALREADY real before assuming a uniform
   3-facet rewrite is needed** — this wave's actual work was narrower than the brief's own worst-case
   framing: diff's STRUCTURE and mutations' TEXT codec were both already real pre-wave (from this
   ticket's own earlier "14-variant vocabulary" phase, confirmed still true per the brief's own
   instruction to verify rather than assume). The real net-new work was: snapshot's full
   text+binary rewrite, diff's binary-frame upgrade, mutations' binary-frame upgrade, and — the one
   genuine surprise — de-duplicating diff's own leaf-value `DocBlock` codec onto document's real one.
2. **"REUSE, don't reinvent" needs checking at EVERY layer, not just the obvious top-level call
   site** — presentation's snapshot facet never had its own `DocBlock` codec (it never had a real
   codec at all, pre-wave), so the obvious risk (a fresh hand-rolled `enc_block` in the NEW code)
   was avoided by construction. The actual violation was in a facet that was ALREADY "real" by a
   shallow read (diff's structure/tri-states/collection-triples) — its own LEAF value encoders had
   quietly reinvented `DocBlock` before this wave even started. Grep for the target type's own name
   (`DocBlock`) across every already-real file, not just the files this wave is about to write.
3. **A cross-subset codec dependency's BINARY primitives may not be `pub(crate)` even when its TEXT
   primitives are** — `document`'s own `write_block`/`read_block` are private; only `enc_block`/
   `dec_block` (text) are `pub(crate)`. When a future subset needs to embed another subset's
   tagged-enum type in a binary pack and only the text codec is reachable, the honest answer is:
   reuse the text codec, embed it as a length-prefixed blob inside the binary frame (§4's new gap
   row) — NOT to hand-duplicate the other subset's private binary shape just to get a "purer" binary
   frame. A future centrally-scoped ticket could make `write_block`/`read_block` (and any analogous
   per-subset binary primitive) `pub(crate)` to close this gap generally.
4. **Copying an already-debugged sibling grammar's sub-production verbatim (not just imitating its
   style) pays off directly** — this wave's `block`/`run`/`run-style`/`list-item`/`doc-row`/
   `doc-cell` productions were copied character-for-character from document's own real, already-
   conformance-tested grammar (renaming only the two names that collided with this file's own
   `row`/`cell`), and `grammar_conformance_law` passed on the very first run for all three facets —
   no bracket-depth bug, no missing tri-state production, unlike document's own first pass (`Quote`
   bracket bug) or drawing's own first pass (missing `option-bool`).
