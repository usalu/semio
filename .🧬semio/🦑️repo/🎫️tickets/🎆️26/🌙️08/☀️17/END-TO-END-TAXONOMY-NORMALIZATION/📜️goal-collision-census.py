#!/usr/bin/env python3
"""🧩️ Collision census: which directories would collide if every leaf were renamed to kind-only."""
import json, subprocess, collections, os

TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
tax = json.load(open(TAX))
kinds = tax["fileKinds"]
ext_emoji = []
for kid, spec in kinds.items():
    for ext in spec["extensionChains"]:
        ext_emoji.append((ext, spec["emoji"], kid))
ext_emoji.sort(key=lambda x: -len(x[0]))

files = [f for f in subprocess.run(["git","ls-files","-z"],capture_output=True).stdout.decode().split("\0") if f]

def kindonly(base):
    for ext, emoji, kid in ext_emoji:
        if base.endswith(ext):
            return emoji + ext, kid, base[:-len(ext)]
    return None, None, None

bydir = collections.defaultdict(list)
unknown = collections.Counter()
for f in files:
    if f.startswith(".🧬semio/") or "/.🧬semio/" in f: continue
    d, _, base = f.rpartition("/")
    tgt, kid, stem = kindonly(base)
    if tgt is None:
        unknown[base.rsplit(".",1)[-1] if "." in base else base] += 1
        continue
    bydir[d].append((base, tgt, stem, kid))

collide = collections.Counter()
collide_ex = collections.defaultdict(list)
stems_in_collision = collections.Counter()
clean_renames = 0
already_ok = 0
for d, items in bydir.items():
    tgtcount = collections.Counter(t for _,t,_,_ in items)
    for base, tgt, stem, kid in items:
        if base == tgt:
            already_ok += 1
        elif tgtcount[tgt] > 1:
            key = (kid, tuple(sorted({s for b,t,s,k in items if t == tgt})))
            collide[key] += 1
            stems_in_collision[stem] += 1
            if len(collide_ex[key]) < 3: collide_ex[key].append(d + "/" + base)
        else:
            clean_renames += 1

print(f"already kind-only : {already_ok}")
print(f"clean renames (no collision in dir): {clean_renames}")
print(f"colliding files   : {sum(collide.values())}")
print(f"unknown extensions: {sum(unknown.values())} -> {unknown.most_common(25)}")
print()
print("=== collision classes (kind, stem-set) by file count ===")
for (kid, stems), c in collide.most_common(45):
    print(f"{c:7d}  {kid:20s}  {list(stems)}")
    for e in collide_ex[(kid,stems)]: print(f"           {e}")
print()
print("=== stems appearing in collisions ===")
for s,c in stems_in_collision.most_common(50): print(f"{c:7d}  {s!r}")
