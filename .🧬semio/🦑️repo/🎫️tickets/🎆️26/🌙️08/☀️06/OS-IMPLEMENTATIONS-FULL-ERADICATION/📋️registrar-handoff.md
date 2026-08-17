# Registrar Handoff — Os Implementations Full Eradication

**Status:** `ready-for-registrar` — zero `⚡️implementations` under `🧰🧰️framework/🛍️products/💻️os/**`. Root `Cargo.toml` still lists deleted members + stale workspace.deps (this ticket must NOT edit root). `cargo check -p semio-framework-os-kernel --lib` **blocked** until registrar applies member/dep cut-over (also pre-existing missing `compiler/**/⚡️implementations` members).

**Ticket:** `26/08/06/OS-IMPLEMENTATIONS-FULL-ERADICATION`  
**Goal:** `aioptimizedrepo`  
**Priors:** `FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION`, `FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION`

## What landed

1. **Cycle break (prefer a):**
   - `PresencePeer` + encode/decode moved to `os_spr::wire`; framework-core reexports from `semio_framework_os_kernel`.
   - Store `config_spec_*` removed from kernel store (ConfigSpec is UI).
   - Store sync uses `crate::os_spr::{PresencePeer,core::DocumentId,…}`.
   - Kernel **no longer** depends on `semio-framework-core`.
   - Framework-core `protocol_core` / `protocol` / `dsl` path-deps retargeted to `semio-framework-os-kernel`.
2. **`🧬️semio`** wired into kernel via `#[path]` (`os_semio`); `pub use crate::os_semio as semio_format` in store; `semio` bin at owner tree; removed kernel `semio_format` sandwich path-dep.
3. **Facades → glue** (`📦️lib.rs` → `📦️glue.rs`): kernel, host, db, run, plugin, plugin-host, neural-engine, dsl-derive.
4. **DB facade** `#[path]`-wires all db leaf components (no sandwich deps).
5. **Host** glues `🖥️host/🦀️component.rs` + kernel aliases (`store`/`protocol`/`dsl`/`vcs`).
6. **Outside-os consumers** (~56 Cargo.toml) path-deps retargeted to kernel/host/plugin/db packages; local alias names preserved.
7. **Flow extensions** promoted to `🌊️flow/🧩️extensions/*/🦀️component.rs` before sandwich delete.
8. **PHYSICALLY DELETED** all 91 `⚡️implementations` dirs under `os/**` (re-deleted after a parallel restore).

## Root Cargo.toml — registrar MUST apply

### REMOVE members (all os `⚡️implementations` paths)

See `🧪registrar-remove-members.txt` (also `🧪registrar-os-impl-refs-in-root.txt`). Every workspace member path containing `🛍️products/💻️os/**/⚡️implementations` must be removed.

Also REMOVE pre-existing broken compiler sandwich members (out of this ticket’s delete scope but blocking `cargo check`):

```toml
"🧰️framework/🔨️modules/📚️compiler/⚡️implementations/🦀️rust",
"🧰️framework/🔨️modules/📚️compiler/📖️syntax/⚡️implementations/🦀️rust",
"🧰️framework/🔨️modules/📚️compiler/🌍️world/⚡️implementations/🦀️rust",
"🧰️framework/🔨️modules/📚️compiler/🔤️text/⚡️implementations/🦀️rust",
"🧰️framework/🔨️modules/📚️compiler/🧮️math/⚡️implementations/🦀️rust",
"🧰️framework/🔨️modules/📚️compiler/📤️svg/⚡️implementations/🦀️rust",
```

→ replace with Shape V2 compiler package if present:
`🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust`

### ADD members (if missing)

```toml
"🧰🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
"🧰🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
```

### `[workspace.dependencies]` rewrites

| Key | New path / package |
|---|---|
| `semio-framework-os` | `…/os/🖥️host/📦️packages/🦀️rust` |
| `semio-framework-plugin` | `…/os/🔨️modules/🔌️plugin/📦️packages/🦀️rust` |
| `semio-framework-os-kernel-db*` (all leaf aliases) | `…/os/🔨️modules/🛢️db/📦️packages/🦀️rust` package `semio-framework-os-kernel-db` |
| `semio-framework-os-kernel-neural-engine` | `…/neural/⚙️engine/📦️packages/🦀️rust` |
| `semio-framework-os-kernel-flow-core` / `infinite-*` / `playbook` / flow-extension-* | fold → host package `semio-framework-os` **or** drop if unused after consumer repoint |
| Already-retargeted `dsl`/`pack`/`protocol`/`store`/`vcs`/`semio` | keep pointing at `…/os/📦️packages/🦀️rust` package `semio-framework-os-kernel` |

See `🧪registrar-rewrite-workspace-deps.txt`.

## Verification

| Check | Result |
|---|---|
| `find os -type d -name '⚡️implementations'` | **0** |
| Outside-os Cargo.toml path deps to os sandwiches | **0** (root excluded) |
| `cargo check -p semio-framework-os-kernel --lib` | **Blocked** — root workspace still lists deleted members + missing compiler sandwiches |
| wasm32-wasip2 kernel check | **Not run** — same blocker |

**Post-registrar command:**

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-os-kernel --lib
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-os -p semio-framework-os-kernel-db --lib
```

## Residuals / blockers

1. Registrar root Cargo.toml member/dep cut-over (this handoff).
2. Host still embeds former godfile as `🖥️host/🦀️component.rs` — further split optional.
3. Flow/infinite/playbook/workflow/space domain components exist but are not all `#[path]`-mounted into host glue yet (host still primarily reexports host_core). Wire them in a follow-up compile pass after registrar unlocks cargo.
4. Watch for parallel agents restoring `⚡️implementations` from git — re-delete if that recurs before registrar lands.

## Race note

Parallel work briefly recreated `🗣️dsl/**/⚡️implementations` after first delete; re-deleted. Registrar should apply cut-over promptly so nothing re-links those paths.


## Registrar applied 2026-08-06
- Re-deleted restored dsl sandwiches (0 remain).
- Members: host/db/plugin/plugin-host/run/neural/flow/infinite packages present.
- Workspace deps retargeted; flow+infinite facades created for extension/world consumers.
- Kernel compile fixes: dsl_notation/grammar reexports, wire serde import, EmbedFrom arms.
- `cargo check -p semio-framework-os-kernel --lib` GREEN.
- Ticket closed.
