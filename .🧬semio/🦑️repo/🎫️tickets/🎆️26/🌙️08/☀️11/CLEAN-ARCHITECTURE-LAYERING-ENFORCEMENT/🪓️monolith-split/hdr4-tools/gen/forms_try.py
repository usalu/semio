import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
from mkcomp import rebase, dep_lines, dep_manifest, lib_ident_of, used_in
write = "--write" in sys.argv
SUB = R + "/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any"
WS = R + "/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms"
CMD = SUB + "/✏️editor/🎮️commands"
AGGP = CMD + "/❓️add-question/📦️packages/🦀️rust"
FAM = ["▶️next-step", "◀️previous-step", "✅️submit", "🎯️set-try-value", "📍️set-try-value-step", "🗃️set-try-values", "↩️reset-try"]
D = CMD + "/↩️reset-try/📦️packages/🦀️rust"
NAME = "semio-s-artifact-forms-forms-commands-try"
IDENT = NAME.replace("-", "_")
mirror = rd(AGGP + "/🪞️mirror/🦀️.rs")
# commands mounts of the family
mods = {}
for d in FAM:
    m = re.search(r'#\[path = "[^"]*/%s/🦀️.rs"\]\n\s*pub mod (\w+);' % re.escape(d), mirror)
    assert m, d
    mods[d] = m.group(1)
print(mods)
for d, m in mods.items():
    pat = '            #[path = "../../../../%s/🦀️.rs"]\n            pub mod %s;\n' % (d, m)
    assert pat in mirror, (d, m)
new_mirror = mirror
for d, m in mods.items():
    new_mirror = new_mirror.replace('            #[path = "../../../../%s/🦀️.rs"]\n            pub mod %s;\n' % (d, m), "", 1)
reexport = "            pub use %s::editor::forms::commands::{%s};\n" % (IDENT, ", ".join(sorted(mods.values())))
new_mirror = new_mirror.replace('            #[path = "../../../../♻️update-form/🦀️.rs"]\n            pub mod update_form;\n', '            #[path = "../../../../♻️update-form/🦀️.rs"]\n            pub mod update_form;\n' + reexport, 1)
assert IDENT in new_mirror
# new crate root: the namespace the family reads through crate::
fam_mounts = ""
for d, m in mods.items():
    p = "../../../../%s/🦀️.rs" % d if d != "↩️reset-try" else "../../🦀️.rs"
    fam_mounts += '            #[path = "%s"]\n            pub mod %s;\n' % (p, m)
root = '''//! ▶️ The try-wizard commands of `semio-s-artifact-forms-forms`: stepping, submitting and the staged try values, read through the same `crate::` namespace the other commands use.

extern crate semio_framework_os_kernel as dsl;

pub use crate::schema::snapshot::FormsSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{forms_steps};
pub use semio_s_artifact_forms_forms_mutations::op;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                        }
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "."]
        pub mod commands {
%s        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard;
                }
            }
        }
    }
}
''' % fam_mounts
text = root + "".join(rd(CMD + "/%s/🦀️.rs" % d) for d in FAM)
aggman = rd(AGGP + "/Cargo.toml")
wsd = WS
deps = {}
_sec = re.search(r"(?ms)^\[dependencies\]\n(.*?)(?=^\[|\Z)", aggman).group(1)
for _l in _sec.split("\n"):
    _m = re.match(r"^([\w\-]+)(\.workspace)?\s*=", _l)
    if _m and _l.strip(): deps[_m.group(1)] = ("dependencies", _l)
keep = []
for key, (sec, l) in deps.items():
    if sec != "dependencies": continue
    names = {key.replace("-", "_")}
    pk = re.search(r'package = "([^"]+)"', l)
    if pk: names.add(pk.group(1).replace("-", "_"))
    dm = dep_manifest(l, AGGP, WS, key)
    if dm and os.path.exists(dm): names.add(lib_ident_of(dm))
    if any(used_in(n, text) for n in names):
        keep.append(rebase(l, AGGP, D))
print("deps kept:", [k.split(" ")[0] for k in keep])
man = '''[package]
workspace = "%s"
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Component crate of semio-s-artifact-forms-forms: commands, the try-wizard family"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
%s
''' % (re.search(r'^workspace = "([^"]+)"', aggman, re.M).group(1), NAME, "\n".join(keep))
print("root lines", root.count("\n"))
if not write:
    print(man); sys.exit(0)
wr(AGGP + "/🪞️mirror/🦀️.rs", new_mirror)
os.makedirs(D, exist_ok=True); assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(man); open(D + "/🦀️.rs", "w", encoding="utf8").write(root)
nm = aggman.replace("[dependencies]\n", "[dependencies]\n%s = { path = \"%s\" }\n" % (NAME, os.path.relpath(D, AGGP)), 1)
wr(AGGP + "/Cargo.toml", nm)
add_member(WS, D, AGGP)
add_owner(NAME, D)
print("done")
