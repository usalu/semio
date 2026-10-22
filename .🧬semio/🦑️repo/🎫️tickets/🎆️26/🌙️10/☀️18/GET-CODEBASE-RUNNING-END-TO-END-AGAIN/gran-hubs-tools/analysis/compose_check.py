"""🧬 Composition check of an ordered list of plans against the checkout: every plan's file edits are applied on the texts the earlier plans produce (rebased, context-anchored); crates whose edits fail are pruned from the later plan (their hub edge stays) until the list composes. Rewrites the plan files. `python3 compose_check.py tag1,tag2,...`"""
import json, os, sys, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(T + "/../staged"))
import apply as A
import re
from manifest_edit import sections, line_ops, apply_ops
ROOT = "/Users/ueli/Documents/semio"
tags = sys.argv[1].split(",")
pd = os.path.normpath(T + "/../staged/plans")
plans = [json.load(open(f"{pd}/{t}.json")) for t in tags]
texts = {}
def cur(rel):
    if rel not in texts:
        texts[rel] = open(os.path.join(ROOT, rel), "rb").read().decode("utf-8")
    return texts[rel]
def dedupe(text):
    """Drops a repeated dependency key within one table when both lines are identical; returns (text, conflict)."""
    lines = text.split("\n")
    drop, conflict = set(), False
    for name, a, b in sections(lines):
        if a < 0:
            continue
        seen = {}
        for i in range(a + 1, b):
            m = re.match(r'^\s*"?([A-Za-z0-9_.-]+?)"?\s*=', lines[i])
            if not m:
                continue
            k = m.group(1)
            if k in seen:
                if lines[seen[k]].strip() == lines[i].strip():
                    drop.add(i)
                else:
                    conflict = True
            else:
                seen[k] = i
    return "\n".join(l for i, l in enumerate(lines) if i not in drop), conflict


pruned = collections.Counter()
for p in plans:
    for c in p.get("creates", []):
        texts[c["path"]] = c["content"]
    while True:
        bad = set()
        trial = {}
        for f in p["files"]:
            base = trial.get(f["path"], cur(f["path"]))
            try:
                trial[f["path"]] = A.new_text_of_file(base, f, A.sha(base.encode("utf-8")) == f["sha256"])
            except ValueError as e:
                bad |= set(f["crates"])
        for m in p["manifests"]:
            base = trial.get(m["path"], cur(m["path"]))
            try:
                res = m["new_text"] if A.sha(base.encode("utf-8")) == m["sha256"] else A.new_text_of_manifest(base, m)
                res2, conflict = dedupe(res)
                if conflict:
                    bad.add(m["crate"])
                    continue
                if res2 != res:
                    m["ops"] = line_ops(base, res2)
                    m["new_text"] = res2
                    m["sha256"] = A.sha(base.encode("utf-8"))
                    m["sha256_after"] = A.sha(res2.encode("utf-8"))
                    res = res2
                trial[m["path"]] = res
            except ValueError:
                bad.add(m["crate"])
        bad = {c for c in bad if c in p.get("crates", {}) or True}
        if not bad:
            break
        pruned[p["tag"]] += len(bad)
        p["files"] = [f for f in p["files"] if not (set(f["crates"]) & bad)]
        p["manifests"] = [m for m in p["manifests"] if m["crate"] not in bad]
        for c in bad: p.get("crates", {}).pop(c, None)
    texts.update(trial)
    p["stats"]["files"], p["stats"]["manifests"] = len(p["files"]), len(p["manifests"])
    p["stats"]["crates"] = len(p.get("crates", {})) or p["stats"].get("crates")
    json.dump(p, open(f"{pd}/{p['tag']}.json", "w"), ensure_ascii=False)
    print(p["tag"], "files", len(p["files"]), "manifests", len(p["manifests"]), "pruned crates", pruned[p["tag"]])
