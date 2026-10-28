# Independent Exchange Source Boundary Audit

## Result

Limited read-only source review found no confirmed new defect in the private IFC admission and cooperative index boundary. This does not close the separate native execution gap or remaining semantic traversal/control work.

Reviewed exchange-source-plan.md; exchange-source.ts; IfcGeometryGraph.capture and its triangle caller; IFC-to-BIM and IFC-to-FEM caller entry points; central captureIfcGraphSteps/copyIfcOwnedSteps/validateIfcGraphSteps/runIfcEditAsync; control.ts; and exchange-source feature, schema, fixture, cases, and authored tests. No production source was edited and no tests were independently executed during this checkpoint. Concurrent root changes may supersede inspected line numbers.

## Source findings

- Each of the three public source endpoints captures the operation controls before callbacks, awaits IfcGeometryGraph.capture, returns the admission refusal without a projection when no graph exists, and rebinds its local snapshot to graph.snapshot before its first semantic callback. Caller graph population/header replacement at semantic callbacks cannot reach the captured graph through those local references.
- captureIfcExchangeGraph uses the shared generator-based owned admission, forwards original signal and configured entity/value/depth limits, captures the callback, prefixes capture phases, maps central cancellation to io.cancelled, and converts central IfcEditError admission refusals to exchange.graph. It deliberately does not swallow arbitrary caller callback exceptions.
- Central copying checks enumerable own data descriptors before copying, excludes input getters without invoking them, preserves plain owned containers, rejects cycles and excess typed admission, and validates the captured graph's schema, identities, value domains and dangling references. Identity validation precedes the new index, preventing duplicate Map overwrites on this route.
- IfcGeometryGraph.capture builds the Map only after private admission. It checkpoints before each bounded row batch and after the final row, including cancellation before publication. The normalized positive batch size is bounded to 4096. No caller callback receives the private graph during index construction.
- Cooperative source capture intentionally can observe caller mutations before capture completes. The plan and central contract state the caller must preserve source stability during capture for exact initial-state semantics. This is not a snapshot-at-entry guarantee and was not treated as a defect.

## Tests and scope limits

The neutral cases name all three endpoints, finite limits, capture cancellation and index cancellation (indexCancelAfter=8). Tests cover semantic callback mutation, duplicate identity refusal, descriptor getter noninvocation, three finite admission limits, capture cancellation and index cancellation with unchanged input and absent semantic work. AJV checks the closed case-document shape. This reviewer read those tests but does not claim their execution or independent semantic reproduction.

The schema's enum-based endpoint/phase object validation does not uniquely pair each endpoint with its intended phase; current data pairs them correctly. This is a test-schema strengthening opportunity, not a confirmed production defect.

Remaining synchronous filters, metadata work and nested semantic traversals after admission are not made bounded merely by this new index. The plan explicitly leaves those traversal changes open. This checkpoint establishes source isolation and index cancellation by inspection, not complete maximum-work or heap guarantees. MaxValues counts graph values rather than limiting string byte size; no stronger physical-memory bound is asserted here.

Native original-wallet, five-axis receipt, retained-prefix cancellation, controlled retirement and genuine Khronos/three.js/IfcOpenShell native-output evidence remain separate. The authentic compiler ingress refusal and unexecuted native tests in native-gltf-owned-independent-audit.md still apply.
