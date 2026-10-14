#!/usr/bin/env python3
"""🧩️ Retargets an artifact's composition contribution (`🧩️composition/🔣️.json`) after a crate cut: every Rust path
that starts with a family crate ident is re-resolved to the crate whose mount tree owns its longest module prefix,
and `symbolPackages` becomes the family packages those paths name (the contribution's own `package` excluded).

usage: 🧩️contribution-paths.py <cut config.json> <contribution json (repo-rel)> [apply]
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
ns = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0].replace("\naggregator_layer = None", ""), CUT, "exec"), ns)
mount_tree = ns["mount_tree"]
config = json.load(open(sys.argv[1], encoding="utf-8"))
contribution_path = os.path.join(REPO, sys.argv[2])
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
artifact = os.path.join(REPO, config["artifact"])
roots = {config["crate"]: os.path.join(artifact, config.get("root", "🦀️.rs"))}
roots.update({spec["package"].replace("-", "_"): os.path.join(artifact, spec["dir"], "🦀️.rs") for spec in config["crates"].values()})
owners = {}


def walk(node, prefix, ident):
    if node[0] == "glob":
        return
    path = prefix + (node[1],)
    if node[0] == "mount":
        owners.setdefault(path, ident)
        return
    for child in node[4]:
        if child[0] == "mount" and child[1] == "component":
            owners.setdefault(path, ident)
        walk(child, path, ident)


for ident, root in roots.items():
    raw = open(root, encoding="utf-8").read()
    for _, tree in mount_tree(root, raw):
        walk(tree, (), ident)
idents = sorted(roots, key=len, reverse=True)
used = set()


def retarget(text):
    def fix(m):
        segs = m.group(2).split("::")
        for k in range(len(segs), 0, -1):
            owner = owners.get(tuple(segs[:k]))
            if owner:
                used.add(owner)
                return owner + "::" + m.group(2)
        used.add(m.group(1))
        return m.group(0)
    return re.sub(r"\b(" + "|".join(map(re.escape, idents)) + r")::((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)", fix, text)


data = json.load(open(contribution_path, encoding="utf-8"))
def visit(value):
    if isinstance(value, dict):
        return {k: visit(v) for k, v in value.items()}
    if isinstance(value, list):
        return [visit(v) for v in value]
    if isinstance(value, str):
        return retarget(value)
    return value
data = visit(data)
main = data["package"].replace("-", "_")
family = {ident.replace("_", "-") for ident in roots}
data["symbolPackages"] = sorted({p for p in data.get("symbolPackages", []) if p not in family} | {ident.replace("_", "-") for ident in used if ident != main})
text = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
print("symbolPackages:", data["symbolPackages"])
if apply:
    open(contribution_path, "w", encoding="utf-8").write(text)
