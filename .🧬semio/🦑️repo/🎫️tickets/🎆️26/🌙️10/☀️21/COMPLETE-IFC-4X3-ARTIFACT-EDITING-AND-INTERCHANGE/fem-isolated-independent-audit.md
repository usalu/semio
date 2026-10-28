# Independent FEM Isolated Node Export Audit

## Result

No new confirmed P1/P2 in the isolated-node topology preflight. Source review on 2026-10-21 covers fem.ts, the closed neutral cases/schema and fixture/test interface, plus fem-isolated-nodes-plan.md. No source edits or duplicate test execution were performed. Worker-reported TS/IfcOpenShell baseline and after-results are independent reference evidence, not native route execution.

## Source Evidence

fem3dToIfc4x3 captures original controls before its initial checkpoint. The new loops use the captured batchSize and fem-topology checkpoints; original signal/callback cannot be replaced by mutating the caller's control object in progress callbacks. Each loop checkpoints at its initial row and bounded batch intervals, and a final checkpoint runs before graph construction. Cancellation at those boundaries propagates io.cancelled without an artifact.

The preflight collects actual authored member start/end identifiers, then checks every declared node identifier for membership. It does not infer incident membership from supports, nodal loads, proximity or coordinates. Isolated supported/load-bearing nodes are refused instead of omitted or attached to invented members. The graph constructor appears strictly after preflight and its final checkpoint, so ordinary stable-input isolated-node refusal occurs before IFC output graph birth.

Zero members return fem.empty before the endpoint scan. This is the explicit projection policy described by the plan; the worker's independent oracle reports empty IFC analysis itself can be valid. The code does not conflate that policy with an EXPRESS prohibition. Multiple disconnected member components remain supported when all nodes are incident.

An endpoint Set is not sufficient to establish all FEM validity and the change does not claim it is. Missing endpoint nodes are later rejected by existing node-map resolution with fem.reference. A self-member passes identifier connectivity but is later rejected by existing zero-length handling. Existing duplicate identity checks cover nodes/elements/materials/sections/supports/load cases and per-case load IDs before publication. Those checks occur after graph birth, but no partial graph is returned on their refusal. The preflight's early-birth guarantee is specific to isolated-node/empty policy, not every malformed source.

## Tests And Limits

Neutral cases cover empty and zero-member input, ordinary/support/load isolated nodes, missing nodes, self-member, connected/shared-node and disconnected-member topology. Fixtures use actual declared identifiers without manufacturing missing endpoints. Tests assert unchanged caller input, unsupported/no-artifact diagnostics, successful central graph validation and import counts; four batchSize=1 cancellation cuts replace caller controls and abort the original signal, asserting original cancellation and no publication.

The worker reports 50 passing tests, one gated skip and 208 assertions across new/existing FEM/control suites; the independent IfcOpenShell interface validates three connected outputs with full EXPRESS and records the pre-change isolated-node inverse violation. This audit did not rerun or independently reproduce those execution results. Native compiler/type checking, authentic native receipts and native FEM output remain distinct unavailable proof.

External FEM model capture/admission remains open. The new preflight still reads the caller model across cooperative callbacks, and a callback could mutate it between preflight and reduction. The plan does not claim start-state/private FEM source semantics; this source boundary must not be presented as solved by the endpoint Set. Remaining eager material/section/identity/filter traversals are also outside this bounded topology change.
