# Framework and Core Testing Taxonomy Acceptance Audit

Date: 2026-09-12  
Scope: current `🧰️framework` and OS/core runner references only. This was a read-only audit: no source, configuration, Git, or test-artifact changes were made. No build or test was run.

## Acceptance result

The physical framework test layout is now clean, including the previously reported raster witness. Three narrow active reference/configuration corrections remain before this area can be accepted without qualification.

| Status | Finding |
| --- | --- |
| Pass | The framework tree contains no `🪨️tests`, `🪞️fixtures`, `🧪️fixtures`, singular `🔮️oracle`, `testkit`, `test-helper`, or `test-harness` directory. Empty legacy directories are not present. |
| Pass | Every framework file under `🧪️tests` has exactly `<case>/<implementation>` depth: `framework_test_files_outside_case_implementation_depth=0`. |
| Pass | The raster witness test at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-witness/🟦️.ts:17` now imports its JSON from the owner-level fixture `…/🧫️fixtures/🖼️wgpu-raster-witness/🔣️.json`; no case-local data file remains. |
| Pass | `semio-jcoprobe-guest` resolves through the Nx Cargo discovery at `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust`, with declared inferred `build`, `check`, and `test` targets. The JCO launch commands therefore resolve. |
| Open | The scale fixture project metadata and two scale launch commands disagree with the actual target/root. |
| Open | Two production package scripts continue to hard-code a test fixture artifact path. |

Rust `include!` module wiring was intentionally accepted and not treated as a taxonomy violation. Rust `testkit` Cargo-feature/module identifiers were also not counted as legacy directories: the audit found no matching physical testing category, and these identifiers configure feature-gated implementation APIs rather than a neighboring test folder.

## Active corrections

### 1. Scale fixture `sourceRoot` names a removed test root

`🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json:4` sets:

```json
"sourceRoot": "🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale"
```

That directory does not exist. The current scale case is `🧰️framework/🛍️products/💻️os/🧪️tests/⚖️scale-profile/🦀️.rs`; the fixture’s Rust project root exists at `…/🧫️fixtures/⚖️scale/📦️packages/🦀️rust`.

**Correction:** point `sourceRoot` at the actual source owner/package root, or omit the field if it has no project-graph purpose. Do not recreate the deleted `🧪️tests/⚖️scale` root.

### 2. Two launch commands request an undeclared scale `build` target

The scale fixture project declares `check`, `check-wasm`, `build-wasm`, and `test`; it does not declare `build`.

| Active runner | Invalid target | Existing target |
| --- | --- | --- |
| `.vscode/launch.json:12417` | `@semio-tech/framework-os-scale-fixture:build` | `@semio-tech/framework-os-scale-fixture:build-wasm` |
| `.vscode/🧩️launch.seed.jsonc:10839` | `@semio-tech/framework-os-scale-fixture:build` | `@semio-tech/framework-os-scale-fixture:build-wasm` |

`build-wasm` is independently referenced by the companion launch entries at `.vscode/launch.json:10174` and `.vscode/🧩️launch.seed.jsonc:8596`, and its `bun 📜️script.ts build-wasm` command and script file both exist. Retarget the two stale entries to `build-wasm`; do not add a compatibility `build` alias.

### 3. Production package scripts still depend on a fixture artifact

The fixture component exists at `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm`, but these non-test script owners hard-code its path and load it:

| Production owner | Path declaration | Consumption |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts` | `:3` | `:320` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | `:10` | `:5363` |

This remains a production-to-fixture dependency, even though the previous `🧫️fixtures/⚖️scale/🟦️.ts` locator no longer exists. Move the scale-fixture orchestration/lookup to the OS test execution owner so these production package scripts no longer name a fixture path. The physical component should remain a fixture; it is not production static data.

## Reference checks performed

1. Searched active `📋️project.json`, `📜️script.ts`, `nx.json`, and VS Code launch configuration for deleted testing-root names. There is no active framework runner/configuration path reference to `🪨️tests`, `🪞️fixtures`, `🧪️fixtures`, singular `🔮️oracle`, `test-helper`, or `test-harness`.
2. Verified the scale fixture project’s `📜️script.ts` exists and its declared commands target that file.
3. Queried Nx project resolution read-only with `NX_DAEMON=false bun nx show project semio-jcoprobe-guest --json`; its root and all three launch targets resolve from the current JCO Cargo manifest.
4. Rechecked the guard census finding for the renderer raster JSON against the live tree. The old case-local path is gone; the owner fixture import is current. Historical reports, generated census records that predate the move, and intentional negative vector data were excluded from violations.

## Out of scope / non-violations

- Historical documents and negative-vector fixture contents were not reported as active references.
- Canonical Rust `include!` wiring was accepted by audit direction.
- A semantic `testkit` feature/module is not itself a filesystem testing category. No `testkit` directory remains in the framework physical census.
