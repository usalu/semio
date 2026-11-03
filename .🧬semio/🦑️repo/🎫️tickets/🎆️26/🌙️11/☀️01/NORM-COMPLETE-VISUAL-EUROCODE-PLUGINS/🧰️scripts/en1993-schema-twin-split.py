#!/usr/bin/env python3
"""Idempotent: split the facet twins of an en1993 schema crate (argv[1]: diff|model) into sibling leaf crates so the crate closure stays inside the source boundary."""
import os, re, sys
R = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
KIND = sys.argv[1] if len(sys.argv) > 1 else "diff"
D = R + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/" + ("🔺️diff" if KIND == "diff" else "📸️snapshot")
TWINS = [("🟦️typescript-twin", "typescript", "🟦️.ts", "TypeScript"), ("🔗️graphql-twin", "graphql", "🔗️.graphql", "GraphQL"), ("🔣️json-schema-twin", "json-schema", "🔣️.json", "JSON schema"), ("🛰️proto-twin", "proto", "🛰️.proto", "Protobuf")]
BASE = "semio-s-artifact-norm-en1993-" + ("diff" if KIND == "diff" else "model")
RUST = BASE.replace("-", "_")
for folder, slug, leaf, label in TWINS:
    pk = f"{D}/{folder}/📦️packages/🦀️rust"
    os.makedirs(pk, exist_ok=True)
    open(f"{D}/{folder}/🦀️.rs", "w").write(f'//! 🍃️ En1993 {KIND} {label} twin leaf, kept next to the schema twin it embeds.\n\n/// 🍃️ The handcrafted {label} twin of the `{KIND}` schema facet.\npub const LEAF: &str = include_str!("../{leaf}");\n')
    open(f"{pk}/Cargo.toml", "w").write(f'''[package]
workspace = "../../../../../../../../.."
name = "{BASE}-{slug}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "EN 1993 {KIND} {label} twin"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
''')
p = R + "/Cargo.toml"; t = open(p).read()
for folder, slug, leaf, label in TWINS:
    m = f'    "{D[len(R) + 1:]}/{folder}/📦️packages/🦀️rust",\n'
    if m not in t:
        anchor = f'    "{D[len(R) + 1:]}/📦️packages/🦀️rust",\n'
        t = t.replace(anchor, anchor + m)
    h = f"[workspace.dependencies.{BASE}-{slug}]"
    if h not in t:
        a = f"[workspace.dependencies.{BASE}]\npath = \"" + D[len(R) + 1:] + "/📦️packages/🦀️rust\"\n"
        t = t.replace(a, a + f"\n{h}\npath = \"{D[len(R) + 1:]}/{folder}/📦️packages/🦀️rust\"\n")
open(p, "w").write(t)
p = D + "/📦️packages/🦀️rust/Cargo.toml"; t = open(p).read()
for folder, slug, leaf, label in TWINS:
    line = f"{BASE}-{slug} = {{ workspace = true }}\n"
    if line not in t:
        t = t.replace("[dependencies]\n", "[dependencies]\n" + line, 1)
open(p, "w").write(t)
p = D + "/🦀️.rs"; t = open(p).read()
t = re.sub(r'typescript: include_str!\("🟦️\.ts"\)', 'typescript: ' + RUST + '_typescript::LEAF', t)
t = re.sub(r'graphql: include_str!\("🔗️\.graphql"\)', 'graphql: ' + RUST + '_graphql::LEAF', t)
t = re.sub(r'json_schema: include_str!\("🔣️\.json"\)', 'json_schema: ' + RUST + '_json_schema::LEAF', t)
t = re.sub(r'proto: include_str!\("🛰️\.proto"\)', 'proto: ' + RUST + '_proto::LEAF', t)
open(p, "w").write(t)
p = R + "/📦️packages/🦀️rust/📜️script.ts"; t = open(p).read()
for folder, slug, leaf, label in TWINS:
    n = f'"{BASE}-{slug}",'
    if n not in t:
        t = t.replace(f'"{BASE}",', f'"{BASE}",{n}', 1)
open(p, "w").write(t)
def lock(path):
    t = open(path).read()
    names = [f"{BASE}-{s}" for _, s, _, _ in TWINS]
    pre = ''.join(f'[[package]]\nname = "{n}"\nversion = "0.1.0"\n\n' for n in sorted(names) if f'name = "{n}"' not in t)
    if pre:
        i = t.index(f'[[package]]\nname = "{BASE}"\n'); j = t.index("[[package]]", i + 5)
        t = t[:j] + pre + t[j:]
    m = re.search(r'(\[\[package\]\]\nname = "' + BASE + r'"\nversion = "0.1.0"\ndependencies = \[\n)(.*?)(\]\n)', t, re.S)
    deps = [l.strip().strip(',').strip('"') for l in m.group(2).splitlines()]
    for n in names:
        if n not in deps: deps.append(n)
    deps = sorted(set(deps))
    t = t[:m.start(2)] + ''.join(f' "{d}",\n' for d in deps) + t[m.end(2):]
    open(path, "w").write(t)
for lp in [R + "/Cargo.lock", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock", "/home/deck/semio/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock"]:
    if os.path.exists(lp): lock(lp)
