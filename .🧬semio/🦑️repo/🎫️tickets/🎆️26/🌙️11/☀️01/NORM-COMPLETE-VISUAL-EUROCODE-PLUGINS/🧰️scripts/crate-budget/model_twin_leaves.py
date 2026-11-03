"""🍃️ Moves the four facet texts (TypeScript, GraphQL, JSON Schema, proto) of a model (snapshot) crate into sibling twin crates (`LEAF` const each), like en1993-model. Usage: model_twin_leaves.py ARTIFACT-DIR. Idempotent."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
from twin_leaves import TWINS


def run(art):
    root = NORM + "/🗿️artifacts/" + art
    base = root + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot"
    toml = base + "/📦️packages/🦀️rust/Cargo.toml"
    parent = re.search(r'name = "([^"]+)"', rd(toml)).group(1)
    short = parent.split("norm-")[1].split("-")[0].upper().replace("EN", "EN ")
    p = base + "/🦀️.rs"
    s = rd(p)
    old = 'typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")'
    if old not in s:
        print("already or unmatched", parent)
        return
    new = ", ".join("%s: %s::LEAF" % (f, "%s_%s" % (parent.replace("-", "_"), t.replace("-", "_"))) for t, _, _, _, f in TWINS)
    for t, d, leaf, label, f in TWINS:
        cname = "%s-%s" % (parent, t)
        pkg = base + "/" + d + "/📦️packages/🦀️rust"
        wr(base + "/" + d + "/🦀️.rs", '//! 🍃️ %s model %s twin leaf, kept next to the schema twin it embeds.\n\n/// 🍃️ The handcrafted %s twin of the `model` schema facet.\npub const LEAF: &str = include_str!("../%s");\n' % (short, label, label, leaf))
        wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s model %s twin"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n' % (relp(root, pkg), cname, short, label))
        register_workspace(root, cname, pkg)
        add_dep(toml, cname)
        script_register(root, parent, cname)
        lock_register(parent, cname, new_deps=[])
    wr(p, s.replace(old, new, 1))
    print("twins", parent)


run(sys.argv[1])
