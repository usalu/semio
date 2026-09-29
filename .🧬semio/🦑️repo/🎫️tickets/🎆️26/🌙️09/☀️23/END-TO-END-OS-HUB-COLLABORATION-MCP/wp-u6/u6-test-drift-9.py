#!/usr/bin/env python3
"""🧪️ U6 set A7c — zip OPC duplicate-member law (test-only, rule 22).

Measured (live probe 11:2x, `[DEBUG]` line removed again): a package whose archive repeats `word/document.xml` is refused by the
ZIP layer itself — `decode_opc` → `decode_zip` → `validate_zip_snapshot_serialization` "ZIP member names must be nonempty and
unique" (the 09-28 codec keys members by name and cannot round-trip a repeated one), surfaced as `OpcError::Zip`, before the
OPC layer's own member scan runs. The law keeps its point — a duplicate member is refused before interpretation — and names
the layer and the invariant that refuse it.

Usage: u6-test-drift-9.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-9") / hashlib.sha256(str(ROOT).encode()).hexdigest()[:12]
SETS = {
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧪️tests/🔬️unit/🦀️.rs": [
        (
            '        assert!(matches!(decode_opc(&bytes), Err(OpcError::Malformed(_))), "path={path}");\n    }\n}\n',
            '        assert!(matches!(decode_opc(&bytes), Err(OpcError::Zip(detail)) if detail.contains("names must be nonempty and unique")), "path={path}");\n    }\n}\n',
            1,
        )
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
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
