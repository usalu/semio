# Original Geometry Progress Failure Implementation

The confirmed geometry callback P2 is corrected. Original caller failures now reject with their exact identity before representation/domain diagnostic conversion. No geometry evaluator or native capability was added.

## Test-First Reproduction And Change

Low's actual endpoint probe showed geometry-phase Error propagated, while the same original Error in vertices/triangles was converted to partial fidelity and geometry.unsupported-product. Root authored closed geometry-progress-cases/schema and registered tests before production. Initial run:5pass/8fail across13 tests. It covered Error, IfcStepError-shaped caller exception, undefined and numeric sentinel. A later direct original source-capture extension exposed a second class-conflation path: source capture converted caller IfcEditError into graph diagnostics or a new cancellation error (53pass/2fail).

control.ts now exposes one first-party IfcIoControlScope. It captures original signal/callback/finite limits through the existing control function, separately records failure occurrence and original thrown value, and offers rethrowCallerFailure. A boolean distinguishes throw undefined from no failure. It does not mint cancellation, mutate the caller input, stringify the failure or infer source/work completion.

Shared exchange-source catches rethrow original caller failure before interpreting IfcEditError code. The geometry product catch does likewise before unsupported-product/cancellation classification; admitted-source boundaries also preserve the original failure if a nested converter returns an outcome. Existing genuine domain refusals and original signal cancellation retain their diagnostics/error behavior. The helper holds fixed operation metadata only; no new source graph, runtime dependency or ownership wallet is added.

## Executed Evidence

The final closed matrix covers original source capture, triangle extraction and actual IFC-to-glTF export; source/geometry/vertices/triangles phases; Error, IfcStepError, IfcEditError, IfcEditError(cancelled), undefined and numeric caller failures. All54 vectors plus schema pass: **55 tests /379 assertions /zero failures**. The same callback failure runs through independent AJV custom-keyword validation and its exact exception identity is compared to the actual endpoint. Tests also replace caller control fields and verify no replacement callback, original source mutation or artifact completion.

Six actual geometry/glTF/control/exchange/source suites pass **136 tests /745 assertions /zero failures** in16.96seconds. Targeted strict TypeScript passes for the real new test/production graph. Independent Low reran its unchanged actual three-phase probe:3/3 now reject the exact original Error identity. The before observation remains preserved separately; geometry-source-control-independent-audit.md records closure.

Source changes: io/control.ts scope; io/exchange-source.ts callback/domain distinction; io/geometry.ts callback/product distinction; io/tests/geometry-progress-cases.json, geometry-progress.schema.json and geometry-progress.test.ts; real IFC package selected suite registration. A subsequent complete58-file actual registration includes this change and passes941tests/18309assertions/12gatedskips/0fail in134.05seconds. The earlier56-file872 snapshot is historical. No genuine native skip is replaced.

## Remaining Work

Cold unit/placement/representation fanout and old2048 triangle intervals remain separate source-control work, with concrete map in the independent audit. The general rectangular extrusion evaluator has not been added. Its source/schema/test plan is swept-geometry-implementation-plan.md and independent physical measurements are rectangle-extrusion-oracle-report.md. No rectangle/mapping/BREP support, host auxiliary mount, native execution or semantic publication is inferred from this correction. Goal and ticket remain active; shared generated oracle dependencies remain in use.
