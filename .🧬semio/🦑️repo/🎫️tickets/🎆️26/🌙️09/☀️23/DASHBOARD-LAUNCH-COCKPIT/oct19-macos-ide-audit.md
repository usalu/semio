# Oct19 macOS IDE Authority Audit

Read-only source audit, 2026-10-19. No runtime verification or tests were run by this auditor. No source edits, git mutations or worktrees were used.

## Current Files

`git ls-files` and a hidden-file inventory excluding dependencies, Git and ticket history found no authored `.vscode`, `.idea`, `.vs`, `.fleet`, `.zed`, `.run`, `.nova`, `.settings`, `.metadata` directories or `.sln`, `.slnx`, `.code-workspace`, `.suo`, `.user`, `.iml`, `.ipr`, `.iws` metadata files. No tracked removal for the principal root paths is currently pending. Existing dashboard authority schemas and fixture suites already forbid these families.

Keep the two `.csproj` files: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🔷️dotnet/🔷️.csproj` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj`. Both are Microsoft.NET.Sdk manifests with explicit compiler inputs, target frameworks and package/build properties. They are executable build authority, not editor preferences.

## Actionable Remaining References

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json:83`: dotnet named input still contains `{workspaceRoot}/Monorepo.sln`. Remove this nonexistent solution dependency; retain project manifests.
2. `🧰️framework/🔨️modules/🏃️process/🏗️build/🎛️environment/🧬️schema/🔣️.json:25`: Protocol.const still declares launch and seed catalogs, `node-terminal` and `launch`. Its `🧪️tests/🟦️.ts:15` test reads those nonexistent catalogs and expects executable configurations. Replace this editor-specific contract and test with dashboard execution environment authority; keep the implicit-cache environment safeguards.
3. `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json:88`: input list still references root `.vscode/launch.json` and `.vscode/tasks.json`. Replace those with the actual neutral command owner metadata needed by the course dashboard checks.

## References That Serve Other Purposes

The dashboard authority schema contains the exact root AGENTS.md launch instruction as a recognized obsolete-instruction marker. Do not remove the detector merely because its text mentions launch.json; AGENTS.md is forbidden to edit. The root user instruction supersedes its old editor workflow.

`.cursor/plans` contains many historical plans referring to authoring/generating launch.json. They are old plan evidence, not current executable generators. Classify their retention explicitly if broad instruction scans include historical plans. Existing ticket history also contains old IDE instructions.

The Go workspace formatter intentionally recognizes IDE metadata families for rejection/cleanup. Test fixtures intentionally contain forbidden example paths. The teaching dashboard fixture uses a negative pattern to forbid launch references. `.gitignore` ignores IDE output. These are prevention mechanisms, not active editor authority.

The devcontainer user named `vscode` and its home paths are container identity, ownership and persistence configuration. They do not configure an editor. Repository VS Code client implementation and its `.vscodeignore` package metadata are a shipped client product, not developer launch authority.

Generic test-runner detection recognizes `.sln` in Go and Rust. This does not generate or require a repository solution. It is separate from the stale named input above.

## Repo MCP Access

Root `.mcp.json` launches `bun ./📜️script.ts dev mcp stdio client`. The root command router dispatches to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`, whose runMcpStdioRepo resolves the repo binary and sets `SEMIO_REPO_MCP_CLIENT=client`. A client can use newline JSON-RPC initialize, notifications/initialized, resources/read with `repo://goals`, tools/list and tools/call. Ticket method schemas are in `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗄️repository/🐹️.go`.

The existing open ticket already covers the exact task. Parent connected MCP and read goals; ticket reopen was rejected because it is already open.
