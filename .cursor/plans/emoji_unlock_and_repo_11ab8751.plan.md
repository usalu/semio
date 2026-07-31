---
name: Emoji Unlock And Repo
overview: Unlock reconfigurable basenames (especially script.ts and project.json), keep hard-tool-mandated names, rename .repo to .🦑️repo with full emoji+word folder names, and rewire all discovery/references.
todos:
  - id: ticket-reopen
    content: Reopen/create ticket; bind plan; keep temp scripts in ticket folder
    status: completed
  - id: unlock-script-project
    content: Rename script.ts/project.json; rewire nx plugin, package.json, launch, BundleScript discovery
    status: completed
  - id: repo-stack
    content: Rename .repo to .🦑️repo; expand bare folders to full emoji+word; rewrite path refs and MCP ticket paths
    status: completed
  - id: bare-sweep-verify
    content: Sweep leftover bare emoji dirs; verify cargo/nx/script smoke; close ticket
    status: completed
isProject: false
---

# Unlock Scripts + Expand .🦑️repo Full Names

## Decisions (locked)

- **Unlock when reconfigurable:** `script.ts` → `📜️script.ts`, `project.json` → `📋️project.json`, remaining plain configs that tools accept via path (root `vitest.config.ts` → `🧪️vitest.config.ts`, `eslint.config.mjs` → `🧹️eslint.config.mjs` if loaders allow).
- **Keep hard-mandated:** `Cargo.toml` / `Cargo.lock`, `package.json` / `bun.lock` / `bunfig.toml`, `go.mod` / `go.sum` / `go.work*`, `AGENTS.md` / `CLAUDE.md`, `README.md` / `LICENSE.md` / `CHANGELOG.md` / `CITATION.cff`, `tsconfig.json`, `nx.json`, Next app-router files, git hook install names, `Trunk.toml`, `*.csproj` / `*.sln`, binaries without extension that are installed by name.
- **Repo stack:** `.repo` → `.🦑️repo`, expand every bare-emoji (and plain) folder under it to emoji+word.

## Part A — Unlock `script.ts` / `project.json`

~219 `script.ts` and ~224 `project.json` files.

1. Reopen ticket [EMOJI-PREFIX-ALL-RENAMABLE-FILENAMES](.repo/🎫️/26/07/31/EMOJI-PREFIX-ALL-RENAMABLE-FILENAMES/) (or open a follow-up under same goal) and put temp scripts there.
2. Rename files deepest-first: `script.ts` → `📜️script.ts`, `project.json` → `📋️project.json`.
3. **Rewire discovery (no dual-name shims):**
   - [🟨️nx-plugin.mjs](🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/🟨️nx-plugin.mjs): `createNodesV2: ["**/📜️script.ts", …]`.
   - [📦️index.ts](🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts): every `base !== "script.ts"`, `**/script.ts` walker, usage strings → `📜️script.ts`.
   - All `project.json` / `📋️project.json` `command: "bun ./📜️script.ts …"` → `bun ./📜️script.ts …`.
   - Root [package.json](package.json) scripts, [.vscode/launch.json](.vscode/launch.json), root [📜️script.ts](script.ts) self-refs.
   - Nx project discovery: configure plugin / `workspaceLayout` so projects load from `**/📋️project.json` (custom createNodes or update existing plugin) — default Nx only auto-loads `project.json`, so the emoji name **must** be registered in the plugin.
4. Spot-verify: `bun nx show projects`, `bun ./📜️script.ts` (root), one package `bun ./📜️script.ts test quick`.

## Part B — `.repo` → `.🦑️repo` with full folder names

Physical rename map (apply deepest-first, then rewrite refs):

| Old | New |
| --- | --- |
| `.repo` | `.🦑️repo` |
| `🎫️` | `🎫️tickets` |
| `🎯️` | `🎯️goals` |
| `🛂️` | `🛂️manifest` |
| `📊️` | `📊️metrics` |
| `💬️` | `💬️chat` |
| `💡️` | `💡️ideas` |
| `🧑️‍💻️` | `🧑️‍💻️devs` |
| `✍️` | `✍️notes` |
| `⚡️` | `⚡️cache` |
| `⚡️/🔀️` | `🔀️diff` (under `⚡️cache`) |
| `⚡️/🤖️` | `🤖️generated` (under `⚡️cache`) |
| `prompts` | `💬️prompts` |
| `tmp` | `🧹️tmp` |

Also emoji-prefix remaining plain files in that tree where tools allow (`config.toml` → `📋️config.toml`, `bullets.txt` → `📝️bullets.txt`, etc.); leave nothing bare-emoji as a directory name.

**Reference rewrite** (path-prefix, not blind substring): `.repo/` → `.🦑️repo/`, `.repo/🎫️/` → `.🦑️repo/🎫️tickets/`, etc., across root scripts, CMakePresets (`binaryDir`), README avatar paths, MCP/`ticket_open` docs in code comments, policy allowlist strings, `.gitignore`, `.mcp.json`, launch configs. Do **not** edit `AGENTS.md` (forbidden); leave its `.repo/🎫️/…` text as historical until a human updates that file.

Repo MCP `ticket_*` tools and any hard-coded `.repo/🎫️` paths in the repo product server must be updated to `.🦑️repo/🎫️tickets` in the same pass so new tickets land correctly.

## Part C — Remaining bare emoji dirs elsewhere

Sweep product roots for any leftover bare-emoji directory names (same vocab as prior [build-vocab.py](.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/build-vocab.py) `BARE_EMOJI_WORDS`) and append the word. Skip Cargo-mandated dirnames (`benches`, `tests`, `examples`, `src`) and Next `app/`.

## Safety

- Use path-aware / ordered longest-prefix rewrites — avoid the previous global basename replace that created `🧪️` / `🟨️` / invalid `🐳️Dockerfile` keys.
- After renames: `cargo metadata`, `bun nx show projects`, one `bun ./📜️script.ts` smoke, confirm a ticket path resolves under `.🦑️repo/🎫️tickets/…`.
