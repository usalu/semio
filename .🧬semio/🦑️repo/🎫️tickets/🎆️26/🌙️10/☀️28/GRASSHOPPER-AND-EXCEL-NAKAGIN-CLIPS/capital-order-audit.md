# Capital Dependency Fallback Audit

Read-only reasoning and saved-graph review; no duplicate heavy graph/ray process or production change. Root owns the dependency-order implementation and portable NetworkX cases.

## Confirmed Existing Defect

Independently read capital-order-graph.json:2602source Items,1190 facing,16215 cleaned edges,12 residual SCCs containing156 members and281 fallback choices. Its recorded source endpoints are unchanged. Of1148 violated edges,1061 connect different initial SCCs, including104 steel_truss→capital and108 steel_truss→capital_seam constraints. These are not all unavoidable choices inside genuine cycles. A depth-minimum fallback over every remaining node can paint a blocked acyclic downstream wall before the upstream frame that must precede it.

## Correct Invariant

Ordinary zero-incoming Kahn choices remain valid. When blocked, select a component with zero incoming edges from other remaining components, then make a deterministic depth/index choice within that source SCC. Removing its chosen node must update both node and component incoming counts consistently. This prevents arbitrary jumps across acyclic condensation edges. Component counts and decrements must use the same granularity: raw cross-edges or carefully maintained distinct-component dependencies.

A fixed initial SCC partition needs a stated limit. Removing a cycle vertex can split that SCC into an upstream residual cycle and downstream acyclic nodes; treating all as one old component can still choose the downstream node first. Portable witness: A↔B,B→C,C↔D,C→E,E→A with depthsA0,E1,C2,D3,B4. After breakingA then processing readyB, residual C↔D points toE. Selecting E from the old all-node SCC violates C→E; recomputing residual source SCCs avoids it. This witness was sent directly to root before implementation.

Generic graph cases should also protect singleton/empty/background inputs, self-loop handling if accepted, multiple external edges between the same components, deterministic ties and local/global Item-index mapping. Production geometry currently has distinct-node pair edges, but the helper's accepted domain should be explicit.

## Remaining Boundary

Preserving every inter-SCC dependency resolves this class of algorithmic error. It does not establish a physically correct global painter order inside actual intersecting/cyclic source geometry. Those cycles still need source polygon/ray diagnosis or justified splitting; no centroid/tolerance/role-filter waiver follows. Fresh capital pixel/source-ID evidence must bind the saved implementation before final acceptance.

## Saved Dependency Helper — Independent Source Review

The saved _dependency_order implements the required residual-component invariant. Each task constructs its current induced adjacency without mutating the original sets, obtains SCCs plus singleton components, builds a deduplicated condensation DAG and counts/decrements incoming component edges at matching granularity. Heap priorities are deterministic(depth,index) minima per component. Reversed task-stack pushes preserve that topological component order.

When a task is one genuine SCC, only its minimum(depth,index) pivot is emitted; its remaining nodes are pushed as a new local task. Local SCCs are therefore recomputed after removal, resolving the fixed-initial-label counterexample. Singleton/self-loop tasks terminate, empty inputs terminate and no recursion depth is required. Caller mapping from local visible indices to original Items remains explicit. Root's nine portable cases/independent NetworkX source-residual-SCC test are recorded as green; this reread did not duplicate a heavy probe.

The tie guarantee is deterministic component priority, not globally smallest depth among every ready node after partial SCC processing. Processing one independent component before another is still dependency-safe but can differ from the previous global greedy sequence; exact raster/physical-ray gates remain necessary. The algorithm also does not promise linear overall time: a persistent large genuine SCC can require repeated local induced-graph/SCC work after each pivot. It avoids recomputing unaffected global components and Python recursion, while retaining graph correctness. Current capital evidence is needed to measure actual cost.

No additional concrete graph-correctness miss was found in this saved source review. The repaired guarantee is every step belongs to a source SCC of the remaining graph; it is not an assertion that breaking an unavoidable physical cycle by depth produces the true nearest-surface painter result. Fresh capital pixel/source-ID/ray evidence remains required.
