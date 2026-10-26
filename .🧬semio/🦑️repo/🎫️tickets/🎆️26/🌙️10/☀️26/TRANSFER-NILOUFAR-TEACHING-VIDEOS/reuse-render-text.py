"""🔤 Reuse complete SVG caches through private copies without overwriting active files."""

import json
import os
from pathlib import Path
import tempfile
from xml.etree import ElementTree

generated = Path(__file__).resolve().parent / "🗑️generated"
results = []
for original, replacement in [*((3, target) for target in (8, 9, 10, 11, 12, 22)),
                              *((4, target) for target in (13, 14, 15)),
                              *((5, target) for target in (16, 17, 18, 19, 20, 21, 23))]:
    source = generated / "videos" / str(original) / "t"
    target = generated / "pre" / str(replacement) / "t"
    target.mkdir(parents=True, exist_ok=True)
    linked = existing = incomplete = 0
    for svg in source.glob("*.svg"):
        if (target / svg.name).exists():
            existing += 1
            continue
        try:
            data = svg.read_bytes()
            ElementTree.fromstring(data)
        except (ElementTree.ParseError, OSError):
            incomplete += 1
            continue
        with tempfile.NamedTemporaryFile(dir=target, suffix=".cache", delete=False) as cache:
            cache.write(data)
            temporary = Path(cache.name)
        try:
            try:
                os.link(temporary, target / svg.name)
                linked += 1
            except FileExistsError:
                existing += 1
        finally:
            temporary.unlink()
    results.append({"original": original, "replacement": replacement, "linked": linked, "existing": existing, "incomplete": incomplete})
    print("[DEBUG]", results[-1], flush=True)
(generated / "text-cache-reuse.json").write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
