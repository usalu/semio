# Shell Typegen Admission Diagnosis

Root reports selected architecture session9854 terminal1 before test collection. Actual log tail independently matches the error at owner-discovery213/221: shell:typegen, importer framework/modules/shell/TypeScript reducer, generated shell mirror, phase compile. This is infrastructure precollection refusal, not an actual CI contract RED. No command or Source edit was executed by this review.

## Exact Role Error

Owner manifest `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json` declares typegen with inputs:[schemaModule], output framework shell generated directory, and no metadata.semio.sourceInputObservation. schemaModule includes Rust schema documents, schema TypeScript face, Cargo manifest and the package script. Existing default admission role is compile.

`🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs:175,204` resolves each selected target role;204 sets scan.compile true by default.209–215 promotes every matched TS file and its transitive imports to compilation, including script command Sources that otherwise receive runtime demand.217 then uses facts.imports for compile demand. At221/224, generated output imports call generated(), whose self-output refusal213 is correct for an actual compiler input.

The importer `🧰️framework/🔨️modules/🐚️shell/🟦️.ts:20–21` uses export type* and import type from its generated mirror; runtime projection erases these imports. Its TypeScript mirror is needed for a consumer compile, not to run the Rust exporter. The shell schema TypeScript face also consumes the generated mirror (`🧬️schema/🟦️.ts:16,21`).

The real typegen implementation at shell `🧪️tests/🔬️rust-package-commands/🟦️.ts:25–26,64–70` uses the admitted Cargo owner to execute the Rust exporter test with feature typegen and SEMIO_TYPEGEN_OUT. It does not compile the schema TypeScript face. Source content is still freshness/input authority; the script runtime import closure must remain admitted.

## Existing Clean Declaration Mechanism

The project schema already admits metadata.semio.sourceInputObservation=compile|freshness at `🏗️build/🧬️schema/📋️project/🔣️.json:216–221`. Admission sourceInputObservation/sourceDemand policy already distinguishes matched freshness data from declared compilation entry and runtime command. The analogous actor:typegen target already sets freshness, covered by the existing language-neutral fixture. No schema/API/guard change is required for this same authored data-producer role.

Smallest repair: add freshness metadata to shell:typegen; add its manifest/target expected freshness row to existing `🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json.dataProducerInputRoles`, and a neighboring test row asserting shell:test stays compile. The existing `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts:22+` authored-producer test reads the actual manifest and compares the own role with Ajv schema and independent SQLite coalesce oracle. Adding the expected shell row first creates a focused declaration RED against the current manifest. Existing traversal vectors runtime-type-erasure accepted, typed-self-output refused, runtime-self-output refused and runtime-to-compiler-upgrade refused retain the actual selfcycle guard and prove that compile authority is not weakened. Native preparation explicitly excludes generator target identities at native-manifests202, so no authored shell:typegen Rust prerequisite needs to consume its output.

## Remaining Structural Question

owner-discovery213 assigns each discovered generated dependency to every selected target in the current owner, while compileModules and demand maps are unions across those targets. If the same admission simultaneously selects shell:typegen and a real TS consumer target, one consumer demand could be attached to the generator itself even after its own matched inputs are freshness. That is a separate per-target provenance gap; the current log does not print the selected target set and cannot establish it is active here. Do not suppress selfcycle checks or weaken compile imports to hide it. First exercise the narrow authored-role correction with existing guard regressions; if the actual selected run still fails, instrument or extend controlled neutral admission tests to retain demand by target.


## Prepared Source Repair Checkpoint

Root authorized schema-first neutral regressions and Source repair. Exact initial9854 argv is run @teaching/architecture-course:test --configuration=release -- quick from usable host prototype, with no task-specific environment. The expanded targetset is not runtime-observed; this change does not assert one.

Authored shell:typegen now explicitly marks matched inputs freshness; shell:test remains compile. Schema sourceDemand now declares scope target and a closed neutral taskSourceDemand entry. Fixtures were changed before implementation: six traversal cases retain existing erasure/realcycle/revisit behavior and add mixed generator+consumer behavior.

Owner inference now retains command cache compiled names per selected target; shared physical scans additionally return compileByTarget; declared compilation entries retain requesting target; traversal demand/visited maps are scoped by target. generated() adds prerequisite only to that requesting target and still rejects a producer consuming its own generated output in runtime or compile phase. The physical input walk remains shared and all matched files stay freshness inputs. Input-only declared contracts can retain owner Source closure without inventing a selected task.

Existing extracted inference test ports were updated to the scoped map contract. Neutral tests compare scoped matches with picomatch; runtime edges with esbuild; compilation import facts with TypeScript; recursive task/Source closure with SQLite; closed demand entry with Ajv. Actual producer declaration roles keep existing Ajv+SQLite oracle. Added separate command-closure fixture tests the actual per-target capture code. All tests remain unexecuted in this worker because Root owns native command execution; no RED/GREEN or architecture collection is claimed.

Changedfiles:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json — 3889bytes, SHA256 6d8a92a93943d3d1fa75483b70abd6baf7fbcd95342dd20e7fda611d05ec3c07
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs — 32059bytes, SHA256 751432fa2b7b5cae85021a326e5bd675ff3e037c2004bd40ea2892317d16f7b7
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json — 7182bytes, SHA256 e21558fc721ddb9a6587d0f485f602ecd5f6a8a23884875c52f41031100c802c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 125160bytes, SHA256 47173d9346ada28d2b9985f5b3c4e618e4911aeab04bfe8b4b15694c05c12ee9
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 67571bytes, SHA256 73478ba256759762ef482c40d8a01d2b0fc2d466e57b151e5adda97fe9028360

Root should run focused source-demand/admission regression through the native owner, inspect any failure before further changes, then retry selected architecture quick to collect the actual CI assertions. No selfcycle guard was removed.


## Necessary Resource-Scope Review Correction

Independent review found a remaining mixed-task authority leak in the compile root filter: the prior scoped implementation still read global scripts.resources/scripts.modules. Root authorized this correction before launching native regression. Added three neutral mixed-task resource/module cases before Source change: peer module cannot promote own resource; peer resource cannot suppress own matched compiler source; declared compile entry overrides own resource.

Each target now captures commandResources as the exact actual closure minus compiledNames for that same closure, using identical canonical workspace path conversion. The provider relativeScriptInputs keeps a per-computation compiled set and stores canonical names in compiledNames keyed by the returned closure array (script-inputs167–168); memo reuse retains that identity. JSON, Cargo, schema and URL data remain closure-only; a path seen as both module/resource within one target remains a module because compiledNames wins. The compile filter uses only target-local resources and modules; input-only roots get empty command evidence and retain declared-entry priority. No global resource/module filter remains.

Command-scope fixture additionally includes JSON/Cargo resources, an overlapping same-target module/resource path, WeakMap exact-array identity and repeated warm capture; existing declared-entry/resource roots test and independent SQL oracle remain. All tests still unexecuted by this worker. Updated freeze roster:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json — 3889bytes SHA256 6d8a92a93943d3d1fa75483b70abd6baf7fbcd95342dd20e7fda611d05ec3c07
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs — 32317bytes SHA256 88976061930a33159e1ca3b67e4de79e58b88db5bbbee25744ce9a134e115270
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json — 7182bytes SHA256 e21558fc721ddb9a6587d0f485f602ecd5f6a8a23884875c52f41031100c802c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 127165bytes SHA256 03c6154949cde2541c7a896ef610c4678a4f4fe88869f4ef2bfd01d7641f3383
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 70589bytes SHA256 07941063a7e6cd966d28d4692146404c7e8c90a30894d8fbe66aef6cae50b5d7


## Final Native Regression Freeze

Release cleared production semantics and identified one Windows-only test assertion: native joined paths cannot be compared to literal /fixture/. The assertion now uses join(workspaceRoot,"/") as its prefix. Root authorized exactly one native workspace:test-workspace-build --configuration=release --skip-cache run without forwarded arguments. Final Source roster:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json — 3889bytes SHA256 6d8a92a93943d3d1fa75483b70abd6baf7fbcd95342dd20e7fda611d05ec3c07
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs — 32317bytes SHA256 88976061930a33159e1ca3b67e4de79e58b88db5bbbee25744ce9a134e115270
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json — 7182bytes SHA256 e21558fc721ddb9a6587d0f485f602ecd5f6a8a23884875c52f41031100c802c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 127165bytes SHA256 03c6154949cde2541c7a896ef610c4678a4f4fe88869f4ef2bfd01d7641f3383
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 70595bytes SHA256 0ed6dd7b94e6dc23ecd336301c1b65380d0071d686993ecfa0b0b5886e4d685c
