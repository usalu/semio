#!/usr/bin/env python3
"""🔖️ S5-CHANNEL wave C: the channel half of the ONE bump to CHANNEL_VERSION 23 (coordinator decision 2026-10-05 09:36).

Wave C carries two owners' layout changes behind one version: S5-NESTED's owner path (N2) and S5-STORE's design §22.28
(`sequence_number` leaves the revision digest). This script is the part no other owner stages:
- PIN: the pin, the hostile rows that bracket it, and the handshake law in BOTH directions of the bump — the corpus gains
  `hostOffset`, so "a guest of 22 at a host of 23" and "a guest of 23 at a host of 22" are two named cases at every future pin.
- FRAMES (N2 field list, S5-NESTED 09:50): `ChildPackEntry.owner` after `envelope_pack` and `ChildHeadPackEntry.owner` after
  `head_pack`, text encoded like `slot`, in the Rust codec, its TypeScript twin, and the golden rows of both suites (one row
  `owner: ""`, one row `owner: "content/forms-1"`). `--without-frames` leaves them out — then S5-NESTED lands `--without-frames`
  too, because its producers name these fields.
- DIGEST (design §22.28, rule from S5-STORE 10:20): the two TypeScript oracles of the canonical edit digest the edit's revision
  value — its members in declaration order without `sequenceNumber` — and the schema says so. `--without-digest` leaves them out
  (then the fixture reseal `🧪️s5-channel-reseal-canonical-edit.ts` stays out too).
- FUNNEL (F9 class, opt-in `--with-funnel`): the program's one typed-value encoder `encode_wire_serialized` (47 uses: faults,
  effects, reports, history patches) orders object members by key bytes at every depth, and a law says so. No version gate: it
  restores the bytes build B0 sent.
`--apply` writes the rows (explicit files, every anchor exactly its count and its replacement not there yet, else nothing) after
backing every file up; `--restore` applies the same rows backwards (every replacement exactly its count, else nothing), so it
holds beside the literals `generate` rewrote in the same files. The generated literals and
the derived fixtures follow by command (printed at the end).
Usage: python3 🧪️s5-channel-wave-c.py [--apply | --verify | --restore] [--without-frames] [--without-digest] [--with-funnel]
"""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
BACKUP = TICKET / "🗑️generated/s5-channel/wave-c-backup"
MANIFEST = TICKET / "🗑️generated/s5-channel/wave-c-manifest.json"
OS = "🧰️framework/🛍️products/💻️os"
OSM = f"{OS}/🔨️modules"
FROM, TO = 22, 23
PIN = f"{OS}/🧫️fixtures/📡️channel/🔖️channel-version.json"
REGISTRY = f"{OS}/🧫️fixtures/📡️channel/📇️consumers.json"
PUBLICATION = f"{OSM}/🔌️plugin/📇️registry/🧫️fixtures/🧬️catalog-publication/🔣️.json"
HANDSHAKE = f"{OSM}/📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake/🔣️.json"
HANDSHAKE_SCHEMA = f"{OSM}/📡️spr/🧵️channel/🧬️schema/🔣️channel-handshake/🔣️.json"
CHANNEL = f"{OSM}/📡️spr/🧵️channel/🦀️.rs"
CHANNEL_TESTS = f"{OSM}/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"
TWIN = f"{OS}/🟦️.ts"
TWIN_TESTS = f"{OS}/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
MCP_TESTS = f"{OSM}/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs"
EDIT_SCHEMA = f"{OSM}/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json"
EDIT_ORACLE = f"{OSM}/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts"
SEALER_ORACLE = f"{OSM}/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts"

Rows = dict[str, list[tuple[str, str, int]]]

PIN_ROWS: Rows = {
    PIN: [
        (f'  "channelVersion": {FROM}\n}}', f'  "channelVersion": {TO}\n}}', 1),
    ],
    PUBLICATION: [
        (f'      "id": "previous-app-channel-version",\n      "change": "protocol",\n      "appChannelVersion": {FROM - 1},', f'      "id": "previous-app-channel-version",\n      "change": "protocol",\n      "appChannelVersion": {TO - 1},', 1),
        (f'      "id": "future-app-channel-version",\n      "change": "protocol",\n      "appChannelVersion": {FROM + 1},', f'      "id": "future-app-channel-version",\n      "change": "protocol",\n      "appChannelVersion": {TO + 1},', 1),
        (f'  "executionProtocol": {{\n    "appChannelVersion": {FROM}\n  }}', f'  "executionProtocol": {{\n    "appChannelVersion": {TO}\n  }}', 1),
    ],
    REGISTRY: [
        (f'      "hostileValues": [\n        {FROM - 1},\n        {FROM + 1}\n      ],\n      "hostileOccurrences": 2', f'      "hostileValues": [\n        {TO - 1},\n        {TO + 1}\n      ],\n      "hostileOccurrences": 2', 1),
    ],
    HANDSHAKE: [
        ('"law": "A host admits a guest only when the guest was compiled for the host\'s own app-channel version; an older or a newer guest is refused with plugin.channel-mismatch before any frame, its notice naming {guest} and {host}.",',
         '"law": "A host admits a guest only when the guest was compiled for the host\'s own app-channel version; an older or a newer guest is refused with plugin.channel-mismatch before any frame, its notice naming {guest} and {host}. A bump is refused in both directions: the previous build\'s guest at this host, and this build\'s guest at the previous build\'s host.",', 1),
        ('    { "name": "far-older-guest", "guestOffset": -20, "admitted": false }\n',
         '    { "name": "far-older-guest", "guestOffset": -20, "admitted": false },\n    { "name": "this-guest-at-the-previous-host", "guestOffset": 0, "hostOffset": -1, "admitted": false },\n    { "name": "this-guest-at-the-next-host", "guestOffset": 0, "hostOffset": 1, "admitted": false },\n    { "name": "previous-guest-at-the-previous-host", "guestOffset": -1, "hostOffset": -1, "admitted": true }\n', 1),
    ],
    HANDSHAKE_SCHEMA: [
        ('Each case offsets the guest from the host\'s CHANNEL_VERSION (the pin), so the corpus stays true across bumps;', 'Each case offsets the guest and, optionally, the host from this build\'s CHANNEL_VERSION (the pin), so the corpus stays true across bumps and names both directions of one;', 1),
        ('"description": "The guest\'s compiled version minus the host\'s." },\n', '"description": "The guest\'s compiled version minus the pin." },\n        "hostOffset": { "type": "integer", "minimum": -64, "maximum": 64, "description": "The host\'s version minus the pin; absent for the host of this build." },\n', 1),
    ],
    CHANNEL_TESTS: [
        ('        let guest = u32::try_from(i64::from(CHANNEL_VERSION) + case["guestOffset"].as_i64().unwrap()).unwrap();\n        match admit_guest_channel_version(guest, CHANNEL_VERSION) {',
         '        let (guest_offset, host_offset) = (case["guestOffset"].as_i64().unwrap(), case["hostOffset"].as_i64().unwrap_or(0));\n        assert_eq!(case["admitted"].as_bool().unwrap(), guest_offset == host_offset, "{name}: the corpus admits exactly a guest of its host\'s version");\n        let guest = u32::try_from(i64::from(CHANNEL_VERSION) + guest_offset).unwrap();\n        let host = u32::try_from(i64::from(CHANNEL_VERSION) + host_offset).unwrap();\n        match admit_guest_channel_version(guest, host) {', 1),
        ('                assert_eq!(fault.param("host"), Some(CHANNEL_VERSION.to_string().as_str()), "{name}");', '                assert_eq!(fault.param("host"), Some(host.to_string().as_str()), "{name}");', 1),
    ],
    TWIN_TESTS: [
        ('as { code: string; cases: { name: string; guestOffset: number; admitted: boolean }[] };', 'as { code: string; cases: { name: string; guestOffset: number; hostOffset?: number; admitted: boolean }[] };', 1),
        ('        const guest = APP_CHANNEL_VERSION + row.guestOffset;\n        const refusal = admitGuestChannelVersion(guest, APP_CHANNEL_VERSION);\n',
         '        const guest = APP_CHANNEL_VERSION + row.guestOffset;\n        const host = APP_CHANNEL_VERSION + (row.hostOffset ?? 0);\n        expect(row.admitted, row.name).toBe(guest === host);\n        const refusal = admitGuestChannelVersion(guest, host);\n', 1),
        ('{ guest: String(guest), host: String(APP_CHANNEL_VERSION) }]);', '{ guest: String(guest), host: String(host) }]);', 1),
    ],
}

MEMBER = "content/forms-1"
MEMBER_HEX = f"{len(MEMBER.encode()):02x}{MEMBER.encode().hex()}"
RUST_ENTRY = 'slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string()'
TS_ENTRY = 'slot: "s", child_id: "c", dialect: "d"'
TAIL = "01010173016301640101"


def rust_golden(label: str, tag: str) -> tuple[str, str, int]:
    return (f'        "{label}" => "{tag}{TAIL}",\n', f'        "{label}" => "{tag}{TAIL}00",\n        "{label}OfAMember" => "{tag}{TAIL}{MEMBER_HEX}",\n', 1)


def ts_golden(label: str, tag: str) -> tuple[str, str, int]:
    return (f'        {label}: "{tag}{TAIL}",\n', f'        {label}: "{tag}{TAIL}00",\n        {label}OfAMember: "{tag}{TAIL}{MEMBER_HEX}",\n', 1)


def rust_corpus(label: str, head: str, entry: str, pack: str) -> tuple[str, str, int]:
    row = '        ("{label}", {head}, entries: vec![{entry} {{ {fields}, {pack}: vec![1]{owner} }}] }}),\n'
    old = row.format(label=label, head=head, entry=entry, fields=RUST_ENTRY, pack=pack, owner="")
    new = row.format(label=label, head=head, entry=entry, fields=RUST_ENTRY, pack=pack, owner=", owner: String::new()") + row.format(label=f"{label}OfAMember", head=head, entry=entry, fields=RUST_ENTRY, pack=pack, owner=f', owner: "{MEMBER}".to_string()')
    return (old, new, 1)


def ts_corpus(label: str, head: str, pack: str) -> tuple[str, str, int]:
    row = '        ["{label}", {{ {variant}: {{ {head}, entries: [{{ {fields}, {pack}: [1]{owner} }}] }} }}],\n'
    old = row.format(label=label, variant=label, head=head, fields=TS_ENTRY, pack=pack, owner="")
    new = row.format(label=label, variant=label, head=head, fields=TS_ENTRY, pack=pack, owner=', owner: ""') + row.format(label=f"{label}OfAMember", variant=label, head=head, fields=TS_ENTRY, pack=pack, owner=f', owner: "{MEMBER}"')
    return (old, new, 1)


FRAME_ROWS: Rows = {
    CHANNEL: [
        ("    /// 📦️ The child's full envelope pack (`encode_document_pack_bytes` framing: pack + spr).\n    pub envelope_pack: Vec<u8>,\n}",
         "    /// 📦️ The child's full envelope pack (`encode_document_pack_bytes` framing: pack + spr).\n    pub envelope_pack: Vec<u8>,\n    /// 🧭️ The owner path of the member that owns this entry (`slot/childId[/slot/childId]*`, `%25` / `%2F` escaped); empty for the document itself. CHANNEL_VERSION 23 wire addition.\n    pub owner: String,\n}", 1),
        ("    /// 📦️ The child's head snapshot pack.\n    pub head_pack: Vec<u8>,\n}",
         "    /// 📦️ The child's head snapshot pack.\n    pub head_pack: Vec<u8>,\n    /// 🧭️ The owner path of the member that owns this entry, as [`ChildPackEntry::owner`]. CHANNEL_VERSION 23 wire addition.\n    pub owner: String,\n}", 1),
        ("                out.string(&entry.dialect)?;\n                out.bytes(&entry.envelope_pack)?;\n            }\n        }\n        AppCommand::ReadChildren { seq } => {",
         "                out.string(&entry.dialect)?;\n                out.bytes(&entry.envelope_pack)?;\n                out.string(&entry.owner)?;\n            }\n        }\n        AppCommand::ReadChildren { seq } => {", 1),
        ("/// 🧸️ `count varint | (slot, child_id, dialect, envelope_pack)*`", "/// 🧸️ `count varint | (slot, child_id, dialect, envelope_pack, owner)*`", 1),
        ("        crate::os_spr::write_bytes(out, &entry.envelope_pack);\n    }\n}\n\n/// 🪆️ `count varint | (slot, child_id, dialect, head_pack)*`",
         "        crate::os_spr::write_bytes(out, &entry.envelope_pack);\n        crate::os_spr::write_str(out, &entry.owner);\n    }\n}\n\n/// 🪆️ `count varint | (slot, child_id, dialect, head_pack, owner)*`", 1),
        ("        crate::os_spr::write_bytes(out, &entry.head_pack);\n", "        crate::os_spr::write_bytes(out, &entry.head_pack);\n        crate::os_spr::write_str(out, &entry.owner);\n", 1),
        ("head_pack: crate::os_spr::read_bytes(bytes, pos)? });", "head_pack: crate::os_spr::read_bytes(bytes, pos)?, owner: crate::os_spr::read_str(bytes, pos)? });", 1),
        ("dialect: crate::os_spr::read_str(bytes, pos)?, envelope_pack: crate::os_spr::read_bytes(bytes, pos)? });", "dialect: crate::os_spr::read_str(bytes, pos)?, envelope_pack: crate::os_spr::read_bytes(bytes, pos)?, owner: crate::os_spr::read_str(bytes, pos)? });", 1),
    ],
    TWIN: [
        ("export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[] };",
         "export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[]; readonly owner: string };", 1),
        ("export type ChildHeadPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly head_pack: readonly number[] };",
         "export type ChildHeadPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly head_pack: readonly number[]; readonly owner: string };", 1),
        ("  writeStr(out, entry.dialect);\n  writeBytes(out, entry.envelope_pack);\n}\nfunction readChildPackEntry", "  writeStr(out, entry.dialect);\n  writeBytes(out, entry.envelope_pack);\n  writeStr(out, entry.owner);\n}\nfunction readChildPackEntry", 1),
        ("dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };\n}\nfunction writeVecChildPackEntry", "dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos), owner: readStr(bytes, pos) };\n}\nfunction writeVecChildPackEntry", 1),
        ("    writeBytes(out, entry.head_pack);\n", "    writeBytes(out, entry.head_pack);\n    writeStr(out, entry.owner);\n", 1),
        ("dialect: readStr(bytes, pos), head_pack: readBytes(bytes, pos) }));", "dialect: readStr(bytes, pos), head_pack: readBytes(bytes, pos), owner: readStr(bytes, pos) }));", 1),
    ],
    CHANNEL_TESTS: [
        ("envelope_pack: vec![7, 8, 9] },", "envelope_pack: vec![7, 8, 9], owner: String::new() },", 1),
        ("envelope_pack: Vec::new() },", f'envelope_pack: Vec::new(), owner: "{MEMBER}".to_string() }},', 1),
        ("head_pack: vec![7, 8, 9] },", "head_pack: vec![7, 8, 9], owner: String::new() },", 1),
        ("head_pack: Vec::new() },", f'head_pack: Vec::new(), owner: "{MEMBER}".to_string() }},', 1),
        rust_corpus("LoadChildren", "AppCommand::LoadChildren { seq: 1", "ChildPackEntry", "envelope_pack"),
        rust_corpus("Children", "AppFrame::Children { in_reply_to: 1", "ChildPackEntry", "envelope_pack"),
        rust_corpus("ChildHeads", "AppFrame::ChildHeads { in_reply_to: 1", "ChildHeadPackEntry", "head_pack"),
        rust_golden("LoadChildren", "0e"),
        rust_golden("Children", "0c"),
        rust_golden("ChildHeads", "20"),
    ],
    TWIN_TESTS: [
        (f'      {{ LoadChildren: {{ seq: 18, entries: [{{ {TS_ENTRY}, envelope_pack: [1] }}] }} }},\n', f'      {{ LoadChildren: {{ seq: 18, entries: [{{ {TS_ENTRY}, envelope_pack: [1], owner: "" }}, {{ slot: "s", child_id: "n", dialect: "d", envelope_pack: [], owner: "{MEMBER}" }}] }} }},\n', 1),
        (f'      {{ Children: {{ in_reply_to: 13, entries: [{{ {TS_ENTRY}, envelope_pack: [1] }}] }} }},\n', f'      {{ Children: {{ in_reply_to: 13, entries: [{{ {TS_ENTRY}, envelope_pack: [1], owner: "{MEMBER}" }}] }} }},\n', 1),
        ('head_pack: [7, 8, 9] }, { slot: "brep", child_id: "child-2", dialect: "s.stdio.brep@1/*", head_pack: [] }] } },\n', f'head_pack: [7, 8, 9], owner: "" }}, {{ slot: "brep", child_id: "child-2", dialect: "s.stdio.brep@1/*", head_pack: [], owner: "{MEMBER}" }}] }} }},\n', 1),
        ts_corpus("LoadChildren", "seq: 1", "envelope_pack"),
        ts_corpus("Children", "in_reply_to: 1", "envelope_pack"),
        ts_corpus("ChildHeads", "in_reply_to: 1", "head_pack"),
        ts_golden("LoadChildren", "0e"),
        ts_golden("Children", "0c"),
        ts_golden("ChildHeads", "20"),
        ("head_pack: [7, 8] }]", f'head_pack: [7, 8], owner: "{MEMBER}" }}]', 2),
    ],
    MCP_TESTS: [
        ("head_pack: vec![1, 2, 3] },", "head_pack: vec![1, 2, 3], owner: String::new() },", 1),
        ("head_pack: vec![4] },", "head_pack: vec![4], owner: String::new() },", 1),
    ],
}

REVISION = "/** 🔢️ An edit's revision value: its members in declaration order without `sequenceNumber` (design §22.28) — where an edit sits in its history is not part of what it is. */\nconst revisionValue = ({ sequenceNumber: _sequenceNumber, ...revision }: Record<string, unknown>): Record<string, unknown> => revision;\n"
REVISION_JSON = "The edit's revision JSON: its members in declaration order without sequenceNumber (design §22.28), the preimage of a single-operation edit's digest."

DIGEST_ROWS: Rows = {
    EDIT_SCHEMA: [
        ('        "expectedJson": {\n          "type": "string",\n          "minLength": 1\n        },', f'        "expectedJson": {{\n          "type": "string",\n          "minLength": 1,\n          "description": "{REVISION_JSON}"\n        }},', 1),
        ('        "expectedJson": {\n          "type": "string"\n        },', f'        "expectedJson": {{\n          "type": "string",\n          "description": "{REVISION_JSON}"\n        }},', 1),
    ],
    EDIT_ORACLE: [
        ('const sha256 = (value: string): string => createHash("sha256").update(value).digest("hex");\n', 'const sha256 = (value: string): string => createHash("sha256").update(value).digest("hex");\n' + REVISION, 1),
        ('    assert.equal(JSON.stringify(fixture.edit), fixture.expectedJson, `${name} canonical JSON emits declaration order`);', '    assert.equal(JSON.stringify(revisionValue(fixture.edit)), fixture.expectedJson, `${name} canonical JSON emits declaration order without the sequence number`);', 1),
        ('    if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(edit))]);\n    const sequence = Buffer.alloc(4);\n    sequence.writeInt32BE(edit.sequenceNumber as number);\n',
         '    if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(revisionValue(edit)))]);\n', 1),
        ('      ...text(edit, "actor"),\n      sequence,\n      Buffer.from(String(edit.startedAt)),', '      ...text(edit, "actor"),\n      Buffer.from(String(edit.startedAt)),', 1),
    ],
    SEALER_ORACLE: [
        ('/** 🧪️ Validates language-neutral canonical bytes and private Store sealer source boundaries. */\n', REVISION + '\n/** 🧪️ Validates language-neutral canonical bytes and private Store sealer source boundaries. */\n', 1),
        ('  if (fixture.expectedJson !== JSON.stringify(fixture.edit) || expected.length <= 4096)', '  if (fixture.expectedJson !== JSON.stringify(revisionValue(fixture.edit)) || expected.length <= 4096)', 1),
        ('canonicalBytes(fixture.edit)', 'canonicalBytes(revisionValue(fixture.edit))', 1),
        ('  if (mapFixture.expectedJson !== JSON.stringify(mapFixture.edit))', '  if (mapFixture.expectedJson !== JSON.stringify(revisionValue(mapFixture.edit)))', 1),
        ('canonicalBytes(mapFixture.edit)', 'canonicalBytes(revisionValue(mapFixture.edit))', 2),
    ],
}

RUNTIME = f"{OSM}/🔌️plugin/🦀️.rs"
DISPATCH_TESTS = f"{OSM}/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"

FUNNEL_ROWS: Rows = {
    RUNTIME: [
        ("    fn encode_wire_serialized<T: ToValue>(value: &T) -> Vec<u8> {\n        store::pack_rt::encode_wire_value(&value.to_value())\n    }\n",
         "    fn encode_wire_serialized<T: ToValue>(value: &T) -> Vec<u8> {\n        store::pack_rt::encode_wire_value(&key_ordered_wire_value(value.to_value()))\n    }\n\n"
         "    /// 🔡️ `value` with the members of every object in key-byte order, through arrays and objects alike — the one order a host\n"
         "    /// reader re-derives from what it decoded. The pack encoder keeps the order a value arrives in and a derived `to_value`\n"
         "    /// arrives in declaration order, which a reader that re-encodes what it read (the history patch of a publication, the\n"
         "    /// document-port control grammar) refuses as non-canonical (live fault F9 on build B1).\n"
         "    fn key_ordered_wire_value(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {\n"
         "        use semio_framework_value::DslValue;\n"
         "        match value {\n"
         "            DslValue::Array(items) => DslValue::Array(items.into_iter().map(key_ordered_wire_value).collect()),\n"
         "            DslValue::Object(members) => {\n"
         "                let mut members: Vec<(String, DslValue)> = members.into_iter().map(|(key, member)| (key, key_ordered_wire_value(member))).collect();\n"
         "                members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));\n"
         "                DslValue::Object(members)\n"
         "            }\n"
         "            scalar => scalar,\n"
         "        }\n"
         "    }\n", 1),
    ],
    DISPATCH_TESTS: [
        ("    assert!(runtime.close_quarantine.borrow().get(7).is_none());\n}\n//#endregion 📡️RegisteredQueryDispatch",
         "    assert!(runtime.close_quarantine.borrow().get(7).is_none());\n}\n\n"
         "/// 🔡️ A typed value whose declaration order is not its key-byte order, at two depths.\n"
         "#[derive(Clone, Debug, PartialEq, semio_framework_value::FromValue, semio_framework_value::ToValue)]\n"
         "#[value(rename_all = \"camelCase\")]\n"
         "struct DeclaredOrderWitness {\n    cursor: u64,\n    can_undo: bool,\n    inner: Vec<DeclaredOrderInner>,\n}\n\n"
         "/// 🪜️ The nested half of [`DeclaredOrderWitness`].\n"
         "#[derive(Clone, Debug, PartialEq, semio_framework_value::FromValue, semio_framework_value::ToValue)]\n"
         "#[value(rename_all = \"camelCase\")]\n"
         "struct DeclaredOrderInner {\n    zulu: u32,\n    alpha: u32,\n}\n\n"
         "/// 🔡️ LAW (live fault F9 class, build B1): every typed value the program sends leaves with its object members in key-byte order\n"
         "/// at every depth — the order a host reader that re-encodes what it decoded reproduces — whatever order its type declares.\n"
         "#[test]\n"
         "fn typed_wire_values_leave_in_key_byte_order_whatever_their_types_declare() {\n"
         "    fn ordered(value: &semio_framework_value::DslValue) -> bool {\n"
         "        use semio_framework_value::DslValue;\n"
         "        match value {\n"
         "            DslValue::Array(items) => items.iter().all(ordered),\n"
         "            DslValue::Object(members) => members.windows(2).all(|pair| pair[0].0.as_bytes() < pair[1].0.as_bytes()) && members.iter().all(|(_, member)| ordered(member)),\n"
         "            _ => true,\n"
         "        }\n"
         "    }\n"
         "    let witness = DeclaredOrderWitness { cursor: 7, can_undo: true, inner: vec![DeclaredOrderInner { zulu: 1, alpha: 2 }] };\n"
         "    assert!(!ordered(&semio_framework_value::ToValue::to_value(&witness)), \"the witness declares `cursor` before `canUndo`: its own order is not the wire order\");\n"
         "    let sent = super::super::encode_wire_serialized(&witness);\n"
         "    let read = semio_framework_os_kernel::pack_rt::decode_wire_value(&sent).expect(\"the sent value decodes\");\n"
         "    assert!(ordered(&read), \"members leave in key-byte order at every depth: {read:?}\");\n"
         "    assert_eq!(<DeclaredOrderWitness as semio_framework_value::FromValue>::from_value(read).expect(\"the witness reads back\"), witness);\n"
         "}\n//#endregion 📡️RegisteredQueryDispatch", 1),
    ],
}

COMMANDS = f"""after --apply, under the same lock hold:
  bun 🧪️s5-channel-reseal-canonical-edit.ts --write   # only with the DIGEST rows; restore: the same with --restore
  (cd {OSM}/🧑‍💻dev/📦️packages/🟦️typescript && bun ./📜️script.ts channel-version generate --guest)
  bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from {FROM} --to {TO} --write
  (cd {OSM}/🧑‍💻dev/📦️packages/🟦️typescript && bun ./📜️script.ts channel-version check)   # pin {TO}, findings = the host fixture component only
restore, in this order:
  bun 🧪️s5-channel-reseal-canonical-edit.ts --restore   # only with the DIGEST rows
  python3 🧪️s5-channel-wave-c.py --restore
  (cd {OSM}/🧑‍💻dev/📦️packages/🟦️typescript && bun ./📜️script.ts channel-version generate --guest)
  bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from {TO} --to {FROM} --write"""


def planned(flags: list[str]) -> Rows:
    groups = [PIN_ROWS] + ([] if "--without-frames" in flags else [FRAME_ROWS]) + ([] if "--without-digest" in flags else [DIGEST_ROWS]) + ([FUNNEL_ROWS] if "--with-funnel" in flags else [])
    plan: Rows = {}
    for group in groups:
        for path, rows in group.items():
            plan.setdefault(path, []).extend(rows)
    if not plan:
        raise SystemExit("[DEBUG] no rows: refusing")
    return plan


def rewrite(plan: Rows, mode: str) -> tuple[dict[str, bytes], dict[str, str]]:
    originals: dict[str, bytes] = {}
    results: dict[str, str] = {}
    failed = False
    for path, rows in plan.items():
        target = ROOT / path
        if not target.is_file():
            raise SystemExit(f"[DEBUG] missing file: {path}")
        originals[path] = target.read_bytes()
        text = originals[path].decode()
        print(f"== {path}")
        ordered = list(enumerate(rows, 1))
        for index, (old, new, count) in reversed(ordered) if mode == "restore" else ordered:
            found = text.count(old) if mode == "forward" else text.count(new)
            wrong = found != count or (mode == "forward" and new in text)
            failed |= wrong
            print(f"   {index} {'MISMATCH' if wrong else 'ok'} x{found}/{count}: {old.strip().splitlines()[0][:100]}")
            text = text.replace(old, new) if mode == "forward" else text.replace(new, old) if mode == "restore" else text
        results[path] = text
    if failed:
        raise SystemExit("[DEBUG] anchors do not match: nothing written")
    return originals, results


def write(originals: dict[str, bytes], results: dict[str, str]) -> None:
    for path, original in originals.items():
        if (ROOT / path).read_bytes() != original:
            raise SystemExit(f"[DEBUG] {path} changed while planning: nothing written")
    for path, text in results.items():
        (ROOT / path).write_bytes(text.encode())


def main() -> None:
    known = {"--apply", "--verify", "--restore", "--without-frames", "--without-digest", "--with-funnel"}
    flags = sys.argv[1:]
    unknown = [argument for argument in flags if argument not in known]
    if unknown or sum(argument in flags for argument in ("--apply", "--verify", "--restore")) > 1:
        raise SystemExit(f"[DEBUG] unknown or conflicting arguments {flags}: refusing")
    if not (ROOT / ".git").exists() or not TICKET.is_dir():
        raise SystemExit("[DEBUG] not the repo root or no ticket folder: refusing")
    if "--restore" in flags:
        if not MANIFEST.is_file():
            raise SystemExit("[DEBUG] no manifest: nothing was applied by this script")
        applied: list[str] = json.loads(MANIFEST.read_text())["flags"]
        originals, results = rewrite(planned(applied), "restore")
        write(originals, results)
        MANIFEST.unlink()
        print(f"[DEBUG] {len(results)} files restored (rows applied backwards; the bytes before --apply stay in {BACKUP.name})")
        return
    plan = planned(flags)
    if "--apply" in flags and MANIFEST.exists():
        raise SystemExit("[DEBUG] a manifest exists: wave C is applied already (--restore first)")
    originals, results = rewrite(plan, "verify" if "--verify" in flags else "forward")
    if "--apply" in flags:
        for path, original in originals.items():
            (BACKUP / path).parent.mkdir(parents=True, exist_ok=True)
            (BACKUP / path).write_bytes(original)
        write(originals, results)
        MANIFEST.write_text(json.dumps({"flags": [flag for flag in flags if flag.startswith("--with")], "files": sorted(results)}, ensure_ascii=False, indent=2))
    print(f"[DEBUG] {len(results)} files, {sum(len(rows) for rows in plan.values())} rows {'written' if '--apply' in flags else ('verified' if '--verify' in flags else 'would change (dry run)')}")
    print(COMMANDS)


if __name__ == "__main__":
    main()
