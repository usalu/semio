# Pure Command Producer Census

Read-only filesystem census includes tracked and untracked authored Rust/TypeScript/JavaScript under framework, excluding build/dependency caches and generated directories. The outer AppChannel command wire remains byte-compatible; only its opaque command payload now requires PureCommandRequest.

## 🧰️framework/🛍️products/💻️os/🧪️tests/🧾️app-channel-codec/🔸️tags-appframe-variant-per/🟦️.ts

```text
84: ["PureCommand", { PureCommand: { seq: 1, command: [1], head: [2] } }],
106: PureCommand: "0d0101010102",
```

## 🧰️framework/🛍️products/💻️os/🧪️tests/🧾️app-channel-codec/🔶️round-trips-app-typed/🟦️.ts

```text
45: { PureCommand: { seq: 17, command: [1], head: [2] } },
46: { PureCommand: { seq: 41, command: [1], head: [] } },
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs

```text
599: let request: semio_framework::kernel::PureCommandRequest = semio_framework_plugin_plugin_program::decode_wire_serialized(command_bytes).await.map_err(|_| {
603: "pure command bytes require one schema-owned PureCommandRequest with a declared invocation and optional explicit caller context",
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧱️components/🦀️.rs

```text
34: /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs

```text
85: /// still lives in guest `VcsArtifactApp` until `AppCommand::PureCommand` + host `dispatch` land;
157: /// have no instance id). Typed stores and the command log remain guest-side until PureCommand apply.
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs

```text
113: /// remain guest-owned until `AppCommand::PureCommand` returns `AppFrame::Emit` for host apply.
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs

```text
305: /// 🧾 Drains the last Emit op packs captured during `handle_command_frame` (PureCommand path).
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs

```text
309: /// and frames the result EXACTLY like `AppCommand::PureCommand` does (`AppFrame::Emit`, carrying the
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs

```text
301: Some(protocol::AppCommand::PureCommand{command,head,..})=>{
303: let request:semio_framework::kernel::PureCommandRequest=decode_wire_serialized(command).await?;
364: protocol::AppCommand::PureCommand{seq,..}=>{let(result,EmitWire{document:document_ops,config:config_ops,draft:draft_ops,children:child_ops})=original_preview.take().expect("original preview completes before decoded command donation");flags.mutated=true;frames.push(protocol::AppFr
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🚏️artifact-routes/🟦️.ts

```text
56: expect(decodeShellAppCommand(appPayloadBytes("pureCommand"))).toEqual({ kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: { text: "hello" } });
309: const command = step.kind === "pureCommand" ? { kind: step.kind, capabilityId: "note.note.appendParagraph", input: { text: "hello" } }
322: expect((await send("2", { kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: [] }))[0]?.kind).toBe("error");
323: const emitted = await send("2", { kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: {} });
341: const frames = await send(instanceId, { kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: { instanceId } });
355: const frames = await send("2", { kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: {} });
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts

```text
124: *    `ReadHistory → PureCommand → TransactionPrepare → TransactionCommit`; the shell has no
125: *    "compute the ops but do not apply them" seam, so `pureCommand` returns a DEFERRED plan and
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🟦️.ts

```text
135: case "pureCommand": {
140: const appCommand = live.app.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}cmd.${command.id}`);
141: const mode = live.app.modes?.find(mode => mode.commands?.some(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}mode.${mode.id}.${command.id}`));
142: const modeCommand = mode?.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}mode.${mode.id}.${command.id}`);
143: const pluginCommand = loadedPluginsRef.current.find(entry => entry.handle.pluginId === live.pluginId)?.manifest.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${live.pluginId}.cmd.${command.id}`);
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧪️tests/🟦️.test.ts

```text
81: const preview = (await send("2", { kind: "pureCommand", capabilityId: fixture.commandCases[0].id, input: {} }))[0];
95: const frame = (await send("2", { kind: "pureCommand", capabilityId: row.id, input: { value: 7 } }))[0];
166: const emitted = (await send("2", { kind: "pureCommand", capabilityId: fixture.commandCases[0].id, input: {} }))[0];
180: const frame = (await send("2", { kind: "pureCommand", capabilityId: row.id, input: {} }))[0];
194: const frame = (await send("2", { kind: "pureCommand", capabilityId: "note.note.inspectRow", input: {} }))[0];
206: const command = step.kind === "pureCommand" ? { kind: step.kind, capabilityId: "note.note.appendParagraph", input: { text: "hello" } }
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/📨️app-command-laws/🦀️.rs

```text
167: assert_command_round_trips(&AppCommand::PureCommand { seq: 18, command: vec![1], head: vec![2] }).await;
168: assert_command_round_trips(&AppCommand::PureCommand { seq: 19, command: vec![1], head: Vec::new() }).await;
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/📚️fixture-corpus-laws/🦀️.rs

```text
30: ("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], head: vec![2] }),
117: "PureCommand" => "0d0101010102",
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/🧾️command-owner/🦀️.rs

```text
27: PureCommandFields { seq: u64, fields: Vec<Vec<u8>> },
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/🧭️command-cursor/🦀️.rs

```text
30: 13 => PagedAppCommandDecodeState::PureCommandFields { seq, fields: Vec::with_capacity(PURE_COMMAND_FIELDS) },
168: PagedAppCommandDecodeState::PureCommandFields { seq, mut fields } => {
172: self.state = PagedAppCommandDecodeState::PureCommandFields { seq, fields };
178: self.state = PagedAppCommandDecodeState::PureCommandFields { seq, fields };
183: Some(AppCommand::PureCommand { seq, command: next(), head: next() })
286: AppCommand::PureCommand { command, head, .. } => vec![command, head],
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/📦️archive-decode/🦀️.rs

```text
8: /// 🧮️ `AppCommand::PureCommand`'s two byte fields, in `encode_app_command`'s own order: `command`, then the `head`
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🗜️codec/📥️command-decoder/🦀️.rs

```text
23: 13 => AppCommand::PureCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)?, head: protocol::read_bytes(bytes, &mut pos)? },
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🗜️codec/📤️command-encoder/🦀️.rs

```text
97: AppCommand::PureCommand { seq, command, head } => {
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/📨️app-command/🦀️.rs

```text
73: PureCommand {
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧠️wasmtime-node-host/🦀️.rs

```text
907: | AppCommand::PureCommand { seq, .. }
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs

```text
245: /// revision (Observe) → dry-run `PureCommand` (Preview) → mint `prep_`.
282: let (ops, warnings, output) = match self.exchange_one(instance, AppCommand::PureCommand { capability_id: capability_id.to_string(), input: input.clone(), view_state: context })? {
284: other => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from PureCommand: {other:?}"))),
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs

```text
138: /// 🪞️ The exact PureCommand op payload (preview) and the exact TransactionPrepare op payload
140: /// on internal state. `MockArtifactChannel::handle` derives its `PureCommand` Emit bytes
158: assert!(log.iter().any(|(_, command)| matches!(command, AppCommand::PureCommand { .. })), "a PureCommand was sent during preview");
168: assert_eq!(prepare_ops, vec![expected_op], "the op bytes committed must be byte-identical to what PureCommand emitted during preview");
726: let sent = || channel.frame_log().into_iter().filter_map(|(_, command)| match command { AppCommand::PureCommand { input, .. } => Some(input), _ => None }).last();
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🔌️channel-port/🦀️.rs

```text
31: /// 🗝️ The named arguments an action's JSON input carries, as the framework's own values: the members of an object, none for a null input, a named refusal for anything else. The one place a channel implementation meets the JSON type of [`AppCommand::PureCommand`].
44: /// never a single stream). `Emit` returns this from `PureCommand`; `TransactionPrepare` sends the
107: PureCommand { capability_id: String, input: serde_json::Value, view_state: Option<semio_framework::ViewModel> },
567: AppCommand::PureCommand { .. } if self.empty_preview => AppFrame::Emit { ops: PreparedOps::default(), warnings: Vec::new(), output: self.preview_output.clone() },
568: AppCommand::PureCommand { .. } if self.child_preview.is_some() => AppFrame::Emit { ops: PreparedOps { children: self.child_preview.clone().unwrap_or_default(), ..PreparedOps::default() }, warnings: Vec::new(), output: self.preview_output.clone() },
569: AppCommand::PureCommand { capability_id, input, .. } => {
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🐚️channel/🦀️.rs

```text
105: AppCommand::PureCommand { capability_id, input, view_state } => serde_json::json!({ "kind": "pureCommand", "capabilityId": capability_id, "input": input, "context": view_state }),
358: AppCommand::PureCommand { capability_id, .. } => Some(capability_id.as_str()),
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs

```text
70: assert_eq!(encode_app_command(&AppCommand::PureCommand {  capability_id: "note.note.appendParagraph".to_string(), input: serde_json::json!({ "text": "hello" }), view_state: None }), fixture("pureCommand"));
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs

```text
106: let fault = router.exchange(0, vec![AppCommand::PureCommand {  capability_id: "totally.unknown.capability".to_string(), input: serde_json::json!({}), view_state: None }]).expect_err("unknown capability must not route to any plugin");
113: let fault = router.exchange(0, vec![AppCommand::PureCommand {  capability_id: "capabilities.search".to_string(), input: serde_json::json!({}), view_state: None }]).expect_err("a gateway-owned capability names no plugin channel");
120: let fault = router.exchange(0, vec![AppCommand::ReadHistory]).expect_err("no known plugin for this instance and no PureCommand to derive one from");
132: /// proving `route_for_slot` decodes the SAME app the earlier `PureCommand` did, and that this
175: let note_result = router.exchange(note_instance, vec![AppCommand::PureCommand {  capability_id: note_verb.clone(), input: serde_json::json!({}), view_state: None }]);
178: let cad_result = router.exchange(cad_instance, vec![AppCommand::PureCommand {  capability_id: cad_verb.clone(), input: serde_json::json!({}), view_state: None }]);
209: let independent = serde_json::from_str::<semio_framework::kernel::PureCommandRequest>(&json);
210: let owned = semio_framework_pack_json::from_json_str::<semio_framework::kernel::PureCommandRequest>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject);
216: let roundtrip: semio_framework::kernel::PureCommandRequest = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(&bytes).expect("owned Pack")).expect("owned pure envelope");
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs

```text
177: /// real answer back, never `channel.not-wired`. `PureCommand`/`TransactionPrepare` are asserted
216: let pure = channel.exchange(0, vec![AppCommand::PureCommand {  capability_id: "note.editor.noop".to_string(), input: serde_json::json!({}), view_state: None }]);
217: println!("[W3] real PureCommand round trip: {pure:?}");
218: assert!(pure.as_ref().err().map(|fault| fault.code.as_str()) != Some("channel.not-wired"), "PureCommand must be a real round trip, not the old host-side short-circuit: {pure:?}");
399: /// without effect (a second `PureCommand` whose ops are dropped leaves the head where it was).
446: let prepared = channel.exchange(0, vec![AppCommand::PureCommand {  capability_id: capability_id.clone(), input: input.clone(), view_state: None }]);
450: other => panic!("PureCommand must answer a real Emit frame, received {other:?}"),
492: let abandoned = channel.exchange(0, vec![AppCommand::PureCommand {  capability_id: capability_id.clone(), input, view_state: None }]).expect("a second preview answers");
546: let ops = match channel.exchange(0, vec![AppCommand::PureCommand {  capability_id: capability_id.clone(), input, view_state: None }]).expect("the guest previews addBlock").into_iter().next() {
548: other => panic!("PureCommand must answer Emit, received {other:?}"),
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧩️artifact-app-frames/🦀️.rs

```text
41: | store::AppCommand::PureCommand { seq, .. }
493: /// `txn_id`/`group_id` — no plugin-specific payload, so (unlike `PureCommand`/
533: AppCommand::PureCommand { capability_id, input, view_state } => {
534: let arguments = action_input_arguments(&input).map_err(|detail| Self::not_wired("PureCommand", detail))?;
541: let request = semio_framework::kernel::PureCommandRequest { invocation, view_state };
545: store::AppCommand::PureCommand { seq: 0, command, head: Vec::new() },
553: other => return Err(Self::not_wired("PureCommand", format!("unexpected real AppFrame variant {other:?}"))),
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🧩️artifact-creation-and-routing/🦀️.rs

```text
107: /// `RoutingArtifactChannel` resolves a `PureCommand` through, and what `prepare_action` below
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🔬️probe-documents/🦀️.rs

```text
64: /// `PluginArtifactChannel::exchange`'s `PureCommand`/`TransactionPrepare` arms for exactly why a
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🏛️gateway-backend-impl/🦀️.rs

```text
31: /// that arm's own honest gap note (`PureCommand`'s guest-side handler unconditionally rejects
36: /// `ReadHistory` (no capability id) before its `PureCommand`, so `RoutingArtifactChannel` must
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔀️routing-and-shell-routed/🦀️.rs

```text
45: /// `AppCommand::ReadHistory` (no capability id at all) BEFORE the `PureCommand` that names one
137: /// needs, not a whole `&HeadlessWorkspace`) can run the identical lookup for a `PureCommand` it is
307: /// one plugin or bust" (`📓️w8-capability-routing.md`). A `PureCommand{capability_id,..}` names its
395: if !commands.iter().any(|command| matches!(command, AppCommand::PureCommand { .. } | AppCommand::TransactionPrepare { .. } | AppCommand::TransactionRedo { .. })) {
527: if let AppCommand::PureCommand { capability_id, .. } = command {
539: AppCommand::PureCommand { capability_id, .. } => return resolve_route_for_capability_in(&catalog, capability_id).map_err(routing_fault),
548: "instance {instance} names no plugin — this workspace's catalog owns {} plugin(s); route a PureCommand for a specific capability on this instance first",
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🧱️channel-record/🦀️.rs

```text
13: /// variants (`PureCommand`/`TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo`) need a real
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/💡️inference-route-and-actions/🦀️.rs

```text
146: return Err(Self::not_wired("PureCommand", format!("app `{}` declares action `{}` but no window kind at all, so there is no pane an agent could address it in", app.id, action.id)));
164: let arguments = arguments().map_err(|detail| Self::not_wired("PureCommand", detail))?;
```

## 🧰️framework/🔨️modules/🪢️wire-contracts/🟦️.ts

```text
116: | { readonly PureCommand: { readonly seq: number; readonly command: readonly number[]; readonly head: readonly number[] } }
```

## 🧰️framework/🔨️modules/🧾️app-channel/🏷️codec-primitives/🟦️.ts

```text
40: MediaFingerprint: 12, PureCommand: 13, LoadChildren: 14, ReadChildren: 15, ReadHistory: 16,
```

## 🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts

```text
114: } else if ("PureCommand" in cmd) {
115: out.push(APP_COMMAND_TAGS.PureCommand);
116: writeVarintU64(out, cmd.PureCommand.seq);
117: writeBytes(out, cmd.PureCommand.command);
118: writeBytes(out, cmd.PureCommand.head);
285: case APP_COMMAND_TAGS.PureCommand:
286: return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), head: readBytes(bytes, pos) } };
```

## 🧰️framework/🔨️modules/🧬️schema/📽️projection/🎟️rows-h-t/🦀️.rs

```text
13: name: "PureCommandRequest",
16: export type PureCommandRequest = { invocation: PureInvocation, viewState?: ViewModel };"####,
```

## 🧰️framework/🔨️modules/🤖️mcp/🐚️channel/🟦️.ts

```text
91: | { readonly kind: "pureCommand"; readonly capabilityId: string; readonly input: unknown; readonly context?: ResolvedPluginViewState | null }
165: case "pureCommand":
166: return { kind: "pureCommand", capabilityId: requireString(record, "capabilityId"), input: record.input, ...(record.context == null ? {} : { context: parseResolvedPluginViewState(record.context) }) };
```

## 🧰️framework/🔨️modules/🎠️kernel/🦀️.rs

```text
610: pub struct PureCommandRequest {
```

## 🧰️framework/📦️packages/🦀️rust/🦀️.rs

```text
4: pub use semio_framework_kernel::{ActionContext, ActionRequest, ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, ActorUiPatchReceipt, ArtifactDiff, ArtifactMergeKind, BrokerHooks, CLIPBOARD_FRAGME
```

## Classification And Migration Result

The sole semantic AppChannel PureCommand.command Pack producer found is MCP workspace artifact-app-frames, now emitting the mandatory kernel PureCommandRequest. The guest runtime and instance dispatcher both require this envelope; there is no raw ActionInvocation or CommandInvocation fallback. The TypeScript app-channel codec copies an already opaque command byte buffer; it does not construct invocation payloads. Rust/TypeScript outer-wire roundtrip fixtures use arbitrary single-byte payloads to test transport opacity and therefore are not stale raw manifest invocations.

The actor shard client public facade and actual actor/shard-client source tree were inspected separately, including its wire/messages/activation modules. They transport encoded frames and expose no PureCommand semantic payload constructor; searches for PureCommand, pureCommand, pure_command, pure-command and action/command invocation construction found no producer there. The framework package actor facade remains unchanged. Standalone action and command codecs used by genuine applying invocation APIs remain unchanged. A tracked-source search outside framework found no additional PureCommand source.

New JSON Schema and neutral corpus live under kernel/schema/pure-command and kernel/tests/pure-command. Strict Ajv validation passed all four explicit-context, retained-context, missing-language and raw-invocation cases. The corresponding native own JSON/Pack versus independent serde_json test is staged in MCP workspace quick tests and requires the next native compilation.
