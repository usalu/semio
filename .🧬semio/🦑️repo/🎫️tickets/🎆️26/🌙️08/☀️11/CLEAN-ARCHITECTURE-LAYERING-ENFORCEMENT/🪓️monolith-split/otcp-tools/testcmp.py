"""Test-name multiset old vs new. usage: testcmp.py <crate-id>
old = P tree of the pre-edit snapshot (otcp-work/<id>/snap), tests mounted there; new = companion lib roots in the live repo plus P's remaining tests.
Module paths are compared as written; cfg `strip` maps the new companion-root prefixes (cfg key "new_prefix": {"<mirror>": "<old path>"})."""
import collections, json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
import testnames, union

cid = sys.argv[1]
c = union.cfg_of(cid)
W = union.W
old_lib = f"{W}/{cid}/snap/{c['p_lib']}"
old = collections.Counter(testnames.names(old_lib))
new = collections.Counter()
for comp in c["comps"]:
    lib = f"{union.ROOT}/{comp['lib']}"
    for n in testnames.names(lib):
        new[n] += 1
new_p = collections.Counter(testnames.names(f"{union.ROOT}/{c['p_lib']}"))
for n, k in new_p.items():
    new[n] += k
amap = c.get("name_map", {})
def mapn(n):
    for a, b in amap.items():
        if n == a or n.startswith(a + "::"):
            return b + n[len(a):]
    return n
newm = collections.Counter()
for n, k in new.items():
    newm[mapn(n)] += k
print(f"old tests {sum(old.values())} distinct {len(old)}; new tests {sum(newm.values())} distinct {len(newm)}")
only_old = old - newm
only_new = newm - old
print("only in old:", sum(only_old.values()), "only in new:", sum(only_new.values()))
for n in sorted(only_old)[:20]: print("  -", n)
for n in sorted(only_new)[:20]: print("  +", n)
print("IDENTICAL" if not only_old and not only_new else "DIFFERENT")
