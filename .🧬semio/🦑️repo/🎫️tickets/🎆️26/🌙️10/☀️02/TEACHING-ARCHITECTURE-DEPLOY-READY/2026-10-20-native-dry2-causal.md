# Native Exec Dry2 Causal Review

## Concrete Finding And Repair Direction

The native exec dry-run is awaiting full repository graph preparation before its one-task dry plan can be printed. The current graph-selection function buildInvocationGraphSelectionV1 in 📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts:109 begins if args[0] is not run, return {kind:all}. This includes exec --projects=workspace --dry-run. Only after normal graph load, selected Rust preparation and graph artifact pinning does the Nx branch in 🏗️build/📜️script.ts apply the exec dry-run special case and call buildExecPlanV1 for workspace.

That exact control flow explains why this one-owner observation dry-run computes global command, playground/native, project assembly, source-import and package closures. It is a concrete excessive-admission problem, not evidence of a stuck observer: no observer has run and no workspace:repo-exec plan appears.

A clean repair is selected-owner exec admission in the existing graph selection/schema boundary: resolve project selectors/exclusions against the declared owner registry, admit only those owners plus genuinely required command/Source closure, retain the exact forwarded command and original authority, and emit the same portable dry plan. No arbitrary bypass, fake envelope, reduced physical guard, ticket-specific compatibility wrapper or raw tool route is needed. Current selection schema already has all/tasks. Exact-owner exec can reuse tasks after the invocation-owned synthetic repo-exec target is admitted before owner closure; current code adds that target only after graph load. Schema-first coverage is required, but a new selector kind is not established as necessary. A neutral test should prove unrelated owner declarations/import failures and expensive native/playground preparation are not consumed for one workspace exec while selected command Source/resource guards remain enforced; compare project selection/task plan with an independent oracle. No repair was implemented or test run here.

## Bounded Actual Log Evidence

Read generated/2026-10-20-native-process-inventory-third-dry2.log once through a descriptor, capped at600000bytes. Snapshot441320bytes,4526newline records, mtime2026-10-20T14:54:24.493Z, SHA256296ee8f5160a645f083ebb55959c06a23725109d6aaf2a0370fa0494554b0e29. Descriptor size/mtime were unchanged before/after read. A later metadata-only stat still matched. This is newer than Root's earlier440795byte snapshot; Root owns actual handle63416 and terminal/poll state.

Simple bounded marker census:31resource-directory-admission:v1 rows; no lease-queue, error, graph-complete, repo-exec or workspace dry-plan marker. Earlier build:commands completes at line867. Later package/project-assembly phases are interleaved with command-source-imports. In this snapshot, the final assembly row is line4445, followed by59command-source-imports rows through4504, then21repository:command resource-admission rows4505–4525. Those later import rows belong to project/target closure processing, not the completed initial build:commands stage. Root's separately parsed census has a different valid-row policy; phase names/counts alone do not name the current project or JavaScript site.

The final rows describe shared directory admission with elapsedMs0–1. These do not identify a lock holder, directory path, requested outer resource, project, current JavaScript site, terminal state or useful graph progress.

## Exact Lease Emitter And Continuation

The reserved name comes from retirementSchema.properties.admissionResource in framework process 🔒️leases/🟦️.ts:231. directoryAdmission at237 replaces the caller resource with that reserved name and invokes acquireUngatedResourceLease; ordinary acquireResourceLeaseBody at244–249 acquires this shared admission gate before the actual requested resource.

In current Source acquireUngatedResourceLease:183–228, waitProgress is reached only after catching SQLITE_BUSY or SQLite error code5 from initialization/transaction/identity acquisition, rolling back, checking remaining authority and constructing {resource,mode,elapsedMs}. Thus the emitted row proves a busy shared admission attempt at that emission point. It does not prove that attempt is still pending when the log is read. The successful acquisition path returns without an acquire-complete row.

waitProgress:58–69 awaits waitForLeaseContinuation around control.publish and then control.yieldContinuation. waitForLeaseContinuation:47–56 bounds the callback against the same signal/deadline and cleans up its timer/listener. After progress, acquireUngatedResourceLease sets next progress to elapsed+1000, awaits a cancellable1–40ms delay and retries. A continuing single busy wait with a responsive event loop should later emit growing elapsed times; repeated0–1ms rows can also be distinct fresh attempts. The current marker lacks request identity, so the sequence cannot distinguish these cases.

The exact issued control callback is framework process 🧭️routing/📥️invocation/🏃️process/🟦️.ts:170: publish delegates to invocation.control.publish, while yield delegates to invocation.control.yieldContinuation with unchanged remaining checks. The real process invocation at142 uses synchronous presentation.publish and setImmediate(done). Its presenter writes process.stderr.write and does not await stream drain or any accepted-child settlement. No resource reacquisition or child-join self-wait was found in these exact callbacks. A callback self-deadlock is unsupported by the inspected chain.

## Reachable Later Package/Assembly Boundaries

1. 🏗️build/🕸️graph/🟦️.ts inferBuildGraphV1 calls the revisioned owner discoverBuildProjectsV1. Because exec selects all, the complete manifest roster is admitted.
2. 📥️inference/📦️project-nodes/🟨️.mjs143–157 calls observeProjectInferenceV1 for each project, runs projectWithDefaults, and waits Promise.all of up to8project workers. The visible progress labels omit owner identity.
3. 🧱️project-assembly/🟨️.mjs184/203/212/219/222/229 awaits targetScriptClosure/nativeTargetCommandInputs/genericTargetCommandInputs and related target Source closure work.
4. 🧭️script-inputs/🟨️.mjs119–167 relativeScriptInputs recursively visits literal imports/runtime refs/URL resources and validates memo observations. Its checkpoint emits build:command-source-imports. genericTargetCommandInputs233–252 then iterates the resulting closure, resolves package names against the lock graph and emits build:command-source-packages.
5. yieldInferenceV1:269–275 invokes advanceScriptInvocation at most every200ms; that function publishes and genuinely yields the original setImmediate continuation. Cached observation validation also calls these checkpoints. A marker is not a stack sample.
6. After project compute, observeInferenceComputationV1 in 👁️observation/🟨️.mjs583–618 can still validate/retain Source facts, synchronously flush record data, build consistency/image data, await Brotli compression, and publish an optional graph record. These later phases have no owner-specific before/after markers. The outer graph body then waits all workers and later graph sealing/publication/admission. The log cannot select one of those awaits as the current one.

No inspected memo function waits a pending computation on itself: memoizedObservedAsyncComputationV1 stores delivered values/observations after compute and awaits explicit checkpoints on cache reuse. This does not rule out other bugs outside the inspected boundaries, but it rules out inventing a promise self-cycle in this function from the labels alone.

## Causal Limit And Next Owned Evidence

The current log proves expensive global inference remains ahead of the dry plan. It does not prove a permanent lock, the identity of an exclusive directory-admission holder, a stalled event loop, or a specific current JS await. Do not cancel63416, clear caches, alter peer Source or launch a duplicate command based on that inference.

If Root needs exact live contention attribution before the selection repair, the existing activity contract needs bounded operation identity: directory identity/digest, original requested resource, unique request token and acquire/continuation transitions, with no argv/environment/Source output. Existing0–1ms rows cannot supply it retroactively. Any diagnostic change must be separately Source-admitted and neutral-tested; this review did not instrument the live run.

## Source Read Identity

These are current filesystem hashes of inspected files, not proof of the exact Source bytes already loaded by the live process:

~~~json
{
  "🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts": "2e8c2a24d3727da0386a32ba7572de9312c6cb93c140f76061e5c5deb2f29123",
  "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts": "f373ddb25a1544457a0d223ac45053692b3bbe57ffb1e76d749f1fc1ee51e617",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs": "e414d587d81013f079efc22f2111a8da6b9948dc5f92f1486261193466e31dfc",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs": "fb5eb4fb4eac56c9799c891be2f06b37d3be9406e1b381867da396bf68bf477c",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs": "644d4e5f02a1921cbedb354a7fd08ea7dc0eeda4d1181dcac87844c2d8814b2b",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts": "b3bea650eafcd99c3b89b3a77c1eb5f38902e4ab0939b37d314e84bbaa8f2573"
}
~~~


## Selected Run Admission And Concurrent Exclusive Scopes

Current Root-owned lanes are global exec dry2 handle63416, selected course RED handle9854, and the early repo-mcp Linux build handle86151. This report does not launch, poll, cancel, or claim a terminal result for any lane. Root’s new selected/build invocations supersede the earlier sole-command wording.

The exact supported next test argv is `repo run @teaching/architecture-course:test --configuration=release -- quick`. `🧭️invocation/🟦️.ts:170,184–192` resolves the caller then calls `buildInvocationGraphSelectionV1`; `🕸️graph/🚪️admission/🟦️.ts:109–113` produces `{kind:tasks,tasks:[{project:@teaching/architecture-course,target:test,configuration:release}]}` and stops option parsing at `--`. The literal quick tail remains task arguments. In the course manifest the declared test target is repo:script, `bun ./📜️script.ts test`, forwardAllArgs:true, inputs:[default], cache:true, outputs:[], with no authored dependsOn. Course `📜️script.ts:65–66` resolves the quick level and calls the owned Vitest port.

This avoids global workspace discovery: `🕸️graph/🟦️.ts:322–324,368–369` closes selected owner facts through `closeBuildOwnerAdmissionV1` instead of the all-graph workspaceBuildFilesV1 fallback. It still admits the actual default inputs/imports, which are broad (course, proctor, course/pets/server/UI/framework Sources and declared Rust manifests). Selected is therefore bounded by real ownership, not necessarily cheap or Rust-free. No execution result or pass is inferred here.

| Scope | Global exec dry2 | Selected test | Actual conflict |
|---|---|---|---|
| Graph writer, held across callback | repository-graph-facts:<workspace> | repository-graph-facts:<workspace>:shard:N | Different resource keys |
| Graph publication state, held across callback | build-graph-state:<graph-root> | build-graph-state:<graph-root>:selected:N | Different resource keys |
| Graph optional ledger opening | repository-graph-optional-ledger:<workspace>, exclusive | Same | Short serialized open; released before graph callback |
| Cache charge reservation | repository-cache-charge:<image-root>, exclusive | Same image may share key | Short reservation/receipt phase; released before store body |
| Optional SQLite publication gate | Same optional-gate.sqlite | Same | Exclusive only around synchronous publication/retirement; connections are not lifetime transactions |
| Image/resource directory admission | Shared normal readers; conditional exclusive creation/maintenance | Same | Shared paths alone do not prove exclusive conflict; maintenance may refuse/defer |

Exact lifetime evidence: graph `🟦️.ts:118–124`; input storage `🟨️.mjs:59–60,114,124–151`; cache store `🟦️.ts:41–45`; input storage `📒️ledger/🟨️.mjs` withGate/transaction/publish releases each SQLite transaction in finally/commit. Opening the ledger releases its logical exclusive lease before returning the live ledger object. Optional low-headroom maintenance is conditional; this report has no observed proof that it is active.

Writer shard count is64. For the registry snapshot SHA25682b09906ace0b94a55d6993bc714d5f1a08b54cebea8a8515c5b8e3f6b10642e, release test scope6586d24fa07285ae26c08ec19392efad02264c0b6ceb4afa35f99e9f060fc3f4 maps to shard5. The candidate release ci-browser-prepare scopebfea237c09e3c227a6116c4b612b4dbc19c75686190abcb561b8595331ec4b0d maps to shard44. These are Source-derived candidates, not a claim of handle9854’s argv. Different selected tasks can collide on one of64shards; the global key does not equal any selected key. Root may validly run the selected test beside global dry2 under these existing guards; no requirement to cancel the global lane follows from this closure.

## Minimal Exact-Owner Exec Gap

The existing selection schema can represent `{kind:tasks,tasks:[{project:workspace,target:repo-exec}]}`. It cannot work merely by returning that object today: owner discovery `📥️inference/🔎️owner-discovery/🟨️.mjs:175` requires the target in the admitted owner view, and admission `🕸️graph/🚪️admission/🟦️.ts:86` rejects an unknown target. `🧭️selection/🟦️.ts:102–106` creates repo-exec only after graph load. Native Nx dry exec itself branches even later at build `📜️script.ts:194–197`, after graph admission, selected Rust preparation and graph pinning.

A minimal repair can reuse tasks for explicit canonical owner identity, with invocation-owned synthetic repo-exec facts installed before closure, using the same command/cwd/forwarding/cache:false/outputs:[] semantics already defined at selection:105. It must keep original command/envelope authority and command Source observations. Do not add an authored workspace target or invent a generic bypass. Full wildcard/tag selectors are a separate question because the registry has names/roots but no tags; exact workspace does not require that broader schema.

Test first: neutral fixtures for exact workspace exec with unrelated owner failure; unknown owner/target rejection; command-tail isolation after --; original authority/Source changes refused; dry plan parity and portable cwd/forwarded argv. Compare project matching with an independent existing third-party matcher and emitted plan with the already-owned task plan oracle. Existing all/run behavior stays covered. No Source change or new test was made by this review.
