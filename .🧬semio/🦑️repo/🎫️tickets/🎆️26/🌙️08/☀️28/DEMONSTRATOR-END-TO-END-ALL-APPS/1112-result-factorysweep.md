# factorysweep (11/12)

## Step 1: removed NoPresence/NoTransient factories (done)
8 files (puzzle 3d/5d/2d viewer + 3d editor play-app, block 3d/5d/2d viewer, flow viewer) now use `semio_plugin::no_presence_local_root_retirement_factory()` / `no_presence_peer_retirement_factory()`; alias `Puzzle3dPresenceRetirementFactory` deleted.

## Step 2: puzzle-3d reds (coordinator scope)
- mutations: Puzzle3dMutation + 38 leaves got `DslReceiving` (NativeReceive not needed; it would force NativeReceive on Puzzle3dScale), `dsl::original_ordinal_operation_receiving!(Puzzle3dMutation)`; document-model records/scalars got `DslReceiving`.
- editor/window: `mutation_wire!` OpBinary now pack-value (store::pack_rt) + `original_pack_value_operation_receiving!(Puzzle3dWindowConfigMutation)`.
- editor/snapshot play-snapshot: `Arc<..>` retirement fields use `arc_custody::ArcLease`.
- precompute/geometry: cursors (Vector, Map, CollisionMeshRetirement) moved `close_step` to inherent `(body, effect)` + `native_retirement_reply!` + `next_computational_work`.
- precompute/brush suggestions-run: owners bundle holds `ArcLease<SceneConfig>`.
- block 2d/3d/5d snapshot: `extern crate pack;` (lib is named `pack`). Block is OUT of closure per coordinator: block-2d/5d-model not verified (debt).

## Verification
see labels below (W-factorysweep-8: semio-hub-puzzle).
- also: puzzle `terminal_frame_bound!(...)` invocations deleted (macro no longer exists; works already implement `terminal_frame_release_bytes`) in puzzle 2d/3d/5d (18 files). raster and remodel still invoke it (not mine); also `Arc<Puzzle3dSnapshot>` in editor/commands prologue -> ArcLease.
- W-factorysweep-10 (`semio-hub-puzzle`, wasm, hub): rc101; failing crates: semio-os-flow-host-session (lane flowext), stdio-deflate-examples/brep|drawing|mesh|model-snapshot/value-model/txt-schema (lane facade). No puzzle crate fails. Puzzle crates downstream of those (the hub-puzzle top and anything depending on flow-host-session / stdio) are NOT confirmed.
- Debt: block-2d/5d-model (extern crate pack fix applied, still 3 errors each, out of scope); raster/remodel terminal_frame_bound!.
- Rerun: `chk.sh W-factorysweep-11 "🌎️hub/Cargo.toml" wasm -p semio-hub-puzzle` after flowext + facade green.

STATUS: BLOCKED puzzle crates compile clean in W-factorysweep-10; hub-puzzle itself needs flow-host-session (flowext) and stdio-* (facade) green
