import re, collections, os, json
from lib import *
import rsparse as P
from rs import blank
f = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
tx = read(f); cl = blank(tx)
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
INC = re.compile(r'include!\(\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\)|"([^"]+)")\s*\)\s*;')
sts = P.statements(tx, cl, 0, len(tx))
mod = [s for s in sts if s.kind == "mod" and s.inline][0]
bo = tx.index("{", mod.start)
inner = P.statements(tx, cl, bo + 1, mod.end - 1)
items = []  # dict(src, kind, name, test, ids, lines, bytes)
def add(src, text, clean, s, include_of=None):
    ist = s.kind == "fn" and bool(re.search(r"#\[[^\]]*test[^\]]*\]", s.attrs or ""))
    head = clean[s.start:s.start + 600]
    selfn = None
    if s.kind == "impl":
        hh = head.split("{", 1)[0]
        hh = re.sub(r"<[^<>]*(?:<[^<>]*>[^<>]*)*>", "", hh, count=1) if hh.lstrip().startswith("impl<") else hh
        hh = hh.split(" where ")[0]
        if " for " in hh:
            hh = hh.split(" for ", 1)[1]
        mm = IDENT.findall(re.sub(r"<.*", "", hh.replace("impl", "", 1)))
        selfn = mm[-1] if mm else None
    items.append(dict(self=selfn, src=src, kind=s.kind, name=s.name, test=ist, ids=set(IDENT.findall(clean[s.start:s.end])), lines=text.count("\n", s.start, s.end) + 1, bytes=len(text[s.start:s.end].encode()), include_of=include_of))
for s in inner:
    seg = cl[s.start:s.end]
    m = INC.search(tx[s.start:s.end]) if s.kind == "other" and "include!" in seg else None
    if m:
        rel_ = m.group(1) or m.group(2)
        base = (PLUG + "/" + PKGDIR) if m.group(1) else os.path.dirname(f)
        target = os.path.normpath(os.path.join(base, rel_.lstrip("/") if m.group(1) else rel_))
        itx = read(target); icl = blank(itx)
        ist_ = P.statements(itx, icl, 0, len(itx))
        # include inside inline mod wrapper: treat atomic
        if len(ist_) == 1 and ist_[0].kind == "mod" and ist_[0].inline or True:
            for s2 in ist_:
                add(os.path.relpath(target, PLUG), itx, icl, s2, include_of=rel_)
        continue
    add("main", tx, cl, s)
byname = collections.defaultdict(list)
for i, it in enumerate(items):
    if it["name"] and it["kind"] != "use":
        byname[it["name"]].append(i)
for i, it in enumerate(items):
    it["deps"] = {j for n in it["ids"] for j in byname.get(n, []) if j != i}
def impl_self(i):
    it = items[i]
    return it.get("self")
for i, it in enumerate(items):
    it["i"] = i
    if it["kind"] == "impl" and it["self"]:
        for j in byname.get(it["self"], []):
            if items[j]["kind"] != "impl":
                items[j]["deps"].add(i)
                it["deps"].add(j)
print("items", len(items), "tests", sum(1 for i in items if i["test"]), "by src:", collections.Counter((i["src"][-40:], i["test"]) for i in items).most_common(30))
# closure per test
def closure(i):
    seen, st = set(), [i]
    while st:
        x = st.pop()
        for d in items[x]["deps"]:
            if d not in seen:
                seen.add(d); st.append(d)
    return seen
tests = [i for i, it in enumerate(items) if it["test"]]
cnt = collections.Counter()
for i in tests:
    for x in closure(i):
        if not items[x]["test"]: cnt[x] += 1
core = [x for x, c in cnt.items() if c >= len(tests) * 0.5]
print("helpers used by >=50% of tests:", len(core), sum(items[x]["lines"] for x in core), "lines", sum(items[x]["bytes"] for x in core), "bytes")
core2 = [x for x, c in cnt.items() if c >= len(tests) * 0.2]
print(">=20%:", len(core2), sum(items[x]["lines"] for x in core2), sum(items[x]["bytes"] for x in core2))
print("tests depending on tests:", sum(1 for i in tests if any(items[x]["test"] for x in closure(i))))
