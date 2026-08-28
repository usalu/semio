# Awaited Completion Fixture

## Initial Mounted RED Assertions (Historical)

The existing `subset_macro_derived_register_is_idempotent` now observes the actual shared validator registry after its two existing, still-unawaited registration calls. It requires exactly one matching dialect, not only a direct validator result. The existing checkpoint/restart test captures the actual pending request count after its still-unawaited executor call, then asserts that observation after cancelling the old task. This avoids introducing a replacement fixture that merely reenacts the bug.

Permanent schema and fixture live at `plugin/🧪tests/⏳completion/{🧪fixture.json,🧪schema.json}`. The schema explicitly excludes retained production registration and guest checkpoint runtime proof. No await repair has landed in these two cases or in the generated conformance test yet. The native REDs are queued with the sole compiler after its current live publication checkpoint.

## Independent Fixture Oracle

R1 was an infrastructure failure: `nx exec` without an explicit project selected a nested project, and its shell re-quoting removed the JavaScript import quotes. Bun failed parsing before any assertions. No fixture failure or native result is attributed to that attempt.

R2 used `bun x nx exec --projects=workspace -- bun -e <quoted expression>` with the inner double quotes escaped for Nx's command re-quoting. Actual session67059 exited0 and printed:

```text
[DEBUG] Awaited completion fixture schema PASS; Lodash registration-set and cancellation-set oracle2 PASS; native execution not claimed.
```

R3 repeated the same canonical oracle after adding the exact unknown/consumed Fault fields to the fixture/schema. Actual session29819 exited0 with the same output. This remains fixture/model evidence, not an executed native Fault path.

Strict Ajv validates the fixture. Lodash independently reduces two identical dialect registrations to one unique entry and removes the exact cancelled instance from the pending-request set. These two set-model outputs validate the declared expected counts; they do not prove an actual Future ran or that a guest checkpoint is bounded.

Exact evaluated expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import assert from "node:assert/strict"; const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/"; const f=await Bun.file(p+"🧪️fixture.json").json(); const s=await Bun.file(p+"🧪️schema.json").json(); const ajv=new Ajv({strict:true,allErrors:true}); const validate=ajv.compile(s); assert(validate(f),JSON.stringify(validate.errors)); assert.equal(_.uniq(_.times(f.registration.attempts,()=>"s.test.subset-macro/1/derived")).length,f.registration.registeredEntries); const pending=_.times(f.checkpoint.parkedRequests,()=>f.checkpoint.instance); assert.equal(_.difference(pending,[f.checkpoint.instance]).length,f.checkpoint.requestsAfterCancel); console.log("[DEBUG] Awaited completion fixture schema PASS; Lodash registration-set and cancellation-set oracle2 PASS; native execution not claimed.");
```

## Actual Native RED And Source Repair

The publication owner executed both exact cases separately against the same compiled binary (SHA256 `6a35395a803de7e10d13695a560123d1234e056411de0e96cbe85728409c926d`). Full logs were read directly: `🧪member-plugin-live-isolated-r5-07-2026-08-27.txt` and `...-08-2026-08-27.txt`. Each ran0PASS/1FAIL/520filtered; checkpoint took.04s and registration.02s.

Registration reached its intended new assertion: actual matching registry entries0, expected1. Checkpoint failed earlier at the existing task-count assertion: actual1, expected0. It did **not** reach the new parked-request assertion. Source diagnosis shows the cfg/test task close helper explicitly polls the original ColdFutureExecutor task, retaining Pending rather than detaching it. The old unawaited executor call had not parked the task; the first cancellation poll did so. The subsequent unawaited registry cancellation did nothing.

The repaired source awaits initial parking and actual request retirement. It then explicitly re-drives the existing cold task close helper. RequestRegistry removal does not wake or publish a cancellation result: the next explicit RequestFuture poll observes its absent entry and returns the existing `plugin.request-registry` Fault with message `request already consumed or unknown`. The test now retains the original task's actual Result in a parent-owned slot and asserts that exact Fault plus original task removal before restore. This is not cooperative-cancellation or production-close proof, and no executor helper, request semantics, generic future Drop, driver bound or budget was changed.

The direct registration test now awaits both calls. Generated conformance now awaits registration and validation and checks its actual registered dialect count. A fresh three-selector native gate (checkpoint, direct registration, generated conformance) is queued; no GREEN result is yet claimed.

## Required Follow-Up

Execute the three exact native selectors after this repair. Preserve any genuine later checkpoint/restore or typed app cleanup failure exposed by the now-executed path rather than weakening ownership guards or suppressing warnings. The observed Fault and cold fixture scope must remain explicit in all result attribution.

## Actual R1 Follow-Up

The compiler owner executed all three selectors. Idempotent registration passed1/521skipped (.063s); generated conformance passed1/521skipped (.016s). Checkpoint reached and passed the repaired parking, original unknown/consumed Fault, old task removal and exact restored-command checks, then failed at the later registry-less `TestApp` direct dispatch: `interactive-job.missing-factory` for `applyCountFromTask`. Unwinding then encountered the strict ArtifactStore terminal-shell guard and aborted. Its summary is0PASS/1FAIL/521skipped (.091s), not a checkpoint success.

The raw checkpoint and idempotent files disappeared before the later report read. The compiler owner's `📓️plugin-awaited-completion-r1-native-2026-08-27.md` preserves exact partial tool-transcript excerpts and records that evidence limitation; its generated-conformance raw output was read successfully. This lane confirmed the checkpoint file is absent and did not recreate it or perform cleanup. The next repair must provide the exact TestApp-owned factory, publication preparation and incremental cleanup; a KeyedTestApp proof, direct reducer call or missing-factory suppression cannot substitute.
