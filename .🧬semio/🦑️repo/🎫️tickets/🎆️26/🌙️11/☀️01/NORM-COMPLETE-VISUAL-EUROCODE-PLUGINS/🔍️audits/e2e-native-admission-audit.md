# Native Source Admission Write Failure Investigation

Read-only inspection, 2026-11-04. Session 52448 reached source admission after its ordinary capacity wait, then exited 1 with EPERM syscall write and no path before native compilation (root execution report).

## Established Source Facts

The cargo launcher enters `withRepositoryCompilerRuntimeEntriesV1` before native store work. Its runtime helper capture calls `captureCompilerBuildSourceClosureV1(root,entry,original,control)` without the optional environment argument. Therefore that path does not enable the Nx authorizer durable source memo. The underlying capture reads imports/resources; its optional memo writer catches and ignores storage failures. A source memo write failure cannot explain an uncaught EPERM here.

The optional memo location is controlled on Linux by an absolute `XDG_CACHE_HOME`, with final root `<XDG_CACHE_HOME>/semio-repo/source-memo`. On macOS it is fixed under the user Library/Caches; Windows uses LOCALAPPDATA. This is a legitimate source memo relocation, not a runtime/dependency-store relocation. However, the actual compiler runtime capture does not pass environment and therefore does not use this override. Setting it is not an evidenced fix for session 52448.

After capture, the runtime scope serializes the admitted helper closure and dispatches it through owned execution stdin; its child environment already specifies `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. This is existing authored behavior, not an admission bypass. Owned execution creates a pipe and publishes the serialized input using `stdin.end(input, callback)`. The same module forwards output through writable streams. Invocation progress also writes stderr. A pathless EPERM with syscall write is consistent with one of these pipe/output writes, but the failing write site is not established without the exception stack.

## Next Safe Concrete Diagnostic

Keep the existing runtime/dependency store and all source/capacity gates. Reproduce the same repo cargo command with `SEMIO_REPO_PROFILE=1` and redirect stdout/stderr to ordinary files inside the ticket generated folder, preserving the complete stack and exit code. Profiling is a documented observation-only switch; it records source.capture/runtime-entry phase timing. If redirection itself resolves the failure, output-pipe handling becomes the leading cause; if not, inspect the exact write stack and final completed phase before changing cache configuration. No writable state/cache override has been proven to repair the actual failure. Do not disable source admission or replace the registered runtime loader.

Sources inspected: framework process native-build source/admission; repo library build/source/closure/compiler; build/source/closure/command/runtime; caching/bootstrap/tools/authorizers/memo; process owned-execution/capture; process invocation progress; repo build phase profiling. No implementation or runtime configuration was changed.

## Profile Evidence Superseding the Initial Stage Assumption

The redirected profile run exited 1 after 337 seconds of outer capacity wait and about 22 seconds inside door.script, including an inner capacity wait. Source.capture completed in 122.6 ms and source.served explicitly reported memo. Thus the failure occurs at the outer captured cargo-command admission, before the inner native-helper runtime scope described above. The earlier omission-of-environment observation is true for the inner helper caller but does not describe the failing outer stage.

The outer `executeCapturedBuildSourceV1` in build/source/closure/command passes environment into captureCompilerBuildSourceClosureV1, so it uses the optional authorizer memo. That memo was successfully replayed, not refused. After capture and per-file progress, this function serializes the closure and calls runOwnedCommand with auxiliaryInput. Owned execution creates a fourth stdio entry (FD 3), then publishOwnedAuxiliaryInputV1 invokes `child.stdio[3].end(text, utf8, callback)`. Its error listener/callback retains the original system error and later rethrows it. The Bun stream loader consumes that descriptor using fstatSync(3)/readSync(3). This is the strongest specific FD writer candidate for a pathless EPERM after successful capture.

No nx.runtime-entry.capture, publication or command profile phase appears, supporting that native storage and native compilation were never reached. Redirection did not solve the failure. Output/progress writes also remain technically possible because the exception has no stack; the log alone cannot prove FD 3. The next safe source-level check is the existing owned-auxiliary real-child regression and its Bun/Linux expectations, then a narrowly scoped diagnostic that records FD and operation at the publication error boundary. Do not relocate or disable stores/admission on this evidence. No additional cargo command was launched during this review.

## Agent Session State Override

The authored process agent-session schema declares `SEMIO_AGENT_SESSION_DIRECTORY` as its registry override. `agentSessionRegistryRootV1` returns that override directly. A legitimate absolute value is the ticket generated agent-sessions directory; this changes only session records/seen/token/lock storage, not XDG, Bun runtime, dependencies, compiler stores or capacity gates. It must be inherited consistently by fleet commands if used.

The current-session resolver first reads the requested SEMIO_AGENT_SESSION address from that registry. If absent, it derives the current real client/session identity and opens it normally; it does not reconstruct the old registry capability. `adoptAgentSessionV1` performs that resolution strictly and publishes the actual resolved address to its child environment. Therefore a new registry may change the derived session address; do not assume the old address attaches, copy token files, or forge records. Non-strict currentAgentSession explicitly falls back to a derived in-process session on read-only state, explaining why the warning can coexist with continued work.

Bounded inspection of the new native-context log found only capacity waiting (latest 44 s, stdout empty), no new writer diagnostic at that observation. No cargo or additional diagnostic process was launched.
