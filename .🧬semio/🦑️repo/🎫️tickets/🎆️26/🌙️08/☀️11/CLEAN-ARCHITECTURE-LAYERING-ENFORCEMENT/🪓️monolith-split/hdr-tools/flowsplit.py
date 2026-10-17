import json, os, re, shutil, sys
# ✂️ flowsplit.py : extracts the flow unit-test harness (`context`) into its own file behind the testkit idiom and moves five heavy test mounts into the companion test crate `unit-2`.
R = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
ORIG = HERE + "/orig/flow"
EDIT = R + "/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests"
U = EDIT + "/🔬️unit"
U2 = EDIT + "/🔬️unit-2"
PK = "/📦️packages/🦀️rust"
ART = R + "/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
MOVE = ["✏️node-graph-edit/", "🔬️interactive-job/", "⚖️declared-verbs/", "📌️panels/🗿️artifact/", "🗂️generations/"]
PKG, PKG2 = "semio-s-artifact-flow-flow-unit-tests", "semio-s-artifact-flow-flow-unit-tests-2"
write = "--write" in sys.argv


def backup(path):
    dst = ORIG + "/" + os.path.relpath(path, R).replace("/", "__")
    if not os.path.exists(dst):
        os.makedirs(ORIG, exist_ok=True)
        shutil.copy(path, dst)


root = open(U + "/🦀️.rs").read()
L = root.split("\n")
s = next(i for i, l in enumerate(L) if l.startswith("pub(crate) mod context {"))
e = next(i for i in range(s + 1, len(L)) if L[i] == "}")
body = [l[4:] if l.startswith("    ") else l for l in L[s + 1:e]]
body = [l.replace("pub(crate) fn", "pub fn") for l in body]
ctx = "//! 🧰️ Flow app fixture harness shared by the unit test crates: the self-closing app fixture, dispatch, settle and render helpers and the registered laws.\n\n" + "\n".join(body).strip("\n") + "\n"
newroot = "\n".join(L[:s] + L[e + 1:])

pk = U + PK
mirror = open(pk + "/🪞️mirror/🦀️.rs").read()
rx = re.compile(r'( *)#\[cfg\(test\)\]\n *#\[path = "([^"]+)"\]\n *pub mod (\w+);\n')
unit_blk = next(m for m in rx.finditer(mirror) if m.group(3) == "unit_tests")
ind = unit_blk.group(1)
kept_unit = (
    f'{ind}#[cfg(any(test, feature = "testkit"))]\n{ind}#[path = "."]\n{ind}pub mod unit_tests {{\n'
    f'{ind}    #[path = "../../../🧰️context/🦀️.rs"]\n{ind}    pub mod context;\n'
    f'{ind}    #[cfg(test)]\n{ind}    #[path = "../../../🦀️.rs"]\n{ind}    mod cases;\n{ind}}}\n'
)
comp_unit = (
    f'{ind}#[cfg(test)]\n{ind}#[path = "."]\n{ind}pub mod unit_tests {{\n'
    f'{ind}    pub use semio_s_artifact_flow_flow_unit_tests::editor::flow::unit_tests::context;\n{ind}}}\n'
)
m1, m2 = mirror, mirror
moved = []
for m in rx.finditer(mirror):
    if m.group(3) == "unit_tests":
        continue
    if any(t in m.group(2) for t in MOVE):
        moved.append(m)
for m in moved:
    m1 = m1.replace(m.group(0), "")
for m in rx.finditer(mirror):
    if m.group(3) == "unit_tests":
        continue
    if not any(t in m.group(2) for t in MOVE):
        m2 = m2.replace(m.group(0), "")
m1 = m1.replace(unit_blk.group(0), kept_unit)
m2 = m2.replace(unit_blk.group(0), comp_unit)
print("moved mounts:", len(moved), [m.group(2).split("/✏️")[-1][-60:] for m in moved])
print("root lines", len(newroot.split("\n")), "context lines", len(ctx.split("\n")))
if not write:
    sys.exit(0)

for p in [U + "/🦀️.rs", pk + "/🦀️.rs", pk + "/🪞️mirror/🦀️.rs", pk + "/Cargo.toml"]:
    backup(p)
os.makedirs(U + "/🧰️context", exist_ok=True)
open(U + "/🧰️context/🦀️.rs", "w").write(ctx)
open(U + "/🦀️.rs", "w").write(newroot)
open(pk + "/🪞️mirror/🦀️.rs", "w").write(m1)
librs = open(pk + "/🦀️.rs").read()
lib1 = librs.replace("#[cfg(test)]", '#[cfg(any(test, feature = "testkit"))]')
open(pk + "/🦀️.rs", "w").write(lib1)

os.makedirs(U2 + PK + "/🪞️mirror", exist_ok=True)
open(U2 + PK + "/🦀️.rs", "w").write(librs.replace("unit-tests of `semio-s-artifact-flow-flow`", "unit-tests-2 of `semio-s-artifact-flow-flow`").replace("🔬️ unit-tests of", "🔬️ unit-tests-2 of"))
open(U2 + PK + "/🪞️mirror/🦀️.rs", "w").write(m2)

cargo = open(pk + "/Cargo.toml").read()
dev_i = cargo.index("[dev-dependencies]")
deps_part, dev_part = cargo[:dev_i], cargo[dev_i:]
corpus = lib1 + m2 + "".join(open(os.path.normpath(os.path.join(pk + "/🪞️mirror", mm.group(2)))).read() for mm in rx.finditer(m2) if mm.group(3) != "unit_tests")


def keep(line):
    mm = re.match(r"^([\w\-]+)\s*=", line)
    if not mm:
        return True
    ident = mm.group(1).replace("-", "_")
    if ident == "dsl_derive" or True:
        pkgm = re.search(r'package = "([^"]+)"', line)
        names = {ident}
        if pkgm:
            names.add(pkgm.group(1).replace("-", "_"))
        return any(re.search(r"\b%s\b" % re.escape(n), corpus) for n in names)


def prune(section):
    out = []
    for l in section.split("\n"):
        if l.startswith("[") or not l.strip() or keep(l):
            out.append(l)
    return "\n".join(out)


deps2 = prune(deps_part)
dev2 = prune(dev_part)
deps2 = deps2.replace('name = "%s"' % PKG, 'name = "%s"' % PKG2).replace("Component crate of semio-s-artifact-flow-flow: unit-tests", "Component crate of semio-s-artifact-flow-flow: unit-tests-2")
dev2 = dev2.rstrip("\n") + '\n%s = { path = "../../../🔬️unit%s", features = ["testkit"] }\n' % (PKG, PK)
rebase = lambda t: t
open(U2 + PK + "/Cargo.toml", "w").write(deps2 + dev2)

feat = '\n[features]\ntestkit = ["semio-framework-plugin/artifact-app-testing", "semio-framework-os-kernel/protocol-laws", "semio-framework-os-kernel/mutation-testing"]\n'
c1 = cargo.replace("\n[dev-dependencies]", feat + "\n[dev-dependencies]", 1)
open(pk + "/Cargo.toml", "w").write(c1)
