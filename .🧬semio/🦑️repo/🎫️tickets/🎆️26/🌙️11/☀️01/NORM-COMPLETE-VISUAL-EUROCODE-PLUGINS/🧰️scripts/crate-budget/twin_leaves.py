"""🍃️ Moves the four generated facet texts (TypeScript, GraphQL, JSON Schema, proto) of a schema root crate into sibling twin crates (`LEAF` const each), so the root crate keeps only its Rust sources inside the source-boundary budget. Usage: twin_leaves.py ARTIFACT-DIR KIND(mutations|diff). Idempotent."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

TWINS = [("typescript", "🟦️typescript-twin", "🟦️.ts", "TypeScript", "typescript"), ("graphql", "🔗️graphql-twin", "🔗️.graphql", "GraphQL", "graphql"), ("json-schema", "🔣️json-schema-twin", "🔣️.json", "JSON Schema", "json_schema"), ("proto", "🛰️proto-twin", "🛰️.proto", "proto", "proto")]


def run(art, kind):
    root = NORM + "/🗿️artifacts/" + art
    base = root + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/" + ("🧬️mutations" if kind == "mutations" else "🔺️diff")
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
        wr(base + "/" + d + "/🦀️.rs", '//! 🍃️ %s %s %s twin leaf, kept next to the schema twin it embeds.\n\n/// 🍃️ The generated %s twin of the `%s` schema facet.\npub const LEAF: &str = include_str!("../%s");\n' % (short, kind, label, label, kind, leaf))
        wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s %s %s twin"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n' % (relp(root, pkg), cname, short, kind, label))
        register_workspace(root, cname, pkg)
        add_dep(toml, cname)
        script_register(root, parent, cname)
        lock_register(parent, cname, new_deps=[])
    wr(p, s.replace(old, new, 1))
    print("twins", parent)


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2])
