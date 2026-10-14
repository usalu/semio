#!/usr/bin/env python3
"""📐️ Predicts each cut layer crate's lib and lib-test unit against the Source ceiling before `✂️crate-cut.py apply`.

Reads the monolith's canonical closure (`📂️closure-files.ts`, cached in `🗑️generated/<crate>-closure.json`), assigns
every source file to a layer by the config's layer rules and every embedded input to the layer of the source that
embeds it, and prints per layer: lib files/lines/bytes and test files/lines/bytes (+1 file and the mount lines for the
generated root), flagging OVER. With `--files <layer>` it lists that layer's files.

usage: 📐️layer-sizes.py <config.json> [--refresh] [--files <layer>]
"""
import collections, json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CEIL = (4000, 262144, 32)
args = sys.argv[1:]
config_path = args[0]
config = json.load(open(config_path, encoding="utf-8"))
base = os.path.dirname(os.path.abspath(config_path))
artifact = os.path.join(REPO, config["artifact"])
root = os.path.join(artifact, config.get("root", "🦀️.rs"))
cache = os.path.join(HERE, "🗑️generated", config["crate"] + "-closure.json")
if "--refresh" in args or not os.path.exists(cache):
    os.makedirs(os.path.dirname(cache), exist_ok=True)
    out = subprocess.run(["bun", os.path.join(HERE, "📂️closure-files.ts"), root], capture_output=True, text=True, check=True).stdout
    open(cache, "w").write(out)
data = json.load(open(cache))
rules_cfg = json.load(open(os.path.join(base, config["layers"]), encoding="utf-8"))
rules = [(re.compile(r), l) for r, l in rules_cfg["rules"]]
def layer(rel_repo):
    rel = os.path.relpath(os.path.join(REPO, rel_repo), artifact)
    for rule, name in rules:
        if rule.search(rel):
            return name
    return "?"
owner = {}
for path in data["test"]:
    if path.endswith(".rs"):
        owner[path] = layer(path)
embedders = collections.defaultdict(set)
for path in data["test"]:
    if not path.endswith(".rs"):
        continue
    text = open(os.path.join(REPO, path), encoding="utf-8").read()
    for m in re.finditer(r'include_(?:str|bytes)\s*!\s*\(\s*"([^"]+)"', text):
        target = os.path.relpath(os.path.normpath(os.path.join(REPO, os.path.dirname(path), m.group(1))), REPO)
        embedders[target].add(owner[path])
rows = collections.defaultdict(lambda: {"lib": [0, 0, 0], "test": [0, 0, 0], "files": []})
for path in data["test"]:
    layers = {owner[path]} if path in owner else (embedders.get(path) or {"?"})
    for name in layers:
        row = rows[name]
        row["files"].append(path)
        for unit in (("lib", "test") if path in data["lib"] else ("test",)):
            lines = data[unit].get(path, 0)
            row[unit][0] += 1; row[unit][1] += lines; row[unit][2] += data["bytes"][path]
def cell(v):
    over = v[0] + 1 > CEIL[2] or v[1] > CEIL[0] or v[2] > CEIL[1]
    return f"{'OVER' if over else 'ok  '} {v[0] + 1:3d}F {v[1]:5d}L {v[2] // 1024:5d}KB"
order = list(config["crates"]) + [config["aggregator"]]
for name in order + sorted(set(rows) - set(order)):
    if name in rows:
        print(f"{name:28s} lib {cell(rows[name]['lib'])} | test {cell(rows[name]['test'])}")
if "--files" in args:
    want = args[args.index("--files") + 1]
    for path in sorted(rows[want]["files"]):
        print(f"  {data['test'].get(path, 0):5d} {'L' if path in data['lib'] else 'T'} {os.path.relpath(os.path.join(REPO, path), artifact)}")
