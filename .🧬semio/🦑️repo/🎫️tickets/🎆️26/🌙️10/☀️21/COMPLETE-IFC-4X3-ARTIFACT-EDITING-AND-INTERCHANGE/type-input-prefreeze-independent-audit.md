# Required Type Input Pre-Freeze Audit

Two confirmed P2 findings in the current TypeScript producer were reported directly to the owning worker before combined source freeze. This is not a final native controller audit. No production edits were made.

1. `details/type_input.ts` prepare uses `$ifcInputs ?? []`, accepting explicit null as omitted. The form subsequently tests only undefined before emitting its fixed array-creation edit. With null it publishes ready=false and a fixed append into the null property. Actual applyStagedFormBinding rejects the published binding with `staging.array`. Reject explicit null before publication; retain the intentionally supported omitted-property guidance path. Add a neutral null input vector and actual binding application regression.
2. The local options helper closes over `control.progress` rather than the captured callback value. A first progress callback replacing the source control's callback redirects all remaining progress. Actual probe observed originalCalls=1, replacementCalls=395. Capture signal and callback once, as the existing apply/staging.ts helper already does; add replacement-callback regression with original cancellation/fault identity checks.

Authored probe `type-input-null-independent.ts` imports the actual producer and shared binding reducer, builds the existing neutral source roles without copying implementation, and records results in generated/type-input-null-independent.json and generated/type-input-control-independent.json. It exited normally. The external control mutation is deliberately authored despite the readonly TypeScript annotation: runtime ownership must not depend on a caller obeying static types.

The producer's declared schema currently requires `$ifcInputs` whereas the guided request intentionally supports its absence. That distinction must remain explicit: it is a final command payload schema, not a complete draft-request acceptance schema. The current physical reference tests exercise actual target membership and inverse mutation separately; they do not establish native execution or guided transport/publication readiness.

Combined required-input native producer/controller source, original grants, strict DomainWork and close review remains pending the workers' stable roster. Missing work-in-progress exports were not treated as defects.
## Independent closure recheck

Reran the actual same endpoint probe after worker fixes: null now rejects type.input-rows before publication; original progress callback ran396 times and replacement ran0. Both source P2s closed. The initial raw endpoint output filenames were overwritten by this same-script rerun; the first actual tool stdout observations are preserved explicitly as a transcribed tool record in generated/type-input-before-tool-record.json, while after files are separately named. This is not a regenerated red claim. Combined native review remains pending freeze.

## Third confirmed P2: sorted partial inputs become unreachable

Actual same-producer/reducer/next-producer probe: primary IFCRELASSOCIATESMATERIAL has one direct RelatingMaterial argument; target IFCRELCONNECTSELEMENTS has optional ConnectionGeometry at0 and required RelatingElement/RelatedElement references at1/2. Existing ascending input list containing only row2 is accepted as a partial draft. The producer emits FixedAppend for missing row1, producing [2,1]; its next prepare rejects `type.input-row`. Thus a published control cannot advance this admitted draft. Preserve ascending row invariants when constructing missing-field guidance, with a neutral multi-required-reference test and native producer parity. The probe uses genuine released direct declaration metadata and an internal native-shape graph; it does not claim a full valid physical IFC population. Exact binding and resulting request are retained in generated/type-input-sorted-independent.json. Reported to both parent and worker before freeze.

## Additional source/fault checks

Actual producer and mutation-builder callback probes preserve thrown undefined, number23 and Error identity on the third original progress callback: six of six; generated/type-input-failures-independent.json. No output was returned on these failures. Controller source review separately confirms RenderWork pins exact snapshot address and all32 canonical revision bytes before any guided render phase, rejects changed body, and GuidedWork pins original StepContext source binding plus exact window/locale. Type/Template producer variants remain a closed action-discriminated enum and receive refuses kind swaps. This is a narrow source checkpoint, not final receipt/custody approval of the still-unfrozen type producer.

## Sorted-row closure recheck

The same actual RelAssociatesMaterial→RelConnectsElements draft now emits a fixed sorted roster Set preserving existing row2, then fills missing row1. Applying that binding yields[1,2]; the next actual form returns ready=true. Separate actual before and after JSON files are retained as generated/type-input-sorted-before.json and type-input-sorted-after.json. The first two fixes and six exact callback failure probes remain green in the same rerun. All three confirmed TS P2s are closed at source/actual TypeScript endpoint level.
