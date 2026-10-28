# Guided Staged Form Independent Audit

Initial read-only checkpoint of framework manifest/arg-schema/staging schema/TS/query/reducer and IFC templates/apply/staging.ts plus actual authored carrier tests. Producer is unmounted. ArtifactEditor/pending original-owner hook, native/browser render and Execute integration are not implemented at this checkpoint. Neither finding below is presented as an observed mounted interaction failure; both are executable producer/reducer contract failures that must be closed before mounting.

## P2: ready carrier commands do not match emitted control paths/choices

valueField unconditionally inserts typedValue/value/items/0 addressing for IFC named domains. Existing closed template command accepts raw scalar carriers, and the actual shared native-cases.json includes those forms. The producer nevertheless returns ready=true without converting the request's staged args to the emitted wrapped form. The generic validator then cannot find required values; attempting an emitted nested edit would also encounter a missing intermediate parent. P_ENUMERATEDVALUE choices similarly offer typed string wrappers while an accepted raw literal does not equal that choice.

Actual retained ticket input staged-form-independent-probe.ts invokes produceIfcTemplateStagedForm then stagedFormInvalidFields on every one of the fifteen existing carrier applications. All fifteen produce ready=true; fourteen are rejected by the generic field validator. Pset_ActionRequest fails at entries/0/value/value/items/0/value; quantities, list/table and bounded values expose the same addressing mismatch. The enum case fails at entries/0/values/0. generated/staged-form-independent-probe.json records all exact rows.

Choose one explicit staged command normal form and enforce it consistently before descriptions are published, or emit paths and full choices that address the current accepted command form. Do not make the renderer guess wrappers or silently change exact named domains. Add an actual producer→generic invalid-fields→event reducer roundtrip test for all fifteen carriers, not only readiness and declaration-choice inventories.

## P2: absent optional records still validate required descendants

stagedFormInvalidFields flattens all fields through fieldsAt and validates every required text/choice leaf, regardless of an absent optional ancestor record. This prevents omitting optional bounded upper/setPoint values even after correcting the first finding.

Independent exact case: take the shared Pset_AirTerminalBoxTypeCommon P_BOUNDEDVALUE carrier, omit upper and setPoint, and wrap its existing lower value in its actual declared IfcVolumetricFlowRateMeasure. Producer returns ready=true. Generic validation reports only the nonexistent upper/value/items/0/value/bits and setPoint/value/items/0/value/bits paths. generated/staged-form-optional-wrapped-probe.json isolates this failure from raw-carrier wrapping. Missing optional record descendants should be inactive; present records must continue to validate their required children. Add absent/present/created/removed optional record cases to the neutral framework contract.

## Other reviewed boundaries and limits

Producer captures request plain own-data and a complete private IFC graph before generating semantic fields. Signal and progress references are captured into its options before yielding; revision/action are checked, and query response correlation includes action/generation/revision/window. Reference choices scan source incrementally and preserve full ids. Identity requirements come from the real selected-entry planner, with explicit empty id/guid drafts; ready requires the actual application diff planner, not a UI max-id guess. Readiness remains structural command staging, not template semantic publication permission.

The generic StagedFormQuery cancels the previous local request and rejects obsolete token responses. This is not the pending original-owner lifecycle needed for native mounting. Native Rust currently describes shared wire types; it does not prove an equivalent mounted reducer or original-wallet form producer. The temporary local referenced enumeration resolver is slated for delegation to the details worker's generic query and must not be described as mounted same-schema parity yet.

Both concrete findings were sent directly to the owning worker and root before this report. No production source was edited by the auditor. No native compiler/type/runtime proof is claimed; authentic compiler ingress remains refused and semantic admission gates remain unchanged.

## Independent correction probe

Worker corrected representation-aware producer paths/choices and optional-record traversal. Reran the original ticket probe against current source: all fifteen ready carrier applications now return an empty stagedFormInvalidFields list; both the raw-lower and explicitly wrapped-lower bounded cases with absent optional upper/setPoint remain ready with empty invalid lists. Generated probe JSON files now contain these corrected after-results; pre-fix values above are the observed historical counterexamples. Native/browser/pending-owner mounting remains open and is not approved by this probe.
