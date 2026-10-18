import json, os, re, shutil, sys
# 🧱️ clumppeel.py <manifest> <new-dir> <suffix> <mod,mod..> [--items fn,fn] [--testkit] [--write] : moves a clump of mutually coupled `use super::*` hub children (and chosen root fns) of a hub root into one new artifact producer crate.
# The new root repeats the pruned `use` list and mounts the children in place (paths rebased), so `use super::*` keeps working inside the clump; the old root re-exports the crate with a glob and keeps every `crate::` path alive.
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
write = "--write" in args
tk = "--testkit" in args
items = args[args.index("--items") + 1].split(",") if "--items" in args else []
lifts = args[args.index("--lift") + 1].split(",") if "--lift" in args else []
consts = args[args.index("--consts") + 1].split(",") if "--consts" in args else []
precise = "--precise" in args
pos = [a for a in args if not a.startswith("--") and a not in items and a not in lifts and a not in consts]
man = os.path.join(R, pos[0]); newname = pos[1]; suffix = pos[2]; mods = pos[3].split(",")
mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
root = os.path.normpath(os.path.join(pkgdir, re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)))
rdir = os.path.dirname(root)
rt = open(root).read()
newdir = os.path.join(rdir, newname)
newpkgdir = os.path.join(newdir, PKG)
newpkg = "%s-%s" % (pkg, suffix)
ident = newpkg.replace("-", "_")
assert not os.path.exists(newdir), newdir
blocks = []
files = []
for mod in mods:
    rx = re.compile(r'(?:#\[cfg\([^\n]*\)\]\n)?#\[path = "([^"]+)"\]\n(pub )?mod %s;\n((?:pub )?use %s::\*;\n)?' % (re.escape(mod), re.escape(mod)))
    m = rx.search(rt)
    assert m, mod
    blocks.append((mod, m))
    files.append(os.path.normpath(os.path.join(rdir, m.group(1))))
moved_items = []
for fn in items:
    im = re.search(r"(?m)^((?:///[^\n]*\n)*)pub fn %s\b" % re.escape(fn), rt)
    assert im, fn
    start = im.start()
    depth = 0; i = rt.index("{", im.end()); j = i
    while True:
        if rt[j] == "{": depth += 1
        if rt[j] == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    moved_items.append((start, j + 1, rt[start:j + 1]))
const_items = []
for c in consts:
    cm = re.search(r"(?m)^((?:///[^\n]*\n)*)(?:pub )?const %s\b[^\n]*;\n" % re.escape(c), rt)
    assert cm, c
    txt = cm.group(0)
    moved_items.append((cm.start(), cm.end(), txt))
    const_items.append((c, txt))
lifted = []
for spec in lifts:
    fn, modn = spec.split(":")
    mm = re.search(r'#\[path = "([^"]+)"\]\n(?:pub )?mod %s;' % re.escape(modn), rt)
    lf = os.path.normpath(os.path.join(rdir, mm.group(1)))
    lt = open(lf).read()
    im = re.search(r"(?m)^((?:///[^\n]*\n)*)pub fn %s\b" % re.escape(fn), lt)
    assert im, spec
    depth = 0; i = lt.index("{", im.end()); j = i
    while True:
        if lt[j] == "{": depth += 1
        if lt[j] == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    lifted.append((lf, lt[im.start():j + 1]))
    moved_items.append((0, 0, lt[im.start():j + 1]))
body_text = "".join(open(f).read() for f in files) + "".join(x[2] for x in moved_items)
def split_items(inner):
    out, depth, cur = [], 0, ""
    for ch in inner:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip(): out.append(cur.strip())
    return out
def used(nm, text): return re.search(r"\b%s\b" % re.escape(nm), text) is not None
def prune(line, text):
    pre = ""
    if line.startswith("#[cfg(test)] "):
        pre = '#[cfg(any(test, feature = "testkit"))] ' if tk else "#[cfg(test)] "
        line = line[len("#[cfg(test)] "):]
    bm = re.match(r"^use \{(.*)\};$", line)
    if bm:
        keep = [it for it in split_items(bm.group(1)) if it.endswith(" as _") or used(it.split(" as ")[-1].split("::")[-1].strip(), text)]
        return (pre if "pre" in dir() else "") + "use {%s};" % ", ".join(keep) if keep else None
    mm = re.match(r"^use (.*?)::\{(.*)\};$", line)
    if mm:
        keep = [it for it in split_items(mm.group(2)) if it.endswith(" as _") or used(it.split(" as ")[-1].split("::")[-1].strip(), text)]
        return pre + "use %s::{%s};" % (mm.group(1), ", ".join(keep)) if keep else None
    mm = re.match(r"^use (.*?)(?: as (\w+|_))?;$", line)
    if mm:
        if mm.group(2) == "_": return pre + line
        nm = mm.group(2) or mm.group(1).split("::")[-1]
        return pre + line if used(nm, text) else None
    return pre + line
raw_uses = [l for l in rt.split("\n") if (l.startswith("use ") or l.startswith("#[cfg(test)] use ")) and not re.match(r"^use \w+::\*;$", l) and not re.match(r"^use semio_[a-z0-9_]+ as \w+;$", l)]
kept = [p for p in (prune(l, body_text) for l in raw_uses) if p]
ext = [l for l in rt.split("\n") if l.startswith("extern crate ") and re.search(r"\b%s::" % re.search(r"as (\w+);", l).group(1), body_text + "".join(kept))]
inner = [l for l in rt.split("\n") if l.startswith("#![")]
mount_rel = lambda f: os.path.relpath(f, newdir)
mounts = ""
gated = []
for (mod, m), f in zip(blocks, files):
    if m.group(0).startswith("#[cfg"):
        gate = m.group(0).split("\n")[0]
        mounts += '%s\n#[path = "%s"]\npub mod %s;\n' % (gate, mount_rel(f), mod)
        gated.append((gate, mod))
        continue
    mounts += '#[path = "%s"]\n%smod %s;\n%s' % (mount_rel(f), "pub " if precise else "", mod, ("pub use %s::*;\n" % mod) if m.group(3) and m.group(3).startswith("pub") else ("use %s::*;\n" % mod if m.group(3) else ""))
newroot = ("//! 🧱️ %s of `%s`: the mutually coupled modules %s.\n\n" % (suffix, pkg, ", ".join("`%s`" % x for x in mods))
           + "".join(l + "\n" for l in inner) + "".join(l + "\n" for l in ext) + "".join(l + "\n" for l in kept) + "\n"
           + "".join(re.sub(r"(?m)^(?:pub )?const ", "pub const ", x[2]) + "\n\n" for x in moved_items) + mounts)
wsrel = re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)
wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))
head, rest = mt.split("[dependencies]\n", 1)
deps_sec = rest.split("[dev-dependencies]\n", 1)[0]
def rebase(l):
    pm = re.search(r'path = "([^"]+)"', l)
    if not pm: return l
    return l.replace(pm.group(0), 'path = "%s"' % os.path.relpath(os.path.normpath(os.path.join(pkgdir, pm.group(1))), newpkgdir))
alltext = newroot + body_text
def keep_dep(l):
    mm = re.match(r"^([\w\-]+)\s*=", l)
    if not mm: return False
    names = {mm.group(1).replace("-", "_")}
    p = re.search(r'package = "([^"]+)"', l)
    if p: names.add(p.group(1).replace("-", "_"))
    return any(re.search(r"\b%s\b" % re.escape(n), alltext) for n in names)
deps = [rebase(l) for l in deps_sec.split("\n") if l.strip() and keep_dep(l)]
desc = "%s: %s" % (re.search(r'description = "([^"]*)"', mt).group(1), suffix) if re.search(r'description = "([^"]*)"', mt) else suffix
manifest = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n'
            % (os.path.relpath(wsabs, newpkgdir), newpkg, desc)) + ("[features]\ntestkit = []\n\n" if tk else "") + "[dependencies]\n" + "".join(l + "\n" for l in deps)
print(newpkg, "files", len(files), "uses", len(kept), "of", len(raw_uses), "deps", len(deps), "root lines", len(newroot.split("\n")))
if not write:
    sys.exit(0)
orig = HERE + "/orig/clump"
os.makedirs(orig, exist_ok=True)
for f in [root, man] + files + [x[0] for x in lifted]:
    shutil.copy(f, orig + "/" + os.path.relpath(f, R).replace("/", "__"))
os.makedirs(newpkgdir)
open(newdir + "/🦀️.rs", "w").write(newroot)
open(newpkgdir + "/Cargo.toml", "w").write(manifest)
for f in files:
    t = open(f).read()
    t = re.sub(r"(?m)^(\s*)pub\((?:crate|super|in [^)]+)\)\s+", r"\1pub ", t)
    open(f, "w").write(t)
nr = rt
for mod, m in blocks:
    nr = nr.replace(m.group(0), "", 1)
for s, e, txt in sorted(moved_items, reverse=True):
    if e:
        nr = nr.replace(txt, "", 1)
for lf, txt in lifted:
    x = open(lf).read()
    assert txt in x
    open(lf, "w").write(x.replace(txt, "", 1))
first = re.search(r"(?m)^#\[path", nr)
if precise:
    reexp = "".join(("pub use %s::%s::*;\n" if (m.group(3) and m.group(3).startswith("pub")) else "use %s::%s::*;\n") % (ident, mod) for mod, m in blocks if m.group(3)) + ("use %s::{%s};\n" % (ident, ", ".join(c for c, _ in const_items)) if const_items else "")
else:
    reexp = "pub use %s::*;\n" % ident
reexp = reexp + "".join("%s\npub use %s::%s;\n" % (g, ident, mn) for g, mn in gated)
nr = nr.replace("//#region 🪢️TaxonomyMounts", reexp + "\n//#region 🪢️TaxonomyMounts", 1) if "//#region 🪢️TaxonomyMounts" in nr else nr + "\n" + reexp
open(root, "w").write(nr)
dm = re.search(r"^\[dependencies\]\n", mt, re.M)
mt2 = mt[:dm.end()] + "%s = { workspace = true }\n" % newpkg + mt2 if False else mt[:dm.end()] + "%s = { workspace = true }\n" % newpkg + mt[dm.end():]
if tk:
    fm = re.search(r"^testkit = \[([^\]]*)\]", mt2, re.M)
    its = [x.strip() for x in fm.group(1).split(",") if x.strip()] if fm else []
    its.append('"%s/testkit"' % newpkg)
    if fm: mt2 = mt2.replace(fm.group(0), "testkit = [%s]" % ", ".join(its), 1)
    dv = re.search(r"^\[dev-dependencies\]\n", mt2, re.M)
    mt2 = mt2[:dv.end()] + '%s = { workspace = true, features = ["testkit"] }\n' % newpkg + mt2[dv.end():]
open(man, "w").write(mt2)
wt = open(wsabs + "/Cargo.toml").read()
mem = os.path.relpath(newpkgdir, wsabs); old = os.path.relpath(pkgdir, wsabs)
line = '  "%s",\n' % old
assert line in wt
wt = wt.replace(line, line + '  "%s",\n' % mem, 1)
tbl = "[workspace.dependencies.%s]\npath = \"%s\"\n" % (pkg, old)
assert tbl in wt, tbl
wt = wt.replace(tbl, tbl + "\n[workspace.dependencies.%s]\npath = \"%s\"\n" % (newpkg, mem), 1)
open(wsabs + "/Cargo.toml", "w").write(wt)
op = R + "/📋️build-owners.json"
o = open(op).read()
rrel = os.path.relpath(newpkgdir, R)
row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/Cargo.toml",\n      "packages": []\n    },\n' % (newpkg, rrel, rrel)
names = [(x.start(), x.group(1)) for x in re.finditer(r'    \{\n      "name": "([^"]+)"', o)]
assert '"%s"' % newpkg not in o
p_ = next(p for p, n in names if n > newpkg)
o = o[:p_] + row + o[p_:]
json.loads(o)
open(op, "w").write(o)
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t%d\n" % (pkg, newpkg, os.path.relpath(newdir, R), len(files)))
print("written")
