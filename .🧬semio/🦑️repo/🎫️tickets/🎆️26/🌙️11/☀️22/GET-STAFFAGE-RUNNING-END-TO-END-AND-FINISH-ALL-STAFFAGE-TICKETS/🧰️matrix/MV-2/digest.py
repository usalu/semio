#!/usr/bin/env python3
"""🧾️ The digest of a plant document without its tags: SHA-256 (first 16 digits) of the canonical JSON (sorted keys, no spaces, UTF-8) of the document with `asset.tags` removed, beside the SHA-256 of the file itself. It tells whether a file still is the proven drawing after another lane rewrote the tags of the whole catalog (2026-11-25 19:09). digest.py <file>…"""
import hashlib, json, sys

for path in sys.argv[1:]:
    raw = open(path, "rb").read()
    doc = json.loads(raw)
    doc["asset"].pop("tags", None)
    canonical = json.dumps(doc, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    print(hashlib.sha256(canonical).hexdigest()[:16], hashlib.sha256(raw).hexdigest()[:16], len(raw), path.split("/")[-2] if path.endswith("🔣️.json") else path.split("/")[-1])
