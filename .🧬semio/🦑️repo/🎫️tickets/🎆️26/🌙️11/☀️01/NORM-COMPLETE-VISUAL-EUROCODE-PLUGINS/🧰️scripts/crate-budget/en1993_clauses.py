"""🪚 Splits en1993-clauses into a core crate (classification, effective width, resistance, stability, global, joints, fire) and a special-parts crate; the clauses root re-exports both."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ART = NORM + "/🗿️artifacts/🔩️en1993"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses"
OLD = "semio-s-artifact-norm-en1993-clauses"
CORE = ["classification", "effective_width", "resistance", "stability", "mcr_fe", "global", "joints", "fatigue_fire", "misc", "toughness"]
root = rd(D + "/🦀️.rs")
MOUNT = re.compile(r'#\[path = "([^"]+)/🦀️\.rs"\]\npub mod (\w+);\n')
ms = MOUNT.findall(root)
assert ms, "already split"
core = [(d, m) for d, m in ms if m in CORE]
spec = [(d, m) for d, m in ms if m not in CORE]
doc = root.split("\n")[0]


def mk(slug, emoji, mods, desc, extra_use, deps):
    name = OLD + "-" + slug
    cd = D + "/" + emoji + slug
    pkg = cd + "/📦️packages/🦀️rust"
    body = "//! %s %s\n\n" % (emoji, desc) + extra_use + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in mods)
    wr(cd + "/🦀️.rs", body)
    wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % (relp(ART, pkg), name, desc) + "".join("%s = { workspace = true }\n" % d for d in deps))
    register_workspace(ART, name, pkg)
    add_dep(D + "/📦️packages/🦀️rust/Cargo.toml", name)
    script_register(ART, OLD, name)
    lock_register(OLD, name, new_deps=deps)
    return name


cn = mk("core", "🧱️", core, "EN 1993 core clause functions: classification, effective widths, resistance, stability, global analysis, joints and fire.", "", ["semio-s-plugin-norm-compliance"])
sn = mk("special", "🧰️", spec, "EN 1993 special-part clause functions: annex parts, joint components, cold-formed, stainless, shells, bridges and towers.", "pub use %s::{effective_width, stability};\n\n" % cn.replace("-", "_"), [cn, "semio-s-plugin-norm-compliance"])
new = doc + "\n\npub use %s::{%s};\npub use %s::{%s};\n" % (cn.replace("-", "_"), ", ".join(m for _, m in core), sn.replace("-", "_"), ", ".join(m for _, m in spec))
tail = root[root.index("#[cfg(test)]"):]
wr(D + "/🦀️.rs", new + "\n" + tail)
