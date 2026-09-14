# 📓️ `🔗️graphql` executor — Rust context port, schema fragments, resolvers, executor

Executor: Opus 5 (`graphql-executor`), resuming the job a rate limit interrupted.
Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql`.
Crate: `semio-framework-repo-graphql` (`📦️packages/🦀️rust/🦀️.rs`, 4 146 lines, parser + executor).

## 1. What was already in place when this session resumed

The interrupted session had landed the whole Rust half — `🎥️ContextPort`, `🖼️Sources`, the
per-aggregate `🏗️Schema*` fragments, `⚡️Execution`, the three resolver regions, `🧱️Executor`,
`📜️Sdl` and `🩻️RecordingContext` — plus the Rust adapters of the three executor cases and the
TypeScript oracle adapters of `▶️query-execution` and `📜️sdl-dump`. Its last note ("now the imports
and Cargo.toml") understated the state: the crate already built and its 12 unit tests already
passed. This session therefore did NOT rewrite the executor; it finished the harness half, found
and fixed the defects the harness exposed, added the missing `❌️execution-errors` case, and
established honestly what the Go side can and cannot do.

## 2. Public API the `⌨️cli` crate consumes

```rust
use semio_framework_repo_graphql::{
    Executor, RepoContext, RecordingContext, ContextError, ExecutionError,
    Schema, build_schema, render_sdl, schema_inventory,
    parse, validate, operation_type, coerce_arguments, Document, Selection, Value, ParseError,
    serde_json,                 // re-export, so no client needs its own serde_json version
};

let context: &dyn RepoContext = /* default context over the domain crates, or a fixture double */;
let executor = Executor::new(context);

executor.schema()                                   -> &Schema
executor.execute(query, &variables)                 -> Result<Json, ExecutionError>
executor.execute_json(query, &variables)            -> Result<String, ExecutionError>  // pretty, CLI shape
executor.validate_query(query)                      -> Result<(), ParseError>
executor.operation_type(query)                      -> Result<String, ParseError>      // "query" | "mutation"

render_sdl(&build_schema())                         -> String   // the committed 🧬️schema/🔣️schema.graphql
schema_inventory(&build_schema())                   -> Json     // types/fields/args/members, for the sdl-dump case
```

`variables` is `serde_json::Map<String, Value>`; `Json` is `serde_json::Value`. `ExecutionError`
carries one `message` field, already wrapped in the `graphql errors: [...]` envelope the CLI prints,
so the CLI verb is `println!("{}", executor.execute_json(...)?)` with the error printed verbatim.

`RepoContext` is the port (49 methods: `root_dir`, `technologies`, `bundles`, `checkpoints`,
`folders`, `files`, `sections`, `definitions`, `contributors`, `goals`, `tickets`, `policies`,
`drafts`, `todos`, `statutes`, `interactions`, `analyze`, `fix`, and the 31 mutations). Every method
returns domain types from `semio-framework-repo-model` — no GraphQL type leaks into it, so the CLI
can supply a filesystem-backed context without depending on this crate's schema vocabulary.
`RecordingContext::from_json` / `from_text` is the fixture-backed double, with `events()` and
`snapshot()` for a mutation case to assert the trail and the resulting record set.

## 3. Defects found and fixed in this session

1. **`RefCell` double borrow in `RecordingContext::extract`** (real panic, `RefCell already
   borrowed`, aborting the whole `✏️mutation-execution` host with exit 101). `match
   self.records.borrow()....cloned() { … None => self.file_create(target) }` keeps the `Ref` alive
   for the whole `match`, and `file_create` takes `borrow_mut`. Fixed by binding the lookup to a
   `let` before the `match`. This was only reachable once the script got past the two fixture
   defects below, which is why the interrupted session never saw it.
2. **`✏️mutation-execution` fixture used a slug the executor does not derive.** `ticketOpen(title:
   "Port The Resolvers")` derives `PORT-THE-RESOLVERS`; the later `ticketChange`/`ticketClose` named
   `PORTTHERESOLVERS`, so the script died at `ticket-change` with
   `ticketChange: ticket 2026/9/6/PORTTHERESOLVERS not found`. Fixture corrected.
3. **`✏️mutation-execution` fixture selected a field the schema does not carry.**
   `interactions { kind summary }` — `Interaction` in the committed SDL has no `summary` (the domain
   struct does; the schema deliberately does not). Changed to `interactions { kind prompt author }`.
4. **`❌️execution-errors` case did not exist** although `🔮️oracle/🔣️.json` already carried its
   recorded no-oracle decision `graphql-owned-executor-diagnostics` and its capability
   `graphql-execution-errors`. Written this session (see §4).

## 4. New case `🧪️tests/❌️execution-errors`

`🥒️.feature` (`@capability-graphql-execution-errors`,
`@no-oracle-graphql-owned-executor-diagnostics`, `@comparison-ordered-json-v1`), `🦀️.rs` and
`🧫️fixtures/🔣️refusals.json` with 11 specification vectors, each an input paired with the verbatim
message it owes. The adapter asserts the message itself and fails the scenario on any drift, so a
reworded diagnostic is a red test rather than merely an unequal projection. Two scenarios:
`@mode-error` (`every-refusal-carries-its-verbatim-message`, `@level-fundamental`) and
`@mode-conformance` (`a-refusal-writes-no-event-and-changes-no-record`, `@level-quick`) — both modes
the decision's `specification-vectors` substitute discharges, so the case is legitimate with one
implementation.

Messages pinned (all verbatim from the running executor, not from reading the source):

| vector | message |
| --- | --- |
| `unknown-root-field` | `graphql errors: [unknown field "nope" on Query]` |
| `unknown-nested-field` | `graphql errors: [tickets: unknown field "nope" on Ticket]` |
| `unknown-field-two-levels-down` | `graphql errors: [tickets: dates: unknown field "nope" on TicketDate]` |
| `mutation-field-selected-as-a-query` | `graphql errors: [unknown field "syncManagement" on Query]` |
| `unknown-mutation-field` | `graphql errors: [unknown field "nope" on Mutation]` |
| `malformed-document` | `graphql errors: [unterminated selection set]` |
| `unresolvable-node-id` | `graphql errors: [node: invalid node id format: garbage]` |
| `close-a-ticket-no-record-carries` | `graphql errors: [ticketClose: ticket 2026/9/6/NO-SUCH-TICKET not found]` |
| `change-a-goal-no-record-carries` | `graphql errors: [goalChange: goal no-such-goal not found]` |
| `move-a-folder-no-record-carries` | `graphql errors: [folderMove: folder nowhere not found]` |
| `reopen-a-ticket-no-record-carries` | `graphql errors: [ticketReopen: ticket 1999/1/1/NOPE not found]` |

Two of these were WRONG in my first draft and were corrected by what the executor actually said
(`TicketDates` → `TicketDate`, `unexpected end of input` → `unterminated selection set`) — recorded
here because the messages in this table are measured, not assumed.

## 5. ❌️ The Go half of the executor cases cannot run — honest finding

**The Go `github.com/usalu/semio/repo/graphql` package accepts a `model.RepoContext` and then
ignores it for most of the schema.** I wrote the complete Go adapter for `▶️query-execution`,
including a full 49-method fixture-backed `model.RepoContext` implementation mirroring the Rust
`RecordingContext` semantics. It compiles, links against the split Go packages and runs. It is
parked at `$TICKET/🔗️graphql-go-query-execution-adapter.go.txt` (drop it back as
`🧪️tests/▶️query-execution/🐹️.go` once the Go side honours the port). Real measured result:

```
[test] level=fundamental cases=1 executed=3 passed=3 failed=0 errored=0 parity=1/3
[test] parity failed: …🔗️graphql::▶️query-execution::corpus-executes-identically::go::subject (24071 differences)
[test] cross-subject parity failed: …::corpus-executes-identically::go~rust (24071 differences)
```

The cause is not the adapter. In `📦️packages/🐹️go/🐹️.go` the query resolvers test `if r.Ctx != nil`
and then call package-level streamers that read the real filesystem instead of the context:

```go
func (r *queryResolver) Tickets(ctx context.Context, …) ([]*model.Ticket, error) {
    if r.Ctx != nil {
        opts := filter.ToStreamOptions()
        ticketChan := make(chan model.Ticket)
        go ticketspkg.StreamTickets(ctx, year, month, day, ticketChan, opts)   // ← reads C:\git\semio
```

`Bundles` and `Contributors` are written the same way; `Technologies` derives from `Bundles`.
Observed consequences on the frozen 3-ticket fixture: `tickets` returned **3 365** rows — this
repository's own ticket folders, with `path` values like
`C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️25\…`; `contributors` returned the five real contributors
of this repository instead of the fixture's two; `technologies` and `bundles` returned **0** rows;
`folder(path: "repo").children` returned `assets, client, lib, server` (real directories) instead of
the fixture's `go, rust`; `file.sections` / `file.definitions` returned 0; section and definition
ids were recomputed by the identity builders (`🗃️repo🗃️go💻main🔖header`) instead of read off the
record (`section:repo/go/main.go#Header`).

The Go package is owned by the concurrent `go-split` executor and my brief forbids editing it, so I
did not. **This is a real gap, not a passing case**: `▶️query-execution`, `✏️mutation-execution`,
`❌️execution-errors` and `📜️sdl-dump` currently run with the Rust subject only (the first and last
against the `graphql-js` oracle, the other two against recorded no-oracle decisions). For
`📜️sdl-dump` there is a second, independent Go blocker: `buildSchema` is unexported and
`Executor.schema` has no accessor, so no external adapter can obtain the Go schema to inventory it
at all.

**What the Go side needs (for the `go-split`/`cli` wave, not done here):**
1. Route `Tickets`, `Bundles`, `Contributors`, `Technologies`, `Folders`/`children`, `Sections`,
   `Definitions` and the node-id builders through `r.Ctx` instead of the package-level streamers and
   identity builders.
2. Export the built schema (`func (e *Executor) Schema() Schema`) so an adapter can inventory it.
3. Then restore the parked adapter and re-run `parity fundamental --owner 🔗️graphql`.

## 6. Verification — real command output

Environment: `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`.

```
$ cargo build -p semio-framework-repo-graphql
   Compiling semio-framework-repo-graphql v0.1.0 (…\🔗️graphql\📦️packages\🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 7.74s

$ cargo test -p semio-framework-repo-graphql
running 12 tests
test tests::reports_the_operation_type ... ok
test tests::applies_defaults_only_to_absent_arguments ... ok
test tests::reproduces_the_reference_diagnostics ... ok
test tests::projects_the_canonical_shape ... ok
test tests::round_trips_the_ast_through_json ... ok
test tests::reports_unknown_fields_and_unresolvable_ids ... ok
test tests::records_every_mutation_it_applies ... ok
test tests::resolves_a_statute_through_a_breach ... ok
test tests::derives_the_ticket_dates_and_identity ... ok
test tests::projects_scalars_lists_and_enums ... ok
test tests::serves_the_committed_sdl ... ok
test tests::honours_aliases_typename_and_arguments ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo clippy -p semio-framework-repo-graphql --all-targets` finishes with 2 distinct warnings, both
inside the LEXER region owned by the `graphql-parser` executor and both explicitly out of my scope
(`should_implement_trait` on `Lexer::next`, `manual_range_patterns` at `🦀️.rs:236`). Nothing in the
executor, context, schema-fragment, resolver, SDL or recording-context regions warns.

```
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner 🔗️graphql
[test] level=fundamental cases=8 executed=17 passed=17 failed=0 errored=0 parity=12/12

$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner 🔗️graphql
[test] level=quick cases=8 executed=27 passed=27 failed=0 errored=0 parity=19/19
```

Per-result breakdown of the fundamental run (`📤️results.jsonl`), all `passed`:

| case | scenario | role | impl |
| --- | --- | --- | --- |
| `📃️document-parsing` | corpus-projects-identically | oracle / subject / subject | typescript / rust / go |
| `🚫️syntax-errors` | malformed-inputs-are-rejected | oracle / subject / subject | typescript / rust / go |
| `🔀️variable-coercion` | arguments-resolve-against-variables | oracle / subject / subject | typescript / rust / go |
| `🙅️unsupported-syntax` | subset-boundary-is-identical | subject / subject | rust / go |
| `▶️query-execution` | corpus-executes-identically | oracle / subject | typescript / rust |
| `📜️sdl-dump` | served-schema-matches-the-committed-sdl | oracle / subject | typescript / rust |
| `✏️mutation-execution` | script-changes-records-and-emits-events | subject | rust |
| `❌️execution-errors` | every-refusal-carries-its-verbatim-message | subject | rust |

`contract` over the whole tree reports **zero** breaches whose scope or text mentions `graphql`
(verified by filtering `⚡️cache/breaches/testing.json`); the breaches it does report belong to
`✏️s/🔌️plugins/**` and the repo-wide discovery baseline, which are other owners' territory.

## 7. launch.json

Five entries appended to `.vscode/🧩️launch.seed.jsonc` in group `4_gate`, following the existing
`⚖️gate<owner><subject>` naming and continuing the order sequence after `⚖️gate🎫️tickets🐙️issue-sync`
(426.5): `⚖️gate🔗️graphql🦀️rust🧪️test` (426.6), `⚖️gate🔗️graphql▶️query-execution` (426.7),
`⚖️gate🔗️graphql✏️mutation-execution` (426.8), `⚖️gate🔗️graphql❌️execution-errors` (426.9),
`⚖️gate🔗️graphql📜️sdl-dump` (427.1). Regenerated:

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

`grep -c "gate🔗️graphql" .vscode/launch.json` → `5`.

## 8. Left for later waves

- **Go context port** (§5) — the single blocker for Go subjects on all four executor cases. Needs an
  owner of `📦️packages/🐹️go`.
- Test-case directories of this owner already carry the leading emoji the 06:10 coordinator decision
  requires (`▶️`, `✏️`, `❌️`, `📜️`, `📃️`, `🚫️`, `🔀️`, `🙅️`), so the audit wave's rename sweep has
  nothing to do here.
- The two lexer clippy warnings belong to the `graphql-parser` executor.
- `RecordingContext` is a test double. The default filesystem-backed `RepoContext` for the Rust CLI
  is the `⌨️cli` executor's job; it only needs `Executor::new(&context)` as in §2.
