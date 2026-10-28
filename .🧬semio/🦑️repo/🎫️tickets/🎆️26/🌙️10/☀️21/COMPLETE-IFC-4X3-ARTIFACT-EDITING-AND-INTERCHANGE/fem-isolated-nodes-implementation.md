# FEM Isolated Node Export Correction

The actual TypeScript `fem3dToIfc4x3` exporter now refuses zero-member analyses and unused analytical nodes before creating an IFC graph. It does not omit authored nodes, supports or loads, and does not manufacture members to satisfy the IFC inverse. Every declared node must be an endpoint of at least one exported member. Multiple disconnected groups of members remain supported.

## Contract And Production Change

The closed `🚪️io/🧪️tests/fem-isolated.schema.json` and ten neutral `fem-isolated-cases.json` vectors preceded the production edit. The initial suite reproduced **6 passes / 9 failures**: empty analyses, a node with no members and unused nodes with/without support/load incorrectly returned artifacts; the four topology cancellation prefixes had no original preflight to interrupt.

`🚪️io/fem.ts` now checks member presence after original control capture and the initial cancellation checkpoint. Zero members return `fem.empty`. It then builds an endpoint membership set one member at a time and checks every declared node one at a time. A node without an incident member returns `fem.isolated-node`. Both scans report actual work under `fem-topology`, respect the captured batch size, and invoke the same captured original signal/callback. Completion is checked before `createIfc4x3ExchangeGraph`; all new refusal and cancellation paths therefore return no partial artifact.

Existing missing-node and zero-length-member checks remain authoritative for those distinct failures. The new connectivity scan does not require the whole analysis to form one connected component. Ordinary material/section/support/load construction and successful export fidelity remain unchanged.

Refusing a completely empty analysis is an explicit supported-projection policy. The independent oracle confirms that an IFC container with no structural items can itself be schema/EXPRESS-valid. The defect fixed here is specifically a declared structural connection with zero incident members; empty FEM analyses are now explicitly unsupported rather than presented as meaningful analytical exports.

## Tests And Independent Comparison

The registered `fem-isolated.test.ts` executes all ten neutral cases, preserves source input equality, checks no artifact on refusal, and roundtrips the three admitted models: one frame with support/load, a shared middle node, and two disconnected members. Four further tests cancel at every actual preflight callback of the small connected case. They replace the caller-owned control signal/callback before aborting the original signal and confirm original cancellation, zero replacement callback use, no returned artifact and unchanged input.

Focused `fem-isolated.test.ts`, existing `fem.test.ts` and `exchange-control.test.ts` pass **50 tests / 208 assertions**, with one pre-existing genuine-output-gated skip and zero failures. After the root worker corrected the separate source fixture, a focused `fem-isolated.test.ts` plus `exchange-source.test.ts` run passes **31 tests / 138 assertions**, zero failures. The new suite is explicitly registered in the IFC TypeScript package router.

Ticket input `fem-isolated-export.ts` exported actual mapper outcomes before and after the production change. The baseline emitted eight artifacts from the ten cases; the corrected mapper emits only the three admitted connected cases. `fem-isolated-oracle.py` uses the existing IfcOpenShell 0.8.3 interface and bundled Python 3.12:

- Its independent IFC4X3_ADD2 declaration reports a minimum of **one** `ConnectsStructuralMembers` inverse relationship on IfcStructuralPointConnection.
- All three baseline unused-node/support/load artifacts produce the expected inverse diagnostics. The zero-member node artifact also has that schema error.
- All three corrected actual output graphs pass complete `express_rules=True` validation with **zero diagnostics**. Every structural point has at least one independent inverse relationship, and actual node/member counts match their neutral inputs.
- The oracle reserializes all three genuine outputs. `fem-isolated-verify.ts` reparses those physical STEP files through the actual parser and importer and matches topology, engineering values, support restraints and nodal loads against the corresponding original FEM inputs: **3/3 supported semantic matches**. Destination identities are intentionally canonicalized. Zero roll and negative-zero roll represent the same orientation and are normalized in this semantic comparison; no bit-preserving angular-roundtrip claim is made.

Generated STEP, before/after mapper outcomes and detailed independent diagnostics are under `🗑️generated/fem-isolated`. The reusable exporter, oracle and reciprocal verifier remain at ticket root. These are TypeScript/IfcOpenShell results, not native Rust execution.

## Verification Limits And Changed Files

Focused strict TypeScript checking reports only the already-known missing `@semio-tech/framework-interaction` transitive shared module. No missing-module stub, compiler bypass, relaxed source check or build repair was introduced.

Production change: `🚪️io/fem.ts`. New permanent inputs/tests: `🚪️io/🧪️tests/fem-isolated-{cases.json,schema.json,fixtures.ts,test.ts}`. Registration: IFC `📦️packages/🟦️typescript/📜️script.ts`. Retained ticket inputs/reports: `fem-isolated-export.ts`, `fem-isolated-oracle.py`, `fem-isolated-verify.ts`, `fem-isolated-nodes-plan.md` and this report.

The root worker owns the independent `exchange-source-fixtures.ts` correction; this worker did not edit it. Native FEM paths, retained EXPRESS engine/audit files, template admission gates, build enforcement, AGENTS and Git state were untouched by this follow-on task. Original TypeScript exporter capture/heap frontiers outside the new topology scan remain their existing broader boundaries; no native ownership or complete physical grant-law claim is added. Shared generated dependencies remain available to the fleet. No ticket or goal lifecycle change was made.
