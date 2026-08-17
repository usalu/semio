# P2-P1 Fix Report: json/csv Conformance Test Parse Failures

## Summary

Root-caused and fixed all 14 originally-failing json/csv conformance tests (5 json + ~4-9 csv,
depending on how you count sub-assertions). **Three independent bugs** were found stacked on top of
each other in the same files — one genuine **framework** bug (fixed, with a regression test) and two
**artifact-authoring mistakes** in json's/csv's own committed `.grammar.semio` files (fixed directly).
None of the failures were caused by anything in `.protocol.semio`'s own body syntax, or by CSV having
a fundamentally different problem than json — both pilots hit the same three bugs, just distributed
differently across their files.

Before (directly reproduced): `cargo test -p semio-s-plugin-stdio --lib
"artifacts::json::standards::v_rfc8259::engine::tests::conformance_laws"` → 1 passed, 5 failed. CSV's
analogous suite wasn't independently re-run pre-fix (its committed files already carried the same Bug
1/Bug 3 shapes as json's, confirmed by static analysis before any framework change — see Bug 1/Bug 3
sections — so I fixed it in the same pass rather than reproduce-then-fix twice).
After: `cargo test -p semio-s-plugin-stdio --lib` → **1497 passed, 0 failed, 1 ignored** (full crate;
was blocked earlier in the session by an unrelated concurrent session's in-progress compile errors in
`artifacts::semio::…::mesh::mutations` — confirmed transient, resolved itself, unrelated to this fix,
not touched by me).
`cargo test -p semio-framework-os-kernel --lib` → 762 passed, 2 failed (same pre-existing baseline —
`fem::2d`/`norm::en1992`/`dag::dag` — 3 hard grammar-recognition mismatches unrelated to stdio,
confirmed unchanged before/after by diffing the failing test names).

## Bug 1 (FRAMEWORK, fixed): comment-unaware operator/quote pre-scanner

**File:** `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs`, `lex()` (~line 272).

**Root cause:** `lex()` is a custom pre-scanner that runs over the *entire raw file text* to find `"`,
`?`, `|` (not in the shared `dsl_core` alphabet) before handing segments off to `core_lex`. It had
**zero knowledge of `#`-comments** — comments are only recognized later, inside `core_lex` itself,
once a segment has already been carved out. So:

- A `?`/`|` character appearing anywhere inside `#`-comment *prose* (e.g. a doc comment illustrating
  `(INT | IDENT)*` or `-? (0 | [1-9][0-9]*)`) was misread as a real grammar Pipe/Question operator,
  splitting the segment mid-sentence. The orphaned remainder of that comment (now missing its leading
  `#`) then got tokenized as if it were real grammar syntax, and a stray markdown-style single
  backtick in it fell through every branch of `core_lex` to "unexpected character".
- A stray `"` inside a comment — even an *escaped-looking* one like `\"` used to illustrate JSON's own
  escape syntax in prose — was blindly treated as opening a real quoted string literal (this
  prescanner also has no backslash-awareness at the *opening* check, only inside an already-open
  quote). This started a runaway "quote-skip" that silently swallowed everything up to the next
  literal `"` anywhere later in the file — including real productions and their own real string
  literals — corrupting quote-parity for the rest of the file.

Empirically confirmed via a standalone segment-boundary simulation (see reasoning trail) before
touching any code: e.g. json's snapshot grammar file's line 17 comment `\" \\ \/ \b \f \n \r \t
\uXXXX` opened a runaway quote-skip spanning lines 17→29, eating the real `artifact-mark`/`document`
productions; json's mutations grammar's line 16 comment `(INT | IDENT)*` (illustrating token
alternation) was misread as a real `|` operator.

**Fix:** `lex()` now recognizes `#`-to-end-of-line and skips it whole, exactly like `dsl_core`'s own
default comment scan, *before* checking for `"`/`?`/`|` — so those characters inside comment prose
never reach the quote/operator dispatch. This matches the framework's own documented convention (the
grammar file's *own* header/body syntax always uses the fixed default `#` comment marker, independent
of whatever `comment` directive the file declares for the compiled Recognizer — see
`push_segment`'s existing `core_lex(segment, &Limits::default(), false)` call, which already only
ever used the default dialect).

**Regression test added:** `os_dsl::grammar::tests::hash_comment_hides_quote_and_pipe_characters_from_the_operator_prescan`
in the same file's `mod tests`, parsing a grammar whose comment illustrates both an escaped quote and
an alternation, asserting the following real production still parses correctly.

**Verified no regression:** all 42 pre-existing `os_dsl::grammar::tests::*` still pass; the framework
`m5_handcrafted_grammar_conformance`/`m5_production_coverage` sweep across all 59 discovered grammar
facets repo-wide shows the exact same 3 pre-existing hard failures (`fem::2d`, `norm::en1992`,
`dag::dag` — non-stdio, unrelated) before and after, and json's own facet flipped from a *soft*
lex-failure to a soft *recognition* mismatch (still stdio-exempt either way, not a hard failure).

## Bug 2 (ARTIFACT, fixed): `(...)*` used for grouping instead of `{...}*`

**Files:** json's snapshot/mutations/diff `.grammar.semio` files.

This dialect reserves bare `( )` **exclusively** for macro-call argument lists (`table("a", b)`) —
grouping is `{ }`, always, never `( )` — documented explicitly in `parse_atom`'s own doc comment
(`📖️grammar/🦀️component.rs` ~line 449): whitespace is trivia and is discarded before parsing, so the
token stream alone can't distinguish `name (group)` from `name(args)`. csv's sibling grammar files
already used `{...}*` correctly throughout; json's did not:

- `object = "{" "}" | "{" member ( "," member )* "}"` and the analogous `array` production used
  `(...)*`, which the parser reads as `member(...)` — a macro call to an undefined macro `member`.
- `hex = ( INT | IDENT )*` in both the mutations and diff grammars — a **bare** `(` at the start of an
  alternative isn't valid syntax at all (`parse_atom` has no case for a standalone `LParen`), giving
  `expected a symbol, found LParen`.

**Fix:** changed all four occurrences to `{...}*`, matching csv's already-correct convention.

## Bug 3 (ARTIFACT, fixed): `hex` modeled as a non-backtracking `Star` production

Once bugs 1 and 2 were fixed, both `ops_grammar_conformance_law` (json) and `diff_grammar_conformance_law`
still failed — not with a parse error, but with `recognize() == false` for real `print_op`/`print_diff`
output. Two independent, stacked issues inside the same `hex = {INT | IDENT}*` idiom (used identically
by json and csv, since both hex-encode opaque byte payloads the same way):

1. **Greedy, non-backtracking `Star` swallows an adjacent keyword.** `Symbol::Star`'s recognizer
   implementation (`match_symbol_tracked`, `📖️grammar/🦀️component.rs` ~line 1957) is a single greedy
   pass with no backtracking — confirmed by reading the code, not assumed. json's `set-member`'s real
   wire shape is `key=<hex> value=<value>`; csv's `set-field-op` is `value=<hex> quoted=<bit>`. The
   literal keyword right after `hex` (`value`, `quoted`) tokenizes as a plain `IDENT` — indistinguishable,
   by *token kind alone*, from a stray hex letter run. `hex`'s own greedy Star, once modeled as a
   production, has no way to know it should stop before that keyword, and silently consumes it,
   desyncing the rest of the sequence. This is unrelated to the specific hex *content* — it reproduces
   for **every** `set-member`/`set-field-op` case, not just the one the test happened to fail on first.
2. **`<digits>e<digits>` inside a hex run lexes as one `FLOAT` token.** `e`/`E` is itself a valid
   lowercase hex digit, and the shared lexer's number scanner greedily commits a trailing digit run
   after `e`/`E` to scientific-notation float parsing. Hex-encoding the literal string `"2.5e10"`
   byte-for-byte produces the hex run `322e35653130`, which lexes as one `Float` token, not alternating
   `INT`/`IDENT` — so `{INT | IDENT}*` doesn't recognize it at all (independent of issue 1; this is
   the one that produced the originally-reported failure message).

**Fix:** removed the `hex = {INT | IDENT}*`-shaped **production** entirely from all four affected files
(json mutations/diff, csv mutations/diff) and replaced it with a new framework-level **macro**
(`hex`, in `default_macros()`), referenced the same way as the pre-existing `edge`/`table`/`quantity`/
`props` macros — a bare `hex` ident with no matching production automatically falls back to macro
lookup (`Symbol::Ref`'s existing fallback, unchanged). `Recognizer::match_macro_span` — unlike
`Symbol::Star` — already tries the **largest token span first and shrinks until its predicate accepts**,
i.e. it already implements real backtracking; I only needed to (a) add the `hex` predicate
(`macro_hex_ok`: every non-whitespace byte in the joined span is an ASCII lowercase hex digit — the
whitespace filter strips the artificial `" "` `slice_source_text` inserts between adjacent tokens,
which isn't real source text), and (b) widen `match_macro_span`'s loop to also try a **zero-width**
match (`pos..=len` instead of `pos+1..=len`) so the empty-hex-value case still works — a strict
widening, since it only changes behavior for a matcher whose predicate accepts `""`, which none of the
four pre-existing macros' predicates do.

This correctly and automatically backtracks off `value`/`quoted` (none of `v`/`l`/`u`/`q`/`t`/`d` are
valid hex digits) with **no per-callsite grammar workaround** — every existing `hex` reference (both
the ones directly followed by an ambiguous keyword, and the ones already safely bounded by `"]"`/`":"`)
now goes through the same macro path uniformly.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` — Bug 1 fix (`lex()`
  comment-skip), Bug 3 fix (`hex` macro + `match_macro_span` zero-width widening), 1 new regression
  test (`hash_comment_hides_quote_and_pipe_characters_from_the_operator_prescan`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
  — Bug 2 fix (`{...}*` grouping), renamed the `string` production to `json-string` (see dialect note
  below), doc comments.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  — Bug 2 fix (`{...}*`), Bug 3 fix (removed `hex` production, now uses the macro), doc comments.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
  — Bug 3 fix (removed `hex` production, now uses the macro), doc comments.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  — Bug 3 fix (removed `hex` production, now uses the macro), doc comments.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
  — Bug 3 fix (removed `hex` production, now uses the macro), doc comments.

No `.protocol.semio` file needed a content change (Bug 1's comment-scanning fix was sufficient for all
three protocol files that were failing to *parse*; none had a Bug-2/3-shaped issue).

## Extra bug found while parse-fixing json's snapshot grammar (ARTIFACT, fixed, not framework)

After Bug 1 was fixed, json's snapshot grammar still failed to parse with `expected Ident, found
Equals` — because it defines a production literally named `string` (`string = TEXT`), and this
dialect's `parse_grammar` main loop (a single unified loop for header directives AND productions, no
separate "header phase") treats a leading `string`/`extension`/`use`/`start`/`comment` ident as
introducing a **header directive** wherever it appears in the file, not just in the header. Renamed
the production to `json-string` (csv's own grammars already avoid all five reserved words). This is
folded into the dialect-recipe note below.

## Note for `📖️grammar-recipe.md` (P2-PC pilot-closer wave)

Two narrow, real, repo-wide dialect constraints every future FG-wave author needs to know, neither of
which produces an obviously-related error message:

1. **`extension`/`use`/`start`/`comment`/`string` can never be a production name**, anywhere in a
   `.grammar.semio`/`.protocol.semio` file — not just in the header. `parse_grammar` uses one unified
   loop for header directives and productions; a leading ident matching one of those five words is
   *always* parsed as a header directive (e.g. `string = TEXT` triggers the `string <quote> <mode>`
   header-directive parser instead, failing with a confusing `expected Ident, found Equals`). Pick a
   different name (json's `string` → `json-string`).
2. **Never model an open-ended opaque/hex content field as a `{INT | IDENT}*`-shaped *production*** if
   it can be followed by a bareword keyword — `Symbol::Star` is a single greedy pass with no
   backtracking, so it silently swallows the next literal if that literal happens to tokenize as the
   same kind (`IDENT`) as the content it's matching (no parse error, just wrong recognition, or worse,
   a false negative that looks unrelated to the actual cause). Use the framework's built-in `hex`
   macro instead (bare `hex` with **no** matching production — `Symbol::Ref` already falls back
   production→macro): `Recognizer::match_macro_span` tries the largest span first and backtracks
   correctly, unlike a `Star` production. (As a bonus this also sidesteps the fact that a
   `<digits>e<digits>` hex run lexes as one `FLOAT` token, not alternating `INT`/`IDENT`.)

## What I deliberately did NOT do

Considered making `Symbol::Star`/`Symbol::Plus` properly backtracking at the framework level (the
"textbook correct" general fix for Bug 3's first half) but rejected it: doing so correctly requires
threading backtracking *across* `Symbol::Ref`/production boundaries (a `Ref` would need to return a
set of possible end positions, not one `Option<usize>`) — a much larger, invasive rewrite of the core
recognizer used by every grammar in the repo, with real risk of subtly changing matching results for
grammars that currently rely on greedy first-match semantics. The `hex`-macro fix achieves the same
practical outcome for this exact, real, recurring shape ("opaque token-kind-ambiguous content followed
by a keyword") with a small, additive, low-risk change instead.
