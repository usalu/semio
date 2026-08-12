# Packet: puzzle + trinity `⚙️engine` elimination

Targets: `✏️s/🔌️plugins/🧩️puzzle` (3 engine dirs, ~9,553 LOC), `✏️s/🔌️plugins/🔱️trinity` (2 engine dirs, ~2,712 LOC).

Status: IN PROGRESS — trinity done by hand, puzzle delegated to 3 parallel sub-agents (one per artifact: ◻2d, 🧊️3d, 🖐️5d), each restricted to its own artifact+app subtree. Shared files (`📦️glue.rs`, plugin-root `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`) are being consolidated centrally after all 3 report back, to avoid concurrent-write conflicts.

This file will be filled in with the full per-directory table, assertion arithmetic, and compiler output once all sub-agents complete and central integration (glue.rs + plugin root + compile verification) is done.

## Trinity — completed by hand

### `♻️rewrite/…/⚙️engine` (526 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `RewriteRuleEngine` struct + impl | DELETED outright | Rule 1 — zero construction sites repo-wide (`grep -rn "RewriteRuleEngine" ✏️s/🔌️plugins/🔱️trinity` → only its own definition) |
| `Lhs`/`Rhs`/`Rule`/`ParameterKind`/`ParameterSpec`/`PatternJson`/`AssignmentJson`/`ApplyRuleResult`/`RuleQueryResult` + `apply_rule`/`apply_rule_json`/`build_rule_query`/`rule_query_json`/`parse_bindings_json`/helpers | `🧬️schema/🦀️component.rs` region `🔖️RuleApplication` | Rule 3 — pure helpers/types over the `jack::Graph` document type, no app/AppIo dependency |
| `#[cfg(test)] mod tests` (10 tests) | `🧬️schema/🦀️component.rs` region `🧪️RuleApplicationTests` (`mod rule_application_tests`) | Rule 8 — moved verbatim beside the code it tests |
| `pub mod io_registry { … }` | `🚪️io/🦀️component.rs` region `🚪️DerivedIoRegistry` | Rule 5 |

Call sites fixed: `🗿️artifacts/♻️rewrite/🦀️component.rs` (`declaration()`'s `.composers(...)` + the shim `io_registry`'s `use ... as v1;`), 4 app files under `🎛️apps/♻️rewrite/` (`🦀️component.rs`, `🪟️windows/🎛️parameters/🦀️component.rs`, `🎮️commands/📜️rule/🦀️component.rs`, `🌍️world/🦀️component.rs`) — 21 occurrences of `crate::artifacts::rewrite::engine::` → `crate::artifacts::rewrite::schema::`, verified none collided with the unrelated `TrinityBoardEngine`/`self.engine` canvas-engine field also present in `🌍️world/🦀️component.rs`. `📦️packages/🦀️rust/📦️glue.rs`: removed the `pub mod engine;` mapping and its `pub mod engine { pub use super::standards::v1::engine::*; }` shim; updated the stale doc comment naming the old `⚙️engine` location.

Assertion arithmetic: 10 `#[test]` fns / 35 `assert!`+`assert_eq!` calls in the original engine file (`git show HEAD:...⚙️engine/🦀️component.rs`) — 10/35 present in the new `🧬️schema/🦀️component.rs` location. Exact match.

### `🔌️jack/…/⚙️engine` (root file 134 lines + 4 kernel submodules 1,887 lines = 2,021 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `TrinityGraphEngine` struct + impl | DELETED outright | Rule 1 — zero construction sites |
| `empty_jack_document()` | `🧬️schema/🦀️component.rs` region `🔖️EmptyDocument` | Rule 3 |
| `#[cfg(test)] mod tests` (1 test) | `🧬️schema/🦀️component.rs` region `🧪️EmptyDocumentTests` | Rule 8 |
| `pub mod io_registry { … }` | `🚪️io/🦀️component.rs` region `🚪️DerivedIoRegistry` | Rule 5 |
| `🌳️ast/`, `🔤️lexer/`, `🧮️executor/`, `🗣️language-service/` (shared jack-query-language kernel, consumed at crate root as `crate::{ast,lexer,executor,language_service}` by both the jack app and rewrite's `apply_rule`) | physically `mv`'d to `🧬️schema/🌳️ast/`, `🧬️schema/🔤️lexer/`, `🧬️schema/🧮️executor/`, `🧬️schema/🗣️language-service/` (content unchanged — verified every internal reference already used crate-root-absolute paths, no `super::`) | Rule 3 — pure compute over the `jack::Graph` document type |

Call sites fixed: `🗿️artifacts/🔌️jack/🦀️component.rs` (`declaration()`'s `.composers(...)` + shim), `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (1 call site, `engine::empty_jack_document()` → `schema::empty_jack_document()`). `📦️glue.rs`: removed `pub mod ast/lexer/executor/language_service` old `#[path]`s → repointed at `🧬️schema/...`; removed jack's `pub mod engine;` grouping (5 nested mod entries) and its shim.

Assertion arithmetic: 1 `#[test]` fn / 2 `assert!` calls in the original root engine file — 1/2 present in the new location. Exact match. (The 4 kernel submodules carried their own pre-existing tests unchanged since they moved as whole files — not re-verified line-by-line since `mv` cannot alter content, confirmed via `diff` against git HEAD below.)

Content-identity check on the 4 moved kernel files (mv, no edits) — `diff` against `git show HEAD:<old path>` for each of `🌳️ast`, `🔤️lexer`, `🧮️executor`, `🗣️language-service`: **all 4 byte-identical**, zero diff lines.

Structural check (trinity only, ticket's own grep):
```
$ grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🔱️trinity   (excluding semio_s_plugin_stdio::artifacts::md::engine::* — that's stdio's own engine, a legitimate cross-plugin dependency, out of scope)
→ 0 matches
$ find ✏️s/🔌️plugins/🔱️trinity -path "*🗿️artifacts*" -name "⚙️engine" -type d
→ 0 matches
```
Both trinity structural checks are clean.

## Puzzle — delegated, pending sub-agent reports

- ◻2d: agent `add56f8c912203f04` — pending
- 🧊️3d: agent `a61bde46086c2c594` — pending
- 🖐️5d: agent `a5a65874afb42c94d` — pending

## Compiler verification

`semio-s-plugin-stdio` is RED right now due to another session's in-progress mesh-diff refactor (confirmed via `git log`/error text, not touched by this packet):
```
error[E0432]: unresolved imports `crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_material_base_color`, ...
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs:11:26
```
A second run mid-packet hit a harder upstream failure — a file physically missing (mid-edit by the other session):
```
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
```
Both `semio-s-plugin-trinity` and `semio-s-plugin-puzzle` depend transitively on `semio-s-plugin-stdio`, so `cargo check -p <pkg> --all-targets` cannot reach either plugin's own compilation while stdio is red — this is upstream per the ticket's own note ("stdio is currently RED and every plugin depends on it"). Will retry once stdio stabilizes; if it doesn't land in time, this packet reports compile status as "blocked upstream" with the evidence above, not a false pass/fail.
