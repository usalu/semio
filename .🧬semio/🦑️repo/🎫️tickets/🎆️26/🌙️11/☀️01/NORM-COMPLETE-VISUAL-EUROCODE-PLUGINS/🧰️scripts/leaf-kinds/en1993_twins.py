"""Splits the four schema twins of the EN 1993 mutation root into sibling twin crates (like the diff crate) so the root crate stays inside the source-boundary budget. Idempotent."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import wire as W

art = W.Art("en1993", "🔩️", "🔩️", None, None)
TWINS = [("typescript", "🟦️typescript-twin", "🟦️.ts", "TypeScript", "typescript"), ("graphql", "🔗️graphql-twin", "🔗️.graphql", "GraphQL", "graphql"), ("json-schema", "🔣️json-schema-twin", "🔣️.json", "JSON Schema", "json_schema"), ("proto", "🛰️proto-twin", "🛰️.proto", "Proto", "proto")]
DIFF = art.any + "/🧬️schema/🔺️diff"


def main():
    names = []
    for short, d, leaf, label, field in TWINS:
        cname = "semio-s-artifact-norm-en1993-mutations-" + short
        names.append(cname)
        root = art.mut + "/" + d
        os.makedirs(root + "/📦️packages/🦀️rust", exist_ok=True)
        open(root + "/🦀️.rs", "w", encoding="utf-8").write("//! 🍃️ En1993 mutations %s twin leaf, kept next to the schema twin it embeds.\n\n/// 🍃️ The handcrafted %s twin of the `mutations` schema facet.\npub const LEAF: &str = include_str!(\"../%s\");\n" % (label, label, leaf))
        t = rd(DIFF + "/" + d + "/📦️packages/🦀️rust/Cargo.toml")
        t = t.replace("en1993-diff-" + short, "en1993-mutations-" + short).replace("EN 1993 diff", "EN 1993 mutations")
        open(root + "/📦️packages/🦀️rust/Cargo.toml", "w", encoding="utf-8").write(t)
    p = art.mut + "/🦀️.rs"
    s = rd(p)
    s = re.sub(r"FacetLeaves \{ rust: include_str!\(\"🦀️\.rs\"\), typescript: [^}]*\}", "FacetLeaves { rust: include_str!(\"🦀️.rs\"), typescript: semio_s_artifact_norm_en1993_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1993_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1993_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1993_mutations_proto::LEAF }", s, count=1)
    open(p, "w", encoding="utf-8").write(s)
    f = art.root + "/Cargo.toml"
    s = rd(f)
    for (short, d, *_), cname in zip(TWINS, names):
        rel = os.path.relpath(art.mut + "/" + d + "/📦️packages/🦀️rust", art.root)
        if '"%s"' % rel not in s:
            s = s.replace("members = [\n", 'members = [\n    "%s",\n' % rel, 1)
        if "[workspace.dependencies.%s]" % cname not in s:
            s = s.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (cname, rel), 1)
    open(f, "w", encoding="utf-8").write(s)
    f = art.root + "/📦️packages/🦀️rust/📜️script.ts"
    s = rd(f)
    anchor = '"semio-s-artifact-norm-en1993-mutations",'
    s = rep(s, anchor, anchor + "".join('"%s",' % n for n in names if '"%s"' % n not in s))
    open(f, "w", encoding="utf-8").write(s)
    s = rd(OWNERS)
    new = ""
    for (short, d, *_), cname in zip(TWINS, names):
        if '"name": "%s"' % cname in s:
            continue
        tmpl = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1993-diff-%s",\n.*?\n    \},\n' % short, s, re.S).group(0)
        new += tmpl.replace("en1993-diff-" + short, "en1993-mutations-" + short).replace("/🧬️schema/🔺️diff/", "/🧬️schema/🧬️mutations/")
    anchor = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1993-mutations",\n.*?\n    \},\n', s, re.S).group(0)
    open(OWNERS, "w", encoding="utf-8").write(s.replace(anchor, anchor + new, 1))
    pkg = art.mut + "/📦️packages/🦀️rust/Cargo.toml"
    s = rd(pkg)
    anchor = "semio-s-artifact-norm-en1993-model = { workspace = true }\n"
    s = rep(s, anchor, anchor + "".join("%s = { workspace = true }\n" % n for n in names if (n + " =") not in s))
    open(pkg, "w", encoding="utf-8").write(s)
    print(names)


main()
