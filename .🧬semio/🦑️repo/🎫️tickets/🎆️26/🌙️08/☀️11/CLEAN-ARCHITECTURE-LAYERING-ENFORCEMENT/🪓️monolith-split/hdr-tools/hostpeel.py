import json, os, re, shutil, sys
# 🧅️ hostpeel.py <host-manifest> <module-dir-name> <crate-suffix> [--consts A,B] [--testkit] [--write] : peels one `use super::*` child module of a hub root into its own artifact producer crate rooted at the module's own file.
# The child gets the parent's pruned `use` lines and extern crates instead of `super::*`; restricted items widen to pub; the parent aliases the new crate under the old module name; `cfg(test)` items become `cfg(any(test, feature = "testkit"))` when --testkit.
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
write = "--write" in args
tk = "--testkit" in args
consts = []
if "--consts" in args:
    consts = args[args.index("--consts") + 1].split(",")
pos = [a for a in args if not a.startswith("--") and a not in consts]
man = os.path.join(R, pos[0])
mdir = pos[1]
suffix = pos[2]
mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
root = os.path.normpath(os.path.join(pkgdir, re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)))
rdir = os.path.dirname(root)
rt = open(root).read()
mfile = os.path.join(rdir, mdir, "🦀️.rs")
mt_ = open(mfile).read()
newpkg = "%s-%s" % (pkg, suffix)
ident = newpkg.replace("-", "_")
modname = suffix.replace("-", "_")
newpkgdir = os.path.join(rdir, mdir, PKG)
mount_rx = re.compile(r'#\[path = "%s/🦀️\.rs"\]\n(?:pub )?mod (\w+);\n' % re.escape(mdir))
m = mount_rx.search(rt)
assert m, "mount not found"
alias = m.group(1)
assert "use super::*;" in mt_
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
def prune_use(line, text):
    bm = re.match(r"^use \{(.*)\};$", line)
    if bm:
        keep = [it for it in split_items(bm.group(1)) if it.endswith(" as _") or used(it.split(" as ")[-1].split("::")[-1].strip(), text)]
        return (pre if "pre" in dir() else "") + "use {%s};" % ", ".join(keep) if keep else None
    mm = re.match(r"^use (.*?)::\{(.*)\};$", line)
    def used(nm): return re.search(r"\b%s\b" % re.escape(nm), text) is not None
    if mm:
        keep = []
        for it in split_items(mm.group(2)):
            if it.endswith(" as _"): keep.append(it); continue
            nm = it.split(" as ")[-1].split("::")[-1].strip()
            if used(nm): keep.append(it)
        return "use %s::{%s};" % (mm.group(1), ", ".join(keep)) if keep else None
    mm = re.match(r"^use (.*?)(?: as (\w+|_))?;$", line)
    if mm:
        if mm.group(2) == "_": return line
        nm = mm.group(2) or mm.group(1).split("::")[-1]
        return line if used(nm) else None
    return line
body = mt_.replace("use super::*;\n", "", 1)
root_uses = [l for l in rt.split("\n") if l.startswith("use ") and not re.match(r"use \w+::\*;$", l) and " as " not in l.split("::")[0] or (l.startswith("use ") and l.endswith("as _;"))]
root_uses = [l for l in rt.split("\n") if l.startswith("use ") and not re.match(r"^use \w+::\*;$", l) and not re.match(r"^use semio_[a-z0-9_]+ as \w+;$", l)]
kept = [p for p in (prune_use(l, body) for l in root_uses) if p]
ext = [l for l in rt.split("\n") if l.startswith("extern crate ")]
ext = [l for l in ext if re.search(r"\b%s::" % re.search(r"as (\w+);", l).group(1), body)]
widened = re.sub(r"(?m)^(\s*)pub\((?:crate|super|in [^)]+)\)\s+", r"\1pub ", body)
if tk:
    widened = widened.replace("#[cfg(test)]", '#[cfg(any(test, feature = "testkit"))]')
const_lines = []
for c in consts:
    cm = re.search(r"(?m)^((?:///[^\n]*\n)*)(?:pub )?const %s: [^\n]*\n" % c, rt)
    assert cm, c
    const_lines.append("pub " + re.sub(r"^(?:pub )?", "", cm.group(0).split("\n")[-2] if cm.group(1) else cm.group(0).rstrip("\n")) )
newtext_head = re.match(r"((?://![^\n]*\n)+)", widened)
doc = newtext_head.group(1) if newtext_head else "//! 🧅️ %s\n" % suffix
widened = widened[len(doc):] if newtext_head else widened
newtext = doc + "\n" + "".join(l + "\n" for l in ext) + "".join(l + "\n" for l in kept) + "\n" + "".join(c + "\n\n" for c in const_lines) + widened.lstrip("\n")
text_all = newtext
wsrel = re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)
wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))
head, rest = mt.split("[dependencies]\n", 1)
deps_sec, dev_sec = rest.split("[dev-dependencies]\n", 1) if "[dev-dependencies]" in rest else (rest, "")
def rebase(l):
    pm = re.search(r'path = "([^"]+)"', l)
    if not pm: return l
    tgt = os.path.normpath(os.path.join(pkgdir, pm.group(1)))
    return l.replace(pm.group(0), 'path = "%s"' % os.path.relpath(tgt, newpkgdir))
def keep_dep(l):
    mm = re.match(r"^([\w\-]+)\s*=", l)
    if not mm: return False
    names = {mm.group(1).replace("-", "_")}
    p = re.search(r'package = "([^"]+)"', l)
    if p: names.add(p.group(1).replace("-", "_"))
    return any(re.search(r"\b%s\b" % re.escape(n), text_all) for n in names)
deps = [rebase(l) for l in deps_sec.split("\n") if l.strip() and keep_dep(l)]
desc = "%s: %s" % (re.search(r'description = "([^"]*)"', mt).group(1), suffix)
newman = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n'
          % (os.path.relpath(wsabs, newpkgdir), newpkg, desc)) + ("[features]\ntestkit = []\n\n" if tk else "") + "[dependencies]\n" + "".join(l + "\n" for l in deps)
print(newpkg, "uses kept", len(kept), "of", len(root_uses), "ext", len(ext), "deps", len(deps), "consts", consts, "lines", len(newtext.split("\n")))
if not write:
    sys.exit(0)
orig = HERE + "/orig/hostpeel"
os.makedirs(orig, exist_ok=True)
for f in (root, mfile, man):
    shutil.copy(f, orig + "/" + os.path.relpath(f, R).replace("/", "__"))
os.makedirs(newpkgdir)
open(newpkgdir + "/Cargo.toml", "w").write(newman)
open(mfile, "w").write(newtext)
newroot = rt.replace(m.group(0), "use %s as %s;\n" % (ident, alias))
for c in consts:
    newroot = re.sub(r"(?m)^((?:///[^\n]*\n)*)(?:pub )?const %s: [^\n]*\n\n?" % c, "", newroot, count=1)
open(root, "w").write(newroot)
dep_line = "%s = { workspace = true }\n" % newpkg
dm = re.search(r"^\[dependencies\]\n", mt, re.M)
mt2 = mt[:dm.end()] + dep_line + mt[dm.end():]
if tk:
    fm = re.search(r"^testkit = \[([^\]]*)\]", mt2, re.M)
    items = [x.strip() for x in fm.group(1).split(",") if x.strip()]
    items.append('"%s/testkit"' % newpkg)
    mt2 = mt2.replace(fm.group(0), "testkit = [%s]" % ", ".join(items), 1)
    dv = re.search(r"^\[dev-dependencies\]\n", mt2, re.M)
    mt2 = mt2[:dv.end()] + '%s = { workspace = true, features = ["testkit"] }\n' % newpkg + mt2[dv.end():]
open(man, "w").write(mt2)
wt = open(wsabs + "/Cargo.toml").read()
mem = os.path.relpath(newpkgdir, wsabs)
old = os.path.relpath(pkgdir, wsabs)
line = '  "%s",\n' % old
assert line in wt
wt = wt.replace(line, line + '  "%s",\n' % mem, 1)
tbl = "[workspace.dependencies.%s]\npath = \"%s\"\n" % (pkg, old)
assert tbl in wt
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
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t1\n" % (pkg, newpkg, os.path.relpath(os.path.dirname(newpkgdir.rstrip("/")).rsplit("/",1)[0], R)))
print("written")
