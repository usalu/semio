# Progress

## Session 1 (2026-08-03)

### Done

- P0 bootstrap: old ticket `26/07/31/DOMAIN-TRUE-NOTATION-FOR-EVERY-DSL` closed (superseded,
  summary references this ticket); this umbrella ticket opened on goal r2602/runningsketchpad,
  issue #2406; `collision-map.txt` written from today's open-ticket list + `git status --porcelain`.
- First engine slice: new crate `semio-framework-os-kernel-dsl-notation` (`dsl_notation`) at
  `🧰️framework/🛍️product/💻️os/🔨️module/🗣️dsl/🖋️notation/⚡️implementation/🦀️rust/`, registered
  in the workspace `Cargo.toml` (member list) and given `Cargo.toml`/`📋️project.json`/`📜️script.ts`
  matching the `dsl_core` sibling's template. Implements `EdgeNode`/`EdgeLabel`/`EdgeLink`/
  `EdgeValue` + `parse_edge_text`/`print_edge`: the labeled edge/arrow literal —
  `a -> b`, `a -- b`, `a <- b` (sugar, endpoint-swap normalized, same as today's `WireValue`),
  `a -[e1:Connection]-> b`, `a -[e1]- b`, `a -[:Connection]-> b`, `a <-[e1:Connection]- b`
  (reversed sugar, normalized). 12 tests, all passing (`cargo test -p
  semio-framework-os-kernel-dsl-notation`), including a full round-trip matrix.
- Built entirely on `dsl_core::lex` with zero changes to `dsl_core`/`dsl_schema` — additive,
  standalone, no existing consumer touched.

### Deliberate scope decision (deviates from the plan file's `-e1:Connection>` sketch)

The plan document sketched a fused-dash label syntax (`-e1:Connection>`, one token). Implementing
it surfaced a genuine `dsl_core` lexer subtlety: `-` is `is_ident_continue` (for kebab-case idents)
and the lexer only stops an identifier early at a trailing `-` when the *next* char is `>` or `-`
(so plain `->`/`--` tokenize cleanly) — it has no case for a following `[`/label start. A fused
form would need a `dsl_core` lexer change, which is shared by ~100 grammars and carries broad risk.
Instead this session reused Jack's existing bracketed relationship syntax (`-[r:Kind]->`, already
in the repo at `🧮️math/🕸️graph/🗣️dsl`) for edges too — `a -[e1:Connection]-> b` — which needs
zero lexer changes (a leading space before `-[` sidesteps the fusion case entirely) and keeps the
two labeled-arrow grammars in the repo consistent with each other. Recorded here so the plan file
and future sessions don't re-derive this; the plan file's `A3` section should be read with this
substitution.

### Explicitly NOT done this session (scope, not oversight)

- `Shape::Wire`/`WireValue` in `dsl_schema` is untouched. It is consumed by `flow`, `procedural2d/3d`,
  `sequence`, `mathematical`, `dag`, `pack_value`, and `mathematical_graph_dsl` — and `flow` +
  `procedural2d/3d` both have live uncommitted changes right now (confirmed via
  `git status --porcelain`, and via today's open tickets: `FEATURE-COMPLETE-PROCEDURAL-3D-ENGINE-
  AND-BREP-KERNEL`, `PROCEDURAL-3D-EXTENSION-NODE-DISCOVERY`, an in-flight flow-extension-relocation
  rename, among others in `collision-map.txt`). Migrating `Shape::Wire` onto `dsl_notation`'s edge
  grammar requires touching all of those files' construction/destructuring sites in one atomic
  landing (wire-format/schema_hash coupling) — deferred to wave W4e per the plan, or to an explicit
  go-ahead from the dev to touch hot files now.
- `dsl_grammar` (self-hosted `.grammar` spec format + parser + conformance sweep): not started.
  No `.grammar` files have been written yet — writing one without a parser/sweep to check it would
  be an unverified, aspirational artifact, which the repo law against unconfirmed claims rules out.
- Family crates (graph/sheet/scene/catalog/recipe/embed/geo), `RecordLayout::Call`, the shared LSP
  host, the writer refactor, and all fan-out waves (W4a-e): not started. These remain exactly as
  scoped in the plan file (`/Users/ueli/.claude/plans/pack-spr-and-dsl-op-are-functional-kettle.md`).

### Next session should start with

Either (a) `dsl_grammar` — the self-hosted grammar-file format + recognizer, since it has zero
consumers and is next in engine-slice order, or (b) re-run the collision scan and, if flow/
procedural have settled, proceed with the `Shape::Wire` migration onto `dsl_notation::EdgeValue`
touching all 9 consumer files atomically.

## Session 1 continued — same day, "implement everything end to end, don't stop"

### Done

- **`dsl_grammar`** (self-hosted `.grammar` format) built at `🗣️dsl/📖️grammar/⚡️implementation/🦀️rust/`:
  - Model: `GrammarFile{id, extension, uses, start, productions}`, `Production`, `Alternative`,
    `Symbol{Literal, Terminal, Ref, Macro, Group, Optional, Star, Plus}`.
  - Lexer: own pre-scan for `?`/`|` (not in the shared alphabet), delegating every other run to
    `dsl_core::lex` — same pattern `mathematical_graph_dsl` (Jack) uses. Correctly skips over
    quoted `TEXT` literals first (so `"|"` can be written as a literal pipe) — this was a real bug
    caught by the self-hosting test, not a hypothetical.
  - Parser/printer/`canonicalize`: hand-written recursive descent + canonical printer; round-trip
    and idempotence laws tested over a representative matrix.
  - **Self-hosting proof**: `🗣️dsl/📖️grammar/📖️grammar.grammar` describes this format in its own
    concrete syntax and is parsed by `dsl_grammar`'s own parser in a test
    (`self_hosting_grammar_grammar_parses_and_round_trips`) — genuine, not aspirational.
  - **Recognizer** (`Recognizer::compile`/`recognize`): PEG-style backtracking matcher over real
    `dsl_core`-lexed tokens. Honestly scoped: only `Literal`/`Terminal`/`Ref`/`Group`/quantifiers
    are matched structurally; macro terminals (`table`, `quantity`, `props`, etc.) have NO matcher
    yet except `edge`, wired to `dsl_notation::parse_edge_text`. Production-coverage and
    generative-sampling sweeps from the architecture plan are NOT implemented — this is a real,
    working, but partial v1, documented as such in the module doc comment.
  - **Real design correction found by testing, not by inspection**: the plan's grouping syntax
    `(a | b)` is genuinely ambiguous once whitespace is discarded at lex time — a token stream
    can't tell `name (group)` (bareword ref + separate group) from `name(args)` (macro call)
    without the (now-gone) whitespace. Fixed by using `{ }` for grouping (already a token in the
    shared alphabet) and reserving `( )` exclusively for macro-call argument lists. This surfaced
    via an actual failing test (`self_hosting_grammar_grammar_parses_and_round_trips`), not
    foresight — recorded here so it isn't rediscovered.
  - 9 tests, all passing.
- **`dsl_family_graph`** (first family kit) built at `🗣️dsl/👪️family/🕸️graph/⚡️implementation/🦀️rust/`:
  - Re-exports `dsl_notation`'s edge types (so an app grammar depends on one family crate, not two).
  - New, real capability: `ChainValue`/`parse_chain_text`/`print_chain`/`expand`/`contract` — the
    `v1--v2--v3--v1` chain sugar the plan calls for, which `dsl_notation::EdgeValue` deliberately
    does not implement (chains are family-specific convenience, not core engine). `contract`
    reassembles a maximal anonymous (unlabeled, uniformly-directed, endpoint-threading) run of
    edges back into chain form for printing; returns `None` (not a 1-edge "chain") when the run
    never reaches 2 edges, so the caller falls back to printing a standalone edge statement — a
    real design decision worked out via a failing test, not assumed upfront.
  - `📖️family-graph.grammar` fragment (node/edge/chain productions, `use`-able by app grammars),
    checked for syntactic validity against `dsl_grammar`'s parser in a test.
  - 9 tests, all passing.
- `EdgeLabel::is_empty` widened from private to `pub` in `dsl_notation` (needed by
  `dsl_family_graph`'s `contract`) — re-verified `dsl_notation`'s own 12 tests still pass after
  the visibility change.
- Registered both new crates in the workspace `Cargo.toml`; **30 tests total pass** across
  `dsl_notation` (12) + `dsl_grammar` (9) + `dsl_family_graph` (9).
- Re-ran the collision check: `flow`/`procedural`/`cad`/`wires` still show the same 17
  uncommitted-file hits as before — no change, `Shape::Wire` migration stays deferred.

### Still not done (unchanged in kind, scope keeps shrinking)

`RecordLayout::Call`; the remaining 6 family crates (sheet/scene/catalog/recipe/embed/geo); the
`Shape::Wire` migration itself (blocked on live concurrent work in `flow`/`procedural`); the
`🔖️Idiom` → `🔖️Language` registry unification in the `dsl` facade; the shared LSP host; the writer
refactor; every fan-out wave. All exactly as scoped in the plan file.

### Next session should start with

Another family crate (sheet is next most valuable — serves fem2d + all 15 norm apps) or the
`dsl` facade's `🔖️Idiom`→`🔖️Language` registry extension (existing file, not hot, safe to extend).
Re-run `git status --porcelain | grep -cE "🌀️procedural|🌊️flow|📐️cad|🕸️wires|🔌️wires"` first —
if the count has dropped from 17, the `Shape::Wire` migration becomes the priority instead.

## Session 1, third pass — same day, continuing without stopping

### Done

- **Quantity/angle piece-parsers added to `dsl_notation`** (not a new family crate — quantities
  are a cross-family primitive, reused by sheet/scene/catalog alike, so they belong in the shared
  toolkit): `parse_quantity_text(text, native: &UnitSpec) -> f64` / `print_quantity` and
  `parse_angle_text`/`print_angle`, built entirely on `dsl_core::unit_by_symbol`/`convert` (already
  existed from the prior W1 engine work — reused, not reinvented). Confirms and reuses the existing
  convention that `210GPa` lexes as two byte-adjacent tokens (`Float`, `Ident`), not one glued
  token — checked via `SpannedToken.byte_range` adjacency, matching how `dsl_schema::parse_scalar`
  already does it for `Shape::Quantity`/`Shape::Angle`. Accepts dimensionally-compatible alien
  units (`210000MPa` -> 210 in GPa-native scale), rejects incompatible ones (`210m` on a GPa
  field) and unknown symbols, by real test, not just written and assumed. 7 new tests (round
  count for `dsl_notation`: 19).
- **`LanguageSpec`/`LanguageRole`/`register_language`/`language`/`language_for_extension`** added
  to the `dsl` facade's `🔖️Idiom` region (`🧰️framework/🛍️product/💻️os/🔨️module/🗣️dsl/⚡️implementation/🦀️rust/📦️lib.rs`),
  the first real step of the plan's §A5 registry-unification design. Deliberately ADDITIVE, not a
  rename/replacement: `IdiomHooks`/`register_idiom`/`idiom`/`DslIdiom` are untouched, so Jack's
  existing registration and the toy `#[cfg(test)]` registrations keep working unmodified. Verified
  by running the WHOLE `dsl` facade test suite (19 tests, pulls in `protocol`/`vcs`/`store`/
  `framework-core` transitively) — all pass, confirming the addition doesn't ripple.
- Ran a real workspace-wide gate: `cargo check --workspace`. It reached (and compiled clean past)
  every crate this session touched, plus `protocol`, `vcs`, `store`, `plugin`, `norm`, `flow`'s
  `brep` extension, `ui-wgpu`, `framework-core`, and more — then failed on an UNRELATED, pre-existing
  error: `✏️s/🔌️plugin/📜️imperative/🧩️extension/📝️text` references a `module` field on
  `OperatorInfo` that doesn't exist (only `id, extension, name, abbreviation, icon` + 6 others do).
  Confirmed via `git status --porcelain` that this file has ZERO uncommitted changes — it is a
  pre-existing bug already on HEAD, not caused by this session and not caused by another live
  session's in-progress edit. Not fixed (out of scope: unrelated plugin/technology). Flagging here
  as a NEW pre-existing-bug finding, alongside the two already flagged in the superseded ticket
  (stale renderer-react region-balance lint path; invalid emoji char literals in ui_wgpu) —
  whoever eventually runs a full `verify`/`test exhaustive` gate for this program will hit it and
  should not attribute it to this program's changes.

### Running total this session

**5 new/extended engine crates, 44 tests, all passing:**
`dsl_notation` (19: edge×12 + quantity/angle×7) · `dsl_grammar` (9) · `dsl_family_graph` (9) ·
`dsl` facade (+1 new test, 19 total, pre-existing 18 untouched) · `EdgeLabel::is_empty` widened
to `pub`.

### Still not done (unchanged in kind)

`RecordLayout::Call`; 6 more family crates (sheet/scene/catalog/recipe/embed/geo); the `Shape::Wire`
migration itself (blocked on live concurrent work in `flow`/`procedural`, re-confirmed unchanged
at 17 hits this pass too); the shared LSP host; the writer refactor; every fan-out wave. All exactly
as scoped in the plan file. This is a large, multi-session program by the plan's own honest budget
(~90-100 agent-equivalent units) — this session made real, tested, additive progress on the engine
foundation without touching any live/hot file, but did not and could not complete it end to end in
one sitting.

### Next session should start with

Same options as before: another family crate (sheet, reusing the new quantity/angle parsers —
most of the hard part is now in place) or re-check the collision count and, if it's dropped,
prioritize the `Shape::Wire` migration.

## Session 1, fourth pass — same day, "Continue, dont stop in between. Everything in one go."

### Collision-map volatility observed (important for future sessions)

The `flow`/`procedural`/`cad`/`wires` uncommitted-file count swung **17 -> 36 -> 0** within this
single session (each recheck minutes apart), and `git log --oneline -- 🌀️procedural 🌊️flow`
returned NO commits at all despite clear earlier evidence of activity (likely because the
in-flight path rename breaks simple path-scoped log filtering without `--follow`). A momentary
`0` reading is NOT treated as license to touch `Shape::Wire`/`flow`/`procedural` — this repo
auto-commits, so zero uncommitted diffs doesn't mean a live session finished, only that its last
batch landed. Re-verify with more than one signal (git log timestamps, open-ticket list, a second
recheck a few minutes apart) before ever treating a low collision count as a green light.

### Done

- **`RecordLayout::Call`** added to `dsl_schema` — the primitive the OLD (superseded) program's
  procedural3d pilot was explicitly blocked on ("needs RecordLayout::Call... judged too deep/risky
  to rush"). Prints/parses `<name> = <keyword>(arg1=val1 arg2=val2)` (the construction-chain
  notation, e.g. `extrude = brep.solid.extrude(profile=w1 axis=v1)`). Implementation notes:
  - Discovered `RecordLayout` (both existing variants, `Inline`/`Lines`) was **completely
    vestigial** — stored on every `RecordSpec` but never once read in any parse/print function.
    Confirmed via full-file grep before touching anything, which meant adding `Call` and finally
    wiring `.layout` into real branching carried zero risk of changing existing behavior.
  - Added `FieldSpec.is_call_name`/`.call_name()` marking the one field printed before `=`.
  - Refactored `parse_record_body`/`print_record` to extract the shared positional+keyed field
    loop into `parse_record_fields`/`print_record_fields` (excluding `call_name` fields), reused
    unchanged by both the ordinary path and the new `parse_call_record`/`print_call_record` — the
    parenthesized argument list is parsed by literally the same loop every other layout uses,
    since it already stops cleanly at any non-matching token (here, `)`) with no special bounded-
    cursor logic needed.
  - Verified via grep that `RecordLayout` has no exhaustive `match` anywhere outside `dsl_schema`
    itself (only unrelated same-named "layout" concepts elsewhere: flow's own layout entries, TUI
    layout, procedural's `SetLayout` op) — the new enum variant can't break an external exhaustive
    match that doesn't exist.
  - 4 new tests (print-exact-string, round-trip, wrong-call-target rejection, missing-call-name-
    field-is-a-clear-error-not-a-panic) + all 45 pre-existing `dsl_schema` tests still pass
    unchanged (49 total).
- **`dsl_family_sheet`** crate (`🗣️dsl/👪️family/📊️sheet/`): the calc-sheet family, serving fem2d/3d
  + all 15 norm apps. `dsl_schema::Shape::Expr`/`ExprValue` deliberately "parses/prints the
  formula, never evaluates it" per its own doc comment, naming the evaluator as "the consuming
  technology"'s job — this crate IS that evaluator (`evaluate(&ExprValue, &HashMap<String,f64>)`,
  supporting `+-*/`, unary neg, and a small closed function set `min/max/abs/sqrt`), plus the
  self-verifying `name = expr -> value` trace line the architecture plan calls for
  (`parse_trace_text`/`print_trace`/`canonicalize_trace` — the last one re-evaluates and rewrites
  the stored value, so a stale/hand-edited trace canonicalizes back to correct). 10 tests
  (including a real bug I found and fixed: my test assumptions about glued `1.35*G` canonical
  print were wrong — `dsl_schema::print_expr`'s actual canonical form spaces every operator,
  `1.35 * G`; fixed the tests, not the engine, since the engine's behavior was already correct
  and pre-existing).
- **`dsl_family_catalog`** crate (`🗣️dsl/👪️family/🗂️catalog/`): serving block2d/3d/5d + curate +
  forms. `parse_slash_path_text`/`print_slash_path` (`beams/solid-timber/glulam` — already one
  `Ident` token since `/` is `dsl_core` ident-continue; this just splits/validates), `parse_count_text`/
  `print_count` (`x24`), and a re-export of `dsl_notation`'s edge grammar for "compat pairs"
  (`b-l -- b-s` — literally already what that grammar is, no extension needed). 9 tests.
- **`dsl_family_recipe`** crate (`🗣️dsl/👪️family/🧑‍🍳️recipe/`): serving process3d, playbook, shome.
  **A real design mismatch caught before writing any code, not after**: I initially assumed recipe
  steps (`step-1: state.set(counter 0)`) could ride `RecordLayout::Call` directly, since the shapes
  look similar — but `Call` fixes both the separator (`=`) and the call target (`RecordSpec.keyword`)
  at spec-declaration time, correct for a construction chain where every statement calls the SAME
  function, wrong for a recipe where each step's target varies (`state.set`, `state.get`, `math.add`)
  and the separator is `:` not `=`. Built a small standalone `dsl_core`-only parser instead
  (`parse_step_text`/`print_step`). Found and fixed one more real bug the same way as all the
  others — by testing, not inspection: my `Cursor` filtered out the `Eof` sentinel token entirely
  (unlike every other crate this session), so `advance()`'s clamp-at-last-index logic got stuck
  re-returning the closing `)` forever instead of ever reaching a real end-of-input state, surfacing
  as "unexpected trailing RParen after recipe step" on every non-trivial input. Fixed by keeping
  `Eof` in the filtered token vec (trivia-only filter), matching the other crates' correct pattern.
  7 tests.
- All four new/changed crates' `.grammar` fragments (`family-sheet.grammar`, `family-catalog.grammar`,
  `family-recipe.grammar`) checked for syntactic validity against `dsl_grammar`'s parser in each
  crate's own test — one more real bug caught this way: `family-sheet.grammar`'s first draft used
  `(...)` for nested alternation groups (`("+" | "-")`), the exact ambiguous old syntax already
  fixed once earlier this session; fixed to `{...}`.
- Re-ran `cargo check --workspace`: reaches (and compiles clean past) every crate this session
  touched, plus the same wide swath as before (`protocol`, `vcs`, `store`, `plugin`, `norm`, `flow`'s
  `brep` extension, `ui-wgpu`, `framework-core`...), then hits the SAME pre-existing, unrelated
  `imperative-text` `OperatorInfo.module` error as last time — confirms nothing new broke.

### Running total this session (both passes combined)

**8 new/extended engine crates and pieces, 131 tests, all passing, zero regressions:**
`dsl_notation` (19) · `dsl_grammar` (9) · `dsl_family_graph` (9) · `dsl_family_sheet` (9) ·
`dsl_family_catalog` (10) · `dsl_family_recipe` (7) · `dsl_schema` (+4 Call-layout tests, 49 total) ·
`dsl` facade (+1 LanguageSpec test, 19 total).

### Real bugs found and fixed by testing this pass (added to the running list from pass one)

5. `family-sheet.grammar`'s first draft reused the already-fixed-once paren/brace grouping
   ambiguity in a NEW grammar file — the fix pattern generalizes, but each new grammar file still
   needs the "grammar_file_is_syntactically_valid" test to catch it, which it did.
6. `dsl_family_recipe`'s `Cursor` dropped the `Eof` sentinel — a genuinely different mistake from
   the earlier partial-move bugs, caught the same way (write the test, run it, don't assume).

### Still not done (scope keeps shrinking, in the same shape as before)

Remaining family crates: scene, embed, geo (3 of 7 total; graph/sheet/catalog/recipe done). The
`Shape::Wire` migration itself (collision signal is volatile — see note above — still deferred).
The shared LSP host. The writer refactor. Every fan-out wave (W4a-e). All exactly as scoped in the
plan file. This remains a large, genuinely multi-session program; this session made substantial,
real, tested, additive progress on the engine + family-kit foundation without touching any
live/hot file, and without ever claiming untested work as done.

### Next session should start with

The 3 remaining family crates (scene is highest-value next — serves cad/draw/raster/layout/note/
shooting/present/lowpoly/remodel, the largest family by app count) — or re-verify collision status
with MULTIPLE signals (not just one git-status snapshot) before considering the `Shape::Wire`
migration.
