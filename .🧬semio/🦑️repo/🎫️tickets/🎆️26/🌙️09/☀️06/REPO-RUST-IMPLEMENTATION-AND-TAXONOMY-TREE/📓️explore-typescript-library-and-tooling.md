# TypeScript repo library and tooling — exploration

Scope: `🧰️framework/🛍️products/🦑️repo/` — the TS library used by every `📜️script.ts`, the CLI/VSCode/sqlite/coordinator clients, launch.json generation, nx wiring, hooks. Read-only pass to inform a Rust port of the Go client.

## 1. Go behaviour already reimplemented in TypeScript

The TS side is **not** a thin wrapper — it has its own full-blown taxonomy/policy/commit engine, parallel to (and in some respects ahead of) the Go client. A Rust port should treat these TS modules as the schema source of truth, not the Go code, to avoid a three-way divergence.

| Concern | File | Lines / symbols |
|---|---|---|
| Taxonomy schema types (all `Semantic*`, `Fixed*`, `Package*` contracts) | `🔨️modules/📚️library/🔍️discovery/🟦️.ts` (10,566 lines total) | types start line 19; `loadTaxonomy()` 1283, `loadCatalogTaxonomy()` 1273, `resolveWorkspaceTaxonomyAuthority` 1207/1263, `canonicalFilenameForKind` 1327, `fileKindIdForSourcePath` 1456, `scopedFileKindIdForSourcePath` 1466, `semanticDirectoryKindId` 1684, dozens more `taxonomyCli*PreparationProblems` 1493-1625 |
| Taxonomy data (compiled JSON, not code) | `🔨️modules/📚️library/🔣️taxonomy.json` | 26,682 lines / ~1MB — the actual grammar data both TS and (presumably) Go/Rust must agree on |
| Taxonomy inventory/plan/verify/apply pipeline (full tree-walk + transactional move/edit engine) | `🔨️modules/📚️library/🧹️normalization/🟦️.ts` (11,848 lines total) | types from line 44 (`TaxonomySeverity`…`TaxonomyApplyResult` through 427); `inventoryTaxonomySources` 2944, `inventoryTaxonomy` 6702, `planTaxonomy` 8131, `verifyTaxonomy` 8283, `applyTaxonomyPlan` 10974; embedded-ticket-root parsing (ticket-id scheme) 2016-2034 and 7641-7670 |
| Ticket id scheme (`YY/MM/DD/SLUG`, regex `^[0-9]{2}/[0-9]{2}/[0-9]{2}/.+`) | `🧹️normalization/🟦️.ts` | `TaxonomyEmbeddedTicketRootDisposition.ticketId` type line 215; validated at 2033-2034; reconstructed via `splitLeadingEmoji` at 7655 |
| Policy/lint engine (parallels Go GraphQL `analyze`/`statutes`/`fix`) | `🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` (6,436 lines, 324 exports) | `BreachRecord` type 59; `BaseLinter`/`TechnologyLinter`/`BundleLinter`/`FolderLinter`/`FileLinter`/`SectionLinter`/`DefinitionLinter` 191-445; `resolveFolderByPath`/`resolveBundleByName`/`resolveTechnologyByName` 445-469; `defineLint` 472; dependency-boundary breach scanning 495-628; layering breaches 628-758; policy-script discovery/run/exit 758-971 |
| Commit/micro-commit pipeline (bundle tags, WIP subjects, bullet-coverage, git hooks) | same file, 4362-6335 | `runMicroCommit` 5129, `runCommit` 6152, git-hook install/render 4940-5121, bundle-subject regex `BUNDLE_WIP_SUBJECT_RE`/`BUNDLE_DATE_SECTION_RE` 5251-5252 |
| Cargo/Go/Rust test & build orchestration (test-level budgets, nextest partitioning, cargo lease/law harness) | same file | `TEST_LEVELS`/`resolveTestLevel` 1108-1145, `runCargoTestBudgeted` 1596, `partitionNextestExecutionFilters` 1536, `runExactCargoLaws` 1912, `runCargoLint` 2268 |
| Coverage merge (lcov, go-profile→lcov) | same file | `parseLcov` 2158, `goProfileToLcov` 2203, `mergeLcov`/`summarizeCoverage`/`enforceCoverageThreshold` 2179-2268 |
| ULOC / size metrics (used for commit-message metric suffixes) | same file | `scanRepoUnifiedLoc*` 3453-3525, `countUnifiedLocForFile` 3336, `formatBundleMetricSuffixes` 3904 |
| Workspace discovery (bun `package.json` `workspaces[]` vs. actual folders) | `🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (186 lines) | `getWorkspaceRoot` 19, `computeWorkspaces` 150, `diffWorkspaces` 176 |
| Artifact/ticket scaffolding | `🔨️modules/📚️library/🏗️builder/🟦️.ts` (210 lines) | `authorArtifactScaffold` 124 |
| GraphQL client used by TS code to call the Go server | `📦️packages/🟦️typescript/🟦️.ts` | `runCliGraphql` 109-151 (spawns CLI binary, `--json graphql --query … -v <json>`) |

Net: taxonomy parsing/plan/apply, ticket-id validation, the breach/policy model, and commit/test/coverage orchestration are **fully duplicated** in TS already (not just consumed from Go). Any Rust port should either (a) become the single implementation both TS and Go delegate to via the compiled `🔣️taxonomy.json` + a shared schema, or (b) at minimum copy the TS regexes/types verbatim rather than re-deriving them from Go source, since the TS copy is the actively-maintained one judging by line count and depth.

## 2. How TS/VSCode invoke the Go client/MCP today

| Caller | Mechanism | Binary resolution |
|---|---|---|
| `runCliGraphql()` (`📦️packages/🟦️typescript/🟦️.ts:109`) | `runCmd(bin, ["--repo", root, "--json", "graphql", "--query", query, "-v", vars])` | `resolveCliBin()` (line 90-93): env `REPO_CLI_BIN` else `defaultCliBin()` line 85 → `<root>/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/{client.exe|client}` (win32 vs other) |
| MCP binary resolver (same file) | n/a directly invoked here, just resolves | `resolveMcpBin()` line 102: env `REPO_MCP_BIN` else `defaultMcpBin()` line 96 → `.../💻️client/{mcp.exe|mcp}` |
| Repo CLI's own `dev`/`build` scripts | `🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` | `dev`: builds via `go build -o <resolveCliBin bin> ./…/⌨️cli/cmd/repo` then runs it with `GOWORK=<root>/go.work`. `build`: `go build -trimpath -ldflags=-s -w -o <root>/🔨️modules/💻️client/{client.exe|client} ./⌨️cli/cmd/repo` |
| MCP go package's own build | `📜️script.ts` under `💻️client/🔌️mcp/📦️packages/🐹️go/` (exists but empty content dumped here — has only a `📋️project.json`) — see file directly if package.json build steps needed |
| Coordinator server build | `🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts` | `go build -o server<ext> .` from the coordinator Go module root |
| Git hooks (`prepare-commit-msg`, `post-commit`, `post-merge`, `post-checkout`, `post-rewrite`) | `🪝️hooks/*` (POSIX sh, installed via `installMicroCommitGitHooks` in the TS lib, rendered by `renderMicroCommitGitHook` line 5065) | `compose_resolve_repo_cli()` in each hook: env `REPO_CLI_BIN` → `<root>/🔨️modules/💻️client/client` → `.../client.exe`; invokes `"$CLI" micro-commit reset` |
| VSCode extension (`💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`) | `execFileAsync(command, ["--json","search",...])` / `execAsync('"cmd" --json search')`, GraphQL doc strings for `Repo`, `Tickets`, `Policies`, `Analyze`, `Fix`, `Codebase`, `Goals` queries (lines ~865-1204) | **Different path convention than the lib**: `getRepoBinaryPath()` (line 1730) looks for `<workspaceRoot>/repo/cli/cli{.exe}` — NOT `resolveCliBin()`'s `<root>/🧰️framework/.../💻️client/client{.exe}`. This is a live inconsistency to flag/align when substituting a Rust binary — the VSCode extension will not find a binary built via the standard `resolveCliBin` convention unless `repo/cli/cli.exe` is also produced (there is a `repo/client/...` checked-in tree at repo root from a previous build, per the earlier `find` listing) |
| Launch entry `🛠️dev🧰️repo⌨️client` (`.vscode/🧩️launch.seed.jsonc:798-801`) | `go build -o 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client ./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go && ./…/client --help` | Note: this seed entry builds the **client** binary from the **mcp** go package's `main`, not from `⌨️cli/cmd/repo` — a second, divergent build path from the one in `⌨️cli`'s own `📜️script.ts build` (`go build … ./⌨️cli/cmd/repo`). Two different Go entrypoints currently produce a binary at the same output path `💻️client/client(.exe)`. |
| Launch entry `🛠️dev🧰️repo🤖️mcp` (seed 767-773) | `bun run dev -- mcp repo`, `MCP_PROXY_AUTH_TOKEN=repo-mcp-token`, proxied via `go run ./…/🔌️mcp/📦️packages/🐹️go` (seed line 783 `uriFormat`) | stdio transport through mcp-inspector proxy on port 6277/6274 |
| Launch entry `🛠️dev🧰️repo🤖️mcp⌨️cursor` (seed 787-790) | `bun ./📜️script.ts dev mcp stdio cursor` | root script.ts `dev mcp stdio <client>` subcommand (see root `📜️script.ts` around line 669 comment on stdio vs http transport, port 6300, range 6012-6205) |

**Where a Rust binary must be substituted:**
1. `defaultCliBin()` / `defaultMcpBin()` in `📦️packages/🟦️typescript/🟦️.ts:85-107` — the single canonical resolution point used by `runCliGraphql`.
2. `getRepoBinaryPath()` in the VSCode extension (`💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts:1730`) — currently pointing at a *different* path (`repo/cli/cli.exe`); must be reconciled or the extension updated to call `resolveCliBin`-equivalent logic.
3. All 5 git hooks under `🪝️hooks/` (`compose_resolve_repo_cli`) — duplicated shell logic, same `client`/`client.exe` path as #1.
4. The `⌨️cli/📦️packages/🟦️typescript/📜️script.ts` `DevScript`/`BuildScript` (`go build … -o <bin>`) — swap for `cargo build` and update `resolveCliBin` output extension logic if the Rust binary name/extension differs.
5. `.vscode/🧩️launch.seed.jsonc` line 798-801 (`🛠️dev🧰️repo⌨️client`) and line 767-790 (mcp launch entries) — hand-edit the seed, then regenerate (see §3).
6. Coordinator's `go build -o server<ext> .` in its own `📜️script.ts` if the coordinator itself is ported (separate Go module, separate binary, not `client`/`mcp`).

## 3. `.vscode/launch.json` generation

- **Source of truth**: `.vscode/🧩️launch.seed.jsonc` — hand-edit ONLY this file (keyboard/mouse shortcuts, bespoke launchers, build/publish groups, and the `devLaunchers` table for per-playground-variant templates appended after the `DEV_LAUNCHERS_MARKER` string literal — see `🖥️launch.ts:24-25`).
- **Generator module**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts` (243 lines). `readSeed()` splits the JSONC at the marker into (a) the verbatim `configurations` skeleton text containing `"@generated:<variant>:<renderer>"` placeholders, and (b) a parsed `devLaunchers` JSON table. `generateLaunchJson()` (not fully read, but referenced) substitutes registry-derived dev-server ports into the placeholders.
- **CLI entrypoint**: sibling `📜️script.ts` in the same `📇️registry` package (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`):
  - `bun ./📜️script.ts generate` (default command) — writes `.vscode/launch.json` (`GenerateScript`, line 1908-1923) alongside the plugin/playground registry catalog.
  - `bun ./📜️script.ts check` — verifies freshness of both the registry catalog and `.vscode/launch.json` (`CheckScript`/`CheckGeneratedScript`, 3073-3156); fails if stale or if `generateLaunchJson()` throws (seed/devLaunchers mismatch).
  - `preview-generated` — dry-run preview without writing.
- **Procedure to add a new launch entry correctly**:
  1. Edit `.vscode/🧩️launch.seed.jsonc` directly — either a static entry in the `configurations` array (grouped/ordered like existing entries: `presentation.group`/`order`, emoji-prefixed `name` following existing conventions e.g. `🛠️dev🧰️repo…`, `⚖️gate…`, `🧹clean…`), or, for a new playground variant, a `devLaunchers` entry (namePrefix/order/command/env templates).
  2. Run `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` (or via nx target on that project) to regenerate `.vscode/launch.json`.
  3. `bun ./📜️script.ts check` (registry package) or the corresponding nx `check` target must pass — this is presumably gated in CI/policy.
- Naming/grouping convention observed in the seed: numeric `presentation.group` prefixes (`1_keyboard`, `2_mouse`, `3_dev`, …) with `order` integers for stable sort; leading emoji hierarchy encodes area (`⌨️`=keyboard tool, `🖱️`=mouse tool, `🛠️dev`=dev launcher, `⚖️gate`=CI gate, `🧹clean`=taxonomy/cleanup check), followed by nested emoji-tagged path segments matching the taxonomy tree (e.g. `🛠️dev🧰️repo⌨️client`, `🛠️dev🎛️dashboard🌀daemon▶️start`).

## 4. nx project/target discovery for Go and Rust

- `nx.json` `plugins` (line ~109): `@nx/js` (infers TS/JS packages from `package.json`), `@nxlv/python`, and two custom local plugins:
  - `./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — `createNodesV2: ["**/📜️script.ts", policyScriptProjects]`. For **every** `📜️script.ts` that `export const policy` (checked via regex `scriptExportsPolicy`), it synthesizes a project named `breach-<slugified-relpath>` with one cacheable `lint` target running `bun "<script>" policy`. This is orthogonal to package/build/test targets and applies uniformly regardless of language (Go, Rust, TS).
  - `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` — `createNodesV2: ["**/*.feature", testCaseProjects]`, generates gherkin test-case projects (not language-specific either).
- **There is no dedicated nx inference plugin for Go modules or Cargo crates.** Go packages (e.g. `💻️client/🔌️mcp/📦️packages/🐹️go`) have **no `📋️project.json` at all** in the observed example — they're only reachable via their `📜️script.ts` (which nx picks up for the `lint`/policy target only) or invoked directly by `go build`/`go test` from other scripts. Membership for Go compilation units is purely `go.work`'s `use (...)` list (`go.work` at repo root lists `⌨️cli`, `🔌️mcp`, `📚️library`, `🎛️coordinator`).
- **Rust crates get an explicit, hand-authored `📋️project.json`** (example: `⌨️cli/📦️packages/🦀️rust/📋️project.json`) with `nx:run-commands` executors, each `cwd` set to the crate dir and `command: "bun ./📜️script.ts <target>"` (`build`, `test`, `test-quick`, `test-long`, `test-exhaustive`, `run`, `daemon`, `workflow` — all delegate into the same `📜️script.ts`). Crate membership in the Cargo workspace is via the root `Cargo.toml`'s `members = [...]` array (line 4).
- **What a new package needs**:
  - TS package: a `package.json` (auto-discovered by `@nx/js`) + `📜️script.ts` (auto-picked-up by the policy plugin if it exports `policy`) + entry in root `package.json` `workspaces[]` array.
  - Go package: add its directory to `go.work`'s `use (...)` list; no `project.json` required unless you want nx-visible targets beyond the auto-generated `breach-*` lint project — in that case hand-author one following the Rust pattern.
  - Rust crate: add to root `Cargo.toml` `members`; hand-author `📋️project.json` (copy the `⌨️cli/📦️packages/🦀️rust/📋️project.json` shape, adjusting `name`/`cwd`/targets) alongside a `📜️script.ts` implementing those targets via `BundleScript`/`ScriptRouter` from the shared TS library.

## 5. Product-root `🧪️tests` and `🪝️hooks`

- `🧪️tests/🧪️transaction-process-ownership/` — one gherkin-style test case directory (`🔣️.json` fixture, `🟦️.ts` support, `🧪️test/🟦️.ts` the actual case, `🧬️schema/🔣️.json`) verifying process/transaction ownership semantics for the repo product; picked up by the `🧪️test/🟨️.mjs` nx plugin (`**/*.feature`-shaped discovery, though here it's JSON/TS not `.feature` — same package family as `🔨️modules/📚️library/🧪️tests/*` which has ~80 similarly-structured case directories).
- `🪝️hooks/` contains 5 POSIX-sh git hooks: `post-checkout`, `post-commit`, `post-merge`, `post-rewrite`, `prepare-commit-msg`. All but `prepare-commit-msg` are near-identical: they wipe micro-commit draft state (`compose_micro_commit_wipe`) and, if a repo CLI binary can be resolved (`compose_resolve_repo_cli`: env `REPO_CLI_BIN` → `<root>/🔨️modules/💻️client/client` → `.../client.exe`), invoke `"$CLI" micro-commit reset`. `prepare-commit-msg` instead calls back into TS: if `$GIT_DIR/compose-micro-commit-active` exists, it resolves a pinned/PATH `bun` and runs `"$BUN" "$ROOT/📜️script.ts" micro-commit prepare-commit-msg "$1" "$2"`, i.e. the **actual message-building logic lives in the TS library** (`buildMicroCommitMessage`/`handlePrepareCommitMsg`/`renderMicroCommitGitHook` in `📦️packages/🟦️typescript/🟦️.ts:4831-5121`), and the Go client is only invoked for the simpler `micro-commit reset` call in the other 4 hooks. These hooks are installed by `installMicroCommitGitHooks()` (same TS file, line 5102), rendered from `renderMicroCommitGitHook()` (line 5065) — so hook content itself is generated from TS, not hand-maintained.

## Key files referenced (absolute paths)

- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\📦️packages\🟦️typescript\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🔍️discovery\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🧹️normalization\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🔣️taxonomy.json`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🗂️workspaces\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️builder\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products🦑️repo\🔨️modules\📚️library\🔌️nx-plugin\🟨️.mjs` (see note below on emoji path — actual: `...\📚️library\🔌️nx-plugin\🟨️.mjs`)
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🧪️test\🟨️.mjs`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\⌨️cli\📦️packages\🟦️typescript\📜️script.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\🧩️vscode\📦️packages\🟦️typescript\🟦️.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\🧩️vscode\📦️packages\🟦️typescript\📜️script.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\🪶️sqlite\` (schema-only package: `📐️schema.sql`, `🗄️.sql`, no script.ts found)
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🖥️server\🎛️coordinator\📦️packages\🟦️typescript\📜️script.ts`
- `C:\git\semio\🧰️framework\🛍️products🦑️repo\📜️script.ts` (root technology policy) — actual: `...\🦑️repo\📜️script.ts`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🪝️hooks\{post-checkout,post-commit,post-merge,post-rewrite,prepare-commit-msg}`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🧪️tests\🧪️transaction-process-ownership\`
- `C:\git\semio\.vscode\🧩️launch.seed.jsonc`
- `C:\git\semio\.vscode\launch.json`
- `C:\git\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\🖥️launch.ts`
- `C:\git\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\📇️registry\📜️script.ts`
- `C:\git\semio\nx.json`
- `C:\git\semio\go.work`
- `C:\git\semio\Cargo.toml`
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\⌨️cli\📦️packages\🦀️rust\📋️project.json`
- `C:\git\semio\.devcontainer\devcontainer.json`, `post-create.sh`
