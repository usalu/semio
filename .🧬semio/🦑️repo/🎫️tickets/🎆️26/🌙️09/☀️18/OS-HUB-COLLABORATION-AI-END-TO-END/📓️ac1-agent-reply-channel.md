# 📓️ AC1 — the agent talks back: a free-text reply channel in `AgentChatPanel`

Slice AC1 of ticket 26/09/18 (`OS-HUB-COLLABORATION-AI-END-TO-END`), closing G19 gap #1
(+ #9, #10). Started 2026-09-22.

## 0. Headline

The agent can talk back. A new bridge frame (`GatewayToShell::AgentReply`, tag 10), its three
codec sites in Rust + the TS twin + two fixture rows, a 28th MCP tool (`conversation_reply`) gated
by a new `conversation.write` scope, a new `AgentConversationEntry` kind (`agentMessage`) and one
render branch in `AgentChatPanel` — plus the reverse direction, where a human turn typed in the
panel now PUSHES `notifications/resources/updated` for `semio://ui/agent-messages` instead of
waiting for the agent's next poll.

Measured, not asserted: `cargo check -p semio-framework-os-mcp --all-targets` green (§4), 11/11
`AgentChatPanel` vitest laws and 58/58 `AgentBridge` vitest laws green (§4), and §5's live run.

## 1. What was there before (measured)

G19 gap #1, re-read in source on 2026-09-22 before touching anything:

| site | before |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` `GatewayToShell` | 10 variants, tags 0–9, none of them free text |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx` `AgentConversationEntry` | `userMessage \| toolCall \| approval` |
| `tools/list` | 27 tools; nothing an agent could use to publish prose |
| the human's own turn | real (`ShellToGateway::AgentMessage` → per-connection `agent_inbox` → `semio://ui/agent-messages`), but **poll-only**: the resource is read-once and nothing told a subscribed client it had grown |

So a user who typed "widen that wall to 300" and got a clarifying question back saw **nothing at
all** in the one surface built for exactly that.

## 2. The design — schema first

**The frame.** `GatewayToShell::AgentReply { reply_id, in_reply_to: Option<String>, text, complete }`,
tag 10, appended after `AgentToolResult` (tag order is the wire contract, so a new variant only ever
goes last).

* `reply_id` identifies the TURN, not the chunk: every chunk of one turn repeats it, so a shell
  appends to one row instead of stacking rows. A client that mints none gets `rep_<n>` from the
  gateway's own counter.
* `in_reply_to` is the `message_id` of the `ShellToGateway::AgentMessage` this answers, or `None`
  for a turn the agent opened itself. `Option`, not a sentinel string.
* `complete` marks the last chunk — the streaming marker. A shell can say "still writing" without
  a heuristic, and never has to guess when a turn ended.
* `text` is **locale-agnostic**: it is the agent's own words. The only translated strings on the
  surface are the role noun and the state word around them (§3).
* Truncated through the existing `truncate_conversation_text` (`AGENT_CONVERSATION_MAX_TEXT` =
  2 048 bytes, cut on a char boundary and marked `…`), so a megabyte turn cannot starve the bounded
  bridge outbox.

**The scope — decided and documented.** A new MCP scope `conversation.write`, expanding to one new
`kernel::CapabilityId` `shell.converse`, rather than reusing `ui.control`. Reasoning, recorded at
the table itself (`🛡️policy/🦀️.rs:44-51`): writing prose changes no shell state, opens no window
and navigates nothing, so all four of `ui.control`'s grants (`shell.control`, `ui.window`,
`ui.dialog`, `shell.navigate`) are the wrong authority for it; but it does write into a surface a
human reads, so `ui.observe` is too weak. Its own row is the honest shape, and it means a read-only
agent granted `ui.observe,conversation.write` can still answer the human without being handed the
right to move their windows.

**The tool name.** `conversation_reply` — `<noun>_<verb>`, the grammar all 27 existing tools use
(`capabilities_search`, `artifact_open`, `action_invoke`, `history_undo`, `ui_focus`, `job_get`).
Capability id `conversation.reply`, `ToolExposure::Direct`, `CapabilityAudience::Agent`,
`CapabilityOwner::Gateway`, alongside `ui.focus`/`ui.reveal` in `ui_capabilities()`.

**No double-printing.** `tools/call` publishes an `AgentToolCall`/`AgentToolResult` pair for every
tool. For `conversation_reply` that pair would carry the same sentence as its `arguments` and print
every turn twice. `SELF_PUBLISHING_CONVERSATION_TOOLS` (`🧵️bridge/🦀️.rs`) is the DECLARED list of
tools whose own result is already a conversation frame, and `handle_tools_call` skips the pair for
them. The audit sink still records the call — the list governs the shell projection only.

**The reverse direction.** The existing path is real, not a dead end: the panel's composer sends
`ShellToGateway::AgentMessage`, the gateway parks it in the per-connection `agent_inbox`, and
`semio://ui/agent-messages` drains it read-once. What it lacked was a push — the resource's own doc
comment said "MCP has no push", which is not true of this server (`"resources": {"subscribe": true}`
is real and `📣️notify` already fans `notifications/resources/updated` out per URI). AC1 adds
`notify::agent_messages_changed()` and calls it from `BridgeHandle::record` (outside the
connections lock) when the inbox grows, so a subscribed MCP client learns of the question the moment
it is asked. No elicitation/prompt was needed: the channel existed, it just had a poll-interval
latency floor.

## 3. What landed (file + line)

All paths under `/Users/ueli/Documents/semio`.

| file | what |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` | `GatewayToShell::AgentReply` variant + `SELF_PUBLISHING_CONVERSATION_TOOLS`; arms in all FOUR Rust encode paths (`encoded_len`, `encode`, `copy_encoded_page`, `BridgeEncodedFrame::encode`) and in `decode`; `inbox_grew` → `notify::agent_messages_changed()` in `BridgeHandle::record` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🟦️.ts` | the TS twin: type member, `encodeGatewayToShell` case, `decodeGatewayToShell` tag 10 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json` | 2 new `gateway_to_shell` rows (a streaming chunk with `inReplyTo`, a final chunk without), hex computed from the wire rules and asserted by BOTH codecs |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs` | `conversation_reply_input_shape/_schema`, `conversation_reply_output_shape/_schema`, two rows in `schemas()` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️.rs` | `("conversation.write", &["shell.converse"])` in `MCP_SCOPE_TABLE` with the rationale |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️conformance/🦀️.rs` | `shell.converse` in `KNOWN_EXACT_SCOPES` (12 → 13) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs` | `conversation_reply_capability()`, `conversation_reply_handler()`, `register_conversation_tools()`, `NEXT_REPLY_ID` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` | `GATEWAY_TOOL_NAMES` 27 → 28; `register_conversation_tools` call in `build_tool_registry` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs` | `handle_tools_call` skips the tool-call pair for a self-publishing tool |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🦀️.rs` | `agent_messages_changed()` |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx` | `agentMessage` entry kind; the `agentReply` frame branch (append-into-one-row); `chat.agentRole`/`chat.replyStreaming`/`chat.replyTo` labels in **en and de** |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx` | the render branch: role noun, state word, `role="status" aria-live="polite" aria-busy` live region, `data-semio-agent-chat-reply`/`-reply-to` hooks |

No default language anywhere: every string on the new surface goes through `useLabel(agentUiLabel(…))`
and is declared in both `en` and `de` bundles. §4's third law asserts exactly that.

## 4. Laws

## 5. Live proof — `⚖️gate🌉️os-mcp💬️agent-reply`

A new sibling gate rather than extra steps inside `live-agent-loop-check`, because peers run that
gate concurrently and a 9-step insert into it would have collided.

* **verb** `agent-reply-check` — `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💬️agent-reply/🟦️.ts`
  (the gate) + `…/💬️agent-reply/🏃️execution/🟦️.ts` (its `BundleScript`), registered in
  `🌉️mcp/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` (`dependsOn: ["build"]`, `cache: false`,
  next to `hub-agent-participant-check`), with the `.vscode/launch.json` row
  `⚖️gate🌉️os-mcp💬️agent-reply` at order `411.107585`, directly after
  `⚖️gate🌉️os-mcp🤖️live-agent-loop` in group `4_gate`.
* **my serve**: `📜️ac1-serve.sh s 6197` (`SEMIO_VITE_HMR=0`, `NX_DAEMON=false`, detached, log
  `🗑️generated/ac1-serve-s-6197.txt`, pids in `🗑️generated/ac1-serve-pid.txt`: wrapper **98314**, vite listener **98612** — still up at
  the end of this slice so the gate can be re-run; kill only those two). `VITE v7.3.6 ready in
  9 733 ms`, `:6197` answers 200. Folder mode — the reply channel is gateway↔shell only and needs no
  hub, so hub 7621 was neither used nor touched. A private `S_AGENT_BRIDGE_DIR`
  (`🗑️generated/ac1-bridge`) keeps the rendezvous off every peer's.
* **precondition, refused before any step**: the serve must transform THIS tree's `🔗️AgentBridge`
  (`curl` of the `/@fs` module → 1 hit for `agentReply`; `AgentChatPanel` → 2 hits for
  `semio-agent-chat-reply`).

**Result — `os-mcp-agent-reply: 9 passed, 0 failed, 0 skipped of 9`, exit 0** (capture
`🗑️generated/ac1-agent-reply-gate.txt`, run against the binary built from the final tree):

| step | measured |
| --- | --- |
| 0 rendezvous | this gateway's own offer, `ws://127.0.0.1:53879/bridge pid=38075` (the pid CHANGED, so it is not a peer's) |
| 1 the tool is on the live surface | `title=Reply In Chat input=object output=object` — **of 28 tools** |
| 0 boot / dials `/bridge` / `ui_reveal` | `ready=s windows=s-home-main`; `ui_focus` stops refusing; chat panel in the DOM |
| **2 the scope is real** | a SECOND gateway on the same launch line with `conversation.write` stripped: `isError=true {"code":"PERMISSION_DENIED","message":"principal agent:local lacks required scope shell.converse"}` |
| **3 a streamed turn is ONE row** | two calls, one `replyId`; `shells: 1` each; the live DOM went `streaming (aria-busy=true)` → `complete (aria-busy=false)`; **rows for this turn = 1**; `inReplyTo=msg_gate`; text = `"Widening that wall means the 300 mm variant — shall I?"` (the two chunks concatenated) |
| **4 no double-printing** | tool-call rows before = 2, after = 2, of which naming `conversation_reply` = **0** |
| **5 the human's turn reaches the agent** | typed into the REAL composer (`#framework.chat.draft`, Enter), echoed in the transcript, and read back by the same MCP client: `semio://ui/agent-messages` = `{"connection":"conn_0","drained":true,"messages":[{"messageId":"msg_62jfeuhckfd_1","text":"gate turn 1790071903325"}]}` |

**Screenshot**: `🗑️generated/ac1-agent-reply-panel.png` — the chat dock on the right of the live `s`
shell showing, in order, two `TOOL CALL` rows (`ui_focus`, `ui_reveal`) with their results, then an
**AGENT** row reading *"Widening that wall means the 300 mm variant — shall I?"*, then a **YOU** row
with the turn typed into the composer beneath it. That is the whole slice in one picture: the agent
talking, and the human answering, in the same transcript.

**One flake, recorded rather than hidden.** An earlier run of the same gate on the same serve
reported `FAIL 0 boot :: ready=null error=s` while all five AC1 steps still passed (the chat dock is
shell chrome and does not need a booted plugin). The next run of the identical command was
`ready=s windows=s-home-main`, 9/9. That is the known flaky `s` boot under fleet load, not
something this slice changed — but a gate that boots the real `s` host will show it again.

## 6. README + `.mcp.json` (G19 gaps #9, #10)

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md`.

**Gap #9 — the stale "nothing to fire on" paragraph is replaced with the CURRENT measured numbers.**
It used to read *"no plugin descriptor in this repo actually declares `destructive: true` yet, so
the approval gate … has nothing to fire on"*. Measured on this tree 2026-09-22 (captures
`🗑️generated/ac1-audit.txt`, `🗑️generated/ac1-destructive-count.txt`):

* `semio-os-mcp audit --folder .` (native, release-path binary, no shell, no workspace) →
  **`29 finding(s) over 59 descriptor(s)`**, exit 1 — of which **25** are
  `WhenDestructive never fires` and **4** are `declares no audience`.
* a count over the 34 committed `✏️s/🔌️plugins/*/🔣️.json` → **128** occurrences of
  `"destructive": true` across **27** plugin descriptors (largest: `🎪️demonstrator` 19, `📕️norm` 14,
  `🧱️block` 11, `📸️remodel` 9, `🌀️procedural` 7), against 4 309 `false`.

So the honest current statement, now in the README, is: most destructive work IS gated, a **named**
minority (the 25 + 4 the audit lists) is not, and `--auto-approve` remains the blanket control. The
paragraph tells the reader to run the audit themselves rather than trust either number. The audit
also still reports one plugin it cannot read at all (`skipping plugin puzzle: … 🔣️.json did not
decode as a PackageDescriptor: missing field artifactSchema`) — not this slice's, unchanged.

**Gap #10 — a hub-bound client config that actually exists.** The README previously described
`--hub`/`--credential-file` in prose and then showed only two `--folder .` configs. It now carries a
third worked example, *"Claude Code, bound to a hub space instead of a folder"*, with the real
`--hub`/`--space`/`--credential-file`/`--scopes` argv, and names the three things that config needs
which the folder one does not (the `0600` credential file; `hubOrigin`/`spaceId` matching the flags;
`--hub` requiring `--space`), plus why the token must never go in `args`.

**Also in the README**: `conversation_reply` added to the Surface list with its streaming contract
and its `shells: 0` tier; `conversation.write` added to the scope enumeration; "Twenty-seven stable
tools" → "Twenty-eight" and the `tools/list` count in the typo-costs-you-a-tool-family paragraph
27 → 28; and the "No model provider" section corrected — the panel now also shows the agent's own
free-text turns, but the words are always the connected client's, relayed over the bridge.

**And the configs themselves**: `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json`,
`.windsurf/mcp.json`, `.kiro/settings/mcp.json`, `.codex/config.toml` all gained
`conversation.write` on their `--scopes` line. Without it this repo's own agent would have had the
tool listed and refused on every call — which is exactly what gate step 2 measures when the scope is
taken back out.

## 7. Honest gaps

1. **The wgpu shell renders no reply.** There is a THIRD codec twin of this wire —
   `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs:334` — and AC1 did not
   touch it. Its `GatewayToShell::decode` answers `BridgeFrameFault::UnknownTag(10)` for a reply
   frame, which `apply_encoded_frame` records in `last_error` (it does not drop the connection). The
   work is: the variant + its decode/encode arm there, an `AgentConversationEntry::AgentMessage`
   variant, and the six match sites in
   `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`agent_chat_entry_node`,
   `agent_chat_role_label`, `agent_chat_entry_kind`, `agent_chat_entry_state_attribute`,
   `agent_chat_state_label`, `AgentConversationEntry::id`). I deliberately did **not** do it: it is a
   21 000-line file three `WGPU-RENDERER-REACT-PARITY` agents are editing right now, and verifying
   it needs a wgpu-renderer build this slice's budget (`-p semio-framework-os-mcp` only) does not
   cover. It belongs to that ticket, and this paragraph is the hand-off. The one thing I DID keep in
   parity is the empty-transcript line, which the wgpu twin copies "word for word" — both were
   updated together.
2. **The reply is a projection, not a record.** `AgentReply` is broadcast to every attached shell
   and retained nowhere: a shell that connects after a turn was published never sees it, and a
   reload loses the transcript. That matches how `AgentToolCall`/`AgentToolResult` already behave
   (the panel is "a live view, not an archive", `AGENT_CONVERSATION_MAX_ENTRIES = 200`), so it is
   consistent rather than new — but it does mean the channel is not a conversation log.
3. **`broadcast`, not `send_to`.** A reply goes to every shell on this gateway's bridge, like every
   other conversation frame. With two windows of the same session open, both show it. Per-connection
   addressing would need a shell-id argument the tool does not have and no caller could supply today.
4. **Nothing makes the agent reply.** The channel exists and is scope-gated; whether a connected
   client chooses to narrate is the client's policy. The tool's description is written to invite it
   ("so the human reads your answer where they typed their question"), and `semio://ui/agent-messages`
   now pushes, so a subscribed client is told when it is being asked something — but there is no
   server-side nudge, and there should not be one in a crate with no model provider.
5. **Truncation is silent to the agent.** A chunk over 2 048 bytes is cut and marked `…` in the
   frame; the tool result reports `ok` with the ORIGINAL char count, so an agent cannot tell from the
   result that the human saw less than it sent. Streaming is the documented way around it, but the
   asymmetry is real.
6. **`cargo test -p semio-framework-os-mcp --lib` is 437 passed / 3 failed on this tree**, and after
   fixing the two that were mine it is **1 failed**, which is not:
   `workspace::quick::authenticated_hub_workspace_resources_are_snapshot_only_scoped_and_fail_closed_when_stale`
   — a peer's in-flight change to the uncommitted `🏠️workspace/🦀️.rs` now answers
   `semio://artifact/shared-doc/schema` where the law expects a refusal. Untouched by this slice and
   in no file it edits. (`transport::quick::terminal_public_fifo…` also failed once with an `EAGAIN`
   under fleet load and passed on re-run; also not mine.)

## 8. Files changed

Mine, all under `/Users/ueli/Documents/semio`:

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🧪️tests/🔬️quick/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🧪️tests/🔬️quick/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🔣️.json          (regenerated: schema-mirror)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts            (regenerated: schema-mirror)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️conformance/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💬️agent-reply/🟦️.ts                (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💬️agent-reply/🏃️execution/🟦️.ts     (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🧪️tests/🧩️component/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs   (empty-transcript copy only, kept word-for-word with React)
.mcp.json  .cursor/mcp.json  .vscode/mcp.json  .windsurf/mcp.json  .kiro/settings/mcp.json  .codex/config.toml
.vscode/launch.json
.🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/📜️ac1-serve.sh                                  (new, ticket-local)
```

Peers' edits seen in the same `git status` and deliberately untouched: `🏠️workspace/**`,
`💡️inference/**`, `🗿️artifact/🦀️.rs`, `🌉️mcp/🟦️.ts`.

Captures, all in the ticket's `🗑️generated/`: `ac1-cargo-check.txt`, `ac1-cargo-test.txt`,
`ac1-cargo-test-lib.txt`, `ac1-cargo-test-schema.txt`, `ac1-build.txt`, `ac1-schema-mirror.txt`,
`ac1-schema-mirror-check.txt`, `ac1-audit.txt`, `ac1-destructive-count.txt`,
`ac1-vitest-panel.txt`, `ac1-vitest-bridge.txt`, `ac1-agent-reply-gate.txt`,
`ac1-serve-s-6197.txt`, `ac1-serve-pid.txt`, `ac1-agent-reply-panel.png`.
