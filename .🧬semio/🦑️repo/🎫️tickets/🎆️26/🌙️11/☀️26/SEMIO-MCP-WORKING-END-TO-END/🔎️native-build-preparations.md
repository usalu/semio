# Native Build Runs Declared Crate Preparations

## Finding
- Crate preparations (`[package.metadata.semio.preparation]`) run inside `prepareCargoOwners`/`prepareCargoWorkspaceInvocation` (🧰️framework/🛍️products/🦑️repo/.../🗂️workspaces/🦀️cargo/🟦️.ts), which `repo cargo` uses.
- `buildComponentClosureV1` only called it when `prepareCargoOwners: true` (set solely by the dev playground). `describeComponentsV1` (`components describe --all`) never did, so source admission saw absent generated files.

## Change
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts: new `prepareComponentOwnersV1` (one loop over the plan's passes through `prepareCargoWorkspaceInvocation`); called unconditionally by `buildComponentClosureV1` and by `describeComponentsV1`, before the driver (so before the release source snapshot). Option `prepareCargoOwners` removed.
- .../🛠️dev/🎮️playground/🟦️.ts: dropped the now-removed option.
- New test .../🧩️driver-stages/🧪️tests/🟦️.ts (ordering + reuse + no hand-named generators): 2 pass. Typecheck of the 3 files: 0 errors.
- Receipt witness skips recipes whose receipt is still current.

## Not done (blocked)
Root 📋️project.json `components.dependsOn` still lists the three generator targets, because the coordinator's `describe --all` (pid 8049) was still running. After it ends: remove `@semio-tech/flow-plugin:graph-generate`, `@semio-tech/stdio-plugin:composition-prepare`, `@semio-tech/stdio-plugin:composition-prepare-catalog-definition` from it; the stdio target `composition-prepare-catalog-definition` is then unreferenced (only the Cargo.toml recipe uses the script command) and can be removed too.
Not validated end to end (no describe run by design).
