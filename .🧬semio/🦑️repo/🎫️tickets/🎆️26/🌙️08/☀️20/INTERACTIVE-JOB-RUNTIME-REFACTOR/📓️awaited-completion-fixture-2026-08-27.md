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

## Exact Restart Mode Packet (Before Const-Owner RED)

Root approved a cfg(test) const mode on the existing TestApp: the default mode retains its genuinely empty app-owned catalog; the retained restart mode has a different compiler-owned concrete type and only the exact `applyCountFromTask` row. The shared neutral fixture now records zero default proofs, one retained proof, distinct concrete owners, one artifact publication, one terminal receipt and unchanged1item/4096byte close grants. The mounted new const-owner test is queued for actual missing-API RED before the fixture type changes.

`🧪️tests/⏳️completion/🦀️.rs` contains the staged, unmounted owned command job and scalar transient store retirement. It is not included in the library yet. The transient law calls the actual retained-mode disposer factory, so the old unconditional terminal disposer can be observed as RED before switching to the exact owner. Its new owner retains the original fixed scalar TransientStore through separate handoff/release steps and verifies the installed terminal root/generation; it does not infer terminality from an empty Option alone.

The neutral transientClose row additionally fixes initial nonterminal state,4095-byte refusal, original-owner survival after handoff, original-owner absence after release and rejection of a foreign terminal root. Session36553 passed the extended schema and a five-output model, but its literal phase-array drop is only fixture shape evidence. It was superseded by session82378's six-output model using an actual distinct object in a Lodash retained-owner set and `without` to model exact release. Neither model is native store destruction evidence; the native Weak/root/generation law remains queued.

## Actual Const-Owner RED And Mount

The compiler owner captured the intended four E0107 errors: existing TestApp took no const parameters. A separate E0252 duplicate declarations import was also present and is assigned to Mutation; no tests executed. The raw output is `🧪️member-plugin-const-owner-red-r1-2026-08-28.md`. After source release, the actual fixture became `TestApp<const RETAINED: bool = false>`. Existing callsites explicitly selectfalse where inference would be ambiguous. Its clipboard helper is generalized only enough to keep the exact owner type. The new true mode's factory, payload builder and scalar artifact preparation are now mounted; the false mode remains unregistered and has no proof rows.

The exact next selector is `checkpoint_restart_` (two tests). The const-owner law should validate the distinct compiler owner and one declared tool. The transient law intentionally still calls the original unconditional terminal disposer and should produce a genuine semantic RED before switching to the new exact store-retirement implementation. The complete checkpoint/restore test still has its known old direct-dispatch tail and is not selected in that gate. No GREEN or complete checkpoint result is claimed from this source mount.

Read-through before dispatch caught and corrected two doubled associated const paths introduced by the explicit-false callsite edit. The compiler owner confirmed no dispatch had occurred. The scoped malformed-path search and diff whitespace check are clean. Mutation separately released its duplicate-import repair; the next native snapshot still requires the coordinated compiler hold.

The subsequent full checkpoint tail must decode the actual restored command, construct the true-mode registry-backed app, and drive the real maintenance/publication/result-ACK path. The app remains structurally outside the fallible test body. It must close incrementally before any collected error is asserted, preserving the primary error instead of triggering an unrelated ArtifactStore Drop abort. Expected publication and terminal receipt counts come from the neutral fixture. This remains a native fixture requeue/dispatch proof, not a guest instance restore or composition-admission proof.

## Actual R2/R3 And Complete Tail Source

R2 executed both tests: const-owner1PASS and transient1FAIL,522unselected,.232s. The transient failure was precisely its first assertion: the old disposer returned terminaltrue for the actual owned store, expectedfalse. There was no secondary abort. R3 selected the new disposer only for TestApp<true> and ran2PASS/522unselected,.087s. Directly read raw `🧪️member-plugin-restart-two-r3-2026-08-28.md` includes the intentionally caught panic after the original store entered its structural retirement owner, followed by successful remaining grant/release/foreign-root assertions. The false/default mode is unchanged. These are exact scalar fixture retirement laws, not generic transient-store, composition-resident or strict callback timing proof.

After R3 release, the complete checkpoint tail now decodes the actual restored bytes and passes that command and restored metadata to a fresh registry-backed TestApp<true>. The helper retains the original app outside its fallible async body. It validates the actual public controller/owner/tool/schema contract, dispatches through the registered job, drives maintenance/publication with1item/4096bytes, acknowledges each exact returned token, consumes the actual full UI scope, and waits for operation retirement. It then drives the real app close before asserting the collected result or publication counts. It reuses the existing100000-turn native fixture bound; no production helper, scheduler, quota or timeout was raised.

The job's raw-input page-count advancement is only this exact fixture's retained-owner/progress behavior. It does not decode those pages and is not evidence of bounded raw-input decoding or physical byte work. The actual restored command is decoded by the cold fixture before dispatch. A nonterminal close remains an explicit failure with the original outcome/close Fault printed; no Drop bypass, forgotten app, fabricated terminal state or forced registry reset is used.

The neutral packet now also requires one UI result and one consumed full UI scope, in addition to one artifact result and one terminal receipt. Actual Nx exec session5709 exited0: strict Ajv and Lodash scalar-publication/UI/terminal model5 passed. This is independent fixture/model validation, not native publication execution. The complete native checkpoint selector remains queued: `checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume`. R3's two laws remain the separate regression selector `checkpoint_restart_`.

## Actual R4 Constructor Catalog RED And R5 Regression

R4 executed the complete checkpoint selector and failed0PASS/1FAIL/523unselected,.296s with SIGABRT. The first failure was construction's exact catalog validation: `generated_migrated=false`, while owner, controller, document schema, factory type, uniqueness and `typed_join` were correct. Its original TestCommand `OpBinary::TOOL_JOB_IDS` contained only `compositeEdit`, omitting the already-authored ApplyCountFromTask variant. Constructor unwind then hit the Interaction ArtifactStore strict Drop guard. The helper had not received the app yet; its later publication, collected-error and close logic did not execute. Raw `🧪️member-plugin-checkpoint-tail-r4-2026-08-28.md` and compiler report `📓️plugin-checkpoint-tail-r4-native-red-2026-08-28.md` preserve the actual output. R5 separately reran the prior two restart laws:2PASS/522unselected,.137s.

Root approved adding only `applyCountFromTask` to the actual fixture command roster and asserting the full two-entry roster from the neutral fixture. This does not grant the default TestApp any proof or change its BatchOnly classifications; the exact catalog intersection remains unchanged. The fixture/schema now declare `generatedToolIds`, and the existing const-owner test compares the actual OpBinary roster directly. Nx exec session49519 exited0 with strict Ajv and the seven-output Lodash model, including positive/empty migrated-roster intersections. Complete checkpoint rerun and the changed const-owner assertion are queued; no publication success is inferred.

## Actual R6 Complete Checkpoint GREEN

R6 executed the exact complete checkpoint test:1PASS/523unselected/.057s,Nx0. Its full report and captured tool output were read in `📓️plugin-checkpoint-tail-r6-native-green-2026-08-28.md`. Actual DEBUG: `outcome=Ok((1, 1, 1, 1, 7)), closed=true, close_fault=None`. Those values are the real Artifact result, UI result, full UI scope, terminal receipt and restored count; every result token was acknowledged through the existing API. The original app reached its actual close witness before outcome assertions. R7's changed const-owner/roster law subsequently passed1/523unselected/.014s; its complete captured output was read in `📓️plugin-restart-roster-r7-native-green-2026-08-28.md`, and all source holds were released.

This completes this scoped native awaited-task/restart fixture repair. It does not fix the generic R4 constructor-unwind owner loss, mount guest checkpoint execution, certify raw-input decoding or establish strict callback timing. Native pre-instantiation/return admission resumes separately in `📓️native-admission-next-packet-2026-08-28.md`; no larger Plugin suite is inferred.

Exact R4 follow-up model expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import assert from "node:assert/strict"; const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/"; const f=await Bun.file(p+"🧪️fixture.json").json(); const s=await Bun.file(p+"🧪️schema.json").json(); const v=new Ajv({strict:true,allErrors:true}).compile(s); assert(v(f),JSON.stringify(v.errors)); const emitted={artifact:[{value:f.checkpoint.restartValue}],ui:{kind:"full"}}; const pages=[...emitted.artifact.map(value=>({lane:"artifact",value})),{lane:"ui",value:emitted.ui},{lane:"terminal",value:"complete"}]; const counts=_.countBy(pages,"lane"); assert.equal(counts.artifact,f.restartAuthority.artifactPublications); assert.equal(counts.ui,f.restartAuthority.uiPublications); assert.equal(counts.terminal,f.restartAuthority.terminalReceipts); assert.equal(_.filter(pages,p=>p.lane==="ui"&&p.value.kind==="full").length,f.restartAuthority.uiScopes); assert.equal(_.last(emitted.artifact).value,f.checkpoint.restartValue); assert.equal(_.intersection(f.restartAuthority.generatedToolIds,[f.restartAuthority.tool]).length,f.restartAuthority.retainedProofs); assert.equal(_.intersection(f.restartAuthority.generatedToolIds,[]).length,f.restartAuthority.defaultProofs); console.log("[DEBUG] Restart completion strict Ajv PASS; Lodash scalar-publication/UI/terminal model7 PASS; native path not executed.");
```

Exact model expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import assert from "node:assert/strict"; const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/"; const f=await Bun.file(p+"🧪️fixture.json").json(); const s=await Bun.file(p+"🧪️schema.json").json(); const v=new Ajv({strict:true,allErrors:true}).compile(s); assert(v(f),JSON.stringify(v.errors)); const emitted={artifact:[{value:f.checkpoint.restartValue}],ui:{kind:"full"}}; const pages=[...emitted.artifact.map(value=>({lane:"artifact",value})),{lane:"ui",value:emitted.ui},{lane:"terminal",value:"complete"}]; const counts=_.countBy(pages,"lane"); assert.equal(counts.artifact,f.restartAuthority.artifactPublications); assert.equal(counts.ui,f.restartAuthority.uiPublications); assert.equal(counts.terminal,f.restartAuthority.terminalReceipts); assert.equal(_.filter(pages,p=>p.lane==="ui"&&p.value.kind==="full").length,f.restartAuthority.uiScopes); assert.equal(_.last(emitted.artifact).value,f.checkpoint.restartValue); console.log("[DEBUG] Restart completion strict Ajv PASS; Lodash scalar-publication/UI/terminal model5 PASS; native path not executed.");
```

The extended neutral fixture oracle ran through canonical Nx exec in session33199 and exited0. Strict Ajv passed; Lodash independently checked the declared registration/cancellation sets and empty-versus-single-row intersections (four model outputs). This is set algebra, not proof registration, runtime factory execution or private owner identity. Exact expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import assert from "node:assert/strict"; const p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/"; const f=await Bun.file(p+"🧪️fixture.json").json(); const s=await Bun.file(p+"🧪️schema.json").json(); const a=new Ajv({strict:true,allErrors:true}); const v=a.compile(s); assert(v(f),JSON.stringify(v.errors)); assert.equal(_.uniq(_.times(f.registration.attempts,()=>"s.test.subset-macro/1/derived")).length,f.registration.registeredEntries); assert.equal(_.difference(_.times(f.checkpoint.parkedRequests,()=>f.checkpoint.instance),[f.checkpoint.instance]).length,f.checkpoint.requestsAfterCancel); const declared=[f.restartAuthority.tool]; assert.equal(_.intersection([],declared).length,f.restartAuthority.defaultProofs); assert.equal(_.intersection([f.restartAuthority.tool],declared).length,f.restartAuthority.retainedProofs); console.log("[DEBUG] Awaited completion strict Ajv PASS; Lodash registration/cancellation/exact-row model4 PASS; native factory/disposer not executed.");
```
