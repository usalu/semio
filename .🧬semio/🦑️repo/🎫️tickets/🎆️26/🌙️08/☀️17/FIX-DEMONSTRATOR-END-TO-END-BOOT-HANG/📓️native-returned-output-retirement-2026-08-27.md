# Native Returned Output Ownership

## Actual Pending Dispatch Join

The post-implementation strict renderer gate also ran: exactly seven known tutorial diagnostics, no new pending-entry, output-cell or fixture typing errors. Four selected source hashes matched before/after. Evidence: `🧪️guest-dispatch-strict-3.log`, `🧪️guest-dispatch-strict-pre-3.log`, and `🧪️guest-dispatch-strict-post-3.log`. This is a source-stable scoped strict boundary, not an all-errors-clear result.

Two tests now exercise actual private ShardClient.send and handleMessage with a pre-reserved output cell. They first reproduced a missing original envelope at pending removal and at worker-error grafting (**2 failed/93 skipped**, 1.25s; `🧪️guest-output-dispatch-red-1.log`). The repaired handler captures the exact response before deleting pending state, recomputing heartbeat state, grafting worker errors or invoking promise settlement. A diagnostic callback throwing during the actual graft rejects the caller while retaining both the original failed envelope and the thrown error.

Full actor GREEN passed **95/95**, six files, 1.25s, start 19:45:53 (`🧪️guest-output-dispatch-green-1.log`). Post-test ShardClient hash: `8bf374dbcd0bf29822d8b919c29be0f1c761191fe2aee5326de9aaac4bb6051c`.

The optional private send slot is tested transport infrastructure. Public lifecycle/operation dispatch does not yet allocate or pass these cells; mounting is still gated on bounded descendant retirement and final close joining. No live strong ownership is inferred from these private-join tests.

## Original Envelope Capture

The reserved output cell now has a distinct original response-envelope root in addition to its normalized promise outcome. Capture occurs before envelope-field extraction and refuses replacement. The test simulates the fixed settlement boundary through pending removal, heartbeat recomputation, error graft and external continuation, with success, graft fault and caller fault. The exact envelope and unknown 8192-byte payload remain in the original strong slot throughout. This tests the isolated cell, not live ShardClient dispatch.

The first run stopped before collection on a transient peer-owned discovery parser placeholder; its log remains `🧪️guest-output-envelope-red-1.log`. After the coherent declaration appeared, actual missing-method RED ran **1 failed/92 skipped**, 0.758s (`🧪️guest-output-envelope-red-2.log`). Full actor GREEN then passed **93/93**, six files, 6.66s, start 19:34:52 (`🧪️guest-output-envelope-green-1.log`). Returned-root release remains absent; this roster is not mounted into live ShardClient or credited as final close.

The runtime coordinator confirmed no verified canonical general return-paging API exists yet. Known-field native UI cursors do not certify arbitrary wrapper fields. The original guest result must also be retained in the worker before receipt/command-ingress normalization can throw; host envelope ownership cannot recover a guest root lost at that earlier boundary. These remain explicit implementation obligations.

## Isolated Reservation Foundation

The schema-first `actor/🪪️activation/🚪️instance/📥️output` module is being developed separately from the released ShardClient/renderer integration. Its three tests actually ran RED on missing owner classes (3 failed/86 skipped, 0.934s, `🧪️guest-lifecycle-output-reservation-red-1.log`). The implementation pre-admits a bounded strong response slot before dispatch; success and rejection values remain anchored before external callbacks can throw. Full/closed/exhausted admission leaves existing roots untouched. Only an empty, never-submitted reservation can be unlinked in constant structural work.

This foundation intentionally provides no returned-root release or final-retirement success method. The selected-field and whole-container retirement joins must exist before this roster replaces the live WeakMap-only provenance path; until then it is not a live-output or completed-close claim. It does not claim immutable payload content or bounded structured-clone/heap disposal. Actual full GREEN passed **89/89**, five files, 1.95s (`🧪️guest-lifecycle-output-reservation-green-1.log`). The final schema reuses the canonical lifecycle u64 definition and validates returned/retained phase consistency; its three focused tests passed again in **0.370s**, 86 skipped (`🧪️guest-lifecycle-output-reservation-green-2.log`).

The complete renderer live-source contract was read at `INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️renderer-live-owned-source-contract-2026-08-27.md`. UI owns the typed `{key, source: OwnedUiInstanceSurface}` response/projection, per-original-turn intake and Shell/Interpreter consumers. Demonstrator owns exact lifecycle/scheduler/capture/ACK and raw-root retention. The old BuiltNode maps must be removed atomically with real content ownership, not retained beside an empty participant. Native issued-patch identity is now being coordinated by Dag and is not guessed by this task.

## Verified Entry Point

The ordinary `ShardClient.turn` path bypassed the captured activation's post-settlement join. A language-neutral accepted/revoked fixture first reproduced `actor-lifecycle.foreign-turn`; the repaired delegation passed the full actor gate **86/86**, four files, 2.07s. Logs: `🧪️guest-lifecycle-ordinary-output-{red,green}-1.log`. Both successful and revoked ordinary results now acquire the original instance provenance before caller settlement.

## Still Required Before Live Mount

- The current WeakMap is provenance only. A strong, pre-admitted output cell must own every returned turn before status decoding or caller callbacks: ordinary, Open, fault, lifecycle ACK and UI ACK alike.
- Raw output aliases remain mutable. Frozen metadata does not freeze records, arrays or typed-array contents. A private source claim must retain the exact selected original and refuse substituted slots; it must not certify arbitrary extra fields as retired.
- Reading an operation is a reservation. `OwnedUiPatchInputAcceptance` proves the UI actually acquired the selected field; `OwnedUiPatchInputRetirement` proves that same field's decoder/typed input and applicable operation obligations are terminal. Both have direct private identity checks.
- `source.releaseInput` intentionally leaves the raw operation slot/patch/turn retained. `inputRetired` refers only to transferred UI input, not the raw container or unknown wrapper payload.
- The UI owner's `OwnedUiWireOperationCursor.fromNative` can retire known selected fields of unoffered canonical operations. It cannot discharge unknown record fields. Failed constructor admission leaves the original with the caller; accepted UI input must never be taken away from its exact later retirement token.
- The output cursor must cover patches never claimed by the UI, not only existing patch authorities. Final host/native close cannot be certified while those roots remain.

## Renderer Connection

The existing renderer still stores copied `UiDocumentState`/BuiltNode values in actor-name maps. An empty `OwnedUiInstance` is not a valid retirement witness for those real content roots. Live mounting must connect the exact instance aggregate to `OwnedUiInstanceSurface` read facades and retain the original aggregate across asynchronous settlement. The UI owner has been asked for its concrete read-source bridge; no copied-tree fallback or premature disposal is authorized.

This is an open implementation boundary, not a timing, immutable-data or all-app success claim. No cleanup, source restoration, build-output publication or heavyweight compilation occurred in this packet.
