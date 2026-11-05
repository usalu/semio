# Source Admission Auxiliary Publication Diagnosis

## Established Boundary

`🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` line 97 starts the registered child with stdio stdin/stdout/stderr plus an extra `pipe` when auxiliaryInput is present. Line 164 sends immutable source DATA by `publishOwnedAuxiliaryInputV1(child.stdio[3], text, ...)`. The auxiliary implementation calls `.end(text,"utf8",callback)` and records the first failure separately from child completion. The reported descriptor=3 phase=end-callback code=EPERM syscall=write therefore identifies refusal to write the parent-owned auxiliary process channel. It is not a path-specific file publication failure; relocating ordinary output directories is not an evidenced repair.

`🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts` executeCapturedBuildSourceV1 captures and authenticates the closure before calling this transport; the child receives `--eval <stream-loader> -- <digest>`, with the snapshot on descriptor 3 and caller stdin preserved. `🏗️build/🏛️sealed/📥️execution/🟦️.ts` uses the same auxiliary transport for the stored implementation. Switching live/sealed or build backend is not an identified solution. The code fails closed and terminates the exact owned child on publication failure, then throws its primary publication error.

## Cause Not Yet Established

EPERM/write alone does not distinguish host sandbox syscall restriction from Bun child-process/stream behavior. The recorded context intentionally preserves code and syscall but does not record the native OS operation behind Bun's stream implementation. No syscall trace or independent runtime reproduction is available from this audit, so labeling it a Bun bug or filesystem permission defect would overstate evidence. The existing neutral failure-context fixture includes EPERM/end-callback specifically to verify diagnostic preservation, not to demonstrate any particular runtime cause.

## Existing Supported Regression Route

The real auxiliary process fixture is `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/📜️script.ts`. Modes `inherit`, `large` and `large-stream` exercise real descriptor 3 publication, ordinary stdin, digest/byte count and bounded output through runOwnedCommand; cancellation/deadline/early-close modes cover cleanup. Its caller must provide an actual issued parent Script invocation. The captured-source stream test suite also uses that actual control and proves source loader identity and physical source binding.

Read-only diagnostic attempted the fixture through direct `repo <script> inherit`; it was correctly refused before child creation with `Original parent Script process invocation required`. Thus direct script ingress is a valid source-reading route but does not itself supply authorization required by this executable fixture. No synthetic envelope, forged control, raw subprocess bypass or source-admission skip was used. This failed diagnostic is not a transport reproduction.

To distinguish runtime versus environment while preserving authority, run the existing owned fixture through its declared test owner and observe the exact callback failure/native syscall in the same sandbox. A comparison under an already authorized equivalent host environment would be evidence; no such execution was requested here. Until that evidence exists there is no confirmed alternate supported invocation that completes the current build while retaining this source admission boundary.

No escalation, process-policy weakening or source edits were made during this read-only audit.

## Parent Registered-Tool Diagnostic

The retained `auxiliary-probe.ts` uses the public first-party tool-registry interface to start short Node/Bun diagnostic queries, without invoking a test/build tool or changing source admission. The default sandbox reproduced EPERM writing both ordinary child stdin and auxiliary descriptor 3, with both `pipe` and `overlapped` stream declarations. Both child programs returned status 0; Node produced no stdout, while Bun printed an empty line instead of the supplied marker. A separate synchronous registry query produced the marker under Bun and empty stdout under Node. Thus this failure is broader than descriptor 3 or one source loader, and exit 0 without expected output is demonstrably insufficient evidence in this environment.

`tool-facts.ts` confirmed the resolved Node executable is a real ELF binary at the pinned v24.15.0 path, rather than a text stub. These observations still do not identify the underlying OS/runtime cause. No speculative runner or pipe-mode change was applied. The diagnostic scripts remain ticket inputs; no generated logs or copied binaries were added.
