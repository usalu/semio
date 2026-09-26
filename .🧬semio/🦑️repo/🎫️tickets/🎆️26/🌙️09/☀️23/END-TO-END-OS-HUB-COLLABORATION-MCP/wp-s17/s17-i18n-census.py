#!/usr/bin/env python3
"""🗣️ S17 i18n census: every English-only user-facing string an extension contributes through its topic payload
(`label`/`name`/`summary`/`title`/`description`/`abbreviation`, recursively, JSON-in-JSON included), with the source
line(s) that author it. usage: python3 s17-i18n-census.py <out.json>
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGINS = ROOT / "✏️s" / "🔌️plugins"
KEYS = {"label", "name", "summary", "title", "description", "abbreviation"}


def walk(value, key_path, out):
    if isinstance(value, dict):
        for key, child in value.items():
            path = f"{key_path}.{key}"
            if isinstance(child, str) and child.strip().startswith(("{", "[")):
                try:
                    walk(json.loads(child), path, out)
                    continue
                except json.JSONDecodeError:
                    pass
            if key in KEYS and isinstance(child, str) and child.strip():
                out.setdefault(child, set()).add(re.sub(r"\[\d+\]", "[]", path))
            else:
                walk(child, path, out)
    elif isinstance(value, list):
        for child in value:
            walk(child, f"{key_path}[]", out)


def main():
    rows = []
    for projection in sorted(PLUGINS.glob("*/🧩️extensions/*/🔣️.json")):
        descriptor = json.loads(projection.read_text())
        if "manifest" not in descriptor:
            continue
        ext_dir = projection.parent
        sources = [p for p in ext_dir.rglob("*.rs") if "/🧪️tests/" not in str(p) and "/target/" not in str(p)]
        texts = {p: p.read_text() for p in sources}
        found = {}
        for contribution in descriptor["manifest"].get("topicContributions", []):
            walk(contribution.get("payload", {}), contribution["topic"], found)
        for text, paths in sorted(found.items()):
            literal = json.dumps(text, ensure_ascii=False)
            locations = [f"{p.relative_to(ROOT)}:{i + 1}" for p, body in texts.items() for i, line in enumerate(body.split("\n")) if literal in line]
            rows.append({"parent": ext_dir.parent.parent.name, "extension": ext_dir.name, "en": text, "fields": sorted(paths), "sources": locations})
    Path(sys.argv[1]).write_text(json.dumps(rows, indent=1, ensure_ascii=False))
    by_family = {}
    for row in rows:
        by_family.setdefault(row["parent"], [0, 0])
        by_family[row["parent"]][0] += 1
        by_family[row["parent"]][1] += 0 if row["sources"] else 1
    total_unlocated = sum(v[1] for v in by_family.values())
    for family, (count, unlocated) in by_family.items():
        print(f"{family}: {count} strings, {unlocated} without a literal source line (built by format!/concat)")
    print(f"total {len(rows)} (unlocated {total_unlocated}), distinct {len({row['en'] for row in rows})}")


if __name__ == "__main__":
    main()
