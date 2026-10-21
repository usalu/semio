# Three-Root App Batch Review — Resumed 2026-10-21

Source-only review; no command launched.

Proposed supported public command:
```
repo --tool=nx run-many --projects=@semio-tech/mit-bestand-demonstrator --targets=test,test-runtime-catalog,canonical-architecture --parallel=1 --workers=off --output=stream
```

Existing targets resolve to Bun root test, runtime-catalog-test, and test long ./🧪️tests/🏛️shell-brand/🟦️.ts respectively. All use repo:script and demonstrator cwd. test has explicit dependsOn []; others have no authored target prerequisites. Vitest config maxWorkers1 bounds child test workers. Public parallel1 bounds task concurrency; workersoff disables persistent action workers. No separator/tail means options remain build options rather than being forwarded to test commands.

Run with SEMIO_BUILD_MODE prototype and the existing ticket/artifact environment plus SEMIO_BUILD_TRACE_AUTHORITY true, retaining caller-owned output directory and logs under ticket generated. Existing authored targets/normalization retain Original and service/native admission requirements; no raw test runner or bypass required.

Concrete scope caveat: buildInvocationGraphSelectionV1 deliberately maps run-many to kind all. This gives one canonical full-registry graph admission/inference, not one three-target-only Source closure. Backend requested roots can be exactly these3, but source normalization and generated prerequisites may be broader. Do not present the plan as a narrow owner-closure optimization. Exact root/dependency result must be observed from native plan/execution authority.

All3targets cache true. A successful run may restore valid cached results; report restoration as such. If the intended evidence requires fresh actual test process execution, add the supported --skip-cache build option before launch. No extra tail is required. canonical-architecture explicitly selects long; ordinary test retains its existing test level semantics.

## Exact Versus All Selection: Proven Source Closure Difference

The difference is not merely a broader canonical graph shape. `🏗️build/🕸️graph/🟦️.ts` computes selected only when selection.kind is tasks. run-many kind all uses whole-workspace inventory and no selected ownerAdmission. In project-nodes, options.ownerAdmission controls selectedTargets; without it selectedTargets is undefined, activeJson is the full json, native preparation uses full nativeCandidate branch, and projectWithDefaults receives undefined selectedTargets. In project-assembly, admitted=selectedTargets===undefined?declared:filteredDeclared and the normalization loop visits every admitted target. Thus all selection computes unrelated targets executable source closures and native/preparation input shapes, even though backend execution roots remain the three requested tasks.

Exact single run can admit many source owner nodes (for example1039 from broad freshness/dependencies) while filtering actual target normalization to ownerAdmission.targets. Those are different scopes. To reach the narrow next actual application assertion without introducing unrelated preprocessing/failure exposure, retain exact unit command first. The three-root run-many batch is supported but cannot be justified as a selected Source closure optimization under the approved current contract. No source bypass or selector change is proposed.
