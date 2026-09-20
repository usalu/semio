# 📓️ LB1 — the live bridge artifact route: an MCP client's edits happen IN the user's shell

Slice LB1 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Source memos:
`📓️r2-reactor-retained-command-owner.md` §12 (findings 1 and 2),
`📓️m7-live-agent-bridge-loop.md`, `📓️m4-mcp-bridge-approval-binding.md`,
`📓️m2-agent-surface-and-inference.md`.
Crates/modules: `semio-framework-os-mcp` (`🌉️mcp`), `📺️renderer/…/🔗️AgentBridge`, `📺️renderer/…/🏛️ShellHost`.

**The missing piece of outcome 4.** R2 measured it on the real `s` host: only `ui_reveal`/`ui_focus`
are shell-bound. `artifact_open/create`, `action_prepare/invoke`, `artifact_snapshot`,
`history_undo/redo`, `transaction_*` and `artifact_export` all resolve through
`ActionAdapter` → `ArtifactChannel` → `PluginArtifactChannel` → the headless `OwnedRuntime`
interpreter on the gateway's own `--folder` workspace. That is a **different document owner** from
the running shell, so the agent's edit never appears in the user's window and never enters the
user's undo history.

## Status

| # | item | state |
|---|---|---|
| 0 | inherited state + measured baseline | ✅ |
| 1 | the wire already existed — three ends of it were dead | ✅ fixed (§1) |
| 2 | Rust: `ShellArtifactChannel` over the bridge (correlated request/response, cancel-on-timeout) | ✅ 16/16 laws |
| 3 | channel selection per session + `channel: "shell" \| "headless"` on `context_resolve` | ✅ laws + schema mirror regenerated |
| 4 | schema-first codec + shared fixtures on both banks | ✅ 19-row fixture, both banks assert it |
| 5 | Shell: `useAgentBridge` answers `appCommand`, ShellHost dispatches on the UI action lane | ✅ 7/7 laws, typechecks, served module verified |
| 6 | approvals in the shell per M4's chain | ✅ unchanged by construction (§6) |
| 7 | gate — `live-agent-loop-check` | ⚠️ **5 passed / 10 failed / 4 skipped of 19** on `:6070` (R2 baseline 6/9/4). (f1)–(f8) still red: `artifact_create` bypasses the session channel entirely (§7.3) |
| 8 | wgpu parity debt | 📝 recorded, not closed (§8.5) |

## 0. Inherited state

No predecessor: `git status` showed no `lb1-*` captures and no `📓️lb1-*.md`. Everything below is
this session's, measured.

## 1. The wire already existed — and one end of it terminated the connection

The `/bridge` codec has carried the artifact route's frames since M4, on BOTH banks:

| frame | Rust | TypeScript |
| --- | --- | --- |
| `GatewayToShell::AppCommand { seq, instance_id, command }` | `🧵️bridge/🦀️.rs:1365` | `🧵️bridge/🟦️.ts:334` |
| `ShellToGateway::AppFrames { in_reply_to, instance_id, frames }` | `🧵️bridge/🦀️.rs:282` | `🧵️bridge/🟦️.ts` |
| `ShellToGateway::Instances { entries: BridgeInstanceRef[] }` | `🧵️bridge/🦀️.rs:281` | idem |
| `BridgeFlags.relay_app_commands` | `🧵️bridge/🦀️.rs:201` | idem |

All four are in the 23-row shared fixture `🧵️bridge/🧫️fixtures/📨️frames.json`. **Nothing consumed
them.** Three measured holes, all now closed:

1. **The transport KILLED any connection that answered.** `🚚️transport/🦀️.rs:1418` and `:1472` both
   matched `AppFrames` into `HttpTerminalReason::Unsupported` — a shell that replied to an
   `AppCommand` had its socket torn down. (Nothing had ever sent an `AppCommand`, so the refusal was
   invisible.)
2. **The bridge DROPPED the reply.** `BridgeHandle::record`'s last arm listed
   `ShellToGateway::AppFrames { .. }` among the frames it ignores, so even a delivered reply reached
   no correlation table.
3. **The shell IGNORED the request.** `🔗️AgentBridge/🟦️.tsx`'s frame switch was literally
   `case "appCommand": break;`.

And `Hello`'s `flags` were parsed and thrown away, so no gateway could know whether an attached
shell was willing to execute artifact commands at all.

## 2. `ShellArtifactChannel` — one more implementation of the SAME port

`crate::actions::ArtifactChannel` is one method: `exchange(instance, commands) -> Vec<AppFrame>`.
So the route is a sibling of `PluginArtifactChannel`, not a parallel stack — every tool above it
(`🗿️artifact`, `🔀️dispatch`'s `ActionAdapter`, its handle table, its typed results, M4's approval
gate) is untouched and returns the same shapes.

`🐚️channel/🦀️.rs` (new, 370 lines):

| item | what |
| --- | --- |
| `ShellArtifactChannel::exchange` | encodes the command, sends `GatewayToShell::AppCommand{seq,…}`, polls `BridgeHandle::last_app_frames` for `in_reply_to == seq`, decodes. A reply addressed to a different `instance_id` is a `channel.not-wired` fault, never accepted. |
| `resolve_instance_id` | the adapter's opaque `instance: u32` → the shell's own `instance_id`. By the capability's owning plugin (same catalog lookup `RoutingArtifactChannel` does, so the two can never disagree), then a sticky per-`instance` binding so every leg of a prepare→commit→undo saga lands on the instance that ran the first leg, then the single-instance case. Each failure names what the human must open. |
| timeout | `SHELL_APP_COMMAND_TIMEOUT_MS = 60_000`, and on expiry it **sends a `cancel` payload for that seq** — the abandoned command stops in the shell instead of committing after the agent gave up. Answers retryable `budget.exceeded`. |
| `Infer` | refused before encoding: `ensure_inference_route` owns its own guest and the shell has no seam to drive one. |

Bridge/transport repairs it needed: `AppFrames` admitted by the transport (both guards),
`ConnectionEntry.last_app_frames` + `BridgeHandle::last_app_frames`, `ConnectionEntry.flags` +
`BridgeHandle::record_hello`/`shell_flags` (the `Hello` payload was being discarded).

## 3. Channel selection and `context_resolve`

`SessionChannelBinding` (`🐚️channel/🦀️.rs`) decides ONCE, and `registry_override_context_resolve`
(`🌉️mcp/🦀️.rs`) is where it is decided — so the `channel` a client reads is the channel every later
`action_invoke`/`history_undo` on that session executes through.

- `ContextSummary.channel: String` — new, schema-first (`🧬️schema/🦀️.rs`), `"shell" | "headless"`.
- Shell is chosen only when a live connection declared `BridgeFlags.relay_app_commands`.
- **Sticky by design.** A session that resolved `shell` and then loses the shell gets a typed,
  retryable `PLUGIN_UNAVAILABLE` naming the lost shell — never a silent re-route onto a different
  document owner mid-transaction. A law asserts exactly that.
- `ShellRoutedArtifactChannel` (`🏠️workspace/🦀️.rs`) holds both routes and is the channel
  `server_for_workspace_options` now builds; `ArtifactChannels` gained a `Shell` variant. With no
  shell attached the behaviour is byte-for-byte the pre-LB1 gateway.

## 4. Codec and fixtures — schema-first, one file, both banks

`🐚️channel/🧫️fixtures/🗿️app-payloads.json` — 19 rows (9 gateway→shell commands, 10 shell→gateway
frames), each carrying `version: 1`. Byte fields are base64 (1.33× vs. ~4× for a number array
against a 1 MiB per-frame outbox credit); both `encode`/`decode` are written out, not delegated to
`btoa`, so the two banks share one definition.

- Rust: `encode_app_command` / `decode_app_frame` in `🐚️channel/🦀️.rs`, asserted against every
  fixture row.
- TypeScript: `decodeShellAppCommand` / `shellAppFrameToJson` in `🐚️channel/🟦️.ts`, asserted against
  the SAME file.
- A payload from another codec version is refused by name on both banks; an unknown `kind` and a
  missing field likewise.

## 5. The shell side — the agent's edit is the human's edit

`🔗️AgentBridge/🟦️.tsx`:

- `case "appCommand": break;` → decode + `answerAgentAppCommand`, which **never goes silent**: no
  handler → a `plugin.unavailable` refusal naming the gap; a throwing handler → a
  `channel.not-wired` error frame on the same correlation id.
- `relayAppCommands` is derived from `onAppCommand` actually being mounted, not hand-set — a shell
  that claims the relay and then refuses everything is worse than one that never claimed it (the
  gateway would have kept its own workspace).
- The instance census (`instances` frames) is published on `welcome` and whenever the host's array
  identity changes, with a stable empty default so an unchanged census never re-publishes.

`🏛️ShellHost/🟦️.tsx` — `agentArtifactRouteRef`, filled by its own effect:

| command | what happens in the shell |
| --- | --- |
| `readHistory` | `plugin.readHistory(session.instanceId)` — the live store's own head and cursor |
| `pureCommand` | validates the verb against the live `app.actions` and returns a **deferred plan** as the document-lane ops |
| `transactionPrepare` | parks the plan under its `txnId` |
| `transactionCommit` | dispatches through `onActionRef.current` — the shell's ONE input funnel, the same call the palette/keybindings/app buttons make — then reads the new head as `editId` |
| `transactionRollback` | drops the plan; the document is genuinely untouched |
| `transactionUndo`/`Redo` | `onAction(… "undo"/"redo")`, the shell's own history |
| `readArtifact` / `exportMedia` | typed refusal naming the gap (see §8) |

`InputOriginV1` gained `"agent"`, so an agent's edit is attributable in the human's ledger and
raises no transient notice (the agent reads the typed outcome instead). Because the mutation leaves
through `onAction`, it obeys the active-window rules and the input ledger exactly as a human's does
— which is what makes it land in that human's undo history and replicate like any local edit.

## 6. Approvals — unchanged by construction, and that is the point

M4's chain (`--auto-approve` → MCP elicitation → the live shell's `ApprovalRequested`/`Approval`
frames, `SHELL_APPROVAL_TIMEOUT_MS` 120 s) lives in `ApprovalCoordinator` and is applied by
`ActionAdapter::invoke_uncached` **above** the `ArtifactChannel` port. The shell route is a
different implementation of that port, so a gated capability still parks a handle and still surfaces
in `🤖️AgentApprovals` / `💬️AgentChatPanel` before any command reaches the shell. Nothing in this
slice weakens, bypasses or duplicates it — there is still no `approval_resolve` tool.

One property the shell route strengthens: the shell lane of the approval chain and the execution
lane are now the SAME connection (`active_shell_connection`), so an approval the human grants and
the mutation it authorises cannot land in two different shells.

`(e1)/(e2)/(e3)` remain SKIP for M7's own reason (§5.4: no plugin in this repo authors a
`destructive` capability that the live catalog exposes), which is M5a's lane, not this one.

## 7. Measured

### 7.1 Tests — run, with their captures

| suite | numbers | capture |
| --- | --- | --- |
| `cargo test -p semio-framework-os-mcp --lib` (rule 25 private target dir) | **16/16 new `shell_channel` laws pass**; 1 pre-existing red: `schema::quick::the_json_mirror_publishes_exactly_the_registry_exports`, **fixed** by regenerating the mirror (below). The run's own binary was SIGKILLed by the coordinator's 11:12 orphan sweep before it printed a total, so there is no `test result:` line — every individual line is in the capture. | `🗑️generated/lb1-cargo-test-lib.txt` |
| `bun ./📜️script.ts agent-bridge-check` | **51 passed / 51**, exit 0 (was 44 before this slice's 7 new laws) | `🗑️generated/lb1-agent-bridge-check.txt` |
| `bunx tsc --noEmit` (react renderer) | 0 errors in any file this slice touched; the one `🏛️ShellHost` error (`:4962` `consumes`) is T4c's pre-existing owned row | — |
| `bun ./📜️script.ts build` (os-mcp) | `Finished dev profile in 2m 13s`, binary restaged 11:29 | `🗑️generated/lb1-mcp-build.txt` |
| `bun ./📜️script.ts schema-mirror` | `exports=68 ajv-draft07-resolved=68 json=1 typescript=1` — `🔣️.json` +34, `🟦️.ts` +9 | `🗑️generated/lb1-schema-mirror.txt` |

The served shells were verified to carry the new code rather than a stale transform (memory law):
`curl http://127.0.0.1:{6070,6071}/@fs/<ShellHost>` → **3 hits for `agentArtifactRouteRef`** on both.

### 7.2 `live-agent-loop-check` — the acceptance gate, two runs

| run | numbers | capture |
| --- | --- | --- |
| `:6080` (M5a's note serve) | **1 passed, 14 failed, 4 skipped of 19** | `🗑️generated/lb1-live-agent-gate-6080.txt` |
| `:6070` (the real `s` host) | **5 passed, 10 failed, 4 skipped of 19** | `🗑️generated/lb1-live-agent-gate-6070.txt` |

`:6080` fails at step 0: its serve answers `404` on `/__semio/agent-bridge` (`~/.semio/agent/bridge/`
shows offers being written, and `:6070`/`:6071` both answer `200`), so that serve's vite rendezvous
plugin is not live — everything after cascades. Not this slice's; recorded for whoever owns that
serve. **`:6070` is the measurement.**

`:6070`, step by step: `0 rendezvous` PASS, `(a)` dial PASS, `(a)` presence PASS, `(d)` `ui_reveal`
PASS, `(c)` Cancel PASS. `boot` and `(d) ui_focus` FAIL for S2's known reason (the `s` host's
signed-out Home publishes no shell session: `ready=null error=s windows=`). `(b)` FAIL is a probe
race on a calm machine (`running=null settled={state:"ok"}` — the call settled before the probe
sampled the running row; R2 measured this step PASS under load). `(e1)`–`(e3)` SKIP, M5a's lane.

### 7.3 The (f) chain — WHY it is still red, measured, and it is not a budget

R2's (f1) failed on the interpreter's wall budget. It now fails differently:

```
FAIL (f1) artifact_create/open :: kind=s.note.note: INTERNAL: `note` refused ReadArtifact
     (channel.not-wired): execute_turn: no Effect::Respond for seq 1 before terminal acknowledgement
```

`execute_turn` is `PluginArtifactChannel`'s own message — so **(f1) never reached the routed
channel at all**. Root cause, read off the source and stated precisely:

> `HeadlessWorkspace::create_plugin_artifact` (`🏠️workspace/🦀️.rs:2040`) and
> `export_artifact_media` (`:2063`) each call `self.open_artifact_channel(plugin_id)` and build
> their **own** `PluginArtifactChannel`. They never touch the `ArtifactChannels` value the server
> was constructed with. So the session channel governs `action_prepare`/`action_invoke`,
> `transaction_*` and `history_undo/redo` — but `artifact_create` and `artifact_export` are wired
> straight to the headless interpreter by construction.

Since `(f1)` is `artifact_create`, the whole (f) chain cascades before a single shell-routed command
is sent. **This is the next step of the route, and it is two changes, not one:**

1. `create_plugin_artifact`/`export_artifact_media` must take the session's channel (the same
   `ArtifactChannels` the adapter holds) instead of opening their own — a `HeadlessWorkspace`
   signature change that touches `🗿️artifact`'s call sites; and
2. the shell must be able to answer `ReadArtifact`/`ExportMedia` (§8 gap 1), or a shell-routed
   `artifact_create` just moves the refusal.

`(f5)` remains R2's runtime-checked SKIP for the same reason — it greens itself the day (f1) lands
in the shell.

### 7.4 What IS proven

The route is complete and executable end to end for the verbs that go through `ArtifactChannel`:
its wire is admitted on both banks (16 Rust + 7 TypeScript laws over one shared fixture), its
selection is reported on `context_resolve`, its refusals are typed and named, its timeout cancels
rather than abandons, and the shell bank dispatches through `onAction`. What is NOT proven at
runtime is an agent edit appearing on screen — because the gate cannot get past `artifact_create`,
which §7.3 shows is not on this route yet. That is stated as a gap, not as a pass.

## 8. Honest gaps

1. **`readArtifact` and `exportMedia` are refused, by name, on the shell route.** Both need the live
   store's own document pack/spr bytes (and, for export, the plugin's media OUT port), which
   `🏛️ShellHost` does not expose to the bridge. So `artifact_create`/`artifact_open` **genesis** and
   `artifact_export` still need a headless context. This is the single biggest remaining piece of
   the route and it is a shell-side data seam, not a protocol one — the frames already carry
   `artifact{pack,spr}` and `exported{port,descriptor,data}` on both banks and both codecs are
   tested, so the day the shell can produce those bytes it is one handler arm each.
2. **The two-phase split is deferred, not simulated.** The shell has no "compute the ops without
   applying them" seam, so `action_prepare` returns a deferred plan and `transaction_commit`
   dispatches. Consequences, stated plainly: a `prepare` does not validate against the guest (only
   against the app's declared actions), and `action_invoke`'s `revisionBefore`/`revisionAfter` are
   real (both are real `readHistory` reads) while the *ops* it reports are this route's own plan
   bytes, not guest-emitted ops. A rollback is genuinely a no-op, which is the property that
   matters most.
3. **The census is the active session only.** `agentBridgeInstances` publishes one entry. A shell
   with several open artifacts exposes only the focused one to an agent; the frame shape
   (`Instances { entries: [...] }`) is already plural, so this is a ShellHost enumeration, not a
   wire change.
4. **The capability→action mapping is the id's last segment.** `pureCommand` takes
   `note.note.appendParagraph` → `appendParagraph` and checks it against the live `app.actions`.
   That is how the shell's own palette addresses verbs, but it is a convention, not a declared
   contract; an app whose action id differs from its capability's last segment is refused by name
   (`capability.not-found`) rather than mis-dispatched.
5. **wgpu parity debt.** `🎯️targets/🧊️wgpu` implements M7's inbound `ShellCommand` chrome verbs but
   has NO artifact route: a wgpu shell never declares `relayAppCommands`, so a session that attaches
   to one resolves `headless` and behaves exactly as before. That is correct-by-default, not a
   silent failure — but it means outcome 4's live route is React-only today. The Rust half is
   already shared (`🐚️channel` is gateway-side); what the wgpu target needs is its own
   `AppCommand` arm plus the census, against `🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`. Recorded here
   as this slice's named debt.
6. **No live two-user replication proof.** The claim "it replicates to collaborators like any local
   edit" follows from the edit going through `onAction` (the same funnel a human's edit uses), but
   this slice did not run a two-browser scenario to observe it. C1c owns that lane.

## 9. Files changed

| file | what |
| --- | --- |
| `🌉️mcp/🐚️channel/🦀️.rs` | **new** — payload codec, `SessionChannelBinding`, `ChannelKind`, `ShellArtifactChannel` |
| `🌉️mcp/🐚️channel/🟦️.ts` | **new** — the TypeScript bank of the same codec |
| `🌉️mcp/🐚️channel/🧫️fixtures/🗿️app-payloads.json` | **new** — the 19-row shared fixture both banks assert against |
| `🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs` | **new** — 16 Rust laws |
| `🌉️mcp/📦️packages/🦀️rust/🦀️.rs` | mounts `pub mod shell_channel` |
| `🌉️mcp/🚚️transport/🦀️.rs` | admits `AppFrames` (was `Unsupported`, twice); records `Hello`'s flags |
| `🌉️mcp/🧵️bridge/🦀️.rs` | `ConnectionEntry.flags` + `.last_app_frames`, `record_hello`, `shell_flags`, `last_app_frames`, `record`'s `AppFrames` arm |
| `🌉️mcp/🏠️workspace/🦀️.rs` | `ShellRoutedArtifactChannel`; `ArtifactChannels::Shell` in both `cfg` spellings |
| `🌉️mcp/🦀️.rs` | `GatewayRuntime.channel_binding` + `channel_binding()`; `WorkspaceToolRegistry.channel_binding`; `registry_override_context_resolve` stamps `summary.channel`; `server_for_workspace_options` builds the shell-routed channel |
| `🌉️mcp/🧠️context/🦀️.rs` | `resolve_context` stamps `channel` |
| `🌉️mcp/🧬️schema/🦀️.rs` | `ContextSummary.channel` |
| `🌉️mcp/🧬️schema/🔣️.json`, `🟦️.ts` | regenerated via `bun ./📜️script.ts schema-mirror` (`exports=68 ajv-draft07-resolved=68`) |
| `📺️renderer/…/🔗️AgentBridge/🟦️.tsx` | `AgentAppCommandRequest`/`AgentAppCommandHandler`, `answerAgentAppCommand`, `NO_ARTIFACT_ROUTE_MESSAGE`, the `appCommand` case, derived `relayAppCommands`, the instance census |
| `📺️renderer/…/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts` | 7 new TypeScript laws (fixture parity, refusal shapes, the live hook round trip) |
| `📺️renderer/…/🏛️ShellHost/🟦️.tsx` | `agentArtifactRouteRef`, `agentDeferredPlansRef`, `agentBridgeInstances`, the route effect |
| `📺️renderer/…/🏛️ShellHost/🎯️input-ledger/🟦️.ts` | `InputOriginV1` gains `"agent"` |


---

## 10. Session 5c round 2 — §7.3 taken, and the route is LIVE

### 10.1 Step 1 — the artifact verbs now obey the session's channel

`create_plugin_artifact` and `export_artifact_media` no longer hardcode `open_artifact_channel`.
Smallest hunks, in a file WR1 is also editing (re-read immediately before each edit):

| file:line | change |
| --- | --- |
| `🏠️workspace/🦀️.rs:1653` | `shell_route: OnceLock<(Arc<SessionChannelBinding>, Arc<Catalog>)>` on `HeadlessWorkspace` |
| `🏠️workspace/🦀️.rs` `bind_shell_route` / `open_session_artifact_channel` | the binding-aware opener |
| `🏠️workspace/🦀️.rs:2040 / :2063` (now `:2072 / :2095`) | `self.open_artifact_channel(…)` → `self.open_session_artifact_channel(…)` — **the two functions named in WR1's note** |
| `🏠️workspace/🦀️.rs` `ArtifactChannels` | `ShellDirect(ShellArtifactChannel)` in both `cfg` spellings |
| `🌉️mcp/🦀️.rs` `server_for_workspace_options` | `workspace.bind_shell_route(binding, catalog)` |

`cargo check -p semio-framework-os-mcp` → **rc=0, 0 errors** (`🗑️generated/lb1-cargo-check2.txt`).

### 10.2 Step 2 — the shell answers `ReadArtifact` / `ExportMedia`

- `🧰️framework/🛍️products/💻️os/🟦️.ts` `AppChannelClient.readDocumentBytes()` (guest `ReadDocument` →
  `Document{pack,spr}`) and `.exportMediaBytes(port)` (guest `MediaOut` → `Media{port,descriptor,data}`),
  both correlated on their own `seq` and both raising the guest's own fault text.
- `🏛️ShellHost` gained the two arms. The export goes **straight to the OUT port and back over the
  bridge** — deliberately not through the UI's download path, so the agent gets bytes, the human
  gets no unasked-for file, and none of it touches the segmented-download queue-ownership path.
- Both arms call the port optionally and refuse **by name** when the live handle does not expose it,
  which is the same never-go-silent contract every other arm keeps. Fixtures needed no change: the
  `artifact{pack,spr}` and `exported{port,descriptor,data}` rows were already in the 19-row shared
  fixture and already asserted on both banks.

### 10.3 The gate, rerun on `:6070` after a fresh build — **6 passed / 9 failed / 4 skipped of 19**

Capture `🗑️generated/lb1-live-agent-gate-6070b.txt` (build `lb1-mcp-build2.txt`, rc=0).

| step | result |
| --- | --- |
| `0 rendezvous` | **PASS** `url=ws://127.0.0.1:51575/bridge pid=23378` |
| `boot` | FAIL `ready=null error=s windows=` — S3's signed-out Home, unchanged |
| `(a)` dial / `(a)` presence | **PASS** / **PASS** (`tone=working`) |
| `(d)` `ui_reveal` | **PASS** (chat panel in DOM) |
| `(d)` `ui_focus` | FAIL — no window id, the same `boot` cause |
| `(b)` tool call → running → result | **PASS** (the probe race of round 1 did not recur) |
| `(c)` Cancel | **PASS** |
| **(f1) artifact_create/open** | FAIL — and the failure is the PROOF the route is live (below) |
| (f2)–(f4), (f6)–(f8) | FAIL, all cascaded from (f1) |
| (f5) | SKIP, R2's runtime-checked precondition |
| (e1)–(e3) | SKIP, M5a's lane |

### 10.4 (f1)'s new message is the measurement

```
FAIL (f1) artifact_create/open :: kind=s.flow.flow: SIDE_EFFECT_REJECTED:
     `flow` rejected ReadArtifact (plugin.unavailable):
     plugin `space` exposes no document-bytes port in this shell
```

Read it carefully — three things are proven at runtime that were not before:

1. the session resolved the **`shell`** channel (it was `headless` in every earlier run);
2. `artifact_create` executed **through that channel** rather than opening its own interpreter
   (§10.1 landed — the old message was `PluginArtifactChannel`'s `execute_turn: no Effect::Respond`);
3. the command **crossed the bridge, reached the React shell, and the shell's own answer came back**
   — `plugin \`space\` exposes no document-bytes port in this shell` is the string this slice wrote
   into `🏛️ShellHost`, arriving at an MCP client as a typed `SIDE_EFFECT_REJECTED`.

So the route is end-to-end live. Two things stand between it and a green (f) chain:

- **`:6070`'s only live instance is `space`**, the `s` host's own shell app, because `boot` fails
  (S3's signed-out Home). The census therefore offers exactly one instance and a capability-less
  `ReadArtifact` resolves onto it. With a booted editor session the same command addresses `flow`.
- **The ShellHost plugin `handle` is not `AppChannelClient`.** §10.2's two ports live on the channel
  client; the object `🏛️ShellHost` holds (`loadedPluginsRef…handle`, the one carrying
  `readHistory(instanceId)`) does not forward them yet. That forwarding is the last hunk of step 2
  and is why the arm refuses instead of answering. It is named, not hidden: the refusal says exactly
  which port is missing on which plugin.

### 10.5 Serves

`:6080` still answers **404** on `/__semio/agent-bridge` (its vite rendezvous plugin is not live), so
it cannot host this gate at all — `:6070` and `:6071` both answer **200** and `:6070` is what every
number above was measured on. No single-plugin serve currently answering the rendezvous was found.


---

## 11. Hand-off — how to rerun the gate, and what f1–f8 should show

### 11.1 The last hunk, landed

`PluginWasmHandle` (`🔌️PluginRuntime/🟦️.tsx:146`) **already carries the document port** —
`readAppDocumentPack?: (instanceId) => Promise<{pack, spr, ops?} | null>`. §10.2's speculative
`AppChannelClient.readDocumentBytes`/`.exportMediaBytes` were the wrong layer and are **reverted**
(no dead code left in `💻️os/🟦️.ts`). `🏛️ShellHost`'s `readArtifact` arm now calls that real port and
returns `artifact{pack,spr}`; the two refusal paths (no port on the handle, `null` document) each
name themselves.

`exportMedia` stays a named refusal, and the reason is now precise rather than a shrug: **no media
OUT port exists on `PluginWasmHandle` at all**. The UI's own export drives the segmented download
queue, which hands the *human* a file — the wrong outcome for an agent, which needs the bytes back
over the bridge. Adding that port is the remaining piece of `artifact_export` on the shell route,
and its wire is already pinned on both banks (`exported{port,descriptor,data}` fixture row + a law).

Measured: `agent-bridge-check` **55 passed / 55** (was 51; 4 new artifact-bytes laws), exit 0
(`🗑️generated/lb1-agent-bridge-check.txt`). `tsc --noEmit` on the react renderer: the only error in
any file this slice touched is T4c's pre-existing `🏛️ShellHost:4970 consumes`.

### 11.2 Rerun the gate — one line, no code reading required

```
cd "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && \
  S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:6070" S_OS_MCP_LIVE_PLUGIN=note \
  bun ./📜️script.ts live-agent-loop-check
```

Three preconditions, each checkable in a second:

1. the serve answers the rendezvous — `curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:6070/__semio/agent-bridge` must be **200** (`:6080` answers 404 and cannot host this gate);
2. the serve is transforming current source — `curl -s "http://127.0.0.1:6070/@fs$PWD/🧰️framework/…/🏛️ShellHost/🟦️.tsx" | grep -c agentArtifactRouteRef` must be **> 0**;
3. the gateway binary is current — `bun ./📜️script.ts build` in that same directory (the nx target does this for you; a bare `script.ts` call does not).

Set `S_OS_MCP_LIVE_SHELL_URL` to any serve meeting (1) and (2); `S_OS_MCP_LIVE_PLUGIN` to the
variant it serves.

### 11.3 What f1–f8 should show once Home boots an editor session

Today's blocker is singular: `:6070` boots no editor, so the shell's census offers exactly one
instance — the `s` host's own `space` app — and a capability-less `ReadArtifact` resolves onto it,
which has no document. With a booted editor session the census names that editor and:

| step | expected once Home boots | if it does not |
| --- | --- | --- |
| **(f1)** `artifact_create/open` | **PASS** — `ReadArtifact` crosses the bridge, the editor answers `readAppDocumentPack`, real `pack`/`spr` bytes come back | a refusal naming the plugin and the missing port — read the message, it says which |
| **(f2)** `action_prepare` | **PASS** — `ReadHistory` off the live store, then the verb validated against the live `app.actions` (a mismatch answers `capability.not-found` naming the verb) | |
| **(f3)** `action_invoke` changes the head | **PASS** — dispatched through `onAction`, the shell's own input funnel; `revisionBefore`/`revisionAfter` are two real history reads | a refusal carrying the input ledger's own reason (`viewer-read-only`, `mutation-rejected`, …) |
| **(f4)** `artifact_snapshot` | **PASS** — same live store | |
| **(f5)** the live shell shows the same artifact | **PASS, and this is the outcome-4 milestone** — R2 wrote it as a runtime-checked SKIP that greens itself the moment the shell owns the document; it does now | |
| **(f6)** `history_undo/redo` | **PASS** — the shell's own history verbs, so the human can undo the agent's edit from their own UI | |
| **(f7)** `transaction begin/rollback/commit` | **PASS** — rollback is a genuine no-op (the plan is parked, never dispatched) | |
| **(f8)** `artifact_export` | **FAIL, expected**, until a media OUT port exists on `PluginWasmHandle` (§11.1). The refusal names exactly that. | |

So the bar to aim for is **(f1)–(f7) green, (f8) the one named gap**. `boot` and `(d) ui_focus` are
S3's; `(e1)`–`(e3)` are M5a's. Everything else already passes (§10.3).

`context_resolve` is the fastest single check that the route is selected at all: its
`channel` field must read `"shell"`. `"headless"` with a shell attached means the shell did not
declare `relayAppCommands`, i.e. `🏛️ShellHost` did not mount `onAppCommand`.
