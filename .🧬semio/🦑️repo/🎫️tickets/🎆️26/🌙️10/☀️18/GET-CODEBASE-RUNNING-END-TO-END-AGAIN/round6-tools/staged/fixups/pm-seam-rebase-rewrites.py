#!/usr/bin/env python3
"""Re-bases the whole-file rewrites of one pm-seam round on the text a peer changed since staging: for every rewrite whose file is neither the recorded old text nor the new text, the change old -> new is merged (git merge-file, three-way) into the CURRENT text, the current text becomes the rewrite's old text and the merged text its new text. A conflict stops the step (nothing is written).
usage: pm-seam-rebase-rewrites.py <pm-seam copy> <root> <stage> (stage = staged | staged2 | staged3)"""
import hashlib, json, os, subprocess, sys, tempfile

COPY, ROOT, STAGE = sys.argv[1:4]
os.environ["PM_ROUND"] = {"staged": "1", "staged2": "2", "staged3": "3"}[STAGE]
sys.path.insert(0, COPY)
sys.path.insert(0, os.path.dirname(COPY))
import paths  # noqa: E402
from winpath import resolve, place  # noqa: E402

def diff(a, b, path=()):
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        for key in sorted(a.keys() | b.keys()):
            if key not in a:
                out.append(("set", path + (key,), None, b[key]))
            elif key not in b:
                out.append(("del", path + (key,), a[key], None))
            else:
                out += diff(a[key], b[key], path + (key,))
    elif isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        for index, (x, y) in enumerate(zip(a, b)):
            out += diff(x, y, path + (index,))
    elif a != b:
        out.append(("set", path, a, b))
    return out


def merge_json(old, new, current):
    """Structural three-way merge: every change old -> new is applied to current unless current already holds it; a path both sides changed differently is a conflict."""
    result = json.loads(json.dumps(current))
    conflicts = []
    for kind, path, before, after in diff(old, new):
        node = result
        for step in path[:-1]:
            node = node[step]
        last = path[-1]
        have = node[last] if (isinstance(node, dict) and last in node) or (isinstance(node, list) and last < len(node)) else None
        if kind == "del":
            if isinstance(node, dict) and last in node:
                if have == before:
                    del node[last]
                elif have != after:
                    conflicts.append(path)
        elif have == after:
            continue
        elif have == before or have is None:
            node[last] = after
        else:
            conflicts.append(path)
    return result, conflicts


sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
plan_path = os.path.join(COPY, STAGE, "plan.json")
plan = json.load(open(plan_path, encoding="utf-8"))
changed = 0
for item in plan.get("rewrites", []):
    target = os.path.join(ROOT, item["path"])
    if not os.path.isfile(target):
        continue
    current = open(target, encoding="utf-8", newline="").read()
    old = open(resolve(paths.NEW, item["old"]), encoding="utf-8", newline="").read()
    new = open(resolve(paths.NEW, item["new"]), encoding="utf-8", newline="").read()
    if sha(current) in (item["oldSha256"], sha(new)):
        continue
    if item["path"].endswith(".json"):
        merged, conflicts = merge_json(json.loads(old), json.loads(new), json.loads(current))
        if conflicts:
            sys.exit("CONFLICT rebasing the rewrite of %s at %s" % (item["path"], ["/".join(map(str, c)) for c in conflicts[:5]]))
        place(paths.NEW, item["old"], current.encode("utf-8"))
        place(paths.NEW, item["new"], (json.dumps(merged, ensure_ascii=False, indent=2) + "\n").encode("utf-8"))
        item["oldSha256"] = sha(current)
        changed += 1
        print("re-based (structural) rewrite", item["id"], item["path"][-70:])
        continue
    with tempfile.TemporaryDirectory() as folder:
        for name, text in (("current", current), ("old", old), ("new", new)):
            open(os.path.join(folder, name), "w", encoding="utf-8", newline="").write(text)
        run = subprocess.run(["git", "merge-file", "-p", os.path.join(folder, "current"), os.path.join(folder, "old"), os.path.join(folder, "new")], capture_output=True, text=True, encoding="utf-8")
    if run.returncode != 0:
        sys.exit("CONFLICT rebasing the rewrite of %s (%d conflict(s)); nothing re-based" % (item["path"], run.returncode))
    place(paths.NEW, item["old"], current.encode("utf-8"))
    place(paths.NEW, item["new"], run.stdout.encode("utf-8"))
    item["oldSha256"] = sha(current)
    changed += 1
    print("re-based rewrite", item["id"], item["path"][-70:])
if changed:
    json.dump(plan, open(plan_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("pm-seam %s: %d rewrite(s) re-based" % (STAGE, changed))
