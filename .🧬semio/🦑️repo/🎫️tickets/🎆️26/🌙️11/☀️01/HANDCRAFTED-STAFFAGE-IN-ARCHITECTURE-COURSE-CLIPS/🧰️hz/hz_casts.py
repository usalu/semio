"""🔎 Cast-id audit: manifest casts versus string literals used by architecture clip scripts."""

import json
import re
import sys
from pathlib import Path

ARCH = Path("C:/git/semio/🎓️teaching/🏛️architecture")
VIS = ARCH / "⚡️energy/🏗️authoring/🎨️visuals"
MANIFEST = VIS / "🧍️staffage/🔣️.json"
OUT = Path(sys.argv[1])

manifest = set(json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"])
prefixes = sorted({k.split("-")[0] for k in manifest})
literal = re.compile(r'"([a-z][a-z0-9]*(?:-[a-z0-9]+)+)"')
clips = []
for p in ARCH.rglob("🐍️.py"):
    parts = p.parts
    if "🎬️clip" not in parts or ".venv" in parts or "🧪️tests" in parts or "__pycache__" in parts:
        continue
    clips.append(p)
used = {}
for p in sorted(clips):
    for ln, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
        for m in literal.finditer(line):
            used.setdefault(m.group(1), []).append(f"{p.relative_to(ARCH).as_posix()}:{ln}")

in_manifest_used = {k: v for k, v in used.items() if k in manifest}
missing = {k: v for k, v in used.items() if k not in manifest and k.split("-")[0] in prefixes}
unused = sorted(manifest - set(used))
vis_used = set()
for line in VIS.joinpath("🐍️.py").read_text(encoding="utf-8").splitlines():
    for m in literal.finditer(line):
        vis_used.add(m.group(1))
lines = [f"# clip files scanned: {len(clips)}", f"# manifest casts: {len(manifest)}", f"# manifest casts used literally by clips: {len(in_manifest_used)}", ""]
lines.append("## Literal cast-like ids in clips but missing from manifest")
for k in sorted(missing):
    lines.append(f"- {k}: {missing[k][:4]}")
lines.append("")
lines.append("## Manifest casts no clip literal names (value = also named in visuals module 🐍️.py)")
for k in unused:
    lines.append(f"- {k} (visuals: {'yes' if k in vis_used else 'no'})")
lines.append("")
lines.append("## Literal cast-like ids in clips that are not in manifest but match a prefix (already in list above)")
OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
print("\n".join(lines))
