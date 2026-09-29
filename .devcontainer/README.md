# Summary

Devcontainer configuration: one Compose service, the image, and the lifecycle commands `devcontainer.json` runs.

# Docs

## devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, the create/start/attach lifecycle commands, and persisted volumes for AI auth, editor server state, GitKraken workspace state, and the shared `.🧬semio/🦑️repo/⚡️cache` root (cargo, Nx, Playwright, and every other repo-managed build cache).

It forwards exactly the ports the launch rows start (a law in `⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap` keeps the list equal to the rows): `os-hub` 8787 (`🛠️dev🗄️os-hub`), the `s` React serve 6070 and its two-person pair 6072/6073, the `s` wgpu serve 6066 and 6067/6068, Storybook 6010, and the MCP Inspector 6274/6277. `hostRequirements` states what a container that builds the hub needs (4 CPUs, 8 GB memory, 32 GB disk: a fresh-clone run measured 7 GB and 25 GB). The hub's Postgres/Neo4j backends (`os-hub-ts:backend-up`) run in the container's own Docker daemon (`docker-in-docker`), whose state persists in that feature's volume.

On macOS, Docker Desktop must be allowed to read the folder the repository lives in (System Settings → Privacy & Security → Files and Folders / Full Disk Access for Docker); without it the bind mount of the checkout fails with `operation not permitted`.

## docker-compose.yml

Compose stack for the devcontainer: one service, **`semio`**. It sets no project, image or container name, so the devcontainer CLI names the project after the checkout folder and two checkouts run side by side.

## Dockerfile

Ubuntu 24.04 devcontainer base with the build toolchain, fonts (`fonts-noto-color-emoji`, CJK, mono), headless-browser libraries, `xvfb`, `ripgrep` and `sqlite3`, plus Bun 1.3.14 and Node 24.15.0 from pinned, checksum-verified Linux x64/arm64 archives. Nx comes from the repository's locked tooling bootstrap.

## Lifecycle

Every lifecycle step is a program argv — no shell script — implemented once in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts` and proven by the law next to it (`🧪️tests/🔁️lifecycle`, recording hosts, no container needed):

| Hook | Command | What it does |
|---|---|---|
| `onCreateCommand` | `sudo chown vscode:vscode` of the two checkout volumes | Docker creates them root-owned |
| `postCreateCommand` | `bun nx run workspace:setup` | every language environment, the generated sources, the agent instruction aliases and both MCP binaries |
| `postStartCommand` | `bun ./📜️script.ts setup devcontainer start` | hands the persisted home volumes and submodules to `vscode`; Noto Color Emoji fontconfig fallback for `sans-serif`/`serif`/`monospace`; keeps `~/.claude.json` inside the Claude volume and links it back; adds only the missing git `safe.directory` entries; with `~/.ssh/id_ed25519_signing.pub`, SSH commit/tag signing through one agent on `~/.ssh/semio-ssh-agent.sock` |
| `postAttachCommand` | `bun ./📜️script.ts setup devcontainer attach` | installs GitKraken Desktop, the `gk` CLI and F3D when missing; creates or completes the GitKraken workspace from the checkout and its submodules and sets it as default; syncs the repo hook configuration; packages the workspace VS Code extension through `@semio-tech/repo-vscode:build-vsix` and installs it into the first editor CLI that confirms it |
| on demand | `bun ./📜️script.ts setup devcontainer gitkraken` | starts GitKraken Desktop on the checkout, on the virtual display `:99` (Xvfb) when none is set |

Optional steps report and continue; a start never runs a destructive source-control or database command. Environment: `SEMIO_GITKRAKEN_WORKSPACE_NAME` (workspace name), `SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL` (skip the GUI tool installs), `SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL` (skip the extension), `SEMIO_REPO_IMPLEMENTATION=go` (sync hooks through the Go repo client).

MCP client configurations are repository files (`.mcp.json`, `.cursor/mcp.json`, `.codex/config.toml`, …) kept consistent by the root `📜️script.ts` policy, not by the container lifecycle.

## Devcontainer Persistence

Named volumes keep CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`, `~/.config/gh`), GitKraken Desktop and CLI state (`~/.gitkraken`, `~/.local/share/GitKrakenCLI`, `~/.local/share/gk`), editor servers (`~/.vscode-server`, `~/.cursor-server`, `~/.windsurf-server`, `~/.antigravity-server`) and `~/.kiro` — shared by every checkout with the same folder name. `node_modules` and the build/tool cache root `.🧬semio/🦑️repo/⚡️cache` (cargo, Nx, Vite, Playwright, Go, …) are volumes private to one checkout (`${devcontainerId}`), so rebuilds never start cold and a second clone never reads the first one's dependencies or build state; `onCreateCommand` hands both (created root-owned by Docker) to `vscode` before `postCreateCommand` installs into them. `PLAYWRIGHT_BROWSERS_PATH` points into the cache volume.

# 💯️Requirements

## Devcontainer

Devcontainer lifecycle commands MUST be program argv run through `bun ./📜️script.ts setup devcontainer <start|attach|gitkraken>`, never shell scripts.

Devcontainer attach MUST resolve the workspace extension through the Nx packaging target, install only after that target succeeds, and validate the installation through the editor CLI's extension list.

Devcontainer start MUST NOT run destructive source-control or database commands and MUST NOT append duplicate git `safe.directory` entries.

Claude Code auth files MUST live in the persisted Claude volume and be linked into the home directory.

Devcontainer attach MUST install Linux GitKraken Desktop and the `gk` CLI when missing and create or complete the GitKraken workspace from the repo root and submodules without manual setup.

Devcontainer start MUST enforce the fontconfig fallback to `Noto Color Emoji` for the generic font families used by Electron and GTK applications.

Forwarded ports MUST equal the ports the launch rows start.

Playwright browser caches MUST use the shared `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` path.
