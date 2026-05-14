# Summary

Devcontainer configuration and lifecycle scripts.

# Neo4j Desktop and MCP

The monorepo registers four Neo4j MCP servers (`neo4j-semio`, `neo4j-elements`, `neo4j-coda`, `neo4j-reuse`) in `.mcp.json` and per-client copies. Each server uses `uvx mcp-neo4j-cypher` and a dedicated database name matching the technology.

**Native (Windows / macOS / Linux):** Run your platform bootstrap (`.devcontainer/install-native.ps1` or `.devcontainer/install-native.sh`) so `NEO4J_URI=bolt://localhost:7687` and credentials are set. Start Neo4j Desktop 2 locally; Bolt listens on `7687` by default.

**Devcontainer:** Neo4j 5 runs as the **`neo4j` Compose service** (see `.devcontainer/docker-compose.yml`). Inside the workspace container, `NEO4J_URI` is **`bolt://neo4j:7687`**. The **`semio`** service **publishes `7687` and `7474` to the Docker host** (so **Windows `127.0.0.1:7687`** reaches the dev container without relying on the editor Ports tunnel). **`neo4j-host-forward.sh`** (from `post-start.sh` / `post-attach.sh`) runs **`socat`** on **`0.0.0.0:7687`** / **`7474`** inside **`semio`** and forwards to **`neo4j`**. `post-start.sh` also runs **`neo4j-bootstrap-databases.py`** so databases **`semio`**, **`elements`**, **`coda`**, and **`reuse`** exist.

**Bonus — Neo4j Desktop on the Windows host:** 1) **Docker Desktop** must be running. 2) **Reopen in Container** (or recreate the Compose stack) so **`semio`** picks up **`ports: 7687:7687`**. 3) Wait until **post-start** has run (or run `bash .devcontainer/neo4j-host-forward.sh` in a devcontainer terminal). 4) **`Test-NetConnection -ComputerName 127.0.0.1 -Port 7687`** should show **`TcpTestSucceeded : True`**. 5) In Desktop: **`bolt://127.0.0.1:7687`**, user **`neo4j`**, password **`password`**. If port **7687** is already taken on Windows (e.g. local Neo4j Desktop), change the **left** side in compose to e.g. **`17687:7687`** and connect on **`17687`**.

**One-time databases:** In the devcontainer they are created automatically. On native Desktop only, create the four databases once if missing.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and Playwright cache under `node_modules`.

## docker-compose.yml

Compose stack for the devcontainer: **`semio`** (this workspace image + features) and **`neo4j`** (official `neo4j:5-community` with `NEO4J_AUTH=neo4j/password`). **`semio`** publishes **`7687:7687`** and **`7474:7474`** to the Docker host; **`neo4j-host-forward.sh`** binds those ports inside **`semio`** and forwards to **`neo4j`**. Neo4j itself has no host ports. MCP uses **`bolt://neo4j:7687`** from inside the app container.

## post-create.sh

Devcontainer provisioning steps for dependency installs, including Playwright browser install into the shared cache path and Linux GitKraken Desktop plus GitKraken CLI installation into persisted user profile locations.

## post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, runs **`neo4j-host-forward.sh`** (Bolt/HTTP for editor port forwarding), bootstraps Neo4j databases via **`neo4j-bootstrap-databases.py`**, and activates the Python virtual environment.

## post-attach.sh

Devcontainer post-attach script that runs **`neo4j-host-forward.sh`** (so Neo4j Bolt/HTTP stay available for editor port forwarding after each attach), uninstalls any existing repo extension via IDE IPC hook CLIs and extensions directory cleanup, clears stale VS Code and Cursor caches, builds and installs the local semio extension via VS Code, Cursor, Windsurf, or Antigravity IPC hook CLIs with list-extensions validation and extensions directory fallback plus extensions.json registration (using `$mid` location keys) on WSL-only CLI responses, generates Windsurf and Codex MCP configs from the repo `.mcp.json`, installs Linux GitKraken Desktop plus CLI when missing, and bootstraps a GitKraken local workspace for the repo plus submodules.

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
