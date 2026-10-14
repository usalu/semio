#!/usr/bin/env python3
"""🕸️ Cuts `semio-framework-graph` into its five domain crates (manifest, engine, algorithms, drawing, dsl).

Each existing module folder becomes a crate root (`<folder>/📦️packages/🦀️rust/Cargo.toml`, `path = "../../🦀️.rs"`);
crate-relative paths become owner-crate paths, the engine's `PropertyBag`/`PropertyValue` re-export is dropped and the
two family-level Rust laws are mounted by the manifest crate that owns what they test.

usage: 🕸️graph-cut.py apply
"""
import os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
GRAPH = os.path.join(REPO, "🧰️framework/🔨️modules/🕸️graph")
UP = "../../../../"
CRATES = {
    "🛂️manifest": ("semio-framework-graph-manifest", "Compile-time graph manifest kernel: schema, registry, property values and strict validation.",
                  ["semio-framework-value", "semio-framework-value-derive", "semio-framework-dsl-record", "semio-framework-pack-json", "semio-framework-replication"], ["serde_json"]),
    "⚙️engine": ("semio-framework-graph-engine", "Pure graph foundation: topology markers, node, handle and edge kinds, storage, views and flow networks.",
                ["semio-framework-value", "semio-framework-value-derive", "geometry", "semio-framework-graph-manifest"], []),
    "🧮️algorithms": ("semio-framework-graph-algorithms", "Index-based graph algorithms: traversal, ordering, cycles, components, shortest paths.",
                    ["semio-framework-value", "semio-framework-value-derive"], []),
    "🖊️drawing": ("semio-framework-graph-drawing", "Graph drawing: node-placement layouts and edge and handle routing geometry.",
                 ["semio-framework-value", "semio-framework-value-derive", "geometry", "semio-framework-graph-engine"], []),
    "🗣️dsl": ("semio-framework-graph-dsl", "Jack, the shared graph query language: lexer, parser, matcher, formatter and language service.",
             ["semio-framework-value", "semio-framework-value-derive", "semio-framework-dsl-record", "semio-framework-dsl", "semio-framework-diagnostic", "semio-framework-pack-json", "semio-framework-graph-manifest"], ["serde_json"]),
}
PATHS = {
    "semio-framework-value": "🌱️value/📦️packages/🦀️rust", "semio-framework-value-derive": "🌱️value/✨️derive/📦️packages/🦀️rust",
    "semio-framework-dsl-record": "🗣️dsl/🧬️schema/📦️packages/🦀️rust", "semio-framework-pack-json": "🎒️pack/🔤️json/📦️packages/🦀️rust",
    "semio-framework-replication": "📡️replication/📦️packages/🦀️rust", "semio-framework-dsl": "🗣️dsl/📦️packages/🦀️rust",
    "semio-framework-diagnostic": "⚠️diagnostic/📦️packages/🦀️rust", "geometry": "📐️geometry/📦️packages/🦀️rust",
    "semio-framework-graph-manifest": "🕸️graph/🛂️manifest/📦️packages/🦀️rust", "semio-framework-graph-engine": "🕸️graph/⚙️engine/📦️packages/🦀️rust",
}


def rewrite(folder, edits):
    for base, _, files in os.walk(os.path.join(GRAPH, folder)):
        for name in files:
            if name != "🦀️.rs":
                continue
            path = os.path.join(base, name)
            text = open(path, encoding="utf-8").read()
            new = text
            for old, rep in edits:
                new = re.sub(old, rep, new)
            if new != text:
                open(path, "w", encoding="utf-8").write(new)
                print("rewrote", os.path.relpath(path, REPO))


def manifest_toml(folder, package, description, deps, dev):
    rows = []
    for dep in deps:
        spec = f'{{ path = "{UP}{PATHS[dep]}" }}' if dep != "geometry" else f'{{ path = "{UP}{PATHS[dep]}", package = "semio-framework-geometry" }}'
        rows.append(f"{dep} = {spec}")
    dev_rows = [f'{dep} = "1"' for dep in dev]
    return "\n".join([
        "[package]", 'workspace = "../../../../.."', f'name = "{package}"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true", f'description = "{description}"', "",
        "[package.metadata.semio]", 'role = "framework"', f'id = "{package.removeprefix("semio-framework-")}"', "",
        "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
        "[lints]", "workspace = true", "", "[lib]", f'name = "{package.replace("-", "_")}"', 'path = "../../🦀️.rs"', "", "[dependencies]", *rows,
        *(["", "[dev-dependencies]", *dev_rows] if dev_rows else []), ""])


def main():
    if sys.argv[1:] != ["apply"]:
        raise SystemExit(__doc__)
    common = [(r"\bvalue_derive::", "semio_framework_value_derive::")]
    rewrite("🛂️manifest", common + [(r"\bcrate::manifest::", "crate::")])
    rewrite("⚙️engine", common + [(r"pub use crate::manifest::\{PropertyBag, PropertyValue\};", "use semio_framework_graph_manifest::{PropertyBag, PropertyValue};"), (r"\bcrate::manifest::", "semio_framework_graph_manifest::")])
    rewrite("🧮️algorithms", common)
    rewrite("🖊️drawing", common + [(r"\buse crate::NodeShape;", "use semio_framework_graph_engine::NodeShape;")])
    rewrite("🗣️dsl", common + [(r"\bcrate::manifest::", "semio_framework_graph_manifest::"), (r"\bcrate::dsl::", "crate::")])
    rewrite("🧪️tests", [(r"\bcrate::manifest::", "crate::")])
    for folder, (package, description, deps, dev) in CRATES.items():
        pkg = os.path.join(GRAPH, folder, "📦️packages", "🦀️rust")
        os.makedirs(pkg, exist_ok=True)
        open(os.path.join(pkg, "Cargo.toml"), "w", encoding="utf-8").write(manifest_toml(folder, package, description, deps, dev))
    manifest_root = os.path.join(GRAPH, "🛂️manifest", "🦀️.rs")
    text = open(manifest_root, encoding="utf-8").read()
    if "canonical_record_owner_tests" not in text:
        text = text.rstrip("\n") + '\n\n#[cfg(test)]\n#[path = "../🧪️tests/🪆️record-owner/🦀️.rs"]\nmod canonical_record_owner_tests;\n\n#[cfg(test)]\n#[path = "../🧪️tests/♻️cold-properties/🦀️.rs"]\nmod cold_property_retirement_tests;\n'
        open(manifest_root, "w", encoding="utf-8").write(text)
    dsl_root = os.path.join(GRAPH, "🗣️dsl", "🦀️.rs")
    text = open(dsl_root, encoding="utf-8").read()
    if "#![allow(async_fn_in_trait)]" not in text:
        lines = text.split("\n")
        end = next(i for i, line in enumerate(lines) if not line.startswith("//!"))
        lines[end:end] = ["", "#![allow(async_fn_in_trait)]"]
        open(dsl_root, "w", encoding="utf-8").write("\n".join(lines))
    for name in ("Cargo.toml", "🦀️.rs"):
        path = os.path.join(GRAPH, "📦️packages", "🦀️rust", name)
        if os.path.exists(path):
            os.remove(path)


main()
