# Process Mounted App Codegen

The Demonstrator and Process pair `wUc5aA5` reached 1,138 compiled units before its only remaining Cargo child, `semio_hub_process`, requested a larger fleet worker. The runtime reported no active local rustc or linker. The previous memory record was 4,102 MiB. The guarded queue retired the run after confirming that the old Process artifact had not been replaced and there was no idle fleet slot. The selected Demonstrator production graph did not depend on the Process Hub library.

The Process 3D editor and viewer have separate app dispatch crates under the Process composition. Each owns the existing `OriginalVcsApp` payload for its own app and `SemioMembers`. The renamed `semio_hub_process_router` target delegates through the two app enums. Its typed conversions still satisfy the native builder registrations for exactly the existing editor and viewer. Artifact declaration, app IDs, example sources, native factories, mutation rosters, capabilities and mounted-owner policy keep the same production path. The Demonstrator's Process surfaces use lifted payload types, so those crates cannot substitute for the standalone Process original-retained owners.

The shared registration fixture and its Rust/Serde checks cover the manifest, both original factories, rejection of an unknown app and retirement of both retained definitions. The ticket witness `verify-process-carve.py` failed before the editor and viewer crates existed, then passed against the same fixture. Pinned Cargo `--offline --locked --unit-graph` resolved 1,013 units for the Process WASI release, including an independently selected editor, viewer, router and component. Each app unit selects only its own artifact role. The component selects the router. Pinned rustc parsed both app sources and the router. A separate locked native test graph resolved 1,102 units for Process registration and Demonstrator standalone parity. The parity test's Process bound checks the original retained wrapper with the exact app's associated member roster.

The earlier pair's exact dependency output lane was detached during the subsequent compiler-owner recovery. Standalone code generation and native fixture execution need either a coherent restored lane or the official next Process build. The locked graph by itself does not establish compiler RSS or behavioral results.

Affected repository paths:

- `🌎️hub/Cargo.toml`
- `🌎️hub/Cargo.lock`
- `🌎️hub/🧩️compositions/🏭️process/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🏭️process/🧩️component/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/🗃️apps/✏️process-editor/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/🗃️apps/✏️process-editor/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🏭️process/🗃️apps/👁️process-viewer/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/🗃️apps/👁️process-viewer/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️standalone-parity/🦀️.rs`

Retained ticket inputs are `verify-process-carve.py` and `verify-process-compiler-unit.py`.
