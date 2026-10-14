# Graph Selected Closure Current Read-Only

Observed UTC: 2026-10-14T19:09:38.655181+00:00

Static audit only. No tests, runtime probes, native/frontend/initializer actions, source edits, formatters, foreign resource actions, or acceptance claims. Existing ticket and prior selected-general-graph-scope-readonly.md supplied by Root define audit scope. Exact hashes below describe source observed at report creation; files may continue changing concurrently.

## Provable Current Defects

1. Wrong-scope immutable reuse: graph/🟦️.ts line 251 compares graph.admission.selection only when requested selection.kind is tasks. A requested all selection returns an immutable selected graph if its path/digest are valid. Therefore selected pin reuse for graph/show/run-many is not refused. Exact digest guarantees content identity, not requested scope identity.

2. Captured versus effective configuration mismatch: admission/🟦️.ts buildInvocationGraphSelectionV1 only reads run project:target[:configuration], ignoring --configuration and -c forms. invocation/🟦️.ts resolveBuildModeV1 reads those options but returns selection.args unchanged; nxBuildArgumentsV1 forwards them. Thus run p:t --configuration prototype and run p:t --configuration release capture identical task selection despite different Nx configuration. Selected configuration-specific command/input closure is also not merged in discoverBuildOwnerAdmissionFactsV1: targets argument carries target names only and normalized.targets[name].inputs uses base target.

3. Dependency input closure is omitted: discoverBuildOwnerAdmissionFactsV1 expand skips every string beginning ^. Source dependency owners are admitted by closeBuildOwnerAdmissionV1 without any target; their inspect receives [] and the targets.length guarded named-input expansion does not run. Hence ^named/default source inputs of a dependency owner are not enumerated by this path unless independently reached through source imports or selected task prerequisites. This is a closure omission even when the downstream cache input declaration still names ^default.

## Other Observations And Limits

Scope hash is full SHA-256 of canonical sorted task selection plus validated entire registry. No truncation collision found. Registry equality is sensitive to declaration order, producing extra scopes rather than hash collisions. Canonical registry is rooted at Root 📋️build-owners.json. It declares S/Hub owners/providers, although selected admission does not inspect independent manifests just to resolve their names. This distinction does not establish a General registry free of S/Hub declarations.

Provider selection tests only admitted consumer owner roots and then admits provider source owner. Loop revisits newly admitted owners/targets until revisions stabilize. Provider source file imports are not separately scheduled by discoverBuildOwnerAdmissionFactsV1 unless also in that owner's matched source inputs or target script closure; this deserves a concrete provider fixture before asserting complete provider dependency closure.

Inactive targets are excluded during expensive project normalization, then restored raw at inference line 1699. They remain published as executable declarations after portable graph normalization. A different explicit run request is refused by selected reuse identity, but no full backend admission guard was established in this audit; automatic prerequisites created during normalization could address restored declarations. Do not claim inactive execution is forbidden based on normalization filtering alone.

Current compile walk includes observedSourceAnalysisV1(...).imports rather than productionImports, and MODULE_NOT_FOUND falls back to TypeScript Bundler resolver. Non-MODULE_NOT_FOUND require errors propagate; unresolved relative compiler imports throw. Named/base inputs are enumerated even cache:false; configuration and dependency expansion defects above remain. No source parse-error swallowing identified in the inspected compile walk.

Root builds captured selection from resolved.args and forwards the same resolved.args through nxBuildArgumentsV1; configuration option omission is the specific known disagreement. Selected mutable graph and inference scopes use the scope key; shared physical store/control remains. No store lifetime acceptance assertion is made here.

## Exact SHA-256

- `📋️build-owners.json`: `b54738b8cf2b28138dc1f75a24c15eb8f59b6b85e49b6aea5a0a50781783af0a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`: `e071cfbd1f52465fafe31b30b7f7a612c7fc853613c570936c53d3f87caee9e0`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts`: `52e283b41619fe955d53f6bb05cdb5159ff499a60d2d98747923d280bd6555f6`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`: `8918b55e7de3a9b3548fc11595cca98a3bd3a91ac30525c4a22732e245b7a1f2`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts`: `007a109b52df50baba8e779a6272acefce4c19ab87e6a2338ebbacc577bb3ebe`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`: `2dc1052d087c8cc97073ce12224654c4391aaaf86d4f67f225d69ed23f5ba553`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️selection.json`: `ad85c0988f8d12119bd28fa3b750708ec9f2c61f97bb3f5798dcf41f072fabe6`
