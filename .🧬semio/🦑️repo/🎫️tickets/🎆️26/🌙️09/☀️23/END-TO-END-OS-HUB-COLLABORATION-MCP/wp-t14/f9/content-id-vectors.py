#!/usr/bin/env python3
"""🆔️ Writes `content-id-vectors.json`: `store::content_id` expectations computed by Python's `hashlib` (the third-party
SHA-256 oracle) — `<prefix>-` + the first 16 hex digits of SHA-256 over the UTF-8 text."""
import hashlib
import json
from pathlib import Path

ROWS = [
    ("catalog", "[]"),
    ("note-text", "block-1\u001f[{\"text\":\"Hallo\"}]"),
    ("document", "{\"schema\":\"s.stdio.semio.document\",\"text\":\"ä€😀\"}"),
    ("x", ""),
    ("raster-asset", "image/png\u001f\u0089PNG"),
    ("shape-model", "{\"nodes\":[" + ",".join(f"{{\"id\":\"n{index}\"}}" for index in range(12)) + "]}"),
    ("remodeling-mesh-io", "chunk-a\u001fchunk-b\u001fchunk-c"),
]
vectors = [{"prefix": prefix, "text": text, "id": f"{prefix}-{hashlib.sha256(text.encode('utf-8')).hexdigest()[:16]}"} for prefix, text in ROWS]
Path(__file__).with_suffix(".json").write_text(json.dumps({"_comment": "🆔️ `store::content_id` vectors computed by Python's hashlib (third-party SHA-256).", "vectors": vectors}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("\n".join(row["id"] for row in vectors))
