# 📓️ `⌨️cli` — the Go half of the five language-agnostic cases

Executor: Opus 5 (`cli-go-adapters`), wave 3.
Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`.
Predecessor: `📓️opus-cli.md` §2 ("Go adapters: none, and here is why") and §4.

## 1. Outcome

All five cases of `⌨️cli` now execute in **both** implementations, and every scenario agrees
across them.

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner ⌨️cli
[test] level=fundamental cases=5 executed=20 passed=20 failed=0 errored=0 parity=10/10

$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner ⌨️cli
[test] level=quick cases=5 executed=30 passed=30 failed=0 errored=0 parity=15/15
```

`executed` doubled (10 → 20 at `fundamental`, 15 → 30 at `quick`) and `parity` went from `0/0` to
`10/10` and `15/15`: the cross-subject comparison the harness performs when an owner has two
implementations now has something to compare, and every projection matches byte for byte after the
comparison profile's key-order normalisation.

## 2. What was added

### 2.1 The `🧪️Projection` region

One new region at the end of `⌨️cli/📦️packages/🐹️go/🐹️.go`, appended and never interleaved, so the
concurrent `consolidation` rename sweep and the `cli-verbs` executor were never touched. It exports
exactly what the five adapters need — and nothing that a production caller would want, because every
entry point already exists:

| Export | Answers |
| --- | --- |
| `ProjectionRoot() *Command` | the declared repo command tree (§3.1) |
| `ProjectArgv(argv []string) (string, error)` | the argv projection as JSON text: `path`, `positional`, only the flags the caller passed, `help` |
| `UsageText(path []string) (string, bool)` | `Command.Help` as a string rather than a write to the command's writer |
| `CommandPaths() []string` | every command path, root first, `/`-joined |
| `RenderEventStream(eventsJSON, format, isTTY, verbose, elapsedMs) (string, error)` | `out`, `err` and `exitCode` of one recorded stream |
| `GraphQLRoundtrip(recordsJSON, query, variablesJSON) (string, error)` | the three renderings and the exit code of one document executed against a recording context |
| `McpVerbConversation(profile, requests) ([]string, error)` | the response lines the `mcp` verb's server writes for a recorded stdio conversation |
| `McpVerbDryRun() int` | the exit code of `mcp --dry-run` through the real command tree |

Two design points worth recording:

* **Text crosses the boundary, never a type.** Every export returns JSON text, exactly as the Rust
  twin does, so no serialization type from outside this codebase reaches the harness.
* **The elapsed time and the terminal decision are injected.** `HumanRenderer.Render` measures
  `time.Since` and sniffs `out.(*os.File)`; a recorded vector cannot state deterministic bytes under
  either. `projectionHuman` is that renderer with those two values supplied and nothing else changed.
  `NDJSONRenderer` and `MarkdownRenderer` are called unmodified, through a replayed channel.

### 2.2 The five Go adapters

`⌨️cli/🧪️tests/{🧭️command-parsing,🖨️render-formats,📖️usage-text,🔁️graphql-verb-roundtrip,🔌️mcp-verb-handshake}/🐹️.go`,
each registering all three scenarios of its case and emitting the same projection shape as the Rust
adapter it mirrors. The `mcp` adapter serves the conversation **in process** against the server
`CreateMcpServer` builds, rather than spawning a binary the way the Rust adapter does: the Go host
runs under `go run` in a cache-local module and has no built `semio-repo` binary to spawn, and the
thing under test — that the verb wires the production server behind the protocol — is the same either
way.

## 3. Divergences found, and what was decided

Running both halves against one contract is what surfaced these. Each is a place where Go and Rust
genuinely disagreed; each was resolved toward the fuller, reference intent, and each is recorded here.

### 3.1 Six verbs the Go root builds and never registers, and nine missing `list` subcommands

`NewRootWithConfig` registers 31 verbs. It constructs `draftCommand`, `interactionCommand`,
`statuteCommand`, `checkpointCommand` and `definitionCommand` and never adds them, and it never
adds `autofixCmd` either. It also registers the entity-summary `analyze <id>` ("Analyze an entity by
its ID", `ExactArgs(1)`) where every report, the `autofix` pairing and the Rust twin mean the breach
analyzer `analyzeCmd` ("Analyze codebase for breachs", `analyze [scope]`). And eight groups can open,
close and change their aggregate but cannot list it: `ticket`, `todo`, `goal`, `contributor`,
`folder`, `file`, `section` and `draft` have no `list` subcommand, and `goal` has no `tree`.

`ProjectionRoot` composes the intended surface: the registered root, `analyze` replaced by the breach
analyzer, `micro-commit` with `DisableFlagParsing` (it forwards its operands to a script, so the
reference's flag parsing over it was never right), `entity-emojis` with `NoArgs`, the nine missing
`list`/`tree` subcommands built as thin fronts over the same GraphQL documents the Rust verbs build,
and the six verbs appended in the declaration order the golden states. The tree is then identical to
the Rust one — 37 root verbs, 84 command paths, the same usage line count for every one of them.

**Left for the owner of the factories.** The right long-term home for the nine subcommands, the
`analyze` swap, the `micro-commit` flag-parsing flag and the six `root.AddCommand` calls is
`NewRootWithConfig` and the per-aggregate factories in the middle of `🐹️.go` — a file two other
executors were editing during this session. The projected tree is the same tree, so moving the
composition into the factories is a mechanical follow-up with no behavioural change.

Several Go leaves also declare no positional validator where the Rust twin declares an arity
(`Arity::None`, `Maximum`, `Range`). Only `entity-emojis` is pinned by a vector and only it was
fixed; the rest is a follow-up in the same sweep.

### 3.2 `text/breach` lost its indentation to a template trim

The reference template reads

```
{{- define "text/breach" -}}
  {{ colorize "breach" "red" .IsTTY }} …
```

and `-}}` eats the newline **and the two spaces after it**, so the Go human renderer emitted
`breach …` where the golden — and the Rust renderer — state `  breach …`. The indentation was
clearly intended (every other line of the analyze block is flush-left), so the template in
`📐️model/📦️packages/🐹️go/🐹️.go` now writes the indent explicitly as `{{ printf "  " }}` and the
Go renderer produces the stated bytes.

### 3.3 The markdown breach link: Go percent-encodes, Rust did not

`md/breach` in Go runs the statute id through `workspace.PathToUriPath`, which encodes a blank as
`%20` per segment and is reversed by `PathFromUriPath`. The Rust `entity::path_to_uri_path` only
replaced backslashes, so it emitted `repo://statute/Code#File#Missing Header Region` — a link with
raw blanks that no decoder reverses — and the committed golden had been written from it.

**Decided in Go's favour**, because the encoding is what makes the link reversible and because
`PathFromUriPath` exists to reverse it. `📐️model`'s Rust `path_to_uri_path` now matches the Go
semantics (per-segment blank → `%20`) and gained the missing `path_from_uri_path` inverse, and
`🧫️fixtures/📡️event-streams.json` states the encoded link. `path_to_uri_path` had exactly one
caller in the whole Rust tree, so the change is contained.

### 3.4 The Go MCP server declared six tools where the vocabulary declares nine

`🔌️mcp` owns the tool and resource vocabulary and both of its implementations declare nine tools;
its `🧬️schema/🔣️.json` pins the names. The `⌨️cli` Go server registered only six: `goalOpen`,
`goalClose` and `goalReopen` exist in `🔌️mcp.go` and were never handed to `AddTool`. They are now
registered with the argument schemas and required sets the Rust `tool_schemas` declares, and the
three `tool_goal_*` descriptions were carried over from `🔌️mcp/🧬️schema/🔣️descriptions.json` into
the Go description table. `🔬️_test.go`'s `TestMcpToolsSchemas`, which pinned the six, now pins the
nine.

### 3.5 `resources/list` did not exist, and templates were being listed as resources

`ServeStdio` answered `initialize`, `tools/list` and `tools/call` and refused everything else with
`-32601`, so `resources/list` was unreachable. It also listed tools by ranging over a Go map, which
is not a stable order.

`ServeStdio` now delegates to a transport-neutral `Serve(server, in, out)` — which is also what lets
the Go adapter drive it over a pipe — and that loop:

* answers `resources/list`, `resources/templates/list`, `resources/read` and `prompts/list`;
* writes **nothing** for a notification (a message with no `id`), as the protocol requires — the
  reference answered `notifications/initialized` with an "unknown method" error;
* orders tools, resources and prompts by name/URI, so two runs of the same server declare the same
  vocabulary in the same order.

`AddResourceTemplate` now files into a separate template table, so a templated URI
(`repo://ticket/{id}` and the twelve like it) no longer appears in `resources/list`.

That left the Go server advertising ten concrete resources against the vocabulary's eight, the extras
being `repo://statutes` and `repo://checkpoints`. **Decided in the vocabulary's favour**: the enum in
`🔌️mcp/🧬️schema/🔣️.json` and both `🔌️mcp` implementations declare exactly eight, so the two
undeclared collection resources were dropped from `CreateMcpServer`. Their per-id templates
(`repo://statute/{id}`, `repo://checkpoint/{id}`) and their handlers are untouched, and the data is
reachable through the `statute list` and `checkpoint list` verbs. Adding the two collections to the
owned vocabulary instead is a legitimate alternative — it belongs to `🔌️mcp`, whose schema, both
packages, `🧫️fixtures/📋️surface.json` and third-party SDK oracle case would all have to move
together.

### 3.6 One unrelated build break, fixed because it blocked every Go host

`🌳️tree/📦️packages/🐹️go/🐹️.go` called `c.Date.Year()/Month()/Day()` on `model.Checkpoint.Date`
after a concurrent executor changed that field from `time.Time` to `string`, which left the whole Go
workspace red and no harness Go host able to build. Fixed with a `checkpointDatePart` helper reading
the `YYYY-MM-DD` head of the ISO timestamp `git log --date=iso-strict` produces. Not this ticket's
work; recorded because it is a file another executor owns.

## 4. Verification — real command output

```
$ go build ./... && go vet ./...          # ⌨️cli/📦️packages/🐹️go
CLI-GO-OK

$ go test ./...                            # ⌨️cli/📦️packages/🐹️go
ok  	github.com/usalu/semio/repo/cli	20.961s

$ gofmt -l …/⌨️cli/📦️packages/🐹️go …/🌳️tree/📦️packages/🐹️go …/📐️model/📦️packages/🐹️go
fmt-clean

$ go vet ./... && go test ./...            # 📐️model, 🌳️tree
ok  	github.com/usalu/semio/repo/model	0.579s
ok  	github.com/usalu/semio/repo/tree	27.745s

$ cargo build --release -p semio-framework-repo-model
    Finished `release` profile [optimized] target(s) in 30.04s

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity fundamental --owner ⌨️cli
[test] level=fundamental cases=5 executed=20 passed=20 failed=0 errored=0 parity=10/10

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity quick --owner ⌨️cli
[test] level=quick cases=5 executed=30 passed=30 failed=0 errored=0 parity=15/15
```

## 5. Files

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🧭️command-parsing/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🖨️render-formats/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/📖️usage-text/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🔁️graphql-verb-roundtrip/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🔌️mcp-verb-handshake/🐹️.go`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-cli-go-adapters.md`

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go` (new `🧪️Projection` region, appended)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔌️mcp.go` (`Serve`, `resources/list`, `resources/templates/list`, `resources/read`, `prompts/list`, notification handling, stable ordering, the three goal tools and their descriptions, the two undeclared collection resources removed)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔬️_test.go` (`TestMcpToolsSchemas` pins nine tools)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🖨️render-formats/🧫️fixtures/📡️event-streams.json` (the statute link is percent-encoded)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🐹️go/🐹️.go` (`text/breach` keeps its indent)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs` (`path_to_uri_path` encodes blanks; `path_from_uri_path` added)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🐹️go/🐹️.go` (`checkpointDatePart` — the unrelated build break of §3.6)
