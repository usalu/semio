# P2-FG1 — `stdio.dxf` (R12) — Real Grammar/Protocol Wave Report

Artifact: `🖊️dxf`, standard `🔖️r12`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/`.

## Summary

Followed `📖️grammar-recipe.md` literally. All three facets (snapshot, mutations, diff) now have
real, dialect-parseable `.grammar.semio`/`.protocol.semio` files replacing the old ABNF-dialect
placeholders (which used the wrong header syntax entirely — `dialect grammar stdio.dxf.snapshot`
on one line, `;` comments, `/` alternation, `%x` char-class ranges, none of which this repo's real
`parse_grammar`/`parse_protocol` understand). Both hand-rolled binary codecs (`OpBinary` for
`DxfMutation`, `DiffCodec` for `DxfDiff`) were upgraded from F6's `print_op()/print_diff()
.into_bytes()` text-as-binary shortcut to real recursive, varint/length-prefixed binary frames.
Both fixtures (`🗣️example.dsl.semio`, `🎒️example.pack.semio`) are now genuine `print_dsl`/
`encode_pack` output of a real, 2-level-nested demo snapshot. All 6 conformance-law tests are
implemented and green. 5-role `LanguageSpec` registration is complete (Document/Ops/Diff/Pack/Spr).

## 1. Snapshot facet (text-native)

**Grammar** (`📸️snapshot/📝️text/📖️component.grammar.semio`): models the REAL group-code/value
line-pair tokenizer 1:1 with `tokenize_dxf` (⚙️`📸️snapshot/🦀️component.rs`:322-338) —
`tag = INT LINE`, `document = artifact-mark tag*`. `LINE` (P2-M1's raw-span terminal) is the exact
primitive the framework's own grammar component doc comment names by artifact — its
`RawSpanEnd::Newline` variant's doc comment literally says *"dxf's opaque group-code value
lines"* (`🗣️dsl/📖️grammar/🦀️component.rs:2007`), confirming this was the intended primitive before
I even wrote the file. Because `LINE` captures the value line as a raw byte span of the ORIGINAL
source text (never re-tokenized), the W0 recon's item (2) concern (`$ACADVER`-style names need `$`
token support) turns out to be moot by construction — a `$`-prefixed name on a value line is just
part of the raw span, no `DOLLAR` terminal needed. `comment none` is declared since a real value
line (e.g. a `TEXT` entity's string) may legally contain a literal `#`. Per-code semantic typing
(W0 item 3, string/int/double/point chosen by an earlier code number) stays exactly what the
mission brief calls it: a Rust-side post-parse concern (`classify_group_code_value`), never
modeled at the grammar layer — section/table/block/entity nesting is likewise a flat-token-stream
Rust-side walk (`parse_header_section`/`parse_tables_section`/`parse_blocks_section`/
`parse_entities_until`), not a grammar-level concern, matching txt/obj/stl's own precedent for
record-oriented formats.

**Protocol** (`📸️snapshot/💾️binary/📡️component.protocol.semio`): text-native pack-container shape
— `framing record` + `chain payload utf8`, the same shape `stdio.json`'s own snapshot protocol
uses. `DxfSnapshot::encode_pack_with` was ALREADY just `print_dxf_document(self).into_bytes()`
wrapped by `wrap_binary` — no upgrade needed here, only the honest description.

## 2. Mutations facet — grammar written, `OpBinary` upgraded from the F6 shortcut

**Grammar** (`🧬️mutations/📝️text/📖️component.grammar.semio`): the real `keyword arg=value ...`
op-text shape `print_dxf_mutation`/`parse_dxf_mutation` already emit (confirmed traced from the
real function, not guessed). Every nested item literal (`header-var`/`layer-item`/`style-item`/
`linetype-item`/`dxf-entity`/`block-item`/`dxf-snapshot`) mirrors `🔺️diff/🦀️component.rs`'s real
`enc_*`/`dec_*` text codecs field-for-field, in the same positional order those functions emit.
`hex` is the framework's built-in macro (pitfall #2), never a hand-rolled `{INT|IDENT}*`
production. `dxf-entity`'s 8-arm shape is bounded, not self-recursive (R12 has no entity-in-entity
nesting) — a `block-item`'s own `entity-list` references `dxf-entity` one level down, giving the
whole grammar a fixed maximum depth.

**`OpBinary` upgrade** (`🧬️mutations/🦀️component.rs`): W0 recon confirmed `DxfMutation::OpBinary`
was still on `print_op().into_bytes()`. Upgraded to a real `format u8 | tag u8 | variant payload`
frame — `tag` is the variant ordinal (0-18, same order `parse_dxf_mutation`'s keyword match uses).
Every variant's payload is real recursive binary, built from a new `#region 🔖️ItemBinaryCodecs`
in `🔺️diff/🦀️component.rs` (mirrors every existing text `enc_*`/`dec_*` item codec 1:1, field
order verified against each): `enc_dxf_value_bin`, `enc_group_code(s)_bin`, `enc_vertex(_es)_bin`,
`enc_dxf_entity(_ies)_bin`, `enc_header_var_bin`, `enc_layer_bin`, `enc_style_bin`,
`enc_linetype_bin`, `enc_block_bin`, `enc_dxf_tag_bin`, `enc_other_table_bin`,
`enc_dxf_tables_bin`, `enc_dxf_snapshot_bin` (needed for `SetSnapshot`'s whole-snapshot payload) —
plus their `dec_*_bin` twins. `SetSnapshot`/`InsertEntity`/`SetEntity`/`InsertBlock`/`SetBlock` all
now genuinely round-trip real recursive binary, not text bytes. `store::write_varint_u64`/
`store::write_varint_i64`/`store::ByteReader` reused (`store`/`dsl`/`protocol` are all `extern
crate self as …` aliases for the SAME kernel crate — confirmed by reading
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`).

**Protocol** (`🧬️mutations/💾️binary/📡️component.protocol.semio`): `header fixed 2 { field format
u8; field tag u8 }` + `chain payload bytes` — the opaque-tail pattern (§2.5), unchanged shape from
the old placeholder but now honestly backed by a real binary codec on the Rust side.

**Demo cases**: `demo_mutation_cases()` (new `#region 🔖️DemoCases`, `#[cfg(test)] pub(crate) fn`,
same placement convention as `stdio.json`'s own) — one instance per variant (19 total) plus 4 extra
`InsertEntity` cases exercising Polyline/Other/Solid/Insert (the base fixture only reached
Line/Circle/Arc/Text before). The existing `mod tests`'s `variants()` fixture now just calls
`demo_mutation_cases()` (dedup, per CLAUDE.md's "extend, don't duplicate" rule) — every existing
property test (`mutation_diff_law`/`inverse_law`/`absorb_law`/`op_text_binary_roundtrip_law`)
still passes against the expanded fixture.

## 3. Diff facet — grammar written, `DiffCodec` upgraded from the F6 shortcut

**Grammar** (`🔺️diff/📝️text/📖️component.grammar.semio`): the real `key=[removed];[modified];
[added]`-shaped space-joined token line `print_dxf_diff`/`parse_dxf_diff` emit. Every collection
triple (name-keyed: `header-vars`/`layers`/`styles`/`linetypes`; index-keyed: `blocks`/`entities`,
the latter reused verbatim at two tree depths for a block's own nested `entities`) mirrors §1.4's
copy-pasteable shape, verified against `enc_name_triple`/`enc_index_triple`'s real output. Every
`Option<T>` diff field uses the `[0]`/`[1,<value>]` tri-state pair (§1.4's worked pattern).
`entity-diff`'s `Replace` arm carries a whole `dxf-entity` for the "Replace on kind change" rule.
`document = diff-token*` naturally matches the empty-diff line too (`Star` matches zero
occurrences, zero lexer tokens for `""`).

**`DiffCodec` upgrade** (`🔺️diff/🦀️component.rs`): W0 recon (and the P2-W0 census: "100% of
stdio's `DiffCodec` impls were still on the text-as-binary shortcut") confirmed `DxfDiff` was on
`print_diff().into_bytes()`. Upgraded to `format u8 | flags u8 | per-present-field payload`.
`DxfDiff` has FOUR independently optional top-level fields (unlike `stdio.json`'s single `value`
field with its one `has_value` byte), so `flags` is a 4-bit presence mask
(bit0=header_vars,bit1=tables,bit2=blocks,bit3=entities). New `#region 🔖️DiffBinaryCodecs` +
`#region 🔖️CollectionTripleBinaryCodecs` provide the real recursive binary twins of every
`enc_*_diff`/`enc_name_triple`/`enc_index_triple` text codec (varint-counted lists replacing the
`;`-separated bracket sections, same removed/modified/added shape).

**Protocol** (`🔺️diff/💾️binary/📡️component.protocol.semio`): `header fixed 2 { field format u8;
field flags u8 }` + `chain payload bytes` — documents the 4-bit mask explicitly, distinct from
`stdio.json`'s single-flag precedent.

**Demo cases**: `demo_diff_cases()` (new `#region 🔖️DemoCases`) — the empty diff, a
single-collection sparse diff (entities-only, exercising the "only one of the four top-level
tokens present" case the flags-mask logic needs), and the rich multi-collection diff (refactored
out of the pre-existing `diff_codec_text_binary_roundtrip_law` test, which now just iterates
`demo_diff_cases()` instead of constructing its own literal — same dedup rule as mutations).

## 4. Engine — 5-role registration + conformance laws (`⚙️engine/🦀️component.rs`)

- `demo_dxf_snapshot()`: a real, 2-level-nested (HEADER incl. a point-component var, all 3 typed
  table kinds, one raw-retained unmodeled table, a BLOCK with a nested entity, every typed entity
  kind plus one raw-retained unmodeled kind) demo snapshot — the single source of truth for
  `fixture_honesty_law` and `grammar_conformance_law`/`protocol_walk_law`.
- `register_pilot_languages()` previously registered ONLY the Document role. Added `stdio.dxf.op`
  (Ops), `stdio.dxf.diff` (Diff, `protocol: None` per the 5-role scheme's own convention —
  `stdio.json.diff` leaves it `None` too even though a real diff protocol file exists),
  `stdio.dxf.pack` (Pack, `grammar: None` + snapshot's protocol — matches `stdio.json.pack`'s own
  shape), `stdio.dxf.spr` (Spr, `grammar: None` + mutations' protocol — matches `stdio.json.spr`).
- `register_schema_spec`: **not called**, deliberately — `DxfEntity`/`DxfValue` are data-carrying
  enums with no `DslField` impl (confirmed by this artifact's own pre-existing doc comments citing
  real `cargo check` rejections of `#[derive(dsl::DslArtifact)]`/`#[derive(dsl::DslDiff)]`/
  `#[derive(dsl::DslOps)]`), same root cause that blocks `stdio.json`/`stdio.csv`/`stdio.zip`/
  `stdio.png`. Filed below as a `mechanism_gaps` entry rather than fabricating an unrelated spec.
- `conformance_laws` module (nested in `mod tests`, mirrors `stdio.json`'s exact structure): all 6
  laws — `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
  `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — all green.

## 5. Fixtures

`🗣️example.dsl.semio` (630 bytes) and `🎒️example.pack.semio` (636 bytes, new file) are genuine
`print_dsl`/`encode_pack` output of `demo_dxf_snapshot()`, generated via a temporary `#[ignore]`
test (`debug_generate_fixtures`, per the recipe's own prescribed method) that called the real
Rust encoders directly and wrote the bytes to disk — run once, verified, then deleted before
finishing (not present in the final diff).

## 6. Verification

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::dxf"
```
→ **25 passed, 0 failed, 0 ignored** (every dxf-scoped test, including all 6 new conformance laws,
the upgraded `op_text_binary_roundtrip_law`/`diff_codec_text_binary_roundtrip_law`, and every
pre-existing property test).

```
cargo test -p semio-s-plugin-stdio --lib
```
→ ran three times over the course of this wave as a concurrent session actively edited
`artifacts::md::…`/`artifacts::xml::…` (confirmed via `git status` showing both trees `M`odified
by another session, not touched by me, per the repo's own concurrent-churn note): first pass 1711
passed/4 failed (md+xml)/3 ignored; a mid-edit `md` engine file briefly failed to even COMPILE
(`error: expected ... found "#"` — a raw-string literal mismatch, i.e. someone else's file was
saved mid-keystroke); retried once and it compiled again; final pass (reported here) —
**1713 passed, 2 failed, 1 ignored**, both remaining failures still in
`artifacts::md::standards::v_commonmark::engine::tests::conformance_laws::{grammar_conformance_law,zz_debug_bisect}`
(xml's own 2 failures resolved between runs — consistent with that session's own in-progress fix
landing). Classified as unrelated by file path throughout, never chased. The 1 ignored test
(`csv::…zzz_generate_p2p1_fixtures`) is pre-existing, unrelated `#[ignore]`d scaffolding in another
artifact's own file — not dxf's (my own temporary generator test was deleted before this final run).

JSON-transfer elimination: `grep -rn "serde_json::to_vec\|serde_json::from_slice\|
serde_json::to_string\|serde_json::from_str\|serde_json::Value"` over the whole `🖊️dxf/` tree
returns nothing — was already clean (not on W0's violator list), stays clean.

## 7. Mechanism gaps hit (all already-known, per the recipe's own table — applied the documented
workaround, not chased as a mystery)

- `protocol-prim-ref-recursion` / `protocol-array-of-records`: `DxfMutation`'s and `DxfDiff`'s
  recursive/nested-collection payloads (`DxfEntity`, `DxfBlock` containing `Vec<DxfEntity>`,
  collection-triple modified/added lists) can't be described field-by-field in a `.protocol.semio`
  file — modeled as a real fixed `format`/`tag`(or `flags`) header plus one opaque trailing `chain
  payload bytes`, genuinely structured and round-trip-tested at the Rust layer instead. Exactly
  the pattern every other pilot with a recursive mutation/diff type used.
- `register-schema-spec-needs-recordspec`: `DxfEntity`/`DxfValue` are hand-rolled data-carrying
  enums with no derivable `RecordSpec` — `register_schema_spec` is not called for either the
  snapshot or diff schema id, same situation json/csv/zip/png are already in. Non-blocking.

Neither gap required a workaround beyond what the recipe already documents; no new gap was
discovered by this wave.

## 8. Deviations from a literal reading of the recipe

- The recipe's collection-triple example wraps the triple in `"<collection>" "{" ... "}"` (§1.4).
  DXF's REAL wire text (traced from `print_dxf_diff`/`enc_name_triple`/`enc_index_triple`) has no
  outer `{}` — it's `key=[removed];[modified];[added]` directly. The grammar models the REAL shape
  (no `{}`), not the recipe's illustrative wrapper, per the recipe's own repeated instruction to
  trace the real function rather than copy an example verbatim when they diverge.
- `DxfDiff`'s binary frame uses a 4-bit presence `flags` byte instead of `stdio.json`'s single
  `has_value` byte, because `DxfDiff` genuinely has 4 independently-optional top-level fields where
  `JsonDiff` has 1 — documented inline in both the protocol file and the Rust doc comment.
- Two `demo_mutation_cases()` `InsertEntity` indices were set to `2` (not the more "distinct-index"
  values `3`/`4` I originally chose) to stay within `base_snapshot()`'s 2-entity bound — an
  out-of-range literal index desyncs the PRE-EXISTING `Mutation::inverse` `InsertEntity` arm (which
  reads the index literally off the mutation payload, not the actual clamped insert position);
  this is a latent property of code outside this wave's scope, not something introduced or fixed
  here, worked around by choosing valid demo indices instead.

## Files touched

- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (imports, `OpBinary` upgrade, `demo_mutation_cases()`, `variants()` dedup)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (new binary codec regions, `DiffCodec` upgrade, `demo_diff_cases()`, test dedup)
- `🏅️standards/🔖️r12/⚙️engine/🦀️component.rs` (`demo_dxf_snapshot()`, 5-role registration, `conformance_laws` module)
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (regenerated, genuine)
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, genuine)
