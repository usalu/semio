#!/usr/bin/env python3
"""#️⃣️ Re-records the `sha256`/`bytes` of every stdio fixture-manifest file entry whose committed file moved under a schema
evolution (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): each `🔮️oracles/🔣️.json` under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`
is read, every `fixtureManifests[].files[]` entry whose file exists and hashes differently gets the file's own digest and
length, and nothing else changes (missing files are reported, never invented). Idempotent; `--check` lists the stale entries
and exits 1 while any is pending.

usage: python3 🧪️s4-stdio-fixture-digests.py [--check]
"""
from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"


def main() -> int:
    check = "--check" in sys.argv
    listed = subprocess.run(["git", "ls-files", "-z", "--", ARTIFACTS], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    pending, missing = 0, []
    for name in sorted(path for path in listed if path.endswith("/🔮️oracles/🔣️.json") and (ROOT / path).exists()):
        catalog_path = ROOT / name
        source = catalog_path.read_text(encoding="utf-8")
        catalog = json.loads(source)
        changed = False
        for fixture in catalog.get("fixtureManifests", []):
            for entry in fixture.get("files", []):
                target = (catalog_path.parent / entry["path"]).resolve()
                if not target.exists():
                    missing.append(f"{name}#{fixture['id']}: {entry['path']}")
                    continue
                content = target.read_bytes()
                digest = "sha256:" + hashlib.sha256(content).hexdigest()
                if entry.get("sha256") != digest or ("bytes" in entry and entry["bytes"] != len(content)):
                    pending += 1
                    print(f"{'stale' if check else 'refreshed'}: {name.removeprefix(ARTIFACTS)}#{fixture['id']}/{entry['role']}")
                    entry["sha256"] = digest
                    if "bytes" in entry:
                        entry["bytes"] = len(content)
                    changed = True
        if changed and not check:
            indent = 2 if source.startswith('{\n  "') else 1
            catalog_path.write_text(json.dumps(catalog, indent=indent, ensure_ascii=False) + "\n", encoding="utf-8")
    for entry in missing:
        print(f"missing (not touched): {entry}")
    print(f"{pending} fixture digest(s) {'stale' if check else 'refreshed'}, {len(missing)} missing file(s)")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
