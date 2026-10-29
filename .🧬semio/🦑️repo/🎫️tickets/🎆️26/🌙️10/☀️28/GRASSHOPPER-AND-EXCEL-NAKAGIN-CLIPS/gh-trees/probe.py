import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
d = m.canvas("trees")
s = d.__class__ and m.evaluate(d)
print("[DEBUG] objects", list(d.objects))
for oid in d.objects:
    spec = d.spec(oid)
    for out in spec.outputs:
        try:
            t = s.tree(oid, out.name)
        except Exception as e:
            print("[DEBUG]", oid, out.name, "ERR", e)
            continue
        if hasattr(t, "stats"):
            st = t.stats()
            print("[DEBUG]", oid, spec.name, out.name, st["path_count"], st["item_count"], str(st["counts"])[:60], list(t.paths())[:2])
    print("[DEBUG]", oid, "state", s.state(oid), s.msgs(oid))
for oid in ("cp1", "cp2"):
    t = s.tree(oid, "Preview") if False else s.tree(oid)
    tags = {}
    for p in t.paths():
        for b in t.branch(p):
            mat = b.tag.get("material") if hasattr(b, "tag") else None
            tags[mat] = tags.get(mat, 0) + 1
    print("[DEBUG]", oid, tags)
print("[DEBUG] views", {k: v["objects"] for k, v in d.views.items()})
sr = s.tree("src")
b = sr.branch((0, 1, 4, 2))
print("[DEBUG] item", type(b[1]), b[1].tag)
print("[DEBUG] panel", m.panel_rows(s.tree("pt"), paths=True)[:6])
print("[DEBUG] pm", list(s.tree("pm", "Tree").counts())[:3] if hasattr(s.tree("pm", "Tree"), "counts") else None)
