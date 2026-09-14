# 📓️ Go Region → Module Dependency Graph

Tool: `$TICKET/🔨️go-symbol-graph/main.go` (module `symbolgraph`, stdlib-only: `go/ast`, `go/parser`, `go/token`).
Raw outputs: `$TICKET/🗑️generated/go-symbol-graph/{decls.jsonl, modules.json, scc-analysis.json, test-module-refs.json}`.
Region→module map (hand-authored, corrigible): `$TICKET/🔨️go-symbol-graph/🔣️region-modules.json`.

## 1. How to run

```sh
cd "$TICKET/🔨️go-symbol-graph"
GOWORK=off go run .
```

`GOWORK=off` is required because the repo root's `go.work` does not list this scratch module. The tool re-derives `$TICKET` from its own working directory (parent of `🔨️go-symbol-graph`), reads the snapshot at `🗑️generated/go-snapshot/client/{🧩️component.go, 📤️event_export.go, 🔬️component_test.go}`, and (re)writes the four files under `🗑️generated/go-symbol-graph/`. Editing `🔣️region-modules.json` and re-running updates `modules.json`/`scc-analysis.json` without touching the parser.

Totals from the last run: **2219 top-level decls** (2215 from `component.go` + 4 from `event_export.go`), **20 modules** populated (of the 24 in plan.md §2 — `search`, `yaml` and `dashboard`/`graphql`-executor-only pieces have no snapshot content in these three files), **560 test functions** in `component_test.go`.

## 2. Method

1. **Decls**: every top-level `FuncDecl` (receiver → `kind:"method"`, receiver type name captured), every spec inside a `GenDecl` (`type`/`var`/`const`, including grouped `( … )` blocks — one JSONL row per name/spec, sharing the group's line range).
2. **Regions**: a blind, depth-only stack walk over `// #region` / `// #endregion` markers (ignores label mismatches, since the file's region nesting is provably broken — see §3). One manual fix-up was applied per the ticket's instruction: a virtual `Policies` region is spliced in right after the `// #endregion 📝️Sections` at line 18420, so the stray unmatched `// #endregion 🧊️Policies` at line 21613 closes it instead of prematurely closing `⚙️Types` (opened at 13192). Every decl's region is the **innermost region open at its start line**.
3. **Region → module**: hand-authored table in `🔣️region-modules.json` (`nameToModule`, plus `lineOverrides` for the two places a region name is reused for something different, plus a `conflicts` log). Built by reading plan.md §2's module descriptions against each region's content, not by guessing from the name alone (e.g. `⭐️Codebase Types` sits inside `🎽️Languages` but plan.md explicitly lists "Codebase Types" under `📐️model`, so it is mapped there).
4. **Refs**: for each decl, a flat "locals" set (func/recv params+results, `:=` LHS, local `var`/`const`/`type`, `range`/type-switch bindings, closures) is collected once, then every `*ast.Ident` in the decl is counted as a reference if its name is a **non-method** top-level name and not in that locals set. `X.Sel` selectors are only followed into `X`; if `X` is an import alias the selector is recorded as `importsUsed[pkgPath] += Sel` instead of being treated as a decl reference (so `pkg.Foo()` never collides with a local decl named `Foo`). This is the "simple identifier resolution" the task asked for, not full lexical scoping — the known false-negative is that **method values/expressions** (`obj.Method`, `Type.Method`) are never counted as edges, since `Sel` is always skipped for non-import selectors. Method-to-method edges are therefore under-counted; only bare-identifier references (funcs, types, vars, consts) show up in the graph.
5. **Modules/edges**: `modules.json` aggregates decl count, total lines, `importsUsed` (stdlib/internal packages touched), and `edgesTo[targetModule] = ["srcModule.decl -> targetModule.symbol", …]` (skips same-module refs).
6. **SCC/violations**: Tarjan over the module graph; a level table baked into `main.go` from plan.md §2's DAG (`L0 workspace/identity/yaml/search/model … L6 dashboard`) flags every edge whose target level is numerically higher than its source's as a violation.
7. **Tests**: same locals/refs walk applied to every `Test*` func in `🔬️component_test.go`; refs are resolved straight to the *module* of the referenced decl (not module-to-module edges), giving a module set per test.

## 3. The "broken depth-1" region and how it was handled

`🌧️Cli Adapter` opens at line 604 and is closed once at 9879 — but there is a **second**, stray `// #endregion 🌧️Cli Adapter` at line 9881 with nothing left on the stack to close, and separately `⚙️Types` (opened 13192) is closed early by a stray, unlabeled `// #endregion 🧊️Policies` at 21613 (no `// #region` for "Policies" was ever written — the block 18420–21613 is simply unmarked). A blind stack walk that ignores label text still produces a coherent innermost-region per line everywhere except the exact few lines around these stray markers; the only place this actually mattered for module assignment was the Policies gap, which is fixed explicitly (see §2.2) per the ticket's instruction. All other label mismatches (several `🔭️/🔖️/📰️/🪨️Missing Hook Functions` regions in the tail of the file reuse different emoji for what is clearly one intent) are recorded as-is in `region-modules.json`'s `nameToModule` (all routed to `hooks`) rather than "fixed", since fixing the source file is out of scope here.

## 4. Region → module table

See `$TICKET/🔨️go-symbol-graph/🔣️region-modules.json` for the full, authoritative 99-entry table (one row per distinct `// #region` label found) plus the `lineOverrides` and `conflicts` sections. Summary by module:

| Module | Regions mapped to it |
| --- | --- |
| `cli` | 🧲️Header, 🤸️Preamble, 🔌️Adapters, 🎁️Templates, 🔑️Engine Events, 🕌️Engine Errors, 🎸️Engine Requests, 🎖️Engine, 🌧️Cli Adapter, 🎵️Auth Command, 🌩️CLI Renderers, 🧊️ANSI, 🔊️Cli, ⚡️Server Client, 🎼️Utilities, 💾️Missing Utility Functions, 🖋️Missing Tool Functions, 🧬️Missing Utilities *(4 catch-alls, flagged — see §6)* |
| `model` | 🎺️Models, 💡️GraphQL Types, 🎡️GraphQL Input Types, ⭐️Codebase Types |
| `languages` | 🎽️Languages, ⚙️Types, 📝️Sections, 🎶️TypeScript, 🎚️Go, 📜️C#, 📭️JSON, 🛒️Markdown, ⏳️Rust, 🪅️Ruby, 📊️Shell, 📢️TOML, 🤸️YAML, 🕌️SQL, 🎙️GraphQL *(language-plugin one only — see line override)* |
| `graphql` | 🌦️GraphQL Helpers, 🎥️GraphQL Context Port, 🩻️Default Context, 🧱️GraphQL Executor, 🐹️Schema Builder, 🗂️Query Resolvers, 💻️Mutation Resolvers, 🧪️Entity Resolvers, 🗼️Resolver Interfaces, 💡️GraphQL Resolver, 📜️Resolver Methods |
| `tickets` | 📋️Tickets, 🎗️Ticket File Resolution, 📦️MoveTicketPlan, 📝️TicketPlanComment, 🪝️TicketMcpHandlers |
| `providers` | 🪵️Providers, 🔭️Provider Interfaces, 🎄️GitHub Management Provider, 🔒️Git Version Control Provider, ⛑️Devcontainer Sandbox Provider, 🎆️Editor Providers, 🎖️Provider Registry |
| `hooks` | 🦀️Hooks, 🔷️Configure, 🔖️MicroCommit, 🗺️ResolvePlanSource, and the 4 mismatched-label `…Missing Hook Functions` regions |
| `tree` | 🏩️Tree Logic, 🩻️Monorepo Tree, 🎊️Query Cache, 📌️Tree Cache, 🧨️Monorepo Tree Types, ⏲️Mermaid |
| `mcp` | 🦀️Mcp, 🎼️Args, 🔧️Paths, 🏬️Mcp Resources Handlers, 🪄️Handlers, 🪪️McpClientKind, 🗣️McpDescriptions, 🦀️McpServerFactory, and the Mcp-nested 🎙️GraphQL (line override 34845–34869) |
| `statutes` | 🧊️Policies (virtual), 🧷️BreachCache, 🪅️Analyze Command *(both occurrences)*, 📃️Fix Command |
| `workspace` | ⚙️RepoConfig, 📦️Utils |
| `testrunner` | 🕸️Test Command, 🖲️Missing Test Functions |
| `identity` | 🧱️Artifact ID, 🪨️Entity Rendering, 🖥️Entity Emojis Command |
| `metrics` | 🔢️LOC Command, 🔮️Benchmark Command |
| `todos` | 📰️Todos, 🎨️Drafts |
| `codebase` | 🏩️Codebase |
| `goals` | ❄️Goals |
| `contributors` | 🪅️Sessions |
| `move` | ⏰️File Utilities, 🔤️Rename |
| `events` | entire `📤️event_export.go` file |

## 5. Module sizes

| Module | Decls | Lines |
| --- | ---: | ---: |
| model | 326 | 2181 |
| languages | 290 | 3574 |
| cli | 219 | 3660 |
| graphql | 205 | 6477 |
| tickets | 184 | 4631 |
| providers | 137 | 553 |
| hooks | 119 | 1975 |
| tree | 102 | 2511 |
| mcp | 94 | 1873 |
| statutes | 92 | 3218 |
| workspace | 84 | 835 |
| testrunner | 81 | 3764 |
| identity | 80 | 2178 |
| metrics | 63 | 1172 |
| todos | 60 | 1703 |
| codebase | 35 | 985 |
| goals | 25 | 369 |
| contributors | 13 | 290 |
| move | 6 | 223 |
| events | 4 | 128 |

`search` and `yaml` have 0 decls (their content is `internal/search`/`internal/yaml`, not in these three files — out of scope for this tool). `dashboard` is Rust-only per plan.md.

## 6. Module edge table (violations against the L0…L6 DAG)

47 module-pair edges violate plan.md §2's DAG (source level ≤ target level required; full symbol list per edge is in `scc-analysis.json.violations[].symbols` and `modules.json[<module>].edgesTo`). Grouped by edge count:

| Edge (from → to) | Levels | Edges | Dominant cause |
| --- | --- | ---: | --- |
| tickets → cli | L3→L5 | 165 | almost entirely `cli.Kind` (see fix 1) and the 5 `build*ID` helpers (fix 2) |
| testrunner → cli | L3→L5 | 111 | same two causes |
| model → languages | L0→L1 | 40 | `Breach`/`Statute`/`PolicyDef`/`CommandOutput` declared inside the Languages/Types region (fix 3) |
| providers → hooks | L1→L3 | 36 | `HookResult`/`HookEvent` types live in `hooks`, but every editor provider implements `FormatHookOutput`/`NativeEventFromHookEvent` |
| workspace → languages | L0→L1 | 35 | `CommandOutput`/`OutputError`/`OutputLine` (fix 3) |
| graphql → cli | L4→L5 | 25 | `cli.Kind` again, plus `cli.GetFolderChildren`/`GetFolderFiles` used by `buildSchema` |
| model → cli | L0→L5 | 25 | `cli.Kind`, `cli.GitDiffStatus` |
| providers → tickets | L1→L3 | 23 | GitHub issue/label/milestone DTOs (`ghAddComment` etc.) declared under `tickets`, called from the GitHub provider |
| todos → cli | L3→L5 | 21 | `cli.Event`/`cli.Command` (fix 1 generalisation) |
| tree → cli | L3→L5 | 20 | `cli.Kind`, `cli.StreamContributors` |
| identity → cli | L0→L5 | 17 | `cli.Kind`, `build*ID` helpers |
| codebase → cli | L2→L5 | 16 | `cli.Kind`, `cli.ParseContributorIdentity` |
| providers → cli | L1→L5 | 16 | `cli.Kind` (every provider implements `Kind() Kind`) |
| tickets → mcp | L3→L5 | 15 | `mcp.McpClientKind` used directly by ticket tool handlers |
| hooks → mcp | L3→L5 | 14 | `mcp.McpClientKind`/`McpClientKiro`/`McpClientClaude` |
| statutes → cli | L2→L5 | 12 | `cli.Kind`, `cli.buildBreachID` |
| model → tickets | L0→L3 | 12 | `tickets.CountLinesAtCheckpoint`, `tickets.StreamOptions` |
| metrics → cli | L2→L5 | 11 | `cli.Config`, `cli.EngineFactory`, `cli.normalizeRepoPath` — `locCommand` is CLI wiring, not metrics logic |
| languages → cli | L1→L5 | 13 | `cli.Kind` |
| languages → statutes | L1→L2 | 5 | `Statute`/`PolicyDef`/`PolicyFunc` still declared under Languages (fix 3, opposite direction) |
| model → codebase | L0→L2 | 5 | `codebase.CodebaseContext` used inside `BuildSemanticDiffs` |
| todos → graphql | L3→L4 | 5 | `todos.repoContext` wraps `graphql.repoContext` directly instead of the `🎥️GraphQL Context Port` |
| tickets → graphql | L3→L4 | 4 | `graphql.NewRepoContext` called from ticket goal-tool handlers |
| move → cli | L3→L5 | 4 | `cli.toolErrorMsg`/`toolErrorResult`, `cli.Config` |
| contributors → cli | L3→L5 | 4 | `cli.Event`, `cli.Kind` |
| statutes → tickets, statutes → tree | L2→L3 | 3 each | `tickets.StreamOptions`/`matchesFilter`; `tree.BuildMonorepoTreeCached` called from `analyzeCommand` |
| events → graphql | L1→L4 | 3 | `ExportToEventLog*` builds a `graphql.RepoContext` to resolve names before exporting |
| graphql → mcp | L4→L5 | 3 | `graphql.printGQL` used from an MCP resolver; `McpKindFromResolvedClient` called from a `repoContext` ticket method |
| workspace → graphql, workspace → contributors | L0→L4, L0→L3 | 3, 2 | `ensureExecutor` builds a `graphql.Executor`; `DefaultRepoConfig`/`LoggingConfig` embed a `contributors.Session` |
| identity → tickets, identity → goals, identity → languages | L0→L{1,3} | 2,1,1 | `tickets.LoadBundles/LoadTechnologies`, `goals.GetRepoGoalsDir`, `languages.GetLanguage` called from ID/name resolution helpers |
| model → todos, statutes → graphql, events → cli, tickets → cli(via mcp) small tails | — | ≤2 each | see `scc-analysis.json` |
| goals → graphql, languages → contributors, workspace → tickets | L3→L4, L1→L3, L0→L3 | 1 each | `parseMilestoneNumber`, `TicketAgent`→`Session`, `SetRootDir`→`InvalidateTechnologyCache` |

## 7. Cycle

Tarjan finds **one strongly connected component covering 19 of the 20 populated modules** (all except `events`, which only feeds `graphql`/`cli` and is not fed back into): `cli, codebase, contributors, goals, graphql, hooks, identity, languages, mcp, metrics, model, move, providers, statutes, testrunner, tickets, todos, tree, workspace`. In other words the current region layout does not yet form a DAG at all — every "lower" module both feeds and is fed by `cli`. This is expected of a legacy single-package file and is exactly what wave 2 (`go-split`) must break.

## 8. Fixes (highest leverage first)

1. **Move `type Kind` out of `cli`.** It is declared once, at `component.go:199`, inside `🔑️Engine Events` (mapped to `cli` because that region is CLI-engine plumbing) — but it is really the entity-kind enum (`Kind() Kind` is implemented by every provider, model type, and ticket helper) and belongs in `identity` (plan.md: "entity emojis, semantic ids… leading-grapheme rules"). This single move removes the dominant cause in `tickets→cli`(165), `testrunner→cli`(111), `graphql→cli`, `model→cli`, `identity→cli`, `providers→cli`, `codebase→cli`, `languages→cli`, `statutes→cli`, `tree→cli`, `contributors→cli`, `todos→cli` — i.e. most of the table in §6.
2. **Move the 5 ID builders out of the `🧬️Missing Utilities` catch-all.** `buildFileID`, `buildFolderID`, `buildSectionID`, `buildDefinitionID` (component.go:36568/36522/36591/36631) belong to `codebase` ("ID/URI builders for folders/files/sections" — plan.md's own wording); `buildBreachID` (36406) belongs to `statutes`/`identity` (breach IDs are artifact IDs). They currently sit under `cli` only because that catch-all region defaults there.
3. **Split the Languages/Types region's data types by kind, not by proximity.** `Breach` (component.go:13275) → `model` (plan.md lists "Breach*" explicitly under `model`). `Statute`, `PolicyDef`, `PolicyFunc` (15770/16343/…) → `statutes` (plan.md: "policies/statutes/breaches" is the whole point of that module). `CommandOutput`, `OutputError`, `OutputLine` (16405…) → `workspace` (they are the return type of `workspace.ExecCommand`, which is already in `workspace`). This clears `model→languages`(40) and `workspace→languages`(35) — the two largest non-`cli` violations.
4. **Move `*Command` factory functions into `cli`, keep only logic in the domain module.** `locCommand` (metrics), `renameCommand`/`ToolRename` (move), `analyzeCommand`/`autofixCmd` (statutes), `ticketCommand` (testrunner) reference `cli.Config`/`cli.EngineFactory`/`cli.toolErrorResult` purely to register themselves as CLI verbs. Plan.md already gives `cli` ownership of "command factories" — physically relocating the thin `xCommand(cmd *cobra.Command, …)` wrapper (not the underlying `ComputeLOC`/`RenameFile`/`StreamPolicies` logic) into `cli` turns these into ordinary L5→L{1,2,3} calls, which is allowed.
5. **Provider ↔ hooks and provider ↔ tickets (36 + 23 edges): invert via the existing port pattern.** Editor providers (`FormatHookOutput`, `NativeEventFromHookEvent`) need `hooks.HookResult`/`HookEvent`; the GitHub provider needs `tickets`' `gh*` DTOs. Plan.md already prescribes ports for graphql (`🎥️GraphQL Context Port`); the same shape (`HookFormatter`/`IssueTracker` interfaces declared in `providers` or `model`, implemented by `hooks`/`tickets`) removes both edges without moving the bulk data types.
6. **`todos`/`tickets`/`statutes`/`events` → `graphql` (5+4+2+3 edges): route through the context port, not the concrete executor.** `todos.repoContext` wraps `graphql.repoContext` directly and `ExportToEventLog*` builds a `graphql.RepoContext` just to resolve names — both should depend on the `🎥️GraphQL Context Port` interface (already slated to live below `graphql` per plan.md) instead of the L4 package.
7. **`mcp.McpClientKind` used from `tickets`/`hooks` (15+14 edges): move `McpClientKind` down.** It is a small enum (which MCP client/IDE is talking) with zero dependency on the rest of `mcp`; moving it to `identity` (per-IDE profile is conceptually an identity concern, next to entity emojis) or `providers` (editor providers already enumerate the same IDE set) turns these into non-violating calls.

After fixes 1–3 alone, the edge table in §6 loses roughly 260 of its ~700 underlying symbol-references (`Kind`, the 5 ID builders, and the four languages-region types account for that much), which should also shrink or fully dissolve the single 19-module SCC in §7 — re-run the tool after `go-split` (wave 2) physically moves these symbols to confirm.

## 9. Symbols referenced by more than 5 modules (candidates for `model`/`identity`)

Full list (32 symbols) in `scc-analysis.json.sharedSymbolsUsedByOver5Modules`. Top of the list, all already in `model`/`identity`/`workspace`/`languages` (i.e. the design in plan.md §2 already put the widest-shared symbols at L0/L1 — good sign):

| Symbol | Defined in | Used by N modules |
| --- | --- | ---: |
| `Kind` | cli *(should be identity — see fix 1)* | 14 |
| `rootDir` | workspace | 12 |
| `Checkpoint` | model | 11 |
| `ReadTextFile` | workspace | 10 |
| `NormalizePath`, `Flat` | workspace | 9 each |
| `Section` | model | 9 |
| `GetLanguage` | languages | 9 |
| `FileExists` | workspace | 9 |
| `emojiText` | identity | 9 |
| `Ticket` | model | 8 |
| `ExecCommand` | workspace | 8 |
| `Config` | cli | 8 |
| `buildFileID`, `EngineFactory` | cli *(buildFileID should be codebase — see fix 2)* | 7 each |
| `GetRepoMetaDir` | workspace | 7 |
| `ToolResult` | languages | 7 |
| `Bundle` | model | 7 |
| `Scope`, `StreamOptions`, `buildFolderID`, `Summary`, `ListTickets`, `Statute` (→statutes, fix 3) | mixed | 6 each |
| `Contributor`, `BundleKindSite`, `SetRootDir`, `findRepoRoot`, `isGitIgnored`, `ScopeRepo`, `GetRootDir` | mixed | 6 each |

`Kind`, `rootDir`/`RootDir` family, `Checkpoint`, `Section`, `Ticket`, `Bundle`, `Contributor` confirm plan.md's choice to put `model`/`identity`/`workspace` at L0 — they are exactly the symbols every other module needs.

## 10. Test split table

560 `Test*` functions in `🔬️component_test.go`; 36 reference no production top-level name we track (likely table-driven helpers or pure-stdlib tests). Module touch counts (a test can touch more than one module):

| Module | Tests touching it | Tests touching *only* it |
| --- | ---: | ---: |
| cli | 177 | 35 |
| model | 165 | 8 |
| workspace | 164 | 30 |
| identity | 109 | 10 |
| languages | 93 | 6 |
| tickets | 90 | 30 |
| hooks | 85 | 14 |
| todos | 84 | 15 |
| statutes | 44 | 2 |
| tree | 42 | 14 |
| graphql | 34 | 0 |
| mcp | 24 | 5 |
| testrunner | 24 | 15 |
| providers | 15 | 14 |
| goals | 8 | 1 |
| contributors | 7 | 0 |
| move | 5 | 3 |
| events | 2 | 0 |
| codebase | 1 | 1 |
| metrics | 1 | 0 |

Most common multi-module combinations (from `test-module-refs.json`): `{identity, model}` — 59 tests (these are the artifact-ID/entity-rendering tests and should split into `identity`'s and `model`'s own `🧪️tests/`, likely duplicated as two Protocol-v2 cases sharing fixtures); `{cli, hooks, todos}` — 39 tests (hook-result/todo-event plumbing, a good candidate for a `hooks`+`todos` shared fixture rather than 3-way); `{cli, languages, model, statutes, workspace}` — 18 tests (the `analyze`/policy pipeline end-to-end tests — keep as one cross-module scenario in `🔨️modules/🧪️test`, do not force-split). Practical rule for wave 2: any test whose module set is a singleton (rightmost column) moves verbatim into that module's `🧪️tests/`; anything touching `cli` plus exactly one domain module is almost always CLI-command-wiring around that module's logic and should become a thin CLI-level scenario that calls the domain module directly instead of asserting on command output; the `{identity, model}` and `{cli, hooks, todos}` clusters are the two largest cross-cutting groups worth a dedicated shared fixture set.
