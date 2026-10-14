#!/usr/bin/env python3
"""🩹️ Rebuilds integration-test modules that `🧪️integration-tests.py` corrupted (its census strip shortens raw strings and
nested-mount removal ran on already rewritten text): from the step's pre-image, nested child test mounts are removed,
`crate::`/`super::`/`self::` paths are rewritten with an offset-preserving strip and the module path from the pre-cut
artifact tree, then every `<monolith>::` path is retargeted to the family crate that owns its longest module prefix.

usage: 🩹️repair-integration.py <cut config> <pre-cut tar> <pre-image> <target file> [apply]
"""
import json, os, re, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
ns = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0].replace("\naggregator_layer = None", ""), CUT, "exec"), ns)
strip, mount_tree, Crate = ns["strip"], ns["mount_tree"], ns["Crate"]
config_path, tar, pre_path, target = sys.argv[1:5]
apply = len(sys.argv) > 5 and sys.argv[5] == "apply"
config = json.load(open(config_path, encoding="utf-8"))
ident = config["crate"]
artifact = config["artifact"]
tmp = tempfile.mkdtemp(dir=os.path.join(HERE, "🗑️generated"))
subprocess.run(["tar", "xf", tar, "-C", tmp, artifact + "/"], check=True)
old = Crate(os.path.join(tmp, artifact, "🦀️.rs"))
rel = os.path.relpath(target, REPO)
modpath = [s for s in old.files[os.path.normpath(os.path.join(tmp, rel))] if s != "component"]
raw = open(pre_path, encoding="utf-8").read()
if os.environ.get("UNMOUNT"):
    raw = re.sub(r'(?m)^[ \t]*(?:#\[cfg\(\s*test\s*\)\]\s*)?#\[path\s*=\s*"[^"]*"\]\s*mod\s+\w+\s*;[ \t]*\n', "", raw)
text = strip(raw)
edits = []
for p in re.finditer(r"(?<![\w:])(?P<head>(?:super\s*::\s*)+|crate\s*::\s*|self\s*::\s*)", text):
    head = p.group("head")
    if re.search(r"pub\s*\(\s*in\s+$", text[max(0, p.start() - 12):p.start()]):
        continue
    if head.startswith("crate"):
        rep = ident + "::"
    elif head.startswith("self"):
        rep = "::".join([ident] + modpath) + "::"
    else:
        rep = "::".join([ident] + modpath[:len(modpath) - head.count("super")]) + "::"
    edits.append((p.start(), p.end(), rep))
out, last = [], 0
for a, b, r in sorted(edits):
    out.append(raw[last:a]); out.append(r); last = b
out.append(raw[last:])
fixed = "".join(out)
roots = {ident: os.path.join(REPO, artifact, "🦀️.rs")}
roots.update({s["package"].replace("-", "_"): os.path.join(REPO, artifact, s["dir"], "🦀️.rs") for s in config["crates"].values()})
owners = {}
def walk(node, prefix, owner):
    if node[0] == "glob": return
    path = prefix + (node[1],)
    if node[0] == "mount":
        owners.setdefault(path, owner); return
    for child in node[4]:
        if child[0] == "mount" and child[1] == "component":
            owners.setdefault(path, owner)
        walk(child, path, owner)
for owner, root in roots.items():
    for _, tree in mount_tree(root, open(root, encoding="utf-8").read()):
        walk(tree, (), owner)
def retarget(m):
    segs = m.group(1).split("::")
    for k in range(len(segs), 0, -1):
        if tuple(segs[:k]) in owners:
            return owners[tuple(segs[:k])] + "::" + m.group(1)
    return m.group(0)
fixed = re.sub(r"(?<![\w:])" + ident + r"::((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)", retarget, fixed)
for pattern, repl in json.loads(os.environ.get("EXTRA", "[]")):
    fixed = re.sub(pattern, repl, fixed)
subprocess.run(["rm", "-rf", tmp])
print(f"{rel}: {len(edits)} paths")
if apply:
    open(target, "w", encoding="utf-8").write(fixed)
else:
    print(fixed[:1500])
