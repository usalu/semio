#!/usr/bin/env python3
"""📤️ Moves a format's schema-export leaves out of its model crate into one data crate per export language.

The artifact schema descriptor embeds five leaves (Rust, TypeScript, GraphQL, JSON Schema, Protocol Buffers) for each of
its four facets. Embedded as one block they exceed the 262,144-byte unit ceiling for most formats, so every language
gets its own data crate `<prefix>-exports-<language>` under `🧬️schema/📤️exports/<emoji><language>/📦️packages/🦀️rust`,
holding one `pub const <FACET>` per facet, and the descriptor composes its `FacetLeaves` from those constants.
The artifact workspace gets one `[workspace.dependencies]` row per data crate.

usage: 📤️exports-crates.py <schema dir, repo-relative> <descriptor fn> <package prefix> <artifact label> [--write]
"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
FACETS = [("ARTIFACT", "artifact", ""), ("SNAPSHOT", "snapshot", "📸️snapshot/"), ("DIFF", "diff", "🔺️diff/"), ("MUTATIONS", "mutations", "🧬️mutations/")]
LANGUAGES = [("rust", "🦀️rust", "🦀️.rs", "Rust", "rust"), ("typescript", "🟦️typescript", "🟦️.ts", "TypeScript", "typescript"), ("graphql", "🔗️graphql", "🔗️.graphql", "GraphQL", "graphql"), ("json", "🔣️json", "🔣️.json", "JSON Schema", "json_schema"), ("proto", "🛰️proto", "🛰️.proto", "Protocol Buffers", "proto")]


def slug_name(directory):
    return re.search(r"[a-z]+$", directory).group(0)


def main():
    schema_rel, function, prefix, label = sys.argv[1:5]
    write = "--write" in sys.argv
    schema = os.path.join(REPO, schema_rel)
    root = os.path.join(schema, "🦀️.rs")
    text = open(root, encoding="utf-8").read()
    start = text.index(f"pub fn {function}()")
    end = text.index("\n}\n", start) + 3
    ident = prefix.replace("-", "_")
    artifact_dir = schema
    while not os.path.exists(os.path.join(artifact_dir, "📜️artifact-definition.json")):
        artifact_dir = os.path.dirname(artifact_dir)
    writes = {}
    for slug, directory, leaf, name, _field in LANGUAGES:
        package_dir = os.path.join(schema, "📤️exports", directory, "📦️packages", "🦀️rust")
        lines = [f"//! {directory[:directory.index(slug_name(directory))]} `{label}` {name} schema exports of the artifact, snapshot, diff and mutation facets, embedded for the artifact schema descriptor.", ""]
        for const, facet, sub in FACETS:
            lines.append(f"/// {directory[:directory.index(slug_name(directory))]} The {facet} facet's {name} leaf.")
            lines.append(f'pub const {const}: &str = include_str!("{os.path.relpath(os.path.join(schema, sub + leaf), package_dir)}");')
        writes[os.path.join(package_dir, "🦀️.rs")] = "\n".join(lines) + "\n"
        writes[os.path.join(package_dir, "Cargo.toml")] = "\n".join([
            "[package]", f'workspace = "{os.path.relpath(artifact_dir, package_dir)}"', f'name = "{prefix}-exports-{slug}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
            f'description = "{label} {name} schema exports of every schema facet"', "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "🦀️.rs"', ""])
    def leaves(const):
        rows = ",\n".join(f"            {field}: {ident}_exports_{slug}::{const}" for slug, _d, _l, _n, field in LANGUAGES)
        return f"semio_framework_schema_registry::FacetLeaves {{\n{rows},\n        }}"
    descriptor_id = re.search(r'id:\s*"([^"]+)"', text[start:end]).group(1)
    body = "".join(f"        {facet}: {leaves(const)},\n" for const, facet, _ in FACETS)
    new = (f"pub fn {function}() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {{\n    semio_framework_schema_registry::ArtifactSchemaDescriptor {{\n"
           f'        id: "{descriptor_id}",\n{body}    }}\n}}\n')
    writes[root] = text[:start] + new + text[end:]
    workspace = os.path.join(artifact_dir, "Cargo.toml")
    ws = open(workspace, encoding="utf-8").read()
    add = ""
    for slug, directory, *_ in LANGUAGES:
        package = f"{prefix}-exports-{slug}"
        if re.search(r"(?m)^(?:" + re.escape(package) + r"\s*=|\[workspace\.dependencies\." + re.escape(package) + r"\])", ws):
            continue
        rel = os.path.relpath(os.path.join(schema, "📤️exports", directory, "📦️packages", "🦀️rust"), artifact_dir)
        add += f'\n[workspace.dependencies.{package}]\npath = "{rel}"\n'
    tables = list(re.finditer(r"(?m)^\[workspace\.dependencies\.[^\]]+\]\s*\n(?:[^\[\n][^\n]*\n)*", ws))
    at = tables[-1].end() if tables else len(ws)
    writes[workspace] = ws[:at] + add + ws[at:]
    for path in writes:
        print(f"{'write' if write else 'plan'} {os.path.relpath(path, REPO)}")
    if write:
        for path, content in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(content)


main()
