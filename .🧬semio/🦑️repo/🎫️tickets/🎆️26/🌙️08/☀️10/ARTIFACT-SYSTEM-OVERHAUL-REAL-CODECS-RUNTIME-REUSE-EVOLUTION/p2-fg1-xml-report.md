# FG1 — 📰xml (standard 1.0) — Grammar & Protocol Overhaul Report

Ticket: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`. Artifact: `📰xml`
standard `1.0`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/`.

## Summary

Rewrote all 6 grammar/protocol files for `stdio.xml` in the real dialect syntax (replacing the
pre-Phase-2 pseudo-ABNF placeholders), upgraded `DiffCodec`/`OpBinary` from F6's
`print_diff/print_op().into_bytes()` text-as-binary shortcut to real binary frames, fixed a genuine
`POLICY_STDIO_JSON_TRANSFER_BAN` violation in `XmlSnapshot::ArtifactPack` (was `serde_json::to_vec`/
`from_slice` of the whole `XmlDocument`, disguised as binary), added the 6 conformance-law tests,
generated real `.dsl.semio`/`.pack.semio` fixtures from the real Rust encoders, and completed
5-role `LanguageSpec` registration (previously only the Document role was registered). All 36 tests
in `artifacts::xml` pass; whole-crate run is 1713 passed / 2 failed, both failures entirely in
`md` (an unrelated, concurrently-in-progress sibling FG1 session's own scope, see §Verification).

## 1. Snapshot facet (`📸️snapshot`)

### Grammar (`📝️text/📖️component.grammar.semio`)

Real XML 1.0 grammar matching the artifact's own character-level parser
(`📸️snapshot/🦀️component.rs`'s `xml_document_from_text`/`parse_node`/`parse_attrs`/
`parse_attr_value`/`xml_unescape_text`/`skip_misc`/`parse_xml_declaration_prolog`), replacing the
old placeholder (wrong header dialect, `/` alternation, `~` negation, `%x` ranges — all outside
this dialect's real lexer alphabet).

Header: `comment none` (XML has no `#`-comment, and `&#65;`/`&#x1F600;`-style numeric character
references use `#` as a literal sigil right after `&` — the default `#`-to-EOL comment would
otherwise swallow the rest of the line at every such reference, the same STEP/DXF-shaped collision
M1's own report names) + `string double raw` + `string single raw` (both quote delimiters, `raw`
mode since `parse_attr_value` captures attribute values completely unescaped — entity decoding
only ever runs over element CONTENT text via `xml_unescape_text`, never over attribute values, a
genuine quirk of the real parser, not a grammar simplification).

Productions model: the XML declaration, a simple `<!DOCTYPE name>` form (the fuller PUBLIC/SYSTEM
external-ID + internal-subset shape stays real Rust-side, preserved byte-for-byte but never
re-parsed into structured fields — same "opaque unless the model round-trips it structurally"
treatment the recipe's own PDF/DWG carve-out uses), `empty-elem-tag`/`s-tag`/`e-tag`/`attribute`,
`:`-qualified names (namespace prefixes — `name = IDENT ":" IDENT | IDENT`, qualified alternative
FIRST, see §3 pitfall below), recursive `content` (element/reference/cdata/comment/pi/text-run),
entity + numeric character references, CDATA, comment, and processing instructions. `<`/`>`/`&`/`;`
use the P2-M1-promoted terminals `LT`/`GT`/`AMP`/`SEMICOLON`; every other XML punctuation (`/` `:`
`=` `[` `]` `!` `?` `#` `--`) is matched via `Symbol::Literal` (text-equality match, independent of
token kind — works even for the `Error`-kind tokens `!`/`?`/`#` fall through to once `comment none`
stops `#` being swallowed).

### Protocol (`💾️binary/📡️component.protocol.semio`)

Text-native shape (`framing record` + `chain payload utf8`), matching json's own precedent exactly
— the pack payload is the artifact's real wire text, opaque at the byte layer (the real internal
grammar is the text grammar's job).

### `ArtifactPack` fix (`📸️snapshot/🦀️component.rs`)

**Real bug fixed, not just described.** `encode_pack_with`/`decode_pack_with` were calling
`serde_json::to_vec(&self.doc)`/`serde_json::from_slice(&inner)` — a literal JSON payload disguised
as binary, exactly the `POLICY_STDIO_JSON_TRANSFER_BAN` violation flagged by name in the P2-W0
recon report's own `mechanism_gaps`-adjacent findings table (xml row, "Yes — in scope"). Replaced
with `xml_document_to_text(&self.doc).into_bytes()` / `xml_document_from_text(text)` — the same
real wire-text codec the DSL facet already uses, matching json's own `ArtifactPack` treatment of
its RFC8259 text. Regenerated `🎒️example.pack.semio` from this real path (see §4).

## 2. Diff facet (`🔺️diff`)

### Grammar (`📝️text/📖️component.grammar.semio`)

Real one-line `print_xml_diff`/`parse_xml_diff` shape (`declaration=... doctype=... root=...`,
each optional, fixed order), replacing the old serde-JSON-struct placeholder. `hex` used throughout
as the framework's built-in macro (never a hand-rolled `{INT|IDENT}*` production — the mandatory
pitfall-2 rule). `node-diff`/`xml-node` are genuinely recursive (children reference node-diff which
references children back) — the same self-recursion pattern M1 already proved safe.

### Protocol (`💾️binary/📡️component.protocol.semio`) — real binary upgrade

`XmlDiff::DiffCodec` was on F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of
stdio per the W0 census). Upgraded to a real binary frame: `header fixed 2` (`format u8` + `flags
u8`, flags bit0/1/2 = declaration/doctype/root presence — `XmlDiff` has THREE independently-
optional top-level fields, unlike json/csv/png's single-optional-field diffs, so this is a bitmask
not one bool) + `chain payload bytes`. Added real recursive binary primitives in
`🔺️diff/🦀️component.rs` (`enc_xml_node_bin`/`dec_xml_node_bin`, `enc_declaration_bin`/
`dec_declaration_bin`, `enc_node_diff_bin`/`dec_node_diff_bin`, `enc_attrs_diff_bin`/
`enc_children_diff_bin` and their `dec_` twins, plus `write_str_lp`/`read_str_lp`/`write_bytes_lp`/
`read_bytes_lp` LEB128-varint-framed primitives) backing genuinely structured `encode_diff`/
`decode_diff` — not a second text encoding.

The past-header payload (declaration/doctype/root, whichever `flags` marks present) is bundled into
ONE opaque trailing `bytes` chain rather than three separately-`Cond`-guarded fields: this dialect's
`Cond` can only test a field's WHOLE value against a literal (no bitwise-AND primitive), so a
per-bit-guarded field list isn't expressible even before `root`'s own recursion (`XmlNodeDiff`, a
data-carrying enum) is considered — `Prim::Ref` unconditionally errors in `walk_protocol`
(`protocol-prim-ref-recursion`, hit by every stdio pilot's own diff/op payload). Once ANY field is
structurally unwalkable, bundling the two simpler fields into the same opaque tail is the honest,
minimal-surface choice; the Rust side stays genuinely, fully structured and round-trip tested
independently (`diff_codec_text_binary_roundtrip_law`, `protocol_walk_law`).

`demo_diff_cases()` extracted as a `pub(crate)` `#[cfg(test)]` helper (was inline in the old test)
so it's the single source of truth for both the local round-trip test and the engine's
`diff_grammar_conformance_law`/`protocol_walk_law`.

## 3. Mutations facet (`🧬️mutations`)

### Grammar (`📝️text/📖️component.grammar.semio`)

Real one-line `print_xml_mutation`/`parse_xml_mutation` `keyword arg=value ...` shape for all 8
variants (`no-mutation`/`set-snapshot`/`set-declaration`/`set-doctype`/`insert-element`/
`remove-element`/`set-attribute`/`set-text`), replacing the old serde-JSON placeholder. `xml-node`
restated self-contained (same shape as the diff grammar's own, per the repo's per-facet-restatement
convention json/csv already use).

### Protocol (`💾️binary/📡️component.protocol.semio`) — real binary upgrade

`XmlMutation::OpBinary` was also on the F6 text-as-binary shortcut. Upgraded to `header fixed 2`
(`format u8` + `tag u8`, the variant ordinal 0-7 matching `print_xml_mutation`'s own keyword match
order) + `chain payload bytes`, same shape json's own upgraded mutations protocol uses. Added
`enc_node_path_bin`/`dec_node_path_bin`/`enc_xml_snapshot_bin`/`dec_xml_snapshot_bin` in
`🧬️mutations/🦀️component.rs`, reusing (rather than duplicating) `🔺️diff/🦀️component.rs`'s
`pub(crate)` `enc_xml_node_bin`/`dec_xml_node_bin`/`enc_declaration_bin`/`dec_declaration_bin`/
`write_str_lp`/`read_str_lp` — same intra-artifact reuse pattern the text codec already used for
`hex_encode`/`enc_xml_node`/`split_top_level`.

`demo_mutation_cases()` extracted as a `pub(crate)` `#[cfg(test)]` helper (was inline in the old
`op_text_binary_roundtrip_law` test), now the single source of truth for that test AND the engine's
`ops_grammar_conformance_law`/`protocol_walk_law`.

## 4. Fixtures

`🗣️example.dsl.semio` and `🎒️example.pack.semio` (new file — didn't exist before) regenerated from
the REAL Rust encoders via a temporary `#[ignore]`d test (`debug_print_demo_fixtures`, added, run
once via `cargo test ... -- --ignored --nocapture`, output copied byte-for-byte, then deleted
before finishing — never hand-derived). `demo_xml_snapshot()` (new helper in `⚙️engine/🦀️component.rs`)
deliberately exercises every construct the W0 census row names: an XML declaration, a `<!DOCTYPE
catalog>`, a `:`-qualified (`xmlns:c`) attribute name, entity decode (`Tom &amp; Jerry`), a
self-closing element (`<empty flag="true"/>`, carrying an attribute so its trailing `/` never fuses
with the preceding ident into one token — see the `name` production's own doc comment), CDATA,
comment, and a processing instruction. The old `🗣️example.dsl.semio` was a bare
`{"hello":"stdio.xml","n":1}` placeholder — replaced. `example.xml` (a separate, pre-existing
fixture used only by `codec_retention_law`/`between_roundtrip_law`, unrelated to the conformance
laws) was left untouched.

## 5. Conformance laws + registration

Added the `conformance_laws` test module (`⚙️engine/🦀️component.rs`, inside the existing `mod
tests`) with all 6 laws: `committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law` — copied structurally from json's own pilot template, demo-case helpers
swapped for xml's own.

`register_pilot_languages()` previously registered ONLY the `stdio.xml` Document role. Completed to
the full 5-role scheme: `stdio.xml.op` (Ops), `stdio.xml.diff` (Diff, `protocol: None` per the
5-role scheme's own shape), `stdio.xml.pack` (Pack), `stdio.xml.spr` (Spr), all
`dsl::passthrough_hooks`.

`register_schema_spec` deliberately NOT called — `XmlSnapshot`/`XmlDiff`/`XmlMutation` are all
hand-rolled (no `#[derive(dsl::DslRecord)]`/`DslArtifact`/`DslDiff`/`DslOps`, confirmed live by F6's
own recon: `XmlNode`/`XmlNodeDiff` are data-carrying recursive enums with no `DslField` impl) — no
`fn() -> RecordSpec` genuinely exists to register. Filed as `mechanism_gaps` below, same treatment
json/csv/zip/png already use.

## 6. A real bug found and fixed while drafting the grammar (pitfall 3)

The snapshot grammar's original draft named a production `comment` for the `<!--...-->` construct
— colliding with the reserved header-directive keyword `comment` (§3 pitfall 3 of the recipe doc:
production names can never collide with `extension`/`use`/`start`/`comment`/`string`, ANYWHERE in
the file, since `parse_grammar`'s unified header/production loop parses ANY leading ident matching
one of those five words as a header directive). Caught by `committed_facet_files_parse`/
`grammar_conformance_law` failing with a confusing `expected Ident, found Equals` pointed at the
UNRELATED real `comment none` header line — bisected with a temporary test that binary-searched
line-prefixes of the grammar file for the actual failure point, confirmed the real cause, renamed
the production to `xml-comment`, deleted the temporary bisection test. Documented inline in the
grammar file so a future author doesn't rediscover this the hard way.

## Mechanism gaps hit (all already known/consolidated, applied per the recipe's own worked pattern)

- `protocol-prim-ref-recursion` — `XmlNode`/`XmlNodeDiff` are genuinely recursive, data-carrying;
  `Prim::Ref` can't describe them. Modeled the real fixed header (`format`/`flags` or `format`/
  `tag`) precisely, opaque trailing `bytes` for the recursive payload. Same treatment every other
  pilot's diff/op payload gets.
- `register-schema-spec-needs-recordspec` — no derivable `RecordSpec` exists (hand-rolled types).
  Skipped the call rather than fabricate one.
- New (not yet in the consolidated table, filed here): `xml-cdata-comment-chardata-raw-span` —
  `charData`/`CDATA`/comment bodies can, per the real XML spec, contain arbitrary bytes up to a
  multi-char closing marker (`]]>`, `-->`) that neither the `LINE` (rest-of-physical-line) nor
  `REST` (rest-of-EOF) raw-span terminal can express (both are whole-remaining-span primitives, not
  "up to the next occurrence of a specific multi-char marker"). Modeled these bodies as a bounded
  token-repetition (`{IDENT | INT | FLOAT}*`) matching this artifact's own real demo/fixture content
  (word/number/space only) — the SAME honest-boundary treatment the consolidated
  `csv-quoted-field-embedded-newline` gap already documents (the artifact's own fixture deliberately
  avoids the unrepresentable input shape rather than silently papering over it). Non-blocking.

## Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::xml"` → **36 passed, 0 failed, 0 ignored**
(includes the 6 new conformance-law tests plus every pre-existing law from the F1/F6d waves —
`mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`, `codec_retention_law`,
`field_sweep_law`, the diff/mutations op-codec round-trip tests, `codec_round_trip`,
`empty_snapshot_matches_schema`, `nontrivial_nested_value_round_trip`-equivalent coverage).

`cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1713 passed, 2 failed, 1 ignored**.
Both failures are `artifacts::md::standards::v_commonmark::engine::tests::conformance_laws::
grammar_conformance_law` and `...::zz_debug_bisect` — entirely inside a DIFFERENT, concurrently
in-progress FG1 sibling session's own scope for `📝️md` (confirmed by `git status`/file mtimes: the
md files were mid-edit throughout this session, with a still-in-place `zz_debug_bisect` debug test
name strongly suggesting that session's own bisection-in-progress, analogous to what this report's
§6 describes for xml). Not touched, per the ticket's "classify, don't chase" rule — outside this
wave's ownership boundary.

Grep-confirmed zero `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` remaining
inside `📰xml`'s `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks (only a doc-comment mentioning the
now-removed prior usage).

`bun run ./📜️script.ts policy` not run this wave (informational-only per the recipe's own
verification section; the ticket's periodic policy-shrink pass reconciles the allowlists once this
and sibling FG1 waves land — not something this wave edits directly).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs` — `demo_xml_snapshot()`,
  5-role `register_pilot_languages`, `conformance_laws` test module.
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `ArtifactPack` JSON-transfer-ban fix.
- `.../📸️snapshot/📝️text/📖️component.grammar.semio` — real grammar (rewritten).
- `.../📸️snapshot/💾️binary/📡️component.protocol.semio` — real protocol description (rewritten).
- `.../🔺️diff/🦀️component.rs` — binary primitives + node/attrs/children-diff binary codecs,
  `DiffCodec::encode_diff`/`decode_diff` real upgrade, `demo_diff_cases()` extracted.
- `.../🔺️diff/📝️text/📖️component.grammar.semio` — real grammar (rewritten).
- `.../🔺️diff/💾️binary/📡️component.protocol.semio` — real protocol description (rewritten).
- `.../🧬️mutations/🦀️component.rs` — binary path/snapshot codecs, `OpBinary::encode_op`/`decode_op`
  real upgrade, `demo_mutation_cases()` extracted.
- `.../🧬️mutations/📝️text/📖️component.grammar.semio` — real grammar (rewritten).
- `.../🧬️mutations/💾️binary/📡️component.protocol.semio` — real protocol description (rewritten).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — real
  fixture (rewritten, replacing the fake JSON placeholder).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — real
  fixture (new file).
- This report: `p2-fg1-xml-report.md`.

No framework files, `📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol/registry modules,
`🧪️fixture-sweep`, or `🏪️store` were touched. No other artifact's files were touched.
