#!/usr/bin/env python3
"""🧾️ Generates the `🏗️rehost.py` config of every norm artifact from one shared layer template (P9 decisions N1–N7) and
the curated mutation groups (`🧾️configs/norm-mutation-groups.json`), so all norm cuts come from one config-based run.

usage: 🧾️norm-config.py <artifact-dir-name>... | --all   (writes 🧾️configs/norm-<n>.json)
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
NORM = "✏️s/🔌️plugins/📕️norm"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
LOGICAL = "standards::v1::subsets::any"
HEADER = ["#![allow(async_fn_in_trait)]", "", "extern crate semio_framework_os_kernel as dsl;", "extern crate semio_framework_os_kernel as protocol;",
          "extern crate semio_framework_os_kernel as store;", "extern crate semio_framework_schema as framework_schema;",
          "extern crate semio_framework_value_derive as value_derive;"]
TITLES = {"en1990": "EN 1990", "en1991": "EN 1991", "en1992": "EN 1992", "en1993": "EN 1993", "en1994": "EN 1994", "en1995": "EN 1995", "en1996": "EN 1996",
          "en1997": "EN 1997", "en1998": "EN 1998", "en1999": "EN 1999", "din4108": "DIN 4108", "din16798": "DIN EN 16798", "din18599": "DIN V 18599",
          "iso16757": "ISO 16757", "vdi3805": "VDI 3805"}


def slug(name):
    return re.sub(r"^[^a-z0-9]*", "", name)


def config(art):
    n = slug(art)
    title = TITLES[n]
    A = f"{NORM}/🗿️artifacts/{art}"
    S = f"{A}/{SUBSET}"
    root_src = open(os.path.join(REPO, A, "🦀️.rs"), encoding="utf-8").read()
    mroot = open(os.path.join(REPO, S, "🧬️schema/🧬️mutations/🦀️.rs"), encoding="utf-8").read()
    enum = re.search(r"pub enum (\w+Mutation)\b", mroot).group(1)
    cap = enum[:-len("Mutation")]
    leafdir = {m.group(1): m.group(2) for m in re.finditer(r'pub mod (\w+)\s*\{\s*#\[path\s*=\s*"[^"]*🧬️mutations/([^/"]+)/', root_src) if m.group(2) not in ("🧭️edit-rules", "🧪️tests")}
    groups = json.load(open(os.path.join(HERE, "🧾️configs", "norm-mutation-groups.json"), encoding="utf-8"))[art]
    pkg = f"semio-s-artifact-norm-{n}"
    crates = {
        "model": {"package": f"{pkg}-model", "owner": f"{S}/🧬️schema/📸️snapshot", "prefix": f"{LOGICAL}::schema::snapshot", "description": f"{title} model: document records, dialect and snapshot"},
        "diff": {"package": f"{pkg}-diff", "owner": f"{S}/🧬️schema/🔺️diff", "prefix": f"{LOGICAL}::schema::diff", "description": f"{title} diff"},
        "mutations": {"package": f"{pkg}-mutations", "owner": f"{S}/🧬️schema/🧬️mutations", "prefix": f"{LOGICAL}::schema::mutations", "description": f"{title} mutations: the dispatch enum over every mutation group"},
        "schema": {"package": f"{pkg}-schema", "owner": f"{S}/🧬️schema", "prefix": f"{LOGICAL}::schema", "description": f"{title} artifact schema and descriptor"},
        "checks": {"package": f"{pkg}-checks", "owner": f"{S}/🧬️schema/💡️inferences/⚖️checks", "prefix": f"{LOGICAL}::schema::checks", "description": f"{title} compliance checks"},
        "inferences": {"package": f"{pkg}-inferences", "owner": f"{S}/🧬️schema/💡️inferences", "prefix": f"{LOGICAL}::schema::inferences", "description": f"{title} inferences"},
        "io": {"package": f"{pkg}-io", "owner": f"{S}/🚪️io", "prefix": f"{LOGICAL}::io", "description": f"{title} io: composition, builder, analyzer, mutation bridge and the text, binary and SQLite codecs"},
        "examples": {"package": f"{pkg}-examples", "owner": f"{S}/📚️examples", "prefix": "examples" if "pub mod examples {" in root_src else "", "description": f"{title} example documents"},
        "editor": {"package": f"{pkg}-editor", "owner": f"{S}/✏️editor", "prefix": None, "description": f"{title} editor"},
        "viewer": {"package": f"{pkg}-viewer", "owner": f"{S}/👁️viewer", "prefix": None, "description": f"{title} viewer"},
        "agg": {"package": pkg, "owner": A, "prefix": "", "description": f"Norm {n} artifact", "role": "artifact"},
    }
    crates["editor"]["prefix"] = f"editor::{n}"
    crates["viewer"]["prefix"] = f"viewer::{n}"
    rules = [["🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/", "agg", "sqlite_snapshot_tests"], ["✏️editor/🏷️field-meta/", "editor", "field_meta"], ["✳️any/🏷️field-meta/", "editor", "field_meta"], ["✏️editor/", "editor"], ["👁️viewer/", "viewer"], ["📚️examples/", "examples"],
             ["🧬️schema/📸️snapshot/", "model"], ["🧬️schema/🔺️diff/", "diff"], ["🧬️schema/🧬️mutations/(🦀️\\.rs|🧭️edit-rules/|🧪️tests/)", "mutations"]]
    create = []
    for gname, leaves in groups.items():
        key = "mut-" + slug(gname)
        crates[key] = {"package": f"{pkg}-mut-{slug(gname)}", "owner": f"{S}/🧬️schema/🧬️mutations/{gname}", "prefix": f"{LOGICAL}::schema::mutations",
                       "description": f"{title} {slug(gname).replace('-', ' ')} mutation leaves"}
        create.append({"path": f"{S}/🧬️schema/🧬️mutations/{gname}/🦀️.rs", "text": f"//! {gname[:gname.index(chr(0xfe0f)) + 1] if chr(0xfe0f) in gname else gname[:1]} {title} {slug(gname).replace('-', ' ')} mutation leaves.\n"})
        for leaf in leaves:
            rules.append([f"🧬️schema/🧬️mutations/{re.escape(leafdir[leaf])}/", key])
    has_compliance = "//#region 🔖️ComplianceHelpers" in open(os.path.join(REPO, S, "🧬️schema/🦀️.rs"), encoding="utf-8").read()
    has_checks_dir = os.path.isdir(os.path.join(REPO, S, "🧬️schema/💡️inferences/⚖️checks"))
    rules += ([["🧬️schema/💡️inferences/⚖️checks/", "checks"], ["🧬️schema/🧪️tests/⚖️compliance/", "checks"]] if has_compliance or has_checks_dir else []) + [["🧬️schema/💡️inferences/", "inferences"],
              ["🧬️schema/", "schema"], ["🚪️io/", "io"], ["🧪️tests/", "agg"], [re.escape(art) + "/🦀️\\.rs$", "agg"]]
    create.append({"path": f"{S}/📚️examples/🦀️.rs", "text": f"//! 📚️ {title} example documents.\n"})
    create.append({"path": f"{S}/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs", "text": (
        f"//! 🧫️ {title}'s committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is\n"
        f"//! the shared `semio_s_mutation_vectors`.\n\n"
        f"#[semio_framework_async_macros::async_test]\nasync fn committed_vectors_hold() {{\n"
        f"    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../../../../🧪️tests/🧫️fixtures/🧬️mutations\");\n"
        f"    semio_s_mutation_vectors::assert_committed_vectors::<{pkg.replace('-', '_')}_model::{cap}Snapshot, {pkg.replace('-', '_')}_mutations::{enum}>(&root, || {{}}).await;\n}}\n")})
    cfg = {
        "root": f"{A}/🦀️.rs", "crate": pkg.replace("-", "_"), "header": HEADER, "crates": crates, "rules": rules, "create": create,
        "transforms": [{"path": f"{S}/🧬️schema/🧬️mutations/🦀️.rs", "sub": [
            [r"(?:#\[cfg\(test\)\]\s*)?#\[path = \"🧪️tests/🔬️(?:fixture|middle-row)/🦀️\.rs\"\]\s*(?:#\[cfg\(test\)\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod \w+;\n?", ""]]}],
        "generic_leaves": {"enum": enum, "snapshot": f"{cap}Snapshot", "diff_path": f"crate::{LOGICAL}::schema::diff::{cap}Diff",
                           "mutations_dir": f"{S}/🧬️schema/🧬️mutations", "mutations_path": f"crate::{LOGICAL}::schema::mutations", "leaves": [leafdir[l] for g in groups.values() for l in g]},
        "facet_leaves": {"schema_file": f"{S}/🧬️schema/🦀️.rs", "facets": {
            "snapshot": ["📸️snapshot", "SNAPSHOT_LEAVES", f"crate::{LOGICAL}::schema::snapshot"],
            "diff": ["🔺️diff", "DIFF_LEAVES", f"crate::{LOGICAL}::schema::diff"],
            "mutations": ["🧬️mutations", "MUTATION_LEAVES", f"crate::{LOGICAL}::schema::mutations"]}},
        "regions": ([{"file": f"{S}/🧬️schema/🦀️.rs", "from_marker": "//#region 🔖️ComplianceHelpers", "to": f"{S}/🧬️schema/💡️inferences/⚖️checks/🦀️.rs",
                      "doc": f"//! ⚖️ {title} compliance checks: the clause-by-clause verification behind the compliance report.", "mount": "checks"}] if has_compliance else []),
        "move_mods": [{"file": f"{S}/🚪️io/🦀️.rs", "name": "mutation_bridge", "to": f"{S}/🧬️schema/🧬️mutations/🦀️.rs",
                       "source_reexport": f"pub use crate::{LOGICAL}::schema::mutations::mutation_bridge;\npub use mutation_bridge::*;",
                       "drop_in_target": [r"^pub use crate::standards::v1::subsets::any::io::mutation_bridge::\{[^}]*\};\n"]}],
        "extra_candidates": ["semio-s-plugin-norm-compliance", "semio-s-plugin-norm-app-surface", "semio-s-plugin-norm-results-config"],
        "dev_features": {"semio-s-plugin-norm-compliance": ["compliance-testing"], "semio-s-plugin-norm-app-surface": ["compliance-testing"], "semio-s-artifact-norm-contract": []},
        "root_to": "model", "delete_unmounted": True,
        "consumers": [A, f"{NORM}/🏭️bridge", "🌎️hub/🧩️compositions/📕️norm", "✏️s/🧑‍💻dev/🧹️fixture-sweep"],
        "test_targets": {"mutations": [{"name": "mutation_vectors", "path": f"{S}/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs",
                                        "dev": ["semio-s-mutation-vectors", "semio-framework-async-macros", f"{pkg}-model"]}]},
    }
    if not has_compliance and not has_checks_dir:
        del crates["checks"]
    if n == "iso16757":
        iso_checks(cfg, S, pkg, title)
    return cfg


def iso_checks(cfg, S, pkg, title):
    """📇️ ISO 16757 checks exceed one unit: the part files become taxonomy folders, part 1 and the shared builders (with the
    schema's compliance helpers) become their own crates below the check catalogue."""
    C = f"{S}/🧬️schema/💡️inferences/⚖️checks"
    moves = {"🧰common.rs": "🧰️common/🦀️.rs", "📈️part1.rs": "📈️part1/🦀️.rs", "📐️part2.rs": "📐️part2/🦀️.rs", "📚️part4.rs": "📚️part4/🦀️.rs", "🔄part5.rs": "🔄️part5/🦀️.rs"}
    for old, new in moves.items():
        text = open(os.path.join(REPO, C, old), encoding="utf-8").read()
        cfg["create"].append({"path": f"{C}/{new}", "text": text.replace('use crate::standards::v1::subsets::any::schema::', 'use crate::standards::v1::subsets::any::schema::')})
        cfg.setdefault("delete", []).append(f"{C}/{old}")
    cfg["transforms"].append({"path": f"{C}/🦀️.rs", "sub": [[re.escape(f'#[path = "{old}"]'), f'#[path = "{new}"]'] for old, new in moves.items()]})
    cfg["regions"][0].update({"to": f"{C}/🧰️common/🧩️helpers/🦀️.rs", "mount": "compliance_helpers",
                              "doc": f"//! 🧩️ {title} compliance helpers: pure part 1, 2, 4 and 5 libraries over the document types."})
    L = "standards::v1::subsets::any::schema"
    cfg["crates"]["checks"]["prefix"] = f"{L}::inferences::checks"
    cfg["crates"]["checks-common"] = {"package": f"{pkg}-checks-common", "owner": f"{C}/🧰️common", "prefix": f"{L}::inferences::checks::common", "description": f"{title} compliance builders and helpers"}
    cfg["crates"]["checks-part1"] = {"package": f"{pkg}-checks-part1", "owner": f"{C}/📈️part1", "prefix": f"{L}::inferences::checks::part1", "description": f"{title} part 1 compliance checks"}
    cfg["rules"] = [["🧬️schema/🧪️tests/⚖️compliance/", "checks-common", "helpers::compliance_tests"], ["⚖️checks/🧰️common/🧩️helpers/", "checks-common", "helpers"], ["⚖️checks/🧰️common/", "checks-common"], ["⚖️checks/📈️part1/", "checks-part1"]] + cfg["rules"]



def main():
    arts = sorted(d for d in os.listdir(os.path.join(REPO, NORM, "🗿️artifacts")) if os.path.isdir(os.path.join(REPO, NORM, "🗿️artifacts", d))) if "--all" in sys.argv else sys.argv[1:]
    for art in arts:
        cfg = config(art)
        out = os.path.join(HERE, "🧾️configs", f"norm-{slug(art)}.json")
        json.dump(cfg, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(out)


if __name__ == "__main__":
    main()
