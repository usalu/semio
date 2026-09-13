# Sol Flow Browser Ownership Execution Report

## Result

The Flow browser, host and declaration bodies now have semantic, implementation-neutral owners with anonymous implementation leaves. The Flow core command is a thin composition boundary: it invokes the existing wasm-bindgen producer and delegates package projection and publication to the semantic publication owner. The authored runtime owners no longer live under an implementation package directory, and production no longer imports declaration-emitter behavior from a test.

The bounded ownership contract is green directly and through the real cache-skipped Nx target. It proves four production owners, one separate test-only schema oracle, two data owners, three projections, seventeen exact consumer/source-data bindings and thirty-nine exact target inputs. Its runtime checks use Ajv, the TypeScript parser/resolver and Bun's bundler. Private filesystem cases prove staged publication success and pre-promotion failure preservation.

The duplicate `@semio-tech/flow-core` identity was resolved at the producer boundary. The tracked public package under `🫀️core/🕸️bindings` is the only package manifest with that name. The intermediate family manifest under `🌊️flow/📦️packages/🦀️rust/🕸️bindings` is a transient compiler output: successful publication reads it, writes the public manifest, promotes the complete staged public directory, and retires only the transient manifest. The four wasm-bindgen native companions remain in the intermediate directory. The tested pre-promotion missing-companion failure preserves the previous public package and does not retire the transient manifest.

## Exact source and projection move

| Concern | Retired path | Current owner/projection |
| --- | --- | --- |
| Browser runtime | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js` |
| Host runtime | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js` |
| ABI authority | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi/🔣️.json` |
| Declaration implementation formerly in a test | production functions in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts` |
| Authored declaration projection | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts` |
| Browser-types fixture | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types/🔣️.json` |
| Published browser module | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️flow-browser.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟨️.js` |
| Published host module | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️host/🟨️.js` |
| Published declaration | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/📝️flow-browser.d.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟦️.d.ts` |
| Intermediate package identity | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/package.json` | retired after successful staged publication; public identity remains at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/package.json` |

The public export URLs remain `@semio-tech/flow-core/🌐️flow-browser.js` and `@semio-tech/flow-core/🖥️flow-host.js`. The wasm-bindgen `flow_core.js`, `flow_core.d.ts`, `flow_core_bg.wasm` and `flow_core_bg.wasm.d.ts` coordinates remain compiler outputs.

## Ownership contract and implementation owners

Created source/contract leaves:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🧬️schema-oracle/🛂️admission/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🟦️.ts`

Moved or generated data/projection leaves:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️host/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟦️.d.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/package.json`

## Exact consumer and registration attribution

Updated executable/source-data consumers:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📇️layout.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts`

Updated package/project/registry surfaces:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The taxonomy registers `flow-browser-package` against `semio-framework-os-flow-core:wasm` and `semio-framework-os-flow-core:preview-generated`, with the family compiler output root ignored and the public core package tracked. The explicit owned-generator inventory is now sorted with nineteen IDs. Its workspace-publication input closure includes the Flow core project, command router and semantic publication owner. The duplicate JSON `previewTarget` property discovered during closing review was removed.

The package has one `test:browser-ownership` Nx selector and one `preview:browser-package` selector. Seed and derived launch catalogs each have one matching ownership command and one matching preview command. Registry generation completed with 59 plugin crates, 61 playgrounds and 50 framework packages; this report attributes only the Flow registrations and launch selection, not unrelated generated catalog rows.

## First-red progression

1. The schema-first fixture initially named absent anonymous owners and projections. Ajv validation and the existence checks failed before implementation movement.
2. After owner creation, the contract exposed missing context kinds and parent relationships. The taxonomy was extended with path-bound semantic ancestry rather than filename exemptions.
3. Consumer checks exposed the production declaration import from a test, the consumed-browser-clock import from a nonexistent package router and stale named runtime/ABI/projection paths. Those bindings were moved to the semantic owners.
4. Exact target-input comparison exposed absent external source/data inputs. `flowBrowserProduction`, `flowBrowserDeclarationOracle` and `flowBrowserRuntimeOracle` now name their actual authorities and consumers.
5. The first preview attempt used the package root as `cwd`, while the command router resolves its files from its own directory. The preview target now uses the exact semantic owner project root and an empty output declaration because it is read-only.
6. The first sibling-package review found two manifests declaring `@semio-tech/flow-core`. Consumer and lockfile evidence showed the family manifest is compiler staging and the tracked core package is the published identity. Staged-success and pre-promotion-failure controls were added before retiring the transient manifest.
7. The selected native Rust law first stopped with `E0277`, because `RetainedConfigStoreHydration<P>` called an API requiring `P: ArtifactPack`. The same obligation already exists on the analogous persisted document hydration. The narrow store bound was added; compilation advanced to the separate missing WindowConfig retained module.

## Verification evidence

### Direct ownership and projection

- `SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/sol-flow-browser-ownership/artifacts bun ./📜️script.ts test-browser-ownership`
  - exit 0, 0.640075 seconds.
  - `[DEBUG] Flow browser ownership: 4 owners, 17 consumers, 39 exact Nx inputs, staged publication, pre-promotion preservation and native TypeScript/Bun projection parity PASS`
  - The private success case observed a complete single public package and absence of the transient family manifest.
  - The private pre-promotion missing-companion case observed no successful retirement and byte-for-byte preservation of the preceding public package sentinel.
- `bun ./📜️script.ts declarations`
  - exit 0, 0.948 seconds.
  - 105 methods, runtime plus TypeScript-parser projections, three package exports, two TypeScript resolutions and five hostile declaration vectors.
- `bun ./📜️script.ts preview-generated`
  - exit 0.
  - Seven files, 6,229 browser bytes and 23,497 declaration bytes; read-only preview.
- The explicit publication of the already prepared family directory exited 0 without rebuilding wasm. It produced the public core package, removed only the transient family `package.json`, and left exactly one current `@semio-tech/flow-core` package identity.

### Registered ownership target

The final registered run used `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, ticket-private `NX_WORKSPACE_DATA_DIRECTORY`, `NX_CACHE_DIRECTORY`, `TMPDIR` and `SEMIO_TEST_ARTIFACT_DIR`:

`bun nx run semio-framework-os-flow-core:test-browser-ownership --skip-nx-cache`

It exited 0 in 19.36 seconds wall time. Nx reported one task, 615 ms target time, 585 ms critical path and cache skipped. This is the final evidence after the staged package-publication controls and transient manifest retirement.

### Runtime routes

- `bun ./📜️script.ts test-browser`: exit 0 in 3.648 seconds; actual runtime/session/host/draw/package checks executed.
- `bun ./📜️script.ts test-browser-clock`: exit 0 in 1.136 seconds; 7,733 real `performance.now()` samples and terminal-empty completion.
- `loadTaxonomy()`: `[DEBUG] workspace taxonomy loaded: 625 kinds, 545 fixed contracts, 23 generators` before the final concurrent catalog additions; no validation problem.
- Current Flow placeholder scan found no placeholder implementation tokens.

### Native Rust reachability boundary

The exact selected command was:

`CARGO_TARGET_DIR=<ticket>/🗑️generated/sol-flow-browser-ownership/cargo-target CARGO_BUILD_JOBS=2 cargo test --locked --offline -p semio-framework-os-flow --lib production_reachability_fixture_and_hostile_source_census_reject_the_old_route -- --exact --nocapture`

The first compilation attempt stopped before test execution with `error[E0277]: the trait bound P: ArtifactPack is not satisfied` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs`. The bounded prerequisite repair imports `ArtifactPack` and adds that existing API requirement to both `RetainedConfigStoreHydration` impls. This is the only kernel source changed by this lane.

The second attempt advanced beyond that type error and stopped before test execution because `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` declares `#[path = "📥️retained/🦀️.rs"] mod retained;`, but the file does not exist. That module is also expected to define six public `WindowConfigPackLoad*` reexports. No definitions exist elsewhere in the current Rust tree. Reconstructing that independent retained-pack behavior was outside the Flow ownership boundary, so the selected Flow law has no native assertion count and is not claimed as passing.

The broader Flow retirement source route resolves the moved schema-oracle import, then fails on a separate Infinite board-source assertion requiring the literal `assert!(turns > 1_600)`, which is absent from the current source. It is not Flow extraction acceptance evidence.

## Limits

- No wasm-pack rebuild, app build, application server, install, JCO generation or hashed distribution publication was run.
- This scope models the actual wasm-bindgen Flow boundary. A current Flow-core/Wasm source search found no JCO manifest or generator consumer, so no JCO relationship was inferred.
- The public package success used the existing native compiler companions. The private controls used injected filesystem fixtures and did not mutate live publication outputs. They prove staged directory promotion and pre-promotion failure preservation; they do not prove a multi-path transaction if family-manifest removal or rollback fails after promotion.
- The selected Rust reachability law did not execute because the unrelated WindowConfig retained source is absent. The direct and registered schema/ownership, Bun bundle, TypeScript resolution and Flow browser runtime evidence remain green and independent of that native limitation.
- The store bound repair is a narrow prerequisite path in this ledger: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs`.

## Ticket artifacts

The durable plan is `📓️sol-flow-browser-owner-plan-2026-09-13.md`. This report is the durable execution record. Ticket-private Nx databases, caches, temporary publication trees and the private Cargo target under `🗑️generated/sol-flow-browser-ownership` are disposable and are removed after acceptance; no live package output is part of scratch cleanup.
