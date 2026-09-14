# 📓️ Opus executor — schema cleanup: the `fix` mutation, `gofmt`, and the harness Go host in `go.work`

Owner: `🔗️graphql` (both languages, schema, SDL golden, cases), `🖥️server/🎛️coordinator` Go formatting,
`go.work`, and the `🧪️test` Go host wiring decision. A concurrent `cli-verbs` executor owned the cli
crate verb regions throughout; every cli edit below was forced by compilation and is named.

## 1. The `fix` mutation is gone from the schema and from the port

The schema advertised `fix(scope: String): FixResult!` while Go's `RepoContext.Fix` could only return
`fix was removed; handle autofix inside script.ts policy export`. Introspection therefore promised a
field that always failed. Removed everywhere, in both languages.

`FixResult` went with it: nothing else in the schema referenced that object type, so leaving it would
have committed an orphan type to the SDL.

### 1.1 `🔗️graphql/📦️packages/🦀️rust/🦀️.rs`

- `RepoContext::fix` — trait method deleted.
- `fix_result_source` — source projector deleted.
- `NamedType::Object(ObjectType { name: "FixResult", … })` — deleted from the shared schema fragment.
- `Field::resolved("fix", nn(ty("FixResult")), "mutation.fix").arg("scope", ty("String"))` — deleted
  from the mutation fragment.
- `"mutation.fix" => …` — resolver arm deleted.
- `RecordingContext`: the `fix: Option<FixResult>` field, its `from_json` decode, and the `fn fix`
  impl deleted.
- `FixResult` dropped from the `semio_framework_repo_model` import list.

### 1.2 `🔗️graphql/📦️packages/🐹️go/🐹️.go`

The same six removals, plus `func (c *RepoContext) Fix` (the erroring one), `func (c *defaultContext)
Fix`, `fixResultSource`, the `Fix *model.FixResult` field on the recording wire shape, and
`func (recording *RecordingContext) Fix`.

### 1.3 The port and its remaining implementations

- `📐️model/📦️packages/🐹️go/🐹️.go` — `Fix(scope *string) (*FixResult, error)` removed from the
  `RepoContext` interface.
- `⌨️cli/📦️packages/🦀️rust/🦀️.rs` — the two `fn fix` impl blocks removed: the `FsContext` one, which
  was the only working implementation in either language, and the delegating wrapper further down.
  Re-read immediately before each edit; nothing else in that file referenced `FixResult` afterwards.
- `⌨️cli/📦️packages/🐹️go/🔬️_test.go` — `testExportContext.Fix` removed, and `TestFixCommand` (which
  asserted that `ToolFix` errors "now that server-side fix was removed") removed with the function it
  tested.
- `⌨️cli/📦️packages/🐹️go/🐹️.go` — `ToolFix` removed. **Forced**: it called `ctx.Fix` through
  `model.RepoContext` and broke `go build` in three `go.work` members the moment the interface method
  went. Verified by the build sweep in §3.
- `⌨️cli/📦️packages/🐹️go/🔌️mcp.go` — `func fix(ctx, request)` removed. It was the only caller of
  `ToolFix` and was never registered as an MCP tool, so it was already dead.

### 1.4 The committed SDL, regenerated from both implementations

`🔗️graphql/🧬️schema/🔣️schema.graphql` was regenerated, never hand-edited. Both renderers were made to
dump their output to a scratch file — Rust through a temporary `SEMIO_SDL_DUMP` branch inside the
existing `serves_the_committed_sdl` test, reverted afterwards; Go through a temporary
`🗑️sdldump_test.go` calling `RenderSDL(BuildSchema())`, deleted afterwards — and the two dumps were
then compared:

```
$ cmp go-sdl.graphql 🔗️graphql/🧬️schema/🔣️schema.graphql && echo "GO SDL BYTE-IDENTICAL"
GO SDL BYTE-IDENTICAL
$ cmp go-sdl.graphql rust-sdl.graphql && echo "GO==RUST BYTE-IDENTICAL"
GO==RUST BYTE-IDENTICAL
```

The committed document lost exactly the two blocks it should have — `type FixResult { … }` and the
`fix(scope: String): FixResult!` mutation field — and nothing else: 607 lines, LF, unchanged in every
other byte. The Rust `serves_the_committed_sdl` assertion, `render_sdl(&build_schema()) ==
include_str!("../../🧬️schema/🔣️schema.graphql")`, passes again against the regenerated file.

### 1.5 Vectors

- `🔗️graphql/🧪️tests/✏️mutation-execution/🧫️fixtures/🔣️mutations.json` — the
  `{ "id": "fix", "source": "mutation { fix { … } }" }` entry removed from the write script.
- `🔗️graphql/🧫️fixtures/🔣️repo-records.json` — the top-level `"fix"` record removed; the file was
  re-parsed to confirm it is still valid JSON.
- `🔗️graphql/🧪️tests/📜️sdl-dump` needed no vector edit: its scenario is `@mode-differential` against
  `graphql-js` `buildSchema` reading the committed document, so regenerating that document is the
  whole update.
- `🔗️graphql/🧪️tests/❌️execution-errors/🧫️fixtures/🔣️refusals.json` carried no `fix` vector — nothing
  to change there.
- `🔌️mcp/🔗️graphql/🔗️.graphql` — a second, hand-maintained copy of the SDL inside the mcp module also
  advertised `fix` and `FixResult`; both removed so it stops promising a dead field. **That file is a
  stale duplicate of `🔗️graphql/🧬️schema/🔣️schema.graphql` — 728 lines against 607, diverging shape,
  its own licence header and regions — and should be deleted rather than maintained (§5).**

### 1.6 Verification

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner 🔗️graphql
[test] level=quick cases=8 executed=34 passed=34 failed=0 errored=0 parity=29/29

$ … parity fundamental --owner 🔗️graphql
[test] level=fundamental cases=8 executed=21 passed=21 failed=0 errored=0 parity=18/18
```

`fundamental` was run as well because the `sdl-dump` scenario is `@level-fundamental` and is the one
that actually proves the committed document against `graphql-js`.

```
$ cargo test -p semio-framework-repo-graphql --lib
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
(includes tests::serves_the_committed_sdl)

$ cargo clippy -p semio-framework-repo-graphql --all-targets
Finished `dev` profile [unoptimized] target(s) in 4.72s          (no warnings)

$ cargo build -p semio-framework-repo-cli
Finished `dev` profile [unoptimized] target(s) in 34.69s

$ go test ./...    (GOWORK=C:/git/semio/go.work)
🔗️graphql               ok  github.com/usalu/semio/repo/graphql       3.720s
📐️model                 ok  github.com/usalu/semio/repo/model         0.489s
⌨️cli                   ok  github.com/usalu/semio/repo/cli          13.767s
🖥️server/🎛️coordinator   ok  github.com/usalu/semio/repo/coordinator   2.882s
```

## 2. `gofmt` over every `📦️packages/🐹️go`

Three offenders, all in the coordinator:

| File | Fault |
| --- | --- |
| `🖥️server/🎛️coordinator/📦️packages/🐹️go/🐹️.go` | trailing blank line at EOF |
| `🖥️server/🎛️coordinator/📦️packages/🐹️go/🧪️_test.go` | trailing blank line at EOF |
| `🖥️server/🎛️coordinator/📦️packages/🐹️go/📦️main/🐹️.go` | CRLF on the final line — a line-ending fault, not a blank line |

Formatted with `gofmt -w`. The sweep over the package directories is now silent:

```
$ find 🔨️modules -type d -path "*/📦️packages/🐹️go" -print0 | xargs -0 gofmt -l
(no output)
```

**Still unformatted, outside this item's scope**: 24 `🧪️tests/**` adapters and 2
`📜️statutes/🧫️fixtures/**` files, listed by `gofmt -l 🔨️modules`. Every adapter has the same fault —
`host "semio.tech/repo/test"` sorted before the `github.com/usalu/…` imports instead of after. They
compile and run, so this is cosmetic and one `gofmt -w` away; it belongs to the audit wave. The two
`📜️statutes/🧫️fixtures/…` files are deliberately malformed test input and MUST NOT be formatted.

## 3. `go build` and `go vet` over every `go.work` member

All 26 members — the 25 that were listed before, plus the harness host added in §4 — build and vet
silently:

```
$ while read m; do (cd "$m" && go build ./... && go vet ./...); done < members.txt
BUILD+VET SWEEP DONE        (no per-member output)
```

Worth knowing for the audit wave: `go build ./...` from the repo root does NOT work —
`pattern ./...: directory prefix . does not contain modules listed in go.work or their selected
dependencies` — because no `go.mod` sits at the root. The sweep has to iterate the `use (…)` entries.

## 4. `🧪️test/📦️packages/🐹️go` is now IN `go.work`

Plan §3 says the root `go.work` lists every module; this one was outside it. Decided by measurement,
not by reading.

**The harness cannot be affected, and the code says so explicitly.** In `🧪️test/📜️script.ts`:

- `goWorkspaceModules` filters the `go.work` entries with `module !== "semio.tech/repo/test"`, so the
  host module is dropped from `reachable` even when the workspace lists it.
- `goSutModule` returns `null` for the same module name, with the comment "The host module is already
  required by every generated host; a case owned by the testing domain itself must not declare it
  twice."
- `materializeGoHost` runs every generated host with `env: { …, GOFLAGS: "-mod=mod", GOWORK: "off" }`.

The generated `go.mod` is therefore byte-identical whether or not the workspace lists the host. Those
two filters exist precisely to make workspace membership a non-event.

**Measured both ways.** Same host, same session, `go.work` the only variable:

| Run | outside `go.work` | inside `go.work` |
| --- | --- | --- |
| `discover` | 415 cases | 416 cases\* |
| `parity fundamental --owner 🪪️identity` | `cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4` | `cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4` |
| `parity fundamental --owner 🔌️mcp` | `cases=4 executed=12 passed=12 failed=0 errored=0 parity=6/6` | `cases=4 executed=12 passed=12 failed=0 errored=0 parity=6/6` |

\* The one extra case is `⌨️cli/🧪️tests/📤️export-verb-records`, added to the working tree by the
concurrent `cli-verbs` executor between the two runs — confirmed by diffing the two `discover`
listings line by line. It is not caused by `go.work`.

Both `🔌️mcp` runs report the same two rust subject hosts failing to compile with `error[E0308]:
mismatched types` at `semio-framework-repo-mcp` `repository_server` (`📋️capability-listing`,
`🤝️jsonrpc-handshake`). **Pre-existing, identical in both runs, not caused by this work** — it is the
concurrent fleet's mcp/cli edit and belongs to that owner.

**Decision: added.** `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐹️go` is now a
`go.work` member, so plan §3 holds with no deviation to record. Its `go.mod` was raised from `go 1.23`
to `go 1.25` to match the workspace and plan §3's package convention. A dated line was appended to
`📋️plan.md` §5.

## 5. Left open — for other owners

1. **The `autofix` verb is now runtime-dead in both CLIs.** It sends
   `mutation Fix($scope: String) { fix(scope: $scope) { … } }`, which the schema no longer has:
   - `⌨️cli/📦️packages/🦀️rust/🦀️.rs` — the `autofix()` command definition, the `["repo", "autofix"]`
     dispatch arm, the `fn autofix(session, parsed)` body, and `EngineCommand::Autofix`.
   - `⌨️cli/📦️packages/🐹️go/🐹️.go` — `var autofixCmd` and its `root.AddCommand(…)` registration.

   Both are cli verb regions owned by the concurrent `cli-verbs` executor, so they were deliberately
   left untouched rather than edited underneath it; they must be removed together to keep the two CLI
   surfaces symmetric. Neither breaks compilation — nothing here is red.
2. **`🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` still carries the removed field**: `export type
   FixResult`, `fix: FixResult` on `Mutation`, `FixMutationVariables`, `FixMutation`, and both
   `FixDocument` constants (the codegen artifact and the source operation). This is graphql-codegen
   output plus its input operation — regenerated, not hand-maintained — and belongs to the `wiring`
   wave. Nothing compiles it as part of the repo test surface today.
3. **`🔌️mcp/🔗️graphql/🔗️.graphql` should be deleted.** Keeping two schema documents is the real
   defect; its `fix`/`FixResult` blocks were removed here only so it does not advertise a dead field.
4. **`gofmt` over the `🧪️tests/**` adapters** — 24 files with reversed import grouping (§2).
5. **`semio-framework-repo-mcp` does not compile** (`E0308` at `repository_server`), failing two rust
   subject hosts in `🔌️mcp`. Pre-existing, concurrent fleet's, reproduced in both §4 runs.
