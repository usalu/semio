# cadmodel (11/12)
## Root causes (cad-model, 61 errors)
1. `Arc<MeshData>`/`Arc<SemioBrepSnapshot>`: RetireOwned (CadGeometry owned_meshes/owned_breps) -> fields now `ArcLease<..>`; `MeshData` derives RetireOwned (framework mesh-engine); `ArcLease` derives PartialEq. Consumers (scene-compute, geometry-import, editor test) use `.0.as_deref()` / `ArcLease::new`.
2. references `close`: cold `ControlledRetirement::step(grant)` gone -> `drop(value)` (same as step-ifc port).
3. sqlite snapshot `restore`: `allocation_stage` third arg is now `&mut dyn NativeSendAllocationLoan`, passed straight to `NativeDecodeControl::new_forwarded`.
## Verify
W-cadmodel-3 (hub wasm, -p semio-framework-mesh-engine -p semio-s-artifact-cad-cad-model): rc 0. MeshData already RetireOwned via artifact_retire_struct!; my duplicate derive removed.
- W-cadmodel-4 (`semio-s-plugin-cad-aec-building-component`, wasm, ext manifest): rc 0.
- cad editor presence: `CadPresenceRetirementFactory` (old grant/Arc API) removed; editor scope uses `semio_plugin::bounded_presence_root_retirement_factory::<Self::Presence>()`; stale fixture factory unit test removed (debt: no replacement retirement-bytes test).
- W-cadmodel-6 (`semio-hub-cad-component`, hub wasm): rc 101, zero cad errors; remaining reds are stdio png-snapshot (1.2) and gltf-snapshot-root (sqlite native/binary pack ports): lanes stdio4/stdio6.
- Not verified: aec-building-energy/-structure extensions, cfg(test) targets, cad-model `🚦️owner` sqlite file (not compiled).
- Rerun: `chk.sh W-cadmodel-7 "🌎️hub/Cargo.toml" wasm -p semio-hub-cad-component` after png/gltf green.
STATUS: FIXED-verified W-cadmodel-3 (cad-model + mesh-engine); hub-cad-component BLOCKED on stdio png/gltf
