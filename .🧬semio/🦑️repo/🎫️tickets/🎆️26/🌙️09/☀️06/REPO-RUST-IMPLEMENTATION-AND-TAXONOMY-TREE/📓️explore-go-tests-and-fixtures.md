# 📓️ Go test/fixture landscape — `⌨️cli`, `🔌️mcp`, `🎛️coordinator`

Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli` (`🔬️component_test.go` ~23.9k lines / 568
`func Test`, `🤝️g1_contract_test.go`, `internal/*_test.go`, `🧫️fixtures/`), `💻️client/🔌️mcp`
(`🧪️contract_test.go`, `🧫️fixtures/`, `🔗️graphql/`), `🖥️server/🎛️coordinator/*_test.go` +
`🧫️fixtures/`, and the product-level `🖼️assets/🧫️fixtures/`. Cross-referenced against the shared
language-agnostic harness at `🔨️modules/🧪️test/`.

## 1. Inventory — `🔬️component_test.go` (568 funcs), by domain

| Domain group | # tests | line range | Notes |
|---|---|---|---|
| Other (ungrouped — see below) | 162 | 985–23709 | hooks, artifact IDs, mermaid, git providers, plan extraction |
| Graph node/edge/definition/section/query | 86 | 1342–23570 | `NodesAndEdges`, `Definitions`, `Sections`, `Kind`, GraphQL-backed |
| Ticket domain | 48 | 726–23747 | ticket ↔ goal `ComposeID` round trips, ticket listing/validation |
| Event/EventStore | 44 | 4103–22455 | `internal/eventstore` behaviour exercised from the CLI |
| Policy/Statute/Breach | 42 | 1008–15550 | registry non-emptiness + lint-breach plumbing |
| Fix pipeline (autofix/comment/formatter) | 30 | 2135–23231 | per-language comment scanning + autofix |
| Command (CLI wiring) | 27 | 4016–23596 | cobra-style command dispatch, JSON/markdown output shaping |
| Goal domain | 25 | 668–21958 | goal path ↔ compose-id |
| Bundle/Folder/File registries | 22 | 916–21874 | "exhaustive" non-empty checks over generated registries |
| File/Workspace/Ignore/Glob | 21 | 1068–22785 | ignore-pattern matching, ticket workspace file filtering |
| Contributor domain | 14 | 220–21855 | contributor discovery / compose-id |
| Repo meta | 10 | 38–22717 | repo root discovery, devcontainer bootstrap, mcp asset paths |
| Analyze/Codebase/Cache | 9 | 2063–15453 | breach-cache JSON read/analyze commands |
| File-header id / FileKind | 7 | 2145–11577 | emoji/id derivation for file headers |
| GraphQL | 4 | 3554–23808 | schema/executor plumbing |
| Search | 4 | 13363–22732 | in-repo search index |
| MCP | 3 | 11613–23613 | mcp bootstrap asset paths |
| Interaction/Author | 3 | 179–14986 | interaction JSON author-shape unmarshalling |
| Devcontainer/Bootstrap | 2 | 304–522 | postAttach / GitKraken workspace bootstrap |
| Template | 2 | 15399–16336 | `text/template` func coverage |
| Sync / Yaml-Config | 2 | 1978 / 22151 | GH sync command, yaml parsing |

The 162-item "Other" bucket is itself several coherent sub-domains worth calling out explicitly
(sample line numbers): **Claude/agent hook plumbing** (`TestRunHookToolBlocking` 15969,
`TestClassifyTool` 18646, `TestExtractToolInputFromStdin` 20929, ~35 tests, 15730–21624);
**artifact ID/URI derivation** (`TestGetArtifactID_*` 6589–7192, `TestSpecExactIDs` 7368,
`TestExhaustiveMonorepoTreeEntityIDs` 7542, ~25 tests); **monorepo/checkpoint tree rendering**
(`TestFilterMonorepoTree` 13187, `TestExhaustiveBuildMonorepoTree` 13655, `TestSortTreeChildren`
13900); **markdown/JSON MCP-result formatting** (`TestFormatMarkdownResult_*` 9488–9857,
`TestExhaustiveMarkdownOutput` 10039); **git version-control provider** (`TestGitVersionControlProvider*`
22288–22413, real `git init`/`clone`/`checkout` in temp dirs); **plan/session extraction**
(`TestExtractPlanStepsFromInput` 20488 and ~12 siblings). `🤝️g1_contract_test.go` adds 18 more
`func Test` (641 lines total) that are ALL fixture-driven (see §2).

`t.Run(` sub-cases are pervasive inside the table-driven tests above (not separately counted; the
`func Test` count already reflects one entry point per behaviour cluster — sub-case names are ad hoc
strings, not a stable machine-readable identifier).

## 2. Fixture-driven vs. golden-file vs. pure in-code, per group

| Group | Driven by | External process? | Real repo tree or temp dir? |
|---|---|---|---|
| `🤝️g1_contract_test.go` (18 tests) | **Fixture-driven**: single `🧫️fixtures/1️⃣g1-contract.json`, unmarshalled into a typed `g1Fixture` struct, asserted against `internal/command`, `internal/glob`, `internal/search`, `internal/eventstore`, `internal/templatefunc`, `internal/yaml` | none | none — pure in-process |
| Exhaustive registry tests (Bundle/Folder/File/Policy/Statute/Breach/Contributor/Ticket) | **In-code**, but query the **real repo** through a live `Executor`/GraphQL layer (`getTestExecutor` → `findTestRepoRoot(cwd)` walks up to the actual monorepo root) | none | **real repo tree**, not a fixture/temp dir |
| Fix/Scan/Comment/Formatter pipeline | **In-code table-driven** (Go string literals as before/after) | none | none (in-memory strings) — `TestFixCommand` itself now just asserts the command errors ("fix was removed"), i.e. gutted |
| EventStore | **In-code** table-driven against `internal/eventstore` | none | temp dirs (`t.TempDir()`) |
| Command / MCP-result formatting | **In-code**, large literal expected-JSON/markdown strings embedded in the test | none | none |
| Hook plumbing (agent.tool.* ) | **In-code**, some spawn the **built CLI binary itself**: `exec.CommandContext(ctx, "./cli", "hook", ...)` (line 18578) | yes — spawns `./cli` | temp dirs |
| Git version-control provider | **In-code**, drives **real `git`** via `exec.Command("git", …)` (init/clone/checkout/rev-parse, lines 16339, 21516–21675, 22167–22421) | yes — real `git` binary | temp dirs (throwaway repos) |
| Devcontainer/GitKraken bootstrap | **In-code** | no (asserts on generated shell/compose content) | none |
| `bun`/mcp entrypoint dev command | **In-code**, spawns `exec.CommandContext(ctx, "bun", "./📜️script.ts", "dev", "mcp", "stdio", "cursor")` (line 23620) | yes — real `bun` | n/a |
| `internal/command/🧪️command_test.go` (5 funcs), `internal/eventstore/🧪️eventstore_test.go` (1 func) | **In-code**, no fixtures/golden files referenced | none | temp dirs |

Aggregate signals across `🔬️component_test.go`: `fixtures` appears only twice, both as a **local Go
variable name**, never a file/dir reference — i.e. **no fixture directory is used by
`component_test.go`**. `t.TempDir()` appears 193×. No `golden` string anywhere. `exec.Command`/
`exec.CommandContext` appear 20× (git, the built `./cli`, and `bun`).

`💻️client/🔌️mcp/🧪️contract_test.go` (20 `func Test`, g2-prefixed) is **fixture-driven** the same way
as g1: `🧫️fixtures/🚪️entrypoint-contract.json` (server profile flow) and
`🧫️fixtures/2️⃣g2-contract.json` (JSON-RPC request/response vectors keyed by scenario `name`), loaded
via `os.ReadFile("🧫️fixtures/...")` and unmarshalled into typed structs — e.g.
`TestG2CanonicalGoldenVectors` (line 142), `TestRepositoryEntrypointContract` (line 55). A handful of
G2 tests (saturation, cancellation, reconnect) layer in-process pipe/mutation scenarios on top of the
fixture-declared protocol version.

`🖥️server/🎛️coordinator` (`🗂️g3_filesystem_test.go` 4 funcs, `🧪️g3_event_store_test.go` 20 funcs,
all G3-prefixed) is **fixture + golden-log driven**: `🧫️fixtures/🧬️g3-event-schema.json` declares the
canonical JSONL encoding/checksum contract (`"schema":"semio.coordinator.event/1"`,
`"encoding":"canonical-jsonl-lf"`, `"checksum":"sha256(stream+nul+sequence+nul+id+nul+generation+nul+type+nul+canonical-payload)"`),
and `🧫️fixtures/📜️g3-event-log.jsonl` is a golden append-log used e.g. by
`TestG3LanguageNeutralGoldenEnvelope` (line 96). The filesystem-fault tests
(`TestG3FaithfulFilesystemFaultsEveryPrecommitOperation` etc.) inject a fake/faulty filesystem
in-code rather than reading fixtures.

## 3. Fixture directory catalogue

| Dir | Files | Format | How located |
|---|---|---|---|
| `⌨️cli/🧫️fixtures/` | `1️⃣g1-contract.json` (44 lines) | single JSON contract, sections: `command`, `glob`, `template`, `search`, `eventStore`, `yaml` | relative path constant `filepath.Join("🧫️fixtures","1️⃣g1-contract.json")` in `loadG1Fixture` |
| `🔌️mcp/🧫️fixtures/` | `2️⃣g2-contract.json`, `🚪️entrypoint-contract.json` | JSON, `vectors[]` of `{name, ready, request, response}` strings (JSON-RPC payloads as escaped strings); entrypoint file lists `profiles[]`, `protocolVersions[]`, `flow[]` | `os.ReadFile("🧫️fixtures/...")` relative to package dir |
| `🎛️coordinator/🧫️fixtures/` | `🧬️g3-event-schema.json`, `📜️g3-event-log.jsonl` | single-line JSON schema descriptor + JSONL golden event log | relative path from test file (coordinator package dir) |
| `🖼️assets/🧫️fixtures/📁️some/📁️folder/` | `🟦️.tsx`, `🧪️file/{🐍️.py,🔷️.cs,🟦️.tsx}`, `🧪️file-fixable/🟦️.tsx`, `🧪️file-fixable-expected/🟦️.tsx`, `🧪️file-fixed/{🐍️.py,🐹️.go,🔷️.cs,🟦️.tsx}`, `🧪️file-invalid/{🐍️.py,🐹️.go,🔷️.cs,🟦️.tsx}` | raw source snippets per language, directory-per-case rather than golden files | **not referenced by any `.go`/`.ts`/`.py`/`.cs` file found repo-wide** — appears orphaned |

Naming convention: every fixture/test directory uses an emoji prefix that encodes its *kind*
(`🧫️fixtures`, `🧪️file*` for a per-language test case, `📁️` for a plain folder, numeric-glyph
prefixes `1️⃣`/`2️⃣` disambiguating the "gN" contract generation, `🥒️.feature`/`🐹️.go`/`🦀️.rs`/`🟦️.ts`
for language-tagged siblings under `🧪️tests/<slug>/`). The `file-fixable` /
`file-fixable-expected` / `file-fixed` / `file-invalid` naming under `🖼️assets/🧫️fixtures` is exactly
the input/expected/output-golden convention the fix pipeline *should* use, but `TestFixCommand` in
`🔬️component_test.go` (line 2135) currently only asserts `ToolFix` returns
`"fix was removed"` — the golden fixtures for the fix pipeline exist but the test that would consume
them has been gutted, and no other test path grep-matches this fixture tree. **Recommend flagging
this fixture set as dead/orphaned or wiring it back to whatever now performs "fix".**

## 4. Third-party cross-validation pattern in this repo, and whether Go tests follow it

The repo's real pattern lives entirely in `🔨️modules/🧪️test/` (see §5): every runtime mutation must
have a **qualifying third-party oracle** (`third-party-library | third-party-cli |
standards-reference-tool`), counted per **engine family**, with a **supplementary**
`cross-semio-implementation` (a second in-repo implementation from the same schema) required but
never substitutable for the third-party oracle. This is enforced by `🔣️oracle.json` per owner +
`bun ./📜️script.ts oracle|parity|dependency`.

**None of the Go test groups under `⌨️cli`, `🔌️mcp`, `🎛️coordinator` follow this pattern.** They are
pre-Protocol-v2 Go-native tests: no `🔣️oracle.json`, no registry entry, no external reference
library invoked for comparison anywhere in `component_test.go`, `g1/g2/g3 contract_test.go`, or the
coordinator tests (`grep -c golden` = 0, no `oracleHostPackages` references). The `gN-contract.json`
fixtures are internally-authored expected values (hand-written JSON-RPC vectors / hand-computed
checksums), not verified against any third-party MCP/JSON-RPC/event-sourcing library. This is the
single biggest gap for the Rust-parity effort: fixtures exist and are language-agnostic in *shape*,
but none of them have been oracle-validated per Protocol v2, and `component_test.go`'s 568 functions
are almost all pure Go closures with no data file at all.

## 5. Shared language-agnostic harness — `🔨️modules/🧪️test/`

Layout (from `README.md`, protocol v2):
```
<owner>/
├── 🧫️fixtures/              immutable, shared by every case of this owner
├── 🧪️tests/<kebab-case>/
│   ├── 🧫️fixtures/          immutable, private to this case
│   ├── 🥒️.feature          normative, language-neutral contract (Gherkin)
│   ├── 🦀️.rs / 🟦️.ts / 🐹️.go / 🐍️.py / 🔷️.cs   one adapter per implementation
└── 📦️packages/<language>/   the implementations under test
```
Concretely for the one worked example, `🖥️host-protocol-parity`:
- `🧪️tests/🖥️host-protocol-parity/🥒️.feature` — tags `@capability-test-host-protocol`,
  `@no-oracle-repo-test-platform`, `@comparison-ordered-json-v1`; 3 scenarios, each tagged
  `@id-…`, exactly one `@level-…` (`fundamental`/`quick`), exactly one `@mode-…`
  (`differential`/`error`/`conformance`).
- `🐹️.go`, `🦀️.rs`, `🟦️.ts`, `🐍️.py`, `🔷️.cs` — five **independently written** adapters
  (package `adapter`) implementing `host.Adapter` with `.Subject("<scenario-slug>", fn)`
  entries whose function bodies read the SAME shared fixture
  (`shared://📡️protocol-vector.txt`, stored once at `🧪️test/🧫️fixtures/📡️protocol-vector.txt`)
  and project a `host.Outcome{Projection: map[string]any{...}}`.
- Because no third party implements this repo's own host protocol, this case is registered as a
  **`noOracleDecisions`** entry (`id: "repo-test-platform"`) in `📇️registry/🔣️.json` instead of an
  oracle — confidence instead comes from **pairwise equivalence** across the five adapters
  (`parity` compares them to each other, not to an oracle).
- `🧬️schema/🔣️.json` is the JSON-Schema ($id
  `https://semio-tech.com/schema/repo/test/v2`) that types every artifact in the pipeline:
  `TestLevel` (`fundamental|quick|long|exhaustive`), `TestMode`
  (`differential|conformance|round-trip|property|error`), `Implementation`
  (`rust|typescript|go|python|dotnet`), `OracleRegistryEntry` (kind/ecosystem/package/engine/
  capabilities/comparisonProfiles/license/testOnly/productionReachable), `ProbeRegistryEntry`,
  `MutationOutcomeClass` (`applied|no-op|empty|disjoint|rejected`), etc.
- `📇️registry/🔣️.json` is the **framework-owned, domain-neutral** registry: only
  `noOracleDecisions` at the platform level; every domain-specific oracle/probe lives in that
  domain's own `🔣️oracle.json` discovered by directory convention.
- `📜️script.ts` (per `README.md`) exposes: `discover | doctor | contract | oracle <level> |
  subject <level> | parity <level> | run <level> | report | clean | dependency | inventory |
  fixture <sub> | probe | matrix | gc`. `--owner/--case/--project/--implementation` narrow scope.
- `📋️project.json` (Nx) wires `test-discover`, `test-contract`, `test-oracle`, `test-subject`,
  `test-parity`, `test-report`, `test-clean`, `test-dependency`, each an `nx:run-commands` calling
  `bun ./📜️script.ts <phase>`; every discovered case additionally gets generated Nx targets
  (`test`, `test-quick`, `test-long`, `test-exhaustive`, `test-contract`, `test-oracle`,
  `test-subject`, `test-parity`).
- `.vscode/launch.json` registers the gate/consumer entries for this platform at lines 8415–8470+
  (`⚖️gate🧪️test🔍️discover`, `🩺doctor`, `🧾️contract`, `🏭️inventory`, `🔬️probe`,
  `🧫️fixture-verify`, …) plus per-product test launch groups (e.g. lines 7616–7756 for the print/viz
  product's `fundamental/quick/long/exhaustive` levels) — these follow the same `bun nx run
  <project>:test-<phase>` shape.
- Exact commands to run the parity example:
  `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts discover --case host-protocol-parity`
  then `... contract --case host-protocol-parity`, `... oracle --case host-protocol-parity` (no-op:
  `noOracleDecisions`), `... subject --case host-protocol-parity --implementation go|rust|...`,
  `... parity --case host-protocol-parity` (cross-checks all 5 adapters against each other).

**How a test is declared once, run against N implementations**: (1) the `.feature` file is the single
normative spec, scenario-tagged with id/level/mode; (2) fixtures referenced as `shared://` or
`local://` URIs are resolved by each host's own `host.Context.Fixture(...)`, never a hardcoded path;
(3) each language contributes exactly one adapter file registering `.Subject(scenario-slug, fn)`
against the SAME scenario ids from the feature file; (4) `parity` executes every adapter against the
identical fixture set, digests/canonicalizes each adapter's `Outcome.Projection`, and diffs — an
oracle-bearing case additionally requires every subject to match the oracle language named in the
owner's `🔣️oracle.json` (`oracleHostPackages[].implementation`).

## 6. Recommendation per Go test group — language-agnostic migration effort

| Group | Recommended fixture format | Effort |
|---|---|---|
| `g1_contract_test.go` | already fixture-driven — just needs adapters under `🧪️tests/<slug>/` per §5 shape, reusing `1️⃣g1-contract.json` as the shared fixture | **S** |
| `🔌️mcp` `g2` tests | already fixture-driven (`2️⃣g2-contract.json`, `🚪️entrypoint-contract.json`) — lift as-is into `🥒️.feature` + per-language adapters; saturation/cancellation/reconnect scenarios need explicit `MutationOutcomeClass` tagging | **M** |
| `🎛️coordinator` `g3` tests | already fixture-driven (schema + JSONL golden log) — filesystem-fault injection needs a portable fault-injection contract per adapter, not just Go's fake FS | **M** |
| Exhaustive registry tests (Ticket/Goal/Contributor/Policy/Statute/Bundle/Folder) | convert "query the live repo" into a frozen JSON snapshot fixture of the taxonomy tree (`repo-snapshot.json`) + expected non-empty/shape assertions; decouples from `findTestRepoRoot` walking the real tree | **M** |
| Fix/Scan/Comment/Formatter pipeline | revive the `file-fixable` / `file-fixable-expected` / `file-fixed` / `file-invalid` convention already sitting unused in `🖼️assets/🧫️fixtures`; wire per-language scanners to read those dirs as golden fixtures instead of Go string literals | **S** (fixtures already exist) |
| EventStore | fixture = ordered JSON list of `eventstore.Input` + expected `sequences`/interrupt outcomes (same shape as g1's `eventStore` section) — largely reuse `g1-contract.json`'s existing schema | **S** |
| Command / MCP-result formatting (JSON/markdown) | golden-file pairs: `input-query.json` → `expected.md`/`expected.json`, one per case dir | **M** |
| Git version-control provider | needs a `🔣️oracle.json` third-party oracle (e.g. shell out to `git` is already the oracle in effect) — formalize as `third-party-cli` oracle entry, keep behavior, wrap in feature file | **S** (behavior stays, just needs oracle registration) |
| Hook plumbing / artifact-ID / monorepo-tree rendering / mermaid / plan-extraction ("Other" bucket) | largest, least uniform group; needs per-sub-domain feature files (hooks, artifact-id, tree-render, mermaid, plan-extraction each as separate owners); mostly pure input→output transforms, straightforward to fixture but high volume | **L** |
| Devcontainer/GitKraken bootstrap | asserts generated file content — turn into golden output files (`expected-devcontainer.json`, `expected-compose.yaml`) | **S** |
| `internal/command`, `internal/eventstore` package-local `_test.go` | small, self-contained — merge into the CLI-level fixtures above rather than keeping separate | **S** |
| `🖼️assets/🧫️fixtures/📁️some/📁️folder/*` | currently orphaned — either delete or resurrect for the Fix pipeline group above | **S** (decision only) |

## Key findings for the Rust-parity ticket

- **`🔬️component_test.go`'s 568 tests are almost entirely Go-only, in-code, no-fixture tests** — only
  `t.TempDir()` (193×) and real external processes (`git`, the built `./cli`, `bun`) are used for
  isolation; zero references to `🧫️fixtures/` from that file.
- The **`gN_contract_test.go` family (g1 CLI, g2 MCP, g3 coordinator) is the existing, working
  language-agnostic pattern** — single JSON/JSONL fixture, typed unmarshal, exercised in-process —
  and is the natural seed for adapters under the `🔨️modules/🧪️test/` harness described in §5.
- The full cross-language harness (schema v2, `noOracleDecisions`, oracle/probe registries,
  `bun ./📜️script.ts oracle|subject|parity`) is **not yet used by any of `⌨️cli`/`🔌️mcp`/`🎛️coordinator`**
  — `host-protocol-parity` is the only worked example in the whole repo.
- The `🖼️assets/🧫️fixtures/📁️some/📁️folder/{file-fixable,file-fixable-expected,file-fixed,file-invalid}`
  golden-file tree exists but is **unreferenced by any test in any language** (repo-wide grep across
  `.go/.ts/.py/.cs` found nothing) while the Fix-pipeline test that should consume it
  (`TestFixCommand`, line 2135) now only asserts the feature was removed — a dead/orphaned
  fixture set worth a decision before building on it.
