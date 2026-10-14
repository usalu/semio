# Build Tool Surface Audit

Read-only source audit; no tests or runtime behavior verified.

## Dashboard

Under `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`:

- `🎮️registry/🦀️.rs`: direct Nx argv at 1405, 1407, 1441; `invokes_nx` at 1569; Nx execution policy, graph reuse and file-map hashing at 588–698 and 1235–1257. Project/target discovery and parameter effects expose Nx through `nxFlags`.
- `📚️inventory/🦀️.rs`: `.nx/workspace-data/project-graph.json` at 18, graph diagnostics at 194, graph publication/read validation at 208 and 453.
- `🧬️schema/🎮️registry/🔣️.json`: `NxFlags`, `nxFlags`, backend-specific project/target/group semantics. Replace public effect vocabulary with neutral task options; backend translates options.
- `🌳️command-tree/🚀️launch/🟦️.ts:55`: emits `bun nx run @semio-tech/repo-dashboard-rs:launch -- run ...`; update generator and its matching `🧬️schema/🚀️launch/🔣️.json:99` together.
- `🧬️schema/🧭️authority/🔣️.json:10–16`: hardcoded root nx/setup/dashboard/install script authority.
- `🔌️plugin-registry/🦀️.rs:23,36`: direct catalog Nx launch and help message.
- `📦️installation/🟦️.ts:160`: strips Nx options on installed launch; make neutral option parsing authoritative.
- `📦️packages/🦀️rust/🦀️.rs:329` and `🌀️daemon/🧠️supervisor/🦀️.rs:42,891`: backend environment ownership belongs in adapter.
- README and oracle descriptions currently identify the command model with Nx.

## Root and Agent Interfaces

- Root `📜️script.ts` hardcodes `bun nx` at 454, 7776, 7814, 8340–8372, 8507, 14274–14321. The test prerequisite guard at 13600 assumes `NX_TASK_TARGET_PROJECT=workspace`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts:19` reads `NX_WORKSPACE_ROOT`. Introduce repository task/root context and have Nx adapter populate/normalize it.
- Root `📜️script.ts:151–156,227` recreates `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md` as links to AGENTS. Editing aliases could modify AGENTS and is forbidden. Supply neutral tool discoverability through MCP schemas/help and README instead.
- Root README has current developer command examples and dashboard vocabulary at 176, 215, 634, 674–732, 841, 881.

## Editor, Container and CI

- `.vscode/launch.json` is a large derived launch document (over 116k lines). Update its owners/generators, including `.vscode/🧩️launch.seed.jsonc`, rather than leaving generator drift. Existing entries carry Nx argv both as command text and serialized SEMIO_SCRIPT_CAPABILITIES arguments/selection/launches.
- `.vscode/settings.json:23–38` publishes NX logging/workspace-data settings for all native hosts.
- `.devcontainer/devcontainer.json:40` postCreate uses `bun nx run workspace:setup`; 113–115 and 192–195 carry Nx env. `.devcontainer/Dockerfile:100` sets NX_WORKSPACE_DATA_DIRECTORY. Editor settings overlay is generated from canonical .vscode settings, per its README.
- `.github/workflows/architecture-quiz.yml:65,67,111,118` calls Nx directly for test/publish/docker image targets.
- Server implementation search found only an Nx docstring in coordinator Go script line 45, not runtime invocation.

## Recommended Boundary

Expose neutral task discovery, task identity, selection/groups, configuration, arguments, cache/dependency options, graph inventory, progress and cancellation. Keep Nx default and isolate its graph/file-map, flags, environment and executable under its backend. Dashboard consumes neutral inventory and produces neutral invocations; Bazel consumes the same task contract. Preserve existing manifest metadata initially as source input, while backend-specific data stays inside the Nx implementation. Run fixture conformance for neutral launch vectors and ensure third-party Nx/Bazel observations match task selection and output.
