# IFC4x3 Final Core Ownership Review

Read-only production-source review, 2026-10-21. Retained inputs: `core-final-repro.ts`, earlier ownership/re-audit repros. No generated files, production edits, build changes, Git modifications or compiler ingress bypass. Focused direct Bun/source tools follow the coordinator's recorded unavailable repo execution route.

## Resolved Ownership Findings

The prior invalid-publication reproduction is resolved. Rerunning `core-ownership-repro.ts` confirms four safe refusals, zero revision advance after corrupt capture, and consistent recorded-command replay. An additional independent phase attack in `core-final-repro.ts` mutates original base/proposal integer payloads on the first matching progress callback:

- `copyOwned` and `validateBase`: the operation refuses `invalidValue` when unsafe data is captured.
- `admitInput`, `applyEntities`, `validateResult`: accepted results remain valid private bigint values despite original input corruption.

The owned authority is no longer exposed through caller aliases. The final graph is validated after owned capture and application; no further caller-data copy occurs afterward. History records its captured command instead of rereading the original command. Final synchronous publication checks cancellation/revision immediately before changing private state. Progress callbacks after result validation concern private inverse admission/public event copying and cannot mutate the prepared graph through original inputs. Existing final-phase cancellation and competing-revision regression cases pass. No new ownership/race defect was found in those audited paths.

## P2 — Native shape admission still accepts malformed required fields/null lanes

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:82` checks allowed fields during capture, but does not enforce required fields or array container kinds. `admitIfcDiffSteps` around `:63` treats null optional lanes as absent. `🧬️mutations/🟦️.ts`, `captureMutation`/`proposal`/`mutationPlan`, builds a proposal from a command without first checking all required tag-specific fields.

Independent reproduction:

1. `await history.executeAsync({mutation:'setEntityName',id:1n} as any,0)` resolves and publishes revision **1**. The graph name stays unchanged, while the event records a command missing its required `name`. Its sparse diff contains an undefined `name`, so the published native event does not satisfy the declared command transport.
2. `await applyIfcDiffAsync(base,{fileDescription:null} as any)` resolves, treating a present null array lane as no change although the native/wire domain requires an array when present.

This does not corrupt the now-private graph, but admits malformed events and advances history for an invalid command. Check required fields and container kinds in the native capture/admission authority, distinguishing omitted optional lanes from explicit null and missing required command payloads. Keep omitted `entity.complex` accepted and canonicalized to `[]` as specified; that is a separate valid optional field.

## Verification and Bounds

- Current focused editing suite: **31 passing tests, 244 assertions, zero failures**. It includes omitted-complex Ajv/independent patch coverage, final-phase cancellation and same-revision competing publication.
- Independent phase-corruption reproduction and retained ownership reproduction executed successfully with safe outcomes except the P2 shape-admission cases above.
- Existing source contract appropriately states incremental capture guarantees a validated owned result; an atomic start-of-call image requires caller inputs to stay unchanged until settlement.
- No broad build, Rust compilation or full EXPRESS/MVD conformance claim. The worker's prior strict TypeScript result remains separately recorded; this review changed no production types.

The report and repro input are retained in the existing ticket; no cleanup of sibling work was performed.
