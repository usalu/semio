import re, os, sys, json
# 🔎️ privscan.py <manifest> : lists cfg(test) `#[path]` test mounts of a crate's lib root, their sizes and the private root items (fn/struct/enum/const/static/type/trait without pub, and private fields) each test names.
R = "/Users/ueli/Documents/semio"
man = os.path.join(R, sys.argv[1])
mt = open(man).read()
lib = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)
root = os.path.normpath(os.path.join(os.path.dirname(man), lib))
rt = open(root).read()
print("root", root.replace(R + "/", "")[-90:], len(rt.split("\n")), "lines")
defs = {}
for mm in re.finditer(r"(?m)^\s*((?:pub(?:\([^)]*\))?\s+)?)(?:async\s+)?(?:unsafe\s+)?(fn|struct|enum|const|static|trait|type)\s+(\w+)", rt):
    vis, kind, name = mm.groups()
    if vis.strip() != "pub":
        defs.setdefault(name, (vis.strip() or "private", kind))
fields = set(re.findall(r"(?m)^\s{4}(\w+):\s", rt))
for m in re.finditer(r'#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\n(?:pub )?mod (\w+);', rt):
    f = os.path.normpath(os.path.join(os.path.dirname(root), m.group(1)))
    if not os.path.isfile(f): continue
    t = open(f).read()
    sup = len(re.findall(r"\bsuper::", t))
    priv = sorted(n for n, (v, k) in defs.items() if len(n) > 5 and re.search(r"\b%s\b" % re.escape(n), t) and k in ("fn", "struct", "enum", "const", "static", "type", "trait"))
    print(" ", m.group(2), len(t.split("\n")), "lines", os.path.getsize(f), "bytes", "super::", sup, "private names:", priv[:14], "(+%d)" % max(0, len(priv) - 14))
