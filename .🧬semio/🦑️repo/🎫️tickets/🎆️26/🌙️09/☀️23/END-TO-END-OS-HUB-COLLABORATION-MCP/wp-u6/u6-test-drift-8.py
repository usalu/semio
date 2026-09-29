#!/usr/bin/env python3
"""🧪️ U6 set A7b — the two zip laws left red after A7 (test-only, rule 22).

* `serialization_validation_refuses_stale_or_unencodable_header_state`: its second half proves a stale local Unicode-path marker
  (a derived extra must persist an EMPTY payload) is refused `Malformed`; with A7's unencodable `λ` still in place the central
  header refused first (`Utf8`). The stale half names a CP437-encodable member again (`δ`, CP437 0xEB), as the law intended.
* `encrypted_wire_archive_composes_to_clean_logical_output` → `encrypted_wire_archive_is_refused_instead_of_composed_from_ciphertext`:
  since the 09-28 codec rewrite the strict decoder refuses encrypted/masked headers (it keeps header state explicitly and cannot
  regenerate them; a real encrypted member's payload is ciphertext, so "clearing the bit" composed garbage). ISO 21320
  composition of such an archive is therefore a typed refusal naming the encryption; the subset validator law keeps flagging
  `CODE_ENCRYPTED` on the raw wire.

Usage: u6-test-drift-8.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-8") / hashlib.sha256(str(ROOT).encode()).hexdigest()[:12]
ZIP = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/"
SETS = {
    ZIP + "🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs": [
        (
            "    entry.metadata.local.extra_fields.push(ZipExtraField { id: EXTRA_UNICODE_PATH, data: vec![1] });\n",
            '    entry.name = "δ.txt".into();\n    entry.metadata.local.extra_fields.push(ZipExtraField { id: EXTRA_UNICODE_PATH, data: vec![1] });\n',
            1,
        )
    ],
    ZIP + "🌐️iso21320/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs": [
        (
            "use crate::standards::v2_0::subsets::iso21320::schema::{CODE_ENCRYPTED, FLAG_ENCRYPTED, check_iso21320_wire_conformance};\n",
            "use crate::standards::v2_0::subsets::iso21320::schema::{CODE_ENCRYPTED, FLAG_ENCRYPTED};\n",
            1,
        ),
        (
            "    async fn encrypted_wire_archive_composes_to_clean_logical_output() {\n"
            "        let raw = raw_zip_with_flags(FLAG_ENCRYPTED, 20);\n"
            "        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&raw) }];\n"
            '        let composed = ZipIso21320ComposerComposition::compose(&sources).expect("decode+canonicalize must clear forbidden wire bits");\n'
            '        let rematerialized = crate::standards::v2_0::subsets::base::io::encode_zip(&composed.snapshot).expect("encode canonical logical archive");\n'
            "        assert!(check_iso21320_wire_conformance(&rematerialized).iter().all(|d| d.code.0 != CODE_ENCRYPTED));\n"
            "    }\n",
            "    async fn encrypted_wire_archive_is_refused_instead_of_composed_from_ciphertext() {\n"
            "        let raw = raw_zip_with_flags(FLAG_ENCRYPTED, 20);\n"
            "        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&raw) }];\n"
            '        let Err(refusal) = ZipIso21320ComposerComposition::compose(&sources) else { panic!("an encrypted member cannot be regenerated, so it never composes") };\n'
            '        assert!(refusal.diagnostics.iter().any(|diagnostic| diagnostic.severity == Severity::Error && diagnostic.message.contains("encrypted")), "{refusal:?}");\n'
            "    }\n",
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
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.split('/🪆️subsets/', 1)[-1][:90]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
