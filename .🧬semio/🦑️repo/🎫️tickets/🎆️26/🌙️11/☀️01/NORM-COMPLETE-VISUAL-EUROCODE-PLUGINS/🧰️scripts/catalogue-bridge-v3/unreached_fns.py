"""Lists pure pub fns of an entity-driven EN code that no check reachable from a snapshot-iterating file calls (directly or through other pure fns). Usage: python3 -I unreached_fns.py CODE_DIR_NAME PURE_SUBSTR... ; output: TSV file<TAB>fn<TAB>line<TAB>direct callers outside pure files<TAB>pure callers"""
import os, re, sys
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
code, pure = sys.argv[1], sys.argv[2:]
SKIP = ("mutations", "🧪️tests", "/tests", "🚪️io", "🔺️diff", "🎬️scene", "scene-members", "✏️editor", "👁️viewer", "📚️examples", "🧭️spec-trace", "🏷️field-meta", "📇️catalogue")
files = []
for r, d, f in os.walk(A + code):
    for n in f:
        if n.endswith(".rs"):
            p = os.path.join(r, n)
            if not any(s in p for s in SKIP): files.append(p)
FN = re.compile(r"^\s*(pub(?:\([a-z]+\))?\s+)?(?:const\s+)?(?:async\s+)?fn\s+([a-z0-9_]+)")
def ispure(p): return any(s in p for s in pure)
defs, sites = {}, []
for p in files:
    cur = None
    for i, line in enumerate(open(p, encoding="utf8"), 1):
        m = FN.match(line)
        if m:
            cur = m.group(2)
            if ispure(p) and m.group(1): defs.setdefault(cur, []).append((p, i))
        for name in re.findall(r"\b([a-z][a-z0-9_]*)\s*[(:<]", line):
            if m and name == m.group(2) and FN.match(line): continue
            sites.append((name, p, cur))
callers = {}
for name, p, cur in sites:
    if name in defs: callers.setdefault(name, []).append((p, cur))
live, work = set(), []
for name, cs in callers.items():
    if any(not ispure(p) for p, _ in cs): live.add(name); work.append(name)
rev = {}
for name, cs in callers.items():
    for p, cur in cs:
        if ispure(p) and cur: rev.setdefault(cur, set()).add(name)
while work:
    n = work.pop()
    for c in rev.get(n, ()):
        if c not in live: live.add(c); work.append(c)
for name, ds in sorted(defs.items()):
    if name not in live:
        for p, i in ds:
            print(f"{p.replace(A + code + '/🏅️standards/🔖️1/🪆️subsets/✳️any/', '')}\t{name}\t{i}\t{len(callers.get(name, []))}")
