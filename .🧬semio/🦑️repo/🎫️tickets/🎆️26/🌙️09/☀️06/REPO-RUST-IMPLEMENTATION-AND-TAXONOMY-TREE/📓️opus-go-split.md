# 📓️ Go Split — execution report

Executor: Opus 5 (resumed job; the first executor was killed by a rate limit right after its emit
pass, before any verification).

Contract: `📋️go-split-brief.md`. Graph: `📓️go-region-dependency-graph.md`. Tool: `🔨️go-split/`.

## 1. State found on resume

- `🔨️go-split/` had emitted every package once (all `🐹️.go`, `🔬️_test.go`, `🔭️exhaustive_test.go`
  stamped 06:16); nothing had been compiled.
- `go.work` already listed the new modules and no longer listed `💻️client/⌨️cli`.
- `⌨️cli` did not compile (`undefined: Command`, `EngineCommand redeclared`), `🔌️mcp` still imported
  `repo/client`, and the whole tree had never been vetted or tested.
- No `📊️Pending` regions had been actioned, no harness `// 🚚️ repoint` marker had been actioned, the
  old `💻️client/⌨️cli` Go module was still present, and the build wiring still pointed at
  `💻️client/⌨️cli/cmd/repo`.

Mid-resume accident and recovery: an inline heredoc truncated `🔨️go-split/🔣️symbol-overrides.json`
to zero bytes. It was **reconstructed, fully and explicitly, from the tool's own
`🗑️generated/go-split/decls.jsonl`** (the assignment the previous emit actually produced): every
top-level declaration and every method now names its target module by name in `symbols`, the seven
godfile `init()` functions by line range in `lineOverrides`, and every test function by name in
`testOverrides`. The reconstruction was validated by diffing the declaration-name set of every
emitted package against a byte snapshot of the 06:16 tree: **the only difference was the intended
`Command` rename**. The config is therefore stricter than the one it replaces — region tables and
prefix rules still run first, but nothing is left implicit.

## 2. Splitter defects found and fixed

All five were real code-generation bugs, not configuration mistakes.

1. **Cross-package name collision.** `byName` was keyed by bare name, so the godfile's
   `type Command string` (engine request) and `internal/command`'s `type Command struct` (the
   cobra-like framework) collided; both were renamed to `EngineCommand` and `⌨️cli` could not
   compile. Fixed with a per-source-file symbol index (`byFile` + `lookup()`), file-qualified rename
   keys (`🎮️command.go:Command` → `Command`, bare `Command` → `EngineCommand`) and file-qualified
   usage sets, plus deterministic (sorted, line-offset) ingestion of the four `internal/` packages.
2. **Self-referential types were never rewritten.** `collectLocals` treats a `GenDecl`'s own
   `TypeSpec` name as a local, so `type Foo struct { parent *Foo }` had its inner `*Foo` skipped.
   `renderUnit` now removes the unit's own declared names from the local set.
3. **Nested composite-literal keys were wrongly skipped.** `buildSkipSet` re-visited every inner
   `CompositeLit` with `structish=true`, so `map[string]map[providers.McpClientKind]string{…}`'s inner
   keys were treated as struct fields and left unqualified (`McpClientGeneric` instead of
   `providers.McpClientGeneric`). Inner literals are now marked and skipped by the outer walk.
4. **Shadowed import aliases were rewritten as packages.** The pre-split godfile imports
   `repo/workspace` twice, as `glob` and as `ignore`; a `for _, ignore := range …` loop variable then
   shadows the alias, and the emitter turned `ignore.Versions` into `workspace.Versions`. The
   selector branch of `renderUnit`/`renderTestDecl` and `declRefs` now consult the local set first.
5. **Test emission used the package name, not the alias**, so a test whose local variable shadowed a
   package name (`events, err := (eventstore.Store{…}).Replay(…)`) emitted `events.Store` with no
   usable import. `renderTestDecl` now uses the same alias table as the production emitter.

Two further emission fixes:

- The exhaustive file duplicated every helper that the plain file already declared, so
  `go vet -tags exhaustive` failed with `redeclared in this block` in four packages. `🔭️exhaustive_test.go`
  now carries only the `TestExhaustive*` functions; the helpers stay in `🔬️_test.go`, which is
  compiled in both configurations.
- `projectJSON`/`scriptTS` emitted a project named `framework-products-repo-modules-<x>-go` and a
  script importing a non-existent `GoPackageScript`. Both templates now match the wave-1 convention:
  `@semio-tech/repo-<x>-go` with `test` / `test-quick` / `test-long` / `test-exhaustive`, and a
  `BundleScript` router that adds `-tags exhaustive` at the exhaustive level.

## 3. DAG fixes beyond the brief's list

The brief's seven binding fixes were already encoded by wave 1 and are preserved. One more was
required, found only by running the tests:

- **`🎁️Templates` + `🧊️ANSI` + `🪄️Template Functions` → `📐️model`.** `TextTpl`/`MdTpl` sat in
  `🏠️workspace` while the template text, `templateFuncMap`, `initTemplates` and its `init()` sat in
  `⌨️cli`. Every package below `⌨️cli` that renders (`📐️model.RenderEntityMarkdownLink`,
  `🌳️tree`) therefore dereferenced a nil `*template.Template` unless `⌨️cli` happened to be linked in
  — `TestRenderEntityMarkdownLink_AllKinds` panicked in `📐️model`. The whole template machinery
  (`TextTpl`, `MdTpl`, `RenderTemplate`, `markdownTemplateContent`, `textTemplateContent`,
  `templateFuncMap`, `colorNameToANSI`, `initTemplates`, the `init()`, the seven `Color*` constants,
  `colorize`, and `TxtFuncMap`/`empty` from `internal/templatefunc`) now lives in `📐️model`, which is
  the lowest level that can see both `model.SanitizeProp` and `workspace.PathToUriPath`.
  `⌨️cli` and `🌳️tree` call `model.RenderTemplate(model.MdTpl, …)` and `model.Colorize(…)`.

**Final verdict — `📊️go-split-scc-analysis.json` (re-run of the graph tool over the output
assignment): `{"cycles": [], "violations": []}`.**

### Module graph (2 368 declarations)

| module | level | decls | imports |
| --- | --- | --- | --- |
| workspace | 0.2 | 116 | — |
| events | 1.1 | 4 | — |
| model | 0.3 | 639 | identity, search, workspace |
| languages | 1.0 | 135 | model, yaml |
| providers | 1.2 | 202 | model, workspace |
| statutes | 2.0 | 95 | languages, model, workspace |
| codebase | 2.1 | 60 | languages, model, statutes, workspace, yaml |
| move | 3.0 | 22 | codebase, events, languages, model, workspace |
| contributors | 3.1 | 29 | codebase, identity, languages, model, workspace |
| goals | 3.2 | 15 | model, providers, workspace |
| tickets | 3.3 | 104 | codebase, contributors, events, goals, languages, model, move, providers, search, workspace |
| todos | 3.4 | 28 | codebase, model, move, tickets, workspace |
| testrunner | 3.5 | 57 | codebase, languages, model, statutes, tickets, todos, workspace |
| hooks | 3.6 | 69 | codebase, contributors, languages, model, providers, statutes, testrunner, tickets, todos, workspace |
| tree | 3.7 | 105 | codebase, contributors, goals, languages, model, search, statutes, tickets, todos, workspace |
| graphql | 4.0 | 214 | codebase, contributors, events, goals, languages, model, move, providers, statutes, tickets, todos, workspace |
| cli | 5.1 | 474 | every level below it |

`yaml`, `search`, `identity` and `metrics` received no declarations: wave 1 had already written them
in full, and the emitter's `dropped` pass (10 symbols, all `🏠️workspace` repo-config) prevents
redeclaration.

Exported renames performed by the split: **175** (unexported symbols that acquired a cross-package
caller), plus the one forced rename `Command` → `EngineCommand`.

## 4. Per-package result

`GOWORK=C:/git/semio/go.work`, `go build ./...`, `go vet ./...`, `go test -count=1 ./...` in each
package directory.

| package | `🐹️.go` | `🔬️_test.go` | `🔭️exhaustive_test.go` | build | vet | test |
| --- | ---: | ---: | ---: | --- | --- | --- |
| ⌨️cli | 10 289 | 4 318 | 2 197 | ok | ok | fail (pre-existing) |
| 🌳️tree | 2 878 | 2 003 | 708 | ok | ok | fail (pre-existing) |
| 🎫️tickets | 3 546 | 830 | 116 | ok | ok | **ok** |
| 🎯️goals | 266 | 77 | — | ok | ok | fail (pre-existing) |
| 🏃️test-runner | 1 879 | 689 | — | ok | ok | fail (pre-existing) |
| 🏠️workspace | 1 645 | 400 | — | ok | ok | fail (pre-existing) |
| 📊️metrics | 1 502 | — | — | ok | ok | fail (pre-existing) |
| 📐️model | 7 040 | 2 908 | — | ok | ok | fail (pre-existing) |
| 📜️statutes | 3 392 | 1 592 | — | ok | ok | fail (pre-existing) |
| 📝️todos | 841 | 827 | — | ok | ok | fail (pre-existing) |
| 📡️events | 1 036 | — | — | ok | ok | **ok** |
| 🔌️mcp | 2 085 | — | — | ok | ok | **ok** |
| 🔎️search | 589 | — | — | ok | ok | **ok** |
| 🔗️graphql | 8 370 | 2 089 | 979 | ok | ok | fail (pre-existing) |
| 🗂️codebase | 1 711 | 280 | — | ok | ok | fail (pre-existing) |
| 🗣️languages | 3 395 | 98 | — | ok | ok | fail (pre-existing) |
| 🚚️move | 818 | 210 | — | ok | ok | fail (pre-existing) |
| 🧑️contributors | 856 | 161 | — | ok | ok | **ok** |
| 🧩️providers | 2 119 | 757 | — | ok | ok | **ok** |
| 🧾️yaml | 485 | — | — | ok | ok | **ok** |
| 🪝️hooks | 2 041 | 3 919 | — | ok | ok | fail (pre-existing) |
| 🪪️identity | 576 | — | — | ok | ok | **ok** |

(Line counts of `🐹️.go` include each package's wave-1 hand-written part; packages with no
`🔬️_test.go` row carry a wave-1 `🧪️_test.go` instead, which is included in the `go test` result.
`🧪️test/📦️packages/🐹️go` is `semio.tech/repo/test`, the harness host, deliberately outside `go.work`.)

`go vet -tags exhaustive ./...` is clean in the four packages that own `🔭️exhaustive_test.go`
(`⌨️cli`, `🌳️tree`, `🎫️tickets`, `🔗️graphql`); those files carry `//go:build exhaustive` and are routed
to the `test-exhaustive` script level.

### "fail (pre-existing)" — the parity proof

The old module could no longer be built standalone (wave-1 agents had already repointed it at the new
packages), so a baseline was taken with a temporary workspace file that re-added
`💻️client/⌨️cli`:

```
GOWORK=<work-with-client> go test -count=1 -skip 'TestExhaustive' .   → 75 distinct failing tests
```

The same measurement over the split tree:

```
for each package: GOWORK=go.work go test -count=1 ./...               → 75 distinct failing tests
comm -13 old-fails new-fails                                          → (empty)
```

**Zero regressions: the set of failing test names after the split is identical to the set before
it.** The 75 are wave-1 divergences (`FileHeaderId` emoji vocabulary, `fix was removed`,
`newline-after-region`, technology-catalog lookups, live-repo fixtures, …) and are not this job's.

Regressions that *were* introduced by the split and are now fixed:

- `TestG1*` (8 tests) read `🧫️fixtures/1️⃣g1-contract.json` relative to the old module. The fixture
  moved to the shared `📚️library/🧪️tests/1️⃣g1-contract/🧫️fixtures/🔣️.json`, because the suite is
  now split across `⌨️cli` and `🌳️tree` and a per-package copy would be a duplicate.
- `TestPathEmojiStatutesLanguageNeutralFixture` walked `../../📚️library/…`; every split target sits
  one level deeper, so the source path became `../../../📚️library/…`.
- `TestRenderEntityMarkdownLink_AllKinds` — the nil-template layering defect (§3).
- `TestTrackHookInOpenTicketUsesStableSessionIDs` panicked in `🪝️hooks` because
  `tickets.OpenGoal` goes through the `model.LookupRepoContext` port that only `🔗️graphql`'s `init()`
  installs. The tool ranks a test by the level of the symbols it *names*, and cannot see a runtime
  port dependency; the test is now pinned to `⌨️cli` (`testOverrides`), the topmost module, which
  links the whole stack.
- `plan.binary` — the pre-split test file was already stale against its own godfile (a wave-1 agent
  had exported `formatterPlan.Binary` without updating the test); corrected at the source before the
  final emit.

## 5. Harness Go adapters

Every `// 🚚️ repoint … after split` marker is gone (`grep -rn "🚚️ repoint" 🔨️modules` → nothing).
13 adapters were repointed off `github.com/usalu/semio/repo/client`; five needed more than a path
swap because the symbols did not land where the marker guessed:

| adapter | marker said | actual home |
| --- | --- | --- |
| `🏃️test-runner/🧪️tests/🧬️scope-identifier-normalisation` | testrunner | `workspace.Flat`, `workspace.PathFromUriPath` |
| `🗣️languages/🧪️tests/💥️malformed-regions` | languages | `languages.ParseSections` + `model.Section` |
| `🗣️languages/🧪️tests/📑️section-parsing` | languages | `languages.Parse*` + `model.Section` |
| `🗣️languages/🧪️tests/🏷️scope-ids` | languages | `languages.BuildScopeID`, `languages.BuildScopesForFile` (it was importing `events`) |
| `📐️model/🧪️tests/🔣️json-encoding-conformance` | model | all of `model.*` plus `tree.TreeNode` |
| `🧩️providers/🧪️tests/🪝️editor-hook-output-format` | providers | `model.HookEvent/ToolKind/HookResultBase/AllHookEvents` + `providers.EditorProvider` |

`SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts subject fundamental --owner <owner> --implementation go`,
every owner that has a Go adapter:

```
🌳️tree          cases=5 executed=0  passed=0  failed=0 not-exercised=5
🎯️goals         cases=4 executed=0  passed=0  failed=0 not-exercised=4
🏃️test-runner   cases=5 executed=2  passed=2  failed=0 not-exercised=4
🏠️workspace     cases=5 executed=6  passed=6  failed=0
📊️metrics       cases=4 executed=9  passed=9  failed=0
📐️model         cases=3 executed=6  passed=6  failed=0
📜️statutes      cases=5 executed=4  passed=3  failed=1 not-exercised=3
📝️todos         cases=4 executed=0  passed=0  failed=0 not-exercised=4
📡️events        cases=4 executed=9  passed=9  failed=0
🔌️mcp           cases=4 executed=4  passed=4  failed=0 not-exercised=2
🔎️search        cases=1 executed=1  passed=1  failed=0
🔗️graphql       cases=8 executed=4  passed=4  failed=0 not-exercised=4
🗂️codebase      cases=4 executed=8  passed=8  failed=0
🗣️languages     cases=5 executed=12 passed=12 failed=0
🚚️move          cases=5 executed=12 passed=12 failed=0 not-exercised=1
🧑️contributors  cases=3 executed=0  passed=0  failed=0 not-exercised=3
🧩️providers     cases=4 executed=7  passed=7  failed=0 not-exercised=2
🧪️test          cases=1 executed=2  passed=2  failed=0
🧾️yaml          cases=2 executed=2  passed=2  failed=0
🪪️identity      cases=2 executed=2  passed=2  failed=0
──────────────────────────────────────────────────────────────────
                        executed=96 passed=95 failed=1
```

The one failure — `📜️statutes::🙈️ignore-directives::a-directive-only-reaches-forward` — is **not**
a split defect. Its `🥒️.feature` and `🦀️.rs` were written at 05:56 by the concurrent statutes agent;
`statutes.ParseIgnoreDirectives` and `NewPolicyContextWithFiles` sit inside the `🚚️Split` region,
i.e. they are the pre-split godfile behaviour moved verbatim. The frozen contract expects a
directive window the Go implementation has never had; closing that gap is the statutes agent's job.

Four owners (`🌳️tree`, `🎯️goals`, `📝️todos`, `🧑️contributors`) still cannot compile their Go hosts,
for the same reason and equally outside this job: their adapters are written against APIs their
owning agents have specified but not yet implemented (`tree.ParseTreeNodeSpec`,
`tree.MemoryTreeSource`, `goals.NewGoals`, `todos.NewFsTodoTree`,
`contributors.NewMemoryCheckpointSource`, …). None of those names exists anywhere in the repository,
so nothing was moved away from them by the split.

## 6. Wiring

- Old `💻️client/⌨️cli` Go module **deleted**: `🧩️component.go`, `🔬️component_test.go`,
  `🤝️g1_contract_test.go`, `📤️event_export.go`, `cmd/`, `internal/`, `go.mod`, `🧫️fixtures/`.
  `💻️client/⌨️cli/📦️packages/🟦️typescript`, `💻️client/🧩️vscode`, `💻️client/🪶️sqlite` and
  `💻️client/📜️script.ts` are untouched (the wiring agent owns their move). `go.work` already omitted
  the module.
- `🔌️mcp/📦️packages/🐹️go` builds, vets and tests green against `repo/cli` + the domain packages.
- The binary is built from `⌨️cli/📦️packages/🐹️go/🚀️bin` to
  `⌨️cli/📦️packages/🐹️go/semio-repo[.exe]`, and the path is wired in four places: root
  `📜️script.ts` (`REPO_CLIENT_GO`, `REPO_CLI_ENTRY_GO`; the now-unused `REPO_CLIENT_DIR` was
  removed), `📚️library/…/🟦️.ts` `defaultCliBin()`,
  `💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` (`dev` + `build`), and `.gitignore`.
- `.vscode/🧩️launch.seed.jsonc`: `🛠️dev🧰️repo⌨️client` rebuilt against the new entry point; the
  seven `framework-products-repo-modules-*-go:test` commands renamed to `@semio-tech/repo-*-go:test`;
  `🧪️test🧰️repo🎫️tickets🐹️go` and `🧪️test🧰️repo⌨️cli🐹️go` added. `.vscode/launch.json`
  regenerated with `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`.
  `bun nx run @semio-tech/repo-cli-go:test` runs the package suite.
- All ten tool-created packages got a wave-1-shaped `📋️project.json` + `📜️script.ts`, and
  `🔗️graphql`'s script gained the exhaustive build tag.

## 7. Deviations and open items

1. **The `📊️Pending` region of `📊️metrics` was deleted without moving its symbols.** It asked the
   split to *delete* the godfile's 57 `loc*` helpers and the three benchmark parsers on the grounds
   that `📊️metrics` already carries exported, git-free reimplementations of them. Those helpers stayed
   in `⌨️cli`, where the split placed them, because (a) `locCommand`/`runLocCommand`/`benchmarkCmd`
   call them and the metrics reimplementation deliberately changed shape — gitignore behind an
   `Ignorer`, git behind `SystemGit`, the contributor lookup behind an `AliasFunc` — so rewiring the
   command is a re-architecture, not a move; and (b) 25 passing `⌨️cli` tests assert directly on the
   unexported helpers and have no counterpart against the ported API. The brief scopes this job to
   "no behaviour changes except the DAG fixes", and `⌨️cli` is the top of the ladder, so the
   duplication costs nothing structurally. **Follow-up ticket material:** port `runLocCommand` and
   `benchmarkCmd` onto `📊️metrics`' ports and delete the `⌨️cli` copies with their tests.
2. **`📜️statutes::🙈️ignore-directives` fails against its own frozen contract** (§5) — statutes agent.
3. **`🌳️tree`, `🎯️goals`, `📝️todos`, `🧑️contributors` Go hosts do not compile** (§5) — their owning
   agents' unimplemented APIs.
4. **75 pre-existing Go test failures survive the split unchanged** (§4). They were failing before it
   and are wave-1 behavioural divergences, not split damage.
5. `⌨️cli/📦️packages/🐹️go/🐹️.go` is 10 289 lines. The `repo-client-cli-main-go` lint in
   `💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` budgets 10 000 lines for a file named
   `🐹️.go`; that package is the wiring agent's to move, so the budget was left alone. Splitting
   `⌨️cli` further (command wiring vs. renderers vs. MCP handler surface) is the natural next cut.

## 8. Artifacts kept

- `🔨️go-split/` — `🏗️main.go`, `🧩️parse.go`, `🚚️emit.go`, `🔬️tests.go`, `🧹️prune.go`, `✍️write.go`,
  `go.mod`, and the reconstructed `🔣️symbol-overrides.json` (2 361 symbols + 7 line overrides +
  585 test overrides + 2 renames). Re-runnable only while a copy of the pre-split godfile exists.
- `📊️go-split-scc-analysis.json` — the zero-violation, zero-cycle verdict of the graph tool over the
  output assignment.
