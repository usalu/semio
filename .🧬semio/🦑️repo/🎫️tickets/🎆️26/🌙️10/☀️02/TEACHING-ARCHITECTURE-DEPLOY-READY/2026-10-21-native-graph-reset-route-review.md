# Native Graph Reset Dispatch Route Review

Corrected current disposition: existing captured runtime already has an early closed recovery branch. No new dispatcher specialization is needed. My initial line95-only route conclusion omitted that branch and was incomplete; Cargo identified it and this full reread confirms it.

Native entry routes run to build/📜️script.ts repo owner, which retains normal capacity and captured runtime admission. Runtime18–19 calls readNativeRepoRecoveryPlanV1 and, on a valid plan, directly enters environment setup without opening diagnostics. Inside that callback23 calls runNativeRepoRecoveryV1 and returns before ensureBuildEnvironment43 or loadBuildInvocationGraph95. Environment setup8–19 only checks/project/restores invocation environment and executes body; it does not open Graph storage.

Recovery selector accepts only run plus fixed two-part target and closed allowlisted options, authenticates the actual owner manifest/dependencies against ResetOwner schema, then maps repo:cache-reset-graph to profile graph and fixed reset scriptRoot. It cannot select a caller path or arbitrary program. Executor checks phase belongs to incoming Original, constructs context with that same invocation/control, dynamically imports only existing reset script and calls runInactiveRepositoryCacheResetV1. No child Original, deadline reset, lease weakening or secondary retirement implementation is introduced.

Thus current Source supports the requested public reset before Graph GC; ordinary unrecognized requests retain generic graph semantics. Actual compiled frontend/currentSource capture, reset borrower admission, terminal outcome and postfences still require real executor evidence. Static route availability is not proof a reset completed. No command or effect performed.

Current correction pins:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts — 19596/f4c27f2136e22abdd69a6c7a55624450d15debde447f6e378a774dbd171696e4
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts — 6144/16e4f5c456b41ea6b36a10846034551d78b196946a0091573852733a5aca2b8e
