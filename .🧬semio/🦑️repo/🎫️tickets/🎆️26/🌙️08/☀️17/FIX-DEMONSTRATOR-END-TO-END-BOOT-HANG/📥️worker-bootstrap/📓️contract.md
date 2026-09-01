# Original Shard Worker Bootstrap Declaration

Historical run3 checkpoint. The shared-purpose funding, gate endings and model below are superseded by [the executed refinement and narrow source-test proposal](./📓️refinement.md). Preserve the original output and hashes as evidence; do not use its 704-byte worker-controller proposal as the current UI-first funding contract. No runtime mount follows from either checkpoint.

Date: 2026-08-28. Status: declaration and model only. No worker/receiver implementation, production caller change, canonical forwarding, publication, or fresh guest claim.

## Boundary

The existing `ShardClient` constructor allocates the client fields/maps and calls `spawnShard` before the slot is pushed into `shards`. `spawnShard` calls a return-only `createWorker`, builds the slot, assigns two handlers, and posts optional heartbeat SAB before returning. Its message handler extracts `event.data` before `handleMessage`; its error handler checks the current index before retaining the original event. There is no `onmessageerror` binding.

This packet specifies the original-client-owned worker participant and its first ingress custody. It does **not** retroactively admit the original client constructor, existing collection backing, platform Worker memory, callback work, or per-request destinations. Those remain explicit requirements before mounting. The existing same injected ledger is the only capacity source. No second pool, channel, fallback transport, worker-wide execution lock, or generic finalizer is introduced.

The already-passing inbox tests characterize existing mixed traffic and existing unacceptable producer traces. They are not the implementation of this declaration.

## Exact Proposed Source Surface

These signatures are declarations, not source-present exports:

```typescript
ShardClient.prepareWorkerBootstrap(grant: ResidentGrant): ResidentStep
ShardClient.beginWorker(index: number, grant: ResidentGrant): ResidentStep
ShardClient.closeWorkerStep(original: OwnedShardWorkerConstruction, grant: ResidentGrant): ResidentStep
type CreateShardWorker = (construction: OwnedShardWorkerConstruction) => void
OwnedShardWorkerConstruction.construct(): void
```

`OwnedShardWorkerConstruction` has a private constructor/brand and the exact original slot, one-shot phase, and retained fixed module-options object. It uses the existing canonical `SHARD_WORKER_URL` and a captured platform constructor; it does not accept a function returning an arbitrary Worker or `adopt(worker)`. No neutral record/cell getter is public. The injected `createWorker` callback becomes an invocation of this owner, not the producer of an untracked return-only Worker. All authored production/test callers must eventually change together; there is no old-factory overload.

The sole allowed construction operation stores the actual returned Worker in the original slot immediately, before returning to any outer callback/wrapper. A wrapper that calls this operation and then throws leaves that worker owned. A throw before calling it leaves an exact empty construction owner and original fault. A native constructor that allocates an inaccessible platform resource and throws before returning is **not** repaired by this JS handoff. That requires the separate platform construction owner; no invented worker or termination receipt is allowed.

Even a callback typed `void` can return a Promise/object at runtime. The original slot's `factoryResult` stores that exact return before checking for `undefined`. A noncanonical return fences further construction and remains owned; it is not ignored, awaited, converted to a new Error, or credited as success. An arbitrary second fault is not allowed to overwrite the first.

The private source entry points are `captureMessage(slot, event)`, `captureError(slot, event)` and `captureMessageError(slot, event)`. The three precreated handler closures each capture only the original slot and call a module-private entry point. The slot contains the original client. These are not public submission capabilities or methods on an unknown caller object.

The eventual handoff observation is private and per actual destination kind. **There is no generic `isAccepted`, callback, raw boolean, or source-present destination proof in this packet.** `observeIngressHandoff(originalSlot, originalEvent, exactDestinationObservation)` is a dependency specification, not an implemented verifier. The receiver packet must supply real pending/effect/retirement owners before any positive handoff can be mounted.

## Original Parent And Admission

The nine added client words are declared in `🧬️declaration.json`: bootstrap cell, bootstrap record, bootstrap phase, first bootstrap fault, child admission cell, child record, child index, original child shell, and `clientAdmissionPurpose`.

The last word prevents a concrete alias collision: UI and worker controllers otherwise both call `prepareAdmission(this, ...)`. A pending UI cell must never be recovered and claimed as a worker cell merely because the original client is equal. The private purpose union is `none | ui-controller | ui-pool | worker-root | worker-slot`; it is installed before preparation and cleared only after exact claimed-cell observation. A foreign purpose refuses with zero allocation and leaves the original cell/phase unchanged. This serializes only one neutral construction handoff, not commands, callbacks, effects, or already-issued retirement turns. No public purpose string supplies authority.

The original client must itself already have a recoverable, independently owned construction shell before this slice is live. The new bootstrap record is installed on that exact client before worker-child construction. Its fault/root stays funded throughout the child record/cell cleanup; it is not refunded because a worker slot is empty.

The neutral controller prefix uses separate grants `[296,64,64,64,264,64,64,64]`: prepare cell; recover original cell; claim; observe claim; reserve record; recover original record; install original client; observe live shell. A consumed child result returns pending; a later observation receives its own grant. A wrapper-after-mutation fault is recovered from `preparedAdmission(this)` or `cell.result.record`, never from a newly allocated replacement. The shared purpose is checked before either recovery.

For a worker child, reserve its full declared record using the existing cell mechanism before allocating the fixed slot shell. Set `workerAdmissionShell` to that original shell immediately before any fallible constructor finalizer. Install the record on the shell, then observe `matchesLiveShell` on a separate grant. Only then link the shell into the existing `shards` roster and allocate/initialize its admitted descendants. Existing array backing must have a separate capacity permit; array growth is not hidden in a 64-byte observation.

There is no second retired-worker roster. Replacement links `newSlot.previous = originalOldSlot` before replacing `shards[index]`. Old callbacks and captured retirement work continue to use their original slot/worker. A failed replacement remains retained in the parent's original child slot; it cannot overwrite a live original or lose the new shell. One-at-a-time construction is not one-at-a-time worker execution. The old-incarnation chain remains until exact terminal proof; it is not synchronously walked or dropped.

## Logical Census

All figures use the existing **logical** shell64/word16 model, not measured V8 heap. JSON lists every field, count, and multiplicity. Neutral record overhead is264/3/3; cell is296/6/6. Metadata allocations, aliases and record charges are distinct from work grants.

| Participant | Bytes | Slots | Owners |
| --- | ---: | ---: | ---: |
| Nine new words on the existing client shell | 144 | 0 | 0 |
| Exact client bootstrap record plus cell | 704 | 9 | 9 |
| Per-worker domain below | 2352 | 23 | 23 |
| Per-worker record plus cell | 2912 | 32 | 32 |
| New client participant plus one worker | 3616 | 41 | 41 |

The new client words add no second shell/base owner: the original logical client576 already includes the UI controller208 (base64 plus nine words). **208 is not added again.** Its other368 bytes remain a named, unadmitted preexisting subset. PendingEntry160 is unchanged and excluded; the prior per-request census cannot be funded by these records. Existing UI pool/controller charges stay unchanged. This is not a replacement numeric caller baseline or evidence that the whole client is admitted.

Per-worker domain:

| Exact allocation / alias | Bytes | Slots / owners |
| --- | ---: | ---: |
| Original slot,24 words | 448 | 1 / 1 |
| Existing five-word heartbeat shell | 144 | 1 / 1 |
| Two empty Set headers, no entries/capacity | 128 | 2 / 2 |
| Construction owner,three words | 112 | 1 / 1 |
| Retained module options,one word | 80 | 1 / 1 |
| Three handler functions | 192 | 3 / 3 |
| Three one-slot lexical environments | 240 | 3 / 3 |
| Three binding records,worker/handler/phase | 336 | 3 / 3 |
| Three external registration aliases | 48 | 3 / 3 |
| Active ingress,six words | 160 | 1 / 1 |
| First violation,six words | 160 | 1 / 1 |
| SAB attachment state plus original message | 208 | 2 / 2 |
| One phaseful terminal observation,two words | 96 | 1 / 1 |

The three new handlers replace the current two; the old two are not also charged. The model deliberately charges three environments rather than relying on engine context-sharing. Private prototype/module functions are not per-worker closures. The existing factory callback and its captured caller options, watchdog timer/callback, client maps, Set entries, roster capacity, pending Promises, AbortControllers, response/parser/scratch, shared SAB backing, platform Worker/EventTarget internals, event graphs and guest memory are **not covered** by these numbers. The48 registration-alias bytes price the logical retained handles, not EventTarget's physical implementation. No actual callback/deadline permit or allocation bound follows from the field census.

## Handler And SAB Fault Frontiers

Before creating a Worker, precreate and retain the three handlers, binding records, active ingress, first-violation cell, fixed options, and terminal observation under the recorded original slot. A constructor/finalizer fault must retain the exact shell before reporting the first raw value.

For each of `message`, `error`, `messageerror`, record the exact worker/handler pair and attempted-install phase **before** installing the handler. A before-mutation throw keeps the attempted pair. An after-mutation throw keeps the installed callback alias and original exception. Property assignment returning, reading a mutable public property, or a wrapper returning `true` is not a terminal observation. The platform adapter must prove its exact captured operation/registration; an arbitrary proxy/setter is not an admitted native Worker. Keep both aliases across uncertainty, and do not allocate a fresh handler to retry.

There is a startup ordering gap that cannot be handwaved away: a Worker may emit while one of its handlers is absent. Construction plus binding therefore needs a proved non-yielding platform turn or a coupled producer-start fence before guest/worker output. This packet does not introduce a new start message/ABI or claim the current producer supplies such a fence. A failed bind never publishes a ready worker. Events already delivered through an installed handler still enter the original source even while startup is faulted.

For SAB attach, pre-own both `{kind,shardIndex,sab}` and its two-word attachment state. Set the attempted-post phase before calling the original worker. Before-post refusal and after-post wrapper throw retain the same message, same SAB and same worker. Because the existing attach message has no receipt, post return is not acceptance and a retry cannot be assumed harmless. Shared-memory contents are not made exclusive or immutable by this metadata record. Do not clear the attachment alias until the eventual exact worker terminal/custody join.

## Pre-Callback Transfer Matrix

| Input | First source mutation | Exact onward owner required | Release rule |
| --- | --- | --- | --- |
| `onmessage(event)` | Store the original event, callback kind and original slot before any `data`, `kind`, `requestId`, `target`, route lookup or log | Existing dispatched pending reply, semantic frame/effect request, heartbeat/trap participant, or exact canonical receiver, each privately registered before the relevant output | Source keeps event/data until exact destination owns the **whole original event** and its private handoff is observed |
| `onerror(event)` | Same original-event capture; do not first read `.error`/`.message` or synthesize Error | Original worker first-fault participant; later already-issued close path | No new-route lookup; event/error descendants remain retained; no disposal from callback return |
| `onmessageerror(event)` | Same original-event capture, distinct fixed callback kind | Original worker decoding/transport-fault participant | Never fold into `onerror` by losing the event identity; no normalized string replacement |
| Unknown or malformed original | Active event is owned before classification | The preowned first-violation cell of the **same** slot | Fixed private move retains event/data/first thrown value; no overwrite or positive response receipt |
| Callback after route replacement/terminate | Capture against the closure's original slot | That old slot's exact accepted destination/retirement authority | Current `shards[index]` is not capture or disposal authority |

Normal ingress reuse is a **same-callback handoff requirement**, not a license to asynchronously occupy the sole active cell. Existing concurrent actors, ordinary results, awaited host-effect replies, shim effect/UI traffic and already-issued retirement output all need exact preadmitted destinations. Until those exist, a one-cell receiver cannot mount without dropping or blocking legitimate traffic. The normal callback cannot simply return while retaining an undelivered valid root and then call the next valid callback a protocol violation.

On a genuine first violation, stop **new admissions** and retain the original pending/activation roots. Already-accepted ordinary results/effects and retirement control remain routed to their original owners; there is no generic allowClosed flag. A violation phase must not reuse `slot.available=false` as a reason to discard those callbacks, and must not call the current synchronous `failShard`/abort fan-out from the capture boundary.

The active and first-violation cells can retain two unresolved original graphs. This is finite **metadata**, not a finite-byte guarantee for arbitrary graphs or hostile uncredited message flooding. If both are occupied and a third raw event arrives, this packet has no bounded complete custody solution and cannot claim the platform callback retained that root merely by returning `refused`. A compliant producer/receiver-credit and platform delivery contract must close that boundary before live mount. New admission fencing does not retroactively stop already-queued traffic.

## Alias Release And Close

1. A private move into the first-violation cell copies original event/data/fault identities and the original slot as origin; it never stores a pointer to the reusable active cell. Only after that exact same-owner move may the active cell clear.
2. A valid destination handoff must pre-own the original event **before** resolving a Promise, invoking callbacks, formatting errors, or acknowledging native data. A post-handoff throw keeps both source and destination aliases until the source observes the exact original private handoff. No public `boolean` or matching request string discharges them.
3. Handler unregister uses the exact original worker/handler pair. Both registration state and callback environment stay charged until actual removal and already-in-flight/queued callback drainage are proved. Replacement, `terminate()`, post return, `actors.delete`, and an empty pending map are not that proof.
4. The phaseful terminal observation is preadmitted but cannot become terminal in this packet. A later private worker close join must prove handler/callback/input/attachment/accepted pending/effect/activation descendants empty and actual platform teardown. Neither neutral record retirement nor a structural callback may mint that proof.
5. Only after that exact domain proof may the original worker record detach, its stable original detachment be observed, neutral record/cell aliases be retired on separate grants, and the exact roster link be unlinked. Original first faults without bounded disposal keep their records. No empty-cancel shortcut or per-slot reuse.
6. The original client bootstrap record stays charged across worker cleanup and faults until its separately proved complete client/controller boundary. It cannot refund the new controller words while those fields still point at cells/records/slots.

## Tests And Limits

The language-neutral fixture declares20 traces: factory before/after fault; all three handler before/after faults; SAB before/after; event-before-data fault; distinct error/messageerror originals; first-violation preservation; old-index callbacks; post-handoff fault; foreign handoff; accepted ordinary/effect/retirement interleaving after fencing; late callback after termination; and no replacement of an already-owned Worker.

The ticket controller runs strict Ajv on both closed schemas, TypeScript AST against the actual three existing layouts and the accepted inbox fixture, exact arithmetic, the20 model traces and eight exact admission-purpose/client/ledger cases. Immer replays the same normative reducer; this checks its immutable output against the fixtures, **not** an independent production implementation or real Worker execution. Missing production methods remain unimplemented; runtime RED/GREEN and actual platform transfer/clock tests must precede a future source mount.

Initial unscoped Nx invocation failed before tests because Nx exec changes to each selected project directory; the corrected command explicitly selects the Actor project and supplies its project-root-relative ticket path. This was controller routing failure, not a product RED. No file was lost or cleaned. The second invocation passed the earlier eight-client-word declaration; it is superseded by the final nine-word run below. No runtime/source failure or full strict-TypeScript claim is inferred.

### Actual Final Run

Command (one explicit Actor project, no all-project execution):

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=@semio-tech/framework-actor -- bun '../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/📜️script.ts' check
```

Executed exit0. Two strict schemas, three existing source layouts,20 model cases and eight admission identity cases passed. Seven selected pre/post hashes were identical. Log: `../🧪️worker-bootstrap-declaration-3.log`. The complete output is retained here as well:

```json
{"status":"PASS","scope":"declaration/schema/source/model-only","schemas":2,"sourceLayouts":3,"cases":20,"admissionCases":8,"thirdPartyReplay":"Immer; same normative reducer, not independent production semantics","resources":{"clientDomain":{"bytes":144,"slots":0,"owners":0},"clientRetained":{"bytes":704,"slots":9,"owners":9},"workerDomain":{"bytes":2352,"slots":23,"owners":23},"workerRetained":{"bytes":2912,"slots":32,"owners":32},"oneWorkerCombined":{"bytes":3616,"slots":41,"owners":41}},"hashes":[{"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/🧬️schema.json","sha256":"4ebe82e53671626a8069fe66ecdf75cb7642cb02df32fba48e65b30c7cd4e751"},{"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/🧬️declaration.json","sha256":"afa9c88d51944756caf70ef2449c55a1b22edce34560715e75b0661d87928d6d"},{"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/🧪️schema.json","sha256":"22d954edce08583eebbee8fa0942ad81fb1e8ea0e723b985edb8e9bff8bc4fc9"},{"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/🧪️fixture.json","sha256":"2dd6c42e9c970ebd6538d24d9909640b545fdaa5a1cd85250ab8e8064d5be7a3"},{"path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📥️worker-bootstrap/📜️script.ts","sha256":"ae729fab0a7772971cc4c7f22f6d78e59617ccbb9546b72ba8bf787ab9bb9d70"},{"path":"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts","sha256":"98710401ee3d18c95536fa64a8e7cfabd09e9ba06adf8d745792d5d452376a73"},{"path":"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️fixture.json","sha256":"8d02dd1fd5d8db33c8f24eee643a97c317a2d74fd7e94c4c4122644860e4a8f4"}]}
```

Only these ticket files and reports changed. Shard/response/materializer and existing runtime source remained unchanged. Root's separate registered inbox selector independently passed2/178skip180,1.24s,start03:55:02,Nx0 with eight stable inputs; that delegated result remains characterization, not this model's runtime implementation.

## Review Request

Approve or refine the exact nine-word shared-client admission gate,24-word original worker slot, factory invocation/custody contract, handler/SAB phases and unchanged-capacity metadata envelope before implementation. Required coupled work remains original client/platform admission, per-kind destination admission and same-callback handoff, worker-side original guest-result/first-fault ownership, one-response producer credit, actual queued-callback close witness, and native paged return/semantic input integration. This packet alone enables none of those paths.

## Reader Hold Release — Delegated Evidence

Runtime coordinator reports its actual `OwnedResidentReader` replay:14PASS/706skip720,3.41s,start04:16:24,Nx0. Its first posthash output was truncated; complete readback confirmed all77 selected pre/post hashes equal, with no demonstrated drift. This is the coordinator's executed evidence, not a rerun by this task.

The imported Shard/input/return/output hold is released. The declaration remains unchanged at `afa9c88d51944756caf70ef2449c55a1b22edce34560715e75b0661d87928d6d`; this notice adds only report text. Bootstrap review is still pending. Reader-source verification does not establish producer/page/semantic integration, raw InputAck, or authority to mount bootstrap/receiver behavior. No runtime edits or cleanup followed this release.
