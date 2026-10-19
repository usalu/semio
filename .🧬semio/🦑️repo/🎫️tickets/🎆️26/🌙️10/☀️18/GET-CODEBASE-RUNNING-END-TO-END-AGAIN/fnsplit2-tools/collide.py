"""🧯 Name collision check for new Rust helper names: usage collide.py <spec.json> <searchRoot>; prints every other definition of each helper or struct name below the search root."""
import json, re, subprocess, sys
spec = json.load(open(sys.argv[1], encoding="utf8"))
root = sys.argv[2]
names = []
for ph in spec.get("phases", []):
    names.append(ph["name"])
    if ph.get("struct"): names.append(ph["struct"])
for ar in spec.get("arms", []):
    names.append(ar["name"])
bad = 0
for n in names:
    r = subprocess.run(["rg", "-n", "--no-ignore", "-g", "*.rs", r"\b(fn|struct|enum|type|const|static|mod|trait)\s+%s\b" % re.escape(n), root], capture_output=True, text=True).stdout.strip().split("\n")
    r = [x for x in r if x]
    if len(r) > 1:
        bad += 1
        print(n, len(r)); [print("   ", x[-160:]) for x in r[:4]]
print("collisions", bad, "of", len(names))
