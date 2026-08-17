# Wave 2 — 🧊3d Brep Engine Migration

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Owner:** Wave 2 s-kernels (🧊3d)

## Delivered

### Content-addressed geometry handles
- `GeometryHandle::content_addressed` in `📐️brep/⚙️engine/🦀️component.rs` (blake3 of session + ordinal + entity material).
- `BrepkitKernel` drops `seq: u32`; registry keys are no longer `solid-1`, `solid-2`, …
- Test: `geometry_handles_are_content_addressed_not_sequential` in kernel `#[cfg(test)]`.

### `BrepEngineHost` (OS engine integration)
- New `📐️brep/⚙️engine/🖥️host/🦀️component.rs`:
  - `BREP_ENGINE_ID = "s.3d.brep"`
  - `BrepDocumentOpEngine` registered on `EngineCache` (document op dispatch stub for Wave 2b).
  - `BrepEngineHost`: `EngineHost` + host-owned `EngineCache` + compute-scoped `BrepkitKernel` mutex.
- `semio-s-3d` `brep` feature now pulls `semio-framework-os-kernel` + `blake3`.

### Plugin globals → host
- **CAD:** `CAD_BREP_KERNEL` (`OnceLock<Mutex<Box<dyn BrepKernel>>>`) → `CAD_BREP_HOST` (`OnceLock<BrepEngineHost>`).
  - `cad_brep_host()` + `cad_brep_kernel()` → `MutexGuard<BrepkitKernel>` on host session.
- **Process3d:** `PROCESS_BREP_KERNEL` → `PROCESS_BREP_HOST`; `ProcessKernelSession` wraps `BrepEngineHost` + replay memo.

### Authority documentation
- Module docs on `Body`, `LabelSource`, `HalfedgeMesh`: value types / engine-compute scope only.

## Inventory ticks (`🧪inventory-core.md`)

| Item | Status |
|------|--------|
| `BrepkitKernel` seq/registry | **Partial** — seq removed; registry remains inside host-owned kernel session (not plugin-owned). Full document-op derive pending. |
| `Store` arena | **Partial** — engine-internal when `Body` is compute-scoped (documented). |
| `Body` | **Partial** — documented compute/cache scope. |
| `LabelSource` | **Partial** — documented inside `Body` only. |
| `HalfedgeMesh` | **Partial** — documented as value payload, not global store. |
| `CAD_BREP_KERNEL` | **Done** — replaced by `cad_brep_host()` / `BrepEngineHost`. |
| `PROCESS_BREP_KERNEL` | **Done** — replaced by `PROCESS_BREP_HOST` + `BrepEngineHost`. |

## Verification

| Gate | Result |
|------|--------|
| `cargo check -p semio-s-3d` | **pass** |
| `cargo test -p semio-s-3d --lib` | **blocked** on host — Xcode license (`cc` exit 69) when linking `semio-framework-os-kernel` dylib |
| `cargo check -p semio-s-plugin-cad` | **blocked** — pre-existing `semio-framework-os` missing `plugin_bundle_installer_shim.rs` (not 3d wave) |

## Follow-ups (integration-requests)

- Wire `BrepDocumentOpEngine::compute` to opcode pack (incremental `derive(parent_handle, step)`).
- M3: inject `&dyn EngineHost` into `DocumentApp::handle` — delete `OnceLock` cad/process host singletons.
- `ArtifactKind::Engine` + WIT `engine-derive` / `engine-read` (Wave 1b).
