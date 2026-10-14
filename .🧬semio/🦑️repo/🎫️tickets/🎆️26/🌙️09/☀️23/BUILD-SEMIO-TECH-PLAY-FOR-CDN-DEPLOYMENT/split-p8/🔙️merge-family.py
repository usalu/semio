#!/usr/bin/env python3
"""🔙️ Folds a cut crate family back into its aggregator (the inverse of `✂️crate-cut.py apply`), so it can be re-cut
along corrected layers: the aggregator root becomes the union of every family root's mount tree plus their non-mount
items, every family path `<layer crate>::` inside the family becomes `crate::`, every consumer's `<layer crate>::`
becomes `<aggregator>::`, and the layer crate roots are deleted (their manifests and `[[test]]` targets are kept
for the re-cut, which rewrites them).

usage: 🔙️merge-family.py <config.json> plan|apply
"""
import json, os, re, sys, shutil, time
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
ns = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0].replace("\naggregator_layer = None", ""), CUT, "exec"), ns)
mount_tree, emit, Crate = ns["mount_tree"], ns["emit"], ns["Crate"]
config_path, mode = sys.argv[1], sys.argv[2]
config = json.load(open(config_path, encoding="utf-8"))
artifact = os.path.join(REPO, config["artifact"])
agg_root = os.path.join(artifact, "🦀️.rs")
agg_ident = config["crate"]
layer_roots = {spec["package"].replace("-", "_"): os.path.join(artifact, spec["dir"], "🦀️.rs") for spec in config["crates"].values()}
idents = sorted(layer_roots, key=len, reverse=True)


unrelocate = {}


def merge(into, node):
    if node[0] == "node":
        holder = next((c for c in into[4] if c[0] == "mount" and c[1] == node[1]), None)
        if holder is not None:
            for child in node[4]:
                if child[0] != "mount":
                    raise SystemExit(f"nested relocation below {node[1]} needs manual merge")
                unrelocate.setdefault(holder[4], []).append(child)
            return
    if node[0] == "mount":
        existing = next((c for c in into[4] if c[0] == "node" and c[1] == node[1]), None)
        if existing is not None:
            into[4].remove(existing)
            into[4].append(node)
            for child in existing[4]:
                unrelocate.setdefault(node[4], []).append(child)
            return
    if node[0] != "node":
        if not any(c[0] == "mount" and c[1] == node[1] and c[4] == node[4] for c in into[4]):
            into[4].append(node)
        return
    existing = next((c for c in into[4] if c[0] == "node" and c[1] == node[1]), None)
    if existing is None:
        into[4].append(("node", node[1], node[2], node[3], []))
        existing = into[4][-1]
    for child in node[4]:
        merge(existing, child)


top = ("node", "<root>", "pub", None, [])
tails, files = [], set()
for root in [agg_root] + [layer_roots[i] for i in idents if os.path.exists(layer_roots[i])]:
    raw = open(root, encoding="utf-8").read()
    trees = mount_tree(root, raw)
    for (a, b), tree in trees:
        merge(top, tree)
    spans = sorted((a, b) for (a, b), _ in trees if a is not None)
    rest, last = [], 0
    for a, b in spans:
        rest.append(raw[last:a]); last = b
    rest.append(raw[last:])
    body = "".join(rest)
    kept = [line for line in body.split("\n") if line.strip() and not line.startswith("//!") and not line.startswith("#![") and not line.startswith("extern crate ")]
    if root != agg_root:
        tails.append("\n".join(kept))
    files |= set(Crate(root).files)
agg_raw = open(agg_root, encoding="utf-8").read()
head = [line for line in agg_raw.split("\n") if line.startswith("//!")]
header = head + [""] + [line for line in agg_raw.split("\n") if line.startswith("#![")] + [""] + sorted({line for root in [agg_root] + list(layer_roots.values()) if os.path.exists(root) for line in open(root, encoding="utf-8").read().split("\n") if line.startswith("extern crate ")})
agg_trees = mount_tree(agg_root, agg_raw)
agg_spans = sorted((a, b) for (a, b), _ in agg_trees if a is not None)
rest, last = [], 0
for a, b in agg_spans:
    rest.append(agg_raw[last:a]); last = b
rest.append(agg_raw[last:])
agg_items = [line for line in "".join(rest).split("\n") if not line.startswith("//!") and not line.startswith("#![") and not line.startswith("extern crate ")]
body = []
for child in top[4]:
    body += emit(child, lambda f: "all", "all", artifact, 0)
virtual = "\n".join(header) + "\n\n" + "\n".join(body) + "\n" + "\n".join(agg_items).strip("\n") + "\n" + "\n".join(t for t in tails if t) + "\n"
pattern = re.compile(r"(?<![\w:])(" + "|".join(map(re.escape, idents + [agg_ident])) + r")(?=\s*::)")
writes = {agg_root: virtual}
for holder, children in unrelocate.items():
    raw = open(holder, encoding="utf-8").read().rstrip("\n") + "\n"
    for child in children:
        _, name, vis, cfg, path = child
        raw += (f"#[cfg({cfg})]\n" if cfg else "") + f'#[path = "{os.path.relpath(path, os.path.dirname(holder))}"]\n{(vis + " ") if vis else ""}mod {name};\n'
    writes[holder] = raw
for f in files:
    if f in layer_roots.values() or f == agg_root:
        continue
    raw = writes.get(f) or open(f, encoding="utf-8").read()
    new = pattern.sub("crate", raw)
    if new != raw:
        writes[f] = new
index = json.load(open(os.path.join(HERE, "🗑️generated", "stdio-consumers.json"), encoding="utf-8"))
consumer_pattern = re.compile(r"(?<![\w:])(" + "|".join(map(re.escape, idents)) + r")(?=\s*::)")
for ident in idents:
    for rel in index.get(ident, []):
        f = os.path.join(REPO, rel)
        if f in files or f in writes:
            continue
        raw = open(f, encoding="utf-8").read()
        new = consumer_pattern.sub(agg_ident, raw)
        if new != raw:
            writes[f] = new
print(f"virtual root {len(virtual.splitlines())} lines; rewrites {len(writes) - 1}; delete {len([r for r in layer_roots.values() if os.path.exists(r)])} roots")
if mode == "apply":
    backup = os.path.join(REPO, ".🧬semio/🦑️repo/⚡️cache/p8/backup", agg_ident + "-merge-" + time.strftime("%H%M%S"))
    for path in list(writes) + list(layer_roots.values()):
        if os.path.exists(path):
            target = os.path.join(backup, os.path.relpath(path, REPO)); os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copy2(path, target)
    for path, text in writes.items():
        open(path, "w", encoding="utf-8").write(text)
    for root in layer_roots.values():
        if os.path.exists(root):
            os.remove(root)
    print("backup", os.path.relpath(backup, REPO))
