#!/usr/bin/env python3
# 🗂️ Stage 7: inserts the producer owner rows into 📋️build-owners.json in lexical order of the project name (text edit, one atomic write).
import json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
path = ROOT + "/📋️build-owners.json"
text = read(path)
data = json.loads(text)
names = [row["name"] for row in data["owners"]]
assert len(set(names)) == len(names)
lines = text.split("\n")
def row_text(crate):
    root = crate["pkgdir"][len(ROOT) + 1:]
    return [ "    {", f'      "name": "{crate["project"]}",', f'      "root": "{root}",', f'      "manifest": "{root}/📋️project.json",', '      "packages": []', "    }," ]
added = 0
for crate in sorted(CRATES, key=lambda c: c["project"]):
    if crate["project"] in names:
        continue
    names = [row["name"] for row in json.loads("\n".join(lines))["owners"]]
    successor = next(name for name in sorted(names) if name > crate["project"])
    index = next(i for i, line in enumerate(lines) if line == f'      "name": "{successor}",') - 1
    assert lines[index] == "    {", lines[index]
    lines[index:index] = row_text(crate)
    added += 1
new = "\n".join(lines)
json.loads(new)
write(path, new)
print("owner rows added", added)
