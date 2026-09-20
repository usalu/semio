# WGPU Agent Invocation Cancellation

## Contract

The gateway owns tool invocation identity and terminal outcome. WGPU preserves the exact `invocationId` received in `AgentToolCall`, publishes it on the retained cancel action, and sends the same value in the existing tag-10 `AgentCancel` frame. An accepted request changes only a matching `running` conversation row to `cancelling`. This is an optimistic, nonterminal state: the gateway's later `AgentToolResult` remains the sole authority that settles the row to `ok` or `failed` and supplies its summary.

Cancellation is accepted only while the bridge reports `Open` and the id is nonempty. A disconnected request queues no frame and changes no row. As in React, a connected programmatic request can carry a nonempty id even if the bounded live transcript has already retired the row; only a matching running row changes state. The retained UI offers the control only for a running tool call, so cancelling and terminal rows cannot dispatch duplicates.

The existing bounded conversation window remains authoritative. Once a cancelled call is displaced beyond `AGENT_CONVERSATION_MAX_ENTRIES`, a late result is absorbed and does not recreate a call-less row or cancellation control.

## Shared Fixture and Independent React Oracle

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧫️fixtures/🛑️cancellation/🔣️.json` is the language-neutral sequence. It uses the deliberately nonordinal id `inv.actual:cancel/7` and records:

- the real inbound tool name and arguments;
- open acceptance and the exact outbound `agentCancel` payload;
- optimistic `cancelling` with no remaining cancel control;
- disconnected refusal with zero outbound frames;
- result-owned failed settlement and summary;
- the 200-entry retirement bound and no late-result resurrection.

The real React `useAgentBridge` test reads this fixture, drives the actual WebSocket hook, checks exact outbound identity, disconnected refusal, and result settlement. The registered React `AgentChatPanel` oracle also reads it, renders the actual panel through Testing Library, clicks the accessible control, checks the exact invocation id received by the callback, then rerenders the cancelling and terminal states and requires the control to be absent.

Focused registered oracle result: **1 passed, 4 skipped**.

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧱️elements/💬️AgentChatPanel/🧪️tests/🧩️component/🟦️.tsx' --silent=false --reporter=verbose --testNamePattern='shared neutral invocation lifecycle'
```

An initial direct invocation of the AgentBridge component test returned “No test files found” because that file is not in the renderer React target's registered include list. It performed no test. The oracle above uses the registered actual panel suite and passed.

## WGPU State and UI Flow

`AgentToolCallState` now includes `Cancelling`. `AgentBridgeState::cancel_tool_call` owns acceptance, queues `ShellToGateway::AgentCancel`, and updates only the matching running row. Existing in-place result settlement handles both running and cancelling rows without a second conversation entry.

The retained chat row publishes `framework.chat.cancel.<invocationId>` with the `framework/cancelToolCall` action and the exact invocation id in action args. The action is ghost-styled, carries the square stop icon, and uses localized English/German copy. Shell dispatch forwards the id to the bridge state. Rebuilding the panel immediately publishes the `cancelling` state attribute and translated label and omits the control. The result later publishes the ordinary terminal state and summary.

The host law crosses the actual retained route: inbound call frame → `build_agent_chat_ui` → exact button action → `dispatch_action` → encoded outbox. It requires one exact `AgentCancel`, no duplicate control while cancelling, result-owned terminal state, and disconnected refusal that preserves `Running`.

The WGPU consumer laws additionally load the neutral fixture directly, verify the open/closed/result sequence, and fill the bounded transcript to prove retirement and late-result absorption.

## Validation

- Shared fixture parses as JSON.
- `rustfmt --edition 2021 --emit stdout` parser checks pass for AgentBridge production and unit laws, Shell production, and the Shell host law.
- Registered React/Testing Library oracle: **1 passed, 4 skipped**.
- No Cargo, native, wasm, browser generator, or runtime build was started in this packet.
- Root-owned native verification remains required for the AgentBridge WGPU unit tests and `wgpu-panel-anchor-model` Shell laws. Native12 and activation9 began before this source batch and therefore are not receipts for it.

## Adjacent Async Follow-up

`📓️astra-sol-scene-transfer.md` now records the exact canonical Nx command for `mandatory_submit_linearizes_after_queue_ownership_is_released` and the queue-lock safety argument: selection moves work out under the lane mutex, job and maintenance execution occurs after the selector returns, timer callbacks execute after their wheel lock scope closes, and mandatory submission drops the queue guard before notifying workers.
