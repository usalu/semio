# Framework and Core Testing Taxonomy Acceptance Audit

Date: 2026-09-12  
Scope: fresh read-only acceptance check of current framework/core test layout and active runner references. No source, configuration, Git, or generated-output change was made by this audit. No test or build was run here.

## Result: accepted

There are no remaining framework/core testing-taxonomy findings in the checked working tree.

The ticket’s full repository census, `📓️testing-taxonomy-census-2026-09-12.md`, records zero findings over 139,975 discovered filesystem entries. This audit independently reconfirmed the framework subset: no obsolete testing-category directory (`🪨️tests`, `🪞️fixtures`, `🧪️fixtures`, singular `🔮️oracle`, `testkit`, `test-helper`, or `test-harness`) exists, and every framework file below `🧪️tests` is exactly at `<case>/<implementation>` depth (`framework_test_files_outside_case_implementation_depth=0`).

## Current runner and reference evidence

| Check | Current evidence | Result |
| --- | --- | --- |
| Scale fixture project root | `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json:4` now sets `sourceRoot` to the existing owner fixture `…/🧫️fixtures/⚖️scale`. | Pass |
| Scale fixture launch target | All four live launch references use the declared `@semio-tech/framework-os-scale-fixture:build-wasm`: `.vscode/launch.json:10174,12417` and `.vscode/🧩️launch.seed.jsonc:8596,10839`. | Pass |
| Host test source reader | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:306` reads the existing `…/🧫️fixtures/⚖️scale/🦀️.rs`, not the deleted test root. | Pass |
| Host component consumer | The component artifact is consumed only by the registered `ui-patch-marshalling-check --native` test command (`📜️script.ts:320`; `📋️project.json:53-70`), which also requires `SEMIO_TEST_ARTIFACT_DIR`. | Pass |
| Dev component consumer | The only dev-side use is `BenchPluginsScript`’s `bench plugins native` path (`📜️script.ts:5363`), reached by the `bench-plugins-native` target which depends on the fixture’s `build-wasm` target (`📋️project.json:268-280`). | Pass |
| JCO launch project | The earlier read-only `NX_DAEMON=false bun nx show project semio-jcoprobe-guest --json` resolved the current JCO fixture Cargo project and its `build`, `check`, and `test` targets. | Pass |

The host check and native benchmark are test/benchmark execution, not production-runtime consumers. Their containing dispatchers also provide other commands, but call-path scope is decisive; the scale fixture remains correctly under `🧫️fixtures`. No move of either complete dispatcher task is required.

Rust `include!` wiring was accepted by audit direction. Cargo-feature or Rust-module identifiers named `testkit` are not physical testing categories; the framework physical census contains no such directory. Historical reports, generated census input predating moves, and intentional negative vectors were excluded from active findings.

## Retained correction record

An earlier copy of this audit was accidentally written to the sibling `.🧬️semio` path with U+FE0F after `🧬`. It is preserved as requested. Its production-fixture-dependency classification is superseded by `📓️framework-testing-scale-consumer-followup-2026-09-12.md`; this report is the corrected copy in the opened `.🧬semio` ticket.
