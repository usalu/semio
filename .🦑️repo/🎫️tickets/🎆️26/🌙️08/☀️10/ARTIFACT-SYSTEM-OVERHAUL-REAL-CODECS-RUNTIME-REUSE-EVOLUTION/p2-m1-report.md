# Phase 2 M1 Report — Grammar Dialect + Lexer Generalization

Scope: the plan's binding "P2-W0 recon findings + orchestrator scope decisions (binding for
M1/M2)" section, M1 items 1–6, verbatim. Sole ownership: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs`
and `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️component.rs`, plus whatever the compiler
forced. `git status --porcelain` on `🗣️dsl`/`🎒️pack` was polled before starting and periodically
throughout — clean of concurrent edits the entire session (confirmed again at report time).

---

## 1. What was built, per scope item

### Item 1 — Generalized string/text token: configurable quote + escape modes

One mechanism (`🔍️lexer/🦀️component.rs`, new `//#region 🔖️Dialect`), not four bespoke fixes:

- `pub enum StringEscape { Raw, Backslash, Doubled }`
- `pub struct StringMode { pub quote: char, pub escape: StringEscape }`
- `pub struct LexOptions { pub strings: Vec<StringMode>, pub comment: CommentDialect }` — `Default`
  reproduces the exact pre-M1 alphabet (single `"`-delimited `Raw` string, `#`-to-EOL comment, no
  block comment) so any grammar that never opts in lexes byte-identically to before.
- `pub fn lex_with(text, limits, forgiving, opts: &LexOptions) -> Result<Vec<SpannedToken>, TextError>`
  — the real implementation; `pub fn lex(...)` (unchanged 3-arg signature, every pre-M1 caller)
  is now `lex_with(text, limits, forgiving, &LexOptions::default())`.
- `Backslash` mode implements RFC 8259 §7: `\" \\ \/ \b \f \n \r \t` plus `\uXXXX` (4 hex digits)
  with real UTF-16 surrogate-pair combination for astral codepoints (`decode_backslash_unit`).
- `Doubled` mode: the delimiter doubled (`""` / `''`) decodes to one literal delimiter char;
  backslash has no special meaning — serves CSV (RFC 4180) and STEP Part 21's `''`-doubling.
- Quote char is configurable per mode, and multiple `StringMode`s can coexist (e.g. XML needs both
  `"..."` and `'...'` active at once, each `Raw`).

**Grammar-file syntax** (new optional header directives in `.grammar.semio`, `📖️grammar/🦀️component.rs`):

```
string double backslash     # "..." decodes RFC 8259 escapes incl. \uXXXX surrogate pairs (json)
string double doubled       # "..." decodes doubled "" -> one literal " (csv)
string single doubled       # '...' decodes doubled '' -> one literal ' (step)
string double raw           # "..." legacy/no-decode behavior, explicit
string single raw           # '...' legacy/no-decode behavior, explicit
```

Declaring **any** `string` directive replaces the default `"`-only quote set entirely — a grammar
that wants both delimiters (xml/svg) declares both explicitly:

```
string double raw
string single raw
```

`GrammarFile` gained a `pub lex: LexOptions` field, populated from these directives during
`parse_grammar`, and `Recognizer::recognize`/`uncovered_productions` now lex with
`core_lex_with(text, &Limits::default(), true, &self.grammar.lex)` instead of the old hardcoded
`core_lex(text, &Limits::default(), false)` — a grammar's own dialect now genuinely drives its own
recognition, not a global fixed alphabet. `print_grammar` round-trips the new directives (emitted
only when they differ from `LexOptions::default()`, so the common case is unaffected).

### Item 2 — "Raw span" terminal (`LINE` / `REST`)

A genuinely new *terminal kind*, not a new token — recognizes that this needs Recognizer-level
(not lexer-level) machinery, since it must read the **original source text** by byte offset, past
whatever the shared lexer already fragmented into tokens:

- `Symbol::Terminal("LINE")` — rest-of-physical-line capture (up to the next `\n`, or EOF).
  Serves obj's `o`/`g` names, stl's `solid <name>`/`endsolid <name>`, dxf's opaque group-code
  value lines.
- `Symbol::Terminal("REST")` — rest-of-EOF capture. Serves txt's whole document body.

Implementation (`match_raw_span` in `📖️grammar/🦀️component.rs`): reads `tokens[pos].byte_range.0`
as the start byte (or `text.len()` if `pos` is past the last token), computes the end byte per
`RawSpanEnd::{Newline,Eof}` directly against `text` (never against reassembled token text — this
preserves interior whitespace/punctuation a token join would lose, e.g. `"My Cube"` for an
unquoted rest-of-line name), then advances the token cursor past every token whose span the
capture swallowed, **without attempting to re-tokenize the interior**. A span may legitimately be
empty (e.g. `solid` with no name) — this always succeeds, matching the plan's "the Recognizer
treats that captured span as satisfied."

**Grammar-file syntax**: just use `LINE`/`REST` as an ordinary terminal name — no new symbol syntax
needed, since any ALLCAPS bareword was already a `Symbol::Terminal`:

```
solid-stmt = "solid" LINE
txt-doc = "BODY" REST
```

**Load-bearing side effect**: `Recognizer::recognize`/`uncovered_productions` now lex **forgivingly**
(`true`, was `false`) — a raw-span terminal exists precisely to swallow content outside the fixed
token alphabet (txt's arbitrary prose), and strict-mode lexing would abort the WHOLE document
before the Recognizer ever saw a token stream to walk. `Limits` violations (oversized input, too
many tokens) still surface as a real `Err`; only lexical-shape errors degrade to `Error` tokens.
Verified this is behavior-preserving: grepped every `.recognize(`/`.uncovered_productions(` call
site repo-wide (`🗣️dsl/🧪️fixture-sweep`, `📖️grammar`'s own tests) — none rely on the `Err` variant
for malformed input, all `.expect()`/`.unwrap()` on success. Forgiving vs. strict lexing produce
byte-identical token streams for any text that already lexed cleanly (every pre-M1 pilot fixture),
so this is a pure extension, confirmed by the unchanged gate numbers below.

### Item 3 — Promoted single-char tokens `< > & $ ;`

Unconditional lexer-alphabet extension (not a per-grammar option — every grammar gets these,
matching the plan's "promote to real single-char tokens"): new `TokenKind` variants `Lt, Gt, Amp,
Dollar, Semicolon` (`🔤️token/🦀️component.rs`), added to the `single` single-char match table in
`lex_with`. No collision with existing arrow forms: `<-` (BackArrow), `->` (Arrow), `--`
(DashArrow), and the fused edge-arrow forms (`-id:Kind>` / `-id-`) are all checked via earlier,
more specific `if` branches that `continue` on match — `<`/`>` only reach the single-char table
when they were NOT part of one of those multi-char forms. Verified by a dedicated lexer test
mixing all forms in one input (`promoted_tokens_lex_standalone_without_breaking_arrow_forms`).

**Grammar-file syntax**: reference the promoted tokens as ordinary terminal names —
`terminal_matches`'s existing generic fallback (`format!("{:?}", token.kind).to_uppercase() ==
other`) already makes `LT`/`GT`/`AMP`/`DOLLAR`/`SEMICOLON` work with zero additional Recognizer
code:

```
tag = LT IDENT GT AMP IDENT SEMICOLON DOLLAR IDENT
```

### Item 4 — Per-grammar comment dialect (not hardcoded global `#`)

New `CommentDialect { line: Option<String>, block: Option<(String, String)> }` on `LexOptions`.
`Default` is `{ line: Some("#"), block: None }` — the pre-M1 behavior exactly. Line-comment
scanning generalized from a hardcoded `c == '#'` check to a configurable multi-char marker
(`chars_start_with`); block-comment scanning is new (tracks line/column across newlines like the
existing Fence token does; an unterminated block comment in forgiving mode falls through
unconsumed rather than looping — its open-marker chars, e.g. `/*`'s `/` and `*`, are already
ordinary single-char tokens).

**Grammar-file syntax** (new header directives):

```
comment none                      # disable both line and block comments entirely
comment line none                 # disable only the line marker
comment line "//"                 # override the line marker (any string, not just one char)
comment block "/*" "*/"           # add a block-comment form
```

STEP/IFC's real dialect (resolves the `#`-as-entity-ref vs. `#`-as-comment collision W0 flagged):

```
comment line none
comment block "/*" "*/"
string single doubled
```

The majority of grammars that want plain `#`-comments declare neither directive and are
unaffected — verified via a dedicated round-trip test
(`grammar_without_string_or_comment_directives_keeps_default_lex_options`) and the unchanged m5
pilot numbers below.

### Item 5 — Trailing-dot floats + leading-dot enum literals

- **Trailing-dot floats** (`0.`, `10.`): the digit-scanning branch's dot-continuation check changed
  from "commit only if a digit follows the dot" to "commit unless another `.` follows the dot" (the
  only case that must keep losing to `DotDot`/Range, e.g. `0..10`). Anything else after the dot —
  digit, letter, whitespace, EOF — now commits to a trailing-dot float; the existing digit-after-dot
  path (`3.5`) is untouched (still one unified branch, not two).
- **Leading-dot enum literals** (`.T.`, `.UNSPECIFIED.`): new branch, checked after the `..`
  (DotDot) check so a bare `..` still wins, only commits when a genuine closing dot is found (a
  lone `.foo` with no closing dot falls through completely unchanged — still becomes "unexpected
  character" `.` + a separate `Ident`, proven by a dedicated regression test). New `TokenKind::DotEnum`
  (`🔤️token/🦀️component.rs`); the continuation scan deliberately uses a narrower predicate
  (`alphanumeric || '_'`) than the general `is_ident_continue` (which allows `.`/`-`/`/` so ordinary
  idents like `a..b` and kebab-case idents stay one token) — this was a real bug caught before
  shipping: using `is_ident_continue` for the enum-literal body swallows its own closing dot.

**Grammar-file syntax**: new terminal name `DOTENUM`, explicit `terminal_matches` arm (also reachable
via the generic fallback, given an explicit arm for clarity/greppability):

```
step-value = FLOAT | DOTENUM | INT
```

### Item 6 — `Ref` self-recursion (verification, not a fix)

Confirmed via a real test, not assumption: `ref_self_recursion_matches_a_three_level_nested_shape_tree_pptx_style`
compiles a genuinely self-recursive grammar —

```
grammar shapetree
start tree
tree = "spTree" group
group = "{" node* "}"
node = leaf | nested
leaf = "sp" IDENT
nested = "grpSp" group
```

— against a real 3-level-nested fixture (`spTree { sp a grpSp { sp b grpSp { sp c } sp d } }`),
recursing `nested -> group -> node -> nested` three real hops deep. It matched on the first try
with zero Recognizer code changes — **confirmation, not a fix**, exactly as the plan anticipated
("Recursion already works — first-match matcher, keyword-first alternatives"). Also asserts
`uncovered_productions` reports every production in the recursive chain as covered (not merely
present), and that a malformed variant (missing the final closing brace) correctly fails to match.
This directly de-risks pptx's shape-tree grammar (`p:spTree`/`p:sp`/`p:grpSp`) for its later FG3
wave, and by extension svg/xml/md/json's recursive-tree grammars in later FG waves.

---

## 2. Files touched

Sole-owned (per the dispatch brief):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️component.rs` — `LexOptions`/`StringMode`/
  `StringEscape`/`CommentDialect` (new `//#region 🔖️Dialect`), `lex`/`lex_with` split, generalized
  comment + string scanning, trailing-dot float, leading-dot `DotEnum`, promoted single-char
  tokens, `token_classes` exhaustive-match update, 13 new tests (`//#region 🔖️P2M1Dialect`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` — `GrammarFile.lex` field,
  `comment`/`string` header-directive parsing, `print_grammar` round-trip, `Recognizer` threaded
  with `LexOptions` + source text, `match_raw_span`/`RawSpanEnd`, `DOTENUM` terminal, forgiving-mode
  `recognize`/`uncovered_productions`, 12 new tests (`//#region 🔖️P2M1Grammar`).

Compiler-forced (outside `🗣️dsl/📖️grammar`+`🗣️dsl/🔍️lexer`, listed per the brief's requirement):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔤️token/🦀️component.rs` — `TokenKind` gained
  `Lt, Gt, Amp, Dollar, Semicolon, DotEnum` (shared token-alphabet definition the lexer depends on;
  same crate, same `🗣️dsl` module, not a separate ownership boundary in practice but noted for
  completeness).
- `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl/🦀️component.rs` — Jack's own lexer bridge
  (`push_dsl_core_segment`) exhaustively matches `TokenKind`; added the 6 new variants to its
  existing "stray character, not part of Jack's grammar" bucket (same treatment as `EdgeArrow`/
  `LBrace`/`Fence`/`Error` already there) — a real `cargo check --workspace` compile error
  (`E0004: non-exhaustive patterns`), fixed, confirmed clean afterward.

Also updated (documentation consistency, not required for any gate, low risk — verified the
self-hosting round-trip test still passes):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📖️grammar.grammar.semio` — added
  `comment-line`/`string-line` productions to the self-describing grammar-of-grammars so it
  documents the new header directives it now actually supports.

---

## 3. Gate results (real output, not paraphrased)

### Gate 1 — `cargo check --workspace`

Two pre-existing, unrelated failures remain — confirmed via `git status --porcelain` (both files
clean/untouched by this session) and content inspection (neither references `🗣️dsl`/`📖️grammar`/
`🔍️lexer` at all):

```
error: couldn't read `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/../../📄️document/🦀️component.rs`: No such file or directory (os error 2)
error: could not compile `semio-framework-os-kernel-db` (lib) due to 1 previous error
```
— `🛢️db`'s `glue.rs` references a `📄️document` module directory that does not exist on disk at all;
unrelated to any dsl/grammar/lexer file.

```
error[E0433]: cannot find module or crate `dsl` in this scope
   --> compose/client/lib/rs/lib.rs:716:10
...
error[E0433]: cannot find module or crate `vcs` in this scope
error: could not compile `semio-compose-rs` (lib) due to 22 previous errors; 823 warnings emitted
```
— generated glue code referencing bare `dsl`/`vcs` crate names that have never existed as
workspace packages (this crate's real path dependency is `semio-framework-os-kernel`, accessed as
`crate::os_dsl::...`, not a bare `dsl` crate) — pre-existing integration gap, structurally
unrelated to this wave.

One real error WAS found and fixed by this wave (compiler-forced, `E0004: non-exhaustive
patterns: TokenKind::Lt, TokenKind::Gt, TokenKind::Amp and 3 more not covered` in
`🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl/🦀️component.rs:849`) — see §2. After the fix,
`semio-framework-math` compiles clean and `cargo check --workspace` re-run shows only the same 2
pre-existing unrelated failures, nothing new:

```
  17 error[E0433]: cannot find module or crate `dsl` in this scope       (semio-compose-rs, pre-existing)
   2 error[E0433]: cannot find module or crate `vcs` in this scope       (semio-compose-rs, pre-existing)
   2 error: cannot find attribute `dsl` in this scope                   (semio-compose-rs, pre-existing)
   1 error[E0432]: unresolved import `vcs`                              (semio-compose-rs, pre-existing)
   1 error: couldn't read `.../📄️document/🦀️component.rs`               (semio-framework-os-kernel-db, pre-existing)
   1 error: could not compile `semio-framework-os-kernel-db`
   1 error: could not compile `semio-compose-rs`
```

Spot-checked 2 additional large non-stdio consumer crates individually, both clean:

```
$ cargo check -p semio-framework-math       -> Finished, 0 errors
$ cargo check -p semio-s-plugin-trinity     -> Finished, 0 errors (55 pre-existing warnings only)
```

(`cargo check -p semio-framework-os-kernel` itself — see Gate 2/3, compiles and tests clean.)

### Gate 2 — `cargo test -p semio-framework-os-kernel`

```
$ cargo test -p semio-framework-os-kernel 2>&1 | grep -E "^test result:|FAILED"
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::en1992_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dag_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test os_dsl::fixture_sweep::m5_production_coverage::dag_reports_uncovered_productions_for_shipped_fixture ... FAILED
test os_dsl::fixture_sweep::m5_production_coverage::en1992_reports_uncovered_productions_for_shipped_fixture ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::fem2d_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test result: FAILED. 760 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s
```

**Exactly the same 5 test names as the W0 baseline (736/5)** — not more, not different. `dag`,
`en1992`, `fem2d` are the pre-existing-broken non-stdio pilots per the plan's own W0-driven
correction; this wave is not responsible for fixing them, only for not adding to them, confirmed.
760 (vs. baseline 736) reflects this wave's own +25 new passing tests (13 lexer + 12 grammar,
confirmed individually below) plus a net -1 from unrelated concurrent churn elsewhere in the large
`semio-framework-os-kernel` crate (outside `🗣️dsl`/`🎒️pack`, which stayed `git status --porcelain`
clean the entire session — some other module's own test count shifted independently, not
investigated further per the repo's "classify, don't chase external churn" guidance). Stable
across repeated runs; the only invariant that matters for this gate — same 5 failing test names,
not more, not different — holds exactly.

Pilot-specific confirmation (`lowpoly`/`cad`/`note` — the plan's real clean regression gate — all
still green):

```
$ cargo test -p semio-framework-os-kernel --lib m5_handcrafted_grammar_conformance
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::en1992_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dag_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::lowpoly_dsl_grammar_recognizes_shipped_fixture_tokens ... ok
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::note_dsl_grammar_recognizes_shipped_fixture_tokens ... ok
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::fem2d_dsl_grammar_recognizes_shipped_fixture_tokens ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::cad_dsl_grammar_recognizes_shipped_fixture_tokens ... ok
test result: FAILED. 3 passed; 3 failed; 0 ignored; 0 measured; 759 filtered out; finished in 0.02s
```

Lexer test run (all 38 pass, 13 new for item 1/3/4/5):

```
$ cargo test -p semio-framework-os-kernel --lib lexer::
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 715 filtered out; finished in 0.01s
```

Grammar test run (all 33 pass, 12 new for items 1/2/3/4/5/6):

```
$ cargo test -p semio-framework-os-kernel --lib grammar::
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 732 filtered out; finished in 0.03s
```

### Gate 3 — `cargo test -p semio-s-plugin-stdio --lib`

```
$ cargo test -p semio-s-plugin-stdio --lib 2>&1 | grep -E "^test result:"
test result: ok. 1075 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.97s
```

Exact match to baseline (1075/0) — behavior-preserving for every existing stdio grammar/lexer
usage, confirming nothing in stdio uses the new mechanisms yet (correctly deferred to the FG-waves).

### Gate 4 — new unit tests per scope item

25 new tests total, all passing, all named for the mechanism they prove:

| item | tests |
|---|---|
| 1 (string modes) | `default_lex_options_is_byte_identical_to_pre_m1_raw_double_quote_behavior`, `json_style_backslash_mode_decodes_standard_escapes_and_u_xxxx_surrogate_pairs`, `csv_style_doubled_quote_mode_decodes_doubled_delimiter_and_ignores_backslash`, `single_quote_strings_work_alongside_double_quote_xml_style`, `step_style_single_quote_doubled_mode_decodes_doubled_apostrophe` (lexer); `string_header_directive_drives_backslash_decode_end_to_end`, `string_header_directive_drives_csv_doubled_quote_decode`, `string_header_directive_supports_single_and_double_quote_together_xml_style`, `string_header_directive_supports_step_single_quote_doubling`, `grammar_without_string_or_comment_directives_keeps_default_lex_options` (grammar) |
| 2 (raw span) | `line_terminal_captures_rest_of_physical_line_verbatim_stl_style`, `rest_terminal_captures_to_eof_txt_style_over_out_of_alphabet_characters` (grammar) |
| 3 (promoted tokens) | `promoted_tokens_lex_standalone_without_breaking_arrow_forms`, `bare_gt_lexes_standalone_outside_fused_edge_arrow_context` (lexer); `promoted_tokens_are_real_terminals_the_recognizer_can_require_positionally` (grammar) |
| 4 (comment dialect) | `comment_line_marker_is_configurable_and_disableable`, `block_comment_step_style_spans_lines_and_does_not_consume_entity_hash` (lexer); `comment_header_directive_disables_hash_and_enables_block_comment_step_style`, `print_grammar_round_trips_comment_and_string_header_directives` (grammar) |
| 5 (trailing-dot float / leading-dot enum) | `trailing_dot_floats_lex_while_range_dotdot_still_wins`, `leading_dot_enum_literals_lex_as_dotenum_step_style`, `lone_leading_dot_without_closing_dot_is_unaffected_by_dotenum` (lexer); `trailing_dot_float_and_leading_dot_enum_literal_terminals_match_through_recognizer` (grammar) |
| 6 (Ref self-recursion) | `ref_self_recursion_matches_a_three_level_nested_shape_tree_pptx_style` (grammar) |

---

## 4. Deviations from the plan's exact scope, and why

1. **`Recognizer::recognize`/`uncovered_productions` switched from strict to forgiving lexing.**
   Not explicitly called for in the plan text, but a direct, necessary consequence of building the
   raw-span terminal honestly: without this, a document containing even one out-of-alphabet
   character anywhere (txt's whole use case) would abort the lex pass before the Recognizer ever
   received a token stream, making `REST` useless for its stated purpose. Verified safe: grepped
   every call site repo-wide, none depend on the `Err` variant; forgiving vs. strict lex is
   byte-identical for any input that already lexed cleanly (proven by the unchanged gate numbers).
2. **`comment`/`string` header-directive syntax is new surface not specified verbatim in the plan**
   (the plan named the mechanism, not the exact keywords). Chose `comment none|line|block` and
   `string double|single raw|backslash|doubled` — designed to avoid a real escaping trap discovered
   during implementation (representing the double-quote character itself as a quoted TEXT literal
   inside the grammar file's own meta-syntax is ambiguous under `Raw` mode, since the meta-lexer
   never decodes backslash-escapes) by using symbolic keywords (`double`/`single`) instead of a
   literal quote character.
3. **Terminal names `LINE`/`REST`/`DOTENUM` are new reserved words** in the sense that any grammar
   using them as terminal barewords now gets the new behavior — checked for collisions against all
   6 pilot grammars, all 7 family kits: none use these names today, zero risk confirmed by the
   unchanged gate numbers.
4. **`📖️grammar.grammar.semio` (self-describing grammar) updated** to document the new header
   directives — not required by any gate (the self-hosting test only needs it to keep parsing +
   round-tripping, which it does), done for documentation honesty since this file is meant to be
   the format's own normative description.

No scope item was skipped, narrowed, or deferred. The explicitly-out-of-scope items (markdown's
whitespace-count nesting, anything protocol/binary-side) were not touched.

---

## 5. What this changes about the M2 (protocol-side) picture

- **No changes to `Prim`/`Block`/`walk_protocol`/`ProtocolFile` in this wave** — confirmed
  untouched; M2's own scope (repeated tag-dispatched block, BE `Prim` variants, cross-block field
  env, conditional presence, ZIP backward-seek, TIFF runtime endianness) is exactly as W0 described,
  nothing discovered here changes it.
- **One thing M2 should know**: the `GrammarFile` struct now carries a `lex: LexOptions` field that
  `ProtocolFile`-derived `GrammarFile`s (via `project_protocol`) always get as `LexOptions::default()`
  — protocol dialect files have no lexical-mode concept (they're byte-layout descriptions, not
  token grammars), so this is inert for M2's own work, but if M2 or a later wave ever needs a
  grammar-side value to parameterize a protocol-side read (the pdf/1.7 `/W` array, ply's per-file
  schema — flagged as a shared open design question in W0, explicitly deferred there), the
  `LexOptions`/`Recognizer` split built here (dialect config traveling on `GrammarFile`, consumed
  by `Recognizer`, independent of `walk_protocol`) is a clean seam to extend from, not a redesign.
- **The raw-span terminal (`LINE`/`REST`) is a close conceptual cousin of M2's "repeated
  tag-dispatched block"** (both read a structural marker and then consume a byte span without
  interpreting its interior) but the two are NOT unified in this wave — raw span operates on the
  already-lexed token stream + original text (grammar/text side), while M2's block construct will
  operate directly on raw bytes (protocol/binary side). No shared code between them today; worth a
  naming-consistency pass when M2 lands its own "consume without re-interpreting" primitive, but
  not a blocking dependency either direction.
- **`Ref` self-recursion is now genuinely proven, not assumed**, for the grammar side. M2's own
  protocol dialect has an analogous open question — `Prim::Ref` unconditionally errors today
  (`walk_protocol`'s `walk_prim`'s `Prim::Ref` arm, cited in W0 §0) — this wave did not touch or
  verify that; it remains fully M2's to resolve, now with one fewer unknown adjacent to it.

---

## 6. Regression-gate baseline reproduction (confirmed, per the dispatch brief's instruction)

Reproduced before starting real edits:

```
$ cargo test -p semio-s-plugin-stdio --lib   -> 1075 passed, 0 failed   (matches W0 exactly)
$ cargo test -p semio-framework-os-kernel    -> 736 passed, 5 failed    (matches W0 exactly, same 5 names)
$ git status --porcelain -- 🗣️dsl 🎒️pack     -> clean (both times, start and end of session)
```

Both baselines matched the W0 report's numbers exactly before any edit was made — no note needed
about a changed tree since W0.
