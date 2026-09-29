#!/usr/bin/env python3
"""📜️ U6 window-3 set C8 (T3-late stdio, docx, interim — the full paged DOCX route is tracked in 📓️wp-u6.md): `set-page`
refused EVERY real document.

Measured 04:13 (`.🧬semio/🌐hub/s14-u6-logs/a6-scratch.txt`): the retained `set-page` work step and `prepare_set_run_text` both ran
`snapshot_is_admitted` — the WHOLE snapshot measured against the bounded owner's 2 KiB / 128-item envelope — so even
`build_minimal_docx` was refused `stdio.docx.set-page.paged-owner-required` before the route reached its store preparation
(`registered_canonical_page_edit_publishes_once…`, 3 `set_page_*` laws). The route now validates what it edits (address + text,
`set_run_text_is_admitted` + `prepare_addressed_xml_mutation`); the whole-snapshot admission stays only where the whole document
is copied — the one-item store preparation — sized from the one-item store budget
(`ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` less one turn; items: one per 16 owned bytes), above which it refuses typed as before;
the language-neutral admission fixture + schema move with it (`refusedTextBytes` above one owner).

Usage: u6-docx-set-page-owner.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/docx-set-page-owner")
EDITOR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/"
SETS = {
    EDITOR + "🦀️.rs": [
        (
            """        if !self.validated {
            preparation::snapshot_is_admitted(input.snapshot).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.paged-owner-required"), message))?;
            let Some(_) = build_set_page_mutation(input.snapshot, address, text)? else {
""",
            """        if !self.validated {
            let Some(_) = build_set_page_mutation(input.snapshot, address, text)? else {
""",
            1,
        )
    ],
    EDITOR + "📬️preparation/🧫️fixtures/🧵️admission/🔣️.json": [
        (
            """  "ownerBytes": 2048,
  "ownerItems": 128,
  "ownerDepth": 32,
  "turnBytes": 4096,
  "refusedTextBytes": 4096,
""",
            """  "ownerBytes": 1044480,
  "ownerItems": 65280,
  "ownerDepth": 32,
  "turnBytes": 4096,
  "refusedTextBytes": 1048576,
""",
            1,
        )
    ],
    EDITOR + "📬️preparation/🧬️schema/🔣️.json": [
        (
            """    "ownerBytes": { "type": "integer", "const": 2048 },
    "ownerItems": { "type": "integer", "const": 128 },
    "ownerDepth": { "type": "integer", "const": 32 },
    "turnBytes": { "type": "integer", "const": 4096 },
    "refusedTextBytes": { "type": "integer", "minimum": 2049 },
""",
            """    "ownerBytes": { "type": "integer", "const": 1044480, "description": "The one-item store budget (1 MiB) less one turn: the largest document or mutation owner the one-item preparation copies." },
    "ownerItems": { "type": "integer", "const": 65280, "description": "One measured owner item per 16 owned bytes." },
    "ownerDepth": { "type": "integer", "const": 32 },
    "turnBytes": { "type": "integer", "const": 4096 },
    "refusedTextBytes": { "type": "integer", "minimum": 1044481, "description": "Run text larger than one owner, refused typed without publication." },
""",
            1,
        )
    ],
    EDITOR + "📬️preparation/🦀️.rs": [
        (
            """const OWNER_BYTES: usize = 2_048;
const OWNER_ITEMS: usize = 128;
""",
            """/// 📏️ The one-item store preparation copies the whole document in one step, so an owner (the document, or one mutation)
/// plus one turn fits the one-item store budget exactly — one measured item per 16 owned bytes — and larger owners refuse
/// typed (`paged-owner-required`) before the store ever sees an inadmissible footprint.
const OWNER_BYTES: usize = app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES - TURN_BYTES;
const OWNER_ITEMS: usize = OWNER_BYTES / 16;
""",
            1,
        ),
        (
            """pub(super) fn snapshot_is_admitted(snapshot: &DocxSnapshot) -> Result<(), String> {
    measure_snapshot(snapshot).map(|_| ())
}

""",
            "",
            1,
        ),
        (
            """pub(crate) fn prepare_set_run_text(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, String> {
    snapshot_is_admitted(snapshot)?;
    set_run_text_is_admitted(address, text)?;
""",
            """pub(crate) fn prepare_set_run_text(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, String> {
    set_run_text_is_admitted(address, text)?;
""",
            1,
        ),
    ],
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
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new and new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/✏️editor/', 1)[-1]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
