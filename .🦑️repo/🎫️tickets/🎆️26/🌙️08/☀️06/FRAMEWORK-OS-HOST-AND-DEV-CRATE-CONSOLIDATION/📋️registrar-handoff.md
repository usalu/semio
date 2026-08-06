# 📋️ Registrar Handoff — Framework Os Host And Dev Crate Consolidation (W8c)

**Status:** `flagged` — Shape V2 host/dev packages **on disk**; **old `⚡️implementations` sandwiches retained**; root `cargo check` **not run** (unrelated dangling workspace member `…/⌨️cli/⚡️implementations/🦀️rust`).

**Ticket:** `26/08/06/FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION`  
**Goal:** `🎯aioptimizedrepo`  
**Coordinate:** `26/08/06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION` owns wasm data plane (`semio-framework-os-kernel`); do **not** delete kernel `⚡️implementations` until that ticket’s wasm admission is green.

## Three-crate boundary (normative)

| Role | Crate | Owner / package path |
|---|---|---|
| Guest (wasm component) | `semio-framework-plugin` | `💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/` (kernel ticket) |
| Wasm kernel | `semio-framework-os-kernel` | `💻️os/📦️packages/🦀️rust/` (kernel ticket) |
| Native host | `semio-framework-os` | `💻️os/🖥️host/📦️packages/🦀️rust/` (**this ticket**) |

Host deps still path-alias **legacy** `store`/`protocol`/`dsl`/`vcs` sandwiches until kernel registrar cut-over.

## What landed (Shape V2)

| Crate | New path | Notes |
|---|---|---|
| `semio-framework-os` | `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/` | `role = "product"`, `id = "os-host"`. Entry `📦️lib.rs` bridges legacy `💻️os/⚡️implementations/🦀️rust/📦️lib.rs`. `AGENTS.md` copied to `🖥️host/AGENTS.md`. |
| `semio-framework-os-kernel-db` | `…/🛢️db/📦️packages/🦀️rust/` | Facade; `sqlite`/`postgres`/`neo4j` features preserved. |
| `semio-framework-plugin-host` | `…/🔌️plugin/🖥️host/📦️packages/🦀️rust/` | Native wasmtime host; fixes broken legacy `[lib] path` on old sandwich manifest. |
| `semio-framework-os-run` | `…/🏃️run/📦️packages/🦀️rust/` | `[[bin]]` still points at legacy `📦️bin.rs` (include! paths unchanged). |
| `semio-framework-os-kernel-neural-engine` | `…/🧠️neural/⚙️engine/📦️packages/🦀️rust/` | Fixes legacy manifest pointing at non-existent `⚙️engine/⚡️…/📦️lib.rs`. |
| `@semio-tech/framework-os-dev` | `…/🧑️‍💻️dev/📦️packages/🟦️typescript/` | Thin `📦️index.ts` + `📜️script.ts` delegate to legacy TS sandwich (full tree purity deferred). |

**Not in this slice (deferred):** `🔁️workflow` / `🪐️space` / `📺️renderer` host merge; dev HTML/CSS/TSX co-location; `🔌️plugin-modules` + `🤖️generated` relocation to `🧑️‍💻️dev/` owner root; absorbing `db_*` leaf sandwiches into one host lib.

## Root `Cargo.toml` — registrar only

### Add members (after kernel cut-over is safe; host can land in same pass)

```toml
    "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
```

### Remove members (only after `cargo check -p` each new crate green and dependents repointed)

```toml
    "🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚡️implementations/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/⚡️implementations/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⚡️implementations/🦀️rust",
```

**Do not remove** any `🛢️db/**` leaf `⚡️implementations` members until a follow-up folds them into the facade (24 dirs remain).

### `[workspace.dependencies]` — replace path (35 refs today)

```toml
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust" }
```
→
```toml
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust" }
```

Also repoint convenience aliases if present:

```toml
semio-framework-os-kernel-db = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚡️implementations/🦀️rust" }
semio-framework-os-kernel-neural-engine = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⚡️implementations/🦀️rust" }
```

→ matching `📦️packages/🦀️rust` paths under each owner.

### Bun workspaces / nx

- Add `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript` to generated workspaces (M6 `workspaces --write`).
- Repoint `@semio-tech/framework-os-dev` nx `project.json` root to new `📋️project.json` under `📦️packages/🟦️typescript` (duplicate project name with legacy path until old entry removed — same class of issue as hub `os-hub`).
- Repoint `@semio-tech/framework-os-core-rs` nx project from `💻️os/⚡️implementations/🦀️rust` → `💻️os/🖥️host/📦️packages/🦀️rust` when registrar swaps host member.

## Verification (this ticket)

| Check | Result |
|---|---|
| `cargo check -p semio-framework-os` | **Blocked** — root workspace fails loading missing `…/⌨️cli/⚡️implementations/🦀️rust` (see `🧪cargo-check-baseline.txt`) |
| `cargo metadata --manifest-path …/🖥️host/…/Cargo.toml` | **Blocked** — same root workspace error when resolving inheritance |
| Inventory | `🧪host-inventory.txt` — host-scope impl dirs: db 24, neural 2, run 1, plugin-host 1, os root 1, workflow 1, space 1 |

**Post-registrar:** `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-os -p semio-framework-plugin-host -p semio-framework-os-run -p semio-framework-os-kernel-db -p semio-framework-os-kernel-neural-engine`, then `bun nx run @semio-tech/framework-os-dev:test-quick` (after nx duplicate-name fix).

## Residuals for follow-up

1. Fold `💻️os/⚡️implementations/🦀️rust/📦️lib.rs` into `🖥️host/**/🦀️component.rs` tree (godfile split).
2. Dev Shape V2 tree purity: move `🌐️index.html`, `🎨️globals.css`, `🧩️multi.tsx`, `🔌️plugin-modules/`, `🤖️generated/` to `🧑️‍💻️dev/` owner root; delete legacy `⚡️implementations/🟦️typescript`.
3. Host `semio-framework-os` Cargo.toml: after kernel admission, depend on `semio-framework-os-kernel` with `protocol`/`store`/`dsl`/`vcs` aliases instead of legacy sandwiches.
4. Renderer host glue (`📺️renderer/…/🧊️wgpu`) — coordinate with surface family ticket; still references legacy paths.
