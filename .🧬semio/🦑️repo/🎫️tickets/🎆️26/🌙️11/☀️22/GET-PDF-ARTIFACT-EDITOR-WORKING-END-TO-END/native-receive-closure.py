#!/usr/bin/env python3
"""usage: native-receive-closure.py <plugin directory> [--roots=Type,Type] [--write]  (derives `semio_framework_value::NativeReceive` on every `FromValue` type reachable by field-type name from a type that already derives it, to a fixpoint inside the plugin directory)"""
import os, re, sys
root = sys.argv[1]; write = "--write" in sys.argv
defs = {}
files = {}
item = re.compile(r'((?:[ \t]*///[^\n]*\n|[ \t]*#\[[^\n]*\]\n)*)[ \t]*pub(?:\([a-z]+\))? (struct|enum) (\w+)([^;{]*?)(\{|\(|;)', re.M)
for r, d, f in os.walk(root):
    if "node_modules" in r: continue
    for n in f:
        if n != "🦀️.rs": continue
        p = os.path.join(r, n); s = open(p, encoding="utf8").read(); files[p] = s
        for m in item.finditer(s):
            attrs = m.group(1)
            derive = " ".join(re.findall(r'#\[derive\(([^\]]*)\)\]', attrs))
            start = m.end() - 1
            depth = 0; i = start; opener = s[start]; closer = {"{": "}", "(": ")", ";": ";"}[opener]
            if opener == ";": body = ""
            else:
                while i < len(s):
                    if s[i] == opener: depth += 1
                    elif s[i] == closer:
                        depth -= 1
                        if depth == 0: break
                    i += 1
                body = s[start:i]
            defs.setdefault(m.group(3), []).append(dict(path=p, derive=derive, body=body, attrs_start=m.start(1), attrs=attrs))
has = lambda d: "NativeReceive" in d["derive"]
fromv = lambda d: re.search(r'\bFromValue\b', d["derive"]) is not None
seeds = [a.split("=",1)[1] for a in sys.argv if a.startswith("--roots=")]
need = set(n for n, ds in defs.items() if any(has(d) for d in ds)) | {r for g in seeds for r in g.split(",") if r in defs}
added = set()
changed = True
while changed:
    changed = False
    for n in list(need):
        for d in defs[n]:
            for ident in set(re.findall(r'\b[A-Z]\w*\b', d["body"])):
                if ident in defs and ident not in need and any(fromv(x) and not has(x) for x in defs[ident]):
                    need.add(ident); added.add(ident); changed = True
print(f"roots {len([n for n in defs if any(has(d) for d in defs[n])])} added {len(added)}")
for n in sorted(added): print("  ", n, [os.path.relpath(d["path"], root)[-60:] for d in defs[n]])
if write:
    edits = {}
    for n in added:
        for d in defs[n]:
            if fromv(d) and not has(d): edits.setdefault(d["path"], []).append(d)
    for p, ds in edits.items():
        s = open(p, encoding="utf8").read()
        for d in sorted(ds, key=lambda d: -d["attrs_start"]):
            a = d["attrs"]
            m = list(re.finditer(r'#\[derive\(([^\]]*)\)\]', a))
            g = [x for x in m if re.search(r'\bFromValue\b', x.group(1))][0]
            new = g.group(0).replace("#[derive(", "#[derive(semio_framework_value::NativeReceive, ", 1)
            na = a[:g.start()] + new + a[g.end():]
            assert s[d["attrs_start"]:d["attrs_start"] + len(a)] == a
            s = s[:d["attrs_start"]] + na + s[d["attrs_start"] + len(a):]
        open(p, "w", encoding="utf8").write(s)
    print("wrote", len(edits), "files")
