#!/usr/bin/env python3
"""🧪️ S4-BUMP: the TypeScript channel twin (`💻️os/🟦️.ts`) for CHANNEL_VERSION 21 (design §20.7).

Deletes `LoadDocument` (tag 6 stays unassigned) with `AppChannelClient.loadDocument` and its document-cache candidate path,
`transactionPrepare.label`, and `transactionProposal.{description, coalesce_key}`; `loadDocumentArchive` snapshots the parent
pair at admission so a caller mutating its archive mid-load never reaches the cache. Every anchor must match exactly once.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🟦️.ts"

EDITS = [
    ("  | { readonly LoadDocument: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }\n", ""),
    ("        readonly prepared_ops: readonly (readonly number[])[];\n        readonly label: string;\n        readonly origin: readonly number[];\n",
     "        readonly prepared_ops: readonly (readonly number[])[];\n        readonly origin: readonly number[];\n"),
    ("        readonly local_ops: readonly (readonly number[])[];\n        readonly description: string;\n        readonly coalesce_key: string;\n        readonly foreign: readonly (readonly number[])[];\n",
     "        readonly local_ops: readonly (readonly number[])[];\n        readonly foreign: readonly (readonly number[])[];\n"),
    ("  LoadDocument: 6, ReadDocument: 7, LoadConfig: 8,", "  ReadDocument: 7, LoadConfig: 8,"),
    ('  } else if ("LoadDocument" in cmd) {\n    out.push(APP_COMMAND_TAGS.LoadDocument);\n    writeVarintU64(out, cmd.LoadDocument.seq);\n    writeBytes(out, cmd.LoadDocument.pack);\n    writeBytes(out, cmd.LoadDocument.spr);\n', ""),
    ("    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);\n    writeStr(out, cmd.transactionPrepare.label);\n",
     "    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);\n"),
    ("    case APP_COMMAND_TAGS.LoadDocument: {\n      const seq = readVarintU64(bytes, pos);\n      const pack = readBytes(bytes, pos);\n      const spr = readBytes(bytes, pos);\n      return { LoadDocument: { seq, pack, spr } };\n    }\n", ""),
    ("      const prepared_ops = readVecBytes(bytes, pos);\n      const label = readStr(bytes, pos);\n      const origin = readBytes(bytes, pos);\n      const prepared_child_ops = readBytes(bytes, pos);\n      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, label, origin, prepared_child_ops } };",
     "      const prepared_ops = readVecBytes(bytes, pos);\n      const origin = readBytes(bytes, pos);\n      const prepared_child_ops = readBytes(bytes, pos);\n      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } };"),
    ("    writeVecBytes(out, frame.transactionProposal.local_ops);\n    writeStr(out, frame.transactionProposal.description);\n    writeStr(out, frame.transactionProposal.coalesce_key);\n",
     "    writeVecBytes(out, frame.transactionProposal.local_ops);\n"),
    ("      const local_ops = readVecBytes(bytes, pos);\n      const description = readStr(bytes, pos);\n      const coalesce_key = readStr(bytes, pos);\n      const foreign = readVecBytes(bytes, pos);\n      return { transactionProposal: { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } };",
     "      const local_ops = readVecBytes(bytes, pos);\n      const foreign = readVecBytes(bytes, pos);\n      return { transactionProposal: { in_reply_to, proposal_id, local_ops, foreign } };"),
    (" * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/`LoadDocument`/\n * `ReadDocument`/`LoadConfig`/`ReadConfig` with",
     " * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/\n * `ReadDocument`/`LoadConfig`/`ReadConfig` with"),
    ("readonly transaction: AppChannelTransactionReply | null; readonly document: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null; readonly resolve:",
     "readonly transaction: AppChannelTransactionReply | null; readonly resolve:"),
    ("   * browser host keeps NO document pack per instance today\"). Populated from BOTH directions —\n   * {@link loadDocument}'s accepted arguments and every\n   * `AppFrame::Document` reply any sent command's outcome carries (`ReadDocument`, `LoadDocument`'s\n   * own echo, or any future command that happens to include one) — so a transaction coordinator can\n",
     "   * browser host keeps NO document pack per instance today\"). Populated from BOTH directions —\n   * {@link loadDocumentArchive}'s ready root pair and every\n   * `AppFrame::Document` reply any sent command's outcome carries (`ReadDocument`, or any future command that\n   * happens to include one) — so a transaction coordinator can\n"),
    ("        this.captureDocumentFrames(reply, waiter.document);\n", "        this.captureDocumentFrames(reply);\n"),
    ("  private captureDocumentFrames(frames: readonly AppFrameValue[], candidate: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null): void {\n    if (this.disposed || frames.some(frame => \"Error\" in frame)) return;\n    if (candidate && frames.some(frame => \"Done\" in frame)) {\n      this.cachedPack = candidate.pack;\n      this.cachedSpr = candidate.spr;\n    }\n",
     "  private captureDocumentFrames(frames: readonly AppFrameValue[]): void {\n    if (this.disposed || frames.some(frame => \"Error\" in frame)) return;\n"),
    ("   * accepted {@link loadDocument} call or `AppFrame::Document` reply has been observed.",
     "   * ready {@link loadDocumentArchive} or `AppFrame::Document` reply has been observed."),
    ("      const document = \"LoadDocument\" in command ? { pack: Uint8Array.from(command.LoadDocument.pack), spr: Uint8Array.from(command.LoadDocument.spr) } : null;\n      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), document, resolve, reject };",
     "      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject };"),
    ("this.pending.push({ seq, queryReceipt: true, transaction: null, document: null, resolve: () => {},", "this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {},"),
    ("  async loadDocument(pack: Uint8Array, spr: Uint8Array): Promise<AppFrameValue[]> {\n    return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });\n  }\n\n", ""),
    ("    const load = new DocumentArchiveLoadHost(archive);\n    for (;;) {",
     "    const load = new DocumentArchiveLoadHost(archive);\n    const root = { pack: Uint8Array.from(archive.parent_pack), spr: Uint8Array.from(archive.parent_spr) };\n    for (;;) {"),
    ("        this.cachedPack = Uint8Array.from(archive.parent_pack);\n        this.cachedSpr = Uint8Array.from(archive.parent_spr);\n        return;",
     "        this.cachedPack = root.pack;\n        this.cachedSpr = root.spr;\n        return;"),
    ("payload: Array.from(payload), prepared_ops: [], label: \"\", origin: [], prepared_child_ops: [] },", "payload: Array.from(payload), prepared_ops: [], origin: [], prepared_child_ops: [] },"),
    ("  /** 🎫️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`label`/`origin` set, `mutationId`", "  /** 🎫️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`origin` set, `mutationId`"),
    ("  async transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], label: string, origin: Uint8Array): Promise<AppFrameValue[]> {",
     "  async transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], origin: Uint8Array): Promise<AppFrameValue[]> {"),
    ("        prepared_ops: preparedOps.map((op) => Array.from(op)),\n        label,\n        origin: Array.from(origin),", "        prepared_ops: preparedOps.map((op) => Array.from(op)),\n        origin: Array.from(origin),"),
]

text = TWIN.read_text()
for old, new in EDITS:
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor matched {found}x (expected 1): {old[:140]!r}")
    text = text.replace(old, new)
TWIN.write_text(text)
print(f"TS twin: {len(EDITS)} edits applied")
