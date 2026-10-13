# 2026-10-12 suite registration: 21 launch entries, 3 nx targets, 4 suites found already reachable

Executor: suite-registration (Sonnet). No git-modifying command, no ticket operation, no sub-agent, no Rust or product TypeScript edited, nothing compiled (release build #9 running). Only `📜️script.ts`, `📋️project.json` and the two `.vscode` launch files were touched. The temporary insertion script is `suite-registration/insert-launch.ts` in this ticket folder (kept as input; it refuses to run twice).

## How suites are registered here

- TS suites run through an nx target that calls `📜️script.ts <command> <suite>` (value: `test <suite>`; plugin: `test-<name> source|native`), plus a launch entry in group `4_gate`, name `⚖️test-<suite><domain emoji><lang emoji>`.
- Rust laws of an artifact or framework crate are reached through the crate's existing generic nx `test` target. `partitionNextestExecutionFilters` (read, and run over the new entries' arguments) classifies `--test <binary>` and `--features` as build selectors, so the sibling pattern `bun nx run <project>:test --skip-nx-cache --excludeTaskDependencies -- long --test <binary> -- --nocapture` selects exactly one integration target. No new nx target is needed for those; a launch entry is the registration.
- `launch.json` and `🧩️launch.seed.jsonc` are not full mirrors of each other (7314 vs 5184 configurations before); the earlier bulk-vec registration added to both, so did I.

## Registered

| # | suite | where | how |
| --- | --- | --- | --- |
| 1 | `♻️retirement/🔓️uncontrolled-owners` TS gate | value `📜️script.ts` suite `uncontrolled-owners`; value `📋️project.json` target `test-uncontrolled-owners-source`; launch `⚖️test-uncontrolled-owners-retirement🌱️🟦️` | sibling of `bulk-vec-retained` |
| 2 | `🧵️retained-command/🪟️mounted/🚨️structural-refusal` TS twin + Rust law | plugin `📜️script.ts` class `RetainedStructuralRefusalScript`, command `test-retained-structural-refusal source\|native`; plugin `📋️project.json` targets `test-retained-structural-refusal-source` / `-native`; launch `⚖️test-retained-structural-refusal🔌️plugin🟦️` / `🦀️` | native mode = the TS twin, then exact lib law `app::tool_run_tests::a_mounted_operation_reports_the_structural_refusal_it_retains` through `runRepositoryExactCargoLaws` (same shape as `test-retained-raw`), artifact dir `🗑️generated/retained-structural-refusal` of this ticket |
| 4 | `preparation_law`: shooting, png, bmp, dag, stdio-semio (with `--features component-app-assembly`), plus epw and txt (same target name and path, same class) | 7 launch entries `⚖️test-preparation-law<domain>🦀️` | generic `test` + `--test preparation_law` |
| 4 | `authoring_law`: shooting | `⚖️test-authoring-law🎥️shooting🦀️` | same |
| 4 | `paged_seal_digest`: replication | `⚖️test-paged-seal-digest📡️replication🦀️` | same |
| 4 | `retirement_law`: block 3d (with `--features component-app-assembly`, its `required-features`), writer, wires, forms, cad, sequence, space, collection | 8 entries `⚖️test-retirement-law<domain>🦀️` | same |
| 4 | `flow_retirement_law`: os flow crate `semio-framework-os-flow` | `⚖️test-retirement-law🌊️flow🦀️`, project `semio-framework-os-flow-core` (its `TestScript` runs that crate) | same |

Orders `900.058609601` ... `900.058609621` (group `4_gate`, after the `⚖️test-original-*` tail `900.0586095`; no clash with any existing order, no new name collides with an existing one). Both files: 21 entries inserted, 0 lines removed (diff against a pre-edit copy), strict `JSON.parse` OK for `launch.json` (7335 configurations) and the seed (5205).

## Found already reachable, not registered again

- Item 3, `🧵️retained-command/🎟️admission` (fixture v2 + TS oracle): the TS oracle is in the `bun test` list of `test-retained-raw` (`source`/`native`), the Rust test `retained_command::admission::tests::retained_raw_admission_has_no_failure_destructor_or_undeclared_allocation` is in its native exact-law group, and launch has `🧪️test🔌️plugin♻️raw-source` / `raw-native`. The oracle passes on its own (2 pass, 28 expect). See the red siblings below.
- `authoring_law` energy-model and raster-raster, `preparation_law` tiff: their crates have an unfiltered launch entry of the generic `test` target (`⚖️gate🔋️energy🔋️model🪶️sqlite🧪️test` with `quick --no-fail-fast`, `🧪️test🖨️raster🦀️`, `⚖️gate🖼️tiff6🦀️native`); `runCargoTestsV1` forces no `--lib`, so nextest includes every `[[test]]` target. Read from the code, not run.
- `⚖️typed-input-law` and `⚖️transient-preparation-law` (`🧰️framework/🔨️modules/🧪️test/`): shared helpers mounted by `#[path]` into the `retirement_law` targets above, with no `#[test]` of their own; the `typed-input-law/🔣️.json` is not a test corpus read by any suite I found. Reachable through the 9 retirement entries, nothing to register.

## Verification

| check | result |
| --- | --- |
| `launch.json` and the seed parse | yes (strict `JSON.parse`, which also satisfies JSONC) |
| `NX_FORCE_REUSE_CACHED_GRAPH` rebuild note | a new nx target is invisible to the cached graph; the first run of each new target ran without it (graph rebuild, minutes), later runs reused it |
| `nx run @semio-tech/framework-value:test-uncontrolled-owners-source --skip-nx-cache --excludeTaskDependencies` | **2 pass, 0 fail** (Ajv schema check + independent scanner equals the 4 reasoned owners) |
| `nx run @semio-tech/framework-plugin:test-retained-structural-refusal-source ...` | **2 pass, 0 fail, 5 expect** |
| `nx run ...:test-retained-structural-refusal-native` without `SEMIO_TEST_ARTIFACT_DIR` | routes, runs the TS twin (2 pass), then stops at the artifact-dir guard before any cargo call (proves the command is registered; the Rust law itself NOT run) |
| 18 generic-`test` Rust entries | `nx show project <p> --json` confirms each of the 17 owning projects (18 entries; shooting has two) has its `test` target, plus the new plugin and value targets; every entry's argument list passes `validateNativeCargoArguments` and `partitionNextestExecutionFilters` with `--test <binary>` (and `--features`) as build args. NOT compiled, NOT run |
| the exact law name `app::tool_run_tests::...` | derived from the source (`mod tool_run_tests` is a direct child of `pub mod app`, lines 462-14960 of the plugin `🦀️.rs`); unproven until a `--list` build runs |

## Findings (not fixed, outside registration)

- `test-retained-raw-source` is red, independent of this change: four existing TS suites in its list fail (`bun test` on each file alone): `🧵️retained-command/🧪️tests/♻️raw-allocation-close` 1 pass / 1 fail (greps Rust text `if !self.raw.is_empty(){return Ok(RetirementDemand{depth:1,..` that `🧵️retained-command/🦀️.rs` no longer contains), `🪟️mounted/♻️frontier` 3 / 1, `♻️lifecycle/🏪️stores` 3 / 5, `♻️cancellation/📦️lease` 3 / 1. The admission oracle in the same list passes. This is why structural-refusal got its own target instead of joining that list. Owners: whoever last changed `retained-command/🦀️.rs`, `♻️lifecycle` and `♻️cancellation` Rust.
- Two new TS suites exist that no registration of mine covers: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🗂️schema-document-index` (ticket `REPLACE-DSL-DERIVE-DIRECTORY-WALK-WITH-SCHEMA-INDEX`, another executor; its `launch.json` entries are there already, the seed lacks them) and `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating` (edited by the compile sweep, not new, no launch entry).
- The Rust entries are unproven until the release build ends; the first run of each compiles its test binary from cold.

## Files touched

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (21 entries each).
- `🧰️framework/🔨️modules/🌱️value/📜️script.ts`, `🧰️framework/🔨️modules/🌱️value/📋️project.json`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`, `.../📋️project.json`.
- This report and `suite-registration/insert-launch.ts` in the ticket folder. No `[DEBUG]` line added (the new native command prints `[TRACE]`); no `🗑️generated` created.
