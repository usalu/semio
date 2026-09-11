# Summary

Devcontainer configuration and lifecycle scripts.

# Neo4j Desktop and MCP

The monorepo registers Neo4j MCP servers in `.mcp.json` and per-client copies. They use `uvx mcp-neo4j-cypher` against graph database **`compose`** (Neo4j Community: one user database per DBMS, named `compose` via `initial.dbms.default_database`).

MCP server ids include **`neo4j-compose`**, **`neo4j-elements`**, **`neo4j-coda`**, **`neo4j-reuse`**, **`neo4j-metabolism`** (argv targets Bolt graph **`metabolism`**), and **`neo4j-extra`** (targets **`NEO4J_EXTRA_GRAPH_DATABASE`** when set).
**Native (Windows / macOS / Linux):** Create a Neo4j Desktop **Local Instance** named **`compose`**, set password **`password`**, and start it on Bolt port **`7687`**. On DBMS editions that support multiple user databases, create the product graphs **`elements`**, **`coda`**, **`reuse`**, and any extra Bolt names you list in **`NEO4J_EXTRA_GRAPH_DATABASES`** (comma-separated) so `bun run generate` and MCP can target them. Graph argv for `neo4j-*` / `generate neo4j`: one or more tokens joined with `-` (e.g. `… neo4j my graph` → database `my-graph`). On **Neo4j Community**, only **one** standard user graph exists per DBMS; use **Enterprise** (or equivalent) for multiple isolated graphs, or point every MCP entry at your single graph name. Native setup enables APOC on the local DBMS when needed, uses the native Neo4j Desktop/DBMS only, does not depend on the devcontainer, and does not edit Desktop internals.

Native Neo4j Desktop connection:

- URL: **`bolt://127.0.0.1:7687`**
- User: **`neo4j`**
- Password: **`password`**
- Browser: **`http://127.0.0.1:7474`**
- Database: **`compose`**

**Devcontainer:** Neo4j 5 Community runs inside the single **`compose`** devcontainer. Inside **`compose`**, `NEO4J_URI` is **`bolt://localhost:7687`**. The **`compose`** container publishes **`127.0.0.1:7687`** (Bolt) and **`127.0.0.1:7474`** (Browser) to the Docker host, and `devcontainer.json` forwards both ports for Codespaces and local devcontainers.

**Neo4j Desktop remote connection for devcontainers:** Docker Desktop must be running for local devcontainers. **Reopen in Container** after the image has been rebuilt once so the Neo4j Debian package is available inside **`compose`**. Then:

1. `Test-NetConnection -ComputerName 127.0.0.1 -Port 7687` on Windows, or `nc -vz 127.0.0.1 7687` on macOS/Linux.
2. Desktop: **`bolt://127.0.0.1:7687`**, user **`neo4j`**, password **`password`**.
3. Browser: **`http://127.0.0.1:7474`** with the same credentials.

**Database:** New devcontainer stores initialize the user graph as **`compose`**. The named **`<workspace>-neo4j-data`** volume retains the live store across container recreation. Startup preserves existing stores and Git stashes.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and the shared `.🧬semio/🦑️repo/⚡️cache` root (cargo, Nx, Playwright, and every other repo-managed build cache).

## docker-compose.yml

Compose stack for the devcontainer: **`compose`** only. Neo4j is installed in the **`compose`** image, started by **`post-start.sh`**, with its live store persisted in the workspace’s **`neo4j-data`** named volume. Repo-owned Cypher files under **`.🧬semio/🦑️repo/🛂️manifest`** remain explicit export/import artifacts. MCP uses **`bolt://localhost:7687`** from inside **`compose`**.

## Neo4j Cypher Persistence

APOC Core and APOC Extended are installed in the **`compose`** image and configured for file import/export. The canonical repo persistence paths are:

- **`.🧬semio/🦑️repo/🛂️manifest/compose.cypher`**
- **`.🧬semio/🦑️repo/🛂️manifest/elements.cypher`**
- **`.🧬semio/🦑️repo/🛂️manifest/coda.cypher`**
- **`.🧬semio/🦑️repo/🛂️manifest/reuse.cypher`**

Container startup preserves the live graph and performs no Cypher replay or pruning. Export technology-scoped graph state with APOC query exports, for example:

```cypher
CALL apoc.export.cypher.query(
  'MATCH (n:Compose) OPTIONAL MATCH (n)-[r]->(m:Compose) RETURN n, r, m',
  '/workspaces/semio/.🧬semio/🦑️repo/🛂️manifest/compose.cypher',
  {format: 'cypher-shell'}
);
```

## compose-entrypoint.sh

Legacy helper kept for existing callers. The current setup does not need entrypoint startup logic because **`post-start.sh`** starts Neo4j inside **`compose`**.

## Dependency Preparation

The image provides Bun 1.3.14 and Node 24.15.0 from pinned, checksum-verified Linux x64/arm64 archives. Nx comes from the repository's locked tooling bootstrap. Container creation invokes `bun nx run workspace:deps-javascript`, which synchronizes the frozen Bun lockfile without building applications. Select additional dependency environments and project builds through their Nx launch configurations. Post-start and post-attach remain separate lifecycle hooks.

## post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, writes Neo4j MCP environment defaults, configures and starts the local Neo4j service, checks **`localhost:7687`**, and activates the Python virtual environment.

## post-attach.sh

The attach hook detects the active editor CLI and invokes `bun nx run @semio-tech/repo-vscode:build-vsix`. Nx owns source invalidation, the build prerequisite and VSIX restoration. A successful package is installed through the selected editor CLI and verified with its extension list; a failed package skips installation. Linux developer-tool and repo configuration steps remain separate parts of this hook.

## Devcontainer Persistence

Devcontainer rebuilds keep AI tooling state by mounting named volumes for CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`), GitKraken Desktop state (`~/.gitkraken`), GitKraken CLI state (`~/.local/share/GitKrakenCLI`, `~/.local/share/gk`), and editor servers (`~/.vscode-server`, `~/.windsurf-server`).
One additional named volume mounts at `${containerWorkspaceFolder}/.🧬semio/🦑️repo/⚡️cache` — the single shared build/tool cache root (cargo, Nx, Vite, Playwright, Go, …) — so every agent and dev process in the container reuses the same cache and rebuilds never start cold.
Claude Code persists its auth files by storing `~/.claude.json` inside the mounted Claude volume and linking it back into `$HOME` on start.
Post-start ownership fixes keep the mounted volumes writable so chat history and tokens survive container replacement.
Post-attach reconciles VS Code workspace chat storage for `GitHub.copilot-chat` and `openai.chatgpt` by merging transcript and chat resource folders from older workspace-storage hashes into the active workspace-storage directories after attach.
Post-attach asks Nx for the current VSIX on every enabled attach, then installs that package with the editor CLI. Source or archive timestamps do not decide whether a build is needed.
Post-attach also materializes Windsurf's MCP config at `~/.codeium/windsurf/mcp_config.json` and merges Codex MCP server entries into `~/.codex/config.toml` from the monorepo `.mcp.json`, so both clients pick up the repo, compose, coda, and Playwright servers after rebuilds without manual setup while preserving existing Codex user settings such as model and personality.
Post-attach installs Linux GitKraken Desktop and its CLI when missing, then creates or updates the local GitKraken workspace from the repo root and submodules.
Engine compatibility for the local extension is aligned to the lowest supported editor build so Cursor and VS Code accept the same VSIX.

## Emoji Font Setup

The devcontainer image installs comprehensive emoji font support including `fonts-noto-color-emoji`, `fonts-noto-cjk`, `fonts-noto-mono`, and additional font packages. Font configuration is automatically applied to ensure emoji rendering works across all applications.

### Font Configuration

- **Automatic fontconfig setup**: Scripts configure `/etc/font/local.conf` with proper emoji font fallbacks
- **Generic font families**: Emoji fonts are added to `sans-serif`, `serif`, and `monospace` font families
- **Locale support**: UTF-8 locale variables are set for proper emoji encoding
- **Comprehensive coverage**: Multiple font packages ensure broad emoji support

### Application Support

- **VS Code**: Emojis display properly in editor, terminal, and UI
- **GitKraken**: Commit messages and interface show emojis correctly
- **Web browsers**: Container browsers render emojis with proper fonts
- **Terminal applications**: Emoji support depends on client capabilities

### Testing

Use the provided test files to verify emoji rendering:

- `test_emoji.py`: Python script to test emoji support
- `emoji_test.html`: HTML page for browser emoji testing

The font configuration refreshes on container start and ensures emoji glyphs are available without manual package installation.

## Devcontainer Extension Install

When an editor CLI is available, post-attach resolves the workspace VSIX through its Nx target and installs it automatically after packaging succeeds.
This keeps the active editor clean of stale versions while aligning installation with a running IDE server, avoiding failures during container creation and preserving automatic delivery.

## GitKraken Zero Touch

GitKraken zero-touch setup persists Linux GitKraken Desktop state, the `gk` runtime, and local workspace metadata across rebuilds and refreshes the Compose workspace automatically on attach.
The bootstrap targets the repo root and declared git submodules, then sets the Compose GitKraken workspace as the default so the same graph opens immediately in Linux GitKraken Desktop.

### WSL Compatibility

The devcontainer automatically detects WSL environments and starts GitKraken with the `--no-sandbox` flag to handle namespace restrictions. This ensures GitKraken works seamlessly in WSL without manual intervention.

### VS Code Integration

A VS Code task is available for launching GitKraken:

- Use `Ctrl+Shift+P` → "Tasks: Run Task" → "🐧️gitkraken"
- Or run from terminal: `bash .devcontainer/gitkraken-launch.sh`

The launcher script automatically:

- Detects if GitKraken is already running
- Applies WSL-compatible flags (`--no-sandbox --no-debug`)
- Prevents debugger hanging issues
- Launches GitKraken in the background

### Environment Variables

Configure GitKraken behavior with these environment variables:

- `SEMIO_GITKRAKEN_WORKSPACE_NAME`: Workspace name (default: "compose")
- `SEMIO_GITKRAKEN_AUTO_START`: Auto-start GitKraken on attach (default: "false", disabled to prevent spurious git stashing in concurrent editing workflows)
- `SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL`: Skip extension installation (default: empty)

### Error Resilience

All setup scripts include comprehensive error handling:

- Failed installations continue with warnings rather than blocking
- Authentication checks prevent unnecessary GitKraken CLI operations
- Extension installation retries across multiple IDE CLIs
- Timeout handling for concurrent operations

## Search Tooling

The devcontainer image installs ripgrep (`rg`) as part of the base apt package set so fast recursive code search is available immediately in all editor terminals and scripts.

## Playwright Browser Cache

Playwright browser downloads live under `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, inside the single named volume mounted at the shared cache root, so the binaries persist across container restarts, editor reloads, and rebuilds.
The devcontainer sets `PLAYWRIGHT_BROWSERS_PATH` to that shared cache location, and the provisioning script installs Chromium into that path so `npx playwright install` is a no-operation once cached.

# 💯️Requirements

## Devcontainer

Devcontainer provisioning MUST install the workspace VS Code extension automatically after editor attach without manual installation steps.

Devcontainer post-attach MUST resolve the workspace extension through the Nx packaging target, install only after that target succeeds, and validate installation through the active editor CLI.

Devcontainer post-attach MUST generate Windsurf MCP config, write `.cursor/mcp.json` with repo-root-absolute MCP commands (so Cursor discovers stdio servers even when the spawn cwd is not the repo root), and merge Codex MCP server entries from the monorepo `.mcp.json` into the clients' home config folders without removing unrelated Codex user settings.

Compose VS Code extension engine compatibility MUST include Cursor's supported VS Code version range.

Playwright browser caches MUST use the shared `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` path so browser install stays cached across reloads and is pruned by the same cache budget as every other build output.

Claude Code and Codex auth plus chat history MUST persist across devcontainer rebuilds via named volumes for CLI config and editor server state.
VS Code chat-provider workspace history MUST persist across devcontainer rebuilds even when the active `workspaceStorage` hash changes for the same repo.

Claude Code auth files MUST live in the persisted Claude volume and be linked into the home directory.

Devcontainer provisioning MUST install Linux GitKraken Desktop and the official GitKraken `gk` CLI when they are missing.

Devcontainer lifecycle scripts MUST persist GitKraken CLI runtime files and local workspace metadata across rebuilds.

Devcontainer post-attach MUST create or update the default Compose GitKraken local workspace from the repo root and submodules without manual GitKraken setup.

Devcontainer provisioning MUST install a color emoji font and refresh fontconfig caches so GUI applications render emoji glyphs without manual setup.

Devcontainer lifecycle scripts MUST enforce fontconfig fallback to `Noto Color Emoji` for the common font families used by Electron and GTK applications.
