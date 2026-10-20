# Independent Generator Admission Review

Current status: initial cross-target resource attribution blocker is resolved in reviewed owner88976061…; final Windows assertion correction is verified in test0ed6dd7b…. No remaining static blocker found for the focused native regression. Actual regression and architecture collection remain unproved.

Read-only filesystem review of the five-file checkpoint in shell-typegen-admission.md. No source edit, command, extracted-code execution, test or private Source operation ran. Offline RED/producer patch, CI workflow and frozen helper remain unchanged.

## Initial Concrete Blocker — Resolved

owner-discovery:210 scopes compileByTarget and declaredCompileEntries by target, but still filters their module roots through global scripts.resources and scripts.modules. These sets belong to the shared createScriptInputCache and accumulate all selected command closures (script-inputs:30,131,133).

A concrete mixed-owner counterexample: selected generator A observes resource.ts only as URL/data while selected consumer B imports the same file as a real module. Both command closures were read before compile-entry filtering. scripts.resources.has(resource.ts) and scripts.modules.has(resource.ts) are therefore both true globally. If A has a matched compile input containing resource.ts, A promotes that data resource to compile via the global module observation from B. A type-only import of its own output then triggers a false selfcycle despite target-local module/resource roles. The inverse also leaks: B observing a path only as a resource can suppress A matched compilation input when A never imported that path at runtime. This is independent of the now-correct per-target demand queue.

The prepared sourceObservationRoots vectors vary resource/compiled flags for one target only; traversal mixed cases start from preconstructed compileModules, so neither exercises mixed-target resource classification. Request target-scoped resource observations alongside compiled names (actual closure minus actual compiledNames), use those in the compileModules filter, and add neutral mixed resource/module cases for both directions. Keep physical reads shared; do not weaken generated guards. Sent exact issue to Root and Runtime.

## Verified Static Invariants

- commandModules captures each target actual sources.closure compiledNames immediately after targetScriptSources; cached array identity is the provider WeakMap key. Fallback/undefined closure yields no TS runtime module. Freshness files still include all canonical closure entries.
- Actual entry admission remains in targetScriptSources: retain reads bound script physically; requiredSources=true closure reads physical Source; relativeScriptInputs rejects workspace escapes and keeps non-source resources distinct. New consumer only receives names produced by that existing validated provider. No new caller-controlled path ingress was found.
- compileByTarget records only compile scans; scope-specific exclusions remain independent. Shared filesystem walking and matched freshness union are retained.
- declared compilation entries retain requesting target and explicitly override resource-only observation. Requested input-only contracts retain the undefined map key rather than inventing a selected task. Generated() adds owner Source/dependency for that case, while attaching prerequisites only when a real target exists.
- demands and visited are keyed by target then absolute file. Runtime-to-compile upgrades are monotonic within that target and enqueue a revisit. Unrelated target compilation does not upgrade another target queue.
- generated() compares producer task to the actual requesting target before attaching a prerequisite. Runtime self-output and compile self-output guards are still present, including bounded diagnostic importer/output/phase. Real runtime-to-compile selfcycle remains refused.
- Shell typegen explicitly sets freshness and shell test retains default compile. This changes matched input role, not declared command execution authority or runtime imports.

## Neutral Test Assessment

Prepared actual-source extracted tests cover picomatch parity for scoped input matches; esbuild runtime type erasure; TypeScript import facts; Ajv closed task demand; SQLite recursive per-target generated closure, phase upgrade and command module roster. Mixed generator/typed-consumer cases correctly compare target-specific generatedDependencies, and real runtime/typed selfcycle vectors remain. These are substantive independent oracles rather than a duplicate traversal implementation. No pass is established until Root executes the native regression owner.

Coverage limits: mixed resource/module attribution above is missing; taskSourceDemand allows target:null but the new roots-schema test supplies only strings; commandScopes uses valid provider-derived fixture paths and does not itself exercise malformed entry rejection. The real provider entry fences were inspected separately. Neither missing null/entry negative vector is by itself a reason to weaken admission or postpone a corrected focused regression.

## Reviewed Snapshot

- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json — 3889B; SHA256 6d8a92a93943d3d1fa75483b70abd6baf7fbcd95342dd20e7fda611d05ec3c07
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs — 32059B; SHA256 751432fa2b7b5cae85021a326e5bd675ff3e037c2004bd40ea2892317d16f7b7
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json — 7182B; SHA256 e21558fc721ddb9a6587d0f485f602ecd5f6a8a23884875c52f41031100c802c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 125160B; SHA256 47173d9346ada28d2b9985f5b3c4e618e4911aeab04bfe8b4b15694c05c12ee9
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 67571B; SHA256 73478ba256759762ef482c40d8a01d2b0fc2d466e57b151e5adda97fe9028360

## Runtime Correction Proposal Review

Root authorized Runtime to correct the confirmed mixed-resource issue before native regression; no regression lane was launched at this checkpoint. Reviewed actual relativeScriptInputs:119–170 and targetScriptSources:188–206. A private compiled Set is created per closure computation; compiledNames stores canonical workspace-relative names keyed by the exact result array. The asynchronous closure memo returns that same array on a warm read, so its WeakMap association remains valid.

Deriving target resources from sources.closure minus that exact compiledNames set preserves local authority. JSON, Cargo, schema and URL data remain closure-only resources; real TypeScript module names enter compiledNames. When one target visits the same file as both a resource and a module, compiledNames wins, so it remains a source for that target. Other targets cannot promote or suppress it once global resource/module sets leave compile-entry filtering.

Both derived sets must use the same canonical workspace-name to absolute-host-path conversion; this keeps Windows separators and package module paths aligned. Provider entry fences already establish canonical in-workspace names. Input-only undefined target has no command resource observation, while declared compile entries explicitly remain authoritative. The physical file walk and read memo can remain shared. Requested neutral mixed promotion/suppression and cold/warm name replay coverage directly from Runtime. This is proposal review, not verification of a final changed source or a test pass.

## Final Corrected Checkpoint

Reviewed actual owner commandResources capture at183 and compilation-entry filter at210. Resources are exactly that target closure minus its compiledNames; module/resource conversion uses the same native join. The compile filter no longer reads global scripts.resources/modules. Both mixed leakage directions and explicit declared override are represented by three neutral cases with independent SQLite results. Actual command capture tests reuse exact-array WeakMap keys, repeat with warm fixture identity, cover JSON/Cargo resources and same-target module/resource overlap. They test the capture contract; they do not execute the provider memo implementation. The existing provider implementation was separately read.

New Windows assertion now uses path.startsWith(join(workspaceRoot,"/")) rather than a raw slash suffix. Read-only review found no further blocker for Root/Runtime to run the actual owned workspace:test-workspace-build regression without forwarded arguments. No test pass or collected architecture RED is claimed.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json — 3889B; SHA256 6d8a92a93943d3d1fa75483b70abd6baf7fbcd95342dd20e7fda611d05ec3c07
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs — 32317B; SHA256 88976061930a33159e1ca3b67e4de79e58b88db5bbbee25744ce9a134e115270
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json — 7182B; SHA256 e21558fc721ddb9a6587d0f485f602ecd5f6a8a23884875c52f41031100c802c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 127165B; SHA256 03c6154949cde2541c7a896ef610c4678a4f4fe88869f4ef2bfd01d7641f3383
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 70595B; SHA256 0ed6dd7b94e6dc23ecd336301c1b65380d0071d686993ecfa0b0b5886e4d685c

## Held Course Patch Consistency

Re-read native-ci.patch: recovery remains birth release mode and exact owned bun repo run repo-mcp:build --skip-cache without explicit configuration, native help must succeed, PATH propagation preserved. printf format contains exactly one backslash before n (bytes37,115,92,110). Offline staged-only patch remains unapplied; it preserves online template, env, certificates, tags and volumes. Actual workflow SHA d6d1e450… and bundle producer97952fc4… remain unchanged. All four frozen helper hashes remain exact. No implementation patch was applied before collected architecture RED.
