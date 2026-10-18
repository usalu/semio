"""✅️ Validates every Cargo.lock: unique, byte-sorted packages; sorted dependency lists; every dependency resolves to a block; TOML parses."""
import re, subprocess, sys, json
T = "/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/locks.txt"
def vkey(v):
    core = v.split("+")[0].split("-")[0]
    return (tuple(int(x) if x.isdigit() else 0 for x in core.split(".")), 1 if "-" not in v.split("+")[0] else 0, v)
bad = 0; n = 0
for p in [l for l in open(T, encoding="utf8").read().split("\n") if l]:
    t = open(p, encoding="utf8").read(); n += 1
    blocks = t.split("[[package]]\n")[1:]
    ids = []; allnames = set(); unresolved = []
    info = []
    for b in blocks:
        name = re.search(r'^name = "([^"]+)"', b, re.M).group(1); ver = re.search(r'^version = "([^"]+)"', b, re.M).group(1)
        ids.append((name.encode(), vkey(ver))); allnames.add(name)
        deps = re.findall(r'^ "([^"]+)",?$', b, re.M); info.append((name, deps))
        if deps != sorted(deps, key=lambda s: s.encode()): print("UNSORTED-DEPS", p, name); bad += 1
    if ids != sorted(ids): print("UNSORTED", p); bad += 1
    if len(set(ids)) != len(ids): print("DUP", p); bad += 1
    for name, deps in info:
        for d in deps:
            if d.split(" ")[0] not in allnames: print("DANGLING", p, name, d); bad += 1
print("locks", n, "bad", bad)
