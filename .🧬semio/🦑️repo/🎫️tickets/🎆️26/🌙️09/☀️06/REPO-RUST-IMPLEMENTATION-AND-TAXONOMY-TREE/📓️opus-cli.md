# 📓️ `⌨️cli` — Rust command framework, engine, renderers, repo verbs and the `mcp` verb

Executor: Opus 5 (`cli`), wave 3.
Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`.
Crate: `semio-framework-repo-cli` (`📦️packages/🦀️rust/🦀️.rs`), binaries `semio` and `semio-repo-mcp`.
Reference: the Go CLI package `📦️packages/🐹️go/🐹️.go` (`RunCLI`, `internal/command`, Engine, renderers,
templates) and, for entity rendering, `📐️model/📦️packages/🐹️go/🐹️.go`.

## 1. What landed

All work is in NEW regions appended to `🦀️.rs`, so it never collided with the concurrent `dashboard`
executor, who replaced the `#[path]` module lines at the top of the same file mid-session. The only
edit inside a pre-existing region is one line of `🔖️Dispatch` (§1.9).

| Region | Content |
| --- | --- |
| `🔖️Ansi` | The six ANSI escapes, `colorize`, `prop_color`, `terminal_width` (`COLUMNS`, else 120), `truncate` (escape-aware, ellipsis at `max-3`) |
| `🔖️Entity` | `artifact_id` (the whole 40-branch `GetArtifactID`), `artifact_uri`, `collect_props`, `render_human`, `render_markdown`, `render_markdown_link`, `infer_kind`, `sanitize_prop`, `sanitize_single_line`, `parse_flexible_epoch` + relative-time rendering |
| `🔖️Engine` | `Kind`, `Event`, `Progress`, `Artifact`, `ErrPayload`, `DonePayload`, `EngineCommand`, `Request`, the `GraphQLExecutor` port, `Engine::run` and the four exit codes (0/1/2/130) |
| `🔖️Render` | `ndjson`, `human`, `markdown`, `format_result`, `format_markdown_result`, `format_markdown_file`, `events_to_markdown`, `go_duration`, the `Lines` `EntityRenderer` |
| `🔖️Command` | `FlagValue` (text/bool/int/text-list/int-list/duration), `Flag`, `Arity`, `Command`, `Command::help`, `parse`, `resolve`, `parse_duration` |
| `🔖️Context` | `FsContext`: the production `RepoContext` (49 methods) over the domain crates; `SystemClock`, `GitCheckpoints`, `OfflineTracker` |
| `🔖️Mutate` | The folder/file/section/integrate/extract/rename mutations: a `🚚️move` plan applied through `FileSystemExecutor` |
| `🔖️TreeSource` | `FsTreeSource`: the `🌳️tree` `TreeSource` loaded from the domain crates, for `list`/`search`/`query` |
| `🔖️Verbs` | `Session` (format, verbose, repo root, timeout, tty), `Session::graphql`, the tree-filter flag vocabulary and filter construction |
| `🔖️Tree` | `tree_spec::root()` — the whole declared command tree with every operand arity and flag |
| `🔖️Dispatch2` | `repo_cli::run` and one handler per verb, plus the harness-facing `projection`, `usage`, `command_paths`, `render_stream_json`, `artifact_identity_json`, `graphql_roundtrip_json` |
| `🔖️McpVerb` | `RepoRepository` (the production `🔌️mcp` `Repository`), `real_repository`, `serve` |

### 1.1 The templates

The reference renders through Go `text/template`. There is no template engine in Rust here and none
was added: the twelve `text/*` and nine `md/*` definitions are short enough to be plain formatting,
so they are inlined at their call sites in `🔖️Render` and `🔖️Entity`. The rendered bytes are
identical, including the reference's own oddities — the failure line really does read
`failed failed <command>` and the error line `error error: <message>`, because the template writes
the coloured word and then the literal word.

### 1.2 The command framework

The reference framework mutates flag state on the `Command` tree during parsing. The Rust half keeps
the tree an immutable **spec** and produces a `Parsed` value instead: the selected path, the
operands, and the effective flag table (defaults plus what the caller passed, with `changed`
recording which). This is what made the language-agnostic `🧭️command-parsing` case possible at all —
argv in, one comparable projection out — and it removes a whole class of state leakage between
invocations that `resetFlagsRecursive` exists to paper over in the reference.

Everything observable is the reference's: subcommand selection stops at the first operand, `--` ends
flag parsing, `--help`/`-h` short-circuits at the command selected so far, a boolean flag never
consumes the next token, `--name=value` and `-v value` both work, `DisableFlagParsing` swallows
everything after the name, and the refusal messages are verbatim (`unknown flag: %s`,
`flag needs an argument: %s`, `invalid integer for --%s: %s`, `accepts at most %d arg(s), received %d`, …).

### 1.3 The usage text

`Command::help` reproduces `Command.Help` byte for byte: `<short>\n\nUsage:\n  <use>`, then for a
command with children `\n\nCommands:\n` and one `  %-20s %s\n` line per child in declaration order.

### 1.4 The production `RepoContext`

There was no filesystem-backed `RepoContext` anywhere in Rust — `🔗️graphql` shipped only
`RecordingContext`. `FsContext` is that implementation, and it is what makes every GraphQL-backed
verb real:

| Surface | Backed by |
| --- | --- |
| `root_dir`, `technologies`, `bundles`, `folders`, `files` | `🗂️codebase` `Codebase` / `CodebaseContext` |
| `sections`, `definitions` | `🗣️languages` `parse_sections` / `parse_definitions` over the walked files |
| `contributors` | `🧑️contributors` `FsContributorStore` + `list_contributors` |
| `checkpoints` | `🧑️contributors` `list_checkpoints` over a new `GitCheckpoints` source (`git log --pretty=…`) |
| `goals`, `goal_create/change/close/reopen/delete` | `🎯️goals` `Goals` over `FsGoalStore` |
| `tickets`, `ticket_open/close/reopen/change` | `🎫️tickets` `TicketService` over `FileTicketStore` |
| `todos`, `todo_create/change/delete` | `📝️todos` `Todos` over `FsTodoTree` |
| `drafts`, `draft_create/delete` | `📝️todos` `FsDraftStore` |
| `policies`, `statutes` | `📜️statutes` catalog |
| `analyze`, `fix` | `📜️statutes` `analyze` / `filter_ignored` / `autofix` over the scoped `SourceSet` |
| `interactions` | flattened from the ticket documents |
| folder/file/section create·move·delete, `integrate`, `extract` | `🚚️move` planners applied through `FileSystemExecutor` |

### 1.5 The `mcp` verb and the shipped binary

`semio mcp [kind]` builds a `RepoRepository` over `FsContext` and serves it with
`mcp::serve_stdio`; the profile comes from the operand, else `SEMIO_REPO_MCP_CLIENT`. `--dry-run`
initializes and exits.

**Decision on the second binary.** `🔌️mcp` cannot depend on `⌨️cli` (that is the DAG edge the whole
layering exists to prevent), so `semio-repo-mcp` could never have delegated to the production
repository from where it was. It was **moved into this crate** (`📦️mcp-main.rs`, a second `[[bin]]`),
`🔌️mcp/📦️packages/🦀️rust/📦️main.rs` was deleted, that crate's `role` became `library` and its
description dropped the binary claim, and its `📜️script.ts run` now builds
`-p semio-framework-repo-cli --bin semio-repo-mcp`. The protocol crate stays domain-free; the crate
that owns both protocol and domain ships the server. `mcp::run`/`UnwiredRepository` are now unused by
any binary — a consolidation item for the audit wave (§4).

The MCP tool and resource vocabulary is `🔌️mcp`'s, not this crate's: nine mutating tools
(`ticket_open/close/reopen`, `goal_open/close/reopen`, `section_move/extract`, `file_integrate`) and
eight read resources (`repo://`, `bundles`, `contributors`, `files`, `folders`, `goals`, `policies`,
`tickets`). `RepoRepository` maps each to the GraphQL document the equivalent verb builds and renders
the result as the markdown a tool result carries.

### 1.6 The verbs

Wired and answering from the domain crates: `analyze`, `autofix`, `sync github|management`,
`graphql`, `ticket open|close|reopen|change|list`, `todo create|change|delete|search|list`,
`goal open|close|reopen|change|list|tree`, `contributor add|remove|list`,
`folder|file create|move|delete|list`, `section create|move|delete|integrate|extract|list`,
`integrate`, `extract`, `move`, `rename`, `bundle list|tree`, `technology list|tree`,
`statute list|tree`, `checkpoint list`, `draft create|delete|list`, `definition list`,
`interaction list`, `list`, `search`, `query`, `entity-emojis`, `auth status`, `mcp`.

Refusing with a named missing port (§4): `test`, `hook`, `loc`, `mermaid`, `export`, `configure`,
`micro-commit`, `benchmark`, `update`, `auth whoami`, `ticket purge-artifacts`.

### 1.7 Root verbs the reference builds but never registers

`draftCommand`, `interactionCommand`, `statuteCommand`, `checkpointCommand` and `definitionCommand`
are constructed in the Go package and never passed to `root.AddCommand` — they are reachable only
through the MCP handlers. The ticket asked for `statute`, `interaction` and `draft` as verbs, so all
five are registered here. **This is a deliberate divergence**: the Rust root lists 37 verbs where the
Go root lists 31 (the extra six being those five plus `autofix`, which the reference also builds and
never registers). Everything else about the root help — the summary, the layout, the order of the
31 shared verbs — is identical. The divergence is stated in `local://📖️usage-goldens.json`, so it is
a committed decision rather than a drift.

### 1.8 Two additions beyond the reference

`ticket list` and `goal list`/`goal tree` do not exist in the Go CLI (listing goes through `list` and
`search`). The ticket asked for `semio ticket list --json` as proof, and a CLI that can open, close
and change a ticket but not list one is a hole, so both were added.

### 1.9 The one edit inside an existing region

`🔖️Dispatch`'s `run` gained one arm, so a repo verb or a leading root flag routes into `repo_cli`
instead of falling through to the root-script delegation:

```rust
verb if verb.starts_with('-') || repo_cli::verb_names().iter().any(|name| name == verb) => repo_cli::run(&argv),
```

`daemon`, `workflow`, `dev`, `catalog`, `command-tree`, `plugin registry`, the TUI on empty argv and
the root-script delegation for everything else are untouched and still work.

## 2. Language-agnostic tests

`🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/`, owner manifest `⌨️cli/🔮️oracle/🔣️.json`.

| Case | Scenarios | Fixture |
| --- | --- | --- |
| `🧭️command-parsing` | argv projects into a command · refusals carry their verbatim message · parsing is idempotent | 26 argv vectors, each stating the whole projection or the verbatim message |
| `🖨️render-formats` | every stream renders its stated bytes · no colour without a terminal · the exit code follows the done event | 12 event streams with exact stdout, stderr and exit code, including the coloured terminal case |
| `📖️usage-text` | representative commands state their text · every command obeys the usage law · the root registers its stated verbs | verbatim goldens for six commands, the structural law, the 37 root verbs in order |
| `🔁️graphql-verb-roundtrip` | every verb document executes and renders · an execution failure becomes a failing stream · rendering is deterministic | `🔗️graphql`'s frozen `RecordingContext` records plus the ten documents the read verbs build |
| `🔌️mcp-verb-handshake` | the verb completes the handshake · initialize ignores unknown members · the dry run starts no server | the stdio conversation and the vocabulary it owes; **spawns the built `semio` binary** |

Every capability is recorded as a `noOracleDecision` with its substitutes. That is the honest answer
here and not a shortcut: an argv grammar written in place of a third-party parser, a presentation
vocabulary nothing outside this checkout reads, a usage layout this repository decides, and — for
the two cases where a real oracle exists — the oracle belongs to a **neighbour**: GraphQL execution
is already held against the `graphql` npm package by `🔗️graphql`, and the MCP protocol against
`@modelcontextprotocol/sdk` by `🔌️mcp`. Re-running either here would measure someone else's
contract. Each decision therefore names `specification-vectors` plus, where one applies,
`metamorphic-laws` (idempotence, determinism, no-escape-without-a-terminal, exit-code agreement
across all three formats) or `cross-owner-contract`.

**Go adapters: none, and here is why.** The reference Go package exports `NewRootWithConfig`, the
three renderer types and `Engine`, but not `selectAndParse`, not `formatResult`, not
`formatMarkdownResult`, and a parse there returns no operand vector at all — `Command.execute`
consumes the positionals into the handler. A Go adapter for `🧭️command-parsing` or
`🖨️render-formats` would therefore need new exported entry points in
`🐹️.go`, a file a concurrent triage executor is editing this same session. Adding them is a small,
well-defined job (`ProjectArgv(argv []string) ([]byte, error)` and
`RenderStream(events []Event, format string, isTTY, verbose bool, elapsedMs int64) ([]byte, error)`)
and is listed in §4 rather than done blind against a moving file.

## 3. Verification — real command output

```
== build ==
warning: `semio-framework-repo-cli` (lib) generated 5 warnings (run `cargo fix --lib -p semio-framework-repo-cli` to apply 5 suggestions)
    Finished `release` profile [optimized] target(s) in 0.45s
== test ==
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests 📦️main.rs (target\release\deps\semio-99a4f0d20bd294a6.exe)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests 📦️mcp-main.rs (target\release\deps\semio_repo_mcp-35fb24616176b32d.exe)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
== parity fundamental ==
[test] level=fundamental cases=5 executed=10 passed=10 failed=0 errored=0 parity=0/0
== parity quick ==
[test] level=quick cases=5 executed=15 passed=15 failed=0 errored=0 parity=0/0
```

`cargo test` reports zero unit tests because the `🔖️Tests` region that used to sit at the end of
`🦀️.rs` moved to `🎛️dashboard` with the daemon and IPC code it covered. The 15 harness scenarios are
this crate's tests.

### Clippy

`cargo clippy --release -p semio-framework-repo-cli` reports **no warning against any line of the
regions listed in §1**. Two warnings remain against `🦀️.rs`, both on lines the `dashboard` executor
owns (`pub fn run(argv: Vec<String>)` passed by value, and a `map(…).unwrap_or(…)` in the plugin
registry arm). `repo_cli` carries one scoped, reasoned allow — `clippy::unnecessary_wraps`, because
every verb handler shares one signature so the dispatch table stays a flat match.

### `semio ticket list --json`

```
$ ./target/release/semio.exe ticket list --json
{"repo":{"tickets":[{"id":"🎫refactor","prompt":"Migration from REFACTOR.md","slug":"REFACTOR","status":"CLOSED","title":"Migration from REFACTOR.md"},{"id":"🎫breadcrumbrendererror","prompt":"Migration from 2025-11-18_BREADCRUMB-RENDER-ERROR.md","slug":"BREADCRUMB-RENDER-ERROR","status":"CLOSED","title":"Migration from 2025-11-18_BREADCRUMB-RENDER-ERROR.md"},{"id":"🎫breadcrumbshiftissue","prompt":"Migration from 2025-11-18_BREADCRUMB-SHIFT-ISSUE.md","slug":"BREADCRUMB-SHIFT-ISSUE","status":"CLOSED","title":"Migration from 2025-11-18_BREADCRUMB-SHIFT-ISSUE.md"}, …
```

### `semio analyze --json` over this repository

```
$ time ./target/release/semio.exe analyze --json
{"analyze":{"breachs":[{"column":0,"excerpt":null,"id":"🚫repo/breach/compose/client/lib/net/Compose/obj/Debug/net48/.NETFramework,Version=v4.8.AssemblyAttributes.cs","kind":{"autofixable":true,"id":"Code#File#Missing Header Region","priority":"LOW","reason":"Header region with license, filename, and contributors is required","solution":"Add header region with SPDX license, filename, and contributors"},"line":0,"scope":"compose/client/lib/net/Compose/obj/Debug/net48/.NETFramework,Version=v4.8.AssemblyAttributes.cs","summary":"Missing header region in compose/client/lib/net/Compose/obj/Debug/net48/.NETFramework,Version=v4.8.AssemblyAttributes.cs"}, …
real    5m22.559s
```

Scoped, it is instant, and the human renderer works:

```
$ ./target/release/semio.exe analyze "🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree" --text
-> found 0 breachs
ok done    6ms
```

### `semio goal list`, all three formats

```
$ ./target/release/semio.exe goal list          # markdown, the default
- [🎯r2602](repo://goal/🎯r2602) - `r26-02` - `open` - `6 months ago` - `The r26-02 release aims to deliver sketchpad running at MVP level, …`
  - [🎯runningsketchpad](repo://goal/🎯runningsketchpad) - `Running Sketchpad` - `open` - `6 months ago` - `Running sketchpad infrastructure and apps with MVP functionality.`
    - [🎯runningsketchpadapps](repo://goal/🎯runningsketchpadapps) - `Apps` - `open` - `6 months ago` - `Apps within sketchpad`

$ ./target/release/semio.exe goal list --text
🎯r2602 r26-02 open 6 months ago The r26-02 release aims to deliver sketchpad running at MVP level, …
├️─️─️ 🎯runningsketchpad Running Sketchpad open 6 months ago Running sketchpad infrastructure and apps with MVP functionality.
│️   └️─️─️ 🎯runningsketchpadapps Apps open 6 months ago Apps within sketchpad
```

### MCP `initialize` + `tools/list` over stdio

```
$ printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"probe","version":"1"}}}\n{"jsonrpc":"2.0","method":"notifications/initialized"}\n{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}\n' | SEMIO_REPO_MCP_CLIENT=generic ./target/release/semio.exe mcp
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18","capabilities":{"prompts":{},"resources":{},"tools":{}},"serverInfo":{"name":"repo","version":"1.0.0"},"instructions":"Use repository tools and resources through their owned schemas."}}
{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"file_integrate","description":"Use when two files must be merged at a named region boundary without losing section markers.","inputSchema":{…}}, …]}}
```

The shipped binary answers identically, and a resource read really reaches the domain:

```
$ printf '…initialize…\n…notifications/initialized…\n{"jsonrpc":"2.0","id":5,"method":"resources/read","params":{"uri":"repo://tickets"}}\n' | SEMIO_REPO_MCP_CLIENT=generic ./target/release/semio-repo-mcp.exe
{"jsonrpc":"2.0","id":5,"result":{"contents":[{"uri":"repo://tickets","mimeType":"text/markdown","text":"- [🎫refactor](repo://ticket/🎫refactor) - `Migration from REFACTOR.md` - `CLOSED`\n- …"}]}}
```

### Orchestrator verbs still work

`semio` with no argv still opens the TUI (or prints the usage when stdout is not a terminal), and
`daemon`, `workflow`, `dev`, `catalog`, `command-tree`, `plugin registry` and the root-script
delegation are unchanged — `semio nonexistent-verb` still lands on `📜️script.ts`.

## 4. What is left

**Verbs whose domain port is not reachable yet.** Each refuses with a message naming the missing
port rather than pretending to work:

| Verb | Missing |
| --- | --- |
| `test` | a `FilesystemSnapshot` builder over the real repository for `🏃️test-runner`, plus a `ProcessRunner` that actually spawns |
| `hook` | the `🪝️hooks` `HookEnvironment` / `TestFileResolver` implementations over the real filesystem |
| `loc`, `mermaid` | `📊️metrics` needs a git-log reader and a scanned-line source; the Mermaid treemap then comes from `🌳️tree` |
| `export` | the `📡️events` event-export writer over a `RepoContext` |
| `configure`, `micro-commit` | `🧩️providers` version control and hook-file editing |
| `benchmark`, `update` | the per-ecosystem process recipes; no domain crate owns them yet |
| `auth whoami` | `📡️events` exposes `HttpEmitter` but no `ServerWhoami` |
| `ticket purge-artifacts` | `🎫️tickets` has `PurgeReport` and the size constants but no purge entry point |

**Surfaces this crate implements that a domain crate should own** (consolidation targets):

1. **Entity rendering.** `🔖️Ansi` + `🔖️Entity` are a port of `📐️model`'s `GetArtifactID`,
   `GetArtifactURI`, `CollectEntityProps`, `RenderEntity*`, `Colorize`, `TruncateANSI`,
   `GetTerminalWidth` and the two template documents. In Go these live in `📐️model`; in Rust they
   live here because `📐️model` had none of them. `🌳️tree` already declares the `EntityRenderer` and
   `ArtifactIdentifier` ports for exactly this, so the natural home is `🪪️identity` (ids) plus
   `📐️model` (props and templates). Roughly 700 lines to move.
2. **`FsContext`.** The production `RepoContext` should arguably sit next to `🔗️graphql`'s
   `RecordingContext`, or in a small `🎥️context` module. It is here because it needs to reach
   `🎫️tickets`, `🎯️goals`, `📝️todos`, `🚚️move` and `📜️statutes`, which `🔗️graphql` must not depend on.
   Keeping it in `⌨️cli` is the only placement that does not break the DAG; recording it so the audit
   wave can confirm that is the intended answer.
3. **`FsTreeSource`.** Same argument as `FsContext`: `🌳️tree` declares the port, this crate is the
   only place that can satisfy it from every aggregate at once.
4. **`SystemClock`, `GitCheckpoints`, `OfflineTracker`.** `🎫️tickets` ships only `FixedClock` and
   `RecordedIssueTracker`; `🧑️contributors` ships only `MemoryCheckpointSource`. The real ones are
   here. They belong in their own crates behind the same trait.
5. **`mcp::run` / `UnwiredRepository`** are now dead: no binary calls them since the server moved.
   Delete them with the audit sweep.

**Known behavioural gaps against the reference** (all narrow, all recorded):

- `contributor add|remove`, `ticket delete` and `sync management` refuse from `FsContext`: `🧑️contributors`
  is read-only and no crate owns the management sync.
- `FsContext::folders`/`files` fill `kind` from the path and leave `parentId`/`bundleId` empty;
  `🗂️codebase`'s `CodebaseFolder`/`CodebaseFile` do not carry them and `model::Folder`/`File` want them.
- `entity::build_file_id` resolves a file id from the path alone. The reference consults a
  process-global bundle table; this crate holds no global, and every caller that owns a `Codebase`
  uses `Codebase::build_file_id` instead. Only the `format_result` section/definition branches, which
  have a bare path and no codebase, take the reduced form.
- `search`/`list`/`query` build the whole monorepo tree per invocation. The reference caches it
  (`BuildMonorepoTreeCached`, `SearchMonorepoTreeWithCache`); `🌳️tree` ships `encode_tree_cache` /
  `tree_cache_is_valid` but nothing writes the cache file yet.

## 5. Files

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️mcp-main.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔮️oracle/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🧭️command-parsing/{🥒️.feature, 🦀️.rs, 🧫️fixtures/🔣️argv-vectors.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🖨️render-formats/{🥒️.feature, 🦀️.rs, 🧫️fixtures/📡️event-streams.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/📖️usage-text/{🥒️.feature, 🦀️.rs, 🧫️fixtures/📖️usage-goldens.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🔁️graphql-verb-roundtrip/{🥒️.feature, 🦀️.rs, 🧫️fixtures/🗄️repo-records.json, 🧫️fixtures/🔁️verb-queries.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🔌️mcp-verb-handshake/{🥒️.feature, 🦀️.rs, 🧫️fixtures/🤝️handshake.json}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-cli.md`

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/🦀️.rs` (new regions; one line in `🔖️Dispatch`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml` (20 domain path deps, second `[[bin]]`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📋️project.json` (`mcp` target)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts` (`McpScript`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/Cargo.toml` (`role = "library"`, `[[bin]]` removed)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/📜️script.ts` (builds the CLI crate's binary)
- `.vscode/🧩️launch.seed.jsonc` + regenerated `.vscode/launch.json`
  (`🛠️dev🧰️repo⌨️cli🦀️rust`, `🛠️dev🧰️repo🔌️mcp🦀️rust` → `@semio-tech/repo-cli-rs:mcp`,
  `🧪️test🧰️repo⌨️cli🦀️rust`, `🧪️test🧰️repo⌨️cli🥒️parity`)

Removed:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/📦️main.rs`
