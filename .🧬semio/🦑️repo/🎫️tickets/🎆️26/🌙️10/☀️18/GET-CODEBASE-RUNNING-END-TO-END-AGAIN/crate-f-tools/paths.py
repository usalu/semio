import re, os, sys
root = "/Users/ueli/Documents/semio"
txt = open(root + "/Cargo.toml").read()
members = re.findall(r'"(🧰️framework/🛍️products/🦑️repo/[^"]*)"', txt)
bad = 0
for m in members:
    d = os.path.join(root, m)
    f = os.path.join(d, "Cargo.toml")
    if not os.path.exists(f):
        print("NOCARGO", m); bad += 1; continue
    s = open(f).read()
    for p in re.findall(r'^path\s*=\s*"([^"]*)"', s, re.M):
        t = os.path.normpath(os.path.join(d, p))
        if not os.path.exists(t):
            print("MISSING", m, "->", p); bad += 1
print(len(members), "members", bad, "bad")
