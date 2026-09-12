# App Verification Script Extraction

Date: 2026-09-12 through 2026-09-13

## Result

The Mathematical, GIS, VCS and Energy verification bodies no longer live in package command files. Four package `📜️script.ts` files are now small routers of 12, 35, 27 and 15 lines. Executable verification behavior lives under explicit `🧪️tests` or `🔮️oracles` concerns and every implementation leaf is anonymous. The Energy Python source and its toolchain pin table also moved out of the Python package into semantic `🏃️execution` and `🛠️toolchain` owners; no compatibility module or data copy remains.

The portable ownership contract records 13 implementation/data leaves, four routers, twelve contextual kinds, eleven direct TypeScript consumer rows and two source-as-data consumer rows. It verifies strict schema admission, anonymous basenames, exported owner APIs, thin-script classification, exact direct-import closure, the Energy registry/module selector, the Energy pin-table reader, package/Nx registration, and both launch projections.

## First Reds And Audit Gates

- Mathematical failed before extraction because its strict schema required at least seven routes while the real authority has six: `MathematicalPublicationAuthority/routes/minItems`, limit 7. The six actual routes and `Artifact`/`WindowConfig` Rust lane authority now define the schema and fixture.
- GIS map verification failed before extraction because strict Ajv rejected the production keyword `x-semio-child-kind`. The shared GIS verification compiler now registers that keyword with a string meta-schema, together with the existing GIS annotations, and still fails closed for unknown annotations.
- The first aggregate test import for the Energy toolchain climbed one directory too far and stopped with zero tests. Rebasing it to the exact repo root made the test executable.
- Independent source review found the first Mathematical and three GIS executable proofs under `🧫️fixtures`. They moved to `🧪️tests`; JSON fixtures stayed under `🧫️fixtures`.
- Independent source review found stale Hub imports and source-text assertions, missing GIS/VCS external Nx inputs, and an Energy pin table still owned by the Python package. All were rebased before final verification.
- The first registered aggregate invocation waited more than six minutes in the shared Nx daemon without reaching the target. Only that idle process chain was interrupted and its orphaned Nx child was removed. The same target passed with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, ticket-private Nx/cache/temp/artifact paths and `--skip-nx-cache`.

## Owned Files

### Mathematical

- Updated `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/📜️script.ts` to a direct test-owner router.
- Updated `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/📋️project.json` with the external test, fixture, schema and production-authority inputs.
- Updated `✏️s/🔌️plugins/➗️mathematical/🧬️schema/🔣️.json` to the exact six-route `Artifact`/`WindowConfig` authority.
- Updated `✏️s/🔌️plugins/➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json` to the same production lane literals.
- Added `✏️s/🔌️plugins/➗️mathematical/🧪️tests/📣️publication-authority/🟦️.ts` as the executable authority owner.

### GIS

- Updated `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts` to route directly to test owners and the accepted fresh-component publication owner.
- Updated `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📋️project.json` to hash external `.ts`, `.json` and Rust sources used by registered routes.
- Added `✏️s/🔌️plugins/🌍️gis/🧬️schema/🟦️.ts` as the shared strict verification compiler without exporting Ajv types.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts`.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/💡️inference-control/🟦️.ts`.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/🧩️map-create-region-group/🟦️.ts`.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/🧩️map-create-region-group/🧬️schema/🔣️.json` for the portable control projection while the proof also compiles the production GIS schema/dependency closure.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/🗄️durable-three-store-assembly/🟦️.ts`.
- Added `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts`, including cancellation, deadline, staging and exact native-law orchestration.

### VCS

- Updated `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts` to a direct test-owner router.
- Updated `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📋️project.json` to hash external test/schema/fixture inputs.
- Added `✏️s/🔌️plugins/🌿️vcs/🧪️tests/📇️native-codecs/🟦️.ts` for strict schema, receipt/protocol hashing, nine hostile controls and two exact Cargo laws.
- Added `✏️s/🔌️plugins/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts` for strict identity admission, eleven hostile controls, Node/WebCrypto parity and one exact Cargo law.

### Energy

- Updated `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/📜️script.ts` to a direct oracle-owner router.
- Updated `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/📋️project.json` with semantic toolchain, execution and contribution-registry inputs.
- Updated `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🔣️.json` to select the semantic Python source directory and anonymous `🐍️` module.
- Added `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts` for pin validation, platform selection, streaming SHA-256, cache/provisioning and environment assembly.
- Moved the pin table from `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/🔣️.json` to `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json`. Root independently confirmed the comment-excluded semantic projection remained byte-equivalent at SHA-256 `ece3337552781b9d3277b9fbe2d12f31070df7c4b15e7402782672c99f886a81`: one OpenStudio tool, six platform pins and Python 3.12.
- Added `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts` for locked `uv run --no-sync`, registry/path/module selection and the eight command classes.
- Moved the Python implementation from `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/🔮️oracles/🐍️.py` to `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py`; the uv project remains the dependency environment and `PYTHONPATH` selects the semantic owner.
- Updated `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json` to name the new Python and pin-table owners.

### Consumer And Portable Ownership Closure

- Updated `🌎️hub/📦️packages/🦀️rust/📜️script.ts` so static and dynamic GIS/VCS consumers import the real test owners; its component source contract now asserts that exact import.
- Updated the contextual-kind entries in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` for app ownership, verification, toolchain and execution concerns.
- Added `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📱️app-verification-source-ownership/🔣️.json`.
- Added `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📱️app-verification-source-ownership/🔣️.json`.
- Added `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📱️app-verification-source-ownership/🟦️.ts`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` with the portable gate and exact cached inputs.
- Updated `.vscode/🧩️launch.seed.jsonc` and derived `.vscode/launch.json` with `🧹clean🧩️taxonomy🧪️app-verification-source-ownership` in existing order.

Shared files above contain concurrent work by other lanes. Ownership here is limited to the described imports, assertions, contextual entries, target/package route and launch entries.

## Verification

- Direct aggregate after the final source-data/document closure: `bun ./📜️script.ts test app-verification-source-ownership` from the repo-lib TypeScript package, exit 0, 6 passed, 0 failed, 188 assertions in 8.27 seconds. The earlier accepted checkpoint was 6/186; the final two assertions cover the Python `@see` and artifact-registration pin-table paths.
- Registered aggregate with isolated Nx state: `bun nx run @semio-tech/repo-lib:test-app-verification-source-ownership --skip-nx-cache --outputStyle=static`, exit 0, 6 passed, 0 failed, 188 assertions, target 5.2 seconds, cache skipped.
- Mathematical registered target: `bun nx run @semio-tech/mathematical-js:test --skip-nx-cache`, exit 0. It reported six routes, one `WindowConfig` lane, strict Ajv, three hostile mutations, and both existing example tests passed.
- GIS registered source targets, serial `run-many`: component cold-map, durable three-store and map-create-region-group all passed. Reported evidence was component Ajv + Node/WebCrypto, five hostile cases and nine markers; durable roles 3, cancellation 4 and rejection 3; map 26 checks clean. A direct bounded invocation also passed all five exported GIS source proofs, including native-codec and controlled-proposal controls.
- VCS registered oracle-only targets, serial `run-many -- --oracle-only`: exit 0. Codec reported one receipt, nine hostile denials, Ajv + Node/WebCrypto parity and three dependency-coherence checks; identity reported one positive and eleven hostile denials with WebCrypto parity.
- VCS registered native targets without `--oracle-only`: exit 0 after 11 minutes 54 seconds under shared Cargo contention. Target `native_codecs` passed `vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution` and `vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind`, with executable receipt SHA-256 `018d1a4de5e4ddb118704c98f52767320dd8de33f4079c79fe5ad43970aaeb34`. Target `native_openable_identity` passed `vcs_guest_descriptor_has_one_canonical_native_openable_identity`, with executable receipt SHA-256 `c79ec3e74fd8ee295bb415fbc0f3644b97f05c5a6369e05dddc7330a61f55540`.
- Energy registered prepared selftest: `bun nx run @semio-tech/energy-oracle-py:test --excludeTaskDependencies --skip-nx-cache`, `UV_OFFLINE=1`, bytecode disabled and all temporary/cache/artifact paths ticket-owned. Exit 0, 11 checks, 0 failures, target 2.9 seconds. Ten native Honeybee cases and one translated semio case each reported the expected 129.60 m³ volume and 12.00 m² glazing; the intended HVAC/free-float distinction held. No dependency sync or download occurred.
- Independent Terra checkpoints passed the portable ownership gate before the final documentary additions; root independently checked the Energy semantic pin projection after its move.

## Limits

- GIS has no supported `--oracle-only` segment on `native-codec-check`. An independent invocation that supplied that unsupported flag printed the source oracles and then correctly entered Cargo. The current workspace Rust build exited 101 in `semio-framework-os-store` because close-byte demand fields were inconsistent. That whole invocation is a native red and is not evidence of a flag-forwarding or extraction defect. This lane did not claim green GIS native laws, fresh component production, browser acceptance, WAL recovery or Hub publication.
- VCS native evidence proves the exact two codec laws and one openable-identity law. It does not prove Hub catalog activation, provider linking or a mounted client.
- The Energy selftest exercises Honeybee/OpenStudio translation and native case construction but does not execute an EnergyPlus physics simulation. The prepared toolchain was not downloaded, extracted or freshly archive-hashed in this lane.
- Mathematical verification is source/schema/publication-authority evidence; it does not compile a separate native Mathematical binary.
- Existing generator dependencies executed by GIS/VCS Nx routes refreshed their ordinary generated products. This lane did not edit or claim ownership of those generated products.

## Scratch

All disposable logs, Nx workspace/cache state, Cargo-law artifacts, uv cache, temporary directories and Python cache for this lane were confined to `🗑️generated/sol-app-verification-extraction`. The report retains the first-red diagnostics and final outcomes above; that disposable subtree is removed after final handoff.
