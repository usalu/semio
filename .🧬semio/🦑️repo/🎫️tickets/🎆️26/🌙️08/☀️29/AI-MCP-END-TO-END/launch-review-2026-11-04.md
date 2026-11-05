# MCP Launch Review — 2026-11-04

Read-only source review of MCP TypeScript composition, gateway, dev composition, root birth declarations and smoke output. No implementation files changed.

## Findings

1. **High: EOF does not cancel compiler-store preparation.** servePublishedMcpStdioV1 passes original control to withAdmittedCompilerStoreV1, while the gateway aborts a separate staging controller on EOF. serveWarmMcpStdio awaits binary before checking abort. Its outer finally awaits held, so pending admission/preparation remains live after EOF. Parent reproduced initialization succeeding followed by a hang. Use one owned cancellation boundary through admission, preparation and staging; test EOF with deliberately blocked preparation.

2. **High: Native EOF shutdown is unbounded.** clientClosed invokes stop, whose implementation only ends child stdin. The input-close handler finishes only without a child; otherwise it waits indefinitely for native exit. An unresponsive child retains both process and publication. Bound graceful EOF, then terminate/escalate, and test with an independent child ignoring EOF.

3. **Observed blocker fixed by parent during review:** generated smoke.json shows a warm initialize followed by Original parent Script process invocation required and exit 1. Cargo provider withCompilerStore calls receiveScriptProcessInvocation(environment,...), requiring the original envelope. Parent reports updating dev composition to scriptInvocationEnvironment(original,env). Rerun the actual launch; the captured smoke is not end-to-end success.

## Authority and Publication

Root birth declares explicit native/catalog descendants, validates matching native resource data and unions exact launches. Dev staging projects child environments under original invocation/control. Binary selection derives from admitted compiler publication and holds it through gateway completion. This fixes dist/build selection in principle; the cancellation findings prevent a reliably bounded lifetime. Actual native publication access still needs a successful configured launch after build.

Gateway correctly waits for client initialized, ignores late staging completion after EOF and catches synchronous backend launch errors. Catalog production remains explicit and progress uses stderr.

## Scope

Read AGENTS.md, launch-summary.md, launch-research.md, generated smoke.json and scoped diffs/source through repo tools exec --light. No native build or tests run by this reviewer. Parent owns implementation follow-up and ticket closure.
