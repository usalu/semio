#!/usr/bin/env python3
"""📏️ Laws of the lines a staged batch ADDS (the whole file is not judged: pre-existing findings stay with their owners): python3 -I docheck.py <batch dir> <root>. Parse-check with C/gate-g-tools/scan.ts, every added docstring opens with an emoji that no other docstring of the file opens with, no added line carries `[DEBUG]`."""
import json, os, re, subprocess, sys, tempfile
batch, root = sys.argv[1:3]
here = os.path.dirname(os.path.abspath(__file__))
c = os.path.dirname(here)
manifest = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
OPENER = re.compile(r"/\*\*\s*\n?\s*\*?\s*((?:[\U0001F000-\U0001FAFF←-⯿©®‼-㊙][️‍]*)+)")
bad = 0
parse = []
for rel, row in manifest.items():
    if not rel.endswith((".ts", ".mts", ".mjs", ".py", ".json")) or row["new"] is None:
        continue
    new = open(os.path.join(batch, "files", row["file"]), encoding="utf8").read()
    old = open(os.path.join(batch, "base", row["file"]), encoding="utf8").read() if row["base"] else ""
    if rel.endswith((".ts", ".mts", ".mjs")) and not rel.endswith(".d.mts"):
        parse.append(os.path.join(root, rel))
    if rel.endswith(".json"):
        json.loads(new)
        continue
    added = [line for line in new.split("\n") if line not in set(old.split("\n"))]
    if any("[DEBUG]" in line for line in added):
        bad += 1
        print("FAIL [DEBUG] in an added line", rel[-70:])
    openers = [m.group(1).replace("️", "") for m in OPENER.finditer(new)]
    for line in added:
        for m in OPENER.finditer(line):
            emoji = m.group(1).replace("️", "")
            if openers.count(emoji) > 1:
                bad += 1
                print("FAIL docstring emoji reused", emoji, rel[-70:])
            if not m.group(1):
                bad += 1
with tempfile.TemporaryDirectory() as scratch:
    listing = os.path.join(scratch, "p.txt")
    open(listing, "w", encoding="utf8").write("\n".join(parse) + "\n")
    run = subprocess.run(["bun", os.path.join(c, "gate-g-tools", "scan.ts"), listing, os.path.join(scratch, "s")], capture_output=True, text=True, cwd=c)
    print(run.stdout.strip().splitlines()[-1] if run.stdout.strip() else run.stderr[-300:])
    if "bad 0" not in run.stdout:
        bad += 1
        print(run.stdout[-1200:])
print("docheck: %d files, %d findings" % (len(manifest), bad))
sys.exit(1 if bad else 0)
