#!/usr/bin/env python3
"""🧮️ Refreshes the `sha256`/`bytes` of every `fixtureManifests[].files[]` entry of the given `🔮️oracles/🔣️.json` documents
from the committed file its `path` names, editing only those two lines so the document's formatting and every peer's
concurrent edit elsewhere survive. Usage: `python3 🧪️w3-stdio-refresh-fixture-digests.py <oracles.json>...`."""

import hashlib
import json
import os
import re
import sys


def refresh(document_path: str) -> int:
    base = os.path.dirname(document_path)
    with open(document_path, encoding="utf-8") as handle:
        lines = handle.read().split("\n")
    changed = 0
    for index, line in enumerate(lines):
        match = re.match(r'^(\s*)"path": "(.+)",$', line)
        if match is None:
            continue
        target = os.path.normpath(os.path.join(base, match.group(2)))
        if not os.path.isfile(target):
            continue
        payload = open(target, "rb").read()
        digest, size = f"sha256:{hashlib.sha256(payload).hexdigest()}", len(payload)
        for offset in range(1, 4):
            probe = lines[index + offset]
            sha = re.match(r'^(\s*)"sha256": "sha256:[0-9a-f]{64}"(,?)$', probe)
            if sha is not None:
                replacement = f'{sha.group(1)}"sha256": "{digest}"{sha.group(2)}'
                changed += replacement != probe
                lines[index + offset] = replacement
            count = re.match(r'^(\s*)"bytes": \d+(,?)$', probe)
            if count is not None:
                replacement = f'{count.group(1)}"bytes": {size}{count.group(2)}'
                changed += replacement != probe
                lines[index + offset] = replacement
    text = "\n".join(lines)
    json.loads(text)
    with open(document_path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return changed


if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"[w3-stdio] {refresh(path)} digest line(s) refreshed in {path}")
