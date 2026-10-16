"""Markdown census of every installed target: lines before (snapshot), parent after, children and the largest child."""
import json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

order = ["statutes", "verbs", "events", "metrics", "move", "g3unit", "g3root", "g3geom", "session"]
print("| Target | Lines before | Parent after | Children | Largest child (lines) | New folders |")
print("| --- | ---: | ---: | ---: | --- | ---: |")
tot_b = tot_c = 0
for name in order:
    mk = f"{W}/{name}/installed.json"
    if not os.path.exists(mk):
        print(f"| {name} | not installed |")
        continue
    t = TARGETS[name]
    files = json.loads(read(mk))["files"]
    before = len(lines_of(read(snap_dir(name) + "/" + t["file"])))
    parent = len(lines_of(read(ROOT + "/" + t["file"])))
    kids = [(len(lines_of(read(ROOT + "/" + f))), f.split("/")[-2]) for f in files if f != t["file"]]
    big = max(kids)
    tot_b += before
    print(f"| `{'/'.join(t['file'].split('/')[-5:-1])}` ({name}) | {before:,} | {parent:,} | {len(kids)} | {big[0]:,} (`{big[1]}`) | {len(kids)} |")
print()
