# Current Boundary Review — 2026-10-14

Read-only targeted current-source inspection. Root and applicable Products, Repo, and OS AGENTS.md were read; no production changes, graph generation, native build, Git mutation, or full-suite test was performed. This report does not claim runtime verification or completed goal acceptance.

## Remaining Concrete Cross-Plugin Bypass

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs:206` skips every destination `🌊️flow` in `crossPluginRules`. The exemption is unconditional across source families and is justified only by the comment “media-graph canvas embed.” Real production dependencies use it:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🟦️.ts:1` imports `componentVertexIds` and `parsePolygonMesh` from Flow BREP mesh implementation.
- The same artifact’s `✏️editor/🎮️commands/🎚️set-widget-input/🟦️.ts:4` and `🚪️io/🟦️.ts:2` import that same implementation.

Next full-scope refactor: give the polygon mesh schema/parser a neutral geometric contract owner, move its language implementations and independent conformance fixtures together, update both Flow and Procedural consumers, and remove the destination exemption. Cover every ordered distinct plugin pair with a language-neutral rule matrix. Merely removing the exception before relocating production consumers will expose real source errors.

## Generic OS Still Contains Concrete Catalog Composition

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts:43` hardcodes Space host metadata; lines 47 onward enumerate concrete Hub composition package identities, Cargo paths, capabilities, activation events, and execution modes. `🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx:11` imports this concrete generated catalog. This is data coupling rather than a resolved TypeScript import into Hub, so source-direction graphs alone cannot prove OCP or neutral ownership here.

Next full-scope refactor: move concrete catalog outputs and their inputs to an application composition owner, inject catalog contracts into generic OS boot/hosting and stories, and have application composition own concrete matrix coverage. Keep generic parsing/deployment contracts in OS. Coordinate with the active registry-composition agent; do not duplicate its current writes.

## Actual OS Test Direction Edges Remain

Current source still contains generic OS tests depending on concrete application fixtures:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts:8-9` imports Stdio TXT SQLite implementation and schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:7-10` imports Draw action/interruption fixtures and schemas.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx:519-521` imports Procedural inspector fixtures.

These are current observations, not inferred acceptance failures from an unrun graph. The parent owns service-test relocation. The next broader cut must retain product-specific integration assertions at composition owners and neutral renderer/IO laws at their mechanism owners, without exempting all tests from direction rules.

## Exact Graph Authority Verifier Scope

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts:116-133` now rejects unknown top-level rule fields and unknown selector fields, compares the used forbidden rules to authored rules, and accepts only error rules with path/pathNot selectors and optional module scope. It reconstructs edge verdicts independently and compares them to summary violations at lines 181 onward. Target export authority and inventory completeness are also checked.

No definite new correctness flaw was established in this bounded source inspection. However, this API intentionally does not admit `dependencyTypes`, absent path selectors, or warning rules. Current constructor rules outside this strict subset include cross-plugin local-only rules, no-escaping-relative-imports, and no-impl-segment. Consequently, successful verification of selected strict framework rules must not be described as verification of the entire constructor policy. To extend acceptance to all source-edge directions, introduce closed schema-first selector semantics for those remaining rules and implement corresponding independent verdict checks and hostile oracle tests. Keep unsupported authority rejection; do not quietly drop unsupported rules.

## Acceptance Limits and Next Ownership Work

No full goal census or full dependency graph was run here. Existing fresh-component narrowing and GIS Cargo receiver API work remains exclusively with the parent. Jack interface and registry composition remain with their assigned agents. Full acceptance still requires the final actual-source graph, concrete catalog ownership acceptance, all language-neutral and independent oracle lanes, runtime evidence where applicable, and the ticket-wide goal requirement inventory.
