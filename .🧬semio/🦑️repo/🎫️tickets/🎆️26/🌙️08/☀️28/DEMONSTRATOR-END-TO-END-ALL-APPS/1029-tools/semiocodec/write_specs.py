#!/usr/bin/env python3
"""Writes the committed record specs from a `record_spec_law` run with SEMIO_RECORD_SPEC_WRITE=1 (lines `RECORD-SPEC|<subset>|<json>`).

usage: write_specs.py <log>
"""
import json
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets")
for line in pathlib.Path(sys.argv[1]).read_text(encoding="utf-8", errors="replace").split("\n"):
    if not line.startswith("RECORD-SPEC|"):
        continue
    _, name, payload = line.split("|", 2)
    document = json.loads(payload)
    target = ROOT / name / "🧬️schema" / "📸️snapshot" / "🗺️record-spec" / "🔣️.json"
    existing = json.loads(target.read_text(encoding="utf-8")) if target.exists() else {}
    if "annotations" in existing:
        document["annotations"] = existing["annotations"]
    target.parent.mkdir(parents=True, exist_ok=True)
    ordered = {key: document[key] for key in ("schema", "envelope", "schemaHash", "graph") if key in document}
    if "annotations" in document:
        ordered["annotations"] = document["annotations"]
    target.write_text(json.dumps(ordered, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print("wrote", name, len(document["graph"]), "records")
