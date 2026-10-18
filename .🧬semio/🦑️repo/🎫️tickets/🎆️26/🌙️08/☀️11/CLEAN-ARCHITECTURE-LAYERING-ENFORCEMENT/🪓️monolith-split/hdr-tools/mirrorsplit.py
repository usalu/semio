import json, os, re, shutil, sys
# 🪞️ mirrorsplit.py <manifest> <node-dir (repo relative)> <suffix> <mount,mount..> [--write] : moves cfg(test) test mounts of a path-preserving mirror into a companion crate with its own copy of the mirror.
# The companion root and mirror are copies of the owner's (paths rebased); test mounts that stay are removed from the companion and the moved ones from the owner. Registers member, dependency table and owner row.
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
write = "--write" in args
pos = [a for a in args if not a.startswith("--")]
man = os.path.join(R, pos[0]); node = pos[1]; suffix = pos[2]; moved = pos[3].split(",")
mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
root = os.path.normpath(os.path.join(pkgdir, re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)))
rt = open(root).read()
mm = re.search(r'#\[path = "([^"]+)"\]\nmod mirror;', rt)
mirror = os.path.normpath(os.path.join(os.path.dirname(root), mm.group(1)))
mtxt = open(mirror).read()
newpkgdir = os.path.join(R, node, PKG)
newroot = os.path.join(newpkgdir, os.path.basename(root))
newmirror = os.path.join(newpkgdir, os.path.relpath(mirror, os.path.dirname(root)))
newpkg = "%s-%s" % (pkg, suffix)
assert not os.path.exists(newpkgdir)
rx = re.compile(r'( *)#\[cfg\(test\)\]\n *#\[path = "([^"]+)"\]\n *pub mod (\w+);\n')
blocks = list(rx.finditer(mtxt))
names = [b.group(3) for b in blocks]
def _isv(b):
    return any(n == b.group(3) or n in b.group(2) for n in moved)
for n in moved: assert any(n == b.group(3) or n in b.group(2) for b in blocks), (n, names)
def rebased(b):
    absf = os.path.normpath(os.path.join(os.path.dirname(mirror), b.group(2)))
    return b.group(0).replace(b.group(2), os.path.relpath(absf, os.path.dirname(newmirror)))
owner_m = mtxt
comp_m = mtxt
comp_m_raw = None
for b in blocks:
    if _isv(b):
        owner_m = owner_m.replace(b.group(0), "", 1)
        pass
    else:
        comp_m = comp_m.replace(b.group(0), "", 1)
comp_root = rt.replace("`%s`" % pkg, "`%s`" % newpkg) if False else rt
dpath = lambda l: re.sub(r'path = "([^"]+)"', lambda m: 'path = "%s"' % os.path.relpath(os.path.normpath(os.path.join(pkgdir, m.group(1))), newpkgdir), l)
wsabs = os.path.normpath(os.path.join(pkgdir, re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)))
def _rb(m):
    if m.group(1) == ".":
        return m.group(0)
    absf = os.path.normpath(os.path.join(os.path.dirname(mirror), m.group(1)))
    return '#[path = "%s"]' % os.path.relpath(absf, os.path.dirname(newmirror))
cm = re.sub(r'^name = "[^"]+"', 'name = "%s"' % newpkg, mt, count=1, flags=re.M)
cm = re.sub(r'^workspace = "[^"]+"', 'workspace = "%s"' % os.path.relpath(wsabs, newpkgdir), cm, count=1, flags=re.M)
cm = re.sub(r'^description = "([^"]*)"', lambda m: 'description = "%s (%s tests)"' % (m.group(1), suffix), cm, count=1, flags=re.M)
cm = "\n".join(dpath(l) if "path = " in l and not l.startswith("path") else l for l in cm.split("\n"))
cm = re.sub(r'(\[lib\][^\[]*?path = )"[^"]+"', lambda m: m.group(1) + '"%s"' % os.path.relpath(newroot, newpkgdir), cm, count=1)
comp_m_raw = re.sub(r'#\[path = "([^"]+)"\]', lambda m: m.group(0), comp_m)
print(newpkg, "moves", moved, "owner mirror lines", len(owner_m.split("\n")), "comp mirror lines", len(comp_m.split("\n")))
if not write:
    sys.exit(0)
orig = HERE + "/orig/mirrorsplit"
os.makedirs(orig, exist_ok=True)
for f in (mirror, man):
    shutil.copy(f, orig + "/" + os.path.relpath(f, R).replace("/", "__"))
os.makedirs(os.path.dirname(newmirror), exist_ok=True)
open(newroot, "w").write(rt)
open(newmirror, "w").write(re.sub(r'#\[path = "([^"]+)"\]', _rb, comp_m_raw))
open(newpkgdir + "/Cargo.toml", "w").write(cm)
open(mirror, "w").write(owner_m)
wt = open(wsabs + "/Cargo.toml").read()
mem = os.path.relpath(newpkgdir, wsabs); old = os.path.relpath(pkgdir, wsabs)
line = '  "%s",\n' % old
assert line in wt
wt = wt.replace(line, line + '  "%s",\n' % mem, 1)
tbl = "[workspace.dependencies.%s]\npath = \"%s\"\n" % (pkg, old)
if tbl in wt:
    wt = wt.replace(tbl, tbl + "\n[workspace.dependencies.%s]\npath = \"%s\"\n" % (newpkg, mem), 1)
open(wsabs + "/Cargo.toml", "w").write(wt)
op = R + "/📋️build-owners.json"
o = open(op).read()
rrel = os.path.relpath(newpkgdir, R)
row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/Cargo.toml",\n      "packages": []\n    },\n' % (newpkg, rrel, rrel)
names_ = [(x.start(), x.group(1)) for x in re.finditer(r'    \{\n      "name": "([^"]+)"', o)]
assert '"%s"' % newpkg not in o
p_ = next(p for p, n in names_ if n > newpkg)
o = o[:p_] + row + o[p_:]
json.loads(o)
open(op, "w").write(o)
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t%d\n" % (pkg, newpkg, os.path.join(node), len(moved)))
print("written")
