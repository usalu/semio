#!/usr/bin/env python3
"""🧷️ Round 2 planner. The round-1 mirror (R1: live + lock-sync closure + round 1) is the base every anchor is proven against.
  plan2.py apply   writes every anchored edit onto the working mirror (R2), starting from the R1 text of each file
  plan2.py cut     proves the anchors once more and writes staged2/plan.json: edits, rewrites (changed files of R1) and new files (from R2)
Anchored edits are declared in plan2_*.py modules (one per concern) through `edit(...)`; files under the seam folder and the listed schema bases are cut from the mirror by difference."""
import hashlib, importlib, json, os, sys
os.environ["PM_ROUND"] = "2"
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths
R1 = os.environ.get("R1", "/private/tmp/pm-seam/r1")
R2 = os.environ.get("R2", "/private/tmp/pm-seam/r2")
GROUPS = ["seam2", "schemas", "storage2", "generation2", "glue", "graph2", "tools2", "policy2", "ratchet2"]
edits = []
def edit(path, id, group, old, new, count=1):
    assert group in GROUPS, group
    edits.append({"path": path, "id": id, "group": group, "old": old, "new": new, "count": count})
def load_specs():
    for name in sorted(n[:-3] for n in os.listdir(paths.HERE) if n.startswith("plan2_") and n.endswith(".py")):
        importlib.import_module(name).declare(edit, paths)
def read(root, rel):
    return open(os.path.join(root, rel), encoding="utf-8", newline="").read()
def proven():
    by_path, problems = {}, []
    for item in edits:
        text = by_path.setdefault(item["path"], {"text": read(R1, item["path"]), "rows": []})
        count = text["text"].count(item["old"])
        if count != item["count"]:
            problems.append("%s [%s] anchor count %d, expected %d" % (item["path"].split("/")[-2] + "/" + item["path"].split("/")[-1], item["id"], count, item["count"]))
            continue
        text["text"] = text["text"].replace(item["old"], item["new"])
        text["rows"].append({"id": item["id"], "group": item["group"], "old": item["old"], "new": item["new"], "counts": [item["count"]]})
    return by_path, problems
SKIP = {"node_modules", ".git", "target", "dist", ".cache"}
def changed_files(edited):
    """Every library file of R2 that differs from R1 and is not produced by an anchored edit, cut by difference."""
    found = []
    library = os.path.join(R2, paths.LIBRARY)
    for current, directories, names in os.walk(library):
        directories[:] = [d for d in directories if d not in SKIP]
        for name in names:
            rel = os.path.relpath(os.path.join(current, name), R2).replace(os.sep, "/")
            if rel in edited or os.path.islink(os.path.join(R2, rel)) or rel.endswith(".pm-seam.tmp"):
                continue
            r1_path = os.path.join(R1, rel)
            if not os.path.exists(r1_path) or read(R1, rel) != read(R2, rel):
                found.append(rel)
    return sorted(found)

def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else ""
    load_specs()
    by_path, problems = proven()
    if problems:
        print("\n".join(problems)); sys.exit(1)
    if mode == "apply":
        for rel, item in by_path.items():
            with open(os.path.join(R2, rel), "w", encoding="utf-8", newline="") as handle:
                handle.write(item["text"])
        print("applied", sum(len(i["rows"]) for i in by_path.values()), "edits in", len(by_path), "files")
        return
    if mode != "cut":
        raise SystemExit(__doc__)
    rewrites, new_files = [], []
    for rel in changed_files(set(by_path)):
        r2 = read(R2, rel)
        r1_path = os.path.join(R1, rel)
        if not os.path.exists(r1_path):
            paths.new_write(rel, r2)
            new_files.append({"path": rel, "group": "ratchet2" if "⚖️ratchet" in rel else "seam2" if rel.startswith(paths.MANAGERS) else "schemas"})
            continue
        r1 = read(R1, rel)
        if r1 == r2:
            continue
        paths.new_write(rel + ".r1", r1)
        paths.new_write(rel + ".r2", r2)
        group = "ratchet2" if "⚖️ratchet" in rel else "seam2" if rel.startswith(paths.MANAGERS) else "schemas"
        rewrites.append({"id": "rewrite-" + hashlib.sha256(rel.encode()).hexdigest()[:8], "group": group, "path": rel, "oldSha256": hashlib.sha256(r1.encode()).hexdigest(), "old": rel + ".r1", "new": rel + ".r2"})
    plan = {"version": 1, "groups": GROUPS, "requires": "round 1 applied", "files": [{"path": p, "edits": i["rows"]} for p, i in by_path.items()], "rewrites": rewrites, "newFiles": new_files}
    json.dump(plan, open(os.path.join(paths.STAGE, "plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("edits", sum(len(i["rows"]) for i in by_path.values()), "files", len(by_path), "rewrites", len(rewrites), "new", len(new_files))
main()
