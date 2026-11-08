# 🔍 1107 Merge Audit: commits 4f4fbeea720, fbf4da6a5aa, 0b2d54d02c4

Lane `aud-merge`, read-only. No repo runs, no sub-agents, no edits outside this folder. Range read: `8b3f9747de2..0b2d54d02c4` (commits 728, 729, 730). Scratch extracts sit in `🗑️generated/1107-*`.

## Ranked findings

### 1. HIGH: value crate `owned/shared-slots` points at a deleted file (compile break)
- Lane to act: **value** (agent `a1bf0499e1a315f51` per `1107-lanes.md`).
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔐️shared-slots/📦️packages/🦀️rust/🦀️.rs:7-8` has `#[path="../../🦀️.rs"] mod slots;`. That target `…/🧳️owned/🔐️shared-slots/🦀️.rs` was deleted in fbf4da6a5aa (59 lines at 8b3f9747de2; `git show 8b3f9747de2:<path>` still has it). Nothing at that path exists now, so rustc fails with a missing-module error.
- The crate is still live: `…/🧳️owned/📦️packages/🦀️rust/Cargo.toml:32` depends on it (`semio-framework-value-native-shared-engine`), and `🧰️framework/Cargo.toml:17` lists it as a workspace member. So the `owned` crate and everything above it fail too.
- Side-by-side duplicate: the same `NativeSharedSlots` now lives in `…/🛬️decode/🏭️construction/🧳️slots/🦀️.rs` (crate `semio-framework-value-native-shared-slots`, added in fbf4da6a5aa). That copy has the new work-accounting API (`capture_work_demand`, `copy_bytes: 2*size_of::<Option<T>>`). `🏭️construction/🪡️field` already depends on the new crate.
- Fix direction: delete the superseded `owned/🔐️shared-slots` crate and its workspace member line, then repoint `owned/Cargo.toml:32` to `semio-framework-value-native-shared-slots`. Keep the `#[path]` to `…/🧳️owned/🪡️field/🦀️.rs` (line 10-11, valid). The Cargo.toml edit requires `LOCK NEEDED` per the brief.

### 2. MEDIUM: demonstrator dependency tree predates the fast-forwarded locks
- Lane to act: **coordinator** (`dependencies sync|lock` is coordinator-only).
- Range changes: `♻️mit-bestand/bun.lock` (M, 0b2d54d02c4), `🌎️hub/bun.lock` (+48), `🧰️framework/Cargo.lock` (+258), `🧰️framework/Cargo.toml` (+19 members, 0 missing paths).
- `♻️mit-bestand/🧺️demonstrator/node_modules` symlinks to `.cache/b/d/m/j/083cae9e0a3e4dc2218d/w/305/node_modules`, which dates from Nov 6 13:21, before the lock change. Expect the dependency sync to be needed before activation.
- Not verified by run.

### 3. LOW-MEDIUM: Node source-closure loader refactor (watch item, checked as resolving)
- Lane to act: **repo build** owner (the coordinator routes it).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs:1-2` now imports `../🧬️schema/🔣️.json` and reads `definitions.Policy.const`. Lines 7 and 25 switch to `P.maximumSnapshotBytes` and a policy check.
- Schema limits change: `…/🧬️schema/🔣️.json:68` `maximumSourceBytes` 16777216 → 20971520, and `maximumSnapshotBytes` 25165824 → 33554432 (also at 177 and 205-209).
- `🟨️.d.mts` is not updated, but the exported signature is unchanged.
- Checked: the schema file exists, `Policy.const` resolves, and all relative imports resolve. Every node source bootstrap in the demonstrator goes through this path, so a regression here stops all Node scripts. Not run.

### 4. INFO: receipt and path contract used by the lane scripts is unchanged and consistent
- `serve.sh:12-14` expects `.cache/b/p/m/browser/prototype/runtime/react/<lane>/activation/🔣️receipt.json` for `generator`, `energy` and `fem3d`, plus `dist/♻️activation/prototype`.
- Framework: receipt name `♻️activation/🟦️.ts:8`. Activation root `🏃️execution/🟦️.ts:80`, serve root `🌐️serve/🟦️.ts:46`, readiness `🩺️readiness/🟦️.ts:150` (all `join(runtime,"activation")`). Runtime root `♻️activation/🟦️.ts:74-78` via `📂️paths/🟦️.ts:7` with `…/🧬️schema/🔣️.json` Protocol `browser/runtime`. Partition root `📦️budget/📂️paths/🟦️.ts:7`.
- None of these files changed in the range (verified by `git diff`).
- The receipts are absent now (no activation since 11/02), so `serve.sh` exits 4 until activation runs. That is expected, not a regression.
- `act.sh:4`'s `--tool=bazel` is in the selection schema (`🏗️build/⌨️command/🧭️selection/🔣️.json:197,224`).

### 5. INFO: target names referenced by the demonstrator still resolve
- `♻️mit-bestand/🧺️demonstrator/📋️project.json:34-42,299-301` and `🧫️pipeline.json` reference `@semio-tech/framework-os-dev:{prepare,activate}-<lane>-react-prototype` for the 9 lanes, plus the four `…:wasm` targets. All 9 lanes exist in the generated playground catalog (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json`, gitignored, Nov 4). Names come from `🧑‍💻dev/♻️activation/🎯️target/🟦️.ts:6`, which is unchanged. The four `wasm` targets exist in their project.json files.
- The `framework-os-dev` target inference in `🎮️playground-targets/🟨️.mjs` and `🏗️build/🕸️graph` is unchanged in the range.
- Added in range (no breakage found): a `typecheck` target in `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` and `TypecheckScript` in `📜️script.ts:2-3,73-82`. Both imports resolve, and `typecheckAdmittedProjectV1` / `createIssuedResourceControlV1` are exported.

### 6. INFO: process and value re-exports and deleted files
- `🏃️process/📦️packages/🟦️typescript/📦️artifacts/…/📋️contract/🟦️.ts:2` and `🏃️process/📦️artifacts/…/📋️contract/🟦️.ts:7` re-export `nativeWindowsAllocationUnitV1` from `📁️directory/🪟️host/📐️allocation/🟦️.ts:11`, which exists. `🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts` gains `toolCommandNameV1` and `playwrightBrowsersPathV1` with none removed.
- The native directory meter (`📁️directory/🟦️.ts:100-118`) moved to BigInt. Read-checked only: the schema keys `intervalBytes` and `intervalMilliseconds` exist, and `admitNativeDirectoryCapacityV1` receives Numbers. Not run.
- Deleted or renamed in range: 12 entries. Every `#[path]`/`mod`/`include_str!`/`fixture_text!` target I checked resolves, except the `shared-slots` case in finding 1. The `persistent/🦀️.rs:14` `mod traversal` is now inline and valid.

## Checks with a clean result
- Demonstrator tree (48 tracked `.ts/.tsx/.mjs`): every relative import resolves, and every named relative import is exported (following `export … from` chains). Bare `@semio-tech/*` named imports resolve through package `exports` maps, including `framework-os-dev/activation`, `plugin-registry/deployment`, `repo-lib/inventory/tooling`, and the `s-dev-catalog-ts/generated/plugins` entry. The generated catalog file exists.
- 156 removed definition names (filtered: length ≥ 5, no test-like prefixes) have no live references; the only hits are generic words in `bun.lock`, HTML and vendored JS.
- Removed-API sweep over 1076 changed-file specifiers: the only real miss is finding 1.
- Caching target `test-build-native-tool-route` is consistent: suite `build-native-tool-route` at `🏗️build/🗂️suites/🔣️.json:476`, and `⚡️caching/📜️script.ts` uses `workspaceTestPythonEnvironmentV1`, which exists.
- `test-paged-refusal-source` in the value crate is registered at `📜️script.ts:525`, and its inputs exist.

## Limits
- Read-only. Nothing compiled or run.
- The removed-name sweep skipped names with prefixes such as `allocation`, `controlled`, `window`, `chrome`, `receiving`, `original` and `cancel`. Those are not yet cross-checked.
- The `…` hits from the import scan were matches inside Rust and JSON fixture text, not module declarations. They were not checked one by one.
- Process note: one probe briefly created `$HOME/.cache/none`, outside this folder. It was removed at once.
