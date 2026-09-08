# Validation Record

## Pre-change Nx Inspection

`bun nx show project @semio-tech/stdio-plugin --json` was invoked through the required root Bun/Nx router before extraction. It remained in repeated project graph construction while the shared workspace changed and produced no JSON result. The coordinator terminated only its own router process (PID 53989), allowing its existing cancellation handler to stop owned children. The shared Nx daemon and other agents’ processes were not stopped. This inspection is **not a passing validation**. A bounded `NX_DAEMON=false` graph run will be used once package declarations settle.

## Planned Required Checks

- Language-neutral package schema positive/negative cases, using a third-party validator.
- Cargo metadata independent artifact closure and DAG.
- Nx project graph, owned source inputs and cache behavior.
- Representative package build and runtime codec/oracle test.
- Home-io and full-catalog compilation and existing registry commitment checks.
- External consumer imports and direct Cargo dependencies.

Results will be appended as commands complete.

## Package Contract Before Extraction

The Nx executor ran `bun '✏️s/🔌️plugins/🗄️stdio/📜️script.ts' package-contract` before production source extraction. The third-party Ajv validator accepted two valid cases and rejected two invalid cases. The source contract then exited 1 at the missing `💾️binary/📦️packages/🦀️rust/Cargo.toml`, demonstrating the intended red state. The coordinator inspected the actual captured output. This validates the schema cases and red-state detection, not the completed package feature. Final command integration must run through Nx.

## Direct Consumer Nx Check

`NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/consumer-forms bun nx run @semio-tech/forms-plugin:check --output-style=static` failed before running forms check. Its framework-graph generation prerequisite rejected two current taxonomy contracts: `print-latex-tokens` and `report-actor-network` previewTarget did not route exactly to their declared owner script invocation. Nx exited130 and explicitly marked forms check not run. Entity and styling generation prerequisites ran first. No consumer compile success is claimed. Raw evidence: `🗑️generated/consumer-forms-check.txt`.
