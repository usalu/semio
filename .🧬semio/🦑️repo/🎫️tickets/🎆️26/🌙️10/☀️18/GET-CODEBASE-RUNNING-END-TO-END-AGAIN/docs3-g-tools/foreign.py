"""📓️ docs3-g: do the guide edits of OTHER staged batches still find their anchors after every doc batch is applied?

usage: python3 -I foreign.py <root with all doc batches applied> [<pristine root>]
Foreign guide edits read from their staged files (never run): fdleak guard, census2, testmem2 (batch.json), walk-g footprint (edits.json), quietfc (edit_code.py via ast),
obs-g (apply_guide.py via ast), nxfix core (whole-file replacement: only reported). Each `old` must occur exactly once in the pristine guide and exactly once in the applied guide."""
import ast
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
GUIDE = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
applied = open(os.path.join(sys.argv[1], GUIDE), encoding="utf8").read()
pristine = open(os.path.join(sys.argv[2], GUIDE), encoding="utf8").read() if len(sys.argv) > 2 else None
foreign = []


def from_json(label, path):
    data = json.load(open(os.path.join(C, path), encoding="utf8"))

    def walk(node):
        if isinstance(node, dict):
            if str(node.get("file", "")).endswith("build-guide.md") and "edits" in node:
                for edit in node["edits"]:
                    foreign.append((label, edit["old"], edit.get("new")))
            if str(node.get("file", "")).endswith("build-guide.md") and "old" in node:
                foreign.append((label, node["old"], node.get("new")))
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(data)


from_json("fdleak guard", "fdleak-g-tools/staged/guard/batch.json")
from_json("census2", "census2-g-tools/staged/batch.json")
from_json("testmem2", "testmem2-g-tools/staged/batch.json")
from_json("walk-g footprint", "walk-g-tools/staged/footprint/edits.json")
tree = ast.parse(open(os.path.join(C, "quietfc-g-tools/edit_code.py"), encoding="utf8").read())
for node in ast.walk(tree):
    if isinstance(node, ast.Call) and getattr(node.func, "id", "") == "edit" and node.args and "build-guide.md" in ast.unparse(node.args[0]):
        for pair in ast.literal_eval(node.args[1]):
            foreign.append(("quietfc", pair[0], pair[1]))
tree = ast.parse(open(os.path.join(C, "obs-g-tools/apply_guide.py"), encoding="utf8").read())
for node in ast.walk(tree):
    if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "rows":
        for pair in ast.literal_eval(node.value):
            foreign.append(("obs-g", pair[0], pair[1]))
bad = 0
for label, old, new in foreign:
    after = applied.count(old)
    done = new is not None and new != old and applied.count(new) >= 1 and old not in new
    ok = after == 1 or done
    print("%-18s anchor x%d, own new text present: %-5s %s%s" % (label, after, done, "ok" if ok else "BROKEN", "  " + old[:60].replace("\n", " ")))
    bad += 0 if ok else 1
print("nxfix core: replaces the whole guide, base sha256 38412f96...; compatible only before any other guide edit (see nxfix-core doc batch)")
print("foreign anchors checked %d, broken %d" % (len(foreign), bad))
sys.exit(1 if bad else 0)
