"""🩹 Remove only persistently invalid generated SVG cache entries after link isolation."""

import json
from pathlib import Path
import time
from xml.etree import ElementTree

generated = (Path(__file__).resolve().parent / "🗑️generated").resolve()
paths = json.loads((generated / "cache-isolation.json").read_text(encoding="utf-8"))["unresolved"]
removed = []
for value in paths:
    svg = Path(value)
    if not svg.resolve().is_relative_to(generated) or svg.suffix != ".svg":
        raise ValueError("Invalid generated SVG path")
    invalid = True
    for _ in range(3):
        try:
            ElementTree.parse(svg)
            invalid = False
            break
        except FileNotFoundError:
            invalid = False
            break
        except ElementTree.ParseError:
            time.sleep(0.1)
    if invalid:
        svg.unlink(missing_ok=True)
        removed.append(str(svg))
(generated / "cache-repair.json").write_text(json.dumps({"removed_invalid_generated_svgs": removed}, indent=2) + "\n", encoding="utf-8")
print(f"[DEBUG] Removed {len(removed)} persistently invalid generated SVG entries", flush=True)
