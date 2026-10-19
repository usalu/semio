import json, re, sys, collections
G = sys.argv[1]
rs = {}
for l in open(G, encoding="utf-8"):
    r = json.loads(l); rs[r["file"]] = r
ok = [r for r in rs.values() if r["rc"] == 0]
bad = [r for r in rs.values() if r["rc"] != 0]
print("suites", len(rs), "ok", len(ok), "bad", len(bad), "tests pass", sum(r["pass_"] for r in rs.values()), "fail", sum(r["fail"] for r in rs.values()))
def kind(r):
    t = open(r["log"], encoding="utf-8", errors="replace").read()
    if "alarm" in t[-200:] or r["rc"] in (142, -14): return "TIMEOUT"
    m = re.search(r"Cannot find (?:package|module) '([^']+)'", t)
    if m:
        s = m.group(1)
        return ("BROKEN-IMPORT " if s.startswith((".", "/")) else "MISSING-DEP ") + s[-70:]
    if r["pass_"] == 0 and r["fail"] == 0: return "NO-TESTS/LOAD " + (re.findall(r"^error: (.*)", t, re.M) or ["?"])[0][:80]
    return "ASSERT " + (re.findall(r"^error: (.*)", t, re.M) or ["?"])[0][:90]
c = collections.Counter()
for r in sorted(bad, key=lambda r: r["file"]):
    k = kind(r); c[k.split(" ")[0]] += 1
    print(f'{r["rc"]:>5} {r["pass_"]}/{r["fail"]} {k} | {r["file"][-110:]}')
print(c)
