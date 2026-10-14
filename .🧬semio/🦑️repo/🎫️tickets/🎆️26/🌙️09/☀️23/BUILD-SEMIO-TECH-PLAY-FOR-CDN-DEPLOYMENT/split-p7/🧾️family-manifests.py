#!/usr/bin/env python3
"""🧾️ Writes the Cargo manifests of a family cut by `✂️family-cut.py` (split lane P7).

Every family crate gets `<owner>/📦️packages/🦀️rust/Cargo.toml` (`[lib] path = "../../🦀️.rs"`) declaring exactly the
dependencies its sources name: a monolith dependency whose extern name (or one of its `extern crate … as` aliases) its
library sources name goes to `[dependencies]`, one only its test code names goes to `[dev-dependencies]` (with the
monolith's dev feature set), and every family crate it names is a dependency too. The aggregator manifest is rewritten
the same way, keeping its own package tables. Every dependency is a `{ workspace = true }` row; the artifact workspace
root gains a `[workspace.dependencies.<name>]` row for every dependency that was an inline path, and for every family
crate. `[[test]]` targets come from the config `tests` and `existing_tests`.

usage: 🧾️family-manifests.py <config.json> plan|apply
"""
import importlib.util, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("family_cut", os.path.join(HERE, "✂️family-cut.py"))
fc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fc)
def toml_loads(text):
    out = subprocess.run(["bun", "-e", "const t=await Bun.stdin.text();console.log(JSON.stringify(Bun.TOML.parse(t)))"], input=text, capture_output=True, text=True, check=True).stdout
    return json.loads(out)


WORDY = {"math", "graph", "machine", "pack", "store", "dsl", "protocol", "kernel"}


def spec_of(value):
    return value if isinstance(value, dict) else {"version": value}


def names_in(text, extern):
    if extern in WORDY:
        return re.search(r"(?<![\w.$])(?:::)?" + re.escape(extern) + r"\s*(?:::|!|\))", text) is not None
    return re.search(r"(?<![\w.$])" + re.escape(extern) + r"\b(?!\s*:(?!:))", text) is not None


def without_tests(text):
    spans = fc.test_spans(text)
    out, last = [], 0
    for a, b in sorted(spans):
        if a < last:
            continue
        out.append(text[last:a])
        last = b
    out.append(text[last:])
    return "".join(out)


def row(name, value, features=None, dev=False):
    parts = ["workspace = true"]
    feats = list(value.get("features", [])) if isinstance(value, dict) else []
    if features is not None:
        feats = features
    if feats:
        parts.append("features = [" + ", ".join(f'"{f}"' for f in feats) + "]")
    if isinstance(value, dict) and value.get("optional"):
        parts.append("optional = true")
    return f"{name} = {{ {', '.join(parts)} }}"


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    stem = os.path.basename(config_path).rsplit(".", 1)[0]
    plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))["family"]
    artifact = os.path.join(REPO, config["artifact"])
    monolith_dir = os.path.join(artifact, "📦️packages/🦀️rust")
    monolith_path = os.path.join(monolith_dir, "Cargo.toml")
    original = os.path.join(os.path.dirname(os.path.abspath(config_path)), "🧾️monolith-Cargo.toml")
    monolith_text = open(original if os.path.exists(original) else monolith_path, encoding="utf-8").read()
    monolith = toml_loads(monolith_text)
    root_path = os.path.join(artifact, "Cargo.toml")
    root_text = open(root_path, encoding="utf-8").read()
    root = toml_loads(root_text)
    rows = dict(root.get("workspace", {}).get("dependencies", {}))
    deps, devs = dict(monolith.get("dependencies", {})), dict(monolith.get("dev-dependencies", {}))
    aliases = config.get("aliases", {})
    externs = {}
    for key in list(deps) + list(devs):
        externs.setdefault(key, key.replace("-", "_"))
    family = {name: name.replace("-", "_") for name in plan}
    package_dir = {name: (monolith_dir if name == config["crate"].replace("_", "-") else os.path.join(artifact, plan[name]["owner"], "📦️packages/🦀️rust")) for name in plan}
    new_rows = []
    for key in list(deps) + list(devs):
        value = deps.get(key, devs.get(key))
        if isinstance(value, dict) and "path" in value and key not in rows:
            absolute = os.path.normpath(os.path.join(monolith_dir, value["path"]))
            entry = {"path": os.path.relpath(absolute, artifact)}
            if "package" in value:
                entry["package"] = value["package"]
            if value.get("default-features") is False:
                entry["default-features"] = False
            rows[key] = entry
            new_rows.append((key, entry))
    for name in plan:
        if name not in rows:
            entry = {"path": os.path.relpath(package_dir[name], artifact)}
            rows[name] = entry
            new_rows.append((name, entry))
    tests = {}
    for t in config.get("tests", []) + config.get("existing_tests", []):
        tests.setdefault(t["package"], []).append(t)
    writes = {}
    for name, info in plan.items():
        lib_text = "\n".join(without_tests(fc.strip(open(os.path.join(artifact, f), encoding="utf-8").read())) for f in info["lib"])
        lib_full = "\n".join(fc.strip(open(os.path.join(artifact, f), encoding="utf-8").read()) for f in info["lib"])
        test_text = "\n".join(fc.strip(open(os.path.join(artifact, f), encoding="utf-8").read()) for f in info["test"])
        test_text += "\n" + "\n".join(fc.strip(open(os.path.join(artifact, f), encoding="utf-8").read()) for t in tests.get(name, []) for f in [t["file"]])
        test_text += "\n" + lib_full
        own = family[name]
        normal, dev = [], []
        for key, extern in externs.items():
            hits = [extern] + [alias for alias, target in aliases.items() if target == extern]
            in_lib = any(names_in(lib_text, h) for h in hits)
            in_test = any(names_in(test_text, h) for h in hits)
            if key in deps and in_lib:
                normal.append(row(key, deps[key]))
            if in_test and (key in devs or (key in deps and not in_lib)):
                dev.append(row(key, devs.get(key, deps.get(key))))
        for other, ident in family.items():
            if other == name:
                continue
            if re.search(r"(?<![\w])" + ident + r"\b", lib_text):
                normal.append(f"{other} = {{ workspace = true }}")
            elif re.search(r"(?<![\w])" + ident + r"\b", test_text):
                dev.append(f"{other} = {{ workspace = true }}")
        is_aggregator = name == config["crate"].replace("_", "-")
        pkg = package_dir[name]
        workspace_rel = os.path.relpath(artifact, pkg)
        lines = []
        if is_aggregator:
            head = re.split(r"(?m)^\[dependencies\]\s*$", monolith_text)[0].rstrip()
            head = re.sub(r"(?ms)^\[\[test\]\].*?(?=^\[|\Z)", "", head).rstrip()
            lines.append(head)
        else:
            description = next((c.get("description", name) for c in config["crates"] if c["package"] == name), name)
            lines += ["[package]", f'workspace = "{workspace_rel}"', f'name = "{name}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{description}"', "",
                      "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"']
        for t in tests.get(name, []):
            lines += ["", "[[test]]", f'name = "{t["name"]}"', f'path = "{os.path.relpath(os.path.join(artifact, t["file"]), pkg)}"']
        lines += ["", "[dependencies]"] + sorted(set(normal))
        if dev:
            lines += ["", "[dev-dependencies]"] + sorted(set(dev))
        text = "\n".join(lines) + "\n"
        writes[os.path.join(pkg, "Cargo.toml")] = text
        print(f"{name}: {len(set(normal))} deps, {len(set(dev))} dev, {len(tests.get(name, []))} tests")
    if new_rows:
        addition = "".join(f'\n[workspace.dependencies.{key}]\n' + "".join(f'{k} = {json.dumps(v, ensure_ascii=False)}\n' if not isinstance(v, bool) else f'{k} = {str(v).lower()}\n' for k, v in entry.items()) for key, entry in new_rows)
        anchor = re.search(r"(?m)^\[workspace\.lints", root_text)
        at = anchor.start() if anchor else len(root_text)
        root_text = root_text[:at].rstrip("\n") + "\n" + addition + "\n" + root_text[at:]
    root_text = re.sub(r'(?m)^\s*"\*\*/🏅️standards/\*\*",?\n', "", root_text)
    writes[root_path] = root_text
    if mode == "apply":
        for path, text in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
        print(f"wrote {len(writes)} manifests")


main()
