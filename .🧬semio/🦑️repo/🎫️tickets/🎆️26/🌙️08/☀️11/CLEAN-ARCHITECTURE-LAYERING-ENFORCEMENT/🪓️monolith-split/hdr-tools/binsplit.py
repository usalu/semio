import json, os, re, sys, shutil
# ✂️ binsplit.py : moves two child test roots out of a plugin companion test root file into a new sibling companion bin (part 3) and registers it (member, owner row, project.json, six routes in launch.json and the seed).
R = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
PKG = "📦️packages/🦀️rust"
COMP = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions"
TESTS = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests"
write = "--write" in sys.argv
src = COMP + "/🪡️tests-07-2"
dst = COMP + "/🫑️tests-07-3"
host = TESTS + "/🧪️time-travel-part-1/🦀️.rs"
MOUNTS = [("gesture_laws", "🧪️gesture"), ("folder_reload_route", "🧪️folder-reload-route")]
PKGN, ID, OLD, OLDID = "semio-framework-plugin-tests-07-3", "plugin-tests-07-3", "semio-framework-plugin-tests-07-2", "plugin-tests-07-2"
ht = open(host).read()
removed = ""
for mod, d in MOUNTS:
    rx = re.compile(r'(?:\n//#region [^\n]*\n)?#\[path = "\.\./%s/🦀️\.rs"\]\nmod %s;\n?(?://#endregion [^\n]*\n)?' % (re.escape(d), mod))
    m = rx.search(ht)
    assert m, mod
    removed += m.group(0)
    ht = ht[:m.start()] + ht[m.end():]
texts = "".join(open(TESTS + "/" + d + "/🦀️.rs").read() for _, d in MOUNTS)
cargo = open(src + "/" + PKG + "/Cargo.toml").read()
head, dev = cargo.split("[dev-dependencies]\n")
corpus = texts
def keep(line):
    mm = re.match(r"^([\w\-]+)\s*=", line)
    if not mm:
        return True
    names = {mm.group(1).replace("-", "_")}
    p = re.search(r'package = "([^"]+)"', line)
    if p:
        names.add(p.group(1).replace("-", "_"))
    return any(re.search(r"\b%s\b" % re.escape(n), corpus) for n in names)
dev3 = "".join(l + "\n" for l in dev.split("\n") if l.strip() and keep(l))
head3 = head.replace('name = "%s"' % OLD, 'name = "%s"' % PKGN).replace('id = "%s"' % OLDID, 'id = "%s"' % ID).replace("(part 2)", "(part 3)")
cargo3 = head3 + "[dev-dependencies]\n" + dev3
root3 = ("//! 🫑️ Companion test crate tests-07 of the plugin SDK partition: hosts the test roots `gesture_laws`, `folder_reload_route` (part 3).\n//!\n"
         "//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.\n\n"
         "extern crate semio_framework_os_kernel as store;\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as pack;\n\n"
         + "".join('#[cfg(test)]\n#[path = "../../🧪️tests/%s/🦀️.rs"]\npub mod %s;\n' % (d, mod) for mod, d in MOUNTS))
print("dev kept:", len(dev3.split("\n")) - 1, "of", len([l for l in dev.split("\n") if l.strip()]))
if not write:
    sys.exit(0)
orig = HERE + "/orig/binsplit"
os.makedirs(orig, exist_ok=True)
shutil.copy(host, orig + "/time-travel-part-1.rs")
open(host, "w").write(ht)
os.makedirs(dst + "/" + PKG)
open(dst + "/🦀️.rs", "w").write(root3)
open(dst + "/" + PKG + "/Cargo.toml", "w").write(cargo3)
open(dst + "/" + PKG + "/📋️project.json", "w").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % ID)
ct = open(R + "/Cargo.toml").read()
line = '    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪡️tests-07-2/%s",\n' % PKG
assert line in ct
ct = ct.replace(line, line + line.replace("🪡️tests-07-2", "🫑️tests-07-3"), 1)
open(R + "/Cargo.toml", "w").write(ct)
op = R + "/📋️build-owners.json"
o = open(op).read()
oname = "@semio-tech/%s-rs" % ID
rrel = os.path.relpath(dst + "/" + PKG, R)
row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/📋️project.json",\n      "packages": []\n    },\n' % (oname, rrel, rrel)
names = [(m.start(), m.group(1)) for m in re.finditer(r'    \{\n      "name": "(@semio-tech/[^"]+)"', o)]
assert oname not in o
pos = next(p for p, n in names if n > oname)
o = o[:pos] + row + o[pos:]
json.loads(o)
open(op, "w").write(o)
ridx = HERE + "/route-index.txt"
k = int(open(ridx).read())
open(ridx, "w").write(str(k + 1))
a_, b_ = 2 * k - 1, 2 * k
def route(label, target, conf, group, order):
    return ('    {\n      "name": "🦀️ %s %s",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/%s-rs:%s --configuration=%s --parallel=1 -- --lib --offline",\n      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "%s",\n        "order": %s\n      }\n    },\n' % (PKGN, label, ID, target, conf, group, order))
routes = "".join([route("🏗️build 🛠️prototype", "build", "prototype", "4_build", "900.96%03d" % a_), route("🏗️build 🚀️release", "build", "release", "4_build", "900.96%03d" % b_),
                  route("🔎️check 🛠️prototype", "check", "prototype", "4_gate", "922.96%03d" % a_), route("🔎️check 🚀️release", "check", "release", "4_gate", "922.96%03d" % b_),
                  route("🧪️test 🛠️prototype", "test", "prototype", "4_test", "903.96%03d" % a_), route("🧪️test 🚀️release", "test", "release", "4_test", "903.96%03d" % b_)])
for lp in (R + "/.vscode/launch.json", R + "/.vscode/🧩️launch.seed.jsonc"):
    lt = open(lp).read()
    assert '"🦀️ %s ' % PKGN not in lt
    last = [m.end() for m in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n' % re.escape(OLD), lt)]
    assert last, lp
    lt = lt[:last[-1]] + routes + lt[last[-1]:]
    open(lp, "w").write(lt)
open(HERE + "/created.tsv", "a").write("%s\t%s\t%s\t2\n" % (OLD, PKGN, os.path.relpath(dst, R)))
print("written", PKGN)
