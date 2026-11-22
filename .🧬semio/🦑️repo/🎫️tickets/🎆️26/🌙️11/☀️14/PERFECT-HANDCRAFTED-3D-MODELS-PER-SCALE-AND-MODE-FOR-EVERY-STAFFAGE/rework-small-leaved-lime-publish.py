"""📦 Small-leaved lime author: writes the validated candidate document into the catalog file (members of `asset` in the style of `volumes-insert.py`, whose functions are taken without running it) and its `volumes` into the ticket source; refuses when the catalog file changed since the candidate was built."""
import ast
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
tree = ast.parse(open(os.path.join(HERE, "volumes-insert.py")).read())
INSERT = {}
exec(compile(ast.Module([n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom, ast.FunctionDef))], []), "volumes-insert.py", "exec"), INSERT)
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 7))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation", "small-leaved-lime", "🔣️.json")
OUT = os.path.join(HERE, "🗑️generated", "rework-small-leaved-lime", "small-leaved-lime")
candidate = json.load(open(os.path.join(OUT, "🔣️.json")))
if os.path.getmtime(CATALOG) > os.path.getmtime(os.path.join(OUT, "pre.json")) and "--again" not in sys.argv:
    raise SystemExit("catalog file is newer than the candidate: rebuild first")
asset = candidate["asset"]
rows = [" " + json.dumps(key, ensure_ascii=False) + ": " + json.dumps(value, ensure_ascii=False) for key, value in candidate.items() if key != "asset"]
members = ["  " + json.dumps(key, ensure_ascii=False) + ": " + INSERT["dump"](value, 1) for key, value in asset.items()]
text = "{\n" + ",\n".join(rows) + ",\n \"asset\": {\n" + ",\n".join(members) + "\n }\n}\n"
if json.loads(text) != candidate:
    raise SystemExit("round trip differs")
with open(CATALOG, "w") as f:
    f.write(text)
source = os.path.join(HERE, "volumes", "vegetation", "small-leaved-lime.json")
with open(source, "w") as f:
    json.dump({"volumes": asset["volumes"]}, f, ensure_ascii=False, separators=(",", ":"))
print(f"published {len(text)} bytes -> {CATALOG}")
