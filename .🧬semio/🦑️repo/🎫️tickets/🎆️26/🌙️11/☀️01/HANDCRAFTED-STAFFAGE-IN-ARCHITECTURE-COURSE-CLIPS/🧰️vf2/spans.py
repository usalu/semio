import ast, sys, importlib
from pathlib import Path
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only")
p = [x for x in m.energy_clip_sources() if "🧲️physics" in str(x) and "🧲️fundamentals" in str(x)][0]
print(p); src = p.read_text(encoding="utf-8"); man = m.manifest_casts()
tree = ast.parse(src)
rest = ast.Module(body=[n for n in tree.body if not isinstance(n, ast.ClassDef)], type_ignores=[])
for g in [*(n for n in tree.body if isinstance(n, ast.ClassDef)), rest]:
    names = [n for n in m.resolved_casts(src, man, g) if man[n]["view"] == "thermal"]
    print(getattr(g, "name", "module"), {n: man[n].get("thermalSpanK") for n in names})
