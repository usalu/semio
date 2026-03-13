# Summary

Devcontainer configuration and lifecycle scripts.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and Playwright cache under `node_modules`.

## post-create.sh

Devcontainer provisioning steps for dependency installs, including Playwright browser install into the shared cache path and Linux GitKraken Desktop plus GitKraken CLI installation into persisted user profile locations.

## post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, and activates the Python virtual environment.

## post-attach.sh

Devcontainer post-attach script that uninstalls any existing semio-repo extension via IDE IPC hook CLIs and extensions directory cleanup, clears stale VS Code and Cursor caches, builds and installs the local semio extension via VS Code, Cursor, Windsurf, or Antigravity IPC hook CLIs with list-extensions validation and extensions directory fallback plus extensions.json registration (using `$mid` location keys) on WSL-only CLI responses, generates Windsurf and Codex MCP configs from the repo `.mcp.json`, installs Linux GitKraken Desktop plus CLI when missing, and bootstraps a GitKraken local workspace for the repo plus submodules.

## Devcontainer Persistence

Devcontainer rebuilds keep AI tooling state by mounting named volumes for CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`), GitKraken Desktop state (`~/.gitkraken`), GitKraken CLI state (`~/.local/share/GitKrakenCLI`, `~/.local/share/gk`), and editor servers (`~/.vscode-server`, `~/.windsurf-server`).
Claude Code persists its auth files by storing `~/.claude.json` inside the mounted Claude volume and linking it back into `$HOME` on start.
Post-start ownership fixes keep the mounted volumes writable so chat history and tokens survive container replacement.
Post-attach uninstalls any existing semio-repo extension across IDE IPC hook CLIs and extensions directories, clears stale VS Code and Cursor caches, installs the fresh VSIX, validates installs by checking list-extensions output, and falls back to direct extensions directory installs plus extensions.json registration (with `$mid` location keys) when CLIs report WSL-only usage.
Post-attach also materializes Windsurf's MCP config at `~/.codeium/windsurf/mcp_config.json` and Codex's MCP config at `~/.codex/config.toml` from the monorepo `.mcp.json`, so both clients pick up the semio-repo, semio, coda, and Playwright servers after rebuilds without manual setup.
Post-create installs Linux GitKraken Desktop plus the official GitKraken `gk` CLI into the devcontainer, and post-attach creates or updates the default local GitKraken workspace from the repo root plus submodules so the Linux GitKraken app picks up the monorepo layout without manual workspace setup.
Engine compatibility for the local extension is aligned to the lowest supported editor build so Cursor and VS Code accept the same VSIX.

## Emoji Font Setup

The devcontainer image installs `fonts-noto-color-emoji` and refreshes `fontconfig` caches so Electron and GTK applications can resolve color emoji glyphs without manual package installation.
Lifecycle scripts also write `/etc/fonts/local.conf` to prepend `Noto Color Emoji` for `sans-serif`, `serif`, and `monospace`, which fixes placeholder glyphs in GUI applications such as VS Code testing surfaces and GitKraken after attach or restart.

## Devcontainer Extension Install

The devcontainer packages the workspace VS Code extension during setup, uninstalls any existing semio-repo extension on attach, and installs the generated `.vsix` across supported IDEs so the extension is ready without manual "Install Extension From Location..." steps.
This keeps the active editor clean of stale versions while aligning installation with a running IDE server, avoiding failures during container creation and preserving automatic delivery.

## GitKraken Zero Touch

GitKraken zero-touch setup persists Linux GitKraken Desktop state, the `gk` runtime, and local workspace metadata across rebuilds and refreshes the Semio workspace automatically on attach.
The bootstrap targets the repo root and declared git submodules, then sets the Semio GitKraken workspace as the default so the same graph opens immediately in Linux GitKraken Desktop.

## Search Tooling

The devcontainer image installs ripgrep (`rg`) as part of the base apt package set so fast recursive code search is available immediately in all editor terminals and scripts.

## Playwright Browser Cache

Playwright browser downloads live under the workspace `node_modules` volume so the binaries persist across container restarts and editor reloads.
The devcontainer sets `PLAYWRIGHT_BROWSERS_PATH` to the shared cache location, and the provisioning script installs Chromium into that path so `npx playwright install` is a no-op once cached.

# 💯Requirements

## Devcontainer

Devcontainer provisioning MUST install the workspace VS Code extension automatically after editor attach without manual installation steps.

Devcontainer post-attach MUST uninstall any existing semio-repo extension via IDE IPC hook CLIs and extensions directory cleanup, clear stale VS Code and Cursor extension caches, install the workspace extension for VS Code, Cursor, Windsurf, and Antigravity, validate installs with list-extensions, and fall back to direct extensions directory installs with extensions.json updates that include mid location keys when CLIs report WSL-only usage.

Devcontainer post-attach MUST generate Windsurf and Codex MCP configs from the monorepo `.mcp.json` and persist them in the clients' home config folders.

Semio VS Code extension engine compatibility MUST include Cursor's supported VS Code version range.

Playwright browser caches MUST use the workspace node_modules volume path so browser install stays cached across reloads.

Claude Code and Codex auth plus chat history MUST persist across devcontainer rebuilds via named volumes for CLI config and editor server state.

Claude Code auth files MUST live in the persisted Claude volume and be linked into the home directory.

Devcontainer provisioning MUST install Linux GitKraken Desktop and the official GitKraken `gk` CLI when they are missing.

Devcontainer lifecycle scripts MUST persist GitKraken CLI runtime files and local workspace metadata across rebuilds.

Devcontainer post-attach MUST create or update the default Semio GitKraken local workspace from the repo root and submodules without manual GitKraken setup.

Devcontainer provisioning MUST install a color emoji font and refresh fontconfig caches so GUI applications render emoji glyphs without manual setup.

Devcontainer lifecycle scripts MUST enforce fontconfig fallback to `Noto Color Emoji` for the common font families used by Electron and GTK applications.
