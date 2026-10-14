#!/usr/bin/env python3
"""🧾️ Writes the Cargo manifest of every new layer crate of a `✂️crate-cut.py` config and trims the source crate's.

Each layer crate gets exactly the dependencies its files name: an external crate is kept when one of the crate's
non-test files names its ident (or its `extern crate … as` alias appears in the header), as a dev-dependency when only
test files name it; the row is copied from the source crate's manifest (renames, features and paths preserved,
relative paths re-rooted). Layer-to-layer edges become `<package> = { workspace = true }` rows, and every new package
gets a `[workspace.dependencies]` row in the artifact workspace root. The source crate's own manifest is rewritten the
same way (its remaining files, plus the layer crates it now depends on).

usage: 🧾️crate-manifests.py <cut-config.json> plan|apply   (run after `✂️crate-cut.py apply`: each layer is read from its own root)
config additions: "keep": [dependency names every layer crate carries because derive expansions name them],
                  "extra": {layer: ["name = { … }"]}, "descriptions": {layer: text}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
cut = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CUT, "exec"), cut)
Crate, strip = cut["Crate"], cut["strip"]
INCLUDE = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*"([^"]+)"\s*\)')


def sections(text):
    out, current = {}, None
    for line in text.split("\n"):
        header = re.match(r"^\[([^\]]+)\]\s*$", line)
        if header:
            current = header.group(1)
            out.setdefault(current, [])
            continue
        if current is not None:
            out[current].append(line)
    return out


def dep_rows(lines):
    rows = {}
    for line in lines:
        m = re.match(r'^([A-Za-z0-9_\-]+)(\.workspace)?\s*=\s*(.*)$', line)
        if m:
            rows[m.group(1)] = "{ workspace = true }" if m.group(2) else m.group(3).strip()
    return rows


def ident_of(name, value):
    rename = re.search(r'package\s*=\s*"([^"]+)"', value)
    return name.replace("-", "_") if True else None


def reroot(value, old_dir, new_dir):
    m = re.search(r'path\s*=\s*"([^"]+)"', value)
    if not m:
        return value
    absolute = os.path.normpath(os.path.join(old_dir, m.group(1)))
    return value.replace(m.group(0), f'path = "{os.path.relpath(absolute, new_dir)}"')


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    base = os.path.dirname(os.path.abspath(config_path))
    artifact = os.path.join(REPO, config["artifact"])
    root = os.path.join(artifact, config.get("root", "🦀️.rs"))
    crate = Crate(root)
    layers = json.load(open(os.path.join(base, config["layers"]), encoding="utf-8"))
    rules = [(re.compile(r), l) for r, l in layers["rules"]]

    def layer(path):
        rel = os.path.relpath(path, artifact)
        for rule, name in rules:
            if rule.search(rel):
                return name
        return "?"

    source_manifest = os.path.join(artifact, config["manifest"]) if config.get("manifest") else os.path.join(os.path.dirname(root), "Cargo.toml")
    if not os.path.exists(source_manifest):
        raise SystemExit(f"no manifest beside {root}")
    source_text = open(source_manifest, encoding="utf-8").read()
    parts = sections(source_text)
    normal = dep_rows(parts.get("dependencies", []))
    dev = dep_rows(parts.get("dev-dependencies", []))
    known = {}
    for table, rows in (("dependencies", normal), ("dev-dependencies", dev)):
        for name, value in rows.items():
            known.setdefault(name.replace("-", "_"), {})[table] = (name, value)
    files, test_files = {}, set()
    for name in list(config["crates"]) + [config["aggregator"]]:
        layer_root = os.path.join(artifact, config["crates"][name]["dir"], "🦀️.rs") if name in config["crates"] else root
        if not os.path.exists(layer_root):
            continue
        layer_crate = Crate(layer_root) if layer_root != root else crate
        test_files |= layer_crate.test_files
        files[name] = [path for path in layer_crate.files if layer(path) == name or path == layer_root]
    idents = {}
    for name, paths in files.items():
        lib, test = set(), set()
        for path in paths:
            text = re.sub(r"(?<![\w:])::", "  ", strip(open(path, encoding="utf-8").read()))
            is_test = path in test_files or "🧪️tests" in os.path.relpath(path, artifact)
            spans = cut["test_spans"](text) if not is_test else []
            for m in re.finditer(r"(?<![\w:])([a-z][a-z0-9_]*)\s*::", text):
                (test if is_test or any(a <= m.start() < b for a, b in spans) else lib).add(m.group(1))
            for m in re.finditer(r"\bextern\s+crate\s+([a-z][a-z0-9_]*)", text):
                (test if is_test else lib).add(m.group(1))
        idents[name] = (lib, test)
    aggregator = config["aggregator"]
    packages = {name: spec["package"] for name, spec in config["crates"].items()}
    packages[aggregator] = config["crate"].replace("_", "-")
    allow = layers["allow"]
    workspace_root = os.path.normpath(os.path.join(os.path.dirname(source_manifest), re.search(r'(?m)^workspace\s*=\s*"([^"]+)"', source_text).group(1), "Cargo.toml"))
    writes = {}
    for name in list(config["crates"]) + [aggregator]:
        lib, test = idents.get(name, (set(), set()))
        header = config.get("header", {}).get(name, [])
        for line in header:
            m = re.match(r"\s*extern\s+crate\s+([a-z][a-z0-9_]*)", line)
            if m:
                lib.add(m.group(1))
        new_dir = os.path.join(artifact, config["crates"][name]["dir"]) if name != aggregator else os.path.dirname(source_manifest)
        deps, devs = {}, {}
        for keep in config.get("keep", []):
            if keep.replace("-", "_") in known and keep.replace("-", "_") not in test:
                lib.add(keep.replace("-", "_"))
        for ident in sorted(lib | test):
            if ident not in known:
                continue
            entry = known[ident]
            if ident in lib and "dependencies" in entry:
                deps[entry["dependencies"][0]] = reroot(entry["dependencies"][1], os.path.dirname(source_manifest), new_dir)
                if "dev-dependencies" in entry:
                    devs[entry["dev-dependencies"][0]] = reroot(entry["dev-dependencies"][1], os.path.dirname(source_manifest), new_dir)
            else:
                pick = entry.get("dev-dependencies") or entry.get("dependencies")
                devs[pick[0]] = reroot(pick[1], os.path.dirname(source_manifest), new_dir)
        for other, package in packages.items():
            if other == name:
                continue
            ident = package.replace("-", "_")
            if other in allow.get(name, []) or ident in lib:
                deps[package] = "{ workspace = true }"
            elif ident in test:
                devs[package] = "{ workspace = true }"
        for row in config.get("extra", {}).get(name, []):
            key, value = row.split("=", 1)
            deps[key.strip()] = value.strip()
        description = config.get("descriptions", {}).get(name)
        if name == aggregator:
            text = source_text
            def replace_table(text, table, rows):
                body = "\n".join(f"{k} = {v}" for k, v in sorted(rows.items()))
                pattern = re.compile(r"(?ms)^\[" + re.escape(table) + r"\]\s*$.*?(?=^\[|\Z)")
                block = f"[{table}]\n{body}\n\n" if rows else ""
                if pattern.search(text):
                    return pattern.sub(lambda _m: block, text, count=1)
                return text.rstrip("\n") + "\n\n" + block
            text = replace_table(text, "dependencies", deps)
            text = replace_table(text, "dev-dependencies", devs)
            writes[source_manifest] = text.rstrip("\n") + "\n"
            continue
        workspace = os.path.relpath(os.path.dirname(workspace_root), new_dir)
        lines = ["[package]", f'workspace = "{workspace}"', f'name = "{packages[name]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true"]
        if description:
            lines.append(f'description = "{description}"')
        lines += ["", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "🦀️.rs"', ""]
        if deps:
            lines += ["[dependencies]"] + [f"{k} = {v}" for k, v in sorted(deps.items())] + [""]
        if devs:
            lines += ["[dev-dependencies]"] + [f"{k} = {v}" for k, v in sorted(devs.items())] + [""]
        writes[os.path.join(new_dir, "Cargo.toml")] = "\n".join(lines)
    ws_text = open(workspace_root, encoding="utf-8").read()
    inline = re.search(r"(?m)^" + re.escape(packages[aggregator]) + r"\s*=\s*\{[^\n]*\}\n", ws_text)
    table = re.search(r"(?m)^\[workspace\.dependencies\." + re.escape(packages[aggregator]) + r"\]\s*\n(?:[^\[\n][^\n]*\n)*", ws_text)
    if not inline and not table:
        tables = list(re.finditer(r"(?m)^\[workspace\.dependencies\.[^\]]+\]\s*\n(?:[^\[\n][^\n]*\n)*", ws_text))
        if not tables:
            raise SystemExit(f"{workspace_root} declares no workspace dependency table")
        table = tables[-1]
    insert = ""
    for name, spec in config["crates"].items():
        if re.search(r"(?m)^(?:" + re.escape(spec["package"]) + r"\s*=|\[workspace\.dependencies\." + re.escape(spec["package"]) + r"\])", ws_text):
            continue
        relative = os.path.relpath(os.path.join(artifact, spec["dir"]), os.path.dirname(workspace_root))
        insert += f'{spec["package"]} = {{ path = "{relative}" }}\n' if inline else f'\n[workspace.dependencies.{spec["package"]}]\npath = "{relative}"\n'
    at = (inline or table).end()
    writes[workspace_root] = ws_text[:at] + insert + ws_text[at:]
    for path, text in writes.items():
        print(f"== {os.path.relpath(path, REPO)}")
        if mode == "plan":
            print(text if path != workspace_root else insert)
    if mode == "apply":
        for path, text in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)


main()
