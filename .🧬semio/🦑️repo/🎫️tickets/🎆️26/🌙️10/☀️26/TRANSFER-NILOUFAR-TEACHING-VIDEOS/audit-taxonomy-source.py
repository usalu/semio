"""🔬 Compare relocated authored AST bodies and immutable assets against originals."""
import ast
import difflib
import hashlib
import json
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "AGENTS.md").is_file())
SOURCE = Path("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial")
PLAN = json.loads((TICKET / "taxonomy-paths.cases.json").read_text(encoding="utf-8-sig"))
TAX_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"


def definitions(tree):
    """🧩 Extract authored callable bodies, preserving enclosing class identity."""
    result = {}
    def visit(nodes, prefix=""):
        for node in nodes:
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                result[prefix + node.name] = ast.dump(node, include_attributes=False)
            elif isinstance(node, ast.ClassDef):
                visit(node.body, prefix + node.name + ".")
    visit(tree.body)
    return result


rows = []
assets = []
for move in PLAN["moves"]:
    old = move["old"].removeprefix("🎓️teaching/")
    if not move["new"]:
        continue
    original = SOURCE / old
    current = ROOT / move["new"]
    if old.startswith("energy/") and __import__("re").search(r"(?:^|/)scene_\d+\.py$", old) or old == "intro/intro_scene.py":
        before = definitions(ast.parse(original.read_text(encoding="utf-8-sig")))
        after = definitions(ast.parse(current.read_text(encoding="utf-8-sig")))
        changed = [name for name in before.keys() & after.keys() if before[name] != after[name]]
        rows.append({"old": old, "current": move["new"], "definitions": len(before), "removed": sorted(before.keys() - after.keys()), "added": sorted(after.keys() - before.keys()), "changed": sorted(changed)})
    if old.endswith(".pdf") or Path(old).name in ("vo_timing.json", "vo_trace.json"):
        a, b = original.read_bytes(), current.read_bytes()
        assets.append({"old": old, "same": a == b, "sha256": hashlib.sha256(b).hexdigest()})
entries = (TICKET / "🗑️generated/taxonomy-baseline/index-entries.z").read_bytes().decode("utf-8").split("\0")
entry = next(line for line in entries if line.partition("\t")[2] == TAX_PATH)
blob = entry.partition("\t")[0].split()[1]
baseline = json.loads(subprocess.check_output(["git", "cat-file", "blob", blob], cwd=ROOT))
current = json.loads((ROOT / TAX_PATH).read_text(encoding="utf-8"))
changes = []

def compare(a, b, path=""):
    """🛡️ Require preservation of baseline fields except additive kind rosters."""
    if isinstance(a, dict) and isinstance(b, dict):
        for key in a:
            if key not in b:
                changes.append({"path": path + "/" + key, "change": "removed"})
            else:
                compare(a[key], b[key], path + "/" + key)
        for key in b.keys() - a.keys():
            changes.append({"path": path + "/" + key, "change": "added"})
    elif a != b:
        if isinstance(a, list) and isinstance(b, list) and all(item in b for item in a):
            changes.append({"path": path, "change": "list-additions", "added": [item for item in b if item not in a]})
        else:
            changes.append({"path": path, "change": "modified"})
compare(baseline, current)
result = {"scenes": rows, "assets": assets, "registry": changes, "baseline_blob": blob}
output = TICKET / "🗑️generated/taxonomy-source-comparison.json"
output.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
print(json.dumps({"scenes": len(rows), "definitions": sum(r["definitions"] for r in rows), "body_changes": [r for r in rows if r["changed"] or r["removed"] or r["added"]], "assets": len(assets), "asset_changes": [r["old"] for r in assets if not r["same"]], "registry_changes": changes}, ensure_ascii=True))
