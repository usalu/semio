# Selected General Graph Scope Read-Only

Observed UTC: 2026-10-14T18:34:42.648938+00:00

## Actual Selection and Coupling

The actual live run is `repo run @semio-tech/framework-actor:canonical-architecture` (PID 53675). Its authored owner is Actor TypeScript project, target line 50: `dependsOn: []`, source execution, command `bun ./📜️script.ts test-product-independent-source`. The direct target declares no S/Hub prerequisite. This does not assert absence of source imports: dependencies and target inputs still need selected closure admission.

Build script line 140 calls `loadBuildGraphV1(root, invocation, {reuse})` before `prepareSelectedRustBuildGraphV1(...resolved.args...)`; only after the complete baseline is loaded does line 142 pin it and line 145 dispatch Nx. Thus a selected target does not bound initial graph discovery. `loadBuildGraphV1` has no selected target/owner-manifest parameter; only root, invocation, reuse. Reuse needs original absolute immutable graph pin and exact digest.

Graph discovery lines 196–222 assembles all project JSON/Cargo/feature/script-policy contributions, maps the full file roster to owners, computes all dependency edges and complete module metadata, and validates one global `BuildGraphV1`. Policy's `catalogProviders` includes S source `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts`. Catalog loading is explicitly provider-driven, but the caller supplies the whole workspace policy rather than the selected target's closure.

Inference `emojiProjectJsonNodes` first expands every physical Cargo candidate, collects the playground catalog, then parses every project JSON (including a global staging-project prefilter). Malformed independent admitted project JSON or Cargo is capable of aborting this General baseline before selection. Deletion before inventory removes that candidate, but surviving referenced manifests/catalog producers can still fail; deletion during discovery produces membership retries. No physical deletion experiment or tests were run here.

This is graph-source scope coupling, not a direct General target dependency requirement. A complete workspace graph is deliberately defined globally, but requiring that global artifact for a single independent source target is an architectural choice. Existing selected reuse proof does not establish clean-start deletion resilience.

## One Extraction Seam

Extract selected owner assembly admission at the current `discoverBuildGraphV1` -> `discoverBuildProjectsV1(files, options, context)` boundary. Incoming portable declaration should state selected task identities and explicit owner manifest roster plus selected catalog providers, then compute dependency owner closure before admitting source-specific assembly. Outgoing remains existing owned `BuildGraphV1` (`BuildProjectV1`, `BuildTargetV1`, execution, dependencies, module metadata); Nx receives the same immutable publication rather than foreign executor/type APIs. This requires a schema-first admission contract alongside `🏗️build/🧬️schema/🔣️.json`, not an exception swallowing unrelated parse failures. `selectedCargoBuildOwnerV1` is already a narrow canonical manifest recognizer, but operates after full baseline discovery and only Cargo build targets; it cannot solve this source target admission.

Existing portable mount is `🏗️build/🧪️tests/🕸️graph/🟦️.ts` and fixture `🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`: it compares graph fixture to independent JSON Schema (line 40), common graph to independent Nx task graph (line 105), and immutable pins (line 174 onward). Extend there with declared selected owner closure and unrelated invalid/deleted owner fixtures; do not assert passing before running.

## Foreign Graph Writer Lifetime

Resource checked exactly: `build-graph-state:/Users/ueli/Documents/semio/.cache/b/g/m`, SHA-named SQLite under `.cache/b/l/m/data`. Its queue contained PID 52024 and PID 53675 exclusive claims. Targeted `lsof -t <exact database>` returned only 52024. Targeted `ps -p 52024,53675` showed 52024 running foreign ticket `2026/10/14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/qualify/📜️script.ts` (~7m47 elapsed), and 53675 running actual General Actor canonical target (~5m26 elapsed). No signals, leases, processes or foreign files were modified. Lsof shows an open descriptor, not sufficient alone to prove SQLite transaction lock mode; queue order plus exact owned protocol identifies the suspected holder/waiter relationship.

`withBuildGraphStateV1` acquires exclusive state lease around entire body. `loadBuildGraphV1` passes all up-to-nine attempts as that body: writer remains held across inventory, inference, validation, retries and canonical publication. `finally` releases it when body ends. Fact-store scopes are callback-live and reuse the original store through AsyncLocalStorage. Cache store charging lease is released before the callback, while mounted image custody closes after callback. Immutable snapshot writer is separately held only through creation/retention and reader acquisition, released before backend body; shared snapshot reader remains throughout backend consumer. These are distinct lifetimes.

## Exact Source SHA-256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`: `e318ed29c795370fd2536bec8699da2975d8d5cb32dcf8522ac43c7ffe8b431b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`: `c57f4b584841d2dfb18a85cb0c55d7a2d1c092ac941f1802ec38790749b63769`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`: `dbd8e5515fb1fc67ea1c7e0ed21be0a21bd098bebe87721ed16081f5bb6fd6d1`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`: `096ddd933a62de43d051049624c41d64c13d742a83d6631d62bb267e7ed24e57`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts`: `0335bc3dd2229a2e6c39d5361fce927e75571d638e59238f17624ad6e2d3c2df`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs`: `8f741183a16bcdac4cabe164dadc9030407d4bd49fc1f6713f5dfebf89c934e6`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts`: `1d22caa29d6ad501c2d0e01e44ec8e5336af121975d783c76d79bcf8a6e1a14a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts`: `141a99223f78033a96937263288e6c0a83d7b4b7940aa7d738b950050162a64f`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`: `c7db0c4214424f79f057c22568b4af298e9ffd6e5d382b6844ced59432baa15d`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json`: `c8b29ffe06a6748735fdcaa1e066bbf52c9a7b898d151742f02b6d9ef3c29a80`
