#!/usr/bin/env python3
"""Moves every product fixture that sits outside a neutral tests tree into the tests tree beside it and rewrites each reader (relative and module-relative strings) in the working overlay r. Source of truth for the red list: /private/tmp/testfix-real/fp-reds.txt (the law's own output); readers: refs.json (scan of every quoted 🧫️fixtures path)."""
import json, os, re, shutil, sys
O = "/private/tmp/testfix-real/r/"
P = "🧰️framework/🛍️products/"
BANNED = {"🧬️schema", "📦️packages", "🎯️targets", "🧩️implementations"}
reds = [P + l for l in open("/private/tmp/testfix-real/fp-reds.txt", encoding="utf8").read().split("\n") if l]
facades = [r for r in reds if r.endswith("🟦️.ts")]
data = [r for r in reds if not r.endswith("🟦️.ts")]
CUSTOM = {
    P + "💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🧪️tests/🧫️fixtures/🔣️.json":
    P + "💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/⌨️native-entrypoint/🎟️launch/🔣️.json",
}
def dest(p):
    if p in CUSTOM: return CUSTOM[p]
    parts = p.split("/")
    i = parts.index("🧫️fixtures")
    assert not any(x in BANNED for x in parts[:i]) and "🧪️tests" not in parts[:i], p
    return "/".join(parts[:i] + ["🧪️tests"] + parts[i:])
mapping = {p: dest(p) for p in data}
for p, d in mapping.items():
    assert not os.path.exists(O + d), ("collision", d)
    n16 = len(("/Users/ueli/Documents/semio/" + d).encode("utf-16-le")) // 2
    if n16 > 256: print("TOO LONG", n16, d)
refs = json.load(open("/private/tmp/testfix-real/refs.json", encoding="utf8"))
SKIP = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧪️tests/🟦️.ts"
report = []
byfile = {}
for old, rs in refs.items():
    new = mapping.get(old, old)
    if old == new: continue
    for f, n, s in rs:
        if f == SKIP: continue
        byfile.setdefault(f, []).append((n, s, old, new))
for f, items in byfile.items():
    path = O + f
    if not os.path.exists(path):
        for o in ("r", "r0"):
            d = "/private/tmp/testfix-real/%s/%s" % (o, f)
            os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copyfile("/Users/ueli/Documents/semio/" + f, d)
    lines = open(path, encoding="utf8").read().split("\n")
    for n, s, old, new in items:
        base = os.path.dirname(f)
        tgt = os.path.normpath(os.path.join(base, s)) if not s.startswith("/") else None
        if tgt == old:
            rel = os.path.relpath(new, base)
            ns = rel if (not s.startswith("./") and rel.startswith(".")) or not s.startswith(".") and not s.startswith("./") else ("./" + rel if not rel.startswith(".") else rel)
            if s.startswith("./") and not rel.startswith("."): ns = "./" + rel
            if not s.startswith(".") and not rel.startswith("."): ns = rel
        elif tgt is not None and old.startswith(tgt + "/"):
            report.append(("MANUAL-DIR", f, n, s)); continue
        elif old in CUSTOM:
            report.append(("MANUAL-CUSTOM", f, n, s)); continue
        else:
            ns = s.replace("🧫️fixtures", "🧪️tests/🧫️fixtures", 1)
        line = lines[n - 1]
        if line.count(s) < 1: report.append(("NOLINE", f, n, s)); continue
        lines[n - 1] = line.replace(s, ns, 1)
        report.append(("EDIT", f[-60:], n, s[-55:], ns[-55:]))
    open(path, "w", encoding="utf8").write("\n".join(lines))
for old, new in mapping.items():
    os.makedirs(os.path.dirname(O + new), exist_ok=True)
    shutil.move(O + old, O + new)
for r in report: print(*r)
print(len(mapping), "moved")
