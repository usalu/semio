# Final Framework Fixture Audit

**Date:** 2026-09-09  
**Status:** Action required — the physical fixture placement is clean, but four indirect framework dependencies still cross the boundary.

## Scope and method

This was an independent, read-only audit of `🧰framework`, excluding `🛍️products/🦑️repo`, generated output, dependency directories, and build directories. It checked the live canonical fixture inventory, source-shaped fixture content, direct Rust includes, literal TypeScript/JavaScript consumers, and framework runtime/build/config references. It also reviewed the earlier [framework separation report](📓️framework-fixture-separation-2026-09-09.md) and [JCO follow-up](📓️jco-browser-fixture-classification-2026-09-09.md). It did not repeat the completed repository-wide scan.

The supporting inventories are in [🗑️generated/framework-final-audit](🗑️generated/framework-final-audit/): [canonical inventory](🗑️generated/framework-final-audit/current-canonical-fixture-inventory.json), [literal references](🗑️generated/framework-final-audit/framework-fixture-literal-references.txt), [parsed config candidates](🗑️generated/framework-final-audit/fixture-config-candidates.json), [case-boundary result](🗑️generated/framework-final-audit/canonical-case-boundary.json), and [resolved Rust include edges](🗑️generated/framework-final-audit/rust-fixture-includes.json).

## Actionable findings

### F1 — A framework static route exposes a fixture root that no longer exists

`🧰framework/🔨modules/🖱ui/🎨styling/🟦.ts:1497` configures the `"/cad-fixture"` static route with `✏️s/🔌️plugins/📐cad/🧫fixtures` as its root. That directory is absent. The static-bundle builder silently skips the copy when the root is absent and the middleware falls through, so the configured route currently serves no asset.

**Action:** decide the canonical CAD reference plane, place the intended browser asset under its semantic `🖼assets` owner, and point `PLAYGROUND_PLAY_STATIC_ASSETS` there. Verify both static-bundle copy and an HTTP `GET /cad-fixture/...`. If the route has no intended consumer, remove it. A production playground route must not be rooted in a fixture tree.

### F2 — Four build/check target inputs hash fixture data for work that does not consume it

Nx's root `production` input is based on `default` and removes test paths but does not remove fixture paths. The following targets therefore retain fixture data in their cache/input boundary even though their commands build or check production code:

| Location | Affected target | Exact correction |
| --- | --- | --- |
| `🧰framework/🛍products/💻os/📦packages/🟦typescript/📋project.json` | `generate-wgpu`, `check-wgpu` | Define a source/generator input excluding `💻os/🧫fixtures`; assign it to both targets. |
| `🧰framework/🛍products/🌉mcp/📦packages/🦀rust/📋project.json` | `build` | Define an MCP source-only input excluding `🌉mcp/🧫fixtures/🔣first-party-codecs.json`; use it for `build` and retain the fixture for test targets. |
| `🧰framework/🔨modules/🌱value/💾resident/📦packages/🦀rust/📋project.json` | `check-wasm` | Give the Cargo library check an input that excludes its JSON test vector. |
| `🧰framework/🛍products/💻os/🔨modules/🖥shell/📦packages/🦀rust/📋project.json` | `check` | Give the Cargo check a source-only input; its fixture input belongs to its tests. |

This is a build-graph/cache dependency rather than a production byte inclusion, but it makes fixtures part of production-oriented target invalidation and obscures the ownership boundary.

### F3 — The Scale marshalling check still reads the removed fixture source path

`🧰framework/🛍products/💻os/🔨modules/🔌plugin/🖥host/📦packages/🦀rust/📜script.ts:306` reads `💻os/🧫fixtures/⚖scale/🦀.rs`. The source and Cargo package have moved to `💻os/🧪testkit/⚖scale`, and the old fixture source path is absent. The affected `UiPatchMarshallingCheck` will fail at file read when exercised.

**Action:** point this check at `💻os/🧪testkit/⚖scale/🦀.rs`. Keep the generated Scale output fixture where it is; only the handwritten Rust test input moved.

### F4 — An obsolete JCO test script imports the removed fixture support module

`🧰framework/🛍products/💻os/🧪tests/🧩jcoprobe-callback/🟦.ts:40` dynamically imports the removed `🧫fixtures/🧩jcoprobe/🌐harness/📞out-callback/🖥host-shim.js`. Its S3 path will fail if that script is run. The current OS Vitest configuration does not register this obsolete script; the live browser harness instead uses `🧪testkit/🧩jcoprobe/🌐harness/🧩support/🖥host-shim.js`, as verified in the JCO follow-up.

**Action:** retire the obsolete `jcoprobe-callback` script or update it to import the testkit support module. Do not restore the support file to fixtures.

## Confirmed boundary properties

- The canonical inventory contains 701 fixture files: 610 structured configuration/data files, 48 opaque data files, and 43 source-shaped files.
- The 43 source-shaped files have bounded test-data roles: 38 generated JCO expected outputs, three Rust/WIT actor-import guest inputs compiled by a test, one multi-shell HTML document used as styling test data, and one hostile Rust compile-fail input. No source-shaped file is a production implementation owner.
- The JCO outputs remain expected generated data. Their current generated bindings reference the testkit-owned support module; the 13 handwritten worker/page/shim files identified by the follow-up are no longer present in the fixture tree.
- No canonical fixture file is physically under a `🧪tests` case. The 12 fixture files under `🧪testkit` are owned by the SPR testkit, so they are testkit data rather than a case-local fixture.
- No legacy `fixture`/`fixtures` directory name was found in the scoped framework tree.
- The Rust include audit resolved 535 literal fixture includes: 534 originate in `🧪tests` or `🧪testkit`. The one remaining caller is `💻os/🔨modules/🔌plugin/🧵retained-command/🦀.rs`, where the include is guarded by `#[cfg(test)]`; there is no unguarded production Rust literal include of a fixture.

## Bounded limits

- This audit did not execute Bun, Nx, Cargo, browser, or HTTP checks. The JCO server/browser success cited above is the result recorded in the reviewed JCO follow-up, not a new test run.
- Rust `include_str!` and `include_bytes!` paths were resolved exactly. TypeScript/JavaScript dynamic path construction was bounded to literal inventory and the production-like runtime/config consumers; 150 scoped source files contain a literal fixture reference and were not all symbolically executed.
- The audit inspected the CAD plugin tree only to resolve the framework static-route target. It did not audit that plugin or any excluded repository product as a separate fixture domain.

## Execution follow-up

**Status:** F1, F3, and F4 are resolved and verified. F2 remained untouched because the coordinator owns the Nx cache-input corrections.

### F1 — CAD static delivery now follows the canonical playground manifest

The dead global `PLAYGROUND_PLAY_STATIC_ASSETS` entry for `/cad-fixture` was removed. CAD and Koordinator already declare the live `/cad-assets` route against the CAD example owner's `🖼️assets` directory in their Cargo metadata, so the generated playground registry is the single source for those per-playground mounts. The WGPU development proxy now forwards `/cad-assets/`, and the registry plus bundled frame worker were regenerated.

An actual `node:http` server was started with `staticDirVitePlugin` and served `/cad-assets/🏗️modelDefinitions/🏢️aec.building/🏷️attributeDefinitions/🚪️opening.json` with status 200, `application/json`, 393 bytes, and SHA-256 `d30750d7b830a0ecd4720e6267f0e631de0a49eba745331ff9230f801c682607`. The same plugin's build hook copied byte-identical content into a temporary ticket output, which was removed after the assertion. The current global static list has zero CAD mounts, while the generated CAD and Koordinator registry rows both use `/cad-assets` and the semantic `📚️examples/🖼️assets` root.

### F3 — Scale source validation follows the testkit owner

The host check now reads `💻️os/🧪️testkit/⚖️scale/🦀️.rs`. `bun nx run @semio-tech/framework-plugin-host:ui-patch-marshalling-check --skip-nx-cache` passed with seven cases, eleven operations, and both sync and async paths.

### F4 — JCO callback script imports live test support

The callback script now imports `../../🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support/🖥️host-shim.js`. Running the script directly with Bun passed S1 through S4 and reported `ALL PASS`.

### Focused generator and consumer verification

- `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` passed and reported 59 plugin crates, 60 playgrounds, and 47 framework packages. This also refreshed `.vscode/launch.json` from the current shared workspace state; its unrelated source-driven changes belong to concurrent work.
- `bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker --skip-nx-cache` passed.
- `bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker --skip-nx-cache` passed, including its four generator dependencies.
- `bun nx run @semio-tech/ui-styling:test-quick --skip-nx-cache` passed: 41 tests and 1,371 assertions.
- A focused literal scan of the modified sources and generated consumers found no remaining `cad-fixture`, CAD `🧫️fixtures` root, old Scale fixture source path, or old JCO fixture host-shim import. The generic styling test's hypothetical `/cad-fixture` string remains isolated test input and is not an active route or filesystem dependency.

The first `plugin-registry:check-generated` run correctly detected the pre-existing stale CAD registry and launch output. A concurrent source change made `.vscode/launch.json` stale again after the first generation, so generation was rerun successfully against the final shared state. No repository-wide layout scan was run in this follow-up.

### Exact authored paths

```json
[
  ".vscode/launch.json",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️final-framework-fixture-audit-2026-09-09.md"
]
```
