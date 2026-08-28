# Native Close Worker Construction Order

## Actual Narrow GREEN2

The repaired source was rebuilt and the same selector executed: **2 passed, 0 failed, 515 filtered, 0.29 seconds, exit 0**. This lane directly read `🧪️member-plugin-close-construction-green-r2-2026-08-27.txt`, including the expected caught injected panic and both passing assertions. The native credit is exactly shell construction before live detachment and preservation of the original live allocation/generation through that injected frontier. No allocator admission, actual allocator-failure, callback quiescence, guest, or full descendant aggregate credit follows from these tests. The next approved actual-registry backing test is now mounted cfg(test)-only; its constructor is still unchanged for behavioral RED.

## Actual Behavioral RED After R8 Baseline

The canonical native Plugin baseline compiled successfully. The unchanged-source selector `instance_lifetime_close_construct` then executed exactly two tests: **0 passed, 2 failed, 515 filtered, 0.23 seconds**. This lane directly read `🧪️member-plugin-close-construction-red-r1-2026-08-27.txt`. The first intended failure is `Some(false)` versus `Some(true)` for construction while the original allocation is live. The second catches the injected construction-boundary panic, recovers/drains the exact original app for cleanup, and fails `live=false` versus required `true`. Neither failure was a compiler error, a close-driver watchdog overrun, or a secondary StoreDrop abort. This is the required behavioral RED, not evidence of a real allocator fault.

Production remained unchanged through Retained's immediately following live-output3 snapshot. After its explicit source release, the narrow repair landed: borrow/clone only the original Arc while the live entry remains, construct the close worker, retain it in the prechecked quarantine slot, then remove the live Arc and advance generation. The same registry guards span the whole transfer; no app payload is cloned and no fallible allocation remains between detachment and quarantine retention. The cfg(test) failure frontier now occurs while the original live registry/generation remain intact.

The repaired source has not yet executed GREEN2. A renewed request for a same-binary Mutation packet arrived immediately after the edit; the compiler owner and coordinator were notified that the source changed, and the current source is held again. An already-built R8 binary can still be tested with accurate historical attribution; the repair must not be rolled back or mislabeled as part of that older binary. Older pending/unexecuted statements below are historical for the actual RED2 only. Aggregate allocation permits, final callback quiescence, guest mounting and full descendant close remain unproved.

## Second Staged Native Failure Frontier

The same production path now has a cfg(test)-only failpoint immediately before allocating RuntimeCloseWorkerState, without changing the old detach/generation ordering. A second native test injects failure there and requires the original live registry root, allocation identity and generation to remain unchanged. The test keeps one explicit strong rescue reference, observes the actual registry state before any repair, then restores that exact root only if needed and drains through the real close lease before assertions. This prevents a secondary Store-drop abort from obscuring the intended ownership failure; it is not a production fallback or an alternate close implementation. An actual allocator error is not injected or claimed.

Both tests remain **native unexecuted** behind the current Plugin metadata repairs. The combined exact selector is `instance_lifetime_close_construct` (construction ordering and construction failure). No production ordering change was made before their behavioral RED. The permanent fixture/schema now records the failure frontier and `actualAllocatorFailureProven:false` explicitly. The unchanged strict Ajv + Lodash three-state model was re-executed against the expanded fixture in session 14414, exit 0; it remains model-only and does not prove either native failure law.

## Exact New Test Boundary

The production close function still takes the original live cell before constructing `Arc<RuntimeCloseWorkerState>`. No semantic correction is made yet. A cfg(test)-only fixed boolean now records whether the original allocation remains in the live registry immediately after that worker shell is constructed and before quarantine publication. It performs only an exact registry lookup and pointer equality, with no new synchronization or source ownership change.

The new existing dispatch-harness test `instance_lifetime_close_constructs_worker_shell_before_exact_live_detachment` invokes the actual close lease, records live/quarantine state, and drains the real existing close worker before asserting the observation. It does not raise the watchdog limit, alter Drop, substitute an empty app, or skip the worker. Thus cold callback failure remains visible separately if encountered; it cannot be called the intended ordering RED unless the ordering assertion executes.

Permanent language-neutral fixture and schema are adjacent to the native lease at Plugin `🚪️lifetime/🧪️construction{,.schema}.json`. The three modeled states are live owner, allocated worker shell while live owner remains, and exact quarantine handoff. Both `allocationGrantProven` and `descendantCloseProven` remain false. This ordering test does not establish allocator admission or the complete native/host descendant aggregate.

Native execution is queued behind the actual missing contributed-mutation-wire fixture in Plugin inventory R5; no native outcome is claimed. Kernel entries4 is being compiled independently through the sole compiler.

## Independent Model

Strict Ajv and the existing Lodash reducer compare all three modeled owner states; this is only fixture/model evidence. Process 58763 actually completed exit 0 using the existing workspace Nx exec route, with no native ordering/allocation proof:

```text
[DEBUG] strict Ajv + Lodash close-construction3 model PASS; no native ordering/allocation proof
```

Exact evaluation in `SEMIO_CLOSE_CONSTRUCTION_EVAL`, invoked through `bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_CLOSE_CONSTRUCTION_EVAL)'` with daemon, graph cache and isolated plugins disabled:

```javascript
(async()=>{const{readFileSync}=await import("node:fs");const{default:Ajv}=await import("ajv");const{default:L}=await import("lodash");const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/";const f=JSON.parse(readFileSync(p+"🧪️construction.json","utf8"));const v=new Ajv({strict:true}).compile(JSON.parse(readFileSync(p+"🧪️construction.schema.json","utf8")));if(!v(f))throw Error(JSON.stringify(v.errors));const states=L.reduce(f.events,(out,event)=>{const s={...(out.at(-1)||{liveOwner:false,workerShell:false,quarantineOwner:false})};if(event==="live")s.liveOwner=true;else if(event==="workerShell")s.workerShell=true;else if(event==="quarantine"){if(!s.workerShell||!s.liveOwner)throw Error("unadmitted");s.liveOwner=false;s.quarantineOwner=true;}out.push(s);return out;},[]);if(!L.isEqual(states,f.states))throw Error(JSON.stringify(states));console.log("[DEBUG] strict Ajv + Lodash close-construction3 model PASS; no native ordering/allocation proof");})()
```
