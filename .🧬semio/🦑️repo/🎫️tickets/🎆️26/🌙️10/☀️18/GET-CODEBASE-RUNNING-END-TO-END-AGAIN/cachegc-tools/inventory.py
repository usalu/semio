"""[cachegc] Read-only inventory of the graph optional cache (same rule as the ledger: optional leaves below inference/ matching OptionalQuota.optionalPath): logical vs allocated bytes, per top context, newest/oldest mtime, sparse leaves. usage: python3 -I inventory.py <repo root>"""
import json, os, re, sys, time
root = sys.argv[1]
schema = json.load(open(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json")))
q = schema["definitions"]["OptionalQuota"]; pattern = re.compile(q["optionalPath"])
base = os.path.join(root, ".cache/b/g/m/inference"); groups = {}; total = [0, 0, 0]; sparse = []; newest = 0
for d, _, names in os.walk(base):
    for n in names:
        p = os.path.join(d, n); rel = os.path.relpath(p, base)
        if not pattern.match(rel): continue
        s = os.lstat(p); alloc = s.st_blocks * 512; total[0] += 1; total[1] += s.st_size; total[2] += alloc; newest = max(newest, s.st_mtime)
        g = "/".join(rel.split("/")[:2]); r = groups.setdefault(g, [0, 0, 0, 0]); r[0] += 1; r[1] += s.st_size; r[2] += alloc; r[3] = max(r[3], s.st_mtime)
        if s.st_size > 1 << 20 and alloc < s.st_size // 4: sparse.append((s.st_size, alloc, rel, time.strftime("%m-%d %H:%M", time.localtime(s.st_mtime))))
print(json.dumps({"files": total[0], "logicalBytes": total[1], "allocatedBytes": total[2], "maximumBytes": q["policy"]["maximumBytes"], "lowWaterBytes": q["policy"]["lowWaterBytes"], "newest": time.strftime("%m-%d %H:%M", time.localtime(newest))}))
for g, r in sorted(groups.items(), key=lambda x: -x[1][1])[:12]: print(r[0], r[1], r[2], time.strftime("%m-%d %H:%M", time.localtime(r[3])), g)
for s in sparse: print("sparse", *s)
