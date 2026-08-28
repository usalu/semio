# Pending Patch Exact Receiver

## Current Native Source Boundary

`Plugin/⚛️reactor/📨️pending/🦀️component.rs` currently stores a checked sequence but derives its instance from the surface string. `apply_published_ack` selects a retained owner using surface and revision. `turn_handback` similarly retains only a numeric instance. Neither selection establishes the canonical `ActorUiPatchReceipt` lifetime. The guest reducer's `PatchRejected` branch bypasses pending ownership entirely and resets the surface by name. These remain source-confirmed gaps, not corrected behavior.

Retained owns the live PatchTracker producer reservation. Its agreed entry will receive the actual captured `NativeCloseKey` from the outer lifetime owner and retain that key in SurfaceSlot, grant and Ready metadata. It must not create the key from a numeric instance, current registry lookup, surface, render generation or operation identity. This lane owns the corresponding Pending receiver and canonical receipt.

## Receiver Contract To Mount

The Pending slot must be reserved before the producer transfers Ready. A private reservation identifies one fixed slot, its captured NativeCloseKey and its checked positive patch sequence. Reservation failure, sequence exhaustion and a refused header/placement grant leave the producer and target unchanged. The slot itself owns the optional Ready target; a local temporary returned from `take_ready_patch` is not the retained target. Retained's `take_ready_into` transfers only after that target and its physical placement bytes are admitted.

The slot records the full `ActorUiPatchReceipt { lifetime, patch_sequence }` at admission. Lifetime comes only from the captured key. Surface and revision remain payload validation, never the authority selector. ACK and rejection first select the exact receipt, then validate payload metadata. Foreign activation, same-activation reused instance, prior guest lifetime, wrong sequence and duplicate feedback cannot consume or mutate another slot. Closing uses the exact key rather than a numeric-instance comparison, including after the surface text has retired.

The canonical return source must keep the typed UI root in an externally retained structural owner while its per-operation encoder reborrows fields. A page-input ACK only frees its corresponding raw page; it is not the semantic UI ACK and not evidence that the original typed patch or Published owner is empty. The existing `take_one -> UiPatch` and `UiTurnPatches::IntoIterator` paths do not supply that final descendant witness. They cannot be used as a temporary whole-copy bridge under this cutover.

## Required Native Laws

1. Exact slot reservation precedes actual in-place Ready transfer; occupied, zero/insufficient grant and exhausted-sequence cases preserve the original source allocation.
2. Same surface/revision and numeric instance under a new guest lifetime rejects the old receipt for both ACK and rejection, retaining the current Published owner.
3. Partial typed close and caught callback failure retain the original slot, cursor and captured key; surface retirement cannot erase its lifetime scope.
4. Raw source/page retirement and semantic ACK are independent. Neither receipt alone releases the aggregate while the other exact owner remains live.

These are a concrete caller contract and queued acceptance laws, not a native pass or a new wire format. The existing ActorUiPatchReceipt schema/codec remains the only semantic patch receipt. The current Plugin no-run inventory holds production source; no Pending API was changed while that snapshot runs.

The first two concrete tests are authored in adjacent `🧪️authority.rs`, currently **unmounted** so the genuine Plugin baseline inventory is not replaced by deliberate missing-API diagnostics. They exercise the real PendingPatchAuthority storage with the existing private test close-key fixture: destination/header admission before external typed source transfer, occupied/zero-grant preservation, checked-sequence exhaustion, and refusal of an old guest close key for a current pending source. Those keys are unit-test fixtures, not proof of a real captured app allocation. The subsequent actual app/Ready/feedback tests still require the native production lifecycle owner and Retained's in-place producer join.

## Neutral Identity Packet

The pending domain now has `🧪️authority.json` and `🧪️authority.schema.json`. Seven exact/foreign cases are exercised for both ACK and rejection, with unchanged surface/revision under a prior guest lifetime explicitly represented. Numeric strings reuse the existing canonical lifetime u64 schema. The fixture explicitly records `liveGuestReceiverProven:false` and `semanticPublicationProven:false`; the reservation/retirement clauses remain queued native obligations, not model-proven grants.

Placement admission must cover the actual whole PendingPatchSlot, not assume a 4096-byte header. The current slot contains a Ready owner plus separate Published and ACK options, each embedding a UiPendingPatch cursor. The staged native test requires actual `size_of::<PendingPatchSlot>()` to fit the unchanged 32768-byte placement ceiling and be included in the required grant. The 64-slot backing's allocation/initialization and exact retained lookup remain separately required; the identity model does not account either by declaring one logical item.

The first strict Ajv oracle invocation (session 42983) exited 1 before the identity model: `required property "activationGeneration" is not defined ... (strictRequired)`. The identity schema now declares its properties explicitly through canonical references; strict mode was not relaxed. The follow-up invocation is recorded separately below when complete.

Follow-up session 33249 actually exited 0:

The same strict fourteen-row oracle was repeated after adding the unchanged 32768-byte placement ceiling and clarifying the scope flag to `liveGuestReceiverProven`; session 36971 also exited 0. The native placement assertion remains unmounted/unexecuted.

```text
[DEBUG] strict Ajv + Lodash exact feedback identity PASS:14 rows; no native receiver/publication claim
```

A separate five-spelling lexical probe exited 0 in session 82924: positive `1` and u64 maximum accepted; trailing newline, zero and u64 overflow rejected. The preceding session 25450 was a shell-quoting/eval SyntaxError before schema execution, not a semantic failure. No canonical schema alteration followed this probe.

Session 33636 repeated that probe with `String.fromCharCode(10)` and logged codepoints `[49,10]` for the rejected trailing-newline case, again exit 0. Exact lexical expression (same Nx exec route; quote the environment expression literally so `$ref` is not expanded by the shell):

```javascript
(async()=>{const{readFileSync}=await import("node:fs");const{default:Ajv}=await import("ajv");const a=new Ajv({strict:true});a.addSchema(JSON.parse(readFileSync("🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json","utf8")));const s=JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧪️authority.schema.json","utf8"));const v=a.compile({...s,$ref:"#/definitions/positive",type:undefined,required:undefined,properties:undefined,additionalProperties:undefined});console.log("[DEBUG] pending positive lexical "+JSON.stringify(["1","1"+String.fromCharCode(10),"0","18446744073709551615","18446744073709551616"].map(value=>({value,codes:Array.from(value,c=>c.charCodeAt(0)),valid:v(value)}))));})()
```

Exact task invocation: set `SEMIO_PENDING_AUTHORITY_EVAL` to the expression below, then run `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_PENDING_AUTHORITY_EVAL)'`. The third-party comparison covers only the fourteen explicit identity/payload-validation outcomes, not native scheduling, memory grants, duplicate application or descendant close.

```javascript
(async()=>{const{readFileSync}=await import("node:fs");const{default:Ajv}=await import("ajv");const{default:L}=await import("lodash");const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/";const f=JSON.parse(readFileSync(p+"🧪️authority.json","utf8"));const a=new Ajv({strict:true});a.addSchema(JSON.parse(readFileSync("🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json","utf8")));const v=a.compile(JSON.parse(readFileSync(p+"🧪️authority.schema.json","utf8")));if(!v(f))throw Error(JSON.stringify(v.errors));let n=0;for(const kind of f.kinds)for(const row of f.feedback){const actual=L.isEqual(L.assign({},f.current,row.changes),f.current);if(actual!==row.accepted)throw Error(kind+":"+row.name);n++;}console.log("[DEBUG] strict Ajv + Lodash exact feedback identity PASS:"+n+" rows; no native receiver/publication claim");})()
```
