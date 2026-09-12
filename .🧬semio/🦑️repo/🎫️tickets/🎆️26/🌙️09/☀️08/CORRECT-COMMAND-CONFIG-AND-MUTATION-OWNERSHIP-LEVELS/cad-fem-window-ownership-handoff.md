# CAD and FEM Exact Window Ownership Handoff

## Bounded Result

CAD camera, projection, sun, and Dislocate preferences now live in the exact Shape, Building, Energy, or Structure Classic world-window instance. FEM2D and FEM3D app configs are strict empty records; their Model window records own camera state and their Results window records own camera plus nullable result-source, result-mode, and non-negative mode-index state. The command reducers, direct editor route, retained route, render route, publication contracts, pack/reload paths, schema facets, neutral fixtures, and native law sources all address the concrete window id and kind.

This handoff closes source work only. No CAD or FEM Cargo command completed against these sources. Root owns the queued native compiler/runtime runs.

## Neutral Evidence

- CAD: `🗑️generated/cad-window-typescript.log` records a passing registered `NX_DAEMON=false bun nx run workspace:cad-document-contract`. It includes the independent Ajv and `fast-json-patch` world-window vectors and the existing CAD document oracle.
- FEM: `🗑️generated/fem-window-neutral.log` records a passing direct Bun run of the shared neutral entry. Ajv, the production strict parsers, and `fast-json-patch` agree for FEM2D Model/Results, FEM3D Model/Results, and both strict empty app-config records.
- FEM: `🗑️generated/fem-window-typescript.log` is empty because strict `tsc --noEmit` completed successfully for the neutral entry and its imported production parsers and laws.
- Rust parser/format evidence: `rustfmt --edition 2021 --check` parsed and accepted the 20 bounded FEM app-config, exact-window-config, schema, codec-law, and runtime-law Rust sources after correcting the two inline struct-literal comparisons it found.
- Launch registration: Bun's JSONC parser accepted both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` and found orders `311.191` through `311.194`.
- The registered FEM Nx neutral attempt did not reach its target because the shared Nx project graph was occupied. This is not recorded as a pass; the direct Bun and strict TypeScript commands above are the completed neutral evidence.

## Native Targets and Filters

Run these registered targets sequentially:

```text
NX_DAEMON=false bun nx run workspace:cad-document-contract-native
NX_DAEMON=false bun nx run workspace:fem2d-window-config-contract-native
NX_DAEMON=false bun nx run workspace:fem3d-window-config-contract-native
```

Their exact Cargo commands and lib-test filters are:

```text
cargo test --manifest-path Cargo.toml -p semio-s-artifact-cad-cad --lib cad_document_contract -- --nocapture
cargo test --manifest-path Cargo.toml -p semio-s-artifact-fem-2d --features component-app-assembly --lib fem2d_window_config_ -- --nocapture
cargo test --manifest-path Cargo.toml -p semio-s-artifact-fem-3d --features component-app-assembly --lib fem3d_window_config_ -- --nocapture
```

The FEM filter includes both per-record codec/inverse laws and the end-to-end same-kind instance isolation/reload/rejection law for its dimension.

## Remaining Compile and Runtime Risks

- Cargo has not compiled the FEM slice. The empty `DslArtifact` app records, exact `WindowConfigOwner` associated-type calls, and the new four owner registrations still need compiler proof.
- The runtime laws use fully qualified `ArtifactPack::decode_pack` through `WindowConfigOwner::State`; compiler feedback may require a shorter local type alias without changing the contract.
- The shared app test context constructs apps with the action registry. If runtime dispatch rejects an unbound instance, bind the test instance id before dispatch, matching the dedicated ownership-law setup.
- The camera match arms consume the supplied camera once. If the borrow checker rejects the current branch structure, clone at the two-kind dispatch boundary.
- The native laws' relative schema includes were checked by Rust parsing but not module compilation.
- `setActiveExample` now publishes only `HostOnly`, and camera/result-display publish `WindowConfig`. The native registry must confirm those lane declarations with mutation commands.
- FEM3D's production Results renderer consumes its exact camera record. The existing production visual path still does not use the result-mode fields to choose a richer mounted visual; the test-only results helper does. That is a renderer capability gap, not a persistence ownership fallback.
- `📜️script.ts`, both launch files, and several FEM test-context files are concurrently edited by the shared `artifact_app_laws` rename. Preserve those edits when resolving any compiler diagnostics.

## CAD Exact File Ledger

The authoritative CAD added/updated/removed ledger, with every path written without glob abbreviation, is preserved under `## Exact File Ledger` in `cad-world-window-migration-implementation.md`. It covers 13 added sources/assets, 16 updated sources/tests, and four removed window-config placeholders. The shared launch/script registrations below are additive to that ledger.

## Shared Registration Ledger

- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `📜️script.ts`
- `✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract/🟦️.ts`

## FEM Exact File Ledger

### Updated FEM2D Sources
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`

### Added FEM2D Sources and Assets

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs`

### Removed FEM2D Sources and Placeholders

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/📌️.empty.md`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📌️.empty.md`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️fem2d-config-preparation-laws/🦀️.rs`

### Updated FEM3D Sources and Assets

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json`

### Added FEM3D Sources and Assets

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs`

### Removed FEM3D Placeholders

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/📌️.empty.md`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📌️.empty.md`

## Root Registration and Registered Neutral Verification

The initial handoff had launch commands and the root script branch but omitted the four FEM entries in both project target maps. Root added `fem2d-window-config-contract`, `fem2d-window-config-contract-native`, and the corresponding FEM3D targets to the root `📋️project.json` and ticket `validation/project.json`. The ticket uses its existing VerifyScript fallback; no duplicate FEM implementation was added.

The registered ticket Nx run of both neutral targets completed successfully in 4.7 seconds: independent Ajv/fast-json-patch, production parsers, and strict TypeScript for both dimensions. Evidence: `🗑️generated/fem-window-neutral-registered-1.log`. CAD/FEM native compilation remains pending; the next reserved root slot is FEM2D after Layout r15.

## Root Native Feedback and Empty Owner Removal

FEM2D native1 compiled the new window owners far enough to find two incorrect relative fixture includes; neither test ran. The same four Model/Results include paths in both dimensions are corrected from three parent traversals to two. Evidence: `fem2d-window-config-native-1.log`.

The source audit also found two independently implemented empty app config/mutation protocols and two empty presence/no-op protocols. Both FEM apps now select framework `NoConfig`/`NoConfigMutation` and `NoPresence`/`NoPresenceMutation` directly. Commands and window consumers use those framework types; 29 obsolete local config/presence source and schema/test files plus their mounts were removed. The four actual window owners and their five schema facets remain the domain state authorities. App descriptors return no app-owned schema. Store close/retirement hooks use the existing framework lifecycle factories for document, empty config/draft/presence/transient lanes.

The two native window fixtures now call the shared registered-operation settlement helper, which drains completion receipts as well as pages, events and effects. The initial local loops omitted the terminal completion outbox. The authored 8 MiB fixture-thread setting remains explicit; no normal-stack FEM runtime pass is claimed.

The first post-removal neutral/native route attempt stopped before FEM validation when an unrelated concurrent taxonomy edit made root-script import validation fail. A dedicated ticket facade now invokes the same FEM oracle and Cargo command without importing root policy initialization. Native3 is the next recorded invocation; no outcome is claimed here. The removal/update ledger is `🗑️generated/fem-empty-owner-removal.json`; both artifact root mounts, exact-window unit fixture includes, shared native-law setup, command signatures and ticket validation facade are in this correction scope.

### Root Empty-Owner Correction Ledger

- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🌉️add-beam/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📏️add-member-udl/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📋️add-load-case/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚪️add-node/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗺️add-region/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🛡️add-support/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚖️set-self-weight/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧱️add-material/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧪️tests/🔬️unit/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🔣️.json`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🔗️.graphql`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🟦️.ts`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🛰️.proto`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📏️add-member-udl/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧊️add-solid/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📋️add-load-case/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚪️add-node/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🛡️add-support/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚖️set-self-weight/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🖼️add-frame/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧱️add-material/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`
- updated: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/📌️.empty.md`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/📌️.empty.md`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🔣️.json`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🔗️.graphql`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🦀️.rs`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🟦️.ts`
- removed: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🛰️.proto`
- updated: `✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract/🟦️.ts`

## FEM Receiver Admission Correction

FEM2D native4 compiles in 3m14s and passes both concrete window codec laws; its first setCamera operation reaches the 30s settlement timeout. The labelled runtime log identifies the first blocked operation. Source inspection then finds the fixture binds and dispatches instance 71 but asks the shared fixture helper to drain receiver 1. The SDK correctly filters page delivery against operation.meta.instance_id and requires the bound instance on ACK. Both FEM runtime fixtures now drain receiver 71; no shared runtime admission or timeout budget was weakened. Native5 is pending.

Residual stale empty-config preparation references are removed from the 2D editor and both empty preparation regions. The 3D Results Rust/TypeScript docs now name its concrete window config. Shared framework absence-type strict admission separately passes all 52 native vectors; see framework-empty-state-admission.md.

The strengthened runtime law now consumes the actual host-owned LoadDocument effect through the public document-load API before comparing window preferences. It also compares both Pack and SPR bytes under exact window ID/kind after reopen; merely counting two restored records is insufficient. These strengthened assertions are pending native5.

## FEM Retained Context Correction

Native5 confirms the receiver fix: first-operation failure is delivered and ACKed immediately instead of timing out (runtime 0.03s; two codec laws pass). It reveals the retained reducer was rejected. Both FEM job constructors discarded request.context with context: None even though camera/result reducers require exact window context. Both now transfer Some(request.context) into the retained input owner, retaining the captured window identity/config and its existing bounded lifecycle. Native6 is running next.

## Remaining Camera Type Ownership

Source inspection also finds FemCamera declared in each FEM artifact root: the 2D type is pan/zoom, and the 3D type wraps an opaque JSON string. Their values now live at concrete windows, but these type declarations still mix viewport vocabulary into the domain artifact root. The UI scene math owner has a separate Camera3d projection type; it is not automatically a compatible persisted viewport contract. Follow-up should define or reuse the actual shared UI camera schema and retarget domain windows only after comparing the renderer/interaction contract. Do not assume the math Camera3d can replace the opaque JSON representation without examining its consumers. This type-level scope remains open.

## FEM Current State Projection

Native6 passes both codec laws and actually publishes one exact WindowConfig page for setCamera and one for setResultDisplay. The next assertion exposed another fixture error: it decoded Pack alone as current state even though the event-sourced store persists its initial state in Pack and its applied history in SPR. A feature-gated artifact_app_laws helper now reads the actual concrete window registry's current projection. Both FEM fixtures compare these live values before/after actual document reset and after loading complete Pack+SPR into a new app, while also preserving byte-exact archive comparisons. The production store/event history is unchanged. Native7 is running.

## FEM Fresh Document History Owner

Native7 confirms live camera/result values and both publication pages, then setActiveExample fails on a worker because the app built and dropped an owned ArtifactEnvelope merely to encode empty history. Both FEM reset-effect helpers now use Store's existing empty_document_spr codec beside their snapshot Pack bytes. This removes the unnecessary cloned snapshot/envelope owner and uses the already-defined fresh-document history contract. No destructor or retirement invariant is bypassed. Native8 is running.

## Store Empty History Cursor

Native8 confirms all three FEM commands settle and current window values are correct, then real document reload rejects the shared empty_document_spr output because its HistoryLog default omitted the now-required cursor. The Store codec now emits an explicit empty applied/redo cursor with no checkpoint. The strict parser remains unchanged; the producer is repaired at the shared persistence owner. FEM9 will rerun after the recursive suite.

## Shared Neutral Admission Vectors In Native Laws

All four FEM native window codec laws now execute the existing language-agnostic invalid vectors already validated independently with Ajv and the TypeScript parsers: foreign OS locale at the top level, null camera, extra camera field, and unsupported result mode where applicable. The previous native laws only consumed the valid base and codec round trips. Source inspection suggests nested camera/ResultMode admission gaps; no production validation change is made before a native red run. FEM9 includes these assertions.
