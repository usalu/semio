#!/usr/bin/env python3
"""🧹️ S4-BUMP wave B driver (design §20.7): the store description wave across every Rust crate except the hot files.

Removes `Edit.description` and every carrier of it — `ArtifactCommand::Apply`/`ApplyInLane.description`, `GroupMeta.description`,
the `.ops` header/command fields, `HistoryEdit.description`, the preparation request/batch plumbing, the `begin_*apply_batch`,
`apply_one`, `apply_command`, `dispatch_apply_exact`, `build_apply_command_bytes`, `dispatch_emit_group` arguments, and the
`preflight(…, description, lane)` parameter with each domain's description envelope check and retained description owner.
Store/spr/canonical-edit get exact anchored edits (each asserted to match exactly as often as expected). Hot files
(`🔌️plugin/🦀️.rs`, `🎮️mutation/🦀️.rs`) are reported, never written — they are edited with the Edit tool.
Usage: python3 🧪️s4-bump-waveb.py [--list <file>] [--preview]            (dry run prints every removal)
       python3 🧪️s4-bump-waveb.py --apply --files-from <file>           (explicit list from `--list`; the computed write set must
                                                                         equal it, originals are kept under s5-channel/waveb-backup)
       python3 🧪️s4-bump-waveb.py --restore                             (puts back every file that still holds the applied bytes)
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("rust_fields", TICKET / "🧪️s4-bump-rust-fields.py")
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

OS = "🧰️framework/🛍️products/💻️os/🔨️modules"
STORE = f"{OS}/🏪️store/🦀️.rs"
HISTORY = f"{OS}/📡️spr/📜️history/🦀️.rs"
CANONICAL = f"{OS}/🏪️store/🧵️canonical-edit/🦀️.rs"
CLI = f"{OS}/📡️spr/⌨️cli/🦀️.rs"
HOT = {f"{OS}/🔌️plugin/🦀️.rs", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs", "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"}

TYPES = {
    "ArtifactCommand::Apply", "ArtifactCommand::ApplyInLane", "GroupMeta", "OpsHeaderLine::Edit", "CommandHeaderLine::Apply",
    "CommandHeaderLine::ApplyInLane", "PendingEdit", "HistoryEdit", "ArtifactStoreBatchAdmissionRejected",
    "ArtifactStoreOneItemPreparationRequest", "ArtifactStoreBatchSourceOf", "MemberStoreOneItemWireRequest",
}
CALLS = {
    "begin_apply_batch": (5, 9), "begin_outbound_apply_batch": (5, 8), "begin_typed_apply_batch": (5, 10), "begin_member_apply_batch": (5, 6),
    "apply_command": (1, 4), "dispatch_apply_exact": (1, 2), "apply_one": (2, 4), "build_apply_command_bytes": (1, 3), ".preflight": (1, 3),
    "dispatch_emit_group": (3, 10), "publish_demo_batch": (3, 4), "publish_demo_batch_with": (3, 5), "item_retained_bytes": (1, 2),
    "DurableOwnedMapMemberAdmissionV1::new": (5, 6), "sample_edit": (2, 3),
}
DEFINITIONS = {"sample_edit", "publish_demo_batch", "publish_demo_batch_with", "item_retained_bytes", "begin_apply_batch", "begin_outbound_apply_batch", "begin_typed_apply_batch", "begin_member_apply_batch", "apply_command", "dispatch_apply_exact", "apply_one", "build_apply_command_bytes", "preflight"}

EXACT: dict[str, list[tuple[str, str, int]]] = {
    STORE: [
        ('strings: [Some(id), actor, description, verb, Some(started_at), finished_at, line]', 'strings: [Some(id), actor, verb, Some(started_at), finished_at, line]', 1),
        ('struct ArtifactStoreEditRetirementState {\n    strings: [Option<String>; 7],', 'struct ArtifactStoreEditRetirementState {\n    strings: [Option<String>; 6],', 1),
        ('        #[value(skip_serializing_if = "Option::is_none")]\n        description: Option<String>,\n        #[value(default, skip_serializing_if = "Option::is_none")]\n        transaction: Option<protocol::TransactionRef>,',
         '        #[value(default, skip_serializing_if = "Option::is_none")]\n        transaction: Option<protocol::TransactionRef>,', 1),
        ('        #[value(skip_serializing_if = "Option::is_none")]\n        description: Option<String>,\n        #[value(default)]\n        lane: HistoryLane,',
         '        #[value(default)]\n        lane: HistoryLane,', 1),
        ('    OwnedSchemaFieldSpec { id: 6, key: "description", required: false },\n', '', 1),
        ('matches!(field_id, 2 | 6 | 10 | 11 | 12)', 'matches!(field_id, 2 | 10 | 11 | 12)', 1),
        ('            1 => Some(0),\n            2 => Some(1),\n            6 => Some(2),\n            9 => Some(3),\n            10 => Some(4),\n            11 => Some(5),\n            12 => Some(6),\n',
         '            1 => Some(0),\n            2 => Some(1),\n            9 => Some(2),\n            10 => Some(3),\n            11 => Some(4),\n            12 => Some(5),\n', 1),
        ('    strings: [std::mem::ManuallyDrop<Option<String>>; 7],\n    forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,', '    strings: [std::mem::ManuallyDrop<Option<String>>; 6],\n    forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,', 1),
        ('        let started_at = self.strings[3].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-started-at-missing", 0))?;', '        let started_at = self.strings[2].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-started-at-missing", 0))?;', 1),
        ('            actor: self.strings[1].take(),\n            line: self.strings[6].take(),\n            forwards,\n            inverse,\n            mutation_meta: Vec::new(),\n            description: self.strings[2].take(),\n            verb: self.strings[5].take(),\n            sequence_number,\n            started_at,\n            finished_at: self.strings[4].take(),',
         '            actor: self.strings[1].take(),\n            line: self.strings[5].take(),\n            forwards,\n            inverse,\n            mutation_meta: Vec::new(),\n            verb: self.strings[4].take(),\n            sequence_number,\n            started_at,\n            finished_at: self.strings[3].take(),', 1),
        ('        actor: Option<String>,\n        finished: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Vec<String>,', '        actor: Option<String>,\n        finished: Option<String>,\n        verb: Option<String>,\n        line: Vec<String>,', 1),
        ('        finished_at: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }', '        finished_at: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }', 1),
        ('    Apply {\n        description: Option<String>,\n        transaction: Option<String>,', '    Apply {\n        transaction: Option<String>,', 1),
        ('    ApplyInLane {\n        description: Option<String>,\n        lane: String,', '    ApplyInLane {\n        lane: String,', 1),
        ('                out.push(u8::from(description.is_some()) | (u8::from(transaction.is_some()) << 1));\n                if let Some(text) = description {\n                    write_command_str(&mut out, text);\n                }\n',
         '                out.push(u8::from(transaction.is_some()) << 1);\n', 2),
        ('                let presence = reader.read_u8()?;\n                let description = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };\n                let transaction = read_command_transaction(reader, presence)?;',
         '                let presence = reader.read_u8()?;\n                let transaction = read_command_transaction(reader, presence)?;', 2),
        ('fn read_command_transaction(reader: &mut crate::os_pack::ByteReader<\'_>, presence: u8) -> Result<Option<protocol::TransactionRef>, crate::os_spr::ProtocolError> {\n    if presence & 0b10 == 0 {',
         'fn read_command_transaction(reader: &mut crate::os_pack::ByteReader<\'_>, presence: u8) -> Result<Option<protocol::TransactionRef>, crate::os_spr::ProtocolError> {\n    if presence & !0b10 != 0 {\n        return Err(crate::os_spr::ProtocolError::Malformed { what: "command presence", offset: 0, detail: format!("presence bits {presence:#04b} are unassigned beyond the 0b10 transaction bit") });\n    }\n    if presence & 0b10 == 0 {', 1),
        ('/// 🧾️ Binary twin of [`transaction_text`]: the id and tool strings after a `0b10` presence bit.', '/// 🧾️ Binary twin of [`transaction_text`]: the id and tool strings after a `0b10` presence bit; every other presence bit\n/// is unassigned and refused.', 1),
        ('                &tag(edit.description.as_ref()),\n                edit.description.as_deref().unwrap_or_default().as_bytes(),\n', '', 1),
        ('    pub description: Option<String>,\n    pub base: SnapshotRead<P>,\n    pub mutation: Mutation,\n}', '    pub base: SnapshotRead<P>,\n    pub mutation: Mutation,\n}', 1),
        ('    pub fn into_owners(self) -> (SnapshotRead<P>, Mutation, Option<String>) {\n        (self.base, self.mutation, self.description)\n    }', '    pub fn into_owners(self) -> (SnapshotRead<P>, Mutation) {\n        (self.base, self.mutation)\n    }', 1),
        ('    inputs: Vec<A::Input>,\n    description: Option<String>,\n    marker: PhantomData<fn() -> (P, Mutation)>,', '    inputs: Vec<A::Input>,\n    marker: PhantomData<fn() -> (P, Mutation)>,', 1),
        ('        let description = self.description.as_deref();\n        let mut total', '        let mut total', 1),
        ('            let item = authority.preflight(input, description, lane)?;', '            let item = authority.preflight(input, lane)?;', 1),
        ('                let (base, mutation, description) = item.into_owners();\n                let _ = base.return_to_registry();\n                self.inputs.push(mutation);\n                self.description = description;\n',
         '                let (base, mutation) = item.into_owners();\n                let _ = base.return_to_registry();\n                self.inputs.push(mutation);\n', 1),
        ('        if let Some(description) = self.description.take() {\n            return SnapshotRetirementStep::Pending { released_items: 0, released_bytes: description.len() };\n        }\n', '', 1),
        ('        self.inputs.is_empty() && self.description.is_none() && self.authority.is_none()', '        self.inputs.is_empty() && self.authority.is_none()', 1),
        ('    pub wire: MemberStoreOneItemWire,\n    pub description: Option<String>,\n}', '    pub wire: MemberStoreOneItemWire,\n}', 1),
        ('    pub mutations: Vec<Mutation>,\n    pub description: Option<String>,\n}', '    pub mutations: Vec<Mutation>,\n}', 1),
        ('    pub fn into_owners(self) -> (String, Vec<Mutation>, Option<String>) {\n        (self.reason, self.mutations, self.description)\n    }', '    pub fn into_owners(self) -> (String, Vec<Mutation>) {\n        (self.reason, self.mutations)\n    }', 1),
        ('&mut self.edit.line, &mut self.edit.description, &mut self.edit.verb', '&mut self.edit.line, &mut self.edit.verb', 1),
        ('            && self.edit.line.is_none()\n            && self.edit.description.is_none()\n', '            && self.edit.line.is_none()\n', 1),
        ('            let (reason, mut mutations, description) = rejected.into_owners();\n            mutations.reverse();\n            ArtifactStoreBatchAdmissionRejected { reason, mutations, description }',
         '            let (reason, mut mutations) = rejected.into_owners();\n            mutations.reverse();\n            ArtifactStoreBatchAdmissionRejected { reason, mutations }', 1),
        ('            stage.edit.description = edit.description.take();\n', '', 1),
        ('/// 🎛️ Cross-member metadata for one `dispatch_group` call. `description` becomes every\n/// dispatched member\'s own `ArtifactCommand::Apply.description`; `actor` is recorded for audit but not wired into\n',
         '/// 🎛️ Cross-member metadata for one `dispatch_group` call. No member edit carries a hand-written description (design\n/// §20.6): history labels every row from its leaf. `actor` is recorded for audit but not wired into\n', 1),
        ('    pub actor: Option<String>,\n    pub description: Option<String>,\n    /// 🪪️ The group identity', '    pub actor: Option<String>,\n    /// 🪪️ The group identity', 1),
        ('    out.push(u8::from(description.is_some()) | (u8::from(transaction.is_some()) << 1));\n    if let Some(text) = description {\n        write_command_str(&mut out, text);\n    }\n', '    out.push(u8::from(transaction.is_some()) << 1);\n', 1),
    ],
    HISTORY: [
        ('    pub finished_at: Option<String>,\n    pub description: Option<String>,\n    /// 🏷️ The id of the action', '    pub finished_at: Option<String>,\n    /// 🏷️ The id of the action', 1),
        ('const F_EDIT_DESCRIPTION: u16 = 4;\n', '', 1),
        ('            FieldSpec::new(F_EDIT_DESCRIPTION, "description", Shape::Text).optional(),\n', '', 1),
        ('        finished_at: Option<String>,\n        description: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }', '        finished_at: Option<String>,\n        verb: Option<String>,\n        line: Option<String>,\n    }', 1),
        ('                    description: field_text(&record, F_EDIT_DESCRIPTION),\n', '', 1),
        ('        if let Some(description) = &edit.description {\n            fields.push((F_EDIT_DESCRIPTION, semio_framework_dsl_record::FieldValue::Text(description.clone())));\n        }\n', '', 1),
        ('// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,\n// bit4 explicit_meta, bit5 has_backwards_section, bit6 lane, bit7 verb), id, started(ts), [actor(dictref)],\n// [finished(ts)], [key(str)], [description(str)], [lane(str)], [verb(str)],',
         '// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bits 2-3 unassigned and refused,\n// bit4 explicit_meta, bit5 has_backwards_section, bit6 lane, bit7 verb), id, started(ts), [actor(dictref)],\n// [finished(ts)], [lane(str)], [verb(str)],', 1),
        ('    if edit.description.is_some() {\n        presence |= 1 << 3;\n    }\n', '', 1),
        ('    if let Some(description) = &edit.description {\n        write_str_field(&mut out, description).await;\n    }\n', '', 1),
        ('    if presence & (1 << 2) != 0 {\n        return Err(ProtocolError::Malformed { what: "edit presence", offset: 0, detail: "bit 2 is unassigned".into() });\n    }\n    let description = if presence & (1 << 3) != 0 { Some(read_str_field(&mut input).await?) } else { None };\n',
         '    if presence & 0b1100 != 0 {\n        return Err(ProtocolError::Malformed { what: "edit presence", offset: 0, detail: "bits 2 and 3 are unassigned".into() });\n    }\n', 1),
        ('    Ok(HistoryEdit { id, actor, started_at, finished_at, description, verb, line, ops, inverse, meta, lane })', '    Ok(HistoryEdit { id, actor, started_at, finished_at, verb, line, ops, inverse, meta, lane })', 1),
    ],
    CANONICAL: [
        ('    fn fields(&self) -> [(&\'static str, bool); 12] {\n        let mut fields = [("", false); 12];\n        match self {\n            Self::Edit(edit) => {\n                fields[..11].copy_from_slice(&[',
         '    fn fields(&self) -> [(&\'static str, bool); 12] {\n        let mut fields = [("", false); 12];\n        match self {\n            Self::Edit(edit) => {\n                fields[..10].copy_from_slice(&[', 1),
        ('                    ("description", edit.description.is_some()),\n', '', 1),
        ('                5 => Self::Scalar(N::String(edit.description.as_deref().ok_or_else(invalid_path)?)),\n                6 => Self::Scalar(N::String(edit.verb.as_deref().ok_or_else(invalid_path)?)),\n                7 => Self::Scalar(N::I64(i64::from(edit.sequence_number))),\n                8 => Self::Scalar(N::String(&edit.started_at)),\n                9 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),\n                10 => Self::Scalar(edit.line.as_deref().map_or(N::Null, N::String)),',
         '                5 => Self::Scalar(N::String(edit.verb.as_deref().ok_or_else(invalid_path)?)),\n                6 => Self::Scalar(N::I64(i64::from(edit.sequence_number))),\n                7 => Self::Scalar(N::String(&edit.started_at)),\n                8 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),\n                9 => Self::Scalar(edit.line.as_deref().map_or(N::Null, N::String)),', 1),
    ],
    CLI: [
        ('    edit.description.hash(&mut hasher);\n', '', 1),
    ],
}

EXTRA_TYPES = {"RetainedClonePreparation", "DurableOwnedMapMemberAdmissionV1"}
DURABLE = f"{OS}/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs"
STORE_UNIT = f"{OS}/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
AFTER: dict[str, list[tuple[str, str, int]]] = {
    DURABLE: [
        (", mutation: Mutation, description: Option<String>) -> Self {\n        Self { operation, expected_generation, expected_revision, actor, mutation: Some(mutation), description }",
         ", mutation: Mutation) -> Self {\n        Self { operation, expected_generation, expected_revision, actor, mutation: Some(mutation) }", 1),
        ("                let description = admission.description.take();\n", "", 3),
        ("                        admission.description = description;\n", "", 3),
    ],
    STORE_UNIT: [
        ("            if let Some(description) = request.description.as_mut().filter(|value| !value.is_empty()) {\n                let bytes = description.chars().next_back().unwrap().len_utf8();\n                if bytes > grant.maximum_bytes {\n                    return Ok(SnapshotRetirementStep::Blocked);\n                }\n                description.pop();\n                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: bytes });\n            }\n", "", 1),
        ("        if let Some(id) = self.description.take() {\n            self.active_id_retirement = Some(ArtifactStoreStringRetirement::new(id));\n            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n        }\n", "", 1),
    ],
    f"{OS}/🏪️store/🧪️tests/🧪️tool-transaction/🦀️.rs": [
        ("        assert_eq!((edit.description.as_deref(), edit.finished_at.is_some()), (None, true));\n", "        assert!(edit.finished_at.is_some());\n", 1),
    ],
    f"{OS}/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs": [
        ("            let description = if rng.next_bool().await { Some(next_text(&mut rng, profile.adversarial).await) } else { None };\n", "", 1),
    ],
    CLI: [
        ("        let description = edit.description.as_deref().map_or(String::new(), |d| format!(\" \\\"{d}\\\"\"));\n", "", 1),
        ("{checkpoint_marker}{description}\"", "{checkpoint_marker}\"", 1),
    ],
    f"{OS}/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs": [
        ("    let (edit_id, description) = (edit.id.clone(), edit.description.clone());\n", "    let edit_id = edit.id.clone();\n", 1),
        ("    assert_eq!(description, None, \"the row label comes from the tool, never from a single-locale description\");\n", "", 1),
    ],
    f"{OS}/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs": [
        ("    for (operation, description) in [(TestMutation::SetCount(SetCount { value: 1 }), \"Set one\"), (TestMutation::SetLabel(SetLabel { value: \"edited\".into() }), \"Relabel\"), (TestMutation::SetCount(SetCount { value: 2 }), \"Set two\")] {",
         "    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: \"edited\".into() }), TestMutation::SetCount(SetCount { value: 2 })] {", 1),
    ],
    f"{OS}/🔌️plugin/🪟️window/🎚️config/🦀️.rs": [
        (".len().saturating_add(description.map_or(0, str::len));", ".len();", 1),
        ("\n            .saturating_add(self.description.as_ref().map_or(0, String::len));", ";", 1),
    ],
}

MESSAGES = [
    (" rejected its lane or description envelope", " rejected its lane"), (" rejects its lane or description envelope", " rejects its lane"),
    (" rejected its lane or description", " rejected its lane"), (" rejected lane or description", " rejected its lane"),
    ("its lane, description, or mutation", "its lane or mutation"), ("-lane-or-description-envelope", "-lane"), (".lane-or-description-envelope", ".lane"),
]
CONDITIONS = [
    re.compile(r"\s*\|\|\s*(?:request\.)?description(?:\.as_ref\(\)|\.as_deref\(\))?\.is_some_and\(\|(\w+)\| \1\.len\(\) > [\w:+ ]+\)"),
    re.compile(r"\s*&&\s*(?:request\.)?description(?:\.as_ref\(\)|\.as_deref\(\))?\.is_none_or\(\|(\w+)\| \1\.len\(\) <= [\w:+ ]+\)"),
]
OWNER_CONDITIONS = [
    re.compile(r"\n([ \t]*)if (?:let Some\(description\) = self\.description\.(?:as_ref|take)\(\)|self\.description\.take\(\)\.is_some\(\)) \{\n(?:\1[ \t]+[^\n]*\n|\n)*?\1\}"),
    re.compile(r"self\.description\.take\(\)\.is_some\(\) \|\| "),
    re.compile(r" \|\| self\.description\.is_some\(\)"),
    re.compile(r" \|\| self\.description\.take\(\)\.is_some\(\)"),
    re.compile(r" && self\.description\.is_none\(\)"),
    re.compile(r"\n[ \t]*\|\| self\.description\.take\(\)\.is_some\(\)"),
    re.compile(r"\n[ \t]*&& self\.description\.is_none\(\)"),
]
SIZED_DESCRIPTION_ASSERT = re.compile(r"\n[ \t]*assert!\([^\n]*\.preflight\([^,\n]*, Some\([^\n]*\n")
TRIPLE_OWNERS = re.compile(r"let \((\w+), (mut )?(\w+), _?\w*\) = (\w+)\.into_owners\(\);")


MASKED = re.compile(r"""//[^\n]*|/\*|(?<![A-Za-z0-9_])b?r#*"|(?<![A-Za-z0-9_])b?"(?:[^"\\]|\\.)*"|'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'""", re.S)


def fast_code_mask(text: str) -> bytearray:
    """🎭️ `lib.code_mask` at regex speed: 1 for Rust code, 0 inside comments, strings and char literals."""
    mask = bytearray(b"\x01") * len(text)
    position = 0
    while True:
        match = MASKED.search(text, position)
        if match is None:
            return mask
        start, end = match.start(), match.end()
        token = match.group(0)
        if token == "/*":
            depth, end = 1, start + 2
            while end < len(text) and depth:
                if text.startswith("/*", end):
                    depth, end = depth + 1, end + 2
                elif text.startswith("*/", end):
                    depth, end = depth - 1, end + 2
                else:
                    end += 1
        elif token.endswith('"') and token.lstrip("b").startswith("r"):
            hashes = token.lstrip("b")[1:-1]
            close = text.find('"' + hashes, end)
            end = len(text) if close < 0 else close + 1 + len(hashes)
        mask[start:end] = bytes(end - start)
        position = max(end, start + 1)


lib.code_mask = fast_code_mask


def bounded_head_path(text: str, mask, brace: int) -> str:
    """🧭️ `lib.head_path` without its unbounded generic walk: `=> {`/`-> T {` name no type, and a generic list is skipped
    only within one item (never across `;`, `{` or `}`)."""
    j = brace - 1
    while j >= 0 and text[j] in " \t\n":
        j -= 1
    if j >= 1 and text[j] == ">" and text[j - 1] in "=-":
        return ""
    if j >= 0 and text[j] == ">" and mask[j]:
        depth, limit = 0, max(0, j - 400)
        while j >= limit:
            if text[j] in ";{}":
                return ""
            if text[j] == ">":
                depth += 1
            elif text[j] == "<":
                depth -= 1
                if depth == 0:
                    j -= 1
                    break
            j -= 1
        else:
            return ""
        while j >= 0 and text[j] in " \t":
            j -= 1
        if j >= 1 and text[j - 1:j + 1] == "::":
            j -= 2
    end = j + 1
    while j >= 0 and (text[j].isalnum() or text[j] in "_:"):
        j -= 1
    return text[j + 1:end]


lib.head_path = bounded_head_path
TRIGGERS = ("ArtifactCommand", "GroupMeta", "HistoryEdit", "PendingEdit", "OpsHeaderLine", "CommandHeaderLine", "ArtifactStoreBatch", "ArtifactStoreOneItemPreparation",
            "MemberStoreOneItemWireRequest", "preflight", "apply_batch", "apply_one", "apply_command", "dispatch_apply_exact", "build_apply_command_bytes", "dispatch_emit_group",
            "request.description", "into_owners", "forwards", "mutation_meta")


def apply_spans(text: str, spans: list[tuple[int, int, int, str]]) -> tuple[str, list[tuple[int, str]]]:
    """✂️ Applies non-overlapping deletion spans right to left; an overlapping span waits for the next sweep."""
    kept, floor = [], len(text) + 1
    for a, b, line, what in sorted(spans, key=lambda span: span[0], reverse=True):
        if b <= floor:
            kept.append((a, b, line, what))
            floor = a
    for a, b, _, _ in kept:
        text = text[:a] + text[b:]
    return text, [(line, what) for _, _, line, what in reversed(kept)]


def sweep(text: str, collect) -> tuple[str, list[tuple[int, str]]]:
    log: list[tuple[int, str]] = []
    while True:
        spans = collect(text, lib.code_mask(text))
        if not spans:
            return text, log
        text, applied = apply_spans(text, spans)
        log += applied


def line_of(text: str, at: int) -> int:
    return text.count("\n", 0, at) + 1


def collect_struct_fields(types: set[str]):
    def collect(text: str, mask: list[bool]) -> list[tuple[int, int, int, str]]:
        spans = []
        for match in re.finditer(r"\{", text):
            brace = match.start()
            if not mask[brace]:
                continue
            head = lib.head_path(text, mask, brace)
            if not head:
                continue
            parts = head.split("::")
            last, tail = parts[-1], "::".join(parts[-2:])
            if not (last in types or tail in types or last == "Edit"):
                continue
            try:
                close = lib.matching(text, mask, brace)
            except ValueError:
                continue
            items = lib.top_level_items(text, mask, brace, close)
            keys = [(re.match(r"\s*(?:ref\s+|mut\s+)*([A-Za-z_][A-Za-z0-9_]*)", text[s:e]) or [None, ""])[1] for s, e in items]
            if last == "Edit" and last not in types and tail not in types and (not ({"forwards", "mutation_meta"} & set(keys)) or (len(parts) > 1 and parts[-2][:1].isupper())):
                continue
            for index, (s, e) in enumerate(items):
                item = text[s:e].strip()
                if re.fullmatch(r"(?:ref\s+|mut\s+)*description(\s*:.*)?", item, flags=re.S):
                    a, b = lib.remove_item(text, items, index, close)
                    spans.append((a, b, line_of(text, brace), f"{head} {{ {item[:60]} }}"))
                    break
        return spans
    return collect


def collect_calls(text: str, mask: list[bool]) -> list[tuple[int, int, int, str]]:
    spans = []
    names = sorted({name.lstrip(".") for name in CALLS})
    for match in re.finditer(r"(\.)?\b(" + "|".join(map(re.escape, names)) + r")\s*\(", text):
        if not mask[match.start(2)]:
            continue
        name = match.group(2)
        key = f".{name}" if f".{name}" in CALLS else name
        if key.startswith(".") and match.group(1) is None:
            continue
        if text[:match.start(2)].rstrip().endswith("fn"):
            continue
        index, arity = CALLS[key]
        open_at = match.end() - 1
        close = lib.matching(text, mask, open_at)
        items = lib.top_level_items(text, mask, open_at, close)
        if len(items) != arity:
            continue
        a, b = lib.remove_item(text, items, index, close)
        spans.append((a, b, line_of(text, match.start()), f"{name}(… arg {index}: {text[items[index][0]:items[index][1]].strip()[:70]})"))
    return spans


def collect_definitions(text: str, mask: list[bool]) -> list[tuple[int, int, int, str]]:
    spans = []
    for match in re.finditer(r"\bfn\s+(" + "|".join(map(re.escape, sorted(DEFINITIONS))) + r")\b", text):
        if not mask[match.start()]:
            continue
        open_at = match.end()
        while open_at < len(text) and text[open_at] in " \t\n":
            open_at += 1
        if open_at < len(text) and text[open_at] == "<":
            depth = 0
            while open_at < len(text):
                depth += {"<": 1, ">": -1}.get(text[open_at], 0)
                open_at += 1
                if depth == 0:
                    break
        if open_at >= len(text) or text[open_at] != "(":
            continue
        close = lib.matching(text, mask, open_at)
        items = lib.top_level_items(text, mask, open_at, close)
        for index, (s, e) in enumerate(items):
            if re.fullmatch(r"\s*(?:mut\s+)?_?description\s*:\s*Option<(?:String|&str|&'\w+ str)>\s*", text[s:e]):
                a, b = lib.remove_item(text, items, index, close)
                spans.append((a, b, line_of(text, match.start()), f"fn {match.group(1)}(… param {text[s:e].strip()})"))
                break
    return spans


def preparation_types(text: str) -> set[str]:
    """🧩️ Structs a domain keeps a retained `description` owner in: every literal head initialised from `request.description`."""
    mask = lib.code_mask(text)
    found = set()
    for match in re.finditer(r"description:\s*request\.description\b", text):
        depth, j = 0, match.start()
        while j >= 0:
            if mask[j]:
                if text[j] in ")]}":
                    depth += 1
                elif text[j] in "([{":
                    if depth == 0:
                        break
                    depth -= 1
            j -= 1
        if j >= 0 and text[j] == "{":
            head = lib.head_path(text, mask, j)
            if head:
                found.add(head.split("::")[-1])
    return found


def strip_definition_fields(text: str, types: set[str]) -> tuple[str, list[tuple[int, str]]]:
    removed: list[tuple[int, str]] = []
    for name in sorted(types):
        pattern = re.compile(r"(\n[ \t]*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(name) + r"\b[^{;]*\{)([^{}]*)\}")
        match = pattern.search(text)
        if match is None:
            continue
        body = match.group(2)
        fixed = re.sub(r"\n[ \t]*(?:pub(?:\([^)]*\))?\s+)?description:\s*Option<String>,", "", body, count=1)
        if fixed != body:
            removed.append((text.count("\n", 0, match.start()) + 2, f"struct {name} {{ description }}"))
            text = text[:match.start(2)] + fixed + text[match.end(2):]
    return text, removed


def transform(path: str, text: str) -> tuple[str, list[tuple[int, str]]]:
    log: list[tuple[int, str]] = []
    for old, new, count in EXACT.get(path, []):
        found = text.count(old)
        if found != count:
            raise SystemExit(f"[DEBUG] {path}: anchor found {found}x, expected {count}x: {old[:120]!r}")
        log.append((text.count("\n", 0, text.find(old)) + 1, f"exact x{count}: {old.splitlines()[0][:80]}"))
        text = text.replace(old, new)
    for match in list(SIZED_DESCRIPTION_ASSERT.finditer(text)):
        log.append((text.count("\n", 0, match.start()) + 1, f"assert dropped: {match.group(0).strip()[:90]}"))
    text = SIZED_DESCRIPTION_ASSERT.sub("\n", text)
    prepared = preparation_types(text)
    if "Self" in prepared:
        log.append((0, "WARNING: a `Self { description: request.description }` literal needs a manual edit"))
        prepared.discard("Self")
    text, removed = strip_definition_fields(text, prepared | {name for name in EXTRA_TYPES if name in text})
    log += removed
    text, removed = sweep(text, collect_struct_fields(TYPES | prepared | PREPARED_ALL | EXTRA_TYPES))
    log += removed
    text, removed = sweep(text, collect_calls)
    log += removed
    text, removed = sweep(text, collect_definitions)
    log += removed
    owners = prepared or any(name in text for name in EXTRA_TYPES)
    for pattern in CONDITIONS + (OWNER_CONDITIONS if owners else []):
        for match in list(pattern.finditer(text)):
            log.append((text.count("\n", 0, match.start()) + 1, f"condition: {match.group(0).strip()[:80]}"))
        text = pattern.sub("", text)
    for match in list(TRIPLE_OWNERS.finditer(text)):
        log.append((text.count("\n", 0, match.start()) + 1, f"owners: {match.group(0)[:80]}"))
    text = TRIPLE_OWNERS.sub(lambda m: f"let ({m.group(1)}, {m.group(2) or ''}{m.group(3)}) = {m.group(4)}.into_owners();", text)
    for old, new, count in AFTER.get(path, []):
        found = text.count(old)
        if found != count:
            raise SystemExit(f"[DEBUG] {path}: after-anchor found {found}x, expected {count}x: {old[:120]!r}")
        log.append((text.count("\n", 0, text.find(old)) + 1, f"after x{count}: {old.strip().splitlines()[0][:80]}"))
        text = text.replace(old, new)
    if log:
        for old, new in MESSAGES:
            if old in text:
                log.append((text.count("\n", 0, text.find(old)) + 1, f"message: {old!r}"))
                text = text.replace(old, new)
    return text, log


def candidates() -> list[str]:
    tokens = r"\bdescription\b"
    out = subprocess.run(["git", "-c", "core.quotepath=off", "grep", "-l", "--untracked", "-P", tokens, "--", "*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True, check=True).stdout
    return [line for line in out.splitlines() if line]


PREPARED_ALL: set[str] = set()


SCRATCH = TICKET / "🗑️generated" / "s5-channel"
BACKUP = SCRATCH / "waveb-backup"
MANIFEST = SCRATCH / "waveb-manifest.json"


def digest(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()


def flag_value(flag: str) -> str | None:
    if flag not in sys.argv:
        return None
    index = sys.argv.index(flag) + 1
    if index >= len(sys.argv) or sys.argv[index].startswith("--"):
        raise SystemExit(f"[DEBUG] {flag} needs a path")
    return sys.argv[index]


def restore() -> None:
    """♻️ Puts every written file back to its pre-apply bytes — only where the file still holds exactly what `--apply` wrote."""
    if not MANIFEST.is_file():
        raise SystemExit(f"[DEBUG] no manifest at {MANIFEST}: nothing was applied")
    manifest = json.loads(MANIFEST.read_text())
    if not manifest:
        raise SystemExit("[DEBUG] empty manifest: refusing to restore")
    restored, conflicts = 0, []
    for path, row in manifest.items():
        backup = BACKUP / path
        current = (ROOT / path).read_text()
        if digest(current) == row["before"]:
            continue
        if digest(current) != row["after"] or not backup.is_file() or digest(backup.read_text()) != row["before"]:
            conflicts.append(path)
            continue
        (ROOT / path).write_text(backup.read_text())
        restored += 1
    print(f"[DEBUG] restored {restored} of {len(manifest)} files; {len(conflicts)} conflicts (edited since the apply, left untouched)")
    for path in conflicts:
        print(f"   CONFLICT {path}")
    if conflicts:
        raise SystemExit(1)


def main() -> None:
    if "--restore" in sys.argv:
        return restore()
    apply = "--apply" in sys.argv
    files_from = flag_value("--files-from")
    list_to = flag_value("--list")
    if apply and files_from is None:
        raise SystemExit("[DEBUG] --apply needs --files-from <list written by a dry run with --list> (explicit file list, fail closed)")
    expected: list[str] = []
    if files_from is not None:
        expected = [line for line in pathlib.Path(files_from).read_text().splitlines() if line]
        if not expected:
            raise SystemExit(f"[DEBUG] {files_from} is empty: refusing to run")
    if not (ROOT / ".git").exists() or not (ROOT / "Cargo.toml").is_file():
        raise SystemExit(f"[DEBUG] {ROOT} is not the repo root: refusing to run")
    writes: dict[str, str] = {}
    sources: dict[str, str] = {}
    hot_logs: dict[str, list] = {}
    hot_results: dict[str, str] = {}
    paths = [path for path in candidates() if (ROOT / path).is_file()]
    if len(paths) < len(EXACT):
        raise SystemExit(f"[DEBUG] only {len(paths)} candidate files: the candidate search is broken, refusing to run")
    for path in paths:
        source = (ROOT / path).read_text()
        if "request.description" in source:
            PREPARED_ALL.update(preparation_types(source) - {"Self"})
    print(f"[DEBUG] preparation types: {sorted(PREPARED_ALL)}")
    for path in paths:
        if not (ROOT / path).is_file():
            continue
        source = (ROOT / path).read_text()
        if path not in EXACT and not any(token in source for token in TRIGGERS):
            continue
        result, log = transform(path, source)
        if result == source:
            continue
        if path in HOT:
            hot_logs[path] = log
            hot_results[path] = result
            continue
        writes[path] = result
        sources[path] = source
        print(f"== {path} ({len(log)})")
        for line, what in log:
            print(f"   {line}: {what}")
        mask = fast_code_mask(result)
        for match in re.finditer(r"\bdescription\b", result):
            if mask[match.start()]:
                start = result.rfind("\n", 0, match.start()) + 1
                end = result.find("\n", match.start())
                print(f"   REMAIN {result.count(chr(10), 0, match.start()) + 1}: {result[start:end].strip()[:150]}")
    for path, log in hot_logs.items():
        print(f"== HOT (Edit tool, not written) {path} ({len(log)})")
        for line, what in log:
            print(f"   {line}: {what}")
    missing = [path for path in EXACT if path not in writes]
    if missing:
        raise SystemExit(f"[DEBUG] exact-edit files untouched: {missing}")
    if files_from is not None and sorted(expected) != sorted(writes):
        extra, lost = sorted(set(writes) - set(expected)), sorted(set(expected) - set(writes))
        raise SystemExit(f"[DEBUG] the write set differs from {files_from}: {len(extra)} not listed {extra[:5]}, {len(lost)} listed but unchanged {lost[:5]} — re-run the dry run with --list and review")
    if list_to is not None:
        pathlib.Path(list_to).write_text("".join(f"{path}\n" for path in sorted(writes)))
    print(f"[DEBUG] {len(writes)} files {'written' if apply else 'would change'}; {len(hot_logs)} hot files reported")
    if apply:
        drifted = [path for path, source in sources.items() if (ROOT / path).read_text() != source]
        if drifted:
            raise SystemExit(f"[DEBUG] {len(drifted)} files changed while the wave was computed, nothing written: {drifted[:5]}")
        manifest = {}
        for path, text in writes.items():
            backup = BACKUP / path
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_text(sources[path])
            manifest[path] = {"before": digest(sources[path]), "after": digest(text)}
        MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=1))
        for path, text in writes.items():
            (ROOT / path).write_text(text)
    elif "--preview" in sys.argv:
        for path, text in {**writes, **hot_results}.items():
            target = SCRATCH / "waveb-preview" / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text)


if __name__ == "__main__":
    main()
