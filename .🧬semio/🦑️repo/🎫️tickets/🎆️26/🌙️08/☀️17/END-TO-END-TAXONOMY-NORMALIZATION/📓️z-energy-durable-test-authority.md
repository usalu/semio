# Durable Energy Test Authority

## Read-only Finding

The current complete-owner test enumerates the live Energy artifact, copies its bytes and the live Cargo/glue inputs into an isolated fixture, then asserts the reviewed source-tree digest and 195 planned moves. After production normalization, the same live inputs are canonical: those source assertions will fail even though the repository is correct. Updating only the digest would not repair the test's meaning. Automatically retaining a generated copy of the old tree would convert tool output into supposed authored authority, contrary to the current retention instruction.

This audit read the current test and proposes no edit to it. The complete-owner rollback/retry/runtime run remains valuable pre-apply acceptance evidence; its 195-file count is a reviewed temporal fact, not a permanent source-layout contract.

## Recommended Separation

1. **Authored normalization scenarios.** Keep a small, pattern-complete language-neutral input contract containing explicitly authored source nodes, canonical destinations, contents, modes, and expected references. Cover the independent format kinds, mutation/profile placement, payload descriptor relation, assertion messages, immutable Cargo-based joins, loop surfaces, and unsupported ownership cases. Use synthetic fixture owners, not live Energy source locators. Handcraft the expected canonical paths and bytes; do not derive the oracle from the normalizer or serialize the live tree into a new golden.
2. **Canonical Energy acceptance.** Once the production move is committed, change the live-owner test to one unambiguous canonical contract: all admitted leaves already have their required canonical paths, inventory has no ownership violations, the plan is empty, actual descriptor pointers and Rust module mounts resolve, and runtime readers/assertions agree. There must be no source-versus-canonical fallback. Added production files are checked by the same semantic ownership rules; an old fixed count must not silently become the permanent definition of Energy.
3. **Temporal migration evidence.** Preserve the authored Markdown record of the completed 195-file pre-apply proof, exact reviewed digests, rollback/retry result, and runtime observations. Dispose of completed generated plans, compiler output, and logs according to their actual ownership and recovery status. Do not make that evidence an ongoing executable test input.

## Schema-first Input Shape

The authored scenario contract should have a closed schema version and explicit scenario identity, source nodes, expected canonical nodes, expected structured references, and expected rejection code. Source nodes belong to a simulated fixture coordinate system. Runtime configuration inputs such as Cargo manifests stay authored inputs; the test materializes them under a unique ticket fixture. Expected canonical output is authored data in the contract, not a captured generated artifact. Independent `syn`/Cargo and JSON parsers should verify the same syntax and runtime observations already exercised by the bounded Rust and payload support tests.

This separation avoids a permanent copy of the old Energy layout, avoids a compatibility branch in production discovery, and keeps the full live monorepo test meaningful after normalization. The coordinator should perform the source-to-canonical test transition in the same reviewed production batch that changes Energy's physical tree.

## Reviewed Transition Boundary

The current permanent full-owner case was re-read at `🧪️tests/🧪️artifact-support-leaf-authority/🟦️.test.ts`. Its pre-normalization `ownerReadiness` fields are temporal assertions, not the future canonical contract. On the production boundary, remove those historical node/file/byte/digest expectations from the live test and vector; retain their observations in the Energy report. Keep the three handcrafted `sourceInputs` mappings and the independent small rollback/retry/generation fixture, since they describe deliberate synthetic normalization scenarios rather than a runtime fallback.

The canonical live-owner case must not reuse `lifecycleFixture()`: that helper deliberately seeds four old-layout leaves and an old-layout consumer, so using it would reintroduce a nonempty plan even when the copied real owner is canonical. Use a separate isolated fixture containing the current live owner, its actual Cargo manifest/glue mounting context, the unchanged schema, and its own empty Git baseline. This does not require a real-repository Git mutation. The context paths remain explicit language-neutral inputs, but current source hashes are not fixed as perpetual product semantics.

Require independent no-follow enumeration/fast-glob parity; every admitted file's `sourcePath === normalizedPath`; zero ownership violations; and zero moves, edits, regenerations, evidence removals, embedded relocations, and symlink retargets. Execute the existing Cargo/`serde_json` oracle against the canonical mutation aggregate and descriptor, and validate the three semantic support destinations through the existing authored JSON Schema/DSL/event expectations. Do not replace the historical 195 count with a new fixed count as the acceptance definition. Normal additions must be governed by the same canonical ownership and runtime checks.

No permanent test/vector was changed during this review. The real reference capture remains active, so no production move is authorized yet. The new canonical test becomes active only in the reviewed production batch; no source/canonical dual-path branch will be added.
