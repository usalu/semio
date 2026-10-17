#!/usr/bin/env python3
"""🔬️ Item/dependency analysis of the builder-contract test module: which helper items each test needs (transitively)."""
import re, collections, sys, json
from lib import *
import rsparse as P
from rs import blank
f = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
t = read(f); c = blank(t)
sts = P.statements(t, c, 0, len(t))
mod = [s for s in sts if s.kind == "mod"][0]
bo = t.index("{", mod.start)
inner = P.statements(t, c, bo + 1, mod.end - 1)
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
items = []
for i, s in enumerate(inner):
    txt = t[s.start:s.end]
    ln = t.count("\n", s.start, s.end) + 1
    ist = bool(re.search(r"#\[[^\]]*test[^\]]*\]", s.attrs or ""))
    items.append(dict(i=i, kind=s.kind, name=s.name, test=ist and s.kind == "fn", lines=ln, bytes=len(txt.encode()), ids=set(IDENT.findall(c[s.start:s.end])), start=s.start, end=s.end, attrs=s.attrs))
byname = collections.defaultdict(list)
for it in items:
    if it["name"] and it["kind"] not in ("use",):
        byname[it["name"]].append(it["i"])
impls = [it for it in items if it["kind"] == "impl"]
def impl_target(it):
    head = c[it["start"]:it["start"] + 400]
    m = re.search(r"impl(?:<[^>]*>)?\s+(?:[A-Za-z_:<>, ']+\s+for\s+)?([A-Za-z_][A-Za-z0-9_]*)", head)
    return m.group(1) if m else None
for it in impls:
    it["target"] = impl_target(it)
# deps
for it in items:
    deps = set()
    for n in it["ids"]:
        for j in byname.get(n, []):
            if j != it["i"]:
                deps.add(j)
    it["deps"] = deps
# impl blocks belong to their target type
for it in impls:
    for j in byname.get(it.get("target"), []):
        items[j]["deps"].add(it["i"])  # type depends on its impls (pull impls along)
        it["deps"].add(j)
def closure(i):
    seen, st = set(), [i]
    while st:
        x = st.pop()
        for d in items[x]["deps"]:
            if d not in seen:
                seen.add(d); st.append(d)
    return seen
tests = [it for it in items if it["test"]]
print("items", len(items), "tests", len(tests), "kinds", collections.Counter(it["kind"] for it in items))
tot_help = sum(it["lines"] for it in items if not it["test"])
print("helper (non-test) lines", tot_help, "bytes", sum(it["bytes"] for it in items if not it["test"]))
print("test lines", sum(it["lines"] for it in tests), "bytes", sum(it["bytes"] for it in tests))
used = set()
for it in tests:
    cl = closure(it["i"])
    it["cl"] = cl
    used |= cl
    it["cl_lines"] = sum(items[x]["lines"] for x in cl if not items[x]["test"])
print("helpers used by some test", sum(items[x]["lines"] for x in used if not items[x]["test"]))
print("tests depending on other tests?", sum(1 for it in tests if any(items[x]["test"] for x in it["cl"])))
import statistics
print("median helper closure lines per test", statistics.median(it["cl_lines"] for it in tests), "max", max(it["cl_lines"] for it in tests))
json.dump(dict(items=[dict(i=x["i"], kind=x["kind"], name=x["name"], test=x["test"], lines=x["lines"], bytes=x["bytes"]) for x in items]), open(GEN + "/pi2w7-bc-items.json", "w"))
