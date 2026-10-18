"""🗺️ OHR shell API map (read only): for names used as `<x>.shell.<name>` in the given files, prints where `<name>` is defined now in the Shell staging tree (impl header per method; component struct per field).
usage: shellmap.py <rs file>..."""
import os, re, sys
R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging"
fns, fields = {}, {}
for dp, dn, fn in os.walk(R):
    for f in fn:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(dp, f)
        impl, struct = None, None
        for i, l in enumerate(open(p, encoding="utf-8"), 1):
            m = re.match(r"^impl(?:<[^>]*>)?\s+(?:[A-Za-z_:<>']+\s+for\s+)?([A-Za-z_0-9]+)", l)
            if m:
                impl, struct = m.group(1), None
            m = re.match(r"^(?:pub )?struct ([A-Za-z_0-9]+)", l)
            if m:
                struct, impl = m.group(1), None
            m = re.match(r"^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?fn ([a-z_0-9]+)", l)
            if m and impl:
                fns.setdefault(m.group(1), set()).add(f"{impl} {os.path.relpath(p, R).split('/')[0]}")
            m = re.match(r"^\s*pub(?:\([^)]*\))?\s+([a-z_0-9]+)\s*:", l)
            if m and struct:
                fields.setdefault(m.group(1), set()).add(f"{struct} {os.path.relpath(p, R).split('/')[0]}")
names = set()
for p in sys.argv[1:]:
    t = open(p, encoding="utf-8").read()
    for m in re.finditer(r"\bshell\s*\.\s*([a-z_0-9]+)\s*(\()?", t):
        names.add((m.group(1), bool(m.group(2))))
for n, call in sorted(names):
    print(("fn " if call else "field ") + n, "->", sorted((fns if call else fields).get(n, [])) or "??")
