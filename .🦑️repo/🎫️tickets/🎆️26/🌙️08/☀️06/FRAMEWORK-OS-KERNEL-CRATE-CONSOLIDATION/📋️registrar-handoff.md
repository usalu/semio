# 📋️ Registrar Handoff — Framework Os Kernel Crate Consolidation (W8c)

**Status:** `flagged` — Shape V2 tree + `protocol→spr` rename **landed on disk**; **wasm admission NOT green yet**; **old crates NOT deleted** (copy-then-verify gate).

**Ticket:** `26/08/06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION`  
**Exclusive tree:** `🧰️framework/🛍️products/💻️os/**`  
**Goal:** `🎯aioptimizedrepo`

## What landed

### New packages (Shape V2)

| Crate | Path | Notes |
|---|---|---|
| `semio-framework-os-kernel` | `💻️os/📦️packages/🦀️rust/` | Owner root = `💻️os/`. Lib name `semio_framework_os_kernel`. |
| `semio-framework-os-kernel-dsl-derive` | `💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/` | Proc-macro **must stay separate**. Component at `✨️derive/🦀️component.rs`. |
| `semio-framework-plugin` (Shape V2 relocate) | `💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/` | Guest SDK de-sandwiched; `🦀️component.rs` at plugin owner. **Deps still point at OLD kernel crates** until cut-over. |

### Domain tree

- `📡️protocol/**` → copied to `📡️spr/**` (`🦀️component.rs` leaves) — **old `protocol` impl dirs still present**
- `🏪️store`, `🗣️dsl`, `🎒️pack`, `♾️infinite`, `🌊️flow`, `🌿️vcs` → `🦀️component.rs` copies beside (or replacing sandwich leaves)
- ~55 `🦀️component.rs` files under `🔨️modules/`
- Kernel `📦️lib.rs` currently wires **slim surface**: `os_dsl`, `os_spr`, `os_pack`, `os_store`, `os_vcs` (infinite/flow component files on disk but **unwired**)

### protocol→spr rename (document every rewrite)

Full table: `🧪rewrite-table.md` (52 survivor rows).

Public module intent after cut-over:

| old lib / dep alias | new path |
|---|---|
| `protocol` | `semio_framework_os_kernel::os_spr` (crate aliases also expose `protocol` / `spr` via `extern crate self`) |
| `protocol_core` | `…::os_spr::core` |
| `protocol_command` | `…::os_spr::command` |
| `protocol_causal` | `…::os_spr::causal` |
| `protocol_crdt` | `…::os_spr::crdt` |
| `protocol_format` | `…::os_spr::format` |
| `protocol_history` | `…::os_spr::history` |
| `protocol_materialize` | `…::os_spr::materialize` |
| `protocol_wire` | `…::os_spr::wire` |
| `protocol_channel` | `…::os_spr::channel` |
| `protocol_io` | `…::os_spr::io` (`cfg(not(wasm))`) |
| `protocol_cli` / bin `protocol` | `…::os_spr::cli` / bin **`spr`** |
| `protocol_testkit` | `…::os_spr::testkit` |
| `store` / `store_sync` / `store_worker` | `…::os_store` (+ sync/worker when re-wired) |
| `dsl*` family | `…::os_dsl::*` |
| `pack*` family | `…::os_pack::*` |
| `vcs` | `…::os_vcs` (folded into kernel to break vcs↔protocol type identity) |

**~36+ manifests** still name `semio-framework-os-kernel-protocol*` (count in `🧪protocol-manifest-count.txt`). Registrar must rewrite those to `semio-framework-os-kernel` after admission+delete.

## Root `Cargo.toml` actions (registrar only — NOT applied)

### Add members

```toml
"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
```

### Remove members (ONLY after wasm admission green + old delete)

All current members under:

- `…/📡️protocol/**/⚡️implementations/🦀️rust`
- `…/🏪️store/**/⚡️implementations/🦀️rust`
- `…/🗣️dsl/**/⚡️implementations/🦀️rust` (except leave nothing if fully merged; **keep derive packages path above**)
- `…/🎒️pack/**/⚡️implementations/🦀️rust`
- eventually infinite/flow impl members when those slices wire+admit
- `…/🔌️plugin/⚡️implementations/🦀️rust` (replaced by packages path)
- `…/🌿️vcs/⚡️implementations/🦀️rust` when vcs fold is finalized repo-wide

### `[workspace.dependencies]`

- Add `semio-framework-os-kernel = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }`
- Rename/remove `semio-framework-os-kernel-protocol*` aliases → kernel
- Rename profile overrides `semio-framework-os-kernel-store` → `semio-framework-os-kernel` (rust-lld CGU=1 workaround — **re-test**)

### CLI bins

- `protocol` bin → `spr`
- `pack` bin stays `pack` (from kernel package)

## Wasm admission

```
DEVELOPER_DIR=/Library/Developer/CommandLineTools
bash .🦑️repo/🎫️tickets/🎆️26/� comb08/☀️06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION/🔧️cargo.sh \
  check -p semio-framework-os-kernel --lib
# then:
# cargo check -p semio-framework-os-kernel --target wasm32-wasip2 --lib
```

**Result (2026-08-06):** native `--lib` still **red** (~39 errors in `🧪check-12.txt`).  
**Root cause:** dual-tree type identity — `semio-framework-core` still re-exports types from **old** `protocol_core`, while kernel defines parallel types in `os_spr::core` (`UndoPolicy`, `OperationId`, …). Also residual `note`/`writer` refs in dsl fixture paths and a few path-rewrite fixes.

**Therefore: do NOT delete old crates yet.**

## Residuals / follow-ups (handoff sections)

### A — Finish kernel compile + wasm32-wasip2 (this ticket or immediate reopen)

1. Qualify/eliminate collisions with `semio-framework-core`'s old protocol reexports (or coordinate a tiny core retarget — **outside os/**, needs core owner).
2. Re-wire `store::sync` / `store::worker` modules.
3. Wire `infinite` + `flow` modules (component files already copied); keep flow-**extensions** as separate packages; kernel must **not** depend on extensions (extensions depend on kernel after cut-over).
4. Feature-gate GPU (`infinite::canvas` / `world`) — `gpu` feature pattern already drafted.
5. Pass `cargo check --target wasm32-wasip2 -p semio-framework-os-kernel --lib`.
6. Then delete old impl sandwiches for the admitted set.

### B — Host plane (follow-up ticket)

Owner: `💻️os/🖥️host/` → `semio-framework-os`  
Absorb: `🛢️db/**`, `🔁️workflow`, `🧠️neural`, `📺️renderer`, `🏃️run`, native-only, existing root `semio-framework-os` aggregator at `💻️os/⚡️implementations/`.  
Update `plugin-host` accordingly.

### C — Plugin cut-over

Point `semio-framework-plugin` packages Cargo.toml deps at `semio-framework-os-kernel` (replace store/protocol/dsl/vcs paths). Delete `🔌️plugin/⚡️implementations`.

## Verification helper

`🔧️cargo.sh` — temporarily inserts kernel as root workspace member, runs cargo, restores root via trap. Safe for agents; registrar still owns permanent member lines.

## Handoff JSON

```json
{
  "owner": "💻️os",
  "ticketPath": "26/08/06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION",
  "newCrates": [
    "semio-framework-os-kernel",
    "semio-framework-os-kernel-dsl-derive (Shape V2 packages path)",
    "semio-framework-plugin (Shape V2 packages path)"
  ],
  "oldMemberLines": [],
  "workspaceDepRenames": [
    "semio-framework-os-kernel-protocol* → semio-framework-os-kernel",
    "semio-framework-os-kernel-store* → semio-framework-os-kernel",
    "semio-framework-os-kernel-dsl* → semio-framework-os-kernel (derive stays)",
    "semio-framework-os-kernel-pack* → semio-framework-os-kernel",
    "profile.*.package.semio-framework-os-kernel-store → semio-framework-os-kernel"
  ],
  "crossDepsFlagged": [
    "semio-framework-core still types against old protocol_core — blocks admission while dual-tree exists",
    "math/ui/plugins still path-dep old dsl/store/protocol manifests (~36+ protocol manifests)",
    "flow extensions must flip to depend on kernel after cut-over (no kernel→extension edges)"
  ],
  "residualsDeferred": [
    "infinite+flow module wiring + gpu feature",
    "store sync/worker re-wire",
    "old crate deletion (blocked on wasm admission)",
    "host plane semio-framework-os at 🖥️host/",
    "plugin dep cut-over + delete implementations sandwich"
  ],
  "tests": { "baseline": null, "now": null },
  "wireProof": "n/a",
  "wasmAdmission": "NOT PASSED — see 🧪check-12.txt",
  "status": "flagged"
}
```
