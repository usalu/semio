import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
write = "--write" in sys.argv
WS = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
SUB = WS + "/🏅️standards/🔖️v1/🪆️subsets/🔊️audio"
MUT = SUB + "/🧬️schema/🧬️mutations"
AGG = MUT + "/📦️packages/🦀️rust"
NAME = "semio-s-artifact-stdio-semio-audio-mut-family"
IDENT = NAME.replace("-", "_")
AGGNAME = "semio-s-artifact-stdio-semio-audio-mutations"
OP = "SemioAudioOp"
comp = rd(MUT + "/🦀️.rs")
variants = re.findall(r"(?m)^    (\w+)\((\w+)::(\w+)\),", comp)
mounts = dict(re.findall(r'#\[path = "([^"/]+)/🦀️.rs"\]\npub mod (\w+);', comp))
mounts = {m: d for d, m in mounts.items()}
assert len(variants) == 8 == len(mounts), (variants, mounts)
leaves = [(v, m, s, mounts[m]) for v, m, s in variants]
D = MUT + "/%s/📦️packages/🦀️rust" % leaves[0][3]
edits = {}
alltext = ""
for v, m, s, d in leaves:
    pp = MUT + "/%s/🦀️.rs" % d
    t = rd(pp)
    t, n = re.subn(r"impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for (\w+) \{", r"impl<Op: %s> protocol::MutationKind<SemioAudioSnapshot, Op> for \1 {" % OP, t); assert n == 1, pp
    t, n = re.subn(r"protocol::MutationOutcome<<SemioAudioMutation as Mutation<SemioAudioSnapshot>>::Diff>", "protocol::MutationOutcome<SemioAudioDiff>", t); assert n == 1, pp
    t, n = re.subn(r"Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError>", "Result<Vec<Op>, semio_framework_value::ValueError>", t); assert n == 1, pp
    assert "SemioAudioMutation" not in t, pp
    edits[pp] = t; alltext += t
    ip = MUT + "/%s/↩️inverse/🦀️.rs" % d
    t = rd(ip)
    t, n = re.subn(r"pub fn inverse\((.*?)\) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> \{", r"pub fn inverse<Op: %s>(\1) -> Result<Vec<Op>, semio_framework_value::ValueError> {" % OP, t); assert n == 1, ip
    t, n = re.subn(r"SemioAudioMutation::\w+\(", "Op::from(", t); assert n >= 1, ip
    assert "SemioAudioMutation" not in t, ip
    edits[ip] = t; alltext += t
    dp = MUT + "/%s/🔺️diff/🦀️.rs" % d
    alltext += rd(dp)
# prelude: the component's imports that the leaves reference
uses = re.findall(r"(?m)^use ([^\n;]+);$", comp)
keep_uses = []
for u in uses:
    names = re.findall(r"\b(\w+)\s*(?:,|\}|$)", u.split("::")[-1]) if "{" in u else [u.split("::")[-1].strip()]
    names = [n for n in re.findall(r"\w+", u.split("::")[-1])] if "{" in u else [u.split("::")[-1]]
    if any(re.search(r"\b%s\b" % re.escape(n), alltext) for n in names) and not u.startswith("protocol::"):
        keep_uses.append(u)
print("prelude:", keep_uses)
bounds = " + ".join("From<%s::%s>" % (m, s) for v, m, s, d in leaves)
mods = "".join('        #[path = "../../../%s/🦀️.rs"]\n        pub mod %s;\n' % (d, m) for v, m, s, d in leaves)
leaves_root = '''//! 🍃️ The semio audio mutation leaves: every payload with its diff and inverse, generic over the aggregate that wraps them.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
%s
%s
        /// 🔗️ The aggregate-facing contract of every audio leaf: an operation enum that diffs an audio snapshot into an audio diff and wraps each leaf.
        pub trait %s: protocol::Mutation<SemioAudioSnapshot, Diff = SemioAudioDiff> + %s {}

        impl<T> %s for T where T: protocol::Mutation<SemioAudioSnapshot, Diff = SemioAudioDiff> + %s {}
    }
}
''' % ("".join("        use %s;\n" % u for u in keep_uses), mods, OP, bounds, OP, bounds)
assert "SemioAudioDiff" in "".join(keep_uses) and "SemioAudioSnapshot" in "".join(keep_uses)
aggman = rd(AGG + "/Cargo.toml")
deps = ["semio-framework-os-kernel", "semio-framework-pack-json", "semio-framework-ui-locale", "semio-framework-value", "semio-framework-value-derive", "semio-s-artifact-stdio-semio-base", "semio-s-artifact-stdio-semio-audio-model"]
for dn in deps: assert re.search(r"(?m)^%s = \{ workspace = true \}" % dn, aggman), dn
leaves_man = '''[package]
workspace = "../../../../../../../../.."
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Component crate of %s: mut-family"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
%s''' % (NAME, AGGNAME, "".join("%s = { workspace = true }\n" % dn for dn in deps))
new_comp = comp
for v, m, s, d in leaves:
    new_comp, n = re.subn(r'#\[path = "%s/🦀️.rs"\]\npub mod %s;\n' % (re.escape(d), m), "", new_comp); assert n == 1, m
new_comp = new_comp.replace("//#region 🔖️Mutations\n", "//#region 🔖️Mutations\npub use %s::schema::mutations::{%s};\n" % (IDENT, ", ".join(sorted(m for v, m, s, d in leaves))), 1)
for u in uses:
    if u.startswith("semio_s_artifact_stdio_semio_base::schema::triples::") or u.endswith("SemioAudioChannelDiff"):
        n_ = u.split("::")[-1]
        rest = re.sub(r"(?m)^use [^\n;]+;$", "", new_comp)
        if not re.search(r"\b%s\b" % n_, rest):
            new_comp = new_comp.replace("use %s;\n" % u, "", 1)
edits[MUT + "/🦀️.rs"] = new_comp
edits[AGG + "/Cargo.toml"] = aggman.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % NAME, 1)
for p, t in edits.items(): print("edit", p[len(R) + 1:][-70:], len(t))
if not write: print(leaves_root); sys.exit(0)
for p, t in edits.items(): wr(p, t)
os.makedirs(D, exist_ok=True)
assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(leaves_man)
open(D + "/🦀️.rs", "w", encoding="utf8").write(leaves_root)
add_member(WS, D, AGG)
print("ws dep", add_ws_dep(WS, NAME, D, AGGNAME))
add_owner(NAME, D)
print("done")
