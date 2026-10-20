# Summary

Devcontainer configuration: one Compose service, the image, and the lifecycle commands `devcontainer.json` runs.

# Docs

## devcontainer.json

Devcontainer configuration with container/remote env, the create/start/attach lifecycle commands, and persisted volumes for AI auth, GitKraken workspace state, and the shared `.🧬semio/🦑️repo/⚡️cache` root (cargo, Nx, Playwright, and every other repo-managed build cache).

It forwards exactly the ports the dashboard commands start (a law in `⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap` keeps the list equal to those commands): `os-hub` 8787 (`os-hub:dev`), the `s` React serve 6070 and its two-person pair 6072/6073 (`playground:s` with `user-slot=1` and `2`), the `s` wgpu serve 6066 and 6067/6068 (`playground:s renderer=wgpu-wasm`), Storybook 6010 (`workspace:dev-storybook`), and the MCP Inspector 6274/6277 (`tool:workspace/mcp-inspector-os`). Inside the container start them with `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream` or `semio run <command id> --detach --wait-ready`. `hostRequirements` states what a container that builds the hub needs (4 CPUs, 8 GB memory, 128 GB disk: the 64 GiB repository cache budget in `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` plus a measured 25 GB fresh-clone working set and 20 % headroom; a runtime-bootstrap law recomputes the minimum from that policy). The hub's Postgres/Neo4j backends (`os-hub-ts:backend-up`) run in the container's own Docker daemon (`docker-in-docker`), whose state persists in that feature's volume.

On macOS, Docker Desktop must be allowed to read the folder the repository lives in (System Settings → Privacy & Security → Files and Folders / Full Disk Access for Docker); without it the bind mount of the checkout fails with `operation not permitted`.

## docker-compose.yml

Compose stack for the devcontainer: one service, **`semio`**. It sets no project, image or container name, so the devcontainer CLI names the project after the checkout folder and two checkouts run side by side.

The service declares no `privileged`, capability or device; the `docker-in-docker` feature of `devcontainer.json` is applied by the devcontainer CLI with the feature's own metadata, which was not examined, so the cache partitions need no privilege but the container as a whole is not proven unprivileged. The native cache partitions (`tools`, `dependencies`, `products`, also used by Bazel) are directories exchanged atomically with `renameat2(RENAME_EXCHANGE)`, which needs neither `CAP_SYS_ADMIN` nor `/dev/loop-control`; they must live on a case-sensitive filesystem with atomic exchange, which is why `.cache` is a named volume (see Devcontainer Persistence) and not part of the bind-mounted checkout. `bun repo preflight` probes the exchange and case sensitivity and refuses a case-insensitive host mount with the exact probe error. The partitions hold the dependency generation receipt (`.cache/b/d/m/generation`) and the Bazel disk cache (`.cache/b/p/m/bazel/actions`), so a container that keeps the `.cache` volume starts warm; the macOS-only image silencing (`.fseventsd`) does not apply on Linux.

## Dockerfile

Ubuntu 24.04 devcontainer base with the build toolchain, the generated `en_US.UTF-8` locale (`LANG`/`LC_ALL`, the locale the Bazel adapter also sets on darwin), fonts (`fonts-noto-color-emoji`, CJK, mono), headless-browser libraries, `xvfb`, `ripgrep` and `sqlite3`, plus Bun 1.4.2 and Node 24.15.0 from pinned, checksum-verified Linux x64/arm64 archives. The pinned build tools (Nx by default, Bazel on request) are provisioned by `bun repo tools setup all`, which `bun repo run workspace:setup` runs. The platform matrix (macOS, Linux, devcontainer, Windows), the Bazel host path rules and the recovery steps for the cache partitions are in the chapter "Build and run" of `🧰️framework/🛍️products/🦑️repo/README.md`.

## Lifecycle

Every lifecycle step is a program argv — no shell script — implemented once in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts` and proven by the law next to it (`🧪️tests/🔁️lifecycle`, recording hosts, no container needed):

| Hook | Command | What it does |
|---|---|---|
| `onCreateCommand` | `sudo chown vscode:vscode` of the three checkout volumes | Docker creates them root-owned |
| `postCreateCommand` | `bun repo run workspace:setup` | every language environment, the generated sources, the agent instruction aliases and both MCP binaries |
| `postStartCommand` | `bun repo run workspace:setup-devcontainer-start` | hands the persisted home volumes and submodules to `vscode`; Noto Color Emoji fontconfig fallback for `sans-serif`/`serif`/`monospace`; keeps `~/.claude.json` inside the Claude volume and links it back; adds only the missing git `safe.directory` entries; with `~/.ssh/id_ed25519_signing.pub`, SSH commit/tag signing through one agent on `~/.ssh/semio-ssh-agent.sock` |
| `postAttachCommand` | `bun repo run workspace:setup-devcontainer-attach` | installs GitKraken Desktop, the `gk` CLI and F3D when missing; creates or completes the GitKraken workspace from the checkout and its submodules and sets it as default; reinstalls the git hooks (`bun ./📜️script.ts micro-commit install-hooks`) |
| on demand | `bun run dashboard run workspace:setup -- devcontainer gitkraken` | starts GitKraken Desktop on the checkout, on the virtual display `:99` (Xvfb) when none is set |

Optional steps report and continue; a start never runs a destructive source-control or database command. Environment: `SEMIO_GITKRAKEN_WORKSPACE_NAME` (workspace name), `SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL` (skip the GUI tool installs).

Headless agent client configurations (`.mcp.json`, `.codex/config.toml`) consume dashboard-owned server declarations. Choose **New Task**, search `agents write` and start `workspace:agents-write` to derive them. The container lifecycle prepares the environment; developer operations remain dashboard tasks.

The dashboard is the exclusive developer control plane for development, debugging, tests and builds. `bun run dashboard run workspace:generate-config` keeps cargo-nextest scope files equal to the root `.config/nextest.toml` plus their own overrides and checks that every `.github/dependabot.yml` directory exists and holds its manifest. It is the `repo dashboard` of `AGENTS.md`; there is no `bun repo dashboard` verb, and agents and automation use `bun repo …` instead.

## Daily Development

Start `bun run dashboard` inside the container. Choose **New Task** (`Ctrl+B n`),
search the action and owner, complete the form and choose **Start**. Select
`workspace:setup` to rerun preparation, `workspace:test`, `workspace:build`,
`workspace:verify`, `workspace:lint` or `workspace:format` for workspace operations,
or the matching project target for a scoped operation.

Choose a service such as `playground:s`, `os-hub:dev` or `workspace:dev-storybook`;
wait for readiness and use **Open ready address** (`Ctrl+B u`). Task overview
(`Ctrl+B h`) shows status, exit code and output. The task menu offers **Restart**,
**Stop** and **Kill**. For debugging choose **Workspace Terminal**, set working
directory and shell, and invoke the installed debugger within that task; toggle
input with `Ctrl+B t`.

The [dashboard daily guide](../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md#-daily-work)
covers file navigation, repository actions, test selection and authored tools.
The lifecycle argv below are automatic container hooks, not a manual developer
workflow. Native Windows, macOS and Linux use the same dashboard command forms.

## Devcontainer Persistence

Named volumes keep CLI auth folders (`~/.claude`, `~/.codex`, `~/.config/openai`, `~/.config/gh`), GitKraken Desktop and CLI state (`~/.gitkraken`, `~/.local/share/GitKrakenCLI`, `~/.local/share/gk`) — shared by every checkout with the same folder name. `node_modules`, the build/tool cache root `.🧬semio/🦑️repo/⚡️cache` (cargo, Nx, Vite, Playwright, Go, …) and the native cache partition root `.cache` (`.cache/b`: tools, dependencies, products, graph, Nx, diagnostics and leases, all sized by the budget policy) are volumes private to one checkout (`${devcontainerId}`), so rebuilds never start cold, the 64 GiB cache budget never lands on the bind-mounted checkout, and a second clone never reads the first one's dependencies or build state; `onCreateCommand` hands all three (created root-owned by Docker) to `vscode` before `postCreateCommand` installs into them. `PLAYWRIGHT_BROWSERS_PATH` points into the cache volume.

# 💯️Requirements

## Devcontainer

Devcontainer lifecycle commands MUST be program argv routed through their registered Nx targets to the root `📜️script.ts`. Developers choose `workspace:setup-devcontainer-start`, `workspace:setup-devcontainer-attach` or `workspace:setup -- devcontainer gitkraken` in the dashboard to rerun those operations.

Devcontainer attach MUST prepare repository tools and hooks without installing editors or extensions.

Devcontainer start MUST NOT run destructive source-control or database commands and MUST NOT append duplicate git `safe.directory` entries.

Claude Code auth files MUST live in the persisted Claude volume and be linked into the home directory.

Devcontainer attach MUST install Linux GitKraken Desktop and the `gk` CLI when missing and create or complete the GitKraken workspace from the repo root and submodules without manual setup.

Devcontainer start MUST enforce the fontconfig fallback to `Noto Color Emoji` for the generic font families used by Electron and GTK applications.

Forwarded ports MUST equal the ports the dashboard commands start.

Playwright browser caches MUST use the shared `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` path.
