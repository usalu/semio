"""One-shot writer of compact 1:1 `replace` transition documents for the E2a ventilation clips (deleted when the batch closes).

Row: (scene.name, intent, source entity, target entity, duration, copy, easing). Everything else is the shared 1:1 replace shape.
"""

import json
from pathlib import Path


def row(prefix, spec):
    ident, intent, source, target, duration, copy, *rest = spec
    easing, relation = (rest + ["smooth", "one-to-one"][len(rest):])[:2]
    item = {
        "id": f"{prefix}.{ident}",
        "intent": intent,
        "source": {"entity": f"{prefix}.{source}"},
        "target": {"entity": f"{prefix}.{target}"},
        "relation": relation,
        "correspondence": {"kind": "order"},
        "recipe": "replace",
    }
    if copy:
        item["copy"] = True
    item["timing"] = {"duration": duration, "easing": easing}
    return item


def write(clip_dir, prefix, rows):
    lines = []
    for spec in rows:
        lines.append("    " + json.dumps(row(prefix, spec), ensure_ascii=False))
    text = '{\n  "$schema": "../../../../../🔀️morph/🧬️schema/🔣️.schema.json",\n  "version": 1,\n  "transitions": [\n' + ",\n".join(lines) + "\n  ]\n}\n"
    target = Path(clip_dir) / "🔀️transitions" / "🔣️.json"
    target.parent.mkdir(exist_ok=True)
    target.write_bytes(text.encode("utf-8"))
    print(f"wrote {len(rows)} transitions -> {target}")
