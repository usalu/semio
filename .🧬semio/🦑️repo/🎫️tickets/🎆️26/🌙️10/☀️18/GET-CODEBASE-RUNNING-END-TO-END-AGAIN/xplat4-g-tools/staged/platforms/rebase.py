"""🧮️ Lists files/ entries missing from bases.tsv as NEW (paths that do not exist live) or with the live digest. Usage: python3 -I rebase.py"""
import hashlib, os
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = "/Users/ueli/Documents/semio"; FILES = os.path.join(HERE, "files")
known = {l.rstrip("\n").split("\t", 1)[1] for l in open(os.path.join(HERE, "bases.tsv"), encoding="utf-8") if l.strip()}
rows = []
for base, _, names in os.walk(FILES):
    for name in names:
        if name == ".DS_Store": continue
        rel = os.path.relpath(os.path.join(base, name), FILES)
        if rel in known: continue
        live = os.path.join(ROOT, rel)
        rows.append(("NEW" if not os.path.exists(live) else hashlib.sha256(open(live, "rb").read()).hexdigest()) + "\t" + rel + "\n")
open(os.path.join(HERE, "bases.tsv"), "a", encoding="utf-8").writelines(rows)
print(len(rows), "added")
