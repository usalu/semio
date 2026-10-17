#!/usr/bin/env python3
"""📦️ bd10: derives the raster crate manifests from the live source closures (`raster-closure.ts` output) and the facade's monolith manifest.

Adapted from the p6 `📦️family-manifests.py` rules (first-party edges from `semio_s_artifact_raster_raster_*` names, external edges from
code identifiers, workspace rows for every dependency). Output goes to a stage directory; nothing under the repo is written.

usage: raster_manifests.py <closure.json> <stage-dir>
"""
import json, os, re, subprocess, sys, importlib.util
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
P6 = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/split-p6")
spec = importlib.util.spec_from_file_location("relocate", os.path.join(P6, "🧭️relocate.py"))
relocate = importlib.util.module_from_spec(spec); spec.loader.exec_module(relocate)

ART = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"
artifact = os.path.join(REPO, ART)
facade_dir = os.path.join(artifact, "📦️packages", "🦀️rust")

def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text())))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)

closure = json.load(open(sys.argv[1], encoding="utf-8"))
EXTRA_TEST = {"agg": [ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️authoring-law/🦀️.rs"]}
for _n, _extra in EXTRA_TEST.items():
    closure[_n]["test"] = closure[_n]["test"] + _extra
stage = sys.argv[2]
monolith = toml_load(os.path.join(HERE, "orig", "raster-facade-Cargo.toml"))
workspace = toml_load(os.path.join(HERE, "orig", "raster-workspace-Cargo.toml"))
workspace_now = toml_load(os.path.join(artifact, "Cargo.toml"))
CRATES = json.load(open(os.path.join(HERE, "crates.json"), encoding="utf-8"))
IDENT = {k: v["ident"] for k, v in CRATES["crates"].items()}
DESC = {k: v["desc"] for k, v in CRATES["crates"].items()}
COMPANION = {k for k, v in CRATES["crates"].items() if v.get("companion")}
FEATURES = CRATES.get("features", {})
DEV_FEATURES = CRATES.get("devFeatures", {})
ALIASES = {"dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "framework_schema": "semio-framework-schema", "value_derive": "semio-framework-value-derive"}
CORE = ["semio-framework-value", "semio-framework-value-derive", "semio-framework-dsl-record", "semio-framework-dsl-record-derive", "semio-framework-pack-json", "semio-framework-os-kernel"]
pkg = lambda ident: ident.replace("_", "-")
family = {n: pkg(i) for n, i in IDENT.items()}
ws_deps = workspace.get("workspace", {}).get("dependencies", {})

def external_rows():
    rows, dev_features = {}, {}
    for table in ("dependencies", "dev-dependencies"):
        for key, value in monolith.get(table, {}).items():
            if isinstance(value, str):
                value = {"version": value}
            value = dict(value)
            dev = table == "dev-dependencies"
            if value.get("workspace"):
                row = dict(ws_deps.get(key, {}))
                if not row:
                    continue
            else:
                row = {k: v for k, v in value.items() if k != "features" or not dev}
                if "path" in row:
                    row["path"] = os.path.relpath(os.path.normpath(os.path.join(facade_dir, row["path"])), artifact)
                row.pop("features", None) if dev else None
            feats = value.get("features", [])
            if dev:
                if key in rows:
                    extra = set(feats) - set(rows[key].get("features", []))
                    if extra:
                        dev_features.setdefault(key, set()).update(extra)
                    continue
                dev_features.setdefault(key, set()).update(feats)
                row.pop("features", None)
                rows[key] = row
                rows[key]["__dev_only"] = True
                continue
            rows[key] = row
    return rows, dev_features

rows, dev_features = external_rows()
for key in list(rows):
    if key in family.values():
        rows.pop(key)

def code_names(key, row):
    names = {key.replace("-", "_")}
    names |= {alias for alias, package in ALIASES.items() if package == (row.get("package") or key)}
    if row.get("package"):
        names.add(row["package"].replace("-", "_"))
    return names

USES_DERIVES = re.compile(r"#\[\s*(derive|value|mutations|dsl|artifact_schema|state|canonical_json|mutation_leaf)\b|(?<![\w:])(dsl|protocol|store)::")
TEST_ATTR = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")
def split_tests(text):
    lib, tests = list(text), []
    for m in TEST_ATTR.finditer(text):
        j, depth = m.end(), 0
        while j < len(text):
            c = text[j]
            if c in "([":
                depth += 1
            elif c in ")]":
                depth -= 1
            elif c == "{":
                j = relocate.close_of(text, j) + 1
                if depth == 0:
                    break
                continue
            elif c == ";" and depth == 0:
                j += 1
                break
            j += 1
        tests.append(text[m.start():j])
        lib[m.start():j] = " " * (j - m.start())
    return "".join(lib), "\n".join(tests)

def texts(paths):
    out = []
    for p in paths:
        if p.endswith(".rs"):
            try:
                out.append(relocate.strip(open(os.path.join(REPO, p), encoding="utf-8").read()))
            except FileNotFoundError:
                pass
    return out

def toml_value(v):
    if isinstance(v, bool): return "true" if v else "false"
    if isinstance(v, str): return json.dumps(v, ensure_ascii=False)
    if isinstance(v, list): return "[" + ", ".join(toml_value(x) for x in v) + "]"
    if isinstance(v, dict): return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    return str(v)

edges, test_edges, manifests, ws_rows, report = {}, {}, {}, {}, {}
displaced = {}
def reaches(start, goal, seen=None):
    seen = seen or set()
    if start == goal:
        return True
    seen.add(start)
    return any(reaches(nxt, goal, seen) for nxt in edges.get(start, ()) if nxt not in seen)
lib_only = {}
for name, info in closure.items():
    lib_stripped = [split_tests(t) for t in texts(info["lib"])]
    lib_text = "\n".join(a for a, _ in lib_stripped)
    edges[name] = sorted(m for m, i in IDENT.items() if m != name and re.search(r"(?<![\w])" + re.escape(i) + r"(?![\w])", lib_text))
for name, info in closure.items():
    lib_files, test_files = info["lib"], info["test"]
    lib_stripped = [split_tests(t) for t in texts(lib_files)]
    lib_text = "\n".join(a for a, _ in lib_stripped)
    spill = "\n".join(b for _, b in lib_stripped)
    test_text = "\n".join(texts(test_files)) + "\n" + spill
    if name in EXTRA_TEST:
        pass
    ident = IDENT[name]
    e = set(edges[name])
    t = {m for m, i in IDENT.items() if m != name and m not in e and re.search(r"(?<![\w])" + re.escape(i) + r"(?![\w])", test_text)}
    displaced[name] = sorted(m for m in t if reaches(m, name))
    t = {m for m in t if m not in displaced[name]}
    test_edges[name] = sorted(t)
    deps, dev = {}, {}
    for m in sorted(e):
        deps[family[m]] = {"workspace": True}
    for m in sorted(t):
        dev[family[m]] = {"workspace": True}
    for key, row in rows.items():
        names = code_names(key, row)
        in_lib = any(re.search(r"(?<![\w])" + re.escape(n) + r"(?![\w])", lib_text) for n in names)
        in_test = any(re.search(r"(?<![\w])" + re.escape(n) + r"(?![\w])", test_text) for n in names)
        if (in_lib or (key in CORE and USES_DERIVES.search(lib_text))) and not row.get("__dev_only"):
            deps[key] = {"workspace": True}
        elif in_test or in_lib:
            dev[key] = {"workspace": True}
        if (in_test or in_lib) and dev_features.get(key):
            entry = dict(dev.get(key) or {"workspace": True})
            entry["features"] = sorted(dev_features[key])
            dev[key] = entry
        if key in deps or key in dev:
            ws_rows[key] = {k: v for k, v in row.items() if k != "__dev_only"}
    FIXTURE_READER = "🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust"
    if re.search(r"(?<![\w])semio_framework_test_fixtures(?![\w])", test_text) or re.search(r"(?<![\w])semio_framework_test_fixtures(?![\w])", lib_text):
        _own = os.path.dirname(os.path.join(REPO, info["root"]))
        _mdir = facade_dir if name == "agg" else (_own if _own.endswith("/📦️packages/🦀️rust") else os.path.join(_own, "📦️packages", "🦀️rust"))
        dev["semio-framework-test-fixtures"] = {"path": os.path.relpath(os.path.join(REPO, FIXTURE_READER), _mdir)}
    for target, feats in DEV_FEATURES.get(name, {}).items():
        dev[family[target]] = {"workspace": True, "features": sorted(feats)}
    if name in COMPANION:
        for key in list(deps):
            dev.setdefault(key, deps[key])
        deps = {}
    owner = os.path.dirname(os.path.join(REPO, info["root"]))
    mdir = facade_dir if name == "agg" else (owner if owner.endswith("/📦️packages/🦀️rust") else os.path.join(owner, "📦️packages", "🦀️rust"))
    lib_path = os.path.relpath(os.path.join(REPO, info["root"]), mdir)
    lines = ["[package]", f'workspace = "{os.path.relpath(artifact, mdir)}"', f'name = "{family[name]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
             "description = " + json.dumps(DESC[name], ensure_ascii=False), "", "[package.metadata.semio]", f'role = "{"artifact" if name == "agg" else "library"}"', "", "[lints]", "workspace = true", "", "[lib]", f'path = "{lib_path}"']
    if FEATURES.get(name):
        lines += ["", "[features]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(FEATURES[name].items())]
    lines += ["", "[dependencies]"]
    lines += [f"{k} = {toml_value(v)}" for k, v in sorted(deps.items())]
    if dev:
        lines += ["", "[dev-dependencies]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(dev.items())]
    manifests[os.path.relpath(os.path.join(mdir, "Cargo.toml"), REPO)] = "\n".join(lines) + "\n"
    report[name] = {"deps": len(deps), "dev": len(dev), "edges": edges[name], "test_edges": test_edges[name], "displaced": displaced[name]}

os.makedirs(stage, exist_ok=True)
json.dump({"manifests": manifests, "ws_rows": ws_rows, "report": report}, open(os.path.join(stage, "plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for n, r in report.items():
    print(n, r["deps"], r["dev"], "edges", r["edges"], "test", r["test_edges"], "DISPLACED", r["displaced"])
