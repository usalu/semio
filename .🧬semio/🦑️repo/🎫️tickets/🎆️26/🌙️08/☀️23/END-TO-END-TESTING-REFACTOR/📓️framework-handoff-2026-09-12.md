# Framework Testing Taxonomy Handoff

Date: 2026-09-12  
Lane: framework

## Immediate Consumer Mappings

### Core Pack Rust Test Surface

- Path: `🧰️framework/🔨️modules/🎒️pack/🧨️testkit/🦀️.rs` -> `🧰️framework/🔨️modules/🎒️pack/🧪️tests/🧨️corruption/🦀️.rs`
- Module: `pack::testkit` -> `pack::corruption_testing`
- Exposure: unconditional `pub mod testkit` -> `#[cfg(any(test, feature = "corruption-testing"))] pub mod corruption_testing`
- Feature: downstream tests enable `semio-framework-pack/corruption-testing` only through dev/test dependency resolution.

### OS Pack Rust Test Surface

- Path: `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/🦀️.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🧬️record-codec-laws/🦀️.rs`
- Module: `crate::os_pack::testkit` -> `crate::os_pack::record_codec_laws`
- Exposure: OS package mount becomes `#[cfg(test)] pub mod record_codec_laws`; the compatibility `testkit` module is removed.

### OS JCO Probe Test Ownership

- Source owner: `🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe` -> canonical case `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback`
- Guest semantic owner: `…/🧪️testkit/🧩️jcoprobe/👽️guest` -> `…/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest`
- Guest package root: `…/👽️guest/📦️packages/🦀️rust` -> `…/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest/📦️packages/🦀️rust`
- Browser implementation: `…/🧪️testkit/🧩️jcoprobe/🌐️harness` -> `…/🧪️tests/🧩️jcoprobe-callback/🟦️typescript`
- Browser support files: `…/🌐️harness/🧩️support/🖥️host-shim.js` -> `…/🟦️typescript/🖥️host-shim.js`; `…/🌐️harness/🧩️support/🪞️preview2-shim/*` -> `…/🟦️typescript/🪞️preview2-shim/*`
- Generated bundle fixtures: `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness` -> `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles`
- Package identity remains `semio-jcoprobe-guest`; only its semantic owner coordinates change.
- Nx project: `@semio-tech/browser-actor-import-testkit` -> `@semio-tech/browser-actor-import-test`; launch consumers must update both `runtime-check` and `pending-host-close-check` target coordinates.

### OS Scale Test Ownership

- Owner: `🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale` -> `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale`
- Package root: `…/🧪️testkit/⚖️scale/📦️packages/🦀️rust` -> `…/🧪️tests/⚖️scale/🦀️rust/📦️packages/🦀️rust`
- TypeScript contract: `…/🧪️testkit/⚖️scale/🟦️.ts` -> `…/🧪️tests/⚖️scale/🟦️typescript/🟦️.ts`
- Component output: `…/🧪️testkit/⚖️scale/📦️packages/🦀️rust/dist/component/*` -> `…/🧫️fixtures/⚖️scale/🧩️component/*`
- Package identity remains `semio-framework-os-scale-fixture`; `package.metadata.semio.role` and Nx tag change from `testkit` to `test`.

### Plugin Rust Test Surface

- Inline module: `crate::app::testkit` -> `crate::app::artifact_app_laws`
- Root re-export: unconditional `pub use app::testkit` is replaced by `#[cfg(any(test, feature = "artifact-app-testing"))] pub use app::artifact_app_laws`; no compatibility alias remains.
- Cargo exposure: downstream plugin test crates must enable `semio-framework-plugin/artifact-app-testing` through dev/test dependency wiring only.
- Framework test consumers must import `crate::app::artifact_app_laws`; `✏️s` consumers are owned by the plugin lane and must apply the same symbol replacement.

### DB Rust Test Surface

- Path: `🧰️framework/🛍️products/💻️os/🔨️modules/🫢️db/🧪️testkit/🦀️.rs` -> `…/🫢️db/🧪️tests/🧯️fault-storage/🦀️.rs`
- Module: `crate::db_testkit` / `db_testkit` -> `crate::db_fault_testing` / `db_fault_testing`
- Exposure: the module and every `DbBackend::Fault`/facet variant are `#[cfg(test)]`; production builds have no fault-storage type edge.

### SPR Rust Test Surface

- Path: `…/📡️spr/🧪️testkit/🦀️.rs` -> `…/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`
- Module: `crate::os_spr::testkit` -> `crate::os_spr::protocol_laws`
- Exposure: `#[cfg(test)] pub mod protocol_laws`; no normal/public testkit mount remains.

### MCP Rust Test Surface

- Path: `…/🌉️mcp/🧪️testkit/🦀️.rs` -> `…/🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs`
- Module: `crate::testkit` -> `crate::source_builders`
- Re-export: `mcp` facade re-export of `testkit::*` is removed; tests address the explicitly test-gated semantic module.

### Small Test-Only Rust Modules

- `crate::window_transient_testkit` -> `crate::window_transient_owners`, path `…/🧪️testkit/📢️scalar-publication/🦀️.rs` -> `…/🧪️tests/📢️publication-owners/🦀️.rs`.
- Machine `testing::support` -> `testing::toggle_model`, path `…/🧪️tests/🔬️testing-support/🦀️.rs` -> `…/🧪️tests/🔀️toggle-machine/🦀️.rs`.
- Directory client `test_support` is co-located as the case-specific `…/🧪️tests/🔬️unit/🧱️transport.rs`; the parent-level module and symbol are removed.
- Reactor `test_support` -> `reactor_driver`, path `…/🧪️tests/🔬️test-support/🦀️.rs` -> `…/🧪️tests/⚛️reactor-driver/🦀️.rs`; it remains explicitly `#[cfg(test)]` because both reactor and lifetime cases consume its semantic driver surface.

## Metadata Ownership Boundary

The native guard lane owns taxonomy/discovery/normalization metadata and root owns root `📜️script.ts`. Those owners must update every old JCO coordinate above; this lane does not edit those scopes.

## Coordinator Correction — Direct Case Implementations

The guard lane verified that implementation directories inside a test case are forbidden: test-implementation-depth and validateCaseContract require direct 🦀️.rs / 🟦️.ts implementations. The proposed JCO case/🦀️rust/... and case/🟦️typescript/... layouts MUST be replaced before final verification. Put the JCO component guest under a semantic owner outside tests, with its package beneath that owner; put browser bridge code under the same semantic owner and invoke it from direct canonical case files. Keep generated browser bundles in owner fixtures. Publish the final corrected mapping here so root Cargo and guard metadata can follow. Do not relax the canonical guard to accommodate nested case package trees. Root needs the scale package final owner path too.


## Root-Owned Final Synthetic Input Relocation In Progress

Root is now relocating JCO guest to OS/🧫️fixtures/🧩️jcoprobe/👽️guest, browser stand-ins/host to OS/🧫️fixtures/🧩️jcoprobe/🌐️browser-host, and scale program to OS/🧫️fixtures/⚖️scale. Executable assertions stay direct leaves at OS/🧪️tests. Root is repairing coordinate references, Cargo and launch refs. Do not restore nested implementation directories under cases. The source-project output restriction is satisfied with dist/component inside the fixture package. Final ledger/report will be 📓️synthetic-input-taxonomy-2026-09-12.md.

Root also moved browser actor-import dispatch/project metadata to the browser-bundle semantic owner; the direct 🧪️tests/🌊️actor-import/🟦️.ts remains. Nx project name and both launch commands stay @semio-tech/browser-actor-import-test. See 📓️browser-dispatch-taxonomy-2026-09-12.md.
