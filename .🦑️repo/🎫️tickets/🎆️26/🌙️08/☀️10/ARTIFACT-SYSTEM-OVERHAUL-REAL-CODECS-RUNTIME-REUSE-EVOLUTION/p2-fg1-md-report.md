# P2-FG1 — `stdio.md` (CommonMark) — real grammars, protocols, and binary codecs

## Scope

Artifact: `📝️md`, standard `🔖️commonmark`, path
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/`. Per the recipe's per-standard
checklist (`📖️grammar-recipe.md` §4): real grammar files, real protocol files, mutations/diff
grammar+protocol (incl. `OpBinary`/`DiffCodec` binary-frame upgrades), real fixtures, the 6
conformance-law tests, 5-role `LanguageSpec` registration, and a JSON-transfer sweep.

## What was wrong before this wave

Every one of the artifact's 6 `.grammar.semio`/`.protocol.semio` files was still the pre-Phase-2 ABNF
placeholder: wrong header dialect (`dialect grammar stdio.md.snapshot` on one line), `;` line
comments, `/` alternation, `%x`/`0*3SP` ABNF quantifiers — none of them valid in this repo's real
`dsl::parse_grammar`/`dsl::parse_protocol` dialect, and none of them matching what the real Rust
codecs actually emit (the mutations/diff grammar files described a serde-JSON wire struct the F6
hand-rolled `OpText`/`DiffCodec` codecs never produce; the snapshot files re-described the SEMIO
envelope's own framing a second time instead of treating it as already-described framework-side).
`MdMutation::OpBinary`/`MdDiff::DiffCodec` were both still on F6's `print_op()/print_diff().into_bytes()`
text-as-binary shortcut. The demo fixture `🗣️example.dsl.semio` was a bare fake
(`{"hello":"stdio.md","n":1}`); `🎒️example.pack.semio` didn't exist at all.

## What this wave did

### Grammar files (real dialect syntax, all 3 facets)

- **Snapshot** (`📸️snapshot/📝️text/📖️component.grammar.semio`): a genuine, honest-subset CommonMark
  block grammar — `comment none` (mandatory: `#` is this dialect's default line-comment marker and
  would otherwise swallow every ATX heading), marker/keyword-driven classifiers for
  `thematic-break` (`"--" "-"`, since the lexer fuses `--` into one `DashArrow` token before the
  third `-`), `fenced-code-block` (the dialect's own built-in `FENCE` raw-multi-line-span token —
  matches CommonMark's real fence rule almost exactly), `block-quote` (genuinely recursive,
  `block-quote = {GT block}+`, proven end-to-end by `grammar_conformance_law` against a real
  2-level-nested `BlockQuote{BlockQuote{Paragraph}}` fixture), `atx-heading` (`"#"+ LINE`), and
  `paragraph` (`LINE`, deliberately ONE line only — no `NEWLINE` terminal exists to bound a `LINE+`,
  which would otherwise greedily swallow the rest of the document). `MdBlock::List` is **not**
  modeled at all — the ticket's own explicitly-excluded, architecturally-impossible
  leading-whitespace-count mechanism gap (no token exists for counting indentation in a
  whitespace-as-trivia lexer). `MdBlock::HtmlBlock` is also left unmodeled (not one of the 5 kinds
  the ticket names as in-scope; its real end condition is the same un-boundable "how many `LINE`s"
  problem as multi-line paragraphs).
- **Mutations** (`🧬️mutations/📝️text/📖️component.grammar.semio`): the real `keyword arg=value ...`
  op-text shape `print_md_mutation`/`parse_md_mutation` already emit (F6), with genuinely, mutually
  recursive `md-block`/`md-inline` productions (tag-prefixed `A`-`I`/`J`-`P`, mirroring
  `enc_block`/`enc_inline` 1:1) — `List`/`BlockQuote` reference `item-list`/`block-list` which
  reference `md-block` back; `Emphasis`/`Strong`/`Link` reference `inline-list` which references
  `md-inline` back. `hex` used as the framework's built-in macro (never a hand-rolled
  `{INT|IDENT}*` production — recipe §3 pitfall #2).
- **Diff** (`🔺️diff/📝️text/📖️component.grammar.semio`): the real `blocks=<triple>` line
  `print_md_diff`/`parse_md_diff` emit, genuinely recursive collection-triple productions
  (`blocks-diff-body`/`list-items-diff-body`, recipe §1.4's shape) over the `Q`-`X` tag range for
  `MdBlockDiff` (incl. the `Replace` kind-change fallback), both real tri-states
  (`List.start: Option<Option<u32>>`, `CodeBlock.info: Option<Option<String>>`) modeled as nested
  `opt-*` wrappers per the recipe's `[0]`/`[1,[0]]`/`[1,[1,x]]` pattern.

All 3 grammar files respect the recipe's §3 pitfalls: `{...}` grouping only, `hex` macro not a
hand-rolled production, no production named `string`/`extension`/`use`/`start`/`comment`, and —
caught live during this wave, same trap zip/png hit — **every production kept to one physical
line** (the multi-line `md-inline =`/`md-block =`/`block-diff =` alternations I originally wrote
across several lines for readability failed `parse_grammar` with `expected Ident, found Pipe`;
collapsed to one line each, per pitfall #4).

### Protocol files (real dialect syntax, all 3 facets)

- **Snapshot**: text-native (`framing record` + `chain payload utf8`), matching json's own
  text-native precedent exactly — the pack container is the SEMIO envelope wrapping
  `render_markdown_blocks`'s UTF-8 bytes verbatim, no re-description of the framework-level
  envelope itself.
- **Mutations**/**Diff**: both upgraded to real binary frames (`header fixed 2` + `field format u8`
  + `field tag u8`/`field has_value u8` + `chain payload bytes`), matching json's own upgraded
  precedent — `Prim::Ref` can't describe `MdBlock`'s self-recursion at the protocol-dialect level
  (`protocol-prim-ref-recursion`, confirmed unchanged), so the recursive payload is one opaque
  trailing `bytes` chain, genuinely structured (and round-trip tested) on the Rust side.

### Rust binary codec upgrades

- `MdMutation::OpBinary` (`🧬️mutations/🦀️component.rs`): real `format u8 | tag u8 | variant payload`
  frame, upgraded from F6's `print_op().into_bytes()` shortcut. Tag 0-5 matches
  `print_md_mutation`'s own keyword order.
- `MdDiff::DiffCodec` (`🔺️diff/🦀️component.rs`): real `format u8 | has_value u8 | blocks-diff
  payload` frame, upgraded from F6's `print_diff().into_bytes()` shortcut.
- Both reuse a new set of `pub(crate)` recursive binary primitives added to `🔺️diff/🦀️component.rs`
  (mirroring where the TEXT primitives already live, so mutations imports them — same
  intra-artifact-reuse split the text codec already used): `write_str_bin`/`read_str_bin`,
  `write_bool_bin`/`read_bool_bin`, `write_option_bin`/`read_option_bin`,
  `write_tristate_bin`/`read_tristate_bin`, `enc_inline_bin`/`dec_inline_bin`(+list),
  `enc_block_bin`/`dec_block_bin`(+list/item-list), `enc_block_diff_bin`/`dec_block_diff_bin`,
  `enc_blocks_diff_bin`/`dec_blocks_diff_bin`, `enc_list_items_diff_bin`/`dec_list_items_diff_bin`.
  `MdMutation`'s own `enc_snapshot_bin`/`dec_snapshot_bin` and
  `enc_path_bin`/`dec_path_bin`/`enc_path_step_bin`/`dec_path_step_bin` stay mutation-specific.
- `demo_mutation_cases()` (mutations) and `demo_diff_cases()` (diff) extracted from what were
  previously inline test-local vectors into `#[cfg(test)] pub(crate) fn`s — single source of truth
  reused by both the existing local round-trip tests and the new engine-level conformance laws.

### Fixtures

`demo_md_snapshot()` added to `⚙️engine/🦀️component.rs` — genuinely exercises `Heading`,
`Paragraph` (with `Strong`/`Emphasis`/`Code` inline content), a real 2-level-nested `BlockQuote`
(proving the snapshot grammar's `block-quote` self-recursion end-to-end), a fenced `CodeBlock` with
a real info string, and `ThematicBreak`. `🗣️example.dsl.semio`/`🎒️example.pack.semio` are its
literal `print_dsl`/`encode_pack` output — generated via a temporary `#[ignore]`d test that called
the real encoders directly, copied the printed bytes, then deleted (per the recipe's own
instruction); never hand-derived.

### The 6 conformance-law tests

Added `mod conformance_laws` to `⚙️engine/🦀️component.rs`'s existing `#[cfg(test)] mod tests`:
`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — same shape every pilot
uses. All 6 pass.

### 5-role `LanguageSpec` registration

`register_pilot_languages()` expanded from the pre-existing single Document-role registration to
the full 5-role set: `stdio.md` (Document), `stdio.md.op` (Ops), `stdio.md.diff` (Diff, `protocol:
None` matching the scheme's own no-6th-role shape), `stdio.md.pack` (Pack), `stdio.md.spr` (Spr) —
all `dsl::passthrough_hooks`. `register_schema_spec` deliberately NOT called: `MdSnapshot`/
`MdDiff`/`MdMutation` derive `schema::ArtifactSchema` but not `dsl::DslArtifact`/`DslRecord`/
`DslDiff`/`DslOps` (recursive data-carrying enums, no `DslField` impl exists or can exist) — no
derivable `RecordSpec`, same situation json/csv/zip/png hit; filed below instead of fabricated.

### JSON-transfer sweep

`grep -rn "serde_json::to_vec\|from_slice\|to_string\|from_str\|Value"` across the whole artifact:
zero hits. Already clean.

## A genuine framework bug found and routed around (not fixed — out of ownership boundary)

While building `demo_md_snapshot()`, `grammar_conformance_law` failed non-deterministically
depending on block ORDER even though every individual block kind recognized correctly in isolation.
Root-caused via direct reproduction (bisected down to a ~15-byte minimal repro, confirmed against
the real lexer's token stream via `dsl::lex_with`): `🔍️lexer/🦀️component.rs`'s `FENCE`-token
scanning loop increments its running `byte_offset` for every consumed character **except** `'\n'`
while walking a fenced block's content — so every token positioned AFTER a fenced block whose
content spans N lines gets its own `byte_range` under-reported by exactly N bytes. This desyncs
`match_raw_span`'s `text.get(start_byte..).find('\n')` lookup (`📖️grammar/🦀️component.rs`) enough
that a `LINE`/`REST` match immediately following the fence can come back zero-width instead of
consuming the token it should — concretely, a `FENCE` followed by `block-quote`'s own nested
`paragraph = LINE` reproducibly failed this way; `atx-heading`/`paragraph`/`thematic-break` placed
directly after a fence were empirically NOT observed to trip it for the layouts tested, but the
underlying corruption is real and general, not something worth depending on for luck.

This is a genuine lexer bug, not a dialect-design mechanism gap and not this artifact's file to fix
(`🔍️lexer/🦀️component.rs` is a framework SDK file, explicitly out of this wave's ownership
boundary). Worked around locally: `demo_md_snapshot()` places `CodeBlock` LAST, followed only by
`ThematicBreak` (a pure-literal-token production that never calls `LINE`/`REST`, structurally
immune); `BlockQuote`/`Paragraph` (both `LINE`-dependent) are placed BEFORE the fence. Documented in
both the snapshot grammar file's own header comment and `demo_md_snapshot()`'s doc comment, and
filed below as `mechanism_gaps` id `md-fence-byte-offset-corruption`.

## Verification

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::md"
```
→ 36 passed, 0 failed, 0 ignored (incl. all 6 new conformance-law tests, the pre-existing parser/
mutation/diff/absorb/field-sweep suite, the demo/serializer/deserializer/analyzer tests).

```
cargo test -p semio-s-plugin-stdio --lib
```
→ 1714 passed, 0 failed, 1 ignored (recipe's own "expect ≥1671/0/1-ignored" baseline — no
regressions; the higher count reflects other concurrent FG-wave landings in the shared tree, not
anything from this wave).

```
bun run ./📜️script.ts policy
```
→ zero entries mentioning `📝️md` anywhere in the full breach report (grepped explicitly). Every
reported breach is pre-existing and unrelated (`os-state-authority/*`, `budget/no-budget-null`,
neither touching this artifact nor any of the 5 policies this wave's checklist cares about).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (real `OpBinary`, `demo_mutation_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (real binary primitives, real `DiffCodec`, `demo_diff_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/⚙️engine/🦀️component.rs` (`demo_md_snapshot()`, 5-role registration, `conformance_laws`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (real fixture, replacing the bare fake)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real fixture)
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-md-report.md`

Not touched: every other schema-representation sibling file (`.g4`/`.graphql`/`.json`/`.ebnf`/
`.proto`/`.ts`/`.ksy`/`.abnf`/`.spicy`) — out of scope (only `.grammar.semio`/`.protocol.semio` per
the recipe); `📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol/registry modules,
`🧪️fixture-sweep`, `🏪️store` — per the ticket's explicit ownership boundary.

## Deviations from the brief

- `demo_md_snapshot()`'s block ORDER is deliberately non-obvious (`CodeBlock` last) — see the
  framework-bug section above; documented in-line rather than left as a silent surprise.
- Grammar-file productions ended up needing every alternative collapsed to one physical line
  (recipe §3 pitfall #4) even though I originally drafted several across multiple lines for
  readability — caught by `committed_facet_files_parse` before landing, not a deviation in
  substance, just noting the drafting mistake and fix for the record.
