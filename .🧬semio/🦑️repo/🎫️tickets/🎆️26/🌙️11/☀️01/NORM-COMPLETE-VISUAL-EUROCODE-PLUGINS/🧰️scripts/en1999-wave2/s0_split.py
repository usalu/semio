"""Stage 0: moves the four diff twin leaves of EN 1999 into sibling twin crates (pattern of en1993) so the diff crate closure stays under the source gate. Idempotent."""
import pathlib, re
A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999")
D = A / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"
TW = [("typescript", "🟦️typescript-twin", "🟦️.ts", "TypeScript"), ("graphql", "🔗️graphql-twin", "🔗️.graphql", "GraphQL"), ("json-schema", "🔣️json-schema-twin", "🔣️.json", "JSON schema"), ("proto", "🛰️proto-twin", "🛰️.proto", "proto")]
N = "semio-s-artifact-norm-en1999-diff"
for slug, d, f, label in TW:
    c = D / d / "📦️packages/🦀️rust"
    c.mkdir(parents=True, exist_ok=True)
    (c / "Cargo.toml").write_text(f'[package]\nworkspace = "../../../../../../../../.."\nname = "{N}-{slug}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1999 diff {label} twin"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n', encoding="utf8")
    (D / d / "🦀️.rs").write_text(f'//! 🍃️ En1999 diff {label} twin leaf, kept next to the schema twin it embeds.\n\n/// 🍃️ The handcrafted {label} twin of the `diff` schema facet.\npub const LEAF: &str = include_str!("../{f}");\n', encoding="utf8")
p = D / "📦️packages/🦀️rust/Cargo.toml"
t = p.read_text(encoding="utf8")
if "diff-typescript" not in t:
    t = t.replace("semio-s-artifact-norm-en1999-model = { workspace = true }\n", "".join(f"{N}-{s} = {{ workspace = true }}\n" for s, *_ in TW) + "semio-s-artifact-norm-en1999-model = { workspace = true }\n", 1)
    p.write_text(t, encoding="utf8")
p = D / "🦀️.rs"
t = p.read_text(encoding="utf8")
if "diff_typescript" not in t:
    old = 'rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")'
    assert old in t
    t = t.replace(old, 'rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1999_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1999_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1999_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1999_diff_proto::LEAF')
    p.write_text(t, encoding="utf8")
p = A / "Cargo.toml"
t = p.read_text(encoding="utf8")
if "typescript-twin" not in t:
    m = f'    "{REL}/📦️packages/🦀️rust",\n'
    assert m in t
    t = t.replace(m, m + "".join(f'    "{REL}/{d}/📦️packages/🦀️rust",\n' for _, d, *_ in TW), 1)
    a = f'[workspace.dependencies.{N}]\npath = "{REL}/📦️packages/🦀️rust"\n'
    assert a in t
    t = t.replace(a, a + "".join(f'\n[workspace.dependencies.{N}-{s}]\npath = "{REL}/{d}/📦️packages/🦀️rust"\n' for s, d, *_ in TW), 1)
    p.write_text(t, encoding="utf8")
p = A / "📦️packages/🦀️rust/📜️script.ts"
t = p.read_text(encoding="utf8")
if f'"{N}-proto"' not in t:
    t = t.replace(f'"{N}",', f'"{N}",' + ",".join(f'"{N}-{s}"' for s, *_ in TW) + ",", 1)
    p.write_text(t, encoding="utf8")
for lock in (A / "Cargo.lock", pathlib.Path("/home/deck/semio/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock"), pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock")):
    t = lock.read_text(encoding="utf8")
    if f'name = "{N}-proto"' in t:
        continue
    m = re.search(r'name = "' + N + r'"\nversion = "0.1.0"\ndependencies = \[\n(.*?)\]\n\n', t, re.S)
    deps = m.group(1) + "".join(f' "{N}-{s}",\n' for s, *_ in TW)
    deps = "".join(sorted(set(deps.splitlines(True))))
    block = m.group(0).replace(m.group(1), deps)
    block += "".join(f'[[package]]\nname = "{N}-{s}"\nversion = "0.1.0"\n\n' for s in ("graphql", "json-schema", "proto", "typescript"))
    t = t.replace(m.group(0), block, 1)
    lock.write_text(t, encoding="utf8")
print("stage 0 done")
