# Summary

Devcontainer configuration and lifecycle scripts.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and the shared `.🧬semio/🦑️repo/⚡️cache` root (cargo, Nx, Playwright, and every other repo-managed build cache).

## docker-compose.yml

Compose stack for the devcontainer: **`compose`** only.

## Dependency Preparation

The image provides Bun 1.3.14 and Node 24.15.0 from pinned, checksum-verified Linux x64/arm64 archives. Nx comes from the repository's locked tooling bootstrap. Container creation invokes `bun nx run workspace:deps-javascript`, which synchronizes the frozen Bun lockfile without building applications. Select additional dependency environments and project builds through their Nx launch configurations. Post-start and post-attach remain separate lifecycle hooks.

## post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, and activates the Python virtual environment.

## post-attach.sh

The attach hook detects the active editor CLI and invokes `bun nx run @semio-tech/repo-vscode:build-vsix`. Nx owns source invalidation, the build prerequisite and VSIX restoration. A successful package is installed through the selected editor CLI and verified with its extension list; a failed package skips installation. Linux developer-tool and repo configuration steps remain separate parts of this hook.

## Devcontainer Persistence

Devcontainer rebuilds keep AI tooling state by mounting named volumes for CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`), GitKraken Desktop state (`~/.gitkraken`), GitKraken CLI state (`~/.local/share/GitKrakenCLI`, `~/.local/share/gk`), and editor servers (`~/.vscode-server`, `~/.windsurf-server`).
One additional named volume mounts at `${containerWorkspaceFolder}/.🧬semio/🦑️repo/⚡️cache` — the single shared build/tool cache root (cargo, Nx, Vite, Playwright, Go, …) — so every agent and dev process in the container reuses the same cache and rebuilds never start cold.
Claude Code persists its auth files by storing `~/.claude.json` inside the mounted Claude volume and linking it back into `$HOME` on start.
Post-start ownership fixes keep the mounted volumes writable so chat history and tokens survive container replacement.
Post-attach reconciles VS Code workspace chat storage for `GitHub.copilot-chat` and `openai.chatgpt` by merging transcript and chat resource folders from older workspace-storage hashes into the active workspace-storage directories after attach.
Post-attach asks Nx for the current VSIX on every enabled attach, then installs that package with the editor CLI. Source or archive timestamps do not decide whether a build is needed.
Post-attach also materializes Windsurf's MCP config at `~/.codeium/windsurf/mcp_config.json` and merges Codex MCP server entries into `~/.codex/config.toml` from the monorepo `.mcp.json`, so both clients pick up the `repo` and `semio` servers after rebuilds without manual setup while preserving existing Codex user settings such as model and personality.
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
