import json, os, re, sys
# 🥣️ mkcomp2.py <manifest> <new-dir-name> <mod-name>... [--write] [--dir-parent P] : moves cfg(test) `#[path]` test mounts of a crate root into a new companion test crate that depends on the crate.
# The moved files stay in place; their `crate::` paths become the owner crate's path. Registers the companion in the artifact workspace and the owner rows (artifact style, no routes).
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
args = sys.argv[1:]
write = "--write" in args
dir_parent = args[args.index("--dir-parent") + 1] if "--dir-parent" in args else None
emoji = args[args.index("--emoji") + 1] if "--emoji" in args else "🥣"
pos = [a for a in args if not a.startswith("--") and a != dir_parent and a != emoji]
man = os.path.join(R, pos[0])
dirname_ = pos[1]
mods = pos[2:]


def die(m):
    print("ABORT:", m)
    sys.exit(2)


mt = open(man).read()
pkg = re.search(r'^name = "([^"]+)"', mt, re.M).group(1)
pkgdir = os.path.dirname(man)
lp = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)
root = os.path.normpath(os.path.join(pkgdir, lp))
rdir = os.path.dirname(root)
rt = open(root).read()
ident = pkg.replace("-", "_")
blocks = []
for name in mods:
    m = re.search(r'#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\n(?:pub(?:\([^)]*\))? )?mod %s;\n' % re.escape(name), rt)
    if not m:
        die("mount of %s not found" % name)
    blocks.append((name, m.group(1), m.group(0)))
base = dir_parent and os.path.join(R, dir_parent) or os.path.join(rdir, "🧪️tests")
newdir = os.path.join(base, "%s️%s" % (emoji, dirname_))
if os.path.exists(newdir):
    die("exists " + newdir)
newpkgdir = os.path.join(newdir, PKG)
texts = {}
aliases = set()
for name, p, blk in blocks:
    f = os.path.normpath(os.path.join(rdir, p))
    t = open(f).read()
    if re.search(r"\bsuper::|\bself::", t):
        die("super/self reference in " + f)
    for a in ("dsl", "protocol", "store", "framework_schema", "value_derive"):
        if re.search(r"\b%s::" % a, t):
            aliases.add(a)
    texts[f] = re.sub(r"\bcrate::", ident + "::", t)
# extern crate aliases of the owner root
ext = dict((a, c) for c, a in re.findall(r"(?m)^extern crate (\w+) as (\w+);", rt))
need_ext = [(ext[a], a) for a in sorted(aliases) if a in ext]
alltext = "\n".join(texts.values())


def sec(name):
    m = re.search(r"\[%s\]\n((?:[^\[\n][^\n]*\n|\n)*)" % re.escape(name), mt)
    return [l for l in (m.group(1).split("\n") if m else []) if l.strip() and not l.strip().startswith("#")]


def key_ident(l):
    k = l.split("=")[0].strip().split(".")[0].strip()
    return k, k.replace("-", "_")


wsrel = re.search(r'^workspace = "([^"]+)"', mt, re.M).group(1)
wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))


def rebase(l):
    m = re.search(r'path\s*=\s*"([^"]+)"', l)
    if not m:
        return l
    tgt = os.path.normpath(os.path.join(pkgdir, m.group(1)))
    return l.replace(m.group(0), 'path = "%s"' % os.path.relpath(tgt, newpkgdir))


dev = {}
for l in sec("dependencies"):
    k, i = key_ident(l)
    if re.search(r"\b%s\b" % re.escape(i), alltext) or any(c.replace("-", "_") == i for c, a in need_ext):
        dev[k] = l
for l in sec("dev-dependencies"):
    k, i = key_ident(l)
    if re.search(r"\b%s\b" % re.escape(i), alltext) or any(c.replace("-", "_") == i for c, a in need_ext):
        dev[k] = l
newpkg = pkg + "-" + dirname_
idm = re.search(r'^id = "([^"]+)"', mt, re.M)
newid = (idm.group(1) + "-" + dirname_) if idm else None
dev_lines = ['%s = { workspace = true }' % pkg] if re.search(r"\[workspace\.dependencies\.%s\]" % re.escape(pkg), open(wsabs + "/Cargo.toml").read()) else ['%s = { path = "%s" }' % (pkg, os.path.relpath(pkgdir, newpkgdir))]
dev_lines += [rebase(l) for k, l in sorted(dev.items()) if k != pkg]
newm = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Companion test crate of %s: %s"\n\n[package.metadata.semio]\nrole = "%s"\n%s\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n\n[dev-dependencies]\n%s'
        % (os.path.relpath(wsabs, newpkgdir), newpkg, pkg, dirname_, "test" if newid else "library", ('id = "%s"\n' % newid) if newid else "", "".join(l + "\n" for l in dev_lines)))
rel_root = lambda f: os.path.relpath(f, newdir)
root_lines = ["//! %s️ Companion test crate of `%s`: the test files stay beside their subjects and are mounted here, naming the owner crate by its own path.\n" % (emoji, pkg)]
for c, a in need_ext:
    root_lines.append("#[cfg(test)]\nextern crate %s as %s;\n" % (c.replace("-", "_"), a))
root_lines.append("")
for name, p, blk in blocks:
    f = os.path.normpath(os.path.join(rdir, p))
    root_lines.append('#[cfg(test)]\n#[path = "%s"]\nmod %s;\n' % (rel_root(f), name))
newroot = "\n".join(root_lines)
print("%s: move %d test mounts into %s; ext %s; dev %s" % (pkg, len(blocks), newpkg, [a for c, a in need_ext], sorted(dev)))
if not write:
    sys.exit(0)
os.makedirs(newpkgdir)
open(newpkgdir + "/Cargo.toml", "w").write(newm)
open(newdir + "/🦀️.rs", "w").write(newroot)
for f, t in texts.items():
    open(f, "w").write(t)
import subprocess
subprocess.run(["python3", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rebasefix.py"), pkgdir, newpkgdir] + list(texts), check=True)
for name, p, blk in blocks:
    rt = rt.replace(blk, "")
open(root, "w").write(rt)
wt = open(wsabs + "/Cargo.toml").read()
mrx = re.search(r'members = \[\n((?:\s+"[^"]+",\n)+)\s*\]', wt)
entries = re.findall(r'"([^"]+)"', mrx.group(1))
indent = re.match(r"(\s*)\"", mrx.group(1)).group(1)
oldm = os.path.relpath(pkgdir, wsabs)
newmem = os.path.relpath(newpkgdir, wsabs)
entries.insert(entries.index(oldm) + 1, newmem) if oldm in entries else entries.append(newmem)
wt = wt[:mrx.start(1)] + "".join('%s"%s",\n' % (indent, e) for e in entries) + wt[mrx.end(1):]
open(wsabs + "/Cargo.toml", "w").write(wt)
otxt = open(R + "/📋️build-owners.json").read()
rrel = os.path.relpath(newpkgdir, R)
oname = ("@semio-tech/%s-rs" % newid) if newid else newpkg
row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/%s",\n      "packages": []\n    },\n' % (oname, rrel, rrel, "📋️project.json" if newid else "Cargo.toml")
names_ = [(x.start(), x.group(1)) for x in re.finditer(r'    \{\n      "name": "([^"]+)"', otxt)]
at = next(p for p, n in names_ if n > oname)
otxt = otxt[:at] + row + otxt[at:]
json.loads(otxt)
open(R + "/📋️build-owners.json", "w").write(otxt)
if newid:
    open(newpkgdir + "/📋️project.json", "w").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % newid)
    ridx = os.path.join(os.path.dirname(os.path.abspath(__file__)), "route-index.txt")
    k = int(open(ridx).read())
    open(ridx, "w").write(str(k + 1))
    a_, b_ = 2 * k - 1, 2 * k
    ltxt = open(R + "/.vscode/launch.json").read()
    def route(label, target, conf, group, order):
        return ('    {\n      "name": "🦀️ %s %s",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/%s-rs:%s --configuration=%s --parallel=1 -- --lib --offline",\n      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "%s",\n        "order": %s\n      }\n    },\n' % (newpkg, label, newid, target, conf, group, order))
    routes = "".join([route("🏗️build 🛠️prototype", "build", "prototype", "4_build", "900.96%03d" % a_), route("🏗️build 🚀️release", "build", "release", "4_build", "900.96%03d" % b_), route("🔎️check 🛠️prototype", "check", "prototype", "4_gate", "922.96%03d" % a_), route("🔎️check 🚀️release", "check", "release", "4_gate", "922.96%03d" % b_), route("🧪️test 🛠️prototype", "test", "prototype", "4_test", "903.96%03d" % a_), route("🧪️test 🚀️release", "test", "release", "4_test", "903.96%03d" % b_)])
    last = [x.end() for x in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n' % re.escape(pkg), ltxt)]
    if not last:
        print("WARNING: no routes of the owner crate found; routes not added")
    else:
        ltxt = ltxt[:last[-1]] + routes + ltxt[last[-1]:]
        json.loads(ltxt)
        open(R + "/.vscode/launch.json", "w").write(ltxt)
open(os.path.dirname(os.path.abspath(__file__)) + "/created.tsv", "a").write("%s\t%s\t%s\t0\n" % (pkg, newpkg, os.path.relpath(newdir, R)))
print("written", newpkg)
