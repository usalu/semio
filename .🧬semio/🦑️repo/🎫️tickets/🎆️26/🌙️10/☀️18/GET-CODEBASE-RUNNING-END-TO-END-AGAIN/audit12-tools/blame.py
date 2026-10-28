#!/usr/bin/env python3
"""🔦 [audit12] attributes tsc errors (file(line,col): error …) to G46 batches: for each error line on live, the batches whose base→new diff added that exact line text, or `pre-G46` when no batch added it.
usage: python3 -I blame.py <tsc output> [--root DIR]"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ns = {}; exec([l for l in open(os.path.join(HERE, "compose.py"), encoding="utf8") if l.startswith("ORDER")][0], ns)
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.dirname(C.split("/.🧬semio/")[0] + "/x")
added = {}
for b in ns["ORDER"]:
    for rel, row in json.load(open(os.path.join(C, b, "manifest.json"), encoding="utf8")).items():
        base = os.path.join(C, b, "base", row["file"]) if row["base"] else "/dev/null"
        out = subprocess.run(["git", "diff", "--no-index", "--no-color", "-U0", base, os.path.join(C, b, "files", row["file"])], capture_output=True).stdout.decode("utf8", "replace").splitlines()
        for l in out:
            if l.startswith("+") and not l.startswith("+++"): added.setdefault((rel, l[1:]), set()).add(b.split("/")[0][:-6] + "/" + b.split("/")[-1])
touched = {rel for rel, _ in added}
for line in open(sys.argv[1], encoding="utf8"):
    m = re.match(r"^(\S.*?)\((\d+),(\d+)\): error (TS\d+)", line)
    if not m: continue
    rel, n = m.group(1), int(m.group(2))
    text = open(os.path.join(root, rel), encoding="utf8").read().splitlines()[n - 1]
    who = sorted(added.get((rel, text), [])) or (["pre-G46 line in a G46 file"] if rel in touched else ["file not touched by G46"])
    print(f"{m.group(4)} {rel[-90:]}:{n} -> {', '.join(who)}")
