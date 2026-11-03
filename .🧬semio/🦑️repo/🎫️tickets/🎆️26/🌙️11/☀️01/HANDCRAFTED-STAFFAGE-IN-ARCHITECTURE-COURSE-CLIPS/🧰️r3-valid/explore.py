import json
import re
import sys
from collections import Counter
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
A = Path("C:/git/semio/🎓️teaching/🏛️architecture")
S = A / "🧍️staffage"
manifest = json.loads((A / "⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
assets = {}
for file in (S / "🗂️catalog").glob("*/*/🔣️.json"):
    assets.setdefault(file.parent.name, []).append(file)
print("assets", len(assets), "ambiguous", [k for k, v in assets.items() if len(v) > 1])
doc = lambda slug: json.loads(assets[slug][0].read_text(encoding="utf-8"))["asset"]
keys = Counter()
problems = []
for cid, cast in manifest.items():
    a = doc(cast["asset"]) if cast["asset"] in assets else None
    if a is None:
        problems.append((cid, "asset"))
        continue
    rep = a["representations"].get(str(cast.get("scale")))
    if not rep or rep["status"] != "authored":
        problems.append((cid, "scale", cast.get("scale"), rep and rep["status"]))
        continue
    if not rep["geometry"][cast["viewpoint"]]:
        problems.append((cid, "geometry", cast["viewpoint"]))
    if "pose" in cast:
        p = [x for x in a["poses"] if x["id"] == cast["pose"]]
        if not p:
            problems.append((cid, "pose", cast["pose"]))
        elif p[0].get("key"):
            problems.append((cid, "pose-key"))
    if "clip" in cast and cast["clip"] not in [c["id"] for c in a["clips"]]:
        problems.append((cid, "clip", cast["clip"]))
    if cast["view"] in ("forces", "moments"):
        keys[("structure", bool(a["structure"]["supports"]))] += 1
    if cast["view"] == "thermal":
        keys[("regions", len(a["thermal"]["regions"]) > 0)] += 1
print(problems)
print(keys)
literal = re.compile(r"""\b(staffage|fitted_staffage|staffage_motion|staffage_height|staffage_width|staffage_parts|staffage_provenance|_staffage|_fitted_staffage|_staffage_motion|_staffage_parts)\(\s*["']([^"']+)["']""")
used = Counter()
broken = []
files = [p for p in A.rglob("🐍️.py") if "🎬️clip" in p.parts and not {"🧪️tests", ".venv", "node_modules", "__pycache__"} & set(p.parts)]
print("clip files", len(files))
for p in files:
    text = p.read_text(encoding="utf-8")
    for m in literal.finditer(text):
        name = m.group(2)
        used[name] += 1
        if name not in manifest and not any(c.startswith(name) for c in manifest):
            broken.append((str(p.relative_to(A)), name))
    for name in manifest:
        if f'"{name}"' in text or f"'{name}'" in text:
            used[name] += 0.001
print("broken", broken[:20], len(broken))
print("orphans", [c for c in manifest if c not in used])
prefixes = set()
for p in files:
    text = p.read_text(encoding="utf-8")
    for m in re.finditer(r"""["']([a-z0-9-]+-)["']""", text):
        prefixes.add(m.group(1))
orph = [c for c in manifest if c not in used and not any(c.startswith(x) for x in prefixes)]
print("orphans after prefixes", orph)
