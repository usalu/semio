#!/usr/bin/env python3
"""📡️ U6 T6 set — stdio product drift behind the 2026-09-29 stdio red count (guest set, window 4; independent of rows 3–6).

* pdf 1.7 `set-snapshot` is a wire mutation like its siblings (the 09-27 leaf declares `binaryTag: 60`, `textOpcode:
  set-snapshot`, surfaces rust/json-schema/text/binary, but the aggregate never learned it — the binary protocol, the only
  source of op tags, has no record, the binary/text rosters stop at `remove-catalog-entry` and the aggregate JSON schema has no
  `setSnapshot` arm): every pdf editor maps every snapshot edit to `SetSnapshot` (`snapshot_edit_set_snapshot`, page drafts), so
  each one failed `encode_op` ("unknown mutation identity"). The leaf gains its direct binary/text identities, the aggregate its
  record, roster rows and arm (the xml twin is LB2 p14); the law's samples carry a whole-snapshot replacement.
* zip `set-archive-comment` carries `comment_utf8` since 09-28 (payload schema `commentUtf8`), the text grammar did not →
  `comment-utf8 = true|false`.
* zip archive edits refused the framework-injected `windowId` (every addressed lane inserts it — shell and agent), so no
  addressed rename/comment edit ever parsed → the allow-list names it.
* the bounded native-edit execution contract passed `NATIVE_MAXIMUM_WORK_ITEMS` (2) in the `max_output_bytes` slot of
  `bounded_first_step` (09-27), so the agent lane's preview refused every native edit ("produced 32 op byte(s); its exact
  output cap is 2" — docx set-page, xlsx cells, zip names alike) → the output cap is the native raw-byte bound, the snapshot-edit
  contract's own shape.
* bmp's grammar defined `hex = IDENT | TEXT`, shadowing the built-in `hex` macro — a snapshot hex starting with a digit never
  recognized → the production is removed (docx/xlsx/png idiom).
* gltf `SetSnapshot` holds the snapshot inline → `GltfMutation` exceeded its 256-byte inline carrier law → `Box<GltfSnapshot>`
  (ifc/dwg idiom).
* zip demo `🗣️.dsl.semio`/`🎒️.pack.semio` are regenerated from `demo_zip_snapshot()` under the 09-28 codec (explicit header
  metadata) — whole-file swaps guarded by the live asset's sha256, payloads under `wp-u6/payload/stdio-wire-drift/`.

Usage: u6-stdio-wire-drift.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/stdio-wire-drift") / hashlib.sha256(str(ROOT).encode()).hexdigest()[:12]
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
PDF = ART + "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/"
ZIP = ART + "🎒️zip/"
BMP = ART + "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
GLTF = ART + "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/"
PAYLOAD_ID = "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/mutation/set-snapshot/schema.json"
MODULES = '\n#[path = "💾️binary/🦀️.rs"]\npub mod binary;\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n'
CREATED = {
    PDF + "📸️set-snapshot/💾️binary/🦀️.rs": """//! 📸️ Direct binary identity for `set-snapshot`.

pub const TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "set-snapshot");
pub const BINARY_TAG: u8 = TAG;

use super::SetSnapshot;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &SetSnapshot) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(payload).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<SetSnapshot, String> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;
    dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
""",
    PDF + "📸️set-snapshot/📝️text/🦀️.rs": """//! 📸️ Direct text identity for `set-snapshot`.

pub const OPCODE: &str = "set-snapshot";
pub const TEXT_OPCODE: &str = OPCODE;

use super::SetSnapshot;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetSnapshot) -> Result<String, String> {
    Ok(pack::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
""",
}


def pdf_aggregate(text: str) -> str:
    document = json.loads(text)
    if json.dumps(document, indent=2, ensure_ascii=False) + "\n" != text:
        raise ValueError("aggregate schema is not in its canonical two-space form")
    if any(arm["allOf"][1]["properties"]["mutation"]["const"] == "setSnapshot" for arm in document["oneOf"]):
        raise ValueError("aggregate already has a setSnapshot arm")
    document["oneOf"].append({"allOf": [{"$ref": PAYLOAD_ID}, {"type": "object", "required": ["mutation"], "properties": {"mutation": {"const": "setSnapshot"}}}]})
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


PAYLOAD = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-u6/payload/stdio-wire-drift")
ZIP_DEMO = ZIP + "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/"
SWAPS = {
    ZIP_DEMO + "🗣️.dsl.semio": ("a815c05ed52be3bb6e5aa8cadea522372fecb1eb407aa157172323553fdadce3", "zip-demo-🗣️.dsl.semio"),
    ZIP_DEMO + "🎒️.pack.semio": ("4cdb9500bf725c16663971ac6308adce9c7593ee84c669326ef488d2914d6322", "zip-demo-🎒️.pack.semio"),
}
SETS = {
    PDF + "📸️set-snapshot/🦀️.rs": [("        Vec::new()\n    }\n}\n", "        Vec::new()\n    }\n}\n" + MODULES, 1)],
    PDF + "💾️binary/📡️.protocol.semio": [("record remove-catalog-entry tag=59\nfield payload bytes\n", "record remove-catalog-entry tag=59\nfield payload bytes\nrecord set-snapshot tag=60\nfield payload bytes\n", 1)],
    PDF + "💾️binary/🦀️.rs": [
        (
            '    ("RemoveCatalogEntry", "removeCatalogEntry", super::remove_catalog_entry::binary::BINARY_TAG),\n];\n',
            '    ("RemoveCatalogEntry", "removeCatalogEntry", super::remove_catalog_entry::binary::BINARY_TAG),\n    ("SetSnapshot", "setSnapshot", super::set_snapshot::binary::BINARY_TAG),\n];\n',
            1,
        )
    ],
    PDF + "📝️text/🦀️.rs": [
        (
            '    ("RemoveCatalogEntry", super::remove_catalog_entry::text::TEXT_OPCODE),\n];\n',
            '    ("RemoveCatalogEntry", super::remove_catalog_entry::text::TEXT_OPCODE),\n    ("SetSnapshot", super::set_snapshot::text::TEXT_OPCODE),\n];\n',
            1,
        )
    ],
    PDF + "🔣️.json": pdf_aggregate,
    PDF + "🧪️tests/🔬️unit/🦀️.rs": [
        (
            '        PdfMutation::SetTrailerEntry(SetTrailerEntry { key: "Marker".into(), value: PdfObject::Bool(true) }),\n',
            '        PdfMutation::SetTrailerEntry(SetTrailerEntry { key: "Marker".into(), value: PdfObject::Bool(true) }),\n'
            '        PdfMutation::SetSnapshot(SetSnapshot { snapshot: text_document(&[(120.0, 80.0, "Replaced")]) }),\n',
            1,
        )
    ],
    ZIP + "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio": [
        ("text-value = IDENT | TEXT\n", 'text-value = IDENT | TEXT\nbool-value = "true" | "false"\n', 1),
        (
            'set-archive-comment = "set-archive-comment" "comment" "=" text-value\n',
            'set-archive-comment = "set-archive-comment" "comment" "=" text-value "comment-utf8" "=" bool-value\n',
            1,
        ),
    ],
    "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs": [
        (
            "ToolExecutionContract::bounded_first_step(Self::NATIVE_MAXIMUM_RAW_BYTES, 4_096, 1, Self::NATIVE_MAXIMUM_WORK_ITEMS, 7_500)",
            "ToolExecutionContract::bounded_first_step(Self::NATIVE_MAXIMUM_RAW_BYTES, 4_096, 1, Self::NATIVE_MAXIMUM_RAW_BYTES, 7_500)",
            1,
        )
    ],
    ZIP + "✏️editor/🦀️.rs": [('!["nodeId", "value", "revision"].contains(&key.as_str())', '!["nodeId", "value", "revision", "windowId"].contains(&key.as_str())', 1)],
    BMP + "📝️text/📖️.grammar.semio": [('hex = IDENT | TEXT\nset-snapshot = "set-snapshot" "snapshot" "=" hex\n', 'set-snapshot = "set-snapshot" "snapshot" "=" hex\n', 1)],
    GLTF + "🧬️schema/🧬️mutations/📸️snapshot/📸️set/🦀️.rs": [
        ("    pub snapshot: GltfSnapshot,\n", "    pub snapshot: Box<GltfSnapshot>,\n", 1),
        ("vec![GltfMutation::SetSnapshot(Self { snapshot: base.clone() })]", "vec![GltfMutation::SetSnapshot(Self { snapshot: Box::new(base.clone()) })]", 1),
    ],
    GLTF + "✏️editor/🦀️.rs": [("GltfMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: snapshot })", "GltfMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: Box::new(snapshot) })", 1)],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        for rel in SWAPS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        for rel in CREATED:
            if (ROOT / rel).exists():
                (ROOT / rel).unlink()
                print(f"REMOVED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        if callable(hunks):
            try:
                text = hunks(text)
            except ValueError as error:
                print(f"PROBLEM {rel}: {error}")
                problems += 1
        else:
            for old, new, count in hunks:
                found = text.count(old)
                if found != count:
                    print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                    problems += 1
                    continue
                text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.split('/🗿️artifacts/', 1)[-1][:90]}")
    for rel, (base_sha, payload) in SWAPS.items():
        current = hashlib.sha256((ROOT / rel).read_bytes()).hexdigest()
        if current != base_sha:
            print(f"PROBLEM {rel}: {'already applied' if current == hashlib.sha256((PAYLOAD / payload).read_bytes()).hexdigest() else 'base sha256 changed'}")
            problems += 1
        else:
            print(f"{'SWAP' if WRITE else 'DRY-SWAP'} {rel.split('/🗿️artifacts/', 1)[-1][:90]}")
    for rel in CREATED:
        if (ROOT / rel).exists():
            print(f"PROBLEM {rel}: exists")
            problems += 1
        else:
            print(f"{'CREATE' if WRITE else 'DRY-CREATE'} {rel.split('/🗿️artifacts/', 1)[-1][:90]}")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
        for rel, (_, payload) in SWAPS.items():
            (BACKUP / key(rel)).write_bytes((ROOT / rel).read_bytes())
            (ROOT / rel).write_bytes((PAYLOAD / payload).read_bytes())
        for rel, text in CREATED.items():
            (ROOT / rel).parent.mkdir(parents=True, exist_ok=True)
            (ROOT / rel).write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} edited + {len(SWAPS)} swapped + {len(CREATED)} created, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
