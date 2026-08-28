# Retained Constructor Admission Results

## Executed RED and GREEN

The R20 canonical neutral test failed at `assert.doesNotThrow`: the actual record constructor had already linked its facade into the ledger before its final `Object.freeze` threw. The original facade remained recoverable only through whole-ledger close, not through the failed caller's result. The exact output is `🧪️shared-resident-admission-red-r20-2026-08-27.txt`.

R21 completed `bun x nx run @semio-tech/value-resident:test --skip-nx-cache` with exit zero and strict component TypeScript zero. Its full output is `🧪️shared-resident-admission-green-r21-2026-08-27.txt`. It executes the prior full neutral cohort plus the five `admissionFailures` vectors. The existing printed census does not yet list these five separately; they are executed assertion loops, not a claimed printed test count.

Each new vector injects an actual facade-finalization failure for owner, record, page, reader or external slot. The result must retain the exact object captured by the throwing finalizer, mark it close-only and report rejection. A zero grant preserves its charge. Closing only that returned object restores the exact prior usage while an unrelated page on the same ledger still yields byte 73 through a genuine registered reader. The ledger and unrelated consuming owner remain live. Strict Ajv validates the neutral cases; Immer and Buffer/BigInt continue to validate the resource and content laws in the combined cohort.

## One Canonical API

| Admission | Result |
| --- | --- |
| `ledger.beginOwner(partition, grant)` | `{step, owner}` |
| `owner.reservePage(length, grant)` | `{step, page}` |
| `owner.beginRead(source, grant)` | `{step, reader}` |
| `ledger.reserveRecord(partition, envelope, grant)` | `{step, record}` |
| `owner.reserveExternalBacking(maximumBytes, grant)` | `{step, slot}` |

There is no overload or old nullable-facade wrapper. Normal readiness returns the registered facade with a ready step. Capacity refusal returns blocked/null without admission. Constructor-finalization failure returns rejected and the exact already registered closing facade, not a replacement. It cannot allocate, install a shell, begin receiving, write or create readers. The original neutral parent still owns it if the caller abandons the result.

Constructor failure costs remain bounded by the already admitted fixed constructor envelope: owner 192, record/page 256, reader 128 and external 320 work bytes. Page backing allocation stays a separate step; neither metadata admission nor rejection implies a large allocation or an eight-millisecond timing certificate.

## Separate Remaining Frontier

An outer wrapper may throw after the canonical function returned but before its typed caller stored the result. Returning a rejected facade does not solve that distinct lost-delivery boundary. The original concrete composition must already own its pending construction/handoff slot and recover the exact original result there; it must not repeat allocation or close unrelated users as a substitute. The actor owner is implementing that original Shard slot. UI pool/source/child adoption is still in progress.

The neutral protocol certifies intrinsic registration/backing lifetime only. It does not mint actor activation, guest lifetime, copied-input receipts, UI publication acknowledgement or concrete-domain terminal authority. The stable record-detachment observation can retain an already domain-empty facade; its existence does not prove arbitrary user objects empty.
