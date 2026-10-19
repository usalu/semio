import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
write = "--write" in sys.argv
SUB = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table"
WS = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
MUT = SUB + "/🧬️schema/🧬️mutations"
AGG = MUT + "/📦️packages/🦀️rust"
NAME = "semio-s-artifact-stdio-semio-table-mut-family"
IDENT = NAME.replace("-", "_")
AGGNAME = "semio-s-artifact-stdio-semio-table-mutations"
comp = rd(MUT + "/🦀️.rs")
variants = re.findall(r"(?m)^    (\w+)\((\w+)::(\w+)\),", comp)
root = rd(AGG + "/🦀️.rs")
dirs = dict(re.findall(r'pub mod (\w+) \{\s*#\[path = "../../([^"]+)/🦀️.rs"\]\s*mod component;', root))
assert len(variants) == 8 == len(dirs), (variants, dirs)
leaves = [(v, m, s, dirs[m]) for v, m, s in variants]
D = MUT + "/%s/📦️packages/🦀️rust" % leaves[0][3]
OP = "SemioTableOp"
edits = {}
def put(p, t): edits[p] = t
for v, m, s, d in leaves:
    pp = MUT + "/%s/🦀️.rs" % d
    t = rd(pp)
    t = t.replace("use crate::schema::mutations::SemioTableMutation;\n", "use crate::schema::mutations::%s;\n" % OP, 1)
    t, n = re.subn(r"impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for (\w+) \{", r"impl<Op: %s> protocol::MutationKind<SemioTableSnapshot, Op> for \1 {" % OP, t)
    assert n == 1, pp
    t, n = re.subn(r"protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff>", "protocol::MutationOutcome<SemioTableDiff>", t)
    assert n == 1, pp
    t, n = re.subn(r"Result<Vec<SemioTableMutation>, semio_framework_value::ValueError>", "Result<Vec<Op>, semio_framework_value::ValueError>", t)
    assert n == 1, pp
    if "SemioTableDiff" not in t.split("//#region")[0]:
        t = t.replace("use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;\n", "use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;\nuse semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;\n", 1)
        assert "schema::diff::SemioTableDiff;" in t, pp
    assert "SemioTableMutation" not in t, pp
    put(pp, t)
    ip = MUT + "/%s/↩️inverse/🦀️.rs" % d
    t = rd(ip)
    t = re.sub(r"use crate::schema::mutations::\{([^}]*)\};", lambda mm: "use crate::schema::mutations::{%s};" % ", ".join(sorted([x.strip() for x in mm.group(1).split(",") if x.strip() != "SemioTableMutation"] + [OP])), t, count=1)
    t = t.replace("use crate::schema::mutations::SemioTableMutation;\n", "use crate::schema::mutations::%s;\n" % OP, 1)
    t, n = re.subn(r"pub fn inverse\((.*?)\) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> \{", r"pub fn inverse<Op: %s>(\1) -> Result<Vec<Op>, semio_framework_value::ValueError> {" % OP, t)
    assert n == 1, ip
    t, n = re.subn(r"SemioTableMutation::\w+\(", "Op::from(", t)
    assert n >= 1, ip
    assert "SemioTableMutation" not in t, ip
    put(ip, t)
bounds = " + ".join("From<%s::%s>" % (m, s) for v, m, s, d in leaves)
mods = ""
for v, m, s, d in leaves:
    mods += '''        #[path = "."]
        pub mod %s {
            #[path = "../../../%s/🦀️.rs"]
            mod component;
            #[path = "../../../%s/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../%s/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
''' % (m, d, d, d)
leaves_root = '''//! 🍃️ The semio table mutation leaves: every payload with its diff and inverse, generic over the aggregate that wraps them.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
        use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

%s
        /// 🔗️ The aggregate-facing contract of every table leaf: an operation enum that diffs a table snapshot into a table diff and wraps each leaf.
        pub trait %s: protocol::Mutation<SemioTableSnapshot, Diff = SemioTableDiff> + %s {}

        impl<T> %s for T where T: protocol::Mutation<SemioTableSnapshot, Diff = SemioTableDiff> + %s {}
    }
}
''' % (mods, OP, bounds, OP, bounds)
aggman = rd(AGG + "/Cargo.toml")
deps = ["semio-framework-os-kernel", "semio-framework-pack-json", "semio-framework-ui-locale", "semio-framework-value", "semio-framework-value-derive", "semio-s-artifact-stdio-semio-base", "semio-s-artifact-stdio-semio-table-model", "semio-s-artifact-stdio-semio-value-model"]
for dn in deps: assert re.search(r"(?m)^%s = \{ workspace = true \}" % dn, aggman), dn
leaves_man = '''[package]
workspace = "../../../../../../../../.."
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Component crate of semio-s-artifact-stdio-semio-table-mutations: mut-family"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
%s''' % (NAME, "".join("%s = { workspace = true }\n" % dn for dn in deps))
# aggregator root: replace the 8 blocks by one re-export
new_root = root
for v, m, s, d in leaves:
    new_root, n = re.subn(r'        #\[path = "\."\]\n        pub mod %s \{\n.*?\n        \}\n' % m, "", new_root, count=1, flags=re.S)
    assert n == 1, m
new_root = new_root.replace("        pub use component::*;\n", "        pub use component::*;\n        pub use %s::schema::mutations::{%s};\n" % (IDENT, ", ".join(sorted(m for v, m, s, d in leaves))), 1)
assert IDENT in new_root
put(AGG + "/🦀️.rs", new_root)
new_aggman = aggman.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % NAME, 1)
put(AGG + "/Cargo.toml", new_aggman)
for p, t in edits.items(): print("edit", p[len(R)+1:][-70:], len(t))
if not write: print(leaves_root[:3000]); sys.exit(0)
for p, t in edits.items(): wr(p, t)
os.makedirs(D, exist_ok=True)
assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(leaves_man)
open(D + "/🦀️.rs", "w", encoding="utf8").write(leaves_root)
add_member(WS, D, AGG)
r = add_ws_dep(WS, NAME, D, AGGNAME)
print("ws dep", r)
add_owner(NAME, D)
print("done")
