# Bazel Argument And Configuration Review

Read-only current Source review; no protected SDK/store access or runtime execution.

## Material Argument Defect

Actual own Nx adapter `library/🔌️nx-plugin/📤️arguments/🟨️.mjs:26–29` combines options.args and __unparsed__ unconditionally, then uses per-command object forwardAllArgs override of top-level option before appending those literal arguments. Bazel backend `build/🔧️tools/🏗️bazel/📜️script.ts:284–285` computes buildExecutionArgumentsV1(execution,task.args) globally before command-specific forwarding. That helper in build/📥️caller/🟦️.ts:43 returns[] immediately for execution.forwardAllArgs===false.

Admitted target example: options.forwardAllArgs=false, options.args=["base"], commands=[{command:"bun ./📜️script.ts",forwardAllArgs:true}], caller arguments=["caller"]. Own Nx source appends base,caller for the object operation; Bazel appends neither because args are already discarded. BuildExecutionV1 explicitly admits command objects with individual flags. This is a Source-proven backend discrepancy; runtime output was not executed by reviewer.

Recommendation: determine effective operation forwarding before deriving literal argument vector, preserving native ownerArguments behavior and default array validation. Add language-neutral vector plus actual same-wire Nx/Bazel oracle regression. Communicated to Bazel agent and parent; no production mutation by reviewer.

## Configuration Change Review

Current taskIdentity chooses requested configuration if available, otherwise target.defaultConfiguration if available, while rejecting explicitly named unavailable colon configurations. include now carries inherited=value.split(":")[2]??configuration??identity.configuration into dependency identity/add recursion. This preserves original requested mode through a middle target's different fallback and propagates root default when no explicit mode exists. No material Source defect identified in this narrow change. Four new Nx oracle rows and ongoing genuine backend gate are owned by Bazel agent; no pass result inferred here.
