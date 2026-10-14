#!/usr/bin/env python3
"""📦️ Writes the manifests of a relocated crate family and wires its artifact workspace.

From the relocation layout (`🧭️relocate.py` → `<config>.layout.json`: per crate its root, lib and test files and the
crate edges) and the monolith's package manifest, every crate gets `<owner>/📦️packages/🦀️rust/Cargo.toml` with
`[lib] path = "../../🦀️.rs"`, exactly the family crates its sources name (lib edges → dependencies, test edges →
dev-dependencies) and exactly the external crates its sources name (an external is "named" when its code identifier,
its rename or one of its `extern crate` aliases appears in the crate's stripped sources). Every dependency is a
workspace row; the artifact workspace gains the family rows, the external rows, the members and loses any exclude that
would hide a member. The aggregator keeps its manifest path and role.

usage: 📦️family-manifests.py <config.json> [--write]
"""
import json, os, re, sys, subprocess, importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py"))
relocate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(relocate)
def toml_load(path):
    """🧾️ Parses TOML through Bun (the system Python predates `tomllib`)."""
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text())))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


config = json.load(open(sys.argv[1], encoding="utf-8"))
write = "--write" in sys.argv
layout = json.load(open(os.path.join(HERE, "🗑️generated", os.path.basename(sys.argv[1]).replace(".json", ".layout.json")), encoding="utf-8"))
artifact = os.path.dirname(os.path.join(REPO, config["root"]))
workspace_path = os.path.join(artifact, "Cargo.toml")
monolith_dir = os.path.join(artifact, "📦️packages", "🦀️rust")
monolith = toml_load(os.path.join(REPO, config["monolithManifest"]) if config.get("monolithManifest") else os.path.join(monolith_dir, "Cargo.toml"))
workspace = toml_load(workspace_path)
aggregator = config["aggregator"]
ALIASES = {"dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "framework_schema": "semio-framework-schema", "value_derive": "semio-framework-value-derive"}
CORE = ["semio-framework-value", "semio-framework-value-derive", "semio-framework-dsl-record", "semio-framework-dsl-record-derive", "semio-framework-pack-json", "semio-framework-os-kernel"]


def package_name(ident):
    return ident.replace("_", "-")


def external_rows():
    """🧾️ Every external dependency of the monolith as (key, workspace row, dev features) with workspace-relative paths."""
    rows, dev_features = {}, {}
    for table, target in (("dependencies", rows), ("dev-dependencies", None)):
        for key, value in monolith.get(table, {}).items():
            if isinstance(value, str):
                value = {"version": value}
            value = dict(value)
            if value.get("workspace"):
                row = dict(workspace.get("workspace", {}).get("dependencies", {}).get(key, {}))
                if not row:
                    continue
                if "features" in value:
                    dev_features.setdefault(key, set()).update(value["features"]) if table == "dev-dependencies" else None
            else:
                row = {k: v for k, v in value.items() if k not in ("features",) or table == "dependencies"}
                if "path" in row:
                    row["path"] = os.path.relpath(os.path.normpath(os.path.join(monolith_dir, row["path"])), artifact)
            if table == "dev-dependencies":
                if key in rows:
                    extra = set(value.get("features", [])) - set(rows[key].get("features", []))
                    if extra:
                        dev_features.setdefault(key, set()).update(extra)
                    continue
                row.pop("features", None) if key not in rows else None
                dev_features.setdefault(key, set()).update(value.get("features", []))
                rows.setdefault(key, row)
                rows[key]["__dev_only"] = True
                continue
            rows[key] = row
    return rows, dev_features


def code_names(key, row):
    names = {key.replace("-", "_")}
    names |= {alias for alias, package in ALIASES.items() if package == (row.get("package") or key)}
    return names


TEST_ATTR = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")


def split_tests(text):
    """✂️ (lib text, test text): every `#[cfg(test)]` item is blanked from the lib text and collected as test text."""
    lib, tests, pos = list(text), [], 0
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


def stripped(paths, keep_tests):
    out = []
    for path in paths:
        full = os.path.join(REPO, path)
        if os.path.exists(full):
            lib, tests = split_tests(relocate.strip(open(full, encoding="utf-8").read()))
            out.append(lib if not keep_tests else lib + "\n" + tests)
            if not keep_tests:
                TEST_SPILL.append(tests)
    return "\n".join(out)


TEST_SPILL = []


def toml_value(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, list):
        return "[" + ", ".join(toml_value(v) for v in value) + "]"
    if isinstance(value, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(v)}" for k, v in value.items()) + " }"
    return str(value)


def reaches(start, goal, seen=None):
    """🔁️ Whether `goal` is reachable from `start` along lib edges (a dev edge there would cycle)."""
    seen = seen or set()
    if start == goal:
        return True
    seen.add(start)
    return any(reaches(nxt, goal, seen) for nxt in layout["edges"].get(start, {}) if nxt not in seen)


displaced = {}
rows, dev_features = external_rows()
family = {name: package_name(info["ident"]) for name, info in layout["crates"].items()}
for key in list(rows):
    if key in family.values():
        rows.pop(key)
manifests, ws_rows = {}, {}
for name, info in layout["crates"].items():
    owner = os.path.dirname(os.path.join(REPO, info["root"]))
    manifest = os.path.join(owner, "📦️packages", "🦀️rust", "Cargo.toml")
    TEST_SPILL.clear()
    lib_text = stripped(info["lib"], False)
    test_text = stripped(info["tests"], True) + "\n" + "\n".join(TEST_SPILL)
    deps, dev = {}, {}
    for crate in sorted(layout["edges"].get(name, {})):
        deps[family[crate]] = {"workspace": True}
    for crate in sorted(layout["test_edges"].get(name, {})):
        if family[crate] not in deps and crate != name:
            if reaches(crate, name):
                displaced.setdefault(name, []).append(crate)
                continue
            dev[family[crate]] = {"workspace": True}
    for key, row in rows.items():
        names = code_names(key, row)
        in_lib = any(re.search(r"(?<![\w])" + re.escape(n) + r"(?![\w])", lib_text) for n in names)
        in_test = any(re.search(r"(?<![\w])" + re.escape(n) + r"(?![\w])", test_text) for n in names)
        if (in_lib or (key in CORE and lib_text.strip())) and not row.get("__dev_only"):
            deps[key] = {"workspace": True}
        elif in_test or in_lib:
            dev[key] = {"workspace": True}
        if (in_test or in_lib) and dev_features.get(key):
            entry = dict(dev.get(key) or {"workspace": True})
            entry["features"] = sorted(dev_features[key])
            dev[key] = entry
        if key in deps or key in dev:
            ws_rows[key] = {k: v for k, v in row.items() if k != "__dev_only"}
    rel_workspace = os.path.relpath(artifact, os.path.dirname(manifest))
    lib_path = os.path.relpath(os.path.join(REPO, info["root"]), os.path.dirname(manifest))
    is_agg = name == aggregator
    description = config["crates"][name].get("description") or re.sub(r"^//!\s*\S+\s*", "", config["crates"][name].get("doc", family[name])).rstrip(".")
    lines = ["[package]", f'workspace = "{rel_workspace}"', f'name = "{family[name]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
             "description = " + json.dumps(description, ensure_ascii=False), "",
             "[package.metadata.semio]", f'role = "{"artifact" if is_agg else "library"}"', "", "[lints]", "workspace = true", "", "[lib]", f'path = "{lib_path}"', "", "[dependencies]"]
    lines += [f"{k} = {toml_value(v)}" for k, v in sorted(deps.items())]
    if dev:
        lines += ["", "[dev-dependencies]"] + [f"{k} = {toml_value(v)}" for k, v in sorted(dev.items())]
    manifests[manifest] = "\n".join(lines) + "\n"
    print(f"{family[name]}: {len(deps)} deps, {len(dev)} dev-deps -> {os.path.relpath(manifest, artifact)}")

text = open(workspace_path, encoding="utf-8").read()
members = sorted(os.path.relpath(os.path.dirname(m), artifact) for m in manifests)
text = re.sub(r"(?ms)^members = \[.*?\n\]", "members = [\n" + "".join(f'    "{m}",\n' for m in members) + "]", text, count=1)
text = re.sub(r'(?m)^\s*"\*\*/🏅️standards/\*\*",\n', "", text)
for name, info in layout["crates"].items():
    if name == aggregator:
        continue
    owner = os.path.dirname(os.path.join(REPO, info["root"]))
    ws_rows[family[name]] = {"path": os.path.relpath(os.path.join(owner, "📦️packages", "🦀️rust"), artifact)}
existing = workspace.get("workspace", {}).get("dependencies", {})
added = []
for key, row in sorted(ws_rows.items()):
    if key in existing:
        continue
    block = f"\n[workspace.dependencies.{key}]\n" + "".join(f"{k} = {toml_value(v)}\n" for k, v in row.items())
    anchor = text.find("\n[workspace.lints")
    text = text[:anchor] + block + text[anchor:]
    added.append(key)
print(f"workspace: {len(members)} members, {len(added)} dependency rows added")
for name, crates in sorted(displaced.items()):
    print(f"DISPLACED tests of {name} reach upward into {sorted(crates)}")
if write:
    for path, body in manifests.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(body)
    open(workspace_path, "w", encoding="utf-8").write(text)
