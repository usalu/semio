# 📓️ WR4 — typed command dispatch (prepare/apply split) + gate hygiene

Slice: outcome 4's two remaining gates. (1) give the agent lane a REAL two-phase typed command
dispatch — `action_prepare` validates/prices without applying, `action_invoke` applies exactly once,
`transaction_*` composes prepared handles, undo reverts one application; (2) make `client-e2e`
measure the HEADLESS lane deterministically and `live-agent-loop-check` the SHELL one, both
asserting `context_resolve.channel` in step 0; (3) (f5) id unification.

Spec read: `📓️wr3-headless-routes-view-state-export.md` (whole — §3 is the root cause handed to this
slice, §3.3 the exact refusal, §5.0 the gate-channel measurement), `📓️ap1-shell-approval-and-live-snapshot.md`
(live gate 16/2/1; (e3) and (f5) remain), `📓️lb1-live-bridge-action-routing.md` §10–§11,
`📓️wr2-headless-command-response-wire.md`.

**Status: landed and measured at runtime.** `live-agent-loop-check` **20 passed / 0 failed / 0
skipped of 20** (AP1 left 16/2/1 of 19). `client-e2e` **15/17** headless-pinned, with both reds
owned elsewhere and named (§5.4). The two-phase split is proven against a real `🗒️note` guest by a
new native law. `agent-bridge-check` 55/55. `workspace::` 48/0.

## 0. Inherited state

No WR4 predecessor: `git status --short` carried no WR4 edit and `🗑️generated/` no `wr4-*` capture.
Inherited: WR1's runtime port, WR2's command/response wire, WR3's seven paged routes + export
surface + the `action_prepare` root cause (§3.3 of that report), AP1's live gate at 16 pass / 2 fail
/ 1 skip, LB1's shell route.

## 1. How the two lanes dispatch a typed command — measured, both banks

| | shell / UI lane | agent lane, as inherited |
| --- | --- | --- |
| wire | `AppCommand::Command { seq, command, view_state }` | `AppCommand::PureCommand { seq, command, document… }` |
| `command` payload | wire-serialized `ManifestActionInvocation` (owner-qualified: `plugin_id`/`app_id`/`mode_id`/`window_kind_id`/`window_instance_id`/`action_id`) | `serde_json` blob `{"capabilityId":…,"input":…}` |
| guest entry | `handle_action_invocation` → `dispatch_action` → `A::command_from_action` → `dispatch_typed_command_inner` | `handle_command_frame` → `dispatch_command_frame` |
| guest body | mounts a typed operation; its publication ladder APPLIES and mints an undo group | a three-line unconditional refusal (`interactive-job.missing-exact-key`) |

So the two lanes never shared a dispatch shape at all. WR2 read the refusal as "the headless gateway
has no window"; WR3 narrowed it to "the payload is not owner-qualified **and** the reactor gives the
pure lane no view state". Both are true, and the fix is one change on each side of the wire.

### 1.1 The shell route already has the split — and it is the design this slice mirrors

`🏛️ShellHost`'s `agentArtifactRouteRef` (LB1) answers `pureCommand` with a **deferred plan** parked
in `agentDeferredPlansRef` and dispatches only on `transactionCommit`; its own doc says why:
*"the shell has no 'compute the ops but do not apply them' seam"*. The guest SDK does have one —
`ArtifactApp::handle` is declared by the trait as *"the pure heart of the app — a total,
side-effect-free function"* — it simply was not reachable from the pure lane. This slice reaches it.

### 1.2 Why routing prepare through the window lane would be a green gate that lies

WR3 §3 already argued it; the machinery confirms it exactly. `dispatch_typed_command_inner` mounts a
`MountedTypedCommandFullOperation` whose publication ladder commits into `self.store`, and
`RoutingArtifactChannel` caches **one channel, one guest instance, per plugin id**
(`🏠️workspace/🦀️.rs`). `ActionAdapter::invoke` then sends `TransactionPrepare{ops}` +
`TransactionCommit`, and `transaction_commit` applies those ops as one `Edit`
(`🔌️plugin/🦀️.rs:30282`). A prepare on the window lane would therefore apply once at prepare time
and once at commit — `action_invoke` still green, `history_undo` (which walks back exactly one
group) leaving half the edit standing.

## 2. The prepare/apply split — landed in source

### 2.1 Guest: `dispatch_command_frame` has a body (`🔌️plugin/🦀️.rs`)

| # | what |
| --- | --- |
| 1 | `dispatch_command_frame` decodes the frame as a wire-serialized `ManifestActionInvocation` — the SAME shape `AppCommand::Command` carries. A frame that is neither still refuses, but now names both admitted shapes instead of asserting none exists. |
| 2 | `preview_addressed_action` — phase one. Runs, by call and not by copy, every check the shell lane runs: app-owner match, non-empty window instance id, mode ownership, window-kind ownership of the verb, `admit_addressed_action_view`'s window projection, the injected `windowId` argument, `validate_ui_dispatch_classification`, the exact controller/owner/factory/tool/schema proof and its bounded contract, `A::command_from_action`. Then it calls `A::handle` **directly** and encodes the three op lanes into `last_emit_wire`. It applies nothing, to no store. |
| 3 | `addressed_preview_view` — the headless `ViewModel`, derived from the address the caller already sent, never invented: one `ViewWindowInstance` whose id is the addressed instance. It is used ONLY when the reactor supplied none; the Command lane's real human `ViewModel` is used unchanged. |
| 4 | `require_proof_operation_authority(proof, verb, decoded_items)` — the bounded-authority rule lifted off `AdmittedToolCommand` so the preview asserts exactly what the applying phase asserts **without beginning a retained tool-wire admission it owns no operation for**. That is what makes an abandoned prepare leave nothing behind. |
| 5 | `"*"` — the catalog's own marker for an APP-scope verb (`🗒️note` declares all 48 of its verbs that way) is admitted against the app registry rather than against a window kind, exactly as `dispatch_action` does. A concrete kind must still own the verb. |
| 6 | the preview is priced against the proof's `max_output_bytes` and refuses by name (`interactive-job.preview-output`) rather than handing the host an oversized op roster; its `output` reports `documentOps`/`configOps`/`draftOps`/`opBytes`, which is what "prices" means for an agent. |

### 2.2 Host: the gateway sends the owner-qualified frame (`🌉️mcp/🏠️workspace/🦀️.rs`)

`PluginArtifactChannel::addressed_action_invocation` builds the address out of the plugin's **own
committed manifest**, walking `🗂️catalog`'s own `app_action_verbs` walk in the same order and with
the same precedence (window-kind declarations first, then app-scope actions under `"*"`), and
recomputing the catalog's id (`{plugin}.{app}.{action}`) to match. It is never parsed out of the
capability id's punctuation — an app id is itself a dotted canonical surface ref
(`s.draw.drawing@1/*#editor`), so splitting on `.` could not recover it.

`window_instance_id` is the window kind's own id, which is exactly what the React shell's
`sessionWindowInstances` mints for a freshly opened app (`app.windowKinds.map(kind => ({id: kind.id,
…}))`) — so the agent addresses the very pane a human's first window is.

### 2.3 The four laws, and where each is pinned

| law | mechanism | pinned by |
| --- | --- | --- |
| prepare applies nothing | `preview_addressed_action` never reaches `dispatch_emit` and touches no store | `a_prepared_action_applies_nothing_and_its_commit_applies_exactly_once`, assertion 2 |
| invoke applies exactly once | `transaction_prepare` stashes + validates against the snapshot; `transaction_commit` applies ONE `Edit` stamped `group_id = txn_id` | same law, assertion 3 |
| a prepared handle never invoked retires without effect | no store write, no pending transaction, no tool-wire admission begun | same law, final assertion |
| undo reverts one application | `transaction_undo(group_id)` refuses unless the tail edit belongs to that group, then `store.undo()` | same law, assertion 4 |

## 3. Gate hygiene — landed

### 3.1 `client-e2e` is pinned to the headless lane

WR3 §5.0 measured the defect: a `--folder <tmpdir>` run resolved `ChannelKind::Shell` and drove a
peer's live shell (`the shell reports 1 open instance(s)`), so the permanent gate for the headless
route was reporting on a human's open document. The pin is the gateway option that already exists
and means exactly this: **`--no-bridge`**, appended to the `.mcp.json` args
(`🌉️mcp/🟦️.ts`, `runOsMcpClientJourney`). With no bridge slot there is no rendezvous offer, no
relay connection, and `SessionChannelBinding::resolve` can only answer `ChannelKind::Headless`.

A second, stronger option was deliberately NOT added: a `--channel headless|shell` flag would be a
second way to say what `--no-bridge` already says, and the repo's rule is to fix the root rather
than add a parallel switch.

### 3.2 Both gates assert the channel in step 0

- `client-e2e`: new first step **`os: context_resolve pins the headless channel`** — and the journey
  **returns** if it is anything else, so no number below it can be attributed to the wrong lane.
- `live-agent-loop-check`: new step **`0 context_resolve pins the shell channel`**, placed
  immediately after `(a) the shell dials /bridge`. Placement is load-bearing: `context_resolve` is
  where the decision is TAKEN and it is sticky, so a resolve issued before the socket registers
  would pin `headless` for the whole session and silently send every (f) step to this process's own
  `--folder` workspace. This raises that gate's denominator from 19 to **20**.

## 4. (f5) id unification — landed

Two ids name one document: the id the client INVENTS and hands the gateway
(`live-agent-loop-<ts>`), and the one the shell's own artifact route answers for it
(`${pluginId}:${appId}:${instanceId}` — minted by `agentArtifactRouteRef`'s `readHistory` and
already visible to an agent as `action_prepare`'s `expectedRevision.artifactId`). The shell knew that
string and never published it.

- `🏛️ShellHost/🟦️.tsx` renders `data-semio-artifact-id={agentBridgeInstances[0]?.artifactRef}` on the
  shell root — the identity the agent route hands out, published where a browser can be asked about
  it. The value is the census entry the bridge already declares, not a new derivation.
- the gate's (f5) now resolves `expectedRevision.artifactId` and asserts
  `[data-semio-artifact-id="<that>"]` in the live DOM, and it is a **fail**, not a skip: "the browser
  shows it" is asserted.

## 5. Measured

| # | what | evidence | verdict |
| --- | --- | --- | --- |
| 1 | `cargo check -p semio-framework-plugin --lib` after the guest SDK change | `wr4-plugin-check.txt` | rc=0, 0 errors, 39 warnings from this crate (warnings are the proof expansion ran) |
| 2 | `cargo check -p semio-framework-os-mcp --all-targets` | `wr4-mcp-check-alltargets.txt` | rc=0, 0 errors, 67 warnings |
| 3 | `agent-bridge-check` (LB1/WR3's bar) after the `🏛️ShellHost` edit | `wr4-agent-bridge-check.txt` | **55 passed / 55**, exit 0 |
| 4 | `note`/`animate`/`draw` rebuilt on the new SDK, staged into the shared uplift dir | `wr4-plugin-rebuild.txt` | note 64 459 968 B, animate 104 268 744 B, draw 62 724 629 B |
| 5 | **the two-phase law against a real `🗒️note` guest** | `wr4-two-phase-law.txt` | **ok. 1 passed** — §5.1 |
| 6 | **`live-agent-loop-check`** against the live `note` serve on `:6080` | `wr4-live-agent-gate.txt` | **20 passed / 0 failed / 0 skipped of 20**, exit 0 — §5.2 |
| 7 | **`client-e2e`**, headless-pinned | `wr4-client-e2e.txt` | **15/17** — §5.3, §5.4 |
| 8 | `cargo test -p semio-framework-os-mcp --lib workspace::` (rule 25 private target dir) | `wr4-workspace-tests.txt` | **48 passed / 0 failed**, 151 s — §5.5 |
| 9 | `cargo test -p semio-framework-os-kernel --lib os_spr::channel` | `wr4-spr-channel-laws.txt` | 87 passed / 2 failed — §5.6, neither in a file this slice touched |

### 5.1 The four properties, proven against a real guest

`a_prepared_action_applies_nothing_and_its_commit_applies_exactly_once`, driving `🗒️note`'s real
compiled component over the headless ingress:

```
[WR4] prepare note.s.note.note@1/*#editor.addBlock: Ok([Emit { ops: PreparedOps { document: [[1, 12, 8, 0, 4, 84, 101, 120, 116, …]], config: [], draft: [] } }])
[WR4] stage:  [TransactionPrepared { txn_id: "wr4-two-phase" }]
[WR4] commit: [TransactionCommitted { txn_id: "wr4-two-phase", edit_id: "edit-805759acadec8e1d" }]
[WR4] undo:   [TransactionUndone { group_id: "wr4-two-phase" }]
[WR4] second undo of the same group: Err(Fault { code: "plugin.internal",
      message: "transaction_undo: this instance's tail edit does not belong to group \"wr4-two-phase\"" })
test result: ok. 1 passed; 0 failed
```

| property | how it is asserted |
| --- | --- |
| prepare produces ops | a real `AppFrame::Emit` carrying one decoded document-lane op — only reachable if the guest resolved the owner-qualified address, admitted the window view, passed the classification and ran `command_from_action` + `A::handle` |
| **prepare applies nothing** | the document witness is byte-identical across it (`pack:299 spr:223` → `pack:299 spr:223`). `note` is event-sourced, so a prepare that applied would have appended to the `.spr` sidecar; equality is a real proof here, not a weak one |
| staging applies nothing | same witness across `TransactionPrepare` |
| **invoke applies exactly once** | the commit moves the document (`spr 223 → 671`) **and** the command log holds exactly `1@transaction:wr4-two-phase` over a document that had `0@`. A prepare routed through the applying dispatch lane would have made this 2 |
| **undo reverts that one application** | a SECOND `TransactionUndo` of the same group is refused by name. The bytes cannot say this — an undo is itself appended (`671 → 765`) — but `transaction_undo` refuses unless the store's TAIL edit belongs to the group, so the refusal is exactly the statement that one application was walked back and none remains |
| an abandoned prepare retires without effect | a further `PureCommand` whose ops are dropped leaves the witness unchanged |

### 5.2 `live-agent-loop-check` — **20 / 0 / 0 of 20**, exit 0

```
PASS 0 rendezvous            PASS boot :: ready=note windows=note-composite,note-navigator
PASS 0 context_resolve pins the shell channel :: channel=shell
PASS (f1)…(f4)               PASS (f5) agent id live-agent-loop-mua14cs9 → shell route ref
                                  note:s.note.note@1/*#editor:1, carried by [data-semio-artifact-id] in the live DOM
PASS (f6) (f7)               PASS (f8) artifact_export :: contentBase64=1764 char(s)
PASS (e1) (e2)               PASS (e3) a silent client returns the typed timeout :: APPROVAL_REQUIRED
os-mcp-live-agent-loop: 20 passed, 0 failed, 0 skipped of 20
```

Recipe (AP1 §5.3; the serve was already up — vite pid 49007, started against the m7 rendezvous,
which is why `S_AGENT_BRIDGE_DIR` is not optional for it):

```
cd "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && \
  S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:6080" S_OS_MCP_LIVE_PLUGIN=note \
  S_AGENT_BRIDGE_DIR="<ticket>/🗑️generated/m7-rendezvous" bun ./📜️script.ts live-agent-loop-check
```

Three rows moved off AP1's 16/2/1, and the run has no skip left:

- **(e3) PASS** — AP1 gap 1 read it exactly right: *"a headless-channel dispatch fault, not an
  approval fault"*. A silent elicitation-capable client with its own empty rendezvous closes the
  shell lane by construction and lands on the HEADLESS one, where it used to meet
  `typed command frames require an owner-qualified manifest command key`. It now reaches the
  approval lane and returns the typed `APPROVAL_REQUIRED` timeout the step asks for.
- **(f5) PASS, and it was a SKIP** — AP1 gap 3 (*"the gateway's artifact id and the shell's live
  document are two identities"*). §4 named them, and the row is now a **fail**, not a skip.
- **(f8) PASS** — WR3 §4.5's media OUT port, driven through a booted editor for the first time
  (WR3 §7 gap 3 asked for exactly this run).
- **step 0 channel PASS** — `channel=shell`, asserted after the socket registers (§3.2). The
  denominator is 20 because of this row.

### 5.3 `client-e2e` — headless-pinned, **15/17**

```
PASS  os: context_resolve pins the headless channel — channel=headless principal=agent:local
PASS  os: artifact_create (a real plugin artifact kind) — kind=s.animate.presentation sizeBytes=485
PASS  os: artifact_export — port=artifact:out base64Bytes=652
FAIL  os: capability catalog health — 44 diagnostic(s)                              [A3's]
FAIL  os: action_prepare — animate.…#editor.setFrame input={}:
      "presentation setFrame requires a 'frame' block"                              [animate's manifest]
client-e2e: 15/17
```

Three things this settles that no earlier run could:

1. **`channel=headless`, asserted.** WR3 §5.0's run was driving a peer's live shell; this one
   cannot (`--no-bridge`), and the gate says so as its own first row.
2. **`animate` opens headlessly in seconds.** WR3 §5.1 reported the gate's target moving to
   `animate` and `artifact_create` never answering inside 240 s. It answers here (485 B) and
   `artifact_export` passes — that was a cold component compile, not a ceiling.
3. **`action_prepare`'s refusal is the PLUGIN's own reducer talking.** Before this slice it was the
   SDK's `interactive-job.missing-exact-key`; it is now
   `presentation setFrame requires a 'frame' block`, i.e. the frame crossed the ingress, decoded as
   an owner-qualified invocation, resolved the window view, passed the proof and reached
   `animate`'s `command_from_action`. §5.4 is why that is not fixable from here.

### 5.4 Why `client-e2e`'s last dispatch row is not this slice's to green

`animate`'s committed manifest declares `setFrame` with **`"args": []`** while its handler requires
a `frame` block. The catalog therefore publishes an empty `inputSchema`, `minimalInputForSchema`
correctly produces `{}`, and **no agent could ever call this capability** — a DECLARATION defect of
the same family as the 44 `capability catalog health` diagnostics, not a dispatch one.

Walking on to the next hit was implemented, measured and **reverted**. The hit roster
(`🐍️wr4-mutation-hits.ts`) is `animate → energy → energy → gis → mathematical → …`: every hit is a
different plugin, a first `action_prepare` compiles that plugin's whole component, stdio dispatches
one request at a time, and abandoning a call client-side does not free the server. Hit 3 is `gis` at
**202 625 736 B**, whose cold compile wedged every later step behind it — the gate died at the 240 s
request wall with `action_prepare` never reported at all, which is strictly worse than a named red.
A per-attempt 90 s budget did not help, for the same serialization reason. The revert is recorded in
the journey's own comment so the next reader does not re-try it.

### 5.5 `workspace::` — **48 / 0**, and two long-standing reds went with it

Both reds this slice's first run inherited are now green:

- `plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired` — red since R2 §10.7
  and still red for WR2 (§9). Its `TransactionPrepare` leg needed WR3's paged routes compiled into
  the guest, which §5's rebuild finally gave it.
- `undo_then_redo_round_trips_a_real_probe_mutation` — a `🏪️store` drop-witness panic on the
  gateway's own `ProbeStore` in the earlier 45/3 run, on no path this slice edits. It passes in a
  quieter machine; recorded as load-dependent rather than explained.

### 5.6 The two red spr channel laws are not this slice's

`paged_generic_decoder_admits_document_config_and_projection_commands_used_during_browser_boot` and
`paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant` both fail on
**`LoadDocumentArchive`** — a route neither WR3 nor this slice added. `🧵️channel/🦀️.rs` and its unit
tests were last written at **13:06 / 13:13**, three and a half hours before this slice's first edit,
and `git status` shows them staged by their own author. 87 of 89 pass, including every route WR3
added.

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | §2.1 — `addressed_preview_view`, `preview_addressed_action`, `require_proof_operation_authority`, and `dispatch_command_frame`'s real body |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | §2.2 — `PluginArtifactChannel::addressed_action_invocation`; the `PureCommand` arm sends the owner-qualified frame. Plus §7's two transaction-path fixes: `ops_pack_lane` splits with the real framing, and `admit_reply_frame` drops a redundant `Done` stamp (`PendingResponsePage::holds_frame`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs` | §2.3 — `a_prepared_action_applies_nothing_and_its_commit_applies_exactly_once` + `note_mutation_capability_id`/`head_cursor` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | §3.1/§3.2 — `--no-bridge` on the spawned `semio` server, and the headless channel assertion as step 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` | §3.2/§4 — the shell channel assertion, and (f5) asserted against the resolvable shell route ref |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | §4 — `data-semio-artifact-id` on the shell root |

Ticket: `📜️wr4-rebuild-plugins.sh`, `🐍️wr4-mutation-hits.ts`, this report. Captures:
`wr4-plugin-check.txt`, `wr4-mcp-check.txt`, `wr4-mcp-check-alltargets.txt`, `wr4-mcp-build.txt`,
`wr4-agent-bridge-check.txt`, `wr4-plugin-rebuild.txt`, `wr4-two-phase-law.txt`,
`wr4-live-agent-gate.txt`, `wr4-client-e2e.txt`, `wr4-client-e2e-baseline.txt`,
`wr4-workspace-tests.txt`, `wr4-spr-channel-laws.txt`.
Staged: `note` 64 459 968 B, `animate` 104 268 744 B, `draw` 62 724 629 B into
`⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/`; the gateway binary rebuilt through
`bun ./📜️script.ts build` (161 461 376 B).

## 7. Two transaction-path defects this slice had to fix to get there

Neither was reachable before: WR2 stopped at `PureCommand`'s refusal and the transaction routes only
crossed the headless ingress with WR3. The first real `TransactionPrepare` found both at once.

| # | defect | why it mattered |
| --- | --- | --- |
| 1 | **`ops_pack_lane` handed the whole ops-pack blob back as ONE opaque element.** Its own doc stated the assumption — *"`TransactionPrepare`'s pre-planned form re-forwards the SAME bytes verbatim, so the framing only has to round-trip through this one port, never be individually addressed"* — and the assumption is false: `VcsArtifactApp::transaction_prepare` runs `A::Mutation::decode_op` over EVERY element. It also made an EMPTY lane non-empty (`encode_ops_vec` of nothing is a one-byte zero count, which became `[[0]]`), and this file's own `TransactionPrepare` arm refuses any prepare whose config/draft lane is non-empty — so **every** two-phase invoke would have been refused for carrying config ops it never emitted. Fixed with `store::causal::decode_ops_vec`, the real framing. |
| 2 | **A transaction route's `Done` stamp read as a duplicate answer.** Every transaction arm publishes its terminal frame AND an `AppFrame::Done{in_reply_to}` on the same turn, and `admit_reply_frame` admitted both — so a guest that answered exactly once was reported as *"the guest published more than one frame answering seq N"*. `Done` is now dropped once a real frame is held, and is still the answer for the verbs that publish nothing else (`LoadDocument`, `TransactionUndo`/`Redo`). A genuine duplicate stays as loud as WR2 made it. |

### 7.1 The rebuild, for the record

The guest SDK change reaches a gate only through a component rebuild. `📜️wr4-rebuild-plugins.sh`
builds `note`, `animate` and `draw` — the three components both permanent gates touch — into a
PRIVATE uplift dir (rule 25) and stages each with rm+cp into the shared one, all under the fleet
wasm mutex (rule 27). Two notes for the next reader: the first submission sat behind a lock held
1 h 20 m by a `tc1` task at 0 % CPU with no child process, and then failed instantly on
`status=0` — **`status` is read-only in zsh**, which burned the slot without building anything.

## 8. Honest gaps

1. **Only `🗒️note` has RUN the two-phase split end to end.** The law drives one real guest; the
   host half is exercised against `animate` by `client-e2e` as far as that plugin's own reducer
   (§5.4). `draw` was rebuilt on the new SDK but no law drives its prepare. Every other staged
   component still carries the old `dispatch_command_frame` until its owner rebuilds it — the same
   shape as WR3 §7 gap 6, and the refusal it gives is precise.
2. **`A::handle` is called with `ArtifactView::new(snapshot, history)`** — the view a plugin's own
   reducer laws use — not with the composed-children/tool-run overlay `start_typed_command_operation`
   assembles for the applying path. An app whose reducer reads `ChildContentView` or a live tool-run
   overlay would preview against less context than it applies against. No plugin in the two gates
   does; it is a real difference and it is named rather than hidden.
3. **The preview ignores `Emit`'s non-op lanes.** `effects`, `events`, `child_emits`,
   `interaction_writes`, `window_config_mutations` and the inline-interaction fold are dropped: a
   preview that requested a host effect would be requesting it twice. The ops are the whole of what
   `TransactionPrepare` can carry, so nothing that survives to the commit is lost — but a verb whose
   observable behaviour is an effect rather than an op previews as zero ops.
4. **`PreparedOps.config`/`draft` still have no wire.** `PluginArtifactChannel`'s
   `TransactionPrepare` arm refuses them by name (the real wire carries one flat document-lane list,
   WR2's finding). A previewed verb that emits config or draft ops therefore prepares fine and
   cannot be invoked. Neither gate's verb does.
5. **`client-e2e`'s dispatch row is red on `animate`'s manifest, and the target moves** (§5.4).
   The journey takes hit 0 of `capabilities_search({query:"set", kind:["mutation"]})`, which moved
   from `draw` to `animate` when A3 repaired `animate`'s descriptor and will move again. The search
   is deliberately NOT pinned — "the first mutation the catalog offers" is the honest agent
   behaviour — and walking past a refusal was measured and reverted for the serialization reason in
   §5.4. Declaring `setFrame`'s `frame` argument in `✏️s/🔌️plugins/🎞️animate`'s manifest and
   re-running its own `describe` is what greens that row; it belongs to `animate`'s owner.
6. **`capability catalog health` stays red on `client-e2e`** — 44 diagnostics, A3's descriptor
   regeneration (WR2 §8, WR3 §5.1). Not this slice's and not touched.
7. **`undo_then_redo_round_trips_a_real_probe_mutation` is red and is not explained** (§5.2 item 3).
   It is a `🏪️store` drop-witness panic on the gateway's own `ProbeStore`, on no path this slice
   edits, and it was not red in WR2's run of the same filter.
8. **`--no-bridge` also closes `ui_focus`/`ui_reveal`/shell approvals for `client-e2e`.** That is
   correct for a gate that owns the headless route — those three are the shell route's surface and
   `live-agent-loop-check` measures them — but it does mean the headless gate can never regress-test
   them. Named so nobody later reads their absence as coverage.
9. **`live-agent-loop-check`'s denominator is now 20, not 19.** The channel row was added; a reader
   comparing against AP1's "of 19" should compare the reds, not the totals.
