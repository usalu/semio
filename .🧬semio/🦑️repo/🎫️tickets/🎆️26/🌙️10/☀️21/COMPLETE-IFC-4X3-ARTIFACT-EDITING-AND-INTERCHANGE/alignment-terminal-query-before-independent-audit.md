# Business Terminal Qualified Query Before Audit

Fresh ticket-only probes use the actual exported validator, immutable released catalog, common query implementation and existing alignmentFixture. Production and the registered 38-case suite were not edited. `terminal-query-complex-before.ts` preserves complete declared ancestor slots using the existing complex fixture before appending the tested constituent. Its output is exclusive-created in `generated/terminal-query-complex-before.json`. The earlier simple-row-plus-constituent probe remains separately preserved in `terminal-query-before.json`; that probe changes the root from flattened to partial serialization and must not be used as evidence that inherited slots remain present.

## Actual before behavior

- Complete baseline complex graph: pass.
- Known IfcRelNests plus late unknown constituent: incomplete, mvd.nesting.unresolved. Its positive role survives, and the existing role.unknown flag already blocks success.
- Positive selected layout, segment and parameter plus late unknown constituent: incomplete under existing membership checks, with mvd.target.unknown, mvd.member.type and mvd.parameter.type respectively. These are not newly reproduced false passes.
- Extra unknown-only discovery entity: pass. `relations` skips it at `if(!role.matches[0])continue` before testing `role.unknown`. Conservative discovery cannot establish that such an opaque entity does not conceal another relevant nesting relation. Treat a change to incomplete as a supported-projection uncertainty policy, not proof that this internal malformed graph is a valid IFC population.
- Known IfcRelAggregates constituent with its own RelatingObject alongside IfcRelNests: incomplete/nesting unresolved and missing. The unqualified query sees two different declaring owners. `terminal-query-status-before.json` confirms ambiguous with both owners, while the exact IFCRELNESTS-qualified query is unique with the original owner pointer. This is conservative false incompleteness on the internal mixed graph, not an admitted physical multi-role example.

The same status probe proves a late unknown constituent yields query status unknown even when one correct match is retained, qualified or unqualified. Returning only `.matches` loses that fact. Existing membership happens to catch the tested late-unknown entity paths, but consumers must retain the shared query result as their own authority rather than rely on incidental preceding role checks.

## Exact released owners and migration tests

Immutable metadata identifies RelatingObject and RelatedObjects as IfcRelNests declarations, DesignParameters as IfcAlignmentSegment, horizontal SegmentLength as IfcAlignmentHorizontalSegment, vertical HorizontalLength as IfcAlignmentVerticalSegment and cant HorizontalLength as IfcAlignmentCantSegment. Owner-qualified consumers should distinguish unknown from missing/ambiguous and retain actual query pointers; query unknown must dominate its populated matches. No second attribute scan/graph implementation is needed. Remove the match-only helper after its sole remaining consumer migrates.

Freeze laws for relation discovery unknown-only, relation positive+lateunknown, selected layout/segment/parameter lateunknown, foreign declaring-owner conflict and duplicate same-owner constituent. The foreign known owner must not cause attribute ambiguity after qualification; duplicate actual owners still must. Preserve global incomplete-before-fail precedence, original cancellation/caller-failure identity and existing private source capture.

## Official predicate parity

The actual business-terminal predicate still passes all four original targets in place: H34 and V45 in sectioned-solid-horizontal.ifc, C61 in segmented-reference-curve.ifc and C2388 in linear-placement-of-signal.ifc. Exact original SHA256 values are recorded in both endpoint outputs and match previous pinned evidence. These are the business-terminal predicate results, distinct from selected direct counterpart C149 having no own Representation. No source assets were copied or changed and no fresh full EXPRESS validity claim was made.

All probe processes have exited, with no live Bun mappings. Native execution/type/receipt proof remains false. The new unknown diagnostic is a contract extension to be shared with the native terminal worker, not an independently executed native result.

## Independent declaring-owner corroboration

`terminal-owner-oracle.py` runs the existing IfcOpenShell 0.9.0.post1 interface and traverses each real ADD2 declaration's direct attributes plus its supertype chain. All six fields have exactly the expected sole declaring owner: IfcRelNests.RelatingObject and .RelatedObjects, IfcAlignmentSegment.DesignParameters, IfcAlignmentHorizontalSegment.SegmentLength, IfcAlignmentVerticalSegment.HorizontalLength, IfcAlignmentCantSegment.HorizontalLength. Output `generated/terminal-owner-oracle.json` records six matches. This corroborates metadata ownership without asserting physical validity of mixed complex fixtures.

`terminal-query-transition-expectations.json` freezes proposed before/after projection verdicts separately from actual before outputs and official physical targets. After expectations have not yet executed; they await root's frozen migration. The foreign-owner internal law should become pass through exact owner selection; unknown-only discovery should become incomplete by explicit conservative uncertainty policy. Existing positive late-unknown cases must remain incomplete, and all four official business layouts must remain pass. Python finished normally; no probe remains active.

## Frozen owner-qualified migration closure

Fresh `terminal-query-complex-after.ts` executes the same complete-complex probe construction against the frozen production migration and exclusively writes `generated/terminal-query-complex-after.json`. Before files are untouched. Baseline stays pass; all four positive late-unknown cases stay incomplete; foreign RelAggregates owner becomes pass; unknown-only discovery becomes incomplete. Official H34/V45/C61/C2388 remain pass with identical hashes.

Source review confirms each required field now calls the same shared ifcAttributeQuerySteps with the six independently corroborated declaring owners. Unknown dominates populated matches, missing values remain incomplete, and actual borrowed match pointers flow into diagnostics. Relation discovery records unknown before skipping a nonpositive role. This is expressly conservative discovery policy over internal unknown projections. The former match-only helper and duplicated match interface are removed from common concepts; no replacement graph query algorithm was introduced. Source/request capture, original callback scope and controlled runner remain in use.

No new confirmed P1/P2 in this bounded migration review. Native parity and physical mixed-complex legitimacy remain unproved; official business predicate parity does not establish full view/schema conformance. All after probes exited normally with no live Bun mappings.
