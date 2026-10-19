# Result fixups4: root self-test import and mcp `workspace_identity`

## 1. Root script no longer imports the stdio plugin self-test
Finding (`result-tst4.md`): the root `📜️script.ts` loaded `semioMemberStoreProducerSourceSelfTests` from the stdio plugin tree, so the neutral coordinator could not build with domain plugin sources removed (`🧱️owned-script-routes`, which counts static and dynamic imports).
Other self-test or test-module imports in the root script: none static (only type imports and the router). The remaining lanes are dynamic imports inside `loadRootVerifyScript` and none points into `✏️s`, `🌎️hub`, `♻️mit-bestand`, `compose` or `🗿️artifacts`. Left unchanged: their lane signatures are synchronous and are used by the gate directly.

Fix (owned route, same mechanism as the fem `verify …` routes):
- New owned route `verify semio-member-store-producer-source`, target `verify-semio-member-store-producer-source` in `@semio-tech/stdio-js` (`workspaceCommand`). No `cache` flag: the lane reads Rust sources outside the project and the Nx hasher cannot see them.
- `StdioVerifyScript` (appended to the artifact-commands module) loads the lane lazily; the stdio router registers it with `registerLazy("verify", …)`.
- New `🧪️tests/🧿️semio-member-store-producer-source/🟦️.ts` (`verifySemioMemberStoreProducerSource`) binds the plugin self-test to the framework check, sources and domain schema, all by relative path.
- The lane is removed from `SelfTestLanes` (with the now unused `MemberProducerSourcesPort`), from the root lane registry and from the `selfTests` sum of the tool-job-coverage run (also dropped its unused type import).

## 2. Zero-touch regressions found by the required run, fixed
`verify zero-touch` initially failed with 2 findings that a peer reintroduced at 19:00 (bare workspace imports in the `root-setup` closure). Re-pointed to owner files, no barrels:
- `L/🎛️commands/🔧️setup/🟦️.ts`: `@semio-tech/framework-os-mcp` -> `…/💻️os/🔨️modules/🌉️mcp/🟦️.ts` (owner of `ensureMcpBinary`; the package barrel would pull `ajv`).
- `L/🏃️process/🟦️.ts`: `@semio-tech/process/artifacts/native-build/source/admission` -> relative path to `…/🏗️native-build/📥️source/🎟️admission/🟦️.ts`.

## 3. mcp workspace Rust `workspace_identity`
`🌉️mcp/🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs` is mounted by `#[path]` into crate `semio-framework-os-mcp-catalog` (`🌉️mcp/📚️catalog-compiler`). Owner crate of `is_workspace_root`: `semio-framework-process-routing-workspace`.
- Added `use semio_framework_process_routing_workspace as workspace_identity;` (same alias as `🏃️run/🏗️bootstrap`, `🏪️store/🔄️sync`, test runner).
- Declared the dependency in `🌉️mcp/📚️catalog-compiler/📦️packages/🦀️rust/Cargo.toml` (path to `🏃️process/🧭️routing/🏠️workspace/📦️packages/🦀️rust`, verified to exist). `Cargo.lock` still needs the new edge; cargo adds it on the first non-`--locked` run.

## Changes (files)
- `📜️script.ts`
- `L/⚖️laws/🧪️self-test-lanes/🟦️.ts`, `L/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts`
- `L/🎛️commands/🔧️setup/🟦️.ts`, `L/🏃️process/🟦️.ts`
- `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📜️script.ts`, `…/📋️project.json`
- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏃️artifact-commands/🟦️.ts`
- new `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🧿️semio-member-store-producer-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs`, `🌉️mcp/📚️catalog-compiler/📦️packages/🦀️rust/Cargo.toml`
(`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.)

## Verification
- `bun ./📜️script.ts verify zero-touch`: `passed.` (after the two re-points).
- `bun build --target=bun 📜️script.ts`: OK. `bun build --target=bun` of the stdio package script and of the tool-job-coverage run module: OK.
- Direct call of `verifySemioMemberStoreProducerSource(repoRoot)` via `bun -e`: returns 13 cases, no throw.
- `bun test ./L/🧪️tests/🧱️owned-script-routes/🟦️.ts`: 3 pass, 1 fail. The root-imports test and the esbuild "builds with every domain plugin source removed" test pass. The failing test is the known one (about 46 `workspaceCommand` routes of value-rs, framework-plugin, framework-replication-rs … without a dispatcher call site); the new route is not in its list.
- `verify docstrings emoji-unique --files-from <2 new/edited ts files>`: repeats=0.
- `bun ./📜️script.ts verify semio-member-store-producer-source` (and every other owned `verify` route, e.g. `fem-pcg-publication-owners`): fails with "Original Repo build caller required" in `readOwnedBuildInvocationV1`. Pre-existing and unrelated to this change.
- `bun repo run @semio-tech/stdio-js:verify-semio-member-store-producer-source`: not completed. 400 s alarm while waiting for shared access to `native-fixed-image` (queue held by peers).
- `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-mcp-catalog --lib --tests`: NOT RUN to completion. 590 s alarm while waiting for exclusive access to the cargo registry lock (held by other fleets). The crate compile is therefore unverified.

## Open items
- Run the cargo check above once the registry lock frees; it also writes the `Cargo.lock` edge for the new dependency.
- Owned `verify` routes cannot be dispatched from the root: "Original Repo build caller required" (capability envelope not passed to the spawned `repo` child). Needs a fix in `🏗️build/📥️caller` or the dispatcher.
- `🏪️store/🔄️sync/🚨️sync-errors-actor-identity-and-envelope-serde/🦀️.rs:7` uses `workspace_identity` like the mcp file did; its alias is declared only in the parent `🔄️sync/🦀️.rs`. Check whether it is a `#[path]` child that needs its own `use` (not touched).
