# 🔀️ Section C: the TS backbone codec twin carries `committed` (ordinal 4) and `sequenced` (ordinal 5), the Rust
# `BackboneMessage::Committed` / `::Sequenced`. Both flow hub → store only: every path that carries a store-bound message admits
# them, every consumer of a guest's egress refuses them.
BINDING_TS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts"
BINDING_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs"
MCP_WORKSPACE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"
WGPU_SHELL = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
TOOL_RUN_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs"
SYNC_LAWS = STORE_DIR + "🔄️sync/🧪️tests/🔬️unit/🦀️.rs"
SHELL_HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
CODEC_LAWS = "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"

edit(OS_ROOT, """export type BinaryBackboneMessage =
  | { readonly kind: "genesis"; readonly pack: Uint8Array }
  | { readonly kind: "mutations"; readonly envelopes: Uint8Array }
  | { readonly kind: "ack"; readonly opIds: readonly string[] };""", """export type BinaryBackboneMessage =
  | { readonly kind: "genesis"; readonly pack: Uint8Array }
  | { readonly kind: "mutations"; readonly envelopes: Uint8Array }
  | { readonly kind: "ack"; readonly opIds: readonly string[] }
  | { readonly kind: "committed"; readonly opIds: readonly string[] }
  | { readonly kind: "sequenced"; readonly envelopes: Uint8Array };""", "codec sequencer kinds")
edit(OS_ROOT, """  if (message.kind === "ack") {
    if (message.opIds.length > maximum / 2) throw new Error("backbone message: item limit");""", """  if (message.kind === "ack" || message.kind === "committed") {
    if (message.opIds.length > maximum / 2) throw new Error("backbone message: item limit");""", "codec committed validation")
edit(OS_ROOT, """  else if (message.kind === "mutations") checkedBytes(message.envelopes);
  else throw new Error("backbone message: unknown kind");
  const symbols = message.kind === "ack" ? packBuildSymbols(message.opIds) : [];
  const symbolIndex = new Map(symbols.map((value, index) => [value, index] as const));
  const out: number[] = [1, message.kind === "genesis" ? 0 : message.kind === "mutations" ? 1 : 2];""", """  else if (message.kind === "mutations" || message.kind === "sequenced") checkedBytes(message.envelopes);
  else throw new Error("backbone message: unknown kind");
  const symbols = message.kind === "ack" || message.kind === "committed" ? packBuildSymbols(message.opIds) : [];
  const symbolIndex = new Map(symbols.map((value, index) => [value, index] as const));
  const out: number[] = [1, { genesis: 0, mutations: 1, ack: 2, committed: 4, sequenced: 5 }[message.kind]];""", "codec sequencer ordinals")
edit(OS_ROOT, """  } else if (message.kind === "mutations") {
    byteField(0, message.envelopes);
  } else {
    out.push(0, PACK_TAG_LIST);""", """  } else if (message.kind === "mutations" || message.kind === "sequenced") {
    byteField(0, message.envelopes);
  } else {
    out.push(0, PACK_TAG_LIST);""", "codec sequenced body")
edit(OS_ROOT, """  const tag = natural(2);
  if (tag !== 0 && bytes.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("backbone message: hot byte limit");""", """  const tag = natural(5);
  if (tag === 3) throw new Error("backbone message: a member lane is not host-routed");
  if (tag !== 0 && bytes.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("backbone message: hot byte limit");""", "codec sequencer decode tags")
edit(OS_ROOT, """  } else if (tag === 1) {
    message = { kind: "mutations", envelopes: byteField(0) };
  } else {""", """  } else if (tag === 1 || tag === 5) {
    message = { kind: tag === 1 ? "mutations" : "sequenced", envelopes: byteField(0) };
  } else {""", "codec sequenced decode")
edit(OS_ROOT, """    message = { kind: "ack", opIds };
  }
  if (pos[0] !== bytes.length) throw new Error("backbone message: trailing bytes");""", """    message = { kind: tag === 2 ? "ack" : "committed", opIds };
  }
  if (pos[0] !== bytes.length) throw new Error("backbone message: trailing bytes");""", "codec committed decode")
edit(OS_ROOT, """/** 🪢 Admits one canonical hot mutation message and its exact bounded causal batch. */
export function parseDocumentBackboneMessage(message: Uint8Array): DocumentBackboneMessage {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind !== "mutations") throw new Error("document backbone: mutations required");
  const envelopes = decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes);
  return { message: message.slice(), envelopes };
}""", """/** 🪢 Admits one canonical hot mutation message and its exact bounded causal batch. */
export function parseDocumentBackboneMessage(message: Uint8Array): DocumentBackboneMessage {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind !== "mutations") throw new Error("document backbone: mutations required");
  const envelopes = decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes);
  return { message: message.slice(), envelopes };
}

/** 🧭️ Admits one canonical hot message bound for a document's store (transport owner → store): a peer's `mutations`, the
 * hub's `sequenced` operations, or its `committed` decision on this replica's own (no envelopes). A store never publishes the
 * latter two ({@link parseDocumentBackboneMessage} guards that direction). */
export function parseDocumentBackboneInboundMessage(message: Uint8Array): DocumentBackboneMessage {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind === "committed") return { message: message.slice(), envelopes: [] };
  if (parsed.kind !== "mutations" && parsed.kind !== "sequenced") throw new Error("document backbone: store-bound message required");
  return { message: message.slice(), envelopes: decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes) };
}""", "codec store-bound admission")
edit(OS_ROOT, """function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneMessage(event.message).message) };
  }""", """function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneInboundMessage(event.message).message) };
  }""", "codec worker event carries store-bound messages")
edit(OS_ROOT, """    if (Object.keys(event).sort().join(",") !== "kind,message" || !Array.isArray(event.message) || event.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !event.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document backbone message");
    return { kind: "documentBackbone", message: parseDocumentBackboneMessage(Uint8Array.from(event.message as readonly number[])).message };""", """    if (Object.keys(event).sort().join(",") !== "kind,message" || !Array.isArray(event.message) || event.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !event.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document backbone message");
    return { kind: "documentBackbone", message: parseDocumentBackboneInboundMessage(Uint8Array.from(event.message as readonly number[])).message };""", "codec worker event parses store-bound messages")

edit(BINDING_TS, """export function documentBackboneEffectV1(bytes: Uint8Array): "mutations" | "remote-ingest-receipt" {
  const message = decodeBackboneMessage(bytes);
  if (message.kind === "genesis") throw new Error("actor-document-port.genesis-requires-cold-pair");""", """export function documentBackboneEffectV1(bytes: Uint8Array): "mutations" | "remote-ingest-receipt" {
  const message = decodeBackboneMessage(bytes);
  if (message.kind === "genesis") throw new Error("actor-document-port.genesis-requires-cold-pair");
  if (message.kind === "committed" || message.kind === "sequenced") throw new Error("actor-document-port.sequencer-message-flows-hub-to-store");""", "binding refuses a guest's sequencer message")

edit(BINDING_LAWS, """        (BackboneMessage::Ack { op_ids: vec!["a".into()] }, "010201016101000c010600", true),
    ];""", """        (BackboneMessage::Ack { op_ids: vec!["a".into()] }, "010201016101000c010600", true),
        (BackboneMessage::Committed { op_ids: Vec::new() }, "01040001000c00", true),
        (BackboneMessage::Committed { op_ids: vec!["a".into()] }, "010401016101000c010600", true),
        (BackboneMessage::Sequenced { envelopes: vec![0xdd] }, "01050001000801dd", true),
    ];""", "binding law sequencer vectors")
edit(BINDING_LAWS, """    for hostile in ["", "00020001000c00", "01030001000c00", "0182000001000c00",""", """    for hostile in ["", "00020001000c00", "01030001000c00", "01060001000c00", "01050001000c00", "0182000001000c00",""", "binding law unknown ordinal and misshaped sequenced hostile")

edit(MCP_WORKSPACE, """                Ok(store::BackboneMessage::Ack { .. }) => continue,
                Ok(store::BackboneMessage::Genesis { .. } | store::BackboneMessage::Member { .. }) => {""", """                Ok(store::BackboneMessage::Ack { .. }) => continue,
                Ok(store::BackboneMessage::Committed { .. } | store::BackboneMessage::Sequenced { .. }) => {
                    return Err(Fault { code: "channel.not-wired".to_string(), message: format!("`{artifact_id}`'s guest published a sequencer's message on its hot document backbone; those flow from the hub to the store") });
                }
                Ok(store::BackboneMessage::Genesis { .. } | store::BackboneMessage::Member { .. }) => {""", "mcp workspace refuses a guest's sequencer message")

edit(WGPU_SHELL, """                    store_sync::os_store::BackboneMessage::Member { .. } => return Err("a composed member requires its exact member transport lane".into()),""",
     """                    store_sync::os_store::BackboneMessage::Member { .. } => return Err("a composed member requires its exact member transport lane".into()),
                    store_sync::os_store::BackboneMessage::Committed { .. } | store_sync::os_store::BackboneMessage::Sequenced { .. } => return Err("a sequencer's message flows from the hub to the store, never from a guest".into()),""", "wgpu shell refuses a guest's sequencer message")

edit(TOOL_RUN_LAWS, """        BackboneMessage::Ack { .. } => (mutations, genesis),
    })""", """        BackboneMessage::Ack { .. } | BackboneMessage::Committed { .. } | BackboneMessage::Sequenced { .. } => (mutations, genesis),
    })""", "tool-run law drain counts no sequencer message")

edit(SYNC_LAWS, """fn document_backbone_event_envelopes(event: &ArtifactEvent) -> Option<Vec<MutationEnvelope>> {
    let ArtifactEvent::DocumentBackbone { message } = event else { return None };
    decode_document_backbone_message_exact(message).ok()
}""", """/// 🧭️ The other authors' operations a hub document's `DocumentBackbone` event hands its store — `Sequenced`, in the hub's
/// order (a `Committed` names this replica's own operations and carries none).
fn document_backbone_event_envelopes(event: &ArtifactEvent) -> Option<Vec<MutationEnvelope>> {
    let ArtifactEvent::DocumentBackbone { message } = event else { return None };
    let BackboneMessage::Sequenced { envelopes } = decode_hot_backbone_message_exact(message).ok()? else { return None };
    decode_document_backbone_envelopes_exact(&envelopes).ok()
}""", "sync law helper reads a hub document's sequenced event")

edit(SYNC_LAWS, """    assert!(receiver.try_recv().is_none(), "member refusal leaves root document ingress empty");
""", """    assert!(receiver.try_recv().is_none(), "member refusal leaves root document ingress empty");
    for sequencer in [BackboneMessage::Committed { op_ids: vec!["mutation-a".into()] }, BackboneMessage::Sequenced { envelopes: encode_envelopes(std::slice::from_ref(&envelope)) }] {
        let message = sequencer.encode_op().expect("sequencer message encodes");
        assert!(matches!(sender.send(ArtifactActorMsg::DocumentBackbone { message }), Err(ArtifactMailboxSendError::Bytes { .. })), "host ingress refuses a sequencer's message: it flows from the hub to the store");
    }
    assert!(receiver.try_recv().is_none(), "sequencer refusals leave root document ingress empty");
""", "sync law host ingress refuses sequencer messages")

edit(SHELL_HOST, """      if (!(message instanceof Uint8Array) || message.length === 0 || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || decodeBackboneMessage(message).kind !== "mutations") throw new Error("document-backbone.invalid-message");""",
     """      if (!(message instanceof Uint8Array) || message.length === 0 || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !["mutations", "sequenced", "committed"].includes(decodeBackboneMessage(message).kind)) throw new Error("document-backbone.invalid-message");""", "shell host admits store-bound messages")

edit(CODEC_LAWS, """    it("throws on an unknown backbone message tag", () => {
      expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("invalid format");
    });""", """    it("throws on an unknown backbone message tag", () => {
      expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("invalid format");
    });

    it("encodes the sequencer's messages exactly like the Rust store's OpBinary", () => {
      const hex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
      const vectors: readonly (readonly [BinaryBackboneMessage, string])[] = [
        [{ kind: "committed", opIds: [] }, "01040001000c00"],
        [{ kind: "committed", opIds: ["a"] }, "010401016101000c010600"],
        [{ kind: "sequenced", envelopes: Uint8Array.of(0xdd) }, "01050001000801dd"],
      ];
      for (const [message, expected] of vectors) {
        expect(hex(encodeBackboneMessage(message))).toBe(expected);
        expect(decodeBackboneMessage(encodeBackboneMessage(message))).toEqual(message);
      }
      for (const hostile of ["01060001000c00", "01050001000c00", "01030001000c00"]) expect(() => decodeBackboneMessage(Uint8Array.from(hostile.match(/../gu)!, (pair) => Number.parseInt(pair, 16)))).toThrow();
    });""", "codec law sequencer vectors")
edit(CODEC_LAWS, """      expect(() => parseDocumentBackboneMessage(genesis)).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(ack)).toThrow("mutations required");""", """      expect(() => parseDocumentBackboneMessage(genesis)).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(ack)).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(encodeBackboneMessage({ kind: "committed", opIds: [] }))).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(encodeBackboneMessage({ kind: "sequenced", envelopes: fromHex("0000") }))).toThrow("mutations required");
      const committed = encodeBackboneMessage({ kind: "committed", opIds: ["d:local-1"] });
      expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "send", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", message: { kind: "documentBackbone", message: committed } }))).toThrow("mutations required");
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ kind: "event", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", event: { kind: "documentBackbone", message: committed } }))).toEqual({ kind: "event", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", event: { kind: "documentBackbone", message: committed } });""", "codec law sequencer messages flow hub to store only")
