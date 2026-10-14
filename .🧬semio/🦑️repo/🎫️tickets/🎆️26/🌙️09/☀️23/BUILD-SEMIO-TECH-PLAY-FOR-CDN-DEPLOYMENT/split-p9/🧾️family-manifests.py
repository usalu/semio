#!/usr/bin/env python3
"""🧾️ Writes the Cargo manifests of one rehosted crate family (P9, after `🏗️rehost.py apply`).

Every crate gets `<owner>/📦️packages/🦀️rust/Cargo.toml` (`[lib] path = "../../🦀️.rs"`, workspace-inherited fields, the
monolith's lints) declaring exactly the crates its sources name: sibling crates from the rehost edges (test-only edges as
dev-dependencies), external crates from the identifiers its production and test sources use (the monolith's dependency
table is the candidate set, the extern-crate header is always named), feature-bearing dev-dependencies copied from the
monolith, and features only where a source gates on them (the aggregator forwards them). The artifact workspace root
gets every member and a `{ workspace = true }` row for every dependency; consumer packages get rows for the family crates
their files now name and lose the aggregator when nothing names it any more.

usage: 🧾️family-manifests.py <config.json> plan|apply
"""
import json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
_ns = {"__file__": os.path.join(HERE, "🦀️rust_items.py")}
exec(compile(open(_ns["__file__"], encoding="utf-8").read(), _ns["__file__"], "exec"), _ns)
strip = _ns["strip"]
HEADER = {"semio-framework-os-kernel", "semio-framework-schema", "semio-framework-value-derive"}
ALIASES = {"dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "vcs": "semio-framework-os-kernel",
           "framework_schema": "semio-framework-schema", "schema": "semio-framework-schema", "value_derive": "semio-framework-value-derive"}


def toml_load(path):
    """📖️ Parses a TOML file through Bun (the repo toolchain; this Python has no tomllib)."""
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(require('fs').readFileSync(process.argv[1], 'utf8'))))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def rel(path, start):
    return os.path.relpath(path, start)


def toml_value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, str):
        return json.dumps(v, ensure_ascii=False)
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    return str(v)


def used_idents(text):
    s = strip(text)
    return set(re.findall(r"(?<![\w:])([a-z_][a-z0-9_]*)\s*::", s)) | set(re.findall(r"extern\s+crate\s+([a-z_][a-z0-9_]*)", s)) | set(re.findall(r"\buse\s+([a-z_][a-z0-9_]*)\s*[;:{]", s))


def production_text(text):
    text = re.sub(r"#\[cfg_attr\(\s*test\s*,[^\]]*\]\]?", "", text)
    return text


def main():
    cfg = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    summary = json.load(open(os.path.join(HERE, "🗑️generated", "rehost-" + cfg["crate"] + ".json"), encoding="utf-8"))
    artifact = os.path.dirname(os.path.join(REPO, cfg["root"]))
    ws_root = os.path.join(REPO, cfg["workspace_root"]) if cfg.get("workspace_root") else artifact
    mono_path = os.path.join(artifact, "📦️packages", "🦀️rust", "Cargo.toml")
    pristine = os.path.join(REPO, ".🧬semio", "🦑️repo", "⚡️cache", "p9", "pristine", os.path.relpath(mono_path, REPO))
    mono = toml_load(pristine if os.path.exists(pristine) else mono_path)
    for extra in cfg.get("extra_candidates", []):
        mono.setdefault("dependencies", {}).setdefault(extra, {"workspace": True})
    ws_path = os.path.join(ws_root, "Cargo.toml")
    ws_text = open(ws_path, encoding="utf-8").read()
    ws = toml_load(ws_path)
    ws_deps = ws["workspace"].get("dependencies", {})
    mono_dir = os.path.dirname(mono_path)
    candidates = {}
    for table in ("dependencies", "dev-dependencies"):
        for key, spec in mono.get(table, {}).items():
            candidates.setdefault(key, {})[table] = spec
    test_files = set(summary["test_files"])
    crates = cfg["crates"]
    pkg_of = {k: v["package"] for k, v in crates.items()}
    ident_of = {k: v["package"].replace("-", "_") for k, v in crates.items()}
    normal_edges, test_edges = {}, {}
    for a, b, n in summary["edges"]:
        normal_edges.setdefault(a, set()).add(b)
    for a, b, n in summary["test_edges"]:
        test_edges.setdefault(a, set()).add(b)
    targets = cfg.get("test_targets", {})
    writes, new_rows = {}, {}
    feature_crates = []
    for key, c in crates.items():
        owner = os.path.join(REPO, c["owner"])
        pkg_dir = os.path.join(owner, "📦️packages", "🦀️rust")
        files = list(summary["files"].get(key, []))
        if os.path.relpath(os.path.join(owner, "🦀️.rs"), REPO) not in files:
            files.append(os.path.relpath(os.path.join(owner, "🦀️.rs"), REPO))
        prod, test = "", ""
        for f in files:
            path = os.path.join(REPO, f)
            if not os.path.exists(path):
                continue
            text = open(path, encoding="utf-8").read()
            if f in test_files:
                test += text
            else:
                prod += production_text(text)
                test += text
        tt = targets.get(key, [])
        for t in tt:
            test += open(os.path.join(REPO, t["path"]), encoding="utf-8").read()
        p_ids, t_ids = used_idents(prod), used_idents(test)
        deps, dev = {}, {}
        for other in sorted(normal_edges.get(key, ())):
            deps[pkg_of[other]] = {"workspace": True}
        for other in sorted(test_edges.get(key, set()) - normal_edges.get(key, set())):
            dev[pkg_of[other]] = {"workspace": True}
        if key != "agg" or True:
            for h in sorted(HEADER):
                deps.setdefault(h, {"workspace": True})
        for cand, specs in candidates.items():
            ident = cand.replace("-", "_")
            aliased = {a for a, target in ALIASES.items() if target == cand}
            in_prod = ident in p_ids or bool(aliased & p_ids and cand in HEADER)
            in_test = ident in t_ids
            if cand in pkg_of.values():
                continue
            if in_prod and "dependencies" in specs:
                spec = {"workspace": True}
                if isinstance(specs["dependencies"], dict) and specs["dependencies"].get("optional"):
                    spec["optional"] = True
                deps[cand] = spec
            elif in_prod and "dev-dependencies" in specs:
                deps[cand] = {"workspace": True}
            if (in_test or (cand in deps and test.strip())) and "dev-dependencies" in specs:
                dspec = specs["dev-dependencies"]
                if isinstance(dspec, dict) and dspec.get("features"):
                    dev[cand] = {"workspace": True, "features": dspec["features"]}
                elif in_test and cand not in deps:
                    dev[cand] = {"workspace": True}
            elif in_test and cand not in deps and "dependencies" in specs:
                dev[cand] = {"workspace": True}
        for name, feats in cfg.get("dev_features", {}).items():
            if not feats:
                dev.pop(name, None)
            elif name in deps and test.strip():
                dev[name] = {"workspace": True, "features": feats}
        for t in tt:
            for d in t.get("dev", []):
                if d not in deps:
                    dev.setdefault(d, {"workspace": True})
        features = {}
        for f in files:
            path = os.path.join(REPO, f)
            if os.path.exists(path):
                for feat in re.findall(r'feature\s*=\s*"([^"]+)"', open(path, encoding="utf-8").read()):
                    mono_feat = mono.get("features", {}).get(feat)
                    if mono_feat is not None:
                        features[feat] = mono_feat
                        for item in mono_feat:
                            if item.startswith("dep:"):
                                deps[item[4:]] = {"workspace": True, "optional": True}
        if features and key != "agg":
            feature_crates.append((key, sorted(features)))
        for d in list(deps) + list(dev):
            if d not in ws_deps and d not in pkg_of.values():
                spec = candidates.get(d, {}).get("dependencies") or candidates.get(d, {}).get("dev-dependencies")
                if isinstance(spec, dict) and "path" in spec:
                    new_rows[d] = {"path": rel(os.path.normpath(os.path.join(mono_dir, spec["path"])), ws_root)}
                elif isinstance(spec, dict) and "version" in spec:
                    new_rows[d] = {k: v for k, v in spec.items() if k in ("version", "features", "package")}
                elif d == "semio-s-mutation-vectors":
                    new_rows[d] = {"path": rel(os.path.join(REPO, "✏️s/🧪️tests/🧪️mutation-vectors/📦️packages/🦀️rust"), ws_root)}
                elif d == "semio-framework-async-macros" and d not in ws_deps:
                    new_rows[d] = {"path": rel(os.path.join(REPO, "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"), ws_root)}
        for internal in [pkg_of[k] for k in crates if k != key]:
            if internal not in ws_deps:
                new_rows[internal] = {"path": rel(os.path.join(REPO, crates[[k for k in crates if pkg_of[k] == internal][0]]["owner"], "📦️packages", "🦀️rust"), ws_root)}
        lines = ["[package]", f'workspace = "{rel(ws_root, pkg_dir)}"', f'name = "{c["package"]}"', "version.workspace = true", "edition.workspace = true",
                 "rust-version.workspace = true", f'description = {json.dumps(c.get("description", c["package"]), ensure_ascii=False)}', "",
                 "[package.metadata.semio]", f'role = "{c.get("role", "library")}"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"']
        for t in tt:
            lines += ["", "[[test]]", f'name = "{t["name"]}"', f'path = "{rel(os.path.join(REPO, t["path"]), pkg_dir)}"']
        if key == "agg":
            fwd = {}
            for k2, feats in feature_crates:
                for feat in feats:
                    fwd.setdefault(feat, []).append(f"{pkg_of[k2]}/{feat}")
            for feat, items in mono.get("features", {}).items():
                if feat == "default":
                    continue
                own = [i for i in items if not i.startswith("dep:") or i[4:] in deps]
                fwd.setdefault(feat, [])
                fwd[feat] = sorted(set(fwd[feat] + (own if feat in features else [])))
            features = {f: [i for i in v if i.split("/")[0] in deps or i.startswith("dep:")] for f, v in fwd.items()}
            features = {f: v for f, v in features.items() if v}
        if features:
            lines += ["", "[features]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(features.items())]
        lines += ["", "[dependencies]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(deps.items())]
        if dev:
            lines += ["", "[dev-dependencies]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(dev.items())]
        writes[os.path.join(pkg_dir, "Cargo.toml")] = "\n".join(lines) + "\n"
    members = sorted(set(ws["workspace"].get("members", [])) | {rel(os.path.join(REPO, c["owner"], "📦️packages", "🦀️rust"), ws_root) for c in crates.values()})
    ws_new = re.sub(r"members = \[[^\]]*\]", "members = [\n" + "".join(f'    "{m}",\n' for m in members) + "]", ws_text, count=1)
    rows = "".join(f"\n[workspace.dependencies.{k}]\n" + "".join(f"{kk} = {toml_value(vv)}\n" for kk, vv in v.items()) for k, v in sorted(new_rows.items()))
    ws_new = ws_new.replace("\n[workspace.lints.rust]", rows + "\n[workspace.lints.rust]", 1)
    writes[ws_path] = ws_new
    for path, text in writes.items():
        print(f"== {os.path.relpath(path, REPO)}")
        if mode == "plan":
            print(text if "📦️packages" in path else "\n".join(l for l in text.splitlines() if "members" in l or "/📦️packages" in l)[:3000])
    if mode == "apply":
        for path, text in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
    print("consumers naming family crates:", json.dumps(summary.get("consumers", {}), ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
