#!/usr/bin/env python3
"""🧪️ S4-BUMP: CHANNEL_VERSION 21 coordinator scope addition in the Rust channel codec (`📡️spr/🧵️channel/🦀️.rs`).

Adds `AppCommand::ReadChildHeads { seq }` (tag 42) → `AppFrame::ChildHeads { in_reply_to, entries }` (tag 32) over the existing
`ChildHeadPackEntry`, and narrows `AppCommand::PureCommand` to `{ seq, command, head }` (design §20.8: a pure command runs on a
HEAD snapshot pack, empty = the live document). Every anchor must match exactly as often as stated.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CHANNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs"

EDITS = [
    ("    ReadDocumentArchive { seq: u64 },\n    ReadDocumentIdentity { seq: u64 },\n    LoadDocumentArchive { seq: u64, decode: PagedDocumentArchiveDecode },\n",
     "    ReadDocumentArchive { seq: u64 },\n    ReadDocumentIdentity { seq: u64 },\n    ReadChildHeads { seq: u64 },\n    LoadDocumentArchive { seq: u64, decode: PagedDocumentArchiveDecode },\n", 1),
    ("                    41 => PagedAppCommandDecodeState::ReadDocumentIdentity { seq },\n",
     "                    41 => PagedAppCommandDecodeState::ReadDocumentIdentity { seq },\n                    42 => PagedAppCommandDecodeState::ReadChildHeads { seq },\n", 1),
    ("            PagedAppCommandDecodeState::ReadDocumentIdentity { seq } => Some(AppCommand::ReadDocumentIdentity { seq }),\n",
     "            PagedAppCommandDecodeState::ReadDocumentIdentity { seq } => Some(AppCommand::ReadDocumentIdentity { seq }),\n            PagedAppCommandDecodeState::ReadChildHeads { seq } => Some(AppCommand::ReadChildHeads { seq }),\n", 1),
    ("                Some(AppCommand::PureCommand { seq, command: next(), document: next(), document_spr: next(), config: next(), config_spr: next(), draft: next(), draft_spr: next() })",
     "                Some(AppCommand::PureCommand { seq, command: next(), head: next() })", 1),
    ("                    | AppCommand::ReadDocumentIdentity { .. }\n                    | AppCommand::ReadWindowConfigs { .. }\n",
     "                    | AppCommand::ReadDocumentIdentity { .. }\n                    | AppCommand::ReadChildHeads { .. }\n                    | AppCommand::ReadWindowConfigs { .. }\n", 1),
    ("                    AppCommand::LoadConfig { pack, spr, .. } => vec![pack, spr],\n                    AppCommand::LoadWindowConfig { entry, .. } => vec![entry.window_id.into_bytes(), entry.window_kind_id.into_bytes(), entry.envelope_pack],\n",
     "                    AppCommand::LoadConfig { pack, spr, .. } => vec![pack, spr],\n                    AppCommand::PureCommand { command, head, .. } => vec![command, head],\n                    AppCommand::LoadWindowConfig { entry, .. } => vec![entry.window_id.into_bytes(), entry.window_kind_id.into_bytes(), entry.envelope_pack],\n", 1),
    ("""    /// 🧾 Host-authoritative command: document/config/draft packs travel with the command; guest
    /// returns `AppFrame::Emit` ops only (host applies). CHANNEL_VERSION 5 wire addition.
    PureCommand {
        seq: u64,
        command: Vec<u8>,
        document: Vec<u8>,
        document_spr: Vec<u8>,
        config: Vec<u8>,
        config_spr: Vec<u8>,
        draft: Vec<u8>,
        draft_spr: Vec<u8>,
    },
""",
     """    /// 🧾 Host-authoritative command: the guest evaluates `command` and returns `AppFrame::Emit` ops only (host
    /// applies). CHANNEL_VERSION 5 wire addition; CHANNEL_VERSION 21 reduced its lanes to one `head` (design §20.8).
    PureCommand {
        seq: u64,
        command: Vec<u8>,
        /// 📸️ The HEAD snapshot pack a stateless evaluation runs against with an empty history — never a fold of `.spr`
        /// history; empty evaluates against the instance's live document.
        head: Vec<u8>,
    },
""", 1),
    ("    ReadDocumentIdentity { seq: u64 },\n}\n//#endregion 🔖️AppCommand\n",
     "    ReadDocumentIdentity { seq: u64 },\n    /// 🪆️ Reads every owned child's CURRENT head snapshot pack (nested members included) for a reader that composes\n    /// parent + children on read (design §20.15). Reply `AppFrame::ChildHeads`. CHANNEL_VERSION 21 wire addition.\n    ReadChildHeads { seq: u64 },\n}\n//#endregion 🔖️AppCommand\n", 1),
    ("    DocumentIdentity { in_reply_to: u64, identity: AppDocumentIdentity },\n}\n//#endregion 🔖️AppFrame\n",
     "    DocumentIdentity { in_reply_to: u64, identity: AppDocumentIdentity },\n    /// 🪆️ Reply to `AppCommand::ReadChildHeads`: one entry per owned child. CHANNEL_VERSION 21 wire addition.\n    ChildHeads { in_reply_to: u64, entries: Vec<ChildHeadPackEntry> },\n}\n//#endregion 🔖️AppFrame\n", 1),
    ("""        AppCommand::PureCommand { seq, command, document, document_spr, config, config_spr, draft, draft_spr } => {
            out.byte(13)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(document)?;
            out.bytes(document_spr)?;
            out.bytes(config)?;
            out.bytes(config_spr)?;
            out.bytes(draft)?;
            out.bytes(draft_spr)?;
        }
""",
     """        AppCommand::PureCommand { seq, command, head } => {
            out.byte(13)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(head)?;
        }
""", 1),
    ("""        AppCommand::ReadDocumentIdentity { seq } => {
            out.byte(41)?;
            out.varint(*seq)?;
        }
""",
     """        AppCommand::ReadDocumentIdentity { seq } => {
            out.byte(41)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadChildHeads { seq } => {
            out.byte(42)?;
            out.varint(*seq)?;
        }
""", 1),
    ("""/// 🧸️ Inverse of [`write_vec_child_pack`].""",
     """/// 🪆️ `count varint | (slot, child_id, dialect, head_pack)*` — `AppFrame::ChildHeads`' list codec.
fn write_vec_child_head(out: &mut Vec<u8>, entries: &[ChildHeadPackEntry]) {
    crate::os_spr::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        crate::os_spr::write_str(out, &entry.slot);
        crate::os_spr::write_str(out, &entry.child_id);
        crate::os_spr::write_str(out, &entry.dialect);
        crate::os_spr::write_bytes(out, &entry.head_pack);
    }
}

/// 🪆️ Inverse of [`write_vec_child_head`]; the declared count never reserves past the archive's member authority.
fn read_vec_child_head(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildHeadPackEntry>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 {
        return Err(malformed("channel child heads", *pos as u64, "child head count exceeds the member authority"));
    }
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildHeadPackEntry { slot: crate::os_spr::read_str(bytes, pos)?, child_id: crate::os_spr::read_str(bytes, pos)?, dialect: crate::os_spr::read_str(bytes, pos)?, head_pack: crate::os_spr::read_bytes(bytes, pos)? });
    }
    Ok(entries)
}

/// 🧸️ Inverse of [`write_vec_child_pack`].""", 1),
    ("""        13 => AppCommand::PureCommand {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            command: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
""",
     """        13 => AppCommand::PureCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)?, head: crate::os_spr::read_bytes(bytes, &mut pos)? },
""", 1),
    ("        41 => AppCommand::ReadDocumentIdentity { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },\n",
     "        41 => AppCommand::ReadDocumentIdentity { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },\n        42 => AppCommand::ReadChildHeads { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },\n", 1),
    ("""            if let Some(id) = &identity.parent_document_id {
                crate::os_spr::write_str(&mut out, id);
            }
        }
    }
    out
}
""",
     """            if let Some(id) = &identity.parent_document_id {
                crate::os_spr::write_str(&mut out, id);
            }
        }
        AppFrame::ChildHeads { in_reply_to, entries } => {
            out.push(32);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_head(&mut out, entries);
        }
    }
    out
}
""", 1),
    ("""            AppFrame::DocumentIdentity { in_reply_to, identity: AppDocumentIdentity { app_instance_id, parent_document_id } }
        }
""",
     """            AppFrame::DocumentIdentity { in_reply_to, identity: AppDocumentIdentity { app_instance_id, parent_document_id } }
        }
        32 => AppFrame::ChildHeads { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_head(bytes, &mut pos)? },
""", 1),
]

text = CHANNEL.read_text()
for old, new, count in EDITS:
    found = text.count(old)
    if found != count:
        sys.exit(f"anchor matched {found}x (expected {count}): {old[:140]!r}")
    text = text.replace(old, new)
CHANNEL.write_text(text)
print(f"channel heads: {len(EDITS)} edits applied")
