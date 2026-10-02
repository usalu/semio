# Public Contract And Export Current Audit

Read-only audit of the current shared checkout, 2026-10-01. No tests, Nx, Cargo, generated launch writer, modifying Git commands, or source changes were executed. The only authored artifact is this report; no generated files were created.

## Findings

- **P1, remaining product-owned public contract:** `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:4` explicitly reexports `neural_engine::ValueType`. Its declared dependency at `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml:25` resolves into `🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine`. This is a repository-owned type, not a third-party type, but still makes neutral graph public contracts dependent on deleting an OS product. Root is already auditing Cargo direction; retain this as corroborating public-API evidence.
- **P2, forwarding alias:** `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:6` reexports its own `Manifest` as `GraphManifest`. Active consumers include `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs:102,123,227,234,291,318`. This is an active alternate public spelling rather than dead code. If the no-forwarding-alias requirement covers Rust synonyms, consolidate those consumers onto `Manifest`; no compatibility obligation exists. It does not leak third-party types.

## Clean Newly Neutral TypeScript Contracts

Exact Cargo (`🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts:8-56`), native artifacts (`🏃️process/📦️artifacts/🏗️native-build/🟦️.ts:94-162`), Wasm (`🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts:7-22`), and testing ports own their structural declarations. Node platform and AbortSignal usage is framework/system usage; it is not third-party leakage. No product implementation import or stale TypeScript forwarding export was found in the audited exact/native/Wasm/testing directories.

SVG export (`🖌️raster/🎥️video/🖋️svg-export/🟦️.ts:9-35`) owns request, page, browser, and control interfaces. Browser implementation (`🌐️browser/🟦️.ts:3-13`) dynamically imports Playwright behind an owned structural shape; its exported return type is owned `ChromiumSvgVideoRuntimeV1`. Playwright Page/Browser classes are not exposed. This is an actual optional tool runtime adapter, not a test oracle, but is correctly behind owned interfaces.

Graph output catalog (`🕸️graph/🛂️manifest/📇️catalog/🟦️.ts:6-13`) receives input areas, exclusions, generic emoji identity policy, and outputs explicitly. Admission receives caller-provided roots and input paths (`📥️admission/🟦️.ts:43,74`). Projection's generated `export *` index (`📽️projection/🟦️.ts:207-208`) is an intentional catalog-owned artifact export surface, not a stale forwarding alias to a deleted module. Identity path declarations (`🪪️identity/🛣️path/🟦️.ts:5-33`) are owned structural values; no product import or external API type is exposed.

Ajv/TypeScript imports in the audited contract tests are independent validation/parser oracles and do not leak into runtime public declarations. No tests were run by this audit.

## Authored Launch Seed Verification

Parsed `.vscode/🧩️launch.seed.jsonc` with TypeScript `parseConfigFileTextToJson`: valid JSONC. Read literal project names and target maps from repository `*project.json` files using JSON parsing, without invoking Nx. All 17 authored configurations for literal `@semio-tech/framework-process`, `@semio-tech/framework-identity`, and `@semio-tech/framework-graph` matched existing target maps; every referenced input ID exists.

Fresh process targets `test-exact-cargo-laws`, `test-native-artifacts`, and `test-wasm-build` are present in `🏃️process/📋️project.json`; script dispatch sources exist at `🏃️process/📜️script.ts:26-28`. Their authored configurations at seed lines 7232-7271 set `SEMIO_TEST_ARTIFACT_DIR` from registered `processContractArtifacts`. Graph's authored environment deliberately uses `SEMIO_TEST_ARTIFACTS_DIR` (plural), matching its suite at `🕸️graph/🧪️tests/🧩️suite/🟦️.ts:12-13`. Identity uses its registered `identityContractArtifacts` input. Actual launches and runtime outcomes remain unverified, as requested.
