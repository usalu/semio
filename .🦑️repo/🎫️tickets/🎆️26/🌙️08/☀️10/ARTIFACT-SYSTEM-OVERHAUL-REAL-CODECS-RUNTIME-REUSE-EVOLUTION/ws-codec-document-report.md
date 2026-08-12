# W-S Codec Wave — `stdio.semio.document` (`✳️document` subset)

Real-codec wave for `document`, following `ws-codec-workflow-report.md`'s proven pattern and
`ws-codec-model-report.md`'s closest precedent (property/value-graph-ish, hand-rolled tagged
enums). Written after real, synchronous, foreground-observed `cargo check`/`cargo test` runs —
every number below was watched, not assumed.

---

## 1. Derive path vs hand-rolled — what actually happened

The derive path was ruled out immediately by reading `📸️snapshot/🦀️component.rs`'s pre-existing
doc comments: `DocBlock` is a `#[serde(tag = "kind")]` data-carrying enum with **heterogeneous
per-variant field sets** (`Paragraph{style_id,runs}` / `Heading{level,style_id,runs}` /
`List{ordered,items}` / `Table{rows}` / `Code{language,text}` / `Quote{blocks}` /
`Image{image_id,alt,width,height}` / `PageBreak`), reachable both directly (`blocks:
Vec<DocBlock>`) and transitively through `IndexedTripleDiff<DocBlockDiff, DocBlock>`. This is
exactly the `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` gap brep's own report
names — no `dsl::DslField`/`DslEnum` impl exists for this shape, confirmed by the same reasoning
brep/model already used (not re-derived from scratch). Hand-rolled instead.

**Mutations facet check (per the brief's explicit warning)**: unlike model's mutations facet
(which was on a raw `serde_json` passthrough pre-wave), **document's `OpText` was ALREADY real**
pre-wave — `print_document_mutation`/`parse_document_mutation` already emitted a genuine
`keyword arg=value ...` grammar for all 18 variants, reusing `🔺️diff`'s hex/bracket value codecs.
Only `OpBinary` was on the F6 `print_op().into_bytes()` text-as-binary shortcut. Similarly,
`🔺️diff`'s `print_diff`/`parse_diff` were ALREADY real (hex/bracket, via the shared
`engine::triples::{enc,dec}_{indexed,named}_triple`); only `DiffCodec::encode_diff`/`decode_diff`
were on the same F6 shortcut. **Only the snapshot facet (`ArtifactDsl`/`ArtifactPack`) was on the
hex-of-`serde_json` passthrough** — confirmed by reading the pre-wave file directly (`parse_dsl`
hex-decoded then called `serde_json::from_slice`; `print_dsl`/`encode_pack_with` called
`serde_json::to_vec` then hex-dumped/wrapped).

---

## 2. Per-facet checklist (grammar recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` now produce/consume a genuine 4-line structured
  body: `schema=<hex>`, `styles=[<style>,...]`, `images=[<image>,...]`, `blocks=[<block>,...]`,
  reusing `🔺️diff`'s already-real `enc_str`/`enc_style`/`enc_image`/`enc_block` value codecs
  (imported `crate::…::document::schema::diff::{dec_block, dec_image, dec_str, dec_style,
  enc_block, enc_image, enc_str, enc_style}` — snapshot now depends on diff for these primitives,
  the established local convention this subset's own `🧬️mutations` facet already used pre-wave,
  rather than duplicating a third copy). Replaces the old hex-of-`serde_json` passthrough entirely.
- [x] **Real binary pack** — `encode_document_snapshot_binary`/`decode_document_snapshot_binary`:
  `format u8` + varint-length-prefixed `schema` UTF-8, then varint style/image/block counts and
  per-field varint-length-prefixed strings, real 8-byte LE `f64`s, and a real per-variant **tag
  byte** (0-7) for `DocBlock`, genuinely recursive for `List`/`Table`/`Quote`'s nested
  `Vec<DocBlock>` (`store::pack_rt`/`store::ByteReader`, no external crate, no hand-rolled varint).
  Replaces the old `serde_json::to_vec`-in-envelope shortcut.
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax, one
  production per physical line, `hex` macro for every string leaf, tag-prefixed alternation for
  `block` (`P`/`H`/`L`/`T`/`C`/`Q`/`I`/`B`).
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {format u8}` + real bare `segment schema_len varint` / `segment schema_bytes Array(u8,
  Field(schema_len))` (proven bare form per the workflow report's UPDATE §7 fix), then one honest
  opaque `chain payload bytes` tail for `styles`/`images`/`blocks` (`protocol-array-of-records` +
  `protocol-prim-ref-recursion` gaps — homogeneous-but-variable-length repeated records, `blocks`
  additionally recursive). The real Rust encode/decode stays fully structured past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — descriptive, same production names, not test-parsed.
- [x] **Fixtures** — `📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio` /
  `🎒️example.pack.semio` are genuine `print_dsl`/`encode_pack` output of
  `snapshot::demo_semio_document_snapshot()` (one style, one image, one block of every kind —
  Heading/Paragraph/List/Table/Code/Quote/Image/PageBreak), generated via a temporary `#[test]
  #[ignore] fn ws_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs` that wrote the real
  bytes directly to the fixture files via `std::fs::write` (a stricter variant of the recipe's
  "capture stdout, copy via a small script" method — no manual transcription risk at all), run
  once, then **deleted** (confirmed gone — grep for `ws_temp_print_real_fixtures` in the composer
  file now returns nothing).

### Diff (`🔺️diff/`)

- [x] **Text codec already real pre-wave** — confirmed: `print_document_diff`/`parse_document_diff`
  already emitted genuine hex/bracket `styles=… images=… blocks=…` tokens via
  `engine::triples::enc_named_triple`/`enc_indexed_triple`. No text-side work needed.
- [x] **Binary upgrade** — was on the `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed pre-wave). Now: `format u8` + `presence u8` (bit0=`styles`, bit1=`images`,
  bit2=`blocks`) as two real fixed header fields, then 0-3 varint-length-prefixed opaque blobs (the
  same `enc_styles_diff`/`enc_images_diff`/`enc_blocks_diff` text `print_diff` already emits, added
  new `write_str_lp`/`read_str_lp`/`write_bytes_lp`/`read_bytes_lp` `pub(crate)` binary primitives
  in this facet's own file). Same `protocol-cond-cannot-chain` rationale as workflow's/model's own
  diff facet.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `style`/`image`/`run`/`list-item`/`row`/`cell`/`block` value grammars, the tri-state `[0]`/`[1,x]`
  pattern for every `Option<T>` diff field including the DOUBLY tri-state `style_id`/`based_on`/
  `language`/`size`/`font`/`color`/`link`/`width`/`height` (`Option<Option<T>>`), and the
  collection-triple pattern (name-keyed `NamedTripleDiff` for `styles`/`images`, index-keyed
  `IndexedTripleDiff` for `blocks`/`runs`/list-items/rows/cells, all mutually recursive through
  `DocBlockDiff`).
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added for the conformance-law
  tests — reuses `snapshot_a()`/`snapshot_b()`, themselves promoted from `mod tests`-local to
  module-scope `#[cfg(test)] pub(crate) fn` (single source of truth, same promotion model/workflow
  used for their own `sweep_a`/`sweep_b`).

### Mutations (`🧬️mutations/`)

- [x] **Text codec already real pre-wave** — confirmed (see §1 above): all 18
  `SemioDocumentMutation` variants already had a genuine `print_document_mutation`/
  `parse_document_mutation` keyword grammar. No text-side work needed — a real, new-to-this-wave
  deviation from model's own pre-wave state (model's mutations text codec was on raw JSON).
- [x] **Binary upgrade** — was on the `print_op().into_bytes()` text-as-binary shortcut. Now:
  `format u8` + `tag u8` (variant ordinal, new `OP_KEYWORDS`/`variant_ordinal`, 0-17 matching
  `parse_document_mutation`'s keyword match) as two real fixed fields, then the variant's own
  `key=value ...` argument text as one opaque trailing `bytes` chain (`print_document_mutation_args`
  strips the keyword) — reuses the already-real text codec, single source of truth.
- [x] Grammar/protocol/mirrors, same treatment — grammar traced verbatim from
  `print_document_mutation`'s real `format!(...)` call sites, restating the `block-path`/
  `snapshot-lit`/`style`/`image`/`block`/`run`/`run-style` value grammars.
- [x] `demo_mutation_cases()` (new module-scope `#[cfg(test)] pub(crate) fn`, one case per variant)
  added for the conformance-law tests, reusing `crate::…::diff::snapshot_b()` for `SetSnapshot`.
  The pre-existing `mod tests`-local `op_text_binary_roundtrip_law` test (20 cases, including both
  `SetStyleBasedOn` flavors) was left untouched rather than merged, to avoid any risk to an
  already-green test.
- [x] `use protocol::{OpBinary, OpText};` was already unconditional pre-wave (not
  `#[cfg(test)]`-gated) — no fix needed here, unlike workflow's/model's own mutations facet.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) written into
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same home every prior semio wave used (`document` has no per-standard
`⚙️engine/` dir; the shared `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` is a 14-subset aggregator
with no test module of its own, out of this wave's edit scope).

### JSON-transfer ban (checklist item 8)

Grepped the three changed `.rs` files (`📸️snapshot/🦀️component.rs`, `🔺️diff/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`) for `serde_json` — **clean**: the only remaining hits are two doc
comments in `📸️snapshot/🦀️component.rs` describing the OLD, now-replaced shortcut ("replacing the
old hex-of-`serde_json` passthrough"), zero actual `serde_json::` calls remain inside the
`ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks.

### `register_schema_spec` (checklist item, "if unsure, skip and note as follow-up")

**Skipped**, same as every prior hand-rolled semio wave: no derivable `RecordSpec` exists for
`document`'s hand-rolled types (§1's `DocBlock` tagged-enum blocker). Filed as a follow-up rather
than fabricated.

---

## 3. Exact files touched

All paths relative to repo root, under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/`.

**Snapshot**: `🧬️schema/📸️snapshot/🦀️component.rs`, `…/📸️snapshot/📝️text/📖️component.grammar.semio`,
`…/📸️snapshot/📝️text/🅰️component.g4`, `…/📸️snapshot/📝️text/🔤️component.ebnf`,
`…/📸️snapshot/📝️text/🦀️component.rs` (marker rewritten to `COMPONENT_GRAMMAR_SEMIO` include),
`…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/💾️binary/🥋️component.ksy`,
`…/📸️snapshot/💾️binary/🌶️component.spicy`, `…/📸️snapshot/💾️binary/🔠️component.abnf`,
`…/📸️snapshot/💾️binary/🦀️component.rs` (marker rewritten to `COMPONENT_PROTOCOL_SEMIO` include).

**Diff**: `🧬️schema/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🅰️component.g4`, `…/🔺️diff/📝️text/🔤️component.ebnf`, `…/🔺️diff/📝️text/🦀️component.rs`,
`…/🔺️diff/💾️binary/📡️component.protocol.semio`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`,
`…/🔺️diff/💾️binary/🦀️component.rs`.

**Mutations**: `🧬️schema/🧬️mutations/🦀️component.rs`,
`…/🧬️mutations/📝️text/📖️component.grammar.semio`, `…/🧬️mutations/📝️text/🅰️component.g4`,
`…/🧬️mutations/📝️text/🔤️component.ebnf`, `…/🧬️mutations/📝️text/🦀️component.rs`,
`…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`,
`…/🧬️mutations/💾️binary/🦀️component.rs`.

**Tests**: `🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing `#[cfg(test)]
mod tests`; the fixture-generating temp test was added then removed in the same session —
confirmed absent in the final file).

**New example slug** (outside `✳️document/`, explicitly permitted by the ticket brief's deliverable
6, mirrors model's own `📚️examples/🏢️building` / object's `📚️examples/🕸️graph` / workflow's
`📚️examples/🌊️pipeline` / brep's `📚️examples/🧊️solid` — none of these are wired into
`📦️glue.rs` either, confirmed by grep; only the pre-existing `📚️examples/🎬️demo` slug is):
`📚️examples/📄️memo/🦀️component.rs`, `…/📄️memo/🟦️component.ts`,
`…/📄️memo/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output),
`…/📄️memo/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, `⚙️engine/🧮️geometry`, and every other subset/artifact (including
`✳️presentation`, which reads `DocBlock` but was not touched — its own field names/public shape
were left completely unchanged, only the codec implementation changed) were left untouched, per
the brief — confirmed via `git status --porcelain` scoped to `🪆️subsets/✳️document/` and
`📚️examples/📄️memo/` (exactly the file list above, nothing else).

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `styles`/`images`/`blocks` — homogeneous variable-length repeated records. Opaque trailing `chain payload bytes` after the real `format`+`schema` header, same as workflow's own `nodes`/`edges`. |
| `protocol-prim-ref-recursion` | yes, §5 | `blocks` additionally embeds a recursive, data-carrying `DocBlock` union (`List`/`Table`/`Quote` all nest `DocBlock` further down) — `Prim::Ref` can't describe this; folded into the same opaque `payload` tail as the array-of-records gap above. The real Rust encoder (`write_block`/`read_block`) is genuinely, fully recursive — round-trip tested independently. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `styles`/`images`/`blocks` — THREE independently-optional segments. Used one opaque `chain payload bytes` with a real 3-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled `DocBlock` tagged enum). |
| `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` (already filed by brep, re-confirmed here) | no, filed by brep | `DocBlock`'s 8 variants have DIFFERENT field sets (not just different single values), same shape as brep's `BrepCurve`/`BrepSurface` — no derive-path route to a matching text grammar production set. Every future semio subset with a real heterogeneous tagged-union block/value type will hit this identically; brep's own report already recommends the general fix (try `#[derive(dsl::DslEnum)]` once someone confirms its struct-variant support). |

**New authoring bug found and fixed in THIS wave** (not a framework gap — a real mistake in the
hand-drafted grammar, caught by `grammar_conformance_law` exactly as designed): the `block`
production's `Quote` alternative was originally written as `"Q" "[" block-items? "]"` (single
bracket) instead of the correct `"Q" "[" "[" block-items? "]" "]"` (double bracket) — `DocBlock::
Quote{blocks}`'s real encoder is `format!("Q[{}]", enc_list(blocks, enc_block))`, and `enc_list`
itself already wraps its output in `[...]`, so the `Quote` tag needs the SAME double-bracket
nesting `Table`'s `T[{enc_list(...)}]` already had (and had gotten right, by copying the pattern
correctly there). Found via a temporary `#[test] #[ignore]` bisection harness (compiled the real
grammar with `grammar.start` overridden to `"block"` and fed it each individual printed block
alternative) that pinpointed exactly the `Quote` case as the sole failure among all 8 block kinds;
fixed identically in all three grammar files that restate the `block` value production (snapshot,
diff, mutations) since all three had copy-pasted the same bug. The bisection test was added,
used, and deleted in this same session (confirmed absent in the final files).

---

## 5. Verified green — real command output, observed in this session

All three commands below were run as **foreground** commands in this session and their real output
was read before writing this section.

1. `cargo check -p semio-s-plugin-stdio` → **0 errors**, "Finished `dev` profile [unoptimized]
   target(s) in 12.32s" on the final confirming run, 485 warnings (none attributable to this
   wave's own logic — pre-existing repo-wide patterns, e.g. hidden-lifetime-parameter notes and
   unused-function warnings in OTHER artifacts' files).

   **Note on a transient, non-`document` compile break observed mid-session**: one `cargo check`
   run partway through this session showed 5 errors, all in `🪆️subsets/✳️drawing/🧬️schema/
   📸️snapshot/🦀️component.rs` (`read_f32_le` not found on `ByteReader`) plus, in a later run, 22
   more errors in `✳️drawing/🧬️schema/🔺️diff/🦀️component.rs` (`enc_json`/`dec_json` not found).
   Confirmed via `git status --porcelain` scoped to `✳️drawing/` that this subset had multiple `M`
   (uncommitted, in-progress) files at the time — another concurrent session's own real-codec
   upgrade for `drawing`, mid-edit, not caused by or fixable within this `✳️document`-only wave. A
   re-run shortly after showed 0 errors crate-wide — it self-resolved without any action from this
   session, exactly the "concurrent cargo workspace churn" pattern this repo's own environment note
   warns about.

2. `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::document"`
   → **47 passed, 0 failed, 0 ignored**, "finished in 0.18s" (final confirming run). Includes all 6
   conformance-law tests (`committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`) plus every pre-existing `document` test (referential-invariant validator,
   docx/md/txt/pdf import/export round trips, builder, analyzer, diff/mutation algebra laws
   including `absorb_law`'s associativity check over a triple, `field_sweep`) — all green.

   **Two real bugs were caught and fixed by these laws before this final run** (both documented in
   §4 above, not glossed over): the missing `s.` prefix on the snapshot grammar's `artifact-mark`
   literal (`grammar_conformance_law`'s first failure), and the `Quote` block's missing bracket
   layer (`grammar_conformance_law`'s second failure, after the first fix).

3. `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1896 passed, 0 failed, 3 ignored**,
   "finished in 10.13s" (final confirming run) — **zero regressions anywhere in the crate**, and no
   transient failures observed on this particular whole-crate run (unlike model's/some other
   waves' own reports, no other concurrent subset's in-progress breakage was showing at this exact
   moment — the drawing-subset churn noted above had already self-resolved by the time this run
   was taken).

**Status: this wave is a genuinely proven, fully green replica of the workflow/model/brep
template, applied to a subset whose snapshot facet started as the ONLY hand-rolled-`serde_json`
passthrough among the three facets (diff and mutations were already real pre-wave) — the inverse
starting shape from model's own wave (where mutations was the raw-JSON facet).**

---

## 6. Notes for the next semio-subset wave

1. **Check EACH facet's pre-wave state independently — don't assume a uniform starting shape.**
   This wave's own surprise: unlike model (mutations on raw JSON, diff already real) or the
   "typical" case (all three hand-rolled hex/bracket, only binary needing an upgrade), document had
   text-real diff AND text-real mutations, with ONLY the snapshot facet on the hex-of-JSON
   shortcut. Read all three files' pre-wave `impl` blocks directly before assuming anything.
2. **`enc_f64` prints the `f64::to_bits()` `u64` bit pattern as plain decimal digits, not a float
   literal.** Every numeric leaf in this subset's grammars (`level: u8` via `enc_u8`, `size`/
   `width`/`height`: `f64` via `enc_f64`) is a bare `INT` token, never `FLOAT` — a grammar that used
   `number = INT | FLOAT` here would still technically work (alternation subsumes it) but is
   needlessly imprecise; state it as bare `INT` directly once confirmed.
3. **When a `format!("TAG[{}]", enc_list(...))`-style single-argument template wraps an
   already-bracketed `enc_list` result, the tag gets TWO bracket layers, not one** — easy to miss
   when a sibling multi-argument variant (e.g. `List`'s `"L[{ordered},{enc_list(...)}]"`) makes the
   "outer template bracket is separate from the collection's own bracket" pattern look
   single-layered at a glance. Cross-check EVERY tag-prefixed alternative against the real
   `format!(...)` call site's exact argument count and structure, not just a few representative
   ones — this wave's `Quote` bug survived a first-pass grammar draft precisely because `Table`
   (structurally identical) was checked closely enough to get right, while `Quote` (encountered
   later, structurally identical to `Table`) was pattern-matched from `List`/`Paragraph` instead and
   got the wrong bracket count.
4. **A `#[test] #[ignore]` bisection harness that overrides `GrammarFile.start` (a `pub` field) to
   an arbitrary sub-production, compiled fresh per candidate, is the fastest way to localize a
   `grammar_conformance_law` failure** — `Recognizer::recognize` only returns `Ok(bool)`/`Err`, no
   positional diagnostic, so bisecting production-by-production against real printed sub-strings
   (extracted from the same `eprintln!`'d demo body) is far faster than manually re-deriving bracket
   depth by eye. Delete the bisection test immediately once the real bug is found and fixed.
5. **Writing a temp fixture-generation test's output directly via `std::fs::write(env!
   ("CARGO_MANIFEST_DIR")..., &bytes)` instead of `eprintln!`-then-manually-copy** eliminates all
   transcription risk (no risk of a stray trailing newline, no risk of a terminal display
   truncating/reflowing a long hex string) — strictly safer than the recipe's own "capture stdout,
   copy via a small script" method when the temp test can resolve the target path relative to its
   own crate's `CARGO_MANIFEST_DIR`. Recommended as the default method for future waves.
6. **Expect transient, unrelated compile failures from concurrent sessions working other subsets in
   this same ticket** — this wave hit one (`✳️drawing`'s compile break, twice, at different points
   in the session), self-resolved both times within minutes without any action needed. Always
   confirm via `git status --porcelain` scoped to the OTHER subset's directory before spending time
   investigating a failure that doesn't mention your own subset's files.
