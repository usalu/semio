"""Marks every spec id that the reach table routes to an entity check or a member/anchor check as `check` (with its check-id fragment) instead of `design-case`, then rewrites the Rust twin through the shared codegen. Run with python3 -I."""
import importlib.util, json, pathlib, re
ROOT = pathlib.Path("/home/deck/semio")
ART = next(ROOT.glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"))
reach = (ART / "🧬️schema/💡️inferences/⚖️checks/📋️member-evaluation/🧷️entity-cases/📍️reach/🦀️.rs").read_text(encoding="utf-8")
entity = dict(re.findall(r'\("([^"]+)", Route::Entity\("([^"]*)"\)\)', reach))
check = dict(re.findall(r'\("([^"]+)", Route::Check\("([^"]*)"\)\)', reach))
path = ART / "🧭️spec-trace/🔣️.json"
doc = json.loads(path.read_text(encoding="utf-8"))
moved = 0
for item in doc["items"]:
    i = item["id"]
    if i in entity:
        item["entity"], item["realisation"] = entity[i], {"kind": "check", "checks": [f"en1992.{i.lower()}."]}
    elif i in check:
        item["entity"] = "anchors[]" if check[i].startswith("en1992-4.") else "members[]"
        item["realisation"] = {"kind": "check", "checks": [check[i]]}
    else:
        item["entity"], item["realisation"] = "cases", {"kind": "design-case", "entry": i}
        continue
    moved += 1
spec = importlib.util.spec_from_file_location("g", ROOT / "✏️s/🔌️plugins/📕️norm/⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py")
g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)
problems = g.verify("en1992", doc)
g.write("en1992", doc)
print(moved, "ids realised by checks;", len(doc["items"]) - moved, "by design cases; problems:", problems)
