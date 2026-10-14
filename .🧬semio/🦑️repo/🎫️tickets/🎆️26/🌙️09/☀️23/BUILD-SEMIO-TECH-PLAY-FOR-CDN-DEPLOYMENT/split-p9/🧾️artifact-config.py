#!/usr/bin/env python3
"""🧾️ Generates the `🏗️rehost.py` config of any `#[path]`-scaffold artifact (one subset `✳️any`) from the shared layer
template of the norm run (P9 N1–N7), the curated mutation groups (`🧾️configs/artifact-mutation-groups.json`, keyed by the
artifact dir) and per-artifact overrides (`🧾️configs/artifact-overrides.json`: extra crates, rules, transforms, the vector
`prepare` body). Without curated groups the leaves stay in the `-mutations` crate and keep naming the dispatch enum.

usage: 🧾️artifact-config.py <artifact dir (repo-relative)>...   (writes 🧾️configs/artifact-<plugin>-<artifact>.json)
"""
import glob, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
LOGICAL = "standards::v1::subsets::any"
HEADER = ["#![allow(async_fn_in_trait)]", "", "extern crate semio_framework_os_kernel as dsl;", "extern crate semio_framework_os_kernel as protocol;",
          "extern crate semio_framework_os_kernel as store;", "extern crate semio_framework_schema as framework_schema;",
          "extern crate semio_framework_value_derive as value_derive;"]


def slug(name):
    return re.sub(r"^[^a-z0-9]*", "", name)


def emoji(name):
    return name[:name.index("️") + 1] if "️" in name else name[:1]


def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(require('fs').readFileSync(process.argv[1], 'utf8'))))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def config(A):
    S = f"{A}/{SUBSET}"
    overrides = json.load(open(os.path.join(HERE, "🧾️configs", "artifact-overrides.json"), encoding="utf-8")).get(A, {})
    groups = json.load(open(os.path.join(HERE, "🧾️configs", "artifact-mutation-groups.json"), encoding="utf-8")).get(A)
    pristine = os.path.join(REPO, ".🧬semio", "🦑️repo", "⚡️cache", "p9", "pristine", A, "📦️packages", "🦀️rust", "Cargo.toml")
    manifest = toml_load(pristine if os.path.exists(pristine) else os.path.join(REPO, A, "📦️packages", "🦀️rust", "Cargo.toml"))
    pkg = manifest["package"]["name"]
    title = overrides.get("title") or manifest["package"].get("description", pkg).replace(" artifact", "")
    root_src = open(os.path.join(REPO, A, "🦀️.rs"), encoding="utf-8").read()
    mroot_path = os.path.join(REPO, S, "🧬️schema/🧬️mutations/🦀️.rs")
    mroot = open(mroot_path, encoding="utf-8").read()
    enum = re.search(r"pub enum (\w+Mutation)\b", mroot).group(1)
    cap = enum[:-len("Mutation")]
    n = re.search(r"pub mod editor \{(?:\s*pub mod field_meta \{[^}]*\})?\s*#\[path = \"\.\"\]\s*pub mod (\w+)", root_src).group(1)
    leafdir = {m.group(1): m.group(2) for m in re.finditer(r'pub mod (\w+)\s*\{\s*#\[path\s*=\s*"[^"]*🧬️mutations/([^/"]+)/', root_src) if m.group(2) not in ("🧭️edit-rules", "🧪️tests")}
    crates = {
        "model": {"package": f"{pkg}-model", "owner": f"{S}/🧬️schema/📸️snapshot", "prefix": f"{LOGICAL}::schema::snapshot", "description": f"{title} model: document records, dialect and snapshot"},
        "diff": {"package": f"{pkg}-diff", "owner": f"{S}/🧬️schema/🔺️diff", "prefix": f"{LOGICAL}::schema::diff", "description": f"{title} diff"},
        "mutations": {"package": f"{pkg}-mutations", "owner": f"{S}/🧬️schema/🧬️mutations", "prefix": f"{LOGICAL}::schema::mutations", "description": f"{title} mutations: the dispatch enum" + (" over every mutation group" if groups else " and its leaves")},
        "schema": {"package": f"{pkg}-schema", "owner": f"{S}/🧬️schema", "prefix": f"{LOGICAL}::schema", "description": f"{title} artifact schema and descriptor"},
        "checks": {"package": f"{pkg}-checks", "owner": f"{S}/🧬️schema/💡️inferences/⚖️checks", "prefix": f"{LOGICAL}::schema::checks", "description": f"{title} compliance checks"},
        "inferences": {"package": f"{pkg}-inferences", "owner": f"{S}/🧬️schema/💡️inferences", "prefix": f"{LOGICAL}::schema::inferences", "description": f"{title} inferences"},
        "io": {"package": f"{pkg}-io", "owner": f"{S}/🚪️io", "prefix": f"{LOGICAL}::io", "description": f"{title} io: composition, builder, analyzer and the text, binary and SQLite codecs"},
        "examples": {"package": f"{pkg}-examples", "owner": f"{S}/📚️examples", "prefix": "examples" if "pub mod examples {" in root_src else "", "description": f"{title} example documents"},
        "editor": {"package": f"{pkg}-editor", "owner": f"{S}/✏️editor", "prefix": f"editor::{n}", "description": f"{title} editor"},
        "viewer": {"package": f"{pkg}-viewer", "owner": f"{S}/👁️viewer", "prefix": f"viewer::{n}", "description": f"{title} viewer"},
        "agg": {"package": pkg, "owner": A, "prefix": "", "description": manifest["package"].get("description", pkg), "role": manifest["package"].get("metadata", {}).get("semio", {}).get("role", "artifact")},
    }
    if not os.path.isdir(os.path.join(REPO, S, "📚️examples")):
        del crates["examples"]
    if not os.path.exists(os.path.join(REPO, S, "🧬️schema/💡️inferences/🦀️.rs")):
        del crates["inferences"]
    rules = [["🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/", "agg", "sqlite_snapshot_tests"], ["✏️editor/🏷️field-meta/", "editor", "field_meta"], ["✳️any/🏷️field-meta/", "editor", "field_meta"],
             ["✏️editor/", "editor"], ["👁️viewer/", "viewer"], ["📚️examples/", "examples"], ["🧬️schema/📸️snapshot/", "model"], ["🧬️schema/🔺️diff/", "diff"],
             ["🧬️schema/🧬️mutations/(🦀️\\.rs|🧭️edit-rules/|🧪️tests/)", "mutations"]]
    create, transforms = [], []
    vectors = overrides.get("vectors", bool(groups))
    if groups:
        for gname, leaves in groups.items():
            key = "mut-" + slug(gname)
            crates[key] = {"package": f"{pkg}-mut-{slug(gname)}", "owner": f"{S}/🧬️schema/🧬️mutations/{gname}", "prefix": f"{LOGICAL}::schema::mutations",
                           "description": f"{title} {slug(gname).replace('-', ' ')} mutation leaves"}
            create.append({"path": f"{S}/🧬️schema/🧬️mutations/{gname}/🦀️.rs", "text": f"//! {emoji(gname)} {title} {slug(gname).replace('-', ' ')} mutation leaves.\n"})
            for leaf in leaves:
                rules.append([f"🧬️schema/🧬️mutations/{re.escape(leafdir[leaf])}/", key])
    rules.append(["🧬️schema/🧬️mutations/", "mutations"])
    schema_src = open(os.path.join(REPO, S, "🧬️schema/🦀️.rs"), encoding="utf-8").read()
    has_compliance = "//#region 🔖️ComplianceHelpers" in schema_src
    has_checks_dir = os.path.isdir(os.path.join(REPO, S, "🧬️schema/💡️inferences/⚖️checks"))
    if not has_compliance and not has_checks_dir:
        del crates["checks"]
    rules += ([["🧬️schema/💡️inferences/⚖️checks/", "checks"], ["🧬️schema/🧪️tests/⚖️compliance/", "checks"]] if "checks" in crates else []) + \
             ([["🧬️schema/💡️inferences/", "inferences"]] if "inferences" in crates else []) + [["🧬️schema/", "schema"], ["🚪️io/", "io"], ["🧪️tests/", "agg"], [re.escape(os.path.basename(A)) + "/🦀️\\.rs$", "agg"]]
    if "examples" in crates and not os.path.exists(os.path.join(REPO, S, "📚️examples/🦀️.rs")):
        create.append({"path": f"{S}/📚️examples/🦀️.rs", "text": f"//! 📚️ {title} example documents.\n"})
    test_targets = {}
    if vectors:
        prepare = overrides.get("prepare", "|| {}")
        helpers = overrides.get("prepare_items", "")
        create.append({"path": f"{S}/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs", "text": (
            f"//! 🧫️ {title}'s committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is\n"
            f"//! the shared `semio_s_mutation_vectors`.\n\n{helpers}"
            f"#[semio_framework_async_macros::async_test]\nasync fn committed_vectors_hold() {{\n"
            f"    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../../../../🧪️tests/🧫️fixtures/🧬️mutations\");\n"
            f"    semio_s_mutation_vectors::assert_committed_vectors::<{pkg.replace('-', '_')}_model::{cap}Snapshot, {pkg.replace('-', '_')}_mutations::{enum}>(&root, {prepare}).await;\n}}\n")})
        transforms.append({"path": f"{S}/🧬️schema/🧬️mutations/🦀️.rs", "optional": True, "sub": [
            [r"(?:#\[cfg\(test\)\]\s*)?#\[path = \"🧪️tests/🔬️(?:fixture|middle-row)/🦀️\.rs\"\]\s*(?:#\[cfg\(test\)\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod \w+;\n?", ""]]})
        transforms.append({"path": f"{A}/🦀️.rs", "optional": True, "sub": [
            [r"[ \t]*#\[cfg\(test\)\]\s*#\[path = \"[^\"]*🧬️mutations/[^/\"]+/🧪️tests/[^\"]+\"\]\s*mod \w+;\n", ""]]})
        test_targets["mutations"] = [{"name": "mutation_vectors", "path": f"{S}/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs",
                                      "dev": ["semio-s-mutation-vectors", "semio-framework-async-macros", f"{pkg}-model"] + overrides.get("vector_dev", [])}]
    io_src = open(os.path.join(REPO, S, "🚪️io/🦀️.rs"), encoding="utf-8").read() if os.path.exists(os.path.join(REPO, S, "🚪️io/🦀️.rs")) else ""
    facet = re.search(r"snapshot: (?:::)?semio_framework_schema_registry::FacetLeaves \{\s*rust: include_str!\(\"📸️snapshot/🦀️\.rs\"\)", schema_src)
    cfg = {
        "root": f"{A}/🦀️.rs", "crate": pkg.replace("-", "_"), "header": HEADER, "crates": crates, "rules": rules, "create": create, "transforms": transforms + overrides.get("transforms", []),
        "facet_leaves": {"schema_file": f"{S}/🧬️schema/🦀️.rs", "facets": {
            "snapshot": ["📸️snapshot", "SNAPSHOT_LEAVES", f"crate::{LOGICAL}::schema::snapshot"],
            "diff": ["🔺️diff", "DIFF_LEAVES", f"crate::{LOGICAL}::schema::diff"],
            "mutations": ["🧬️mutations", "MUTATION_LEAVES", f"crate::{LOGICAL}::schema::mutations"]}} if facet else None,
        "regions": ([{"file": f"{S}/🧬️schema/🦀️.rs", "from_marker": "//#region 🔖️ComplianceHelpers", "to": f"{S}/🧬️schema/💡️inferences/⚖️checks/🦀️.rs",
                      "doc": f"//! ⚖️ {title} compliance checks.", "mount": "checks"}] if has_compliance else []),
        "move_mods": ([{"file": f"{S}/🚪️io/🦀️.rs", "name": "mutation_bridge", "to": f"{S}/🧬️schema/🧬️mutations/🦀️.rs",
                        "source_reexport": f"pub use crate::{LOGICAL}::schema::mutations::mutation_bridge;\npub use mutation_bridge::*;",
                        "drop_in_target": [r"^pub use crate::standards::v1::subsets::any::io::mutation_bridge::\{[^}]*\};\n"]}] if "pub mod mutation_bridge" in io_src else []),
        "root_to": "model", "delete_unmounted": True, "consumers": [A, "🌎️hub", "✏️s/🧑‍💻dev/🧹️fixture-sweep"] + overrides.get("consumers", []),
        "test_targets": test_targets, "generated": overrides.get("generated", {}), "root_to_map": {"pilot_languages": "io", **overrides.get("root_to_map", {})}, "root_to_extra": overrides.get("root_to_extra", []),
    }
    if groups:
        cfg["generic_leaves"] = {"enum": enum, "snapshot": f"{cap}Snapshot", "diff_path": f"crate::{LOGICAL}::schema::diff::{cap}Diff",
                                 "mutations_dir": f"{S}/🧬️schema/🧬️mutations", "mutations_path": f"crate::{LOGICAL}::schema::mutations", "leaves": [leafdir[l] for g in groups.values() for l in g]}
    for key, extra in overrides.get("crates", {}).items():
        cfg["crates"][key] = extra
    cfg["rules"] = overrides.get("rules", []) + cfg["rules"]
    return cfg


def main():
    for A in sys.argv[1:]:
        cfg = config(A)
        parts = A.rstrip("/").split("/")
        out = os.path.join(HERE, "🧾️configs", f"artifact-{slug(parts[-3])}-{slug(parts[-1])}.json")
        json.dump(cfg, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(out)


if __name__ == "__main__":
    main()
