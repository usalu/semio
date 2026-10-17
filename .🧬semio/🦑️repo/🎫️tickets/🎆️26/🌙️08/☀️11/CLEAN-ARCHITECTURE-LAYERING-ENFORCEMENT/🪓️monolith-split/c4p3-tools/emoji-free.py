import os, sys, json
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/c4p3-folder-emoji.json"
repo = set(json.load(open(G, encoding="utf8")))
d = sys.argv[1]
cands = sys.argv[2:]
def lead(name):
    out = ""
    for ch in name:
        if ch == "️" or (ch.isascii() and ch.isalnum()): break
        out += ch
    return out
used = {lead(n) for n in os.listdir(d)}
if not cands:
    print(" ".join(sorted(e for e in repo if e not in used)))
    sys.exit()
for c in cands:
    base = c.replace("️", "")
    print(("USED-HERE " if base in used else "free      "), ("repo-ok " if base in repo else "NOT-IN-REPO "), c)
