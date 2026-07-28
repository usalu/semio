# Wave 1 Part B — mathematical/graph/dsl unification — status

## Scope
Entirely within `mathematical/graph/dsl/rs/lib.rs` (+ Cargo.toml) and its JS twin
`mathematical/graph/dsl/core/js/index.ts`, plus one downstream test-fixture fix.

## Wire module (`pub mod wire`)
- Gutted internals to delegate lex/parse/print to `dsl_schema::parse_wire_text` /
  `dsl_schema::{print_shape, Shape::Wire, FieldValue::Wire, Writer, JoinMode}`.
- Kept public signatures unchanged: `WireNode`, `WireEdge`, `wire_literal_from_dag`,
  `dag_from_wire_literal`.
- Kept the "edge ports mandatory on both ends" rule as a validation layer on top of the unified
  parse (`GraphDslError::EdgeTargetMissingPort`), since the shared grammar itself leaves ports
  optional — that's engine business, not a syntax difference.
- New unified syntax observed end-to-end: `->`/`<-`/`--`, `{k=v}` double-quoted properties
  (was `{k: 'v'}`).
- Behavior change: port names are now `dsl_core` idents (must start with a letter/`_`, never a
  digit) — a port literally named `"3d"` is no longer lexable (renamed to `"d3"` in the crate's
  own test; **any real fixture elsewhere in the repo with a digit-leading wire port name will now
  fail to parse** — flagged for Wave 2 fixture regen owners to check).

## Jack (Cypher-subset query language)
- Replaced the private character-scanning lexer with `dsl_core::lex`, keeping Jack's own richer
  `Token`/`Parser`/AST (grammar-aware, case-insensitive keywords). Two genuinely Cypher-specific
  pieces stayed local (pre-scanned ahead of delegating): dual-quote strings (`dsl_core` only lexes
  `"..."`) and `!=` (`dsl_core` has no relational operators).
- **Pattern grammar changed**: `dsl_core` has no standalone `-` token (only `->`/`--`/`<-`), so a
  bracketed edge label's leading connector is now `--` or `<-`, never a bare dash:
  `(a)--[r:kind]->(b)` (was `(a)-[r:kind]->(b)`), `(a)<-[r:kind]--(b)`. Unlabeled edges use a
  single connector: `(a)->(b)` / `(a)<-(b)` / `(a)--(b)`. `<-` swaps which parsed node is
  structurally "left" so `PatternEdge.right` is always the forward-direction target (mirrors
  `dsl_schema`'s own wire `<-` normalization).
- `#` is now a comment-to-end-of-line (unified with the rest of the engine) instead of a parse
  error.
- Added `With`/`Unwind`/`Call` clauses to the AST + parser (new `Clause::With/Unwind/Call`,
  `UnwindClause`, `CallClause`). **Not wired into `execute()`** — returns
  `GraphDslError::UnsupportedClause`, marked with a `TODO(unify-architect)` comment, per the
  instruction that this is prep work for a later wave (compose's Architect unifying onto Jack,
  Wave 2 / P9).
- Removed `GraphDslError::NumberUtf8` (dead after delegating byte-scanning to `dsl_core`). Added
  `GraphDslError::Lex(#[from] dsl_core::TextError)` and `GraphDslError::UnsupportedClause`.

## Downstream fallout found and fixed
- `trinity/jack/core/rs/lib.rs`: `format_is_idempotent` test used the old bare-dash pattern
  syntax — that crate's own `format()` delegates to `mathematical_graph_dsl::format` (its
  `parse`/`run`/`Parser` are an **independent** reimplementation, unaffected). Fixed the one test
  fixture string. `trinity_jack`'s own duplicate lexer/parser was NOT touched (out of scope; it
  doesn't import anything from the unified engine either — a separate technology debt, flagged
  for a later ticket if the plan wants it unified too).

## JS twin (`mathematical/graph/dsl/core/js/index.ts`)
- `formatWirePropertyValue`/`formatWireProperties` now emit the unified syntax byte-for-byte
  matching the Rust printer: `{ key=value ... }` (sorted keys, space-padded braces/brackets, no
  colons/commas), double-quoted strings escaped via a TS port of `dsl_core::escape_text`
  (`\\ \" \n \r \t` + `\u{XXXX}`), instead of the old `{key: 'value', ...}`.
- `parseMatchReturn` (the file's only actual "Jack matcher") never parsed string literals to begin
  with (no WHERE-clause support), so there was no quote-style branch to update there.

## Test/verification commands run (all green)
- `cargo check -p mathematical_graph_dsl`
- `cargo test -p mathematical_graph_dsl` — 76 passed
- `cargo check -p trinity_jack -p trinity_jack_lsp -p neural_dag -p infinite_board_port_directed_dag`
- `cargo test -p trinity_jack -p trinity_jack_lsp -p neural_dag -p infinite_board_port_directed_dag` — 112 + 1 + 21 + 30 passed
- `cargo clippy -p mathematical_graph_dsl --lib` — clean
- `cd mathematical/graph/dsl/core && bun ./script.ts test` — 8 passed

## Consumer crate count note
Only 4 crates directly depend on `mathematical_graph_dsl` per
`grep -rln 'graph/dsl/rs"' --include=Cargo.toml .` (excluding `.claude/worktrees`):
`trinity/jack/core/rs`, `trinity/jack/lsp/rs`, `neural/dag/rs`,
`infinite/board/port/directed/dag/rs`. The prompt's "~8 downstream crates" estimate didn't match
grep; all 4 found were checked (compile + test green).
