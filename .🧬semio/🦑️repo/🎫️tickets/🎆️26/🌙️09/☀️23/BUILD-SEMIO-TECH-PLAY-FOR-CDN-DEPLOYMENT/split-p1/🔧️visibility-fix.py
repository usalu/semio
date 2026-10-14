#!/usr/bin/env python3
"""🔧️ Applies the visibility a split made necessary, straight from rustc's JSON diagnostics (lane P1).

E0603 (private item), E0624 (private method / associated fn) and E0451 use the definition span rustc reports; E0616
(private field) names `field` and `struct`, and the field is found inside that struct's definition under the roots.
Every found definition gains `pub`. Prints each edit.
usage: 🔧️visibility-fix.py <cargo json log> <root>...
"""
import json, os, re, sys
log, roots = sys.argv[1], sys.argv[2:]
WS = "/Users/ueli/Documents/semio"
def absf(f):
    if os.path.isabs(f): return os.path.normpath(f)
    for base in [WS] + roots + [os.path.dirname(r) for r in roots]:
        c = os.path.normpath(os.path.join(base, f))
        if os.path.exists(c): return c
    return os.path.normpath(os.path.join(WS, f))
sources = {}
for root in roots:
    for r, dirs, fs in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
        for f in fs:
            if f.endswith(".rs"): sources[os.path.join(r, f)] = None
def text(f):
    if sources.get(f) is None: sources[f] = open(f, encoding="utf-8").read().split("\n")
    return sources[f]
def make_pub(f, j, name):
    lines = text(f); l = lines[j]
    m = re.match(r"^(\s*)(pub(?:\([^)]*\))?\s+)?", l)
    if m.group(2) and m.group(2).strip() == "pub": return False
    lines[j] = m.group(1) + "pub " + l[len(m.group(0)):]
    print(f"{os.path.relpath(f, WS)}:{j + 1} -> pub {name}"); return True
done = set()
for line in open(log, encoding="utf-8"):
    try: d = json.loads(line)
    except Exception: continue
    if d.get("reason") != "compiler-message": continue
    m = d["message"]; code = (m.get("code") or {}).get("code")
    if code not in ("E0603", "E0616", "E0624", "E0451"): continue
    names = re.findall(r"`([^`]+)`", m["message"])
    if code == "E0616" and len(names) >= 2:
        field, struct = names[0], names[1].split("::")[-1].split("<")[0]
        for f in sources:
            lines = text(f)
            for i, l in enumerate(lines):
                if re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + re.escape(struct) + r"\b", l):
                    for j in range(i + 1, min(i + 200, len(lines))):
                        if re.match(r"^\}", lines[j]): break
                        if re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?" + re.escape(field) + r"\s*:", lines[j]):
                            if (f, j) not in done: done.add((f, j)); make_pub(f, j, f"{struct}.{field}")
                            break
        continue
    spans = [sp for c in m.get("children", []) for sp in c.get("spans", [])] + [sp for sp in m.get("spans", []) if not sp.get("is_primary")]
    name = names[0].split("::")[-1] if names else None
    for sp in spans:
        f = absf(sp["file_name"])
        if f not in sources: continue
        lines = text(f)
        for j in range(sp["line_start"] - 1, min(sp["line_start"] + 5, len(lines))):
            if name and re.search(r"\b(?:fn|struct|enum|const|static|type|trait|mod)\s+" + re.escape(name) + r"\b", lines[j]):
                if (f, j) not in done: done.add((f, j)); make_pub(f, j, name)
                break
for f, lines in sources.items():
    if lines is not None: open(f, "w", encoding="utf-8").write("\n".join(lines))
