# 📓️ WR2 — the headless command/response wire (the guest acknowledges a command and answers nothing)

Slice: outcome 4's headless lane, continuing WR1. WR1 put the MCP gateway's plugin host on
`WasmtimeRuntime` (`🖍️draw` open >240 s → **34 ms**, `client-e2e` from a dead 240 s tool call to
**9.7 s at 12/16**). The remaining blocker WR1 measured (§5.4): for a command the guest drives
ingress to `CommandBatchProgress::Complete`, then goes `Idle` with `next_wake == None` having
published **no `Effect::Respond` at all**. The seq-0-vs-envelope-seq hypothesis was instrumented and
disproven by WR1 — because it was tested on a lane the answer never travels.

**Root cause, measured: the gateway was reading the wrong lane.** A command's answer is never an
`Effect::Respond`. It is an `Effect::SendMessage{target: Shell{instance}}` carrying encoded
`AppFrame` bytes. The answer was sitting in the same turn's effects, unread.

**Gate: `client-e2e` 12/16 → 13/16** (`🗑️generated/wr2-client-e2e.txt`).
`artifact_create (a real plugin artifact kind)` is **green for the first time in this ticket**: a
real `s.draw.drawing` artifact, 615 bytes, seeded from `draw`'s own genesis document. The three reds
left each fail on a named, different mechanism (§7).

## 1. Inherited state

`git status` showed no prior WR2 edit and `🗑️generated/` carried no `wr2-*` capture; what was
inherited is WR1's measurement and WR1's `12/16` gate capture, plus R2 §10.7's unnamed
`command ingress: faulted for seq 1 after bounded exact-owner cleanup` on the channel law.

## 2. The React host's turn, frame by frame

`🔌️PluginRuntime/🟦️.tsx` `runQueuedTurn` (the live, shipping consumer of the identical machinery):

| step | what |
| --- | --- |
| 1 | `inspectEncodedAppCommand(events)` reads **the `AppCommand`'s OWN `seq`** — that is the correlation key |
| 2 | `createShardCommandIngressPages({owner, generation, commandIndex, instance, seq, command})`, one `submitTurn` per page |
| 3 | continue turns until `commandIngress.tag === "command-complete"`, bounded by `commandIngressContinuationCeilingV1`, with `commandIngressUnownedV1` as the progress rule |
| 4 | `settleAcknowledgedPluginTurns(...)` — the settle step, which folds every turn's effects |
| 5 | **the answer**: `for (const effect of routeHostEffects(...)) { const frame = shellFrameBytes(effect, instanceId); if (frame) outFrames.push(frame); }` |
| 6 | `commandIngressNeedsReplyStampV1(outFrames.map(encodedFrameReplySequence), inspected.seq)` → if no frame carries `in_reply_to === seq`, push `AppFrame::Done{in_reply_to: seq}` |

`shellFrameBytes` (`🎭️actor/🖼️wire-turn/🟦️.ts:134`) is the whole demux: a `Shell{instance}` payload is
an `AppFrame` unless it starts with `TYPED_OPERATION_PAGE_MAGIC`/`TYPED_OPERATION_ACK_MAGIC`. Its
own doc says it outright — *"a `Shell{instance}` send-message IS an `AppFrame` reply"*.

The guest half confirms it: `⚛️reactor/🔄️turn/🦀️.rs`'s `route_app_frame` sends every non-`UiPatch`
frame as `Effect::SendMessage{ target: Shell{instance}, payload: encode_app_frame(frame) }`, and
`Effect::Respond` is pushed in exactly one place — `inbound_request_effects`, under
`Event::Request`, the extension-capability seam (`🔄️turn/🦀️.rs:841`). **The gateway never sends an
`Event::Request`, so it could never receive a `Respond`.**

## 3. The headless gateway's turn, frame by frame — and the two defects

`🌉️mcp/🏠️workspace/🦀️.rs`, as found:

1. `exchange_one_turn` minted an envelope `seq` from `self.next_seq` and encoded a `store::AppCommand`
   whose own `seq` was a hardcoded **`0`** at all twelve call sites in `exchange`.
2. both the turn loop and `await_response` scanned `turn.effects` for `Effect::Respond{req}` with
   `req.0 == seq` — a lane that carries nothing here.
3. `PendingResponsePage` held one `FixedCommandPage`, i.e. a **4096-byte ceiling** on the answer,
   which is what a `RequestOutcome` is. An `AppFrame::Document{pack, spr}` is not: it would have been
   refused as `guest success response exceeds the fixed 4096-byte MCP response authority` the moment
   the lane was read correctly.

So even a host reading the right lane would have correlated nothing (defect 1), and even a host
correlating correctly would have refused the answer (defect 3).

## 4. The measurement

`🗑️generated/wr2-law-before.txt` — the new law (§6) run against the tree as WR1 left it, with one
added instrumentation: the fault now names what the guest DID publish.

```
[WR2] real ReadArtifact round trip: Err(Fault { code: "channel.not-wired", message:
  "the guest acknowledged command seq 1 and then went idle without publishing a response for it;
   it published no effects at all;
   the acknowledging turn published sendMessage(shell:0, app-frame, 579 B), sendMessage(shell:0, app-frame, 252 B)" })
```

**Two app-frames, 579 B and 252 B, addressed at this very instance, on the acknowledging turn.** The
answer was never missing. `await_response` then woke a guest that had already said everything it had
to say, found `no effects at all`, and reported an idle guest.

## 5. Root fix

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`.

| # | what |
| --- | --- |
| 1 | `app_command_seq_mut` — exhaustive `&mut u64` over every `AppCommand` variant (the write-side twin of `🏃️run/🦀️.rs`'s `app_command_seq`). `exchange_one_real` mints ONE sequence per command and **stamps it into the command itself**, so the envelope and the `AppCommand` carry the same number and `in_reply_to` correlates. `exchange_one_turn` takes the seq rather than minting its own |
| 2 | `shell_app_frame_payload` — the Rust twin of `shellFrameBytes`: a `Shell{instance}` message addressed at this instance, minus the two typed-operation magic prefixes |
| 3 | `app_frame_reply_seq` (exhaustive) — the three correlation shapes: `in_reply_to: u64`, `in_reply_to: Option<u64>` (frames also pushed unsolicited), and none at all |
| 4 | `app_frame_is_transaction_terminal` — `TransactionPrepared`/`Committed`/`RolledBack` carry no reply sequence because they correlate on their own `txn_id`, which the caller's own arm checks |
| 5 | `admit_reply_frame` replaces the `Effect::Respond` scan in the turn loop; `await_response` scans the same lane |
| 6 | `PendingResponsePage` now holds the guest's frame bytes under the **declared** host-answer ceiling, `COMMAND_MAXIMUM_BYTES` (8 MiB, `GUEST_HOST_ANSWER_CEILING_BYTES`), not one 4 KiB page; `close_step` releases one grant at a time so a faulted command's cleanup stays bounded however large the queued answer |
| 7 | `PendingResponsePage::Stamped` + `stamp_settled` — a settled command that published no frame naming itself answers `AppFrame::Done{in_reply_to: seq}`, the React host's own `commandIngressNeedsReplyStampV1` rule. Without it every verb whose guest legitimately has nothing to say (`LoadDocument`, `TransactionUndo`/`Redo`) would fault |
| 8 | a `Respond` under a request id this gateway never sent is still a typed fault — that shape really is a routing defect |
| 9 | `ingress_fault_text` + `PendingExchange.ingress_fault` — the guest's OWN reason for a faulted ingress, captured on the turn that reports it and carried through the bounded cleanup. It used to be discarded: `command ingress: faulted for seq N after bounded exact-owner cleanup` (R2 §10.7's reading) named nothing |
| 10 | `effect_shape`/`named_shapes` — a fault that cannot find the answer names every effect the guest published, by shape and never by payload |

## 6. The native law — red without the fix, green with it

`🏠️workspace/🧪️tests/🔬️long/🦀️.rs`
`a_real_app_command_is_answered_over_the_shell_message_lane_not_the_respond_lane`: opens a real
channel to `🗒️note`'s compiled `.wasm`, drives `AppCommand::ReadArtifact`, and asserts a real
`AppFrame::Artifact` with a non-empty pack.

| run | result |
| --- | --- |
| before (`wr2-law-before.txt`) | **FAILED** — §4's measurement, 109.65 s |
| after (`wr2-law-after2.txt`) | **ok** — `Artifact { pack: [137, 83, 69, 77, … "note.note.pack v1" …], spr: [137, 83, 80, 82, …] }`, a real 250-byte `.spk` container and its `.spr` index |

`🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`: the two `PendingResponsePage` laws were rewritten for the new
carrier — `pending_response_close_releases_one_grant_at_a_time` (bounded retirement of an
oversized answer) and `pending_response_faults_oversize_and_duplicate_and_stamps_a_silent_settlement`
(the ceiling, the duplicate, and the `Done` stamp). Both green.

## 7. The gate — `🗑️generated/wr2-client-e2e.txt`, **13/16**

```
PASS  os: artifact_create (a real plugin artifact kind) —
      artifactId=mcp-client-e2e-mu9oa8c7-typed kind=s.draw.drawing pluginId=draw sizeBytes=615
```

| row | WR1 (12/16) | WR2 (13/16) |
| --- | --- | --- |
| `artifact_create` (plugin kind) | `the guest acknowledged command seq 1 and then went idle without publishing a response for it` | **PASS** — a real 615-byte `s.draw.drawing` artifact |
| `action_prepare` | same fault | `typed command frames require an owner-qualified manifest command key before deserialization` — the guest's own `dispatch_command_frame`, reached for the first time |
| `artifact_export` | `PLUGIN_UNAVAILABLE` (the create never landed) | `plugin \`draw\` declares no media output port to export … through` (`declaredExportFormats: []`) — the create landed; this is a declaration gap |
| capability catalog health | 58 diagnostics | 58 diagnostics — a peer's descriptor regression, §8 |

### 7.1 `action_prepare` — root-caused, and it is a window, not a wire

The capability the gate prepares, `draw.s.draw.drawing@1/*#editor.setSnapshot`, is declared at
`manifest/apps[0]/windowKinds[0]/actions[4]` of `✏️s/🔌️plugins/🖍️draw/🔣️.json` — a **window-kind
action**, `kind: "mutation"`. `draw`'s app-level `commands` list is empty.

The gateway sends it as `store::AppCommand::PureCommand{command: {capabilityId, input}}`. In the
guest (`🔌️plugin/🦀️.rs:38151` → `:26766`) a bare typed frame lands on `dispatch_command_frame`,
which refuses **by design**: *"a bare typed frame has no owner-qualified exact command key, so it
cannot select a command-specific pre-serde envelope and is rejected without inspecting its payload"*.

The lane that does dispatch is `AppCommand::Command{seq, command, view_state}` (`🦀️.rs:37749`), whose
payload decodes as `ManifestActionInvocation` first and `ManifestCommandInvocation` second. Its tag
**is** admitted by the paged decoder (`1 => CommandPayload`), so the transport is not the obstacle.
The obstacle is the first thing that arm does:

```rust
let addressed_view = meta.view_state.as_ref()
    .ok_or_else(|| plugin_internal_fault("action context is missing viewState"))
    .and_then(|view| admit_addressed_action_view(view, &invocation));
```

An action is addressed at `ActionAddress{plugin_id, app_id, mode_id, window_kind_id,
window_instance_id, action_id}` inside a live `ViewModel`. A headless gateway has no window
instance, and `PluginArtifactChannel`'s own doc has refused to invent one since P7 — *"what does a
headless agent's 'window'/'mode' even mean"*. Minting a synthetic headless view state is a real
design decision with a real alternative already live: LB1's shell route, where the windows are the
human's own. **Not taken here**; named instead, which is what this slice can honestly claim.

### 7.2 `artifact_export` — two independent gaps, both named

1. **`draw` declares no export surface at all.** Measured across every committed descriptor: no
   plugin in the repo declares a non-empty `io.exportFormats`, and `draw` declares no app-specific
   OUT port either (`writer: text:out`, `lowpoly: mesh:out`, `cad: brep:out`, … do). The gateway's
   refusal — `plugin \`draw\` declares no media output port to export … through`,
   `declaredExportFormats: []` — is therefore **true**, not a bug in the export path.
2. **`MediaOut` cannot cross the headless ingress anyway** (§10 gap 2): its tag is not one of the
   routes the paged `AppCommand` decoder admits.

So the LB1 note ("`exportMedia` is a named refusal because `PluginWasmHandle` has no media OUT
port") has a headless twin one layer deeper: there is no media OUT port because no *plugin* declares
one. Adding the port to `PluginWasmHandle` alone would not make either channel export `draw`.

## 8. The 58 catalog diagnostics — the 28 new ones by cause

Captured from the staged binary itself (`🗑️generated/wr2-catalog-diagnostics.txt`): **58 lines, 29
distinct plugins, each reported twice per process** (the registry is loaded once at startup and once
when the per-capability channel routing binds). A1/R2's 30 was **15 plugins × 2**; WR1's 58 is
**29 × 2** — so the 28 new diagnostics are exactly **14 newly-undecodable descriptors**.

| # | cause | plugins |
| --- | --- | --- |
| 10 | `did not decode as a PackageDescriptor: missing field \`artifactSchema\`` | `dag`, `demonstrator`, `forms`, `mathematical`, `puzzle`, `reasoning`, `sequence`, `sourcing`, `trinity`, `vcs` |
| 3 | `missing field \`executionProtocol\`` | `cad-extension-aec-building-energy`, `cad-extension-aec-building-structure`, `cad-extension-spatial-shape` |
| 1 | `missing field \`windowKindId\`` | `animate` |
| (15, pre-existing) | `NotFound: no committed descriptor at …` — the file does not exist | `imperative-extension-{control,effect,logic,math,text}`, `playbook`, `playbook-module-procedural`, `process-extension-{concrete,metal,robotic,wood}`, `sourcing-module-{beams,slabs,windows}`, `stdio` |

**None of the 28 is a source declaration error, so none is a one-line fix.** All 14 are *generated*
descriptors that are stale against a schema change:

- **`artifactSchema` is a RENAME.** `AppIo.artifact_schema`/`artifact_media_type`
  (`🛂️manifest/🦀️.rs:5615`) used to be `document_schema`/`document_media_type`. Measured, key for
  key: `🕸️dag/🔣️.json` (09-15) carries `['artifact','documentMediaType','documentSchema',
  'exportFormats','importFormats','ports']`; `🗒️note/🔣️.json` (09-20) carries
  `['artifact','artifactMediaType','artifactSchema',…]`. Everything else about the `io` block is
  identical. The 10 listed plugins were simply not re-described after the rename.
- **`executionProtocol` is a NEW required field** with a constant value. `📐️cad/🧩️extensions/
  🏢️aec-building/🔣️.json` (re-described 09-16) carries `"executionProtocol": {"appChannelVersion": 17}`;
  its three siblings (all 09-15) have no such key. Same `"execution": "declarative"`.
- **`animate`** is a single-line 44 KB descriptor; its app object is missing `actions`, which every
  decodable app carries — a third field of the same 191-declaration sweep.

**Deliberately not hand-repaired.** These files are self-hashing: `describe`'s emitter writes
`hashes.descriptorSha256` over the descriptor's own encoded pack in a documented two-pass scheme
(`🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:434-442`). Editing the JSON by hand leaves that hash
describing a document that no longer exists, which is a worse defect than the one it fixes. The
repair is the producer verb — `bun ./📜️script.ts describe` in each plugin's rust package, i.e. what
`📜️ds1-redescribe.sh` already automates — and each run is a `wasm32-wasip2` `wasm-dev` component
build, so 14 of them is hours of the shared build lock and belongs to whoever owns the plugin bar
sweep (B3c/B3d/F2 re-describe these plugins as part of their own activations anyway).

## 9. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | §5 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs` | the new law (§6) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` | the two `PendingResponsePage` laws rewritten for the new carrier |

No other file was touched. `✏️s/🔌️plugins/**` descriptors were **read only** (§8).

Staged binary: `🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`, 161 319 328 B, built through
`bun ./📜️script.ts build` (rm+cp+codesign; `codesign -v` clean).
Captures: `wr2-check1.txt`, `wr2-check2.txt`, `wr2-check3.txt`, `wr2-law-before.txt`,
`wr2-law-after.txt`, `wr2-law-after2.txt`, `wr2-build-mcp.txt`, `wr2-client-e2e.txt`,
`wr2-catalog-diagnostics.txt`, `wr2-workspace-tests.txt`.

### Regression check

`cargo test -p semio-framework-os-mcp --lib workspace::` (`wr2-workspace-tests.txt`, 341 s, rule 25
private target dir): **46 passed / 1 failed**. The one red is
`plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`, which was already red
before this slice (R2 §10.7) — it now fails one verb LATER (`PureCommand` reaches the guest and gets
the guest's own `interactive-job.missing-exact-key`; `TransactionPrepare` is the new stopping point)
and with the guest's own named reason instead of an anonymous one.

## 10. Honest gaps

1. **16/16 not reached; the gate is 13/16.** Three reds, three different named mechanisms (§7,
   §7.1, §7.2, §8) — none of them the command/response wire this slice owned.
2. **`TransactionPrepare` (and every other transaction verb, and `MediaOut`) is refused by the
   guest's paged `AppCommand` decoder.** Named for the first time (`wr2-law-after2.txt`):
   `plugin.command-route-state-machine-required: this AppCommand kind requires its route-specific
   retained decoder before admission` — `📡️spr/🧵️channel/🦀️.rs:1789`. That state machine admits tags
   0–4, 6–9, 13, 15, 16, 27, 29–36 and refuses everything else, which covers
   `TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo`, `MediaIn`/`MediaOut`/`MediaFingerprint`
   and `Presence`. So the ticket's undo/redo/transaction/export legs cannot cross the **headless**
   ingress at all until those routes exist — and because that decoder is compiled INTO every guest,
   adding them re-builds all 20 components. This is the single largest remaining structural gap in
   the headless lane and deserves its own slice.
3. **`action_prepare` needs a headless view state, not a wire fix** (§7.1). Deliberately not
   invented here.
4. **`artifact_export` has no plugin that declares an export surface** (§7.2). Measured across every
   committed descriptor: zero non-empty `io.exportFormats` anywhere.
5. **`headless_command_budget`'s 5 s ceiling and `await_response`'s wall are still unexercised** —
   every command now settles in milliseconds, so WR1 gaps 2 and 3 stand unchanged.
6. **Only `🗒️note` and `🖍️draw` are proven on this wire** — `note` by the law, `draw` by the gate.
   The other 18 staged components drive no command in any law.
7. **The `Done` stamp is a policy, not a measurement.** A settled command that publishes no frame
   now answers `AppFrame::Done`. That is the React host's own rule and the only way a silently
   completing verb can answer at all, but it does mean a guest that settles a command and forgets to
   publish its answer reads as a `Done` rather than as a fault. The shapes that are still loud: a
   duplicate answer, an oversized answer, and a `Respond` under a foreign request id.
8. **The inference lane is untouched and unmeasured here** (WR1 gap 6 stands).
