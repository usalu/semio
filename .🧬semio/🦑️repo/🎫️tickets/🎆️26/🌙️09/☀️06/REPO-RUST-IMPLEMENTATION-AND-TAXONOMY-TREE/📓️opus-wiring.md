# 📓️ Wiring — Rust implementation selection and the last taxonomy moves

Agent: Opus `wiring` (wave 4). Ticket `$TICKET` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE`.

## 1. Phase A — taxonomy moves and legacy deletion

| From | To |
| --- | --- |
| `🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript` | `🔨️modules/⌨️cli/📦️packages/🟦️typescript` |
| `🔨️modules/💻️client/🧩️vscode` (package + `🖼️assets` + `AGENTS.md`/`README.md`) | `🔨️modules/🧩️vscode` |
| `🔨️modules/💻️client/🪶️sqlite/{🗄️.sql,📐️schema.sql}` | `🔨️modules/🪶️sqlite/🧬️schema/` |
| `🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go` | `🔨️modules/🪪️identity/📦️packages/🐹️go/🤖️generated.go` (`package identity`) |

Deleted: the whole `🔨️modules/💻️client/` subtree (including its bundle policy router
`💻️client/📜️script.ts`, whose only lint was the generic "bundle root has package.json" check for a
bundle that no longer exists) and the untracked legacy root tree `./repo/` (`client/cli/client.exe`,
`client/cli/client-win-test.exe`, `client/client.exe`, `client/vscode/{out,node_modules,.vscode-test,repo.vsix}`,
`lib/js`, `server/coordinator/{.next,server.exe,node_modules}`, `assets/fixtures/reports`). `./repo/` was
**entirely untracked** — `git ls-files repo` returned 0 rows before the delete.

`🪶️sqlite` is now a schema-only module: its empty `📦️packages/🟦️typescript` (a `package.json` with no
targets, nx project `@semio-tech/repo-sqlite`) was removed together with its `package.json` workspaces entry,
matching plan §2 (`🪶️sqlite | — | — | client-local SQLite schema`).

The `⌨️cli` TypeScript package's stale `policyFile = "🐹️.go"` line-budget lint (it pointed at a sibling
godfile that has not existed since the go-split) moved to the Go package's own
`⌨️cli/📦️packages/🐹️go/📜️script.ts` as `repo-cli-go-godfile`, where `🐹️.go` really is a sibling.

### References repointed (Phase A)

| File | Change |
| --- | --- |
| `package.json` | two `💻️client/...` workspaces repointed, the sqlite one dropped |
| `.gitignore` | four dead `💻️client/{client,client.exe,client_bin,mcp}` entries dropped |
| `.vscode/settings.json` | vscode extension `directory` |
| `🧅️layering.json` | two entries for deleted `💻️client/⌨️cli/*.go` files dropped, vscode test path repointed |
| `🔒️dependencies.json` | 7 vscode `package.json` manifest paths |
| `📚️library/🔣️taxonomy.json` | `repo-cli-go-module` `pathPattern`/`path` → `⌨️cli/📦️packages/🐹️go/go.mod`; entity-catalog generated Go output root → `🪪️identity/…/🤖️generated.go` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` | Go emit target path and `package client` → `package identity` |
| root `📜️script.ts` | `DEPENDENCY_REPO_POLICY_ROUTERS` lost the deleted `💻️client` router (and the self-test filter that produced the "missing" case now filters `/📚️library/`); build alias `repo-cli` → `@semio-tech/repo-cli:build`; test scope `repo-client` → `repo-cli` |
| `⌨️cli`/`🧩️vscode` `📋️project.json` + `📜️script.ts` | `cwd`, `$schema` and relative-import depth reduced by one level; nx project `@semio-tech/repo-client` renamed `@semio-tech/repo-cli` |

`grep -rn 💻️client` over `*.ts *.json *.jsonc *.mjs *.sh *.ps1 *.toml *.go *.rs` (excluding
`node_modules`, `.nx/`, `.🧬semio/…/⚡️cache`, ticket folders and `.cursor/plans/`) now returns only:
two prose comments (`🌳️tree/🧪️tests/*/🐹️.go`, `🏃️test-runner/📦️packages/🦀️rust/🦀️.rs` — other
agents' docstrings describing the frozen Go snapshot), the deliberate `forbiddenFragments` entries I added
to `⌨️cli/📦️packages/🐹️go/🔬️_test.go`, two frozen library authority fixtures (see §5), and one entry in the
monorepo-wide semantic-directory vocabulary list of `🔣️taxonomy.json` (line 8861) that I left in place —
removing vocabulary is the statute-hygiene agent's territory.

## 2. Phase B — one implementation switch

New in `🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`:

```ts
export type RepoImplementation = "rust" | "go";
export const REPO_IMPLEMENTATION_ENV = "SEMIO_REPO_IMPLEMENTATION";
export const REPO_RUST_CLI_CRATE = "semio-framework-repo-cli";
export const REPO_GO_CLI_DIR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go";
export const REPO_GO_MCP_DIR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go";
export function resolveRepoImplementation(env = process.env): RepoImplementation;
export function repoMcpBinArgs(): string[];      // ["mcp"] for rust, [] for go
export function buildRepoCliBin(root?): string;  // cargo build --release -p … | go build …/🚀️bin
export function buildRepoMcpBin(root?): string;  // rust ⇒ buildRepoCliBin
```

Resolution table (`REPO_CLI_BIN` / `REPO_MCP_BIN` still win when set):

| | `rust` (default) | `go` |
| --- | --- | --- |
| `resolveCliBin` | `target/release/semio[.exe]` | `⌨️cli/📦️packages/🐹️go/semio-repo[.exe]` |
| `resolveMcpBin` | `target/release/semio[.exe]` (+ argv `mcp`) | `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp[.exe]` |

Consumers repointed at that one function:

- root `📜️script.ts` — `dev mcp stdio <profile>` now `buildRepoMcpBin()` + `runCmd(bin, repoMcpBinArgs())`;
  `setup git` runs `buildRepoCliBin()` then `<cli> configure`; `setup` full builds the Rust binary **and**
  both Go alternatives. The old `buildRepoMcpClient()` (hard-wired `go build`) is gone; `buildRepoMcpBinPath()`
  now names the Go cache path explicitly so the Go `test repo-mcp` scope can never overwrite `target/release/semio`.
- `⌨️cli/📦️packages/🟦️typescript/📜️script.ts` — `dev`/`build`/`test` all branch on
  `resolveRepoImplementation()` (cargo vs go).
- Git hooks — `compose_resolve_repo_cli()` in `renderMicroCommitGitHook` is now the shell twin of
  `resolveCliBin`: `REPO_CLI_BIN`, then `$SEMIO_REPO_IMPLEMENTATION`-selected candidates. Rendered and
  installed into `.git/hooks/*` and `🪝️hooks/*`.
- VS Code extension `getRepoBinaryPath()` — was the outlier (`<root>/repo/cli/cli[.exe]`, a path that no
  longer exists). It now mirrors `resolveCliBin` inline (`REPO_CLI_BIN` → implementation-selected path);
  the extension is a vite CJS bundle and cannot import the repo library at runtime, so the mirror carries a
  docstring naming the library function it tracks.
- `🔩️native/🥾️bootstrap/🐚️.sh`, `🔵️.ps1`, `.devcontainer/post-create.sh` — build the Rust binary first
  (`cargo build --release -p semio-framework-repo-cli`), then the Go CLI and Go MCP alternatives.
  `.devcontainer/post-attach.sh` selects the CLI it calls `configure` on the same way.
- `.claude/settings.json`, `.windsurf/hooks.json`, `.factory/hooks.json` — the commented agent-hook
  commands moved from `repo/client/client hook …` to `target/release/semio hook …`.
- `.vscode/🧩️launch.seed.jsonc` (regenerated with the registry `generate` target):
  `🛠️dev🧰️repo🤖️mcp`'s inspector `serverArgs` now `cargo run --release -p semio-framework-repo-cli -- mcp`;
  `🛠️dev🧰️repo⌨️client` runs the CLI package router (Rust default); new `🛠️dev🧰️repo⌨️client🐹️go`
  (order 280.25) is the same command with `SEMIO_REPO_IMPLEMENTATION=go`.
- `⌨️cli/📦️packages/🐹️go/🔬️_test.go` — the three bootstrap-script contract cases now require the cargo
  line, the Go `semio-repo` output path and the Go MCP package path, and forbid `💻️client` outright.

`.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`, `.windsurf/mcp.json`, `.codex/config.toml` and
`.kiro/settings/mcp.json` needed **no edit**: they already spawn `bun ./📜️script.ts dev mcp stdio <profile>`
with the profile argument, and that route is what changed underneath them.

## 3. Verification (real output)

`semio mcp` landed from the `cli` agent while Phase B was in flight, so **both** paths are verified.

Rust default — `bun ./📜️script.ts dev mcp stdio client` (the exact `.mcp.json` command), fed one
`initialize` carrying an unknown member:

```
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{},"tools":{}},"serverInfo":{"name":"repo","version":"1.0.0"},"instructions":"Use repository tools and resources through their owned schemas."}}
```

Go alternative — same command with `SEMIO_REPO_IMPLEMENTATION=go`:

```
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{},"tools":{}},"serverInfo":{"name":"repo","version":"1.0.0"},"instructions":"Use repository tools and resources through their owned schemas."}}
```

Hook resolver, sourced out of the installed `🪝️hooks/post-commit`:

```
rust default -> [/c/git/semio/target/release/semio]
go           -> [/c/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/semio-repo]
```

`bun x nx show projects | grep semio-tech/repo` — `@semio-tech/repo-cli`, `repo-cli-go`, `repo-cli-rs`,
`repo-vscode`, `repo-lib`, `repo-test`, the 22 domain `-go`/`-rs` pairs and the three coordinator projects.
`@semio-tech/repo-client` and `@semio-tech/repo-sqlite` are gone.

`cargo build --release -p semio-framework-repo-cli` → `Finished \`release\` profile [optimized] target(s)`.

`go test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/ -short` →
`ok github.com/usalu/semio/repo/cli 2.438s` (includes the three rewritten bootstrap-contract cases).

Entity catalog regenerated: `entity catalog refreshed (58 entity kinds) -> 🤖️generated/🟦️entity-kinds.ts, 🐹️entity_kinds.g.go, 🤖️generated.rs`;
`go build ./…/🪪️identity/📦️packages/🐹️go/` is clean with the new `🤖️generated.go`.

`.vscode/launch.json` regenerated:
`plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages)` +
`.vscode/launch.json regenerated`.

`git status --short | grep -iE '\.exe|\.vsix|\.pdb'` → empty, and `git ls-files | grep -iE '\.exe$|\.vsix$|\.pdb$'` → empty.

## 4. Deviations

- `.🧬semio/🦑️repo/compose-micro-commit-bun` is a tracked file that `installMicroCommitGitHooks` rewrites
  with the host's bun path. It held a macOS path from another dev; I restored the committed content after
  installing the hooks so the move does not churn it.
- `📚️library/🔣️taxonomy.json` line 8861 keeps `💻️client` in the monorepo-wide semantic-directory
  vocabulary even though no directory carries that slug any more. Vocabulary belongs to the statute-hygiene
  agent; flagged for the audit.

## 5. Left for the audit

- Two **frozen** authority fixtures still carry `💻️client` paths:
  `📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json` and
  `…/🧼️remaining-package-purity-authority/🔣️.json` (consumed by
  `📚️library/🧪️tests/🏺️historical-package-owner-identity`). They are recorded snapshots of an older tree,
  not live assertions; re-recording them is a sweep the audit should do together with the test-case
  directory renames named in plan §5.
- Pre-existing, unrelated to this wiring, confirmed while verifying:
  - `bun nx run @semio-tech/repo-vscode:build` fails with rollup `INVALID_TLA_FORMAT` on
    `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`, reached through the
    extension's `@semio-tech/framework` import (top-level await in a CJS lib bundle).
  - the registry `check` target fails on a missing `wgpu-frame-worker` generated output.
  - `bun ./📜️script.ts verify dependencies` fails on 88 new third-party deps introduced under `temp/brepkit`
    by another fleet; the repo-policy-router self-tests that this ticket touched run before that point and pass.
  - Go packages `repo/tree` and `repo/todos` were transiently uncompilable mid-session while other agents
    edited them; both were green again by the end.
- `🎮️commands/` removal (plan §2 "Removed at the end") belongs to the `dashboard` agent.
