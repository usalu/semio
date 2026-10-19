import json, os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
from mkcomp import rebase, dep_manifest, lib_ident_of, used_in
write = "--write" in sys.argv
WS = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg"
B = WS + "/🏅️standards/🔖️1.1/🪆️subsets/🧱️base"
SNAP = B + "/🧬️schema/📸️snapshot/📦️packages/🦀️rust"
D = B + "/🧬️schema/🔺️diff/📦️packages/🦀️rust"
NAME = "semio-s-artifact-stdio-svg-diff"
IDENT = NAME.replace("-", "_")
SNAPIDENT = "semio_s_artifact_stdio_svg_snapshot"
P = "standards::v1_1::subsets::base"
root = rd(SNAP + "/🦀️.rs")
new_root = root
old_diff = '''                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
'''
assert old_diff in new_root
new_root = new_root.replace(old_diff, "", 1)
for rep in ("💾️binary", "📝️text"):
    old = '                        #[path = "../../../../🚪️io/%s/🔺️diff/🦀️.rs"]\n                        pub mod diff;\n' % rep
    assert old in new_root, rep
    new_root = new_root.replace(old, "", 1)
droot = '''//! 🔺️ diff of `semio-s-artifact-stdio-svg` — the `SvgDiff` schema with its text and binary codecs.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use %(S)s::SvgSnapshot;
pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use %(S)s::%(P)s::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use %(S)s::%(P)s::io::binary::snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use %(S)s::%(P)s::io::text::snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
''' % {"S": SNAPIDENT, "P": P}
# manifest
snapman = rd(SNAP + "/Cargo.toml")
sec = re.search(r"(?ms)^\[dependencies\]\n(.*?)(?=^\[|\Z)", snapman).group(1)
text = droot + rd(B + "/🧬️schema/🔺️diff/🦀️.rs") + rd(B + "/🚪️io/📝️text/🔺️diff/🦀️.rs") + rd(B + "/🚪️io/💾️binary/🔺️diff/🦀️.rs")
deps = []
for l in sec.split("\n"):
    m = re.match(r"^([\w\-]+)(\.workspace)?\s*=", l)
    if not m or not l.strip(): continue
    key = m.group(1)
    names = {key.replace("-", "_")}
    pk = re.search(r'package = "([^"]+)"', l)
    if pk: names.add(pk.group(1).replace("-", "_"))
    dm = dep_manifest(l, SNAP, WS, key)
    if dm and os.path.exists(dm): names.add(lib_ident_of(dm))
    if any(used_in(n, text) for n in names):
        deps.append(rebase(l, SNAP, D))
deps.append('%s = { path = "%s" }' % ("semio-s-artifact-stdio-svg-snapshot", os.path.relpath(SNAP, D)))
print("deps:", [d.split(" ")[0] for d in deps])
man = '''[package]
workspace = "%s"
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Component crate of semio-s-artifact-stdio-svg: diff"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
%s
''' % (re.search(r'^workspace = "([^"]+)"', snapman, re.M).group(1), NAME, "\n".join(deps))
# consumers
cons = [l for l in open("/Users/ueli/.cache/hdr4/tmp/svgcons.txt", encoding="utf8").read().split("\n") if l]
cmap = json.load(open("/Users/ueli/.cache/hdr4/tmp/svgmap.json", encoding="utf8"))
pat = re.compile(r"%s::standards::v1_1::subsets::base::((?:schema|io::binary|io::text)::diff)\b" % SNAPIDENT)
edits = {}; manifests = set()
for f in cons:
    p = R + "/" + f
    t = rd(p)
    if not pat.search(t): continue
    t2 = pat.sub(lambda m: "%s::standards::v1_1::subsets::base::%s" % (IDENT, m.group(1)), t)
    edits[p] = t2
    owners = cmap.get(f, [])
    assert len(owners) >= 1, f
    for o in owners: manifests.add(R + "/" + o)
print("consumer files:", len(edits), "manifests:", len(manifests))
for mf in sorted(manifests):
    mt = rd(mf)
    if re.search(r"(?m)^%s\s*=" % re.escape(NAME), mt): continue
    line = '%s = { path = "%s" }\n' % (NAME, os.path.relpath(D, os.path.dirname(mf)))
    sline = re.search(r"(?m)^semio-s-artifact-stdio-svg-snapshot = .*\n", mt)
    assert sline, mf
    edits[mf] = mt[:sline.end()] + line + mt[sline.end():]
    print("manifest", mf[len(WS)+1:][-80:])
edits[SNAP + "/🦀️.rs"] = new_root
for p in edits: print("edit", p[len(R) + 1:][-90:])
if not write: sys.exit(0)
for p, t in edits.items(): wr(p, t)
os.makedirs(D, exist_ok=True); assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(man); open(D + "/🦀️.rs", "w", encoding="utf8").write(droot)
add_member(WS, D, SNAP)
add_owner(NAME, D)
print("done")
