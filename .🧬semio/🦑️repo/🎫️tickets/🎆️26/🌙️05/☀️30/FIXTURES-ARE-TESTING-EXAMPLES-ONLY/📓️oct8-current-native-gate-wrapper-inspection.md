# Current Native Gate and Process Wrapper Inspection

Read-only current source inspection requested by Root after first full fixture-zero / seven-owned-scope checkpoint and while its catalog refresh runs. Actual producers remain held. No neutral tests, source edits, generation, task-graph command, build or activation was executed. This records source wiring, not a new runtime verdict. The existing authoritative 16.0 s neutral preparation receipt remains unchanged in `📓️oct8-current-preparation-custody-settlement.md`.

## Normal Dependency Pair

Current root `📋️project.json` `deps-cargo` is an uncached `nx:run-commands` target calling existing dependency bootstrap native `📜️script.ts sync cargo`. The WGPU TypeScript project retains its authored `workspace:deps-cargo`, `workspace:deps-trunk`, and `workspace:deps-wasm-opt` prerequisites for both `wasm` and `wasm-release`.

Native `prepareDependencies("cargo")` enumerates current `discoverCargoWorkspaces(workspace)` and awaits `runPreparedCargoDependencyPairV1(workspace, owner.manifest, signal)` for every owner. Default runner equality with `runTool` selects no external runner. `runPreparedCargoDependencyPairV1` supplies its callback to `withPreparedCargoDependencyPairV1` and invokes the private `executeTool` directly. The private executor spawns exact Cargo argv with progress/cancellation; it does not call `prepareCargoWorkspaceInvocation`.

The current pair owns one exclusive `cargo-preparation:<root>` lease, calls `prepareCargoOwners` once, publishes current membership, and freezes original source/input/member/owner/recipe custody. Its callback arguments are exactly `['update','--workspace','--manifest-path',manifest]` from repository root followed by `['fetch','--locked','--manifest-path',manifest]` from repository root. It checks custody before update, after update, after fetch, and separately binds the resulting lock hash across locked fetch. The bootstrap and native owner scripts are in its source custody list. No public skip switch, persistent preparation reuse or modified Cargo arguments were found.

The separate ordinary `runTool("cargo", ...)` still prepares before general Cargo consumers, and invocation schema still names update/fetch/metadata/build among required consumers. The intentional strict pair avoids entering that general wrapper for either adjacent callback; changing its default executor back to `runTool` would reintroduce duplicate preparation, but that change is absent in current source.

## Current Global Process Wrapper

Current Nx plugin `projectWithDefaults` can wrap selected Bun owner commands whose closure reaches framework process context or Vitest owners. Its explicit condition excludes commands already wrapped as native `owner-command|repository-test-body`; it binds the selected script to its real owner path and calls library TypeScript `📜️script.ts owner-command --cwd <actual owner> -- <bound Bun script> ...`.

That `OwnerCommandScript` parses exact cwd/command, derives existing repository Vitest/process-owner context through `devToolingEnv`, and invokes framework-neutral `runOwnedCommand` with budget 0. The framework-neutral runner uses Node spawn, streamed output, progress and owned process-tree cancellation. It has no Cargo workspace/preparation import or preparation call. Thus an outer owner-command wrapper around the native bootstrap or WGPU Bun script does not itself execute Cargo preparation. It preserves the selected nested owner and the environment forwarded by current policy.

Repository-specific `runRepositoryCommand` is a different wrapper and still prepares only when its selected command is literally `cargo`. It is not used by the dependency pair's private executor or the global `OwnerCommandScript`. Current inferred `withNativePreparation` only attaches explicitly owned generator prerequisites to native leaves; it does not dispatch the Cargo preparation helper in a process wrapper. It deduplicates generator target additions already declared in `dependsOn`.

The actual WGPU compiler still calls general `runTool` for its separate locked/offline metadata operation and Trunk, and invokes existing Cargo relay via Bun for the actual unchanged raw compiler query. Those subsequent distinct consumption boundaries are not the adjacent update/fetch pair. No concrete new duplicate preparation through the peer global process wrapper was identified in this source inspection. Actual runtime confirmation remains gated by Root's release.

## Actual Native Wrapper Qualification

The above generic Process wrapper conclusion is superseded for the actual released WGPU dependency task. At approximately 15:20:27 UTC actual Nx PID 10061 dispatched child 10810 as `bun <Cargo driver> native owner-command --manifest ./Cargo.toml --cwd . -- bun <absolute native bootstrap script> sync cargo`. That child's 10812 was the actual preparation script `prepare --manifest Cargo.toml` before the selected bootstrap body. Current `NativeScript` in `⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts` eagerly calls `prepareCargoWorkspaceInvocation(test, actual manifest)` for native owner-command. Native-policy inference takes priority over generic Process wrapper inference when its eager closure includes a native-build owner. Consequently the actual normal native dependency task incurs an extra whole-workspace preparation before its correct strict adjacent pair. This is concrete runtime wiring evidence; the generic Process wrapper itself remains neutral. Root and Low were notified; no source edit or command bypass was made, and the producer remains owned by session 15567 pending Root direction.

## Source Custody

Fresh read-only identities are appended below, with current six authored endpoint hash equality against the settled ledger. No source hashes were adopted into the authored ledger or rewritten by this inspection.

All 6 current authored endpoints match the settled exact SHA ledger.

- 📋️project.json: 6bfefebe55f80a0fa310596375d86200cfc0c332110e341b524b2e80632040b6
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs: e9a2e397bbc9bfba254b6cf999e14e3c151db19fa02d1cac0fbc6400dfd9097a
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts: dc98cd0776828782a1aff69a67b25e0b3b53cf9a3cb51db3051703727aafde5b
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts: c71c17ad3f36176dffacbc8c0e798b8d2611b2b4bb08406fcfed69bb294a46c3
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts: a15057507c60a963ecd40c9ef7508a097e7b910b68a670ae071bd703ae3341aa
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts: a5a216eccb54bf9f9353781c4f79a9d7fb8588a278d712a42f24be9ae1e1196d
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts: 0ec5b66b7ade530c5a7461c0238b4bf417e3ee8683f988fe0961f268f0b26456
- 🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts: b91633467acfb794bce4dfca15979a4c0620f61c86d892fecb3aceb0ea4f1001
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json: 405898da3b0b2b956637579baef675b783e01b23a4450b8027dde4fc125d6725

Current devToolingEnv spreads process.env and supplied extras, removes debugger NODE_OPTIONS/VSCODE_INSPECTOR_OPTIONS, and applies only unset Nx tooling defaults. It leaves the approved Cargo directories/resource root and original budget selections intact. Environment owner identity: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts: 35f589921a1b2a5dfa572fc658f0868990f12e6a8689b369367f925e6edbb486
