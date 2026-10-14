#!/usr/bin/env python3
"""🔬️ Item-level reference listing of one Rust file: per top-level item its kind, lines, in-file item references and
the external module heads it names (e.g. `geometry::`, `scene_paint::`), plus the reverse (who uses it).

usage: 🔬️item-deps.py <file.rs> [--heads h1,h2,...]
"""
import os, re, sys, importlib.util, collections
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py")); r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
path = sys.argv[1]
raw = open(path, encoding="utf-8").read(); text = r.strip(raw)
heads = sys.argv[sys.argv.index("--heads") + 1].split(",") if "--heads" in sys.argv else []
class M: pass
m = M(); m.file = path; m.start = 0; m.end = len(text)
t = r.Tree.__new__(r.Tree); t.text = {path: text}; t.raw = {path: raw}
spans = r.Tree.top_level(t, m)
items = []
for a, b in spans:
    head = re.sub(r"^(\s*#!?\s*\[[^\]]*\])*\s*", "", text[a:b])
    k = r.ITEM_HEAD.match(head)
    if k: items.append((k.group("name"), k.group("kind"), a, b)); continue
    i = re.match(r"(?:unsafe\s+)?impl\b\s*(?:<(?:[^<>]|<[^<>]*>)*>)?\s*([^{]*)", head)
    if i:
        tgt = re.findall(r.IDENT, re.sub(r"<.*", "", re.split(r"\s+for\s+", i.group(1))[-1].strip().lstrip("&")))
        items.append(("impl " + (tgt[-1] if tgt else "?"), "impl", a, b)); continue
    u = re.match(r"(pub\s+)?use\b", head)
    items.append((("use" if u else "other") + f"@{raw.count(chr(10), 0, a) + 1}", "use" if u else "other", a, b))
names = {n for n, k, a, b in items if k not in ("impl", "use", "other")}
uses = collections.defaultdict(set)
for n, k, a, b in items:
    body = text[a:b]
    ids = set(re.findall(r"(?<![\w.])(" + r.IDENT + r")\b", body))
    refs = sorted((ids & names) - {n})
    hs = sorted({h for h in heads if re.search(r"(?<![\w:])" + h + r"\s*::", body)})
    for x in refs: uses[x].add(n)
    print(f"{raw.count(chr(10), 0, a) + 1:5d} {body.count(chr(10)) + 1:4d} {k:7s} {n}: {' '.join(refs)} {'| ' + ' '.join(hs) if hs else ''}")
print("--- reverse")
for n in sorted(uses): print(f"{n} <- {' '.join(sorted(uses[n]))}")
