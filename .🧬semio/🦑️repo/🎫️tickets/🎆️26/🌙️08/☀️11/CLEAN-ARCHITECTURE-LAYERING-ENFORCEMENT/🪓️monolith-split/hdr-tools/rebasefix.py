import re, os, sys
# 🧭️ rebasefix.py <old-pkgdir> <new-pkgdir> <file>... : re-bases manifest-dir-relative fixture literals of files that moved to a crate with another manifest directory.
old, new = [os.path.join("/Users/ueli/Documents/semio", a) if not a.startswith("/") else a for a in sys.argv[1:3]]
n = 0
def sub(m):
    p = m.group(2)
    if os.path.exists(os.path.normpath(os.path.join(new, p))):
        return m.group(0)
    tgt = os.path.normpath(os.path.join(old, p))
    if not os.path.exists(tgt):
        print("UNRESOLVED", p[-70:])
        return m.group(0)
    return m.group(1) + '"' + os.path.relpath(tgt, new) + '"'
def sub2(m):
    p = m.group(2)
    if os.path.exists(os.path.normpath(os.path.join(new, p))):
        return m.group(0)
    tgt = os.path.normpath(os.path.join(old, p))
    if not os.path.exists(tgt):
        print("UNRESOLVED", p[-70:])
        return m.group(0)
    return m.group(1) + '"/' + os.path.relpath(tgt, new) + '"'
for f in sys.argv[3:]:
    f = os.path.join("/Users/ueli/Documents/semio", f) if not f.startswith("/") else f
    t = open(f).read(); o = t
    t = re.sub(r'((?:runtime_fixture_(?:text|bytes)\(|fixture_(?:text|bytes|path)!\())"(\.\./[^"]+)"', sub, t)
    t = re.sub(r'(env!\("CARGO_MANIFEST_DIR"\),\s*)"/(\.\./[^"]+)"', sub2, t)
    if t != o:
        open(f, "w").write(t); n += 1
print("rebased files:", n)
