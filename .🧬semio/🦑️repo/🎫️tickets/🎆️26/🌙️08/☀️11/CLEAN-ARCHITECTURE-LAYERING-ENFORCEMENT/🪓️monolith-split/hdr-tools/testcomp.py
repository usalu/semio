import json, os, re, shutil, sys
# 🧪️ testcomp.py <owner-manifest> <test-mod> <suffix> [--access fn,fn] [--write] : moves one white-box `use super::*` test module of a framework hub into a companion test crate rooted at the test file itself.
# The owner gains `testkit = []` and a `cfg(any(test, feature = "testkit"))` module of thin accessors for the named private fns; the test file imports the owner's public API, the accessors and the pruned `use` list of the owner root.
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
write = "--write" in args
acc = args[args.index("--access") + 1].split(",") if "--access" in args else []
pos = [a for a in args if not a.startswith("--") and a not in acc]
man = os.path.join(R, pos[0]); tmod = pos[1]; suffix = pos[2]
mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
root = os.path.normpath(os.path.join(pkgdir, re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)))
rt = open(root).read()
m = re.search(r'#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\nmod %s;\n' % re.escape(tmod), rt)
assert m, "mount"
tfile = os.path.normpath(os.path.join(os.path.dirname(root), m.group(1)))
tt = open(tfile).read()
assert "use super::*;" in tt
newpkgdir = os.path.join(os.path.dirname(tfile), PKG)
newpkg = "%s-%s" % (pkg, suffix)
ident = pkg.replace("-", "_")
wrappers = ""
for fn in acc:
    sm = re.search(r"(?m)^(?:pub(?:\([^)]*\))? )?fn %s\b[^{]*\{" % re.escape(fn), rt)
    assert sm, fn
    sig = sm.group(0)[:-1].strip()
    sig = re.sub(r"^(?:pub(?:\([^)]*\))? )?fn", "pub fn", sig)
    open_i = sig.index("(")
    depth = 0
    for k in range(open_i, len(sig)):
        if sig[k] == "(": depth += 1
        if sig[k] == ")":
            depth -= 1
            if depth == 0:
                close_i = k
                break
    params = sig[open_i + 1:close_i]
    depth = 0; parts = []; cur = ""
    for ch in params:
        if ch in "<([": depth += 1
        if ch in ">)]": depth -= 1
        if ch == "," and depth == 0: parts.append(cur); cur = ""
        else: cur += ch
    if cur.strip(): parts.append(cur)
    names = [re.sub(r"^mut\s+", "", p.split(":")[0].strip()) for p in parts]
    wrappers += "    %s {\n        super::%s(%s)\n    }\n" % (sig, fn, ", ".join(names))
def split_items(inner):
    out, depth, cur = [], 0, ""
    for ch in inner:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0: out.append(cur.strip()); cur = ""
        else: cur += ch
    if cur.strip(): out.append(cur.strip())
    return out
def used(nm, text): return re.search(r"\b%s\b" % re.escape(nm), text) is not None
def prune(line, text):
    if line.startswith("#[cfg(test)] "): line = line[len("#[cfg(test)] "):]
    bm = re.match(r"^use \{(.*)\};$", line)
    if bm:
        keep = [it for it in split_items(bm.group(1)) if it.endswith(" as _") or used(it.split(" as ")[-1].split("::")[-1].strip(), text)]
        return (pre if "pre" in dir() else "") + "use {%s};" % ", ".join(keep) if keep else None
    mm = re.match(r"^(?:pub(?:\([^)]*\))? )?use (.*?)::\{(.*)\};$", line)
    if mm:
        keep = [it for it in split_items(mm.group(2)) if it.endswith(" as _") or used(it.split(" as ")[-1].split("::")[-1].strip(), text)]
        return "use %s::{%s};" % (mm.group(1), ", ".join(keep)) if keep else None
    mm = re.match(r"^(?:pub(?:\([^)]*\))? )?use (.*?)(?: as (\w+|_))?;$", line)
    if mm:
        if mm.group(2) == "_": return line
        nm = mm.group(2) or mm.group(1).split("::")[-1]
        return line if used(nm, text) and "*" not in nm else None
    return None
body = tt.replace("use super::*;\n", "", 1)
root_uses = [l for l in rt.split("\n") if re.match(r"^(#\[cfg\(test\)\] )?(pub(\([^)]*\))? )?use ", l)]
kept = [p for p in (prune(l, body) for l in root_uses) if p]
own = set()
for ul in re.findall(r"(?m)^use [^;]*;", body):
    for nm in re.findall(r"\b(\w+)\b(?=\s*[,}]|\s*;)", ul):
        own.add(nm)
def dedupe(line):
    mm = re.match(r"^use (.*?)::\{(.*)\};$", line)
    if mm:
        keep = [it for it in split_items(mm.group(2)) if it.split(" as ")[-1].split("::")[-1].strip() not in own]
        return "use %s::{%s};" % (mm.group(1), ", ".join(keep)) if keep else None
    mm = re.match(r"^use (.*?)(?: as (\w+|_))?;$", line)
    if mm and (mm.group(2) or mm.group(1).split("::")[-1]) in own: return None
    return line
kept = [x for x in (dedupe(l) for l in kept) if x]
kept = list(dict.fromkeys(kept))
ext = [l for l in rt.split("\n") if l.startswith("extern crate ") and re.search(r"\b%s::" % re.search(r"as (\w+);", l).group(1), body + "".join(kept))]
docm = re.match(r"((?://![^\n]*\n)*)", body)
doc = docm.group(1)
rest = body[len(doc):]
newtest = doc + "#![cfg(test)]\n#![allow(unused_imports)]\n\nuse %s::*;\n%s%s%s\n" % (ident, ("use %s::testkit::*;\n" % ident) if acc else "", "".join(l + "\n" for l in ext), "".join(l + "\n" for l in kept)) + rest
wsrel = re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)
wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))
head, tail = mt.split("[dependencies]\n", 1)
deps_sec, dev_sec = (tail.split("[dev-dependencies]\n", 1) + [""])[:2]
def rebase(l):
    pm = re.search(r'path = "([^"]+)"', l)
    if not pm: return l
    return l.replace(pm.group(0), 'path = "%s"' % os.path.relpath(os.path.normpath(os.path.join(pkgdir, pm.group(1))), newpkgdir))
def keep_dep(l):
    mm = re.match(r"^([\w\-]+)(?:\.workspace)?\s*=", l)
    if not mm: return False
    names = {mm.group(1).replace("-", "_")}
    p = re.search(r'package = "([^"]+)"', l)
    if p: names.add(p.group(1).replace("-", "_"))
    return any(used(n, newtest) for n in names)
devs = [rebase(l) for l in (deps_sec + "\n" + dev_sec).split("\n") if l.strip() and keep_dep(l) and not l.startswith("[")]
devs = list(dict.fromkeys(devs))
owner_line = ('%s = { path = "%s", features = ["testkit"] }' if acc else '%s = { path = "%s" }') % (pkg, os.path.relpath(pkgdir, newpkgdir))
desc = "Companion white-box tests of %s: %s" % (pkg, suffix)
owner_role = (re.search(r'role = "([^"]+)"', mt) or [None, 'library'])[1]
cm = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[lints]\nworkspace = true\n\n[package.metadata.semio]\nrole = "%s"\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n\n[dev-dependencies]\n'
      % (os.path.relpath(wsabs, newpkgdir), newpkg, desc, owner_role)) + owner_line + "\n" + "".join(l + "\n" for l in devs if not l.startswith(pkg + " ") and not l.startswith(pkg + "."))
print(newpkg, "uses", len(kept), "of", len(root_uses), "dev", len(devs), "wrappers", acc, "test lines", len(newtest.split("\n")))
if not write:
    sys.exit(0)
orig = HERE + "/orig/testcomp"
os.makedirs(orig, exist_ok=True)
for f in (root, tfile, man):
    shutil.copy(f, orig + "/" + os.path.relpath(f, R).replace("/", "__"))
os.makedirs(newpkgdir)
open(newpkgdir + "/Cargo.toml", "w").write(cm)
open(tfile, "w").write(newtest)
nr = rt.replace(m.group(0), "", 1)
if acc:
    nr = nr.rstrip("\n") + '\n\n#[cfg(any(test, feature = "testkit"))]\npub mod testkit {\n    use super::*;\n' + wrappers + "}\n"
open(root, "w").write(nr)
if acc:
    if "[features]" in mt:
        mt2 = mt.replace("[features]\n", "[features]\ntestkit = []\n", 1) if "testkit" not in mt else mt
    else:
        mt2 = mt.replace("[dependencies]\n", "[features]\ntestkit = []\n\n[dependencies]\n", 1)
    open(man, "w").write(mt2)
wt = open(wsabs + "/Cargo.toml").read()
mem = os.path.relpath(newpkgdir, wsabs); old = os.path.relpath(pkgdir, wsabs)
line = '    "%s",\n' % old
if line not in wt: line = '  "%s",\n' % old
assert line in wt, old
wt = wt.replace(line, line + line.replace(old, mem), 1)
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
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t1\n" % (pkg, newpkg, os.path.relpath(os.path.dirname(tfile), R)))
print("written")
