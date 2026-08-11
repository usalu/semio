# P2-P1 Independent Verification Report — `json` (rfc8259) and `csv` (rfc4180) grammar pilots

Scope: independent re-verification of the two P1 pilots' self-reports (`p2-p1-json-report.md`,
`p2-p1-csv-report.md`). Both self-reports state they were **blocked from running
`cargo test`** by unrelated concurrent-session compile breakage and therefore report
`tests_passed`/`tests_failed` as unknown, substituting "static/design-level verification" (manual
tracing of grammar productions against the real Rust parser/codec source). This report actually
ran the tests — repeatedly, until a clean compile window was caught — and found **both pilots'
own new conformance tests genuinely fail**, for a reason neither self-report anticipated. This is
the headline finding; details below.

## 0. Compile-instability finding (independently reproduced, not merely trusted)

`cargo test -p semio-s-plugin-stdio --lib` compiles the ENTIRE crate (all stdio artifacts) as one
test binary — confirmed true by direct observation: error sets changed between consecutive
attempts seconds apart (41 errors → 3 errors → 2 errors → 0 errors, touching `epw`, `html`, `mp3`,
`wav`, then finally clean), and `ps aux`-style evidence (fluctuating diagnostics) matches a live
concurrent session's mid-refactor, exactly as both self-reports describe. This independently
confirms the self-reports' claim about *why* they couldn't get a definitive run — that part is
real, not an excuse. Repeated polling (roughly 15 attempts over ~15 minutes, `sleep`-spaced)
eventually caught clean compiles, at which point real test output was captured (below). Static
verification alone was insufficient — see §2.

## 1. Real, run test counts (scoped filters, real code, real dialect confirmed by inspection)

### `json`
- Scoped run (`artifacts::json`): **61 passed, 5 failed**, 1432–1492 filtered out (fluctuates with
  concurrent-session file churn elsewhere, not this artifact).
- The 5 failures are ALL 5 of the new P2-P1 conformance tests:
  `conformance_laws::committed_facet_files_parse`, `conformance_laws::grammar_conformance_law`,
  `conformance_laws::ops_grammar_conformance_law`, `conformance_laws::diff_grammar_conformance_law`,
  `conformance_laws::protocol_walk_law`.
- `conformance_laws::fixture_honesty_law` **passes**.
- All pre-existing (F6-era) tests pass.

### `csv`
- Scoped run (`artifacts::csv`): **20–21 passed, 4 failed, 1 ignored**, stable across 6 repeated
  runs.
- The 4 failures: `engine::tests::committed_grammar_and_protocol_files_parse`,
  `engine::tests::grammar_conformance_law`,
  `subsets::any::schema::mutations::component::tests::ops_grammar_conformance_law`,
  `subsets::any::schema::diff::component::handcrafted_diff_codec_tests::diff_grammar_conformance_law`.
- `engine::tests::fixture_honesty_law` and `engine::tests::protocol_walk_law` **pass**.
- All pre-existing (F6-era) tests pass, including `diff_codec_text_binary_roundtrip_law` and
  `op_text_binary_roundtrip_law` (the real binary-frame round-trips).

### Full crate (one clean run, caught after polling)
`1479–1481 passed, 16–18 failed, 1 ignored` (small fluctuation is the still-live, unrelated
`artifacts::semio::v1` subsets churn — `brep`/`mesh`/`model`/`workflow` diff/mutation tests,
confirmed by file path to be outside json/csv's ownership, consistent with the "large concurrent
session" this ticket's briefing already flags). Of the failures, **exactly 9 are json(5)+csv(4)**;
the remaining 7–9 are the unrelated semio-v1 churn. No other stdio artifact showed a failure in
this run.

## 2. Root cause of the 9 failures — a real, reproducible framework-lexer bug, NOT flakiness

All 9 failures are `TextError`s from `dsl::parse_grammar`/`dsl::parse_protocol` itself — i.e. the
*committed grammar/protocol files fail to parse under the real dialect parser*, not a downstream
recognition mismatch (with one partial exception, see below). Root-caused by directly reading
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs`'s `lex()` function (the
`.grammar.semio`/`.protocol.semio` file's own meta-lexer, M1's module) and reproducing its exact
algorithm in a standalone Python script against the actual committed files:

`lex()` pre-scans the raw file text character-by-character for bare `"`, `?`, `|` — used to peel
off `?`/`|` as real grammar-alternation tokens and to skip over quoted `TEXT` literals whole (so a
literal `"|"` inside a real grammar production isn't mistaken for the alternation operator) —
**before** delegating the remaining segments to the shared `core_lex`. This pre-scan has **no
concept of `#`-line-comments**: it operates on raw bytes, not on `core_lex`'s own comment-aware
tokenization. Two concrete failure shapes, both confirmed by simulation against the real files:

1. **A `|` or `?` character inside `#`-comment prose** (e.g. json mutations grammar line ~14:
   `` # ... `(INT | IDENT)*` — zero or more ... ``, describing the grammar's own alternation
   syntax in English) gets peeled off as a real `Pipe`/`Question` token, splitting the comment
   line in half. The second half (e.g. `` IDENT)*` — zero or more of either —... ``) no longer
   starts with `#`, so when handed to `core_lex` as its own segment it is tokenized as *code*, not
   comment — and the stray `` ` `` (backtick, not in the shared lexer's alphabet at all) triggers
   `"unexpected character '`'"`. This exact shape is what breaks `ops_grammar_conformance_law` for
   BOTH json (mutations grammar, backtick error at segment-local "line 1 col 9") and csv (same
   signature, same root cause, its own mutations grammar comment describing
   `` `(INT | IDENT)*` ``-shaped prose).
2. **An odd number of raw `"` characters inside comment prose** (e.g. csv snapshot grammar's line
   ~32, `` a literal `"` escaped by doubling (`""`) `` — 3 raw `"` characters on one line, natural
   phrasing for documenting RFC 4180's doubled-quote escape) desyncs the pre-scan's open/close
   quote-tracking for the rest of the file — everything from that point is alternately treated as
   "inside a fake string literal" or "code," consuming or misrouting real production lines,
   eventually surfacing a `|`/`` ` `` in the wrong place. This is what breaks
   `grammar_conformance_law`/`committed_facet_files_parse`/`committed_grammar_and_protocol_files_parse`
   for both artifacts' snapshot grammars (json's own trigger: line 17's `` `\" \\ \/ \b ...` ``,
   documenting RFC8259's own backslash-escape set — desyncs from that `\"` through to the first
   real quote in `artifact-mark = "stdio.json"`).

Reproduced independently (not inferred from the Rust error alone) with a standalone Python
re-implementation of `lex()`'s exact byte-scanning algorithm run directly against the 12 committed
files (`/private/tmp/.../scratchpad/simlex.py` in this session's scratchpad, not committed
anywhere) — confirms an odd/unterminated quote span or a mid-comment pipe/question split in
exactly the files whose conformance tests fail, and none in the files whose tests pass (both
protocol-only files and the diff/mutations protocol files for both artifacts, which happen not to
contain this comment phrasing, parse and pass).

**This is a real, load-bearing bug in shared framework code (`🗣️dsl/📖️grammar`, M1's ownership,
outside both artifacts' boundary and outside mine) — but its *symptom* is that the two pilots'
own, as-authored, currently-committed grammar files do not actually parse via the real
`dsl::parse_grammar` they were written to be validated against.** Writing accurate, thorough
prose documentation of a text format's own escape/alternation syntax (exactly what CLAUDE.md's
"be extremely thorough" + docstring conventions ask for, and exactly what these two pilots did)
is the direct trigger. Neither self-report's `mechanism_gaps` section anticipated this failure
mode — both instead documented a different, real, already-known gap (`Prim::Ref` recursion /
`register_schema_spec`'s `RecordSpec` requirement). This is a genuinely new finding, produced only
by actually executing the parser against the actual files, which is exactly why the self-reports'
"static/design-level verification... traced token-by-token" substitute was insufficient here —
manual tracing checked *production correctness* against the real Rust output shape, but neither
self-report actually ran `dsl::parse_grammar`/`dsl::parse_protocol` against its own comment prose,
which is precisely where the bug lives.

### The one non-parse failure: csv's `diff_grammar_conformance_law`
Distinct from the other 8: csv's diff grammar *does* parse, but the `Recognizer` fails to
recognize a real `print_diff()` output line (`"has-header=0 records{[2];[0:[1,[[1,[V:...` — a
representative multi-field-diff case with nested record modifications). This is a genuine
grammar-content gap (the grammar under-describes the recursive `records{...}` collection-diff
shape for a non-trivial case), not the lexer-comment bug — worth flagging separately for whoever
picks this up next, since fixing the comment-lexer bug alone will not make this one pass.

## 3. Static-verification items independently re-confirmed as accurate

Despite the above, the *content* claims in both self-reports check out against direct reading of
the real Rust source, independent of the broken parse:

- **Real dialect headers**: both artifacts' 6 files use the real M1 header
  (`dialect grammar`/`grammar <id>`/`extension <ext>`/`start <production>`,
  `dialect protocol`/`protocol <id>`/`version 1`/`schema <id>`/`start <production>`) — not the old
  one-line ABNF-placeholder fossil. Confirmed by direct read of all 12 files.
- **Grammar bodies genuinely describe the real format**: json's snapshot grammar's
  `object`/`array`/`value`/`number` productions and RFC8259 escape set (`string double backslash`)
  match `📸️snapshot/🦀️component.rs`'s real `Parser::parse_string`/`parse_number` byte-for-byte
  (traced directly, lines 193-310). csv's snapshot grammar's quote-state-aware
  `record`/`field`/`quoted-field`/`unquoted-field` productions and `string double doubled` mode
  match `⚙️engine/🦀️component.rs`'s real `parse_csv_records` tokenizer (traced directly, lines
  20-90) — comma/CR/LF-in-quotes, doubled-`""`-escape, both handled correctly.
- **`DiffCodec`/`OpBinary` real binary frames**: grepped both artifacts for the literal
  `print_op().into_bytes()`/`print_diff().into_bytes()` F6 shortcut pattern — zero hits in either
  (only doc-comment prose describing what was replaced). Both `encode_op`/`decode_op` and
  `encode_diff`/`decode_diff` are real hand-rolled binary codecs (`format u8 | tag/ordinal u8 |
  recursive payload`, LEB128 varints via `store::pack_rt`/`dsl::ByteWriter`/`dsl::ByteReader`) —
  confirmed by direct read, not merely by absence-of-pattern.
- **Fixtures are real**: both `🗣️example.dsl.semio` files start with the mandatory
  `semio stdio.<artifact>.dsl v1` preamble and contain genuine, non-trivial content (json: 3-level
  nested object/array/scalar mix exercising every `JsonValue` variant; csv: a header row plus a
  quoted field with an embedded comma AND an embedded doubled-quote, exercising RFC 4180's real
  edge cases). Both `🎒️example.pack.semio` files exist and are real binary data (not text).
  `fixture_honesty_law` passes for both, confirmed by actual test run, not just static claim.
- **5-role `LanguageSpec` registration**: both artifacts' `register_pilot_languages()` register
  exactly the 5 roles (Document/Ops/Diff/Pack/Spr, ids `stdio.<artifact>[.op|.diff|.pack|.spr]`),
  all `dsl::passthrough_hooks`, matching note's exemplar pattern — confirmed by direct read.
  `register_schema_spec` was **not called** by either — both self-reports disclose this honestly
  with a real, verified justification (`JsonValue`/`CsvSnapshot`+`CsvDiff` have no derivable
  `RecordSpec`; hand-rolled codecs, confirmed by reading the actual types, not fabricated).
- **JSON-transfer elimination**: json's self-report claim (zero `serde_json::to_vec/from_slice`
  usage) re-confirmed by direct grep — clean.

## 4. Per-artifact summary

| artifact | tests_passed | tests_failed | real_dialect_confirmed | binary_frame_confirmed | fixtures_real | registration_confirmed | notes |
|---|---|---|---|---|---|---|---|
| json | 61 | 5 | true | true | true | true (5-role; `register_schema_spec` honestly skipped, justified) | All 5 failures are `dsl::parse_grammar`/`parse_protocol` errors on the committed files themselves, caused by a framework meta-lexer bug (see §2), not by incorrect grammar content. `fixture_honesty_law` passes. Self-report's "COMPLETE on the artifact side" is not accurate as stated — the artifact's own new conformance tests fail when actually run. |
| csv | 20-21 | 4 | true | true | true | true (5-role; `register_schema_spec` honestly skipped, justified) | 3 of 4 failures are the same framework meta-lexer bug as json (comment-embedded `|`/`?`/odd-`"`-count). The 4th (`diff_grammar_conformance_law`) is a distinct, genuine grammar-content gap: the diff grammar does not recognize a real multi-record-diff `print_diff()` output. `fixture_honesty_law` and `protocol_walk_law` pass. |

## 5. Recommendation

The bug in §2 is framework-owned (`🗗️framework/…/🗣️dsl/📖️grammar/🦀️component.rs`'s `lex()`), out
of both my and both pilots' ownership boundaries. It will very likely recur for every later
FG-wave artifact, since documenting a text format's own escape/alternation syntax in `#`-comments
is exactly what CLAUDE.md's thoroughness rule encourages, and is exactly the trigger. Whoever owns
`🗣️dsl`/`📖️grammar` next should either make the pre-scan comment-aware (skip to end-of-line on an
unescaped `#` before checking for `"`/`?`/`|`) or document a hard constraint ("no literal `"`, `?`,
or `|` characters in `.grammar.semio`/`.protocol.semio` `#`-comments") that every future grammar
author must follow — the current silent failure mode (a confusing, segment-local, wildly
mislocated `TextError`) is very easy to miss exactly as both P1 pilots missed it here.

## Files touched by this verification session
None in the artifacts themselves — read-only verification. Scratch files (Python lexer simulation)
are in this session's scratchpad, not under the ticket folder or repo tree (no repo changes made).
This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-p1-verify-report.md`.
