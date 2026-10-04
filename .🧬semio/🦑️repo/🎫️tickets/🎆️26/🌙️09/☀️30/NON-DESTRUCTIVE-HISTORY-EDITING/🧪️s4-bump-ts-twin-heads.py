#!/usr/bin/env python3
"""🧪️ S4-BUMP: TS channel twin (`💻️os/🟦️.ts`) for the CHANNEL_VERSION 21 scope addition — `ReadChildHeads` (command tag 42) →
`ChildHeads` (frame tag 32) with `AppChannelClient.readChildHeads()`, and the one-lane `PureCommand { seq, command, head }`.
Twin of the Rust codec in `📡️spr/🧵️channel/🦀️.rs`; every anchor must match exactly once.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🟦️.ts"

EDITS = [
    ("export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[] };\n",
     "export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[] };\n"
     "/** 🪆️ One owned child's CURRENT head snapshot pack — twin of the Rust `ChildHeadPackEntry` (`AppFrame::ChildHeads`). */\n"
     "export type ChildHeadPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly head_pack: readonly number[] };\n"),
    ("  | { readonly PureCommand: { readonly seq: number; readonly command: readonly number[]; readonly document: readonly number[]; readonly document_spr: readonly number[]; readonly config: readonly number[]; readonly config_spr: readonly number[]; readonly draft: readonly number[]; readonly draft_spr: readonly number[] } }\n",
     "  | { readonly PureCommand: { readonly seq: number; readonly command: readonly number[]; readonly head: readonly number[] } }\n"),
    ("  | { readonly ReadChildren: { readonly seq: number } }\n",
     "  | { readonly ReadChildren: { readonly seq: number } }\n  | { readonly ReadChildHeads: { readonly seq: number } }\n"),
    ("  | { readonly Children: { readonly in_reply_to: number; readonly entries: readonly ChildPackEntry[] } }\n",
     "  | { readonly Children: { readonly in_reply_to: number; readonly entries: readonly ChildPackEntry[] } }\n  | { readonly ChildHeads: { readonly in_reply_to: number; readonly entries: readonly ChildHeadPackEntry[] } }\n"),
    ("""function readVecChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
""",
     """function readVecChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
function writeVecChildHeadPackEntry(out: number[], entries: readonly ChildHeadPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) {
    writeStr(out, entry.slot);
    writeStr(out, entry.child_id);
    writeStr(out, entry.dialect);
    writeBytes(out, entry.head_pack);
  }
}
function readVecChildHeadPackEntry(bytes: Uint8Array, pos: [number]): ChildHeadPackEntry[] {
  const count = readVarintU64(bytes, pos);
  if (count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS) throw new Error("decodeAppFrame: child head count exceeds the member authority");
  return Array.from({ length: count }, () => ({ slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), head_pack: readBytes(bytes, pos) }));
}
"""),
    ("TakeMediaExportChunk: 40, ReadDocumentIdentity: 41,", "TakeMediaExportChunk: 40, ReadDocumentIdentity: 41, ReadChildHeads: 42,"),
    ("MediaExportChunk: 30, DocumentIdentity: 31,\n} as const;", "MediaExportChunk: 30, DocumentIdentity: 31, ChildHeads: 32,\n} as const;"),
    ("""    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.document);
    writeBytes(out, cmd.PureCommand.document_spr);
    writeBytes(out, cmd.PureCommand.config);
    writeBytes(out, cmd.PureCommand.config_spr);
    writeBytes(out, cmd.PureCommand.draft);
    writeBytes(out, cmd.PureCommand.draft_spr);
""",
     """    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.head);
"""),
    ("""  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
""",
     """  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
  } else if ("ReadChildHeads" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildHeads);
    writeVarintU64(out, cmd.ReadChildHeads.seq);
"""),
    ("      return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), document: readBytes(bytes, pos), document_spr: readBytes(bytes, pos), config: readBytes(bytes, pos), config_spr: readBytes(bytes, pos), draft: readBytes(bytes, pos), draft_spr: readBytes(bytes, pos) } };\n",
     "      return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), head: readBytes(bytes, pos) } };\n"),
    ("""    case APP_COMMAND_TAGS.ReadChildren:
      return { ReadChildren: { seq: readVarintU64(bytes, pos) } };
""",
     """    case APP_COMMAND_TAGS.ReadChildren:
      return { ReadChildren: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadChildHeads:
      return { ReadChildHeads: { seq: readVarintU64(bytes, pos) } };
"""),
    ("""    writeVecChildPackEntry(out, frame.Children.entries);
  } else if ("Ephemeral" in frame) {""",
     """    writeVecChildPackEntry(out, frame.Children.entries);
  } else if ("ChildHeads" in frame) {
    out.push(APP_FRAME_TAGS.ChildHeads);
    writeVarintU64(out, frame.ChildHeads.in_reply_to);
    writeVecChildHeadPackEntry(out, frame.ChildHeads.entries);
  } else if ("Ephemeral" in frame) {"""),
    ("""      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
""",
     """      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.ChildHeads:
      return { ChildHeads: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildHeadPackEntry(bytes, pos) } };
"""),
    ("""    if (identity.app_instance_id !== this.instanceId) throw new Error("document identity: foreign instance");
    return identity;
  }
""",
     """    if (identity.app_instance_id !== this.instanceId) throw new Error("document identity: foreign instance");
    return identity;
  }

  /** 🪆️ Reads every owned child's CURRENT head snapshot pack (`AppCommand::ReadChildHeads` → `AppFrame::ChildHeads`), for a
   * reader that composes parent + children on read (design §20.15). A guest fault rejects with the guest's own fault text. */
  async readChildHeads(): Promise<readonly ChildHeadPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadChildHeads: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const heads = frames.find((frame): frame is Extract<AppFrameValue, { readonly ChildHeads: unknown }> => "ChildHeads" in frame && frame.ChildHeads.in_reply_to === seq);
    if (!heads) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): missing ChildHeads frame for seq ${seq}`);
    return heads.ChildHeads.entries;
  }
"""),
]

text = TWIN.read_text()
for old, new in EDITS:
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor matched {found}x (expected 1): {old[:140]!r}")
    text = text.replace(old, new)
TWIN.write_text(text)
print(f"TS twin heads: {len(EDITS)} edits applied")
