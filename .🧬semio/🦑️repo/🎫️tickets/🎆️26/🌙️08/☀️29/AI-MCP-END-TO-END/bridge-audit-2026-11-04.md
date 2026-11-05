# Frontend Bridge Audit — 2026-11-04

Read-only exploration of the semio OS MCP frontend, separate from repo infrastructure. Root, products and OS AGENTS instructions read. No source edits or tests run.

## Existing Integration

The MCP module is 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp. Its README describes action prepare/invoke, transactions, artifacts, history, inference and shell tools, with local bridge rendezvous. Frontend 📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge already handles real binary frames, approval lifecycles, presence, appCommand replies and instance census updates. AgentBridge/🔸️part2/🟦️.tsx routes appCommand at lines 144–152. AgentBridge/🟢️shared/🟦️.tsx contains answerAgentAppCommand and explicit missing-route refusal, and options expose instances and onAppCommand.

ShellHost 🔒️identity-document-ports/🪪️identity-and-directory-state/🟦️.ts installs the bridge with current census and stable callback into agentArtifactRouteRef. Actual artifact handling is in ShellHost 🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts, useToolTabSelectionEffects, lines 130–230. It handles readHistory, pureCommand, transaction prepare/rollback/commit/undo/redo, cancellation refusal, readArtifact, exportMedia. Live target resolves primary and spawned plugin instances.

## Concrete Gaps and Candidate Fixes

1. Deferred plans are keyed only by txnId (lines 183, 189), store only JSON action/args, and are consumed against whichever live instance a later transactionCommit request names. A prepare on instance A followed by commit on B can execute A’s verb against B if gateway routing changes or malformed request reaches the shell. Bind plans to instance and plugin identity; reject mismatches without dispatch. This is the strongest isolated frontend task.
2. dispatchVerb (line 164) chooses controllerId from resolved target but passes provenance.windowId from activeWindowIdRef rather than the target’s own window. onActionRef is a shared shell route, so a request explicitly addressed to an unfocused spawned editor needs a regression proving it modifies that editor while another program owns canvas. Verify downstream routing before concluding this is a defect.
3. pureCommand produces a deferred JSON plan rather than calling actual plugin preview. It validates declared action name only, so action_prepare currently does not prove action argument acceptance or calculate a true diff. transactionPrepare accepts the first document op only and does not validate JSON shape before acknowledging prepared. Multi-command atomicity and truthful dry-run semantics require an owned schema/route design.
4. cancel returns a budget.exceeded frame but does not abort pending dispatch/read/export promises; investigate propagation of cancellation to host/plugin operations.

## Strong Test Targets

Existing AgentBridge 🧪️tests/🚏️artifact-routes/🟦️.ts covers callback relay and explicit missing-route refusal, not actual ShellHost transaction semantics. Add route-level tests for cross-instance prepare/commit, malformed/extra ops, repeat commit, rollback, target closure and cancellation. Existing bridge tests cover tool-call lifecycle, handshake ladder, frame codecs, discovery. MCP 🧪️tests/🔄️end-to-end/🟦️.ts and main 🟦️.ts expose real-client harness and staged-plugin preflight; broad tests must prove installed component identity before measuring bridge behavior. A headless gateway journey alone cannot prove visible UI edits/navigation/history.
