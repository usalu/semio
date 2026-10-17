import json, os, re, sys
# 🧅️ peel2.py <census-name> <module> [--write] [--crate-suffix S] : moves one directly mounted module tree out of a framework crate into a new producer crate rooted at the module's own file.
# The parent keeps the module name as an alias of the new crate (private or public as before), so callers keep their paths; restricted items of the module root are widened to pub.
R = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.dirname(HERE)
PKG = "📦️packages/🦀️rust"
PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]*)"\]')


def die(msg):
    print("ABORT:", msg)
    sys.exit(2)


def snake(s):
    return s.replace("-", "_")


args = sys.argv[1:]
write = "--write" in args
pos = [a for a in args if not a.startswith("--")]
name, mod = pos[0], pos[1]
suffix = args[args.index("--crate-suffix") + 1] if "--crate-suffix" in args else mod.replace("_", "-")
rows = {r["name"]: r for r in json.load(open(F + "/🗑️generated/hdr-near.json"))}
man = R + "/" + rows[name]["manifest"]
mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
lp = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)
root = os.path.normpath(os.path.join(pkgdir, lp))
rdir = os.path.dirname(root)
rtxt = open(root).read()
m = re.search(r'((?:#\[[^\n]*\]\n)*)#\[path\s*=\s*"([^"]+)"\]\n((?:#\[[^\n]*\]\n)*)(pub(?:\([^)]*\))?\s+)?mod\s+%s\s*;\n' % re.escape(mod), rtxt)
if not m:
    die("mount of module %s not found in %s" % (mod, root))
if "cfg(test)" in m.group(0):
    die("module is cfg(test)")
vis = (m.group(4) or "").strip()
mroot = os.path.normpath(os.path.join(rdir, m.group(2)))
mdir = os.path.dirname(mroot)
if os.path.basename(mroot) != "🦀️.rs":
    die("module root file is not a 🦀️.rs: " + mroot)


def closure(f, acc):
    if f in acc or not os.path.isfile(f):
        return
    acc.append(f)
    for mm in PATH_RX.finditer(open(f).read()):
        p = mm.group(1)
        if p != ".":
            closure(os.path.normpath(os.path.join(os.path.dirname(f), p)), acc)


files = []
closure(mroot, files)
siblings = [x.group(1) for x in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', rtxt) if x.group(1) != mod]
texts = {f: open(f).read() for f in files}
# outgoing references
bad = []
for f, t in texts.items():
    for ln, l in enumerate(t.split("\n"), 1):
        if l.strip().startswith("//"):
            continue
        for mm in re.finditer(r"\bcrate::(\w+)", l):
            nm = mm.group(1)
            if nm != mod:
                bad.append((os.path.relpath(f, R)[-60:], ln, "crate::" + nm))
        if f == mroot and re.search(r"\bsuper::", l):
            bad.append((os.path.relpath(f, R)[-60:], ln, "super:: at module root"))
if bad:
    die("outgoing references %s" % bad[:6])
# dependencies used
deps_sec = re.search(r"\[dependencies\]\n((?:[^\[\n][^\n]*\n|\n)*)", mt)
dev_sec = re.search(r"\[dev-dependencies\]\n((?:[^\[\n][^\n]*\n|\n)*)", mt)


def dep_lines(sec):
    return [l for l in (sec.group(1).split("\n") if sec else []) if l.strip() and not l.strip().startswith("#")]


def ident_of(line):
    key = line.split("=")[0].strip().split(".")[0].strip()
    pk = re.search(r'package\s*=\s*"([^"]+)"', line)
    return key, snake(key)


alltext = "\n".join(texts.values())
# extern crate aliases of the parent root
ext = re.findall(r"(?m)^extern crate (\w+) as (\w+);", rtxt)
alias_of = {a: c for c, a in ext}
used_aliases = [a for a in alias_of if re.search(r"\b%s::" % a, alltext) or re.search(r"\b%s\b" % a, alltext) and a in ("dsl", "protocol", "store", "pack", "vcs", "value_derive", "framework_schema")]
need_ext = [(c, a) for c, a in ext if a in used_aliases]
new_deps, new_dev = [], []
for line in dep_lines(deps_sec):
    key, ident = ident_of(line)
    if re.search(r"\b%s\b" % re.escape(ident), alltext) or any(c == ident for c, a in need_ext):
        new_deps.append(line)
for line in dep_lines(dev_sec):
    key, ident = ident_of(line)
    if re.search(r"\b%s\b" % re.escape(ident), alltext):
        new_dev.append(line)
# names
base_id = re.search(r'^id = "([^"]+)"', mt, re.M)
newpkg = "%s-%s" % (pkg, suffix)
newid = ("%s-%s" % (base_id.group(1), suffix)) if base_id else None
newpkgdir = os.path.join(mdir, PKG)
if os.path.exists(newpkgdir):
    die("exists " + newpkgdir)
wsrel = re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)
wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))


def rebase_dep(line):
    mm = re.search(r'path\s*=\s*"([^"]+)"', line)
    if not mm:
        return line
    tgt = os.path.normpath(os.path.join(pkgdir, mm.group(1)))
    return line.replace(mm.group(0), 'path = "%s"' % os.path.relpath(tgt, newpkgdir))


desc = re.search(r'^description = "([^"]*)"', mt, re.M)
newm = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n' % (os.path.relpath(wsabs, newpkgdir), newpkg)
        + 'description = "%s - the %s module"\n\n[package.metadata.semio]\nrole = "%s"\n%s\n' % ((desc.group(1) if desc else pkg).rstrip("."), mod, re.search(r'^role = "([^"]+)"', mt, re.M).group(1) if re.search(r'^role = "([^"]+)"', mt, re.M) else "framework", ('id = "%s"\n' % newid) if newid else "")
        + ("[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n" if newid else "") + "[lints]\nworkspace = true\n\n[lib]\npath = \"../../🦀️.rs\"\n\n"
        + "[dependencies]\n" + "".join(rebase_dep(l) + "\n" for l in new_deps) + "\n[dev-dependencies]\n" + "".join(rebase_dep(l) + "\n" for l in new_dev))
# module root edits: widen restricted items, rewrite crate::<mod>:: paths, add extern crate aliases
newroot = texts[mroot]
newroot = re.sub(r"(?m)^(\s*)pub\((?:crate|super|in [^)]+)\)\s+", r"\1pub ", newroot)
for f in files:
    t = texts[f]
    t = re.sub(r"\bcrate::%s::" % re.escape(mod), "crate::", t)
    t = re.sub(r"\bcrate::%s\b" % re.escape(mod), "crate", t)
    if f == mroot:
        t = re.sub(r"(?m)^(\s*)pub\((?:crate|super|in [^)]+)\)\s+", r"\1pub ", t)
        if need_ext:
            exts = "".join("extern crate %s as %s;\n" % (c, a) for c, a in need_ext)
            lines = t.split("\n")
            at = 0
            while at < len(lines) and (lines[at].startswith("//!") or lines[at].startswith("#![") or not lines[at].strip()):
                at += 1
            lines.insert(at, exts.rstrip("\n"))
            t = "\n".join(lines)
    texts[f] = t
print("%s: module %s %d files %d bytes -> %s; ext aliases %s; deps %d dev %d; vis=%r" % (pkg, mod, len(files), sum(len(t) for t in texts.values()), newpkg, [a for c, a in need_ext], len(new_deps), len(new_dev), vis))
if not write:
    sys.exit(0)
os.makedirs(newpkgdir)
open(newpkgdir + "/Cargo.toml", "w").write(newm)
if newid:
    open(newpkgdir + "/📋️project.json", "w").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % newid)
for f in files:
    open(f, "w").write(texts[f])
# parent root: replace the mount by an alias of the new crate
alias = "%suse %s as %s;\n" % ((vis + " ") if vis else "", snake(newpkg), mod)
rtxt2 = rtxt.replace(m.group(0), alias)
open(root, "w").write(rtxt2)
# parent manifest: dependency
line = '%s = { path = "%s" }\n' % (newpkg, os.path.relpath(newpkgdir, pkgdir))
dm = re.search(r"^\[dependencies\]\n", mt, re.M)
mt2 = mt[:dm.end()] + line + mt[dm.end():]
open(man, "w").write(mt2)
# workspace member, owner row, routes
ws_toml = wsabs + "/Cargo.toml"
wt = open(ws_toml).read()
mrx = re.search(r'members = \[\n((?:\s+"[^"]+",\n)+)\s*\]', wt)
entries = re.findall(r'"([^"]+)"', mrx.group(1))
indent = re.match(r"(\s*)\"", mrx.group(1)).group(1)
oldm = os.path.relpath(pkgdir, wsabs)
newm_rel = os.path.relpath(newpkgdir, wsabs)
if oldm in entries:
    entries.insert(entries.index(oldm) + 1, newm_rel)
else:
    entries.append(newm_rel)
wt = wt[:mrx.start(1)] + "".join('%s"%s",\n' % (indent, e) for e in entries) + wt[mrx.end(1):]
open(ws_toml, "w").write(wt)
otxt = open(R + "/📋️build-owners.json").read()
oname = ("@semio-tech/%s-rs" % newid) if newid else newpkg
rrel = os.path.relpath(newpkgdir, R)
row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/%s",\n      "packages": []\n    },\n' % (oname, rrel, rrel, "📋️project.json" if newid else "Cargo.toml")
assert '"%s"' % oname not in otxt
names_ = [(x.start(), x.group(1)) for x in re.finditer(r'    \{\n      "name": "([^"]+)"', otxt)]
at = next(p for p, nme in names_ if nme > oname)
otxt = otxt[:at] + row + otxt[at:]
json.loads(otxt)
open(R + "/📋️build-owners.json", "w").write(otxt)
if not newid:
    open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t0\n" % (pkg, newpkg, os.path.relpath(mdir, R)))
    print("written", newpkg)
    sys.exit(0)
ridx = HERE + "/route-index.txt"
k = int(open(ridx).read())
open(ridx, "w").write(str(k + 1))
a_, b_ = 2 * k - 1, 2 * k
ltxt = open(R + "/.vscode/launch.json").read()


def route(label, target, conf, group, order):
    return ('    {\n      "name": "🦀️ %s %s",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/%s-rs:%s --configuration=%s --parallel=1 -- --lib --offline",\n      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "%s",\n        "order": %s\n      }\n    },\n' % (newpkg, label, newid, target, conf, group, order))


routes = "".join([route("🏗️build 🛠️prototype", "build", "prototype", "4_build", "900.96%03d" % a_), route("🏗️build 🚀️release", "build", "release", "4_build", "900.96%03d" % b_),
                  route("🔎️check 🛠️prototype", "check", "prototype", "4_gate", "922.96%03d" % a_), route("🔎️check 🚀️release", "check", "release", "4_gate", "922.96%03d" % b_),
                  route("🧪️test 🛠️prototype", "test", "prototype", "4_test", "903.96%03d" % a_), route("🧪️test 🚀️release", "test", "release", "4_test", "903.96%03d" % b_)])
last = [x.end() for x in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n' % re.escape(pkg), ltxt)]
if last:
    ltxt = ltxt[:last[-1]] + routes + ltxt[last[-1]:]
else:
    print("note: no existing routes of the parent; routes appended after the last 4_test route")
    anchor = list(re.finditer(r'    \},\n', ltxt))
    die("parent routes not found")
json.loads(ltxt)
open(R + "/.vscode/launch.json", "w").write(ltxt)
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t0\n" % (pkg, newpkg, os.path.relpath(mdir, R)))
print("written", newpkg)
