# Dashboard Generation Retirement Execution

The root and repo AGENTS instructions were read. The explicit dashboard-only objective replaces the standing IDE registration instruction. No AGENTS file or Git state was modified.

## Implemented

- Removed the library launch-catalog generator, schema, public reexports, tests and editor fixture.
- Removed crate-tree editor launch-routes generator, schema and obsolete test import/test. The crate authoring command retains `registerBuildOwnerRows` and no longer calls an editor projection hook. The capability agent confirmed the dashboard already derives native controls from neutral owner declarations and project targets.
- Removed `GenerateScript` launch routing, its generator/check method, and root generate-launch/check targets. Supported generators load their own dependencies asynchronously only when selected, so rejected commands do not require unrelated compiler packages.
- Removed the dashboard editor projection module, schema, fixture and tests, the package `LaunchScript` implementation and launch target. Native run/build/daemon routes remain.
- Replaced the old library test-launch-catalog route with test-dashboard-control. Added a JSON Schema, language-neutral fixture and Gherkin scenario validating generation retirement and retention of native controls. The fixture schema is independently validated by the repository compiler and Ajv. TypeScript's JSON parser independently validates inspected project manifests against standard JSON parsing.
- Preserved `.vscode/launch.json` and its seed until the capability agent's conservation audit authorizes deletion; that deletion belongs to the parent/capability phase.

## Validation

1. The new retirement regression initially failed because the launch-catalog implementation still existed; its schema/Ajv differential passed.
2. After implementation, `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts` passed all 3 tests and 42 assertions. Runtime tests directly invoke the retired `GenerateScript` launch and launch --check dispatch, observe exit code 1 and usage without launch, and print `[DEBUG] generate launch and generate launch --check reject with exit 1`. No shell, editor document or generator function is invoked by those rejected paths.
3. The complete existing crate-tree aggregate ran 119 tests: 105 passed, 14 failed, 467 assertions. Failures involve existing Windows path representation and fixture discovery: slash/backslash mismatch, missing expected generated paths, applied-layout fixture reporting no artifact package, and one 5-second fixture timeout. The two build-owner registration/rename tests passed during this run. No crate implementation was changed.
4. Subsequent isolated and aggregate verification attempts could not resolve shared workspace aliases `@semio-tech/compiler-syntax-rust-ts`; a later rerun of the new suite similarly could not resolve `@semio-tech/schema-validator-ts`. These same aliases resolved during the completed runs above. The parent confirmed the isolated dependency layout intentionally does not expose those ambient aliases. The new suite now imports the schema validator through its relative public package entry, and GenerateScript loads only the selected live command dependencies; retired invocations reject before loading unrelated workspace packages. Final rerun: all 3 tests passed, 42 assertions. No dependency state was changed by this agent.
5. Scoped ripgrep found no remaining active source references to removed APIs outside deliberate authority/retirement assertions. Read-only `git diff --check` passed for the root target manifest, GenerateScript and dashboard package script.

## Files

Updated:
- `📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️authoring/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️crate-tree/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json`

Created:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🥒️.feature`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🔣️.json`

Removed:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🚀️launch-catalog/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🚀️launch-catalog/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🚀️launch-routes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚀️launch-catalog/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🚀️launch-catalog/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌳️command-tree/🚀️launch/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🚀️launch/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🚀️launch/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🚀️launch/🔣️.json`
