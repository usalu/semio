# Graph Selected Config Current Read-Only

Static audit, observed UTC: 2026-10-14T19:19:06.686586+00:00. No source edits, tests, runtime probes, frontend/native/initializer or resource actions. Concurrent source edits can supersede these exact hashes. This is not actual qualification.

## Provable Remaining Defects

- Inference line 1571 derives configuration only from an explicit matching selected task. It does not read target.defaultConfiguration. The backend's Bazel task resolver line 45 does read defaultConfiguration. Therefore admission's effective execution closure is not derived by the same default-selection rule as task execution. Original target normalization additionally examines all authored configuration recipes, so this can over-admit configuration source inputs; it does not establish an omission for existing scripts. Dependency task configuration defaults are equally ignored by the effective selected object.
- The same line silently sets overrides to undefined when an explicitly requested configuration does not exist. Admission neither refuses nor validates that missing configuration; it observes base execution. resolveBuildTargetV1 in graph lines 309-313 does throw for an absent configuration. Admission must establish that guard before source assembly; downstream refusal does not make the admission fallback exact.
- Script physical checking at inference 1577 uses recipe.cwd ?? owner.root; targetScriptSources line 542 uses options.cwd ?? '.'. For owner 'leaf', command 'bun ./📜️script.ts' with omitted cwd, the closure resolves workspace-root script while the manual check resolves leaf/📜️script.ts. If only workspace script exists this introduces a false missing-source refusal. This is a concrete disagreement between two source resolvers, independent of CLI behavior.

## Fixes Confirmed By Source Inspection

Immutable reuse at graph line 251 now compares canonical requested and admitted selection for every kind, including all. It rejects absent admission and wrong-scope selected pins after digest validation.

Admission's CLI parser recognizes --configuration value, -c value, --configuration=value, -c=value and positional p:t:configuration before --; conflicting values throw. Root mode parsing recognizes the same aliases and preserves original args. No executable Nx installation was found at node_modules/nx/src/tasks-runner/create-task-graph.js, so installed Nx parser/default behavior was not independently verified here.

Dependency source input groups now have a separate inputs map, inspect parameter, and convergence revision. ^group requests propagate to every discovered source dependency independently of scheduled targets. On subsequent inspect the requested group recursively expands named shorthand; nested ^group then propagates onward. Unknown named groups and named cycles throw. Explicit tasks absent from owner declarations throw; optional ^target tasks absent from the dependency owner are removed before active inspect, so this mechanism does not create absent executable targets.

Scope keys use full SHA-256 over canonical task selection plus validated registry. Mutable selected graph and authority paths use the full key; writer lease includes the same full scope. Shared store.root/control remains physical quota/control. Selected inference dataset isolation was not fully traced in this limited pass.

Membership before/after checks, current registry scope comparison, source facts rechecks, final admission equality, and graph module/schema/policy observations are present. Inactive raw target declarations are still restored at inference 1704. This pass did not establish a complete backend execution guard forbidding those declarations; no execution acceptance claim is made.

Earlier tentative allegation that configured command scripts were absent from admitted files is withdrawn: targetScriptSources(selected) adds its returned closure through addFile at 1575-1576. Provider-specific import closure and topology qualification remain outside this static pass.

## Exact SHA-256

- `📋️build-owners.json`: `b54738b8cf2b28138dc1f75a24c15eb8f59b6b85e49b6aea5a0a50781783af0a`
- `📜️script.ts`: `67021eebc98c2c60b43a0a23e9425d11a57713e4a554c0a43d7576fa35a0e5fe`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`: `9e39f347157c262430f5ff47409905a2e12de96b61f5677268e1afcf7f505fd6`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts`: `8ebf1192aa019a6f7105d1616fe8c684d28278053e725b5823811b30c2522d26`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`: `c8d1edf946e4ecc77d119d7cdf9503d31a5309ea8e25ff3da880336e37f356fe`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts`: `007a109b52df50baba8e779a6272acefce4c19ab87e6a2338ebbacc577bb3ebe`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`: `2dc1052d087c8cc97073ce12224654c4391aaaf86d4f67f225d69ed23f5ba553`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️selection.json`: `ad85c0988f8d12119bd28fa3b750708ec9f2c61f97bb3f5798dcf41f072fabe6`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json`: `97811ccef62f138dba6dfcbdca069e22f2fef85ce2a854a4db9d15b44a0c1173`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`: `01185478d8a48ebc564eebf616f82b8809815ffdf921f4ebc5bdef115dc0d986`
