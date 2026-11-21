"""📡 Stages the closed multipart carrier beside the existing causal and document wire parsers."""

import hashlib
import json
from os.path import relpath
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
MODULES = Path("🧰️framework/🔨️modules")
BATCH = MODULES / "📡️replication/🚪️io/💾️binary/🔗️causal/🧮️backbone/🟦️.ts"
ASSEMBLY = BATCH.parent / "🧩️atomic-group/🟦️.ts"
MESSAGE = MODULES / "🛰️backbone/🔀️message/🟦️.ts"
ACTOR = MODULES / "🛰️backbone/🎭️artifact-actor/🟦️.ts"
BACKBONE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/🦀️.rs")
NATIVE = BACKBONE.with_name("🧩️atomic-transport.rs")
STORE_DRIVER = BACKBONE.parent.parent / "⚙️engine/📥️lifted/🏪️store/📑️delegating-operations-and-publication/🦀️.rs"
STORE_CURSOR = BACKBONE.parent.parent / "⚙️engine/🧱️store-components/📥️lifted/🏪️store/🖱️store-cursor-operations/🦀️.rs"
WORKER = BACKBONE.parent.parent / "👷️worker"
RELAY = WORKER / "🧩️atomic-document-relay/🟦️.ts"
LIFECYCLE = WORKER / "♻️artifact-lifecycle/🟦️.ts"
OPEN = WORKER / "🚪️artifact-open/🟦️.ts"
TEARDOWN = WORKER / "🛑️session-teardown/🟦️.ts"
TURNS = WORKER / "🧷️actor-reservation/📤️turn-effects/🟦️.ts"
INBOUND = WORKER / "🧷️actor-reservation/📬️document-backbone/🟦️.ts"
OUTPUT = TICKET / "🗑️generated/raw-spr-transport-overlay.json"


def stage():
    preimages = {}
    candidate = {}
    native_live = (ROOT / NATIVE).exists()

    def edit(path, old, new, count=1):
        if native_live and path.suffix == ".rs":
            return
        raw = (ROOT / path).read_bytes()
        preimages[str(path)] = hashlib.sha256(raw).hexdigest()
        source = candidate.get(str(path), raw.decode("utf8"))
        assert source.count(old) == count, (path, old)
        candidate[str(path)] = source.replace(old, new)

    assert not (ROOT / ASSEMBLY).exists()
    preimages[str(ASSEMBLY)] = None
    candidate[str(ASSEMBLY)] = (TICKET / "raw-spr-group-transport.ts").read_text(encoding="utf8")
    edit(BATCH, "const MUTATION_DAG_CAPACITY",
         'import { ATOMIC_BACKBONE_MAXIMUM_BYTES } from "./🧩️atomic-group/🟦️.ts";\n\nconst MUTATION_DAG_CAPACITY')
    edit(BATCH, "normalizedDocumentBackboneBatchLimits(limits: DocumentBackboneBatchLimits)",
         "normalizedDocumentBackboneBatchLimits(limits: DocumentBackboneBatchLimits, ceiling: DocumentBackboneBatchLimits)")
    for field in ("maximumBytes", "maximumPayloadBytes"):
        edit(BATCH, f'limits.{field}, DOCUMENT_BACKBONE_BATCH_LIMITS.{field}',
             f'limits.{field}, ceiling.{field}')
    edit(BATCH, "  terminal: boolean,",
         "  terminal: boolean,\n  ceiling: DocumentBackboneBatchLimits = DOCUMENT_BACKBONE_BATCH_LIMITS,")
    edit(BATCH, "normalizedDocumentBackboneBatchLimits(suppliedLimits);",
         "normalizedDocumentBackboneBatchLimits(suppliedLimits, ceiling);")
    edit(BATCH, '    const diffPayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");',
         '    const rowBytes = Math.min(DOCUMENT_BACKBONE_BATCH_LIMITS.maximumPayloadBytes, limits.maximumPayloadBytes - totalPayloadBytes);\n'
         '    const diffPayload = readBytes(rowBytes, "payload-bytes");')
    edit(BATCH, '    const inversePayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");',
         '    const inversePayload = readBytes(rowBytes - diffPayload.length, "payload-bytes");')
    edit(BATCH, "/** 🧭 Reads a bounded exact causal batch",
         """const ATOMIC_DOCUMENT_LIMITS = {...DOCUMENT_BACKBONE_BATCH_LIMITS,
  maximumBytes: ATOMIC_BACKBONE_MAXIMUM_BYTES, maximumPayloadBytes: ATOMIC_BACKBONE_MAXIMUM_BYTES };

/** 🔐️ Decodes the sealed atomic carrier's canonical causal rows within the unchanged per-row limit. */
export function decodeAtomicDocumentBackboneBatch(bytes: Uint8Array): readonly ExactWireMutationEnvelope[] {
  return readDocumentBackboneEnvelopeBatchAtExact(bytes, 0, ATOMIC_DOCUMENT_LIMITS, true, ATOMIC_DOCUMENT_LIMITS).envelopes;
}

/** 🧭 Reads a bounded exact causal batch""")
    assembly_relative = "../../📡️replication/🚪️io/💾️binary/🔗️causal/🧮️backbone/🧩️atomic-group/🟦️.ts"
    edit(MESSAGE, "//#region 🔖️Backbone",
         f'import {{ atomicBackbonePart }} from "{assembly_relative}";\n\n//#region 🔖️Backbone')
    edit(MESSAGE, "/** 🔙️ Admits one canonical hot message",
         """export type DocumentBackboneTransportMessage =
  | (DocumentBackboneMessage & Readonly<{ kind: "batch" }>)
  | Readonly<{ kind: "atomic-part"; message: Uint8Array; part: Uint8Array }>;

/** 🧵️ Checks the original outer envelope and selects atomic custody before ordinary batch decoding. */
export function parseDocumentBackboneTransportMessage(message: Uint8Array): DocumentBackboneTransportMessage {
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind !== "mutations") throw new Error("document backbone: mutations required");
  if (atomicBackbonePart(parsed.envelopes) !== undefined) {
    return { kind: "atomic-part", message: message.slice(), part: parsed.envelopes };
  }
  return { kind: "batch", message: message.slice(), envelopes: decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes) };
}

/** 🔙️ Admits one canonical hot message""")
    edit(MESSAGE, "return parseDocumentBackboneMessage(message).message;",
         "return parseDocumentBackboneTransportMessage(message).message;")
    edit(ACTOR, "parseDocumentBackboneMessage", "parseDocumentBackboneTransportMessage", 3)
    edit(BACKBONE, '#[path="📤️original-outbox/🦀️.rs"]\npub mod original_outbox;',
         '#[path="📤️original-outbox/🦀️.rs"]\npub mod original_outbox;\n#[path="🧩️atomic-transport.rs"]\npub mod atomic_transport;')
    edit(BACKBONE, """                state.inbound_bytes = 0;
                Ok(state.inbound.drain(..).collect())""",
         """                let ordinary = state.inbound.iter().position(|frame| atomic_transport::atomic_wire_front(frame)).unwrap_or(state.inbound.len());
                let original: Vec<_> = state.inbound.drain(..ordinary).collect();
                state.inbound_bytes -= original.iter().map(Vec::len).sum::<usize>();
                Ok(original)""")
    edit(BACKBONE, """let mut inbound = self.inbound.as_ref().ok_or_else(|| VcsError::Backbone("channel backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())""",
         """let mut inbound = self.inbound.as_ref().ok_or_else(|| VcsError::Backbone("channel backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        let ordinary = inbound.iter().position(atomic_transport::atomic_message_front).unwrap_or(inbound.len());
        Ok(inbound.drain(..ordinary).collect())""")
    if not native_live:
        preimages[str(NATIVE)] = None
        candidate[str(NATIVE)] = (TICKET / "raw-store-atomic-backbone.rs").read_text(encoding="utf8")
    driver = (TICKET / "raw-store-atomic-delivery-methods.rs").read_text(encoding="utf8")
    after = "    /// 🛑️ Returns private original causal suffixes"
    edit(STORE_DRIVER, after, driver + after)
    wrapper = (TICKET / "raw-store-atomic-transaction-store.rs").read_text(encoding="utf8")
    edit(STORE_DRIVER, after, wrapper + after)
    edit(STORE_DRIVER, "        match self.wire.backbone.as_ref(){Some(Backbones::Channel(channel))=>channel.send_original",
         """        if self.wire.original_outbox.front().is_some_and(semio_os_spr::atomic_group::AtomicBackboneSender::matches_prepared) {
            return Ok(None);
        }
        match self.wire.backbone.as_ref(){Some(Backbones::Channel(channel))=>channel.send_original""")
    cursor = (TICKET / "raw-store-atomic-transaction-cursor.rs").read_text(encoding="utf8")
    edit(STORE_CURSOR, "    pub fn admit_apply_batch_with_pages(\n", cursor + "    fn admit_apply_batch_for_purpose(\n")
    edit(STORE_CURSOR, "group:Option<&str>, outbound:bool, grant:RetainedCloneGrant,\n",
         "group:Option<&str>, outbound:bool, media_app:Option<&str>, grant:RetainedCloneGrant,\n")
    edit(STORE_CURSOR, "        let admitted_items = mutations.len() + pages.as_ref()",
         """        if let Some(app) = media_app {
            use semio_os_spr::atomic_group::ATOMIC_BACKBONE_TOOL;
            let tool_length = app.len().saturating_add(1 + ATOMIC_BACKBONE_TOOL.len());
            if app.is_empty() || app.contains('#') || tool_length > ARTIFACT_STORE_ONE_ITEM_ID_BYTES ||
                transaction.is_some() || pages.as_ref().is_none_or(|input| input.len() == 0) ||
                original_actor.as_str() != store_log.local_actor_id().0.as_str() ||
                !store_log.operation_wire.as_ref().is_some_and(|wire| wire.original_message_admission.is_some()) {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original media requires its registered document actor, pages and causal issuer"));
            }
        }
        let admitted_items = mutations.len() + pages.as_ref()""")
    edit(STORE_CURSOR, "        let text_bytes=stamped.map_or(0,str::len)",
         """        use protocol::mutation::transaction_admission::{admit_tool_transaction, tool_transaction_demand};
        use semio_os_spr::atomic_group::ATOMIC_BACKBONE_TOOL;
        let media_demand = media_app.map(|app| tool_transaction_demand(original_actor.as_str(), app, ATOMIC_BACKBONE_TOOL))
            .transpose()?.unwrap_or_default();
        let text_bytes=stamped.map_or(0,str::len)""")
    edit(STORE_CURSOR, "size_of_val(actor)+text_bytes;",
         "size_of_val(actor)+text_bytes+media_demand.copy_bytes;")
    edit(STORE_CURSOR, "shared_retirement_allocation_bytes::<ArtifactStoreOneItemLiveAuthority>()+text_bytes;",
         "shared_retirement_allocation_bytes::<ArtifactStoreOneItemLiveAuthority>()+text_bytes+media_demand.capacity_bytes;")
    edit(STORE_CURSOR, "        let stamped_edit_id=stamped.map(text);",
         """        let minted = if let Some(app) = media_app {
            let (transaction, _) = admit_tool_transaction(original_actor.as_str(), &next_clock, app, ATOMIC_BACKBONE_TOOL, grant)?
                .expect("funded registered original media transaction");
            Some(transaction)
        } else { None };
        let stamped_edit_id=stamped.map(text);""")
    edit(STORE_CURSOR, "transaction:std::mem::ManuallyDrop::new(transaction.take())",
         "transaction:std::mem::ManuallyDrop::new(minted.or_else(||transaction.take()))")
    assert not (ROOT / RELAY).exists()
    preimages[str(RELAY)] = None
    relay = (TICKET / "raw-local-atomic-relay.ts").read_text(encoding="utf8")
    for marker, quoted in (("ATOMIC_MODULE", ASSEMBLY), ("CAUSAL_MODULE", BATCH), ("BACKBONE_MODULE", MESSAGE)):
        relay = relay.replace(marker, relpath(quoted, RELAY.parent))
    candidate[str(RELAY)] = relay
    for path in (LIFECYCLE, OPEN, TURNS, INBOUND):
        edit(path, "parseDocumentBackboneMessage", "parseDocumentBackboneTransportMessage", 2)
        edit(path, "parseDocumentBackboneTransportMessage, ", "") if path in (LIFECYCLE, INBOUND) else None
        if path in (LIFECYCLE, INBOUND):
            anchor = candidate[str(path)].splitlines()[0]
        else:
            anchor = next(line for line in candidate[str(path)].splitlines() if "import " in line and "parseDocumentBackboneTransportMessage" in line)
            edit(path, anchor, anchor.replace(", parseDocumentBackboneTransportMessage", "") if path == TURNS
                 else anchor.replace("import { parseDocumentBackboneTransportMessage } from \"../../../../🟦️\";", ""))
            anchor = candidate[str(path)].splitlines()[0]
        edit(path, anchor, f'import {{ parseDocumentBackboneTransportMessage }} from "{relpath(MESSAGE, path.parent)}";\n' + anchor)
    edit(LIFECYCLE, "export async function handleLocalMsg", 
         'import { localAtomicDocumentFrames } from "../🧩️atomic-document-relay/🟦️.ts";\n\nexport async function handleLocalMsg')
    edit(LIFECYCLE, "      if (parsed.envelopes.some", """      if (parsed.kind === "atomic-part") {
        if (hubBinding(state.config) !== null) throw new Error("atomic backbone: Hub binding does not admit grouped history");
        if (state.executionTargetLease?.live && !state.executionTargetLease.fields().grant.write) {
          rejectLocally(state, localCommandRejectionV1("local.backbone-scope-mismatch", "document backbone write authority is unavailable"));
          break;
        }
        const sealed = await localAtomicDocumentFrames(state, parsed, "outgoing");
        if (!sealed) break;
        const rows = sealed.envelopes.map(opaqueEnvelopeFromWire);
        rows.forEach((row, index) => state.exactLocalEnvelopes.set(row, {
          envelope: sealed.envelopes[index]!, bytes: index === 0 ? sealed.bytes : 0, messages: index === 0 ? sealed.messages : 0,
        }));
        noteAuthoredEnvelopeIds(state.ingestedMutationIds, sealed.envelopes.map(row => row.mutation_id));
        state.pendingMutations.push(...rows);
        setStatus(state, { pendingMutations: state.pendingMutations.length });
        for (const physical of sealed.frames) state.channel.postMessage({ kind: "documentBackbone", message: physical });
        break;
      }
      if (parsed.envelopes.some""")
    edit(OPEN, "  channel.onmessage = (messageEvent) => {",
         """  channel.onmessage = async (messageEvent) => {""")
    edit(OPEN, "      if (parsed.envelopes.length > 0",
         """      if (parsed.kind === "atomic-part") {
        if (hub !== null) throw new Error("atomic backbone: Hub binding does not admit grouped history");
        const sealed = await localAtomicDocumentFrames(state, parsed, "incoming");
        for (const message of sealed?.frames ?? []) {
          emitEvent(state, { kind: "documentBackbone", message });
        }
        return;
      }
      if (parsed.envelopes.length > 0""")
    first = candidate[str(OPEN)].splitlines()[0]
    edit(OPEN, first, 'import { localAtomicDocumentFrames } from "../🧩️atomic-document-relay/🟦️.ts";\n' + first)
    edit(TURNS, "messageMutations = parsed?.envelopes.length ?? 0;",
         'messageMutations = parsed?.kind === "batch" ? parsed.envelopes.length : 0;')
    edit(TURNS, 'mode.kind === "app-command" && parsed !== null) commandEnvelopes.push',
         'mode.kind === "app-command" && parsed?.kind === "batch") commandEnvelopes.push')
    edit(INBOUND, "    if (parseDocumentBackboneTransportMessage(message).envelopes.some",
         '    const parsed = parseDocumentBackboneTransportMessage(message);\n'
         '    if (parsed.kind === "batch" && parsed.envelopes.some')
    first = (ROOT / TEARDOWN).read_text(encoding="utf8").splitlines()[0]
    edit(TEARDOWN, first, 'import { retireLocalAtomicDocumentFrames } from "../🧩️atomic-document-relay/🟦️.ts";\n' + first)
    edit(TEARDOWN, "  state.closed = true;", "  state.closed = true;\n  retireLocalAtomicDocumentFrames(state);")
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps({"preimages": preimages, "candidate": candidate}, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(candidate)} document transport codec postimages")


if __name__ == "__main__":
    stage()
