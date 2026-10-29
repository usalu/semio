# Negative Inspector Raw Capture And Observation Boundary

Raw input: 🗑️generated/refusals/missing-language.json from root-owned72515. Native status=1, signal=null, stdout bytes=0, stderr bytes=42626. Request SHA256=dc5ce99903347f1fc6512902e089db91b564d69d2b9cd3ba5e4c36076a476eb3. Source SHA256=fc2b208bc899cec9cba98ee08a2409885cb5abc081e0f0e109bd7200377b15cf.

The actual compiler reports Document language is required and a1.2919921875KiB log. Before/after parent-only observations are {"before":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3598963},"after":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3593634}}. These do not directly observe the child compiler inner signal. No structured refusal report was emitted to stdout.

Bounded secondary stack excerpt:

~~~text
OriginalAdmissionClosed: Original process admission closed - first failure: Print compiler failed (1)
      at closedV1 (/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:205:165)
      at admit (/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:221:223)
      at withIssuedResourceOperationV1 (/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:293:188)
      at inspectArtifactV1 (/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📥️inspection/🟦️.ts:30:47)
      at /home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts:32:859
      at /home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts:32:1504
      at /home/deck/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:289:568
~~~

This confirms an Original-admission consequence while artifact inspection/rechecking runs, naming the actual compiler refusal as the first failure. It does not establish whether the hidden capture log read succeeded, whether the exact primary retained its WeakMap identity, or which child terminal observation rejected the report: capturePrintCompilerRefusalV1 suppresses secondary read errors in its finally before rethrowing the exact primary. Core owns those source/lifetime questions.

## Lawful Ticket-Only Observation Route

The public runPrintCompilerRefusalInspectorV1 function explicitly accepts a compiled-probe runner port. A distinct diagnostic ticket child can call that unchanged inspector with a wrapper around the unchanged withCompiledPrintProbeV1. The wrapper can observe the provided execution.phaseControl signal/remaining clock immediately before and after the actual compile call, while rethrowing the same primary. The exact inner compiler fonts/material control and the refusal WeakMap are private; no lawful ticket-only accessor exposes them. They must not be guessed, reflected, monkey-patched or replaced.

To observe the genuine child ScriptInvocation as well, the wrapper can enter the existing withPrintStorageTaskV1 API before invoking the actual compiler and capture context.invocation from its accepted callback. Its actual Original control and the provided incoming phase can then be observed separately, without issuing a replacement Original or renewing its clocks. This introduces an additional real storage-operation boundary, so it is explicitly an instrumented diagnostic route, not an identical execution proof of the fixed production entry. Terminal observations would be bounded plain DATA written after failure, without creating new controls or reading storage after the Original closes; output failures must not replace the captured primary. The supplied execution/intention/body would still be passed unchanged to the actual compiler.

The current exact-entry replay remains unchanged, and no additional child job, product edit, forged capability, raw cache access or private ownership change was performed. Root approval/coordination determines whether this extra diagnostic boundary is needed; the existing raw capture may already suffice for source diagnosis.

## Actual Diagnostic24510

The root-owned one-case replay24510 emitted these bounded temporary diagnostic facts from the unchanged strict guard:

~~~text
[DEBUG] Print refusal capture-registered {"status":1,"logBytes":1323,"registered":true,"compilerCancelled":false,"compilerReason":"undefined"}
[DEBUG] Print refusal inspector-terminal {"owned":false,"diagnosticFound":false,"originalCancelled":false,"originalReason":"undefined","compilerReason":"undefined","remainingMilliseconds":3590659}
~~~

Raw joined child status=1, signal=null, stdout bytes=0. Captured parent observations={"before":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3599178},"after":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3595643}}. The facts establish that compiler diagnostic registration succeeded (1323bytes and un-aborted control), while the inspector terminal received a different primary which failed the exact OwnedCommandFailure identity test. Original cancellation and deadline exhaustion are not supported explanations. This observes primary replacement between registration and the inspector terminal; core/audit are tracing its exact storage/read/drain location and own the corrective source change. No cause-chain report fallback or guard weakening is accepted.

The replay input already supports SEMIO_THESIS_NEGATIVE_COUNT=6 using the same genuine retained rsLAcRv rendering grant, prepared stores and exact six neutral mutations. After the source correction and removal of temporary DEBUG instrumentation, root can run that existing input under the same1GiB accounted wrapper, with no positive recompilation or preparation. No script edits or additional job were performed here.

The complete raw joined capture is separately preserved as 🗑️generated/refusals/missing-language-24510.json before the mutable per-case filename can be overwritten by the six-case run. The reduced primary-identity-evidence.json is supplemental only.

## Actual Diagnostic19272 After Reader Settlement Fix

The complete raw capture is separately retained as 🗑️generated/refusals/missing-language-19272.json. Earlier24510 raw evidence remains unchanged. The strict classifier still rejects the first negative, but actual debug facts now show the terminal received the exact OwnedCommandFailure and found its registered diagnostic. The reader settlement correction therefore resolved the observed identity replacement.

~~~text
[DEBUG] Print refusal capture-registered {"status":1,"logBytes":1323,"registered":true,"compilerCancelled":false,"compilerReason":"undefined"}
[DEBUG] Print refusal inspector-terminal {"owned":true,"diagnosticFound":true,"originalCancelled":false,"originalReason":"undefined","compilerCancelled":true,"compilerReason":"OwnedCommandFailure: Print compiler failed (1)","remainingMilliseconds":3590899}
~~~

Raw status=1, signal=null, stdout bytes=0. Parent observations={"before":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3599195},"after":{"parentOriginalCancelled":false,"parentScratchCancelled":false,"parentRemainingMilliseconds":3595888}}. The original signal remains un-aborted with more than3590seconds; the compiler signal is aborted with the actual compiler primary as reason. Core is adjudicating the exact-primary failure fanout separately from genuine caller cancellation. No guard weakening, cause-chain fallback, source edit or extra job was performed here. Wait for the final terminal schema/tests before another strict closure or six-case replay.
