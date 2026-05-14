# Summary

Devcontainer configuration and lifecycle scripts.

# Neo4j Desktop and MCP

The monorepo registers one Neo4j MCP server (`neo4j`) in `.mcp.json` and per-client copies. It uses `uvx mcp-neo4j-cypher` against the default `neo4j` database.

**Native (Windows / macOS / Linux):** Run your root platform bootstrap (`setup.windows.script.ps1`, `setup.mac.sh`, or `setup.linux.sh`) so `NEO4J_URI=bolt://localhost:7687` and credentials are set. Native setup uses the native Neo4j Desktop/DBMS only; it does not depend on the devcontainer.

**Devcontainer:** Neo4j 5 Community runs inside the single **`semio`** devcontainer. Inside **`semio`**, `NEO4J_URI` is **`bolt://localhost:7687`**. The **`semio`** container publishes **`127.0.0.1:7687`** (Bolt) and **`127.0.0.1:7474`** (Browser) to the Docker host, and `devcontainer.json` forwards both ports for Codespaces and local devcontainers.

**Neo4j Desktop remote connection:** Docker Desktop must be running for local devcontainers. **Reopen in Container** after the image has been rebuilt once so the Neo4j Debian package is available inside **`semio`**. Then:

1. `Test-NetConnection -ComputerName 127.0.0.1 -Port 7687` on Windows, or `nc -vz 127.0.0.1 7687` on macOS/Linux.
2. Desktop: **`bolt://127.0.0.1:7687`**, user **`neo4j`**, password **`password`**.
3. Browser: **`http://127.0.0.1:7474`** with the same credentials.

**Database:** The devcontainer uses the default **`neo4j`** database. Neo4j Community supports one user database per DBMS, so separate technology databases are intentionally not created.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and Playwright cache under `node_modules`.

## docker-compose.yml

Compose stack for the devcontainer: **`semio`** only. Neo4j is installed in the **`semio`** image, started by **`post-start.sh`**, and persisted in repo-owned Cypher files under **`.repo/🛂`**. The live Neo4j store is container-local and replayed from those Cypher files on an empty DB. MCP uses **`bolt://localhost:7687`** from inside **`semio`**.

## Neo4j Cypher Persistence

APOC Core and APOC Extended are installed in the **`semio`** image and configured for file import/export. The canonical repo persistence paths are:

- **`.repo/🛂/semio.cypher`**
- **`.repo/🛂/elements.cypher`**
- **`.repo/🛂/coda.cypher`**
- **`.repo/🛂/reuse.cypher`**

On devcontainer start, **`post-start.sh`** imports non-empty schema files with **`apoc.cypher.runFile`** only when the live database is empty. Export technology-scoped graph state with APOC query exports instead of dumping the whole database, for example:

```cypher
CALL apoc.export.cypher.query(
  'MATCH (n:Semio) OPTIONAL MATCH (n)-[r]->(m:Semio) RETURN n, r, m',
  '/workspaces/semio/.repo/\uD83D\uDEC2/semio.cypher',
  {format: 'cypher-shell'}
);
```

## semio-entrypoint.sh

Legacy helper kept for existing callers. The current setup does not need entrypoint startup logic because **`post-start.sh`** starts Neo4j inside **`semio`**.

## post-create.sh

Devcontainer provisioning steps for dependency installs, including Playwright browser install into the shared cache path and Linux GitKraken Desktop plus GitKraken CLI installation into persisted user profile locations.

## post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, writes Neo4j MCP environment defaults, configures and starts the local Neo4j service, checks **`localhost:7687`**, and activates the Python virtual environment.

## post-attach.sh

Devcontainer post-attach script that uninstalls any existing repo extension via IDE IPC hook CLIs and extensions directory cleanup, clears stale VS Code and Cursor caches, builds and installs the local semio extension via VS Code, Cursor, Windsurf, or Antigravity IPC hook CLIs with list-extensions validation and extensions directory fallback plus extensions.json registration (using `$mid` location keys) on WSL-only CLI responses, generates Windsurf and Codex MCP configs from the repo `.mcp.json`, installs Linux GitKraken Desktop plus CLI when missing, and bootstraps a GitKraken local workspace for the repo plus submodules.

## Devcontainer Persistence

Devcontainer rebuilds keep AI tooling state by mounting named volumes for CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`), GitKraken Desktop state (`~/.gitkraken`), GitKraken CLI state (`~/.local/share/GitKrakenCLI`, `~/.local/share/gk`), and editor servers (`~/.vscode-server`, `~/.windsurf-server`).
Claude Code persists its auth files by storing `~/.claude.json` inside the mounted Claude volume and linking it back into `$HOME` on start.
Post-start ownership fixes keep the mounted volumes writable so chat history and tokens survive container replacement.
Post-attach reconciles VS Code workspace chat storage for `GitHub.copilot-chat` and `openai.chatgpt` by merging transcript and chat resource folders from older workspace-storage hashes into the active workspace-storage directories after attach.
Post-attach uninstalls any existing repo extension across IDE IPC hook CLIs and extensions directories, clears stale VS Code and Cursor caches, installs the fresh VSIX, validates installs by checking list-extensions output, and falls back to direct extensions directory installs plus extensions.json registration (with `$mid` location keys) when CLIs report WSL-only usage.
Post-attach also materializes Windsurf's MCP config at `~/.codeium/windsurf/mcp_config.json` and merges Codex MCP server entries into `~/.codex/config.toml` from the monorepo `.mcp.json`, so both clients pick up the repo, semio, coda, and Playwright servers after rebuilds without manual setup while preserving existing Codex user settings such as model and personality.
Post-create installs Linux GitKraken Desktop plus the official GitKraken `gk` CLI into the devcontainer, and post-attach creates or updates the default local GitKraken workspace from the repo root plus submodules so the Linux GitKraken app picks up the monorepo layout without manual workspace setup.
Engine compatibility for the local extension is aligned to the lowest supported editor build so Cursor and VS Code accept the same VSIX.

## Emoji Font Setup

The devcontainer image installs comprehensive emoji font support including `fonts-noto-color-emoji`, `fonts-noto-cjk`, `fonts-noto-mono`, and additional font packages. Font configuration is automatically applied to ensure emoji rendering works across all applications.

### Font Configuration

- **Automatic fontconfig setup**: Scripts configure `/etc/fonts/local.conf` with proper emoji font fallbacks
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

The devcontainer packages the workspace VS Code extension during setup, uninstalls any existing repo extension on attach, and installs the generated `.vsix` across supported IDEs so the extension is ready without manual "Install Extension From Location..." steps.
This keeps the active editor clean of stale versions while aligning installation with a running IDE server, avoiding failures during container creation and preserving automatic delivery.

## GitKraken Zero Touch

GitKraken zero-touch setup persists Linux GitKraken Desktop state, the `gk` runtime, and local workspace metadata across rebuilds and refreshes the Semio workspace automatically on attach.
The bootstrap targets the repo root and declared git submodules, then sets the Semio GitKraken workspace as the default so the same graph opens immediately in Linux GitKraken Desktop.

### WSL Compatibility

The devcontainer automatically detects WSL environments and starts GitKraken with the `--no-sandbox` flag to handle namespace restrictions. This ensures GitKraken works seamlessly in WSL without manual intervention.

### VS Code Integration

A VS Code task is available for launching GitKraken:

- Use `Ctrl+Shift+P` → "Tasks: Run Task" → "🐧gitkraken"
- Or run from terminal: `bash .devcontainer/gitkraken-launch.sh`

The launcher script automatically:

- Detects if GitKraken is already running
- Applies WSL-compatible flags (`--no-sandbox --no-debug`)
- Prevents debugger hanging issues
- Launches GitKraken in the background

### Environment Variables

Configure GitKraken behavior with these environment variables:

- `SEMIO_GITKRAKEN_WORKSPACE_NAME`: Workspace name (default: "semio")
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

Playwright browser downloads live under the workspace `node_modules` volume so the binaries persist across container restarts and editor reloads.
The devcontainer sets `PLAYWRIGHT_BROWSERS_PATH` to the shared cache location, and the provisioning script installs Chromium into that path so `npx playwright install` is a no-op once cached.

# 💯Requirements

## Devcontainer

Devcontainer provisioning MUST install the workspace VS Code extension automatically after editor attach without manual installation steps.

Devcontainer post-attach MUST uninstall any existing repo extension via IDE IPC hook CLIs and extensions directory cleanup, clear stale VS Code and Cursor extension caches, install the workspace extension for VS Code, Cursor, Windsurf, and Antigravity, validate installs with list-extensions, and fall back to direct extensions directory installs with extensions.json updates that include mid location keys when CLIs report WSL-only usage.

Devcontainer post-attach MUST generate Windsurf MCP config, write `.cursor/mcp.json` with repo-root-absolute MCP commands (so Cursor discovers stdio servers even when the spawn cwd is not the repo root), and merge Codex MCP server entries from the monorepo `.mcp.json` into the clients' home config folders without removing unrelated Codex user settings.

Semio VS Code extension engine compatibility MUST include Cursor's supported VS Code version range.

Playwright browser caches MUST use the workspace node_modules volume path so browser install stays cached across reloads.

Claude Code and Codex auth plus chat history MUST persist across devcontainer rebuilds via named volumes for CLI config and editor server state.
VS Code chat-provider workspace history MUST persist across devcontainer rebuilds even when the active `workspaceStorage` hash changes for the same repo.

Claude Code auth files MUST live in the persisted Claude volume and be linked into the home directory.

Devcontainer provisioning MUST install Linux GitKraken Desktop and the official GitKraken `gk` CLI when they are missing.

Devcontainer lifecycle scripts MUST persist GitKraken CLI runtime files and local workspace metadata across rebuilds.

Devcontainer post-attach MUST create or update the default Semio GitKraken local workspace from the repo root and submodules without manual GitKraken setup.

Devcontainer provisioning MUST install a color emoji font and refresh fontconfig caches so GUI applications render emoji glyphs without manual setup.

Devcontainer lifecycle scripts MUST enforce fontconfig fallback to `Noto Color Emoji` for the common font families used by Electron and GTK applications.
