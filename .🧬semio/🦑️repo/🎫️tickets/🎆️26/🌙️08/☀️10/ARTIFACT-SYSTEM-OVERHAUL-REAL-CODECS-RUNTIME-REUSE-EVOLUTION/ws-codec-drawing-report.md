# W-S Codec Wave — `stdio.semio.drawing` (`✳️drawing` subset)

Real-codec wave for **semio**'s `drawing` subset, following the proven, fully-verified `✳️workflow`
pilot (`ws-codec-workflow-report.md`) and `✳️brep` wave (`ws-codec-brep-report.md`, the closest
precedent per the brief — brep's tagged-enum `BrepCurve`/`BrepSurface` blocker generalizes here to
`PathSegment`/`DrawNode`, with `DrawNode` additionally RECURSIVE via `Group.children`). Scope:
`✳️drawing`'s three facets (snapshot, diff, mutations), plus a new example fixture slug.

**Status: fully verified green, in this session, synchronously — no deferred/unverified claims.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per the brief, the `#[derive(dsl::DslArtifact)]` path was reconsidered now that the 6 shared
`⚙️engine/🧮️geometry` value types (`SemioPoint2/3`, `SemioUv`, `SemioRgba`, `SemioQuaternion`,
`SemioTransform`) all derive `dsl::DslRecord`. It remains blocked here, for the SAME shape brep's
own report identified (`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path`): `PathSegment`
(`MoveTo`/`LineTo`/`CubicTo`/`QuadTo`/`ArcTo`/`Close`) and `DrawNode` (`Path`/`Text`/`Group`/`Image`)
are data-carrying tagged enums whose variants hold different field sets, and `DrawNode` is
additionally genuinely RECURSIVE (`Group.children: Vec<DrawNode>`) — no `DslEnum`-over-
heterogeneous-recursive-payload mechanism exists. Hand-rolled instead: single-letter tag prefix per
variant (`M[to]`/`L[to]`/`C[c1,c2,to]`/`Q[c,to]`/`A[rx,ry,xRotation,largeArc,sweep,to]`/`Z` for
`PathSegment`; `P[...]`/`T[...]`/`G[...]`/`I[...]` for `DrawNode`), same convention brep's own
`enc_curve`/`enc_surface` established, generalized here to a genuinely recursive `enc_node`/
`dec_node` pair (`Group`'s own `children` field calls `enc_node`/`dec_node` on itself).

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (replacing the old
hex-of-`serde_json` passthrough entirely), with the primitive value codecs (`enc_str`/`enc_rgba`/
`enc_point2`/`enc_transform`/`enc_path_segment`/`enc_node`/`enc_style`/`enc_layer`/`enc_canvas`, all
`pub(crate)`) living in `📸️snapshot/🦀️component.rs` — since `DrawNode`/`PathSegment`/`DrawStyle`/
`DrawLayer`/`DrawCanvas` are all OWNED by the snapshot facet, and `🔺️diff`/`🧬️mutations` both already
depend on `schema::snapshot` for the plain types, importing the codec functions from there too
(rather than duplicating them per facet) keeps exactly one source of truth for the entity encoding
across all three facets — a small deviation from brep's own per-facet-duplication convention,
justified because drawing's own dependency direction (snapshot has zero dependents among its
siblings' own types) makes single-sourcing safe without inventing a reverse dependency.

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 4-line structured body: `schema=<hex>`, `canvas=<canvas>`, `styles=[<style>,...]`,
  `layers=[<layer>,...]`. Every `style`/`layer`/`node`/`segment` is real hex/bracket-encoded value
  text (tag-prefixed for the two tagged enums) — never a hex dump of a JSON blob.
- [x] **Real binary pack** — `encode_drawing_snapshot_binary`/`decode_drawing_snapshot_binary`:
  `format u8` + varint-length-prefixed `schema` UTF-8, then real fixed-field `canvas`, varint counts
  + per-style/per-layer fields (varint-length-prefixed strings, real 8-byte LE `f64`/4-byte LE `f32`
  fields, and a real per-variant tag byte for `PathSegment`/`DrawNode`, `DrawNode`'s own `Group`
  variant recursing into `write_node`/`read_node` for its `children`). Replaces the old
  `serde_json::to_vec`-in-envelope shortcut entirely (`store::pack_rt`/`store::ByteReader`, no
  external crate, no hand-rolled varint). One real bug caught and fixed: `store::ByteReader` has no
  native `f32` reader (only `f64_le`) — added a local `read_f32_le` (4 raw bytes,
  `f32::from_le_bytes`), same fix mesh's own wave independently needed.
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line, tagged alternation for `node`/
  `segment`), matching `print_drawing_snapshot_body` field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes Array(u8,
  Field(schema_len))` (proven bare form, not the braced form), then one honest opaque `chain payload
  bytes` tail for `canvas`/`styles`/`layers` (`protocol-array-of-records` gap — `layers` embeds a
  further RECURSIVE `DrawNode` tree). The real Rust encode/decode stays fully structured past that
  point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the OLD ABNF-dialect hex-dump-of-JSON
  placeholder scaffolding to real, descriptive (not test-parsed) mirrors of the new grammar/protocol.
- [x] **Fixtures** — `📚️examples/🖍️sketch/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`:
  genuine `print_dsl()`/`encode_pack()` output of a NEW `demo_drawing_snapshot()` fixture (exercises
  every `PathSegment` variant — incl. `CubicTo`/`QuadTo`/`ArcTo` — and every `DrawNode` variant incl.
  a nested empty `Group`, chosen deliberately richer than the pre-existing `sample()` test fixture to
  stress the encoder's least-tested corners). Generated via a temporary `#[test] #[ignore] fn
  drawing_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs` that `eprintln!`'d both outputs,
  run once, bytes extracted with a small Python script (never hand-transcribed), temp test then
  deleted (confirmed absent in the final file).

### Diff (`🔺️diff/`)

- [x] **Text codec leaf values upgraded off `serde_json`** — pre-wave, the diff facet's STRUCTURE
  (tri-states, collection triples, recursive node tags) was already hand-rolled real text, but every
  compound LEAF value (`SemioRgba`/`SemioPoint2`/`SemioTransform`/`Vec<PathSegment>`/whole `DrawNode`/
  `DrawLayer`/`DrawStyle`) went through `enc_json`/`dec_json` — hex-encoded `serde_json`, a real
  JSON-transfer-ban violation the brief specifically warned to check for. Fixed: every leaf now calls
  the sibling `📸️snapshot` facet's real primitives (`enc_rgba`/`enc_point2`/`enc_transform`/
  `enc_path_segment`/`enc_node`/`enc_style`/`enc_layer`, imported), `enc_json`/`dec_json` deleted
  entirely from this file.
- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed pre-wave). Now: `format u8` + `presence u8` (bit0=`canvas`, bit1=`styles`,
  bit2=`layers`) as two real fixed header fields, then 0-3 varint-length-prefixed opaque blobs (the
  same `enc_canvas`/`enc_named_triple`/`enc_indexed_triple` text this facet's own `print_diff`
  already emits, now free of `serde_json`). Same `protocol-cond-cannot-chain` rationale as workflow's/
  brep's own diff facets.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `style`/`layer`/`node`/`segment` value grammars, the tri-state `option-x` pattern for every
  `Option<T>` diff field (incl. the DOUBLY tri-state `background`/`fill`/`stroke`/`stroke_width`/
  `opacity`), the collection-triple pattern for `styles` (name-keyed `NamedTripleDiff`) and `layers`
  (index-keyed `IndexedTripleDiff`), and a further NESTED index-keyed triple for `Group.children`
  (genuinely recursive `node-diff` production). **One real bug caught and fixed by
  `diff_grammar_conformance_law`**: the grammar was missing an `option-bool` production
  (`layer-diff`'s `visible: Option<bool>` field prints via the generic `[0]`/`[1,0-or-1]` tri-state
  pattern, not the `bool` production directly) — added `option-bool = "[" "0" "]" | "[" "1" "," bool
  "]"`; the g4/ebnf mirrors already had it (written correctly the first time), only the real
  `.grammar.semio` file was missing it.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors — rewritten field-for-field to match the new real grammar/
  protocol.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — `sweep_a()`/
  `sweep_b()`/`transform()` promoted from `#[cfg(test)] mod tests`-local to module-scope `#[cfg(test)]
  pub(crate) fn` (a private item of `mod tests` is not visible to the sibling `composer` module —
  same real, documented pattern brep's own report flags).

### Mutations (`🧬️mutations/`)

- [x] **Real text codec — NOT already real, confirmed a genuine policy violation.** Pre-wave,
  `OpText::print_op`/`parse_op` were a plain `serde_json::to_string`/`from_str` whole-enum passthrough
  (matching model's own report's warning: "don't assume the mutations facet started real just because
  the sibling diff facet did" — drawing's diff facet WAS structurally real pre-wave, but mutations was
  strictly less-real). Replaced with a real `keyword arg=value ...` grammar
  (`print_semio_drawing_mutation`/`parse_semio_drawing_mutation`), one clause per all 18
  `SemioDrawingMutation` variants (incl. `NoMutation`), reusing the sibling `📸️snapshot` facet's real
  value codecs.
- [x] **Binary upgrade** — was `serde_json::to_vec`/`from_slice` of the whole enum (JSON-transfer-ban
  violation, confirmed and fixed). `format u8` + `tag u8` (variant ordinal, `OP_KEYWORDS`/
  `variant_ordinal`, 0-17 matching `parse_semio_drawing_mutation`'s keyword match) as two real fixed
  fields, then the variant's own `key=value ...` argument text as one opaque trailing `bytes` chain —
  reuses the real, tested `print_semio_drawing_mutation`/`parse_semio_drawing_mutation` text codec
  (one source of truth).
- [x] Grammar/protocol/mirrors — grammar traced verbatim from `print_semio_drawing_mutation`'s real
  `format!(...)` call sites (never guessed); a new `snapshot-lit`/`enc_drawing_snapshot`/
  `dec_drawing_snapshot` production+function pair for `SetSnapshot`'s whole-snapshot payload
  (`[hex(schema),canvas,[style,...],[layer,...]]`), reusing the snapshot facet's own entity encoders,
  and a `node-path`/`enc_node_path`/`dec_node_path` production+function pair for the `NodePath`
  address type every per-node mutation variant carries.
- [x] Promoted `base()`/`all_variants()` (pre-existing test-local helpers) into module-scope
  `#[cfg(test)] fn fixture()` + `pub(crate) fn demo_mutation_cases()`, matching workflow's/brep's own
  convention exactly; the existing test module's own `base()`/`all_variants()` now delegate to them.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests`, in a new nested `mod
conformance_laws` — same home workflow's/brep's own reports identify as correct (drawing has no
per-standard `⚙️engine/` dir; the shared 14-subset `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs`
aggregator has no test module of its own, and is out of this wave's `✳️drawing/`-only edit scope).

### Not done (explicit, per brief item 9 / recipe's own instruction)

`LanguageSpec`/`register_schema_spec` registration — skipped, same reasoning as every prior semio
wave: no derivable `RecordSpec` exists for drawing's hand-rolled tagged-enum types (`PathSegment`/
`DrawNode`), and no clear per-subset registration site was found beyond `🎹️composer::register()`
itself. Filed as a follow-up rather than guessed at.

### JSON-transfer ban (checklist item 8)

```
$ grep -n "serde_json" 📸️snapshot/🦀️component.rs 🔺️diff/🦀️component.rs 🧬️mutations/🦀️component.rs
🧬️mutations/🦀️component.rs:232:/// `OpText`/`OpBinary`, replacing the old whole-enum `serde_json` passthrough (a real
🧬️mutations/🦀️component.rs:412:/// ⚡️ Real binary op frame, replacing the old whole-enum `serde_json::to_vec` shortcut. `format u8`
🔺️diff/🦀️component.rs:764:/// imported above) -- drawing wave: replaces the old hex-of-`serde_json` shortcut these leaf values
📸️snapshot/🦀️component.rs:4://! no `serde_json::Value`, no bare tuples/nested fixed arrays (geometry fields reuse
📸️snapshot/🦀️component.rs:151:/// hand-rolled `ArtifactDsl` below — replaces the old hex-of-`serde_json` passthrough.
📸️snapshot/🦀️component.rs:415:/// `serde_json::to_vec`-in-envelope shortcut.
📸️snapshot/🦀️component.rs:715:/// 🎁 Real structured text/binary codecs (drawing wave — off the old hex-dump-of-`serde_json`
```
All 7 hits are doc-comment prose describing the OLD, now-replaced shortcuts — zero `serde_json::`
calls remain inside any `ArtifactPack`/`OpBinary`/`DiffCodec` impl body, confirmed by direct grep.

---

## 3. Exact files touched

All paths relative to repo root, base
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/`.

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

**Tests**: `…/✳️drawing/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`; the fixture-generating temp test was added then removed in the same
session — confirmed absent in the final file).

**New example slug** (outside `✳️drawing/`, explicitly permitted by the brief, mirroring
workflow's/brep's/mesh's/model's/object's own precedent — note two OTHER pre-existing example slugs,
`📚️examples/📐️drawing` and `📚️examples/🖼️swatch`, belong to the DIFFERENT `cad`/`image` subsets
respectively and were confirmed untouched):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖍️sketch/🦀️component.rs`,
`…/🖍️sketch/🟦️component.ts`, `…/🖍️sketch/🖼️assets/🗣️example.dsl.semio` (real, byte-verified),
`…/🖍️sketch/🖼️assets/🎒️example.pack.semio` (real, byte-verified).

Nothing outside these was touched — confirmed via `git status --porcelain` scoped to
`✳️drawing/`/`📚️examples/🖍️sketch/` (exactly the 26 files above) and to the rest of
`🧿️semio/`/`🗄️stdio/` (all other `M`/`??` entries pre-date this session — other concurrent agents'
in-progress subsets, e.g. `✳️animation`, `✳️cad`, `✳️document`, `✳️presentation`, per this repo's own
heavy-concurrency ground rules). `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry`/`⚙️engine/🧰️triples` modules, and every
other subset were left untouched.

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `canvas`/`styles`/`layers` — `layers` is a homogeneous variable-length repeated record whose own `root: DrawNode` field is a further RECURSIVE tagged union. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `canvas`/`styles`/`layers` — 3 independently-optional segments; used one opaque `chain payload bytes` with a real 3-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled tagged-enum types). |
| `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` | no (brep's own, confirmed re-hit) | `PathSegment`/`DrawNode` are data-carrying tagged enums with heterogeneous per-variant field sets, `DrawNode` additionally recursive — no derive-path route to a matching TEXT grammar production set. Hand-rolled per brep's own established convention; `enc_node`/`dec_node` is the first genuinely RECURSIVE instance of this pattern in the semio wave series (brep's `enc_curve`/`enc_surface` were flat, non-recursive tagged enums). |
| **`store-bytereader-no-f32-reader`** (NEW — not in recipe's table) | no | `store::ByteReader` (the shared binary-pack cursor every semio wave's `ArtifactPack` reuses) has typed readers for `u8`/`u16`/`u32`/`u64`/`f64`(LE)/varint, but no `f32` reader — `SemioRgba`'s 4 `f32` fields needed a local `read_f32_le` (`read_bytes(4)` + `f32::from_le_bytes`), matching mesh's own wave's independent fix for the same gap (mesh's PBR color/material fields are also `f32`). **Recommend**: any future semio subset with `f32`-valued fields (color/material-heavy subsets, matching mesh's and drawing's own shape) should expect this and copy the `read_f32_le` pattern directly rather than rediscover it; a shared `ByteReader::read_f32_le` method would be a legitimate, small, centrally-scoped fix for whichever future ticket is allowed to touch `store`. |

---

## 5. Verification — real, not claimed

All commands below were run directly, synchronously, in the foreground in this session, and their
real output was read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** →
   ```
   Finished `dev` profile [unoptimized] target(s) in 52.94s
   ```
   **0 errors** (485 pre-existing warnings, none attributable to drawing's own files). Confirmed via
   a second, `grep -E "^error"`-only run — zero matches.

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::drawing"`**
   → **28 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests individually
   confirmed `ok`: `committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`. Two real, in-session bugs hit and fixed along the way (first full run:
   26/28, `diff_grammar_conformance_law` and `fixture_honesty_law` failing):
   - `diff_grammar_conformance_law` failed on a genuinely missing `option-bool` production in the
     diff `.grammar.semio` file (§2's diff section, §4's own note) — fixed, re-run green.
   - `fixture_honesty_law` failed on the placeholder `PLACEHOLDER_WILL_BE_REGENERATED_FROM_REAL_
     print_dsl_OUTPUT` fixture text, exactly as expected before fixture generation — fixed by the
     temp-test-then-delete method (§2's snapshot section), re-run green.

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) → **1894 passed, 2 failed, 4
   ignored** (both re-runs of the whole suite, ~30s apart, gave the identical 1894/2 result). The 2
   failures are both `artifacts::semio::standards::v1::subsets::document::composer::tests::
   conformance_laws::{fixture_honesty_law,grammar_conformance_law}` — **not this wave's code**:
   `document`'s own shipped fixture still contains the literal `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"`
   string, and `git status --porcelain` confirms `…/🪆️subsets/✳️document/…` is `M`-modified
   (extensively, ~24 files) by a different, concurrent session mid-way through its OWN real-codec
   wave on the `document` subset — the same "concurrent cargo workspace churn" pattern this ticket's
   own environment notes and every prior semio wave's report (workflow/brep/mesh/model/object) has
   independently hit and correctly attributed. One transient, unrelated compile error (`E0658`,
   somewhere in a concurrently-edited file, most likely `✳️cad` per its own `M` status at the time)
   was also observed once mid-session and self-resolved on the next run without any action from this
   session — not chased, per this ticket's own concurrent-development ground rules.
   **Zero failures attributable to anything in `artifacts::…::drawing`.**

**Status: this wave is genuinely proven, fully green for `✳️drawing`'s own scope**, with two honest,
explicitly-flagged, pre-existing, out-of-scope failures in a sibling subset (`document`) this session
did not touch and is not responsible for.

---

## 6. Notes for the next semio-subset wave

1. **A recursive tagged enum (`DrawNode`) is a real, distinct step up from brep's flat tagged enums**
   (`BrepCurve`/`BrepSurface`) — the text/binary primitives (`enc_node`/`dec_node`,
   `write_node`/`read_node`) simply call themselves for the `Group.children`/`Group` variant, no new
   mechanism needed beyond what brep already established; the diff facet's own `DrawNodeDiff`/
   `apply_node_diff`/`between_node`/`inverse_node_diff`/`absorb_node_diff` were ALREADY real and
   recursive pre-wave (this wave only had to fix their LEAF value encoding, not their recursive
   structure) — a future subset with an analogous recursive scene-graph/tree shape (model's spatial
   hierarchy, if it ever becomes genuinely tree-shaped rather than flat-with-`parent_id`) should
   expect this exact template to transfer directly.
2. **Check whether a sibling facet's LEAF values, not just its top-level structure, are still on
   `serde_json`** — drawing's diff facet's STRUCTURE (triples, tri-states, recursive node tags) was
   already real hand-rolled text pre-wave, which could look "already real, skip it" at a glance; the
   actual violation was one level deeper, in the leaf-value encoders (`enc_json`/`dec_json`). Always
   grep for `serde_json` inside the WHOLE file, not just the top-level `print_diff`/`parse_diff`
   dispatcher.
3. **`store::ByteReader` has no `f32` reader** — copy the `read_f32_le` pattern (§4) directly for any
   subset with `f32`-valued fields (color/PBR-heavy subsets); don't waste time searching for a
   built-in method that doesn't exist.
4. **Single-sourcing value-codec primitives in the snapshot facet (not per-facet duplication) is
   viable and preferable when the snapshot facet has zero dependents among its own sibling types** —
   drawing's `diff`/`mutations` both already depend on `schema::snapshot` for the plain types, so
   importing the CODEC functions from there too costs nothing extra and avoids a third independent
   copy; only duplicate per-facet (brep's own convention) when the snapshot facet would otherwise
   need a reverse dependency on a sibling facet's own types.
