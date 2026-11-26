# FEM Mounted App Codegen

The first lean FEM/GIS build `wL2CqpJ` failed before its FEM component link. The compiler warden terminated the upstream `semio_hub_fem` library at 1,871 MiB while its selected library memory ceiling was 1,536 MiB and the swap rate reached 68.59 MiB/s. Cargo reported exit 143 without a semantic diagnostic. The GIS root can still reach its own final crate.

The FEM hub currently closes four concrete mounted applications together. Its artifact declarations require the exact lifted `VcsArtifactApp` editor and viewer payloads for both 2D and 3D. The carve gives each of these four roles a compiler unit and delegates through a thin `semio_hub_fem_router` target. Its explicit `From` implementations pass the exact existing payload types into the two unchanged FEM artifact constructors. Session initialization, editor and viewer roster registration, four activation kinds and write capability stay at the hub entry point.

The red source oracle `verify-fem-app-carve.py` reads the four entries in the checked-in FEM component descriptor and failed because the old router did not give the surfaces independent owners. The new native fixture compares the authored IDs and roles against the real typed artifact declarations. The existing editor/viewer dialect and read-only viewer laws remain the behavior checks.

The pinned Cargo resolver selected 937 units for the offline locked WASI release build. The four role libraries, `semio_hub_fem_router` and `semio_hub_fem_component` are separate units. The offline locked native test graph selected 949 units and includes the router test and the four role libraries. The portable oracle passed against the authored descriptor after the split. The native fixture execution and release compile measurements await an admitted FEM build.

Affected sources

- `🌎️hub/🧩️compositions/🏗️fem/🦀️.rs` and `🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/Cargo.toml`.
- `🌎️hub/🧩️compositions/🏗️fem/🧩️component/🦀️.rs`.
- `🌎️hub/🧩️compositions/🏗️fem/🧪️tests/🔬️surface/🦀️.rs`.
- `🌎️hub/🧩️compositions/🏗️fem/🗃️apps/◻️2d/✏️editor/🦀️.rs` and its `📦️packages/🦀️rust/Cargo.toml`.
- `🌎️hub/🧩️compositions/🏗️fem/🗃️apps/◻️2d/👁️viewer/🦀️.rs` and its `📦️packages/🦀️rust/Cargo.toml`.
- `🌎️hub/🧩️compositions/🏗️fem/🗃️apps/🧊️3d/✏️editor/🦀️.rs` and its `📦️packages/🦀️rust/Cargo.toml`.
- `🌎️hub/🧩️compositions/🏗️fem/🗃️apps/🧊️3d/👁️viewer/🦀️.rs` and its `📦️packages/🦀️rust/Cargo.toml`.
- `🌎️hub/Cargo.toml` and `🌎️hub/Cargo.lock`.
