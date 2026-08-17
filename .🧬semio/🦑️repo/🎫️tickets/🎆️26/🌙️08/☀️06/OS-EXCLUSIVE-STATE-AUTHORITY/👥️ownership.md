# Ownership — Os Exclusive State Authority

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Wave 0:** baseline only (this file). Later waves MUST stay inside disjoint globs below.

## Integrator (single owner)

Exclusive edit rights for repo-wide glue; merges `📥️integration-requests.md` entries.

| Glob / path | Notes |
|---|---|
| `Cargo.toml` | workspace members, `[workspace.dependencies]` |
| `Cargo.lock` | lockfile after member/dep churn |
| `📜️script.ts` (repo root) | verify gate, nx orchestration |
| `.dependency-cruiser.cjs` | dependency policy |
| `eslint.config.mjs` | lint policy |
| `nx.json` | task graph |
| `**/📋️project.json` | nx project wiring (any depth) |
| `.vscode/launch.json` | dev launch entries |

**Rule:** Every other agent that needs a root-file or integrator-only change MUST append a row to `📥️integration-requests.md` (append-only). Do not edit integrator roots directly.

## Wave 1a — Store / VCS / DB

| Glob | Owner wave |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**` | 1a Store |
| `🌿️vcs/**` | 1a Store (VCS backbone) |
| `🛢️db/**` | 1a Store (persistence leaf under OS db module tree) |

Host-authoritative `DocumentStore`, receiverless document app contract, store ↔ VCS sync.

## Wave 1a — Engine (new tree)

| Glob | Owner wave |
|---|---|
| `🧰️framework/🛍️products/💻️os/**/⚙️engine/**` | 1a Engine |

Content-addressed engine hosts live under OS product only. Cross-crate path/member requests → `📥️integration-requests.md`.

## Wave 1a — Draft lane

| Scope | Owner wave |
|---|---|
| Draft / ephemeral state regions inside `🏪️store/**` glue and aligned host APIs | 1a Draft |

Coordinated with Wave 1a Store (same agent or explicit handoff in ticket notes). No parallel edits to store `DocumentStore` invariants without draft owner sync.

## Wave 1b — Plugin / SPR / WIT

| Glob | Owner wave |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` | 1b Plugin |
| `🧰️framework/🛍️products/💻️os/**/📡️spr/**` | 1b Plugin (SPR channel wire) |
| `🧰️framework/🛍️products/💻️os/**/🌍️world/**` (`world.wit` and companions) | 1b Plugin |

Plugin host, registry, wasm world surface; no new plugin-local authoritative document state.

## Wave 2 — s kernels (Rust)

| Glob | Owner wave |
|---|---|
| `✏️s/🔨️modules/**` | 2s-kernels |
| `✏️s/🔌️plugins/**/🗿️artifacts/**` | 2s-plugins-artifacts |
| `✏️s/🔌️plugins/**/⚙️engine/**` | 2s-plugins-engine |
| `✏️s/🔌️plugins/**/🎛️apps/**` | 2s-plugins-apps |

Migrate hosts, static globals, and session structs to OS store/draft/engine; delete local CORE state.

## Wave 2 — framework (non-OS Rust)

| Glob | Owner wave |
|---|---|
| `🧰️framework/🔨️modules/**` | 2framework-rust |
| `🧰️framework/📦️packages/🦀️rust/**` | 2framework-rust |

**Exclude:** `🧰️framework/🛍️products/💻️os/**` (owned by 1a/1b).

## Wave 2 — TypeScript (framework + s plugins)

| Glob | Owner wave |
|---|---|
| `🧰️framework/🔨️modules/**/📦️packages/🟦️typescript/**` | 2framework-ts |
| `🧰️framework/📦️packages/🟦️typescript/**` | 2framework-ts |
| `✏️s/**/📦️packages/🟦️typescript/**` | 2s-plugins-ts |
| `✏️s/🔌️plugins/**/🔨️modules/**` | 2s-plugins-ts (CAD core, stately, brepjs, …) |

## Wave 3 — Seal (integrator + policy)

| Glob | Owner wave |
|---|---|
| Repo-root verify gate rules inside `📜️script.ts` | Integrator |
| New policy lint for “no CORE state outside OS” | Integrator (after 2* lands) |

## Disjointness

- No glob overlap between concurrent waves except **1a Store** ↔ **1a Draft** (coordinated).
- `🧰️framework/🛍️products/💻️os/**` outside the table rows above defaults to **1a** owners; ask integrator if unsure.
- Ticket folder `.🦑️repo/🎫️tickets/**/OS-EXCLUSIVE-STATE-AUTHORITY/**` — any agent (logs, handoffs).
