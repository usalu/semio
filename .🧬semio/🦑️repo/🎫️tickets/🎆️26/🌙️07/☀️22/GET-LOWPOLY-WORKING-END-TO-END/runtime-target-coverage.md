# Runtime Target Coverage

Read-only source inventory, 2026-11-05. No Native, server, graph setup, or source edits.

## Missing Native Package Execution

Permanent Lowpoly owner `🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts` RuntimeTestScript currently selects nine packages: plugin-command, pixels, hub composition, editor, engine, IO, mut-objects, modes, viewer. None selects the new framework cancellation projection unit.

Exact owning Cargo package is **semio-framework-plugin-tests-11-2**, manifest:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🔧️tests-11-2/📦️packages/🦀️rust/Cargo.toml`.
It is an explicit root Cargo workspace member. Its [lib] points ../../🦀️.rs; that entry mounts the part-6 source as cfg(test) module plugin_builder_contract_tests_6. Part6 includes the shared-actions source, so the new test is compiled/run only when this package's lib test target is selected. Dependency compilation cannot execute it.

Project metadata is @semio-tech/plugin-tests-11-2-rs with no local declared test task; use the existing Lowpoly admitted Cargo helper package selection. Add exact package string above to RuntimeTestScript (ten total packages). For a genuinely focused direct Native route later, package selector is -p semio-framework-plugin-tests-11-2 --lib typed_cancellation_projection_obeys_the_neutral_contract (substring filter; do not add --exact without the complete Rust module path).

Current test-runtime cache inputs explicitly cover Lowpoly sources, command crate and pixels, but not this new unit host/test tree/shared projection implementation. Recommend these three additional explicit inputs:

- {workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🔧️tests-11-2/**/*
- {workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/**/*
- {workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🌳️tree/**/*

These cover concrete host manifest/entry, test source+neutral JSON/schema, and the projection source it verifies. Existing ^default alone is not evidence that sibling external #[path]/include test sources participate in hashing.

## Permanent React Picker Route

Owner target: **@semio-tech/framework-renderer-react:test**.
Owner project/script: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/{📋️project.json,📜️script.ts}`.
TestScript calls resolveTestLevel(segments,"quick"), then runVitest with config ../../🧪️tests/🎚️config/🟦️.ts.

Important: default/quick config includes only quickTestSuite. The picker suite is explicitly listed in engineTestSuites, which requires **long** (or exhaustive). Supplying a file filter alone at quick level cannot include it.

Exact owning public script route from repo root, once genuine JS dependency publication is ready:

```powershell
repo './🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts' test long '../../../../🧪️tests/💾️media-export-and-natural-files/🟦️.ts'
```

Equivalent graph target selector is @semio-tech/framework-renderer-react:test with forwarded arguments long plus the same relative file filter. The direct owning script route above avoids ambiguity about argument forwarding. This is a future execution instruction, not a claimed run; current doctor still reports no JS publication.

## Cancellation Corpus Follow-Up

Verify now reports fourteen authored cases, adding wrong proof-controller, wrong definition/registry-controller, registryless authored false/true preservation, plus explicit registryless assertion. Strict Ajv passed under Verify execution; Native remains unexecuted. This closes the earlier source-review coverage follow-up in cancellation-projection-review.md at authored level only.
