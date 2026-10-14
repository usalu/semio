#!/usr/bin/env python3
"""🧾️ Writes the Cargo manifests of the remodeling crate family after `✂️crate-cut.py apply` (lane P2).

For every crate root the cut wrote it walks the crate's `#[path]` closure, collects which external crates and family
crates its production sources name (normal dependencies) and which only its test sources name (dev-dependencies), adds
the dependencies derive expansions name implicitly, keeps only the `extern crate … as dsl/protocol/store/framework_schema`
aliases the crate uses, and writes `<package dir>/Cargo.toml`. The aggregator keeps its package and gains the `editor`
integration test target. The artifact workspace gets every member and one workspace dependency row per crate.

usage: 🧾️manifests.py plan|apply
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "census", "exec"), ns)
Crate, strip = ns["Crate"], ns["strip"]
config = json.load(open(os.path.join(HERE, "📸️remodeling-cut.json"), encoding="utf-8"))
ART = os.path.join(REPO, config["artifact"])
WS = os.path.join(ART, "Cargo.toml")
AGG_DIR = os.path.join(ART, "📦️packages/🦀️rust")
F = "../../../../../🧰️framework"
# 📚️ dependency key -> (workspace row body relative to the artifact workspace, rust ident)
EXTERNAL = {
    "semio-framework": (None, "semio_framework"), "semio-framework-os": (None, "semio_framework_os"), "semio-framework-plugin": (None, "semio_framework_plugin"),
    "semio-framework-job": (None, "semio_framework_job"), "semio-framework-tool-run": (None, "semio_framework_tool_run"), "semio-framework-artifact-reference": (None, "semio_framework_artifact_reference"),
    "semio-framework-dsl-record-derive": (f'path = "{F}/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"', "semio_framework_dsl_record_derive"),
    "semio-framework-dsl-record": (f'path = "{F}/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust"', "semio_framework_dsl_record"),
    "semio-framework-async": (f'path = "{F}/🔨️modules/⏳️async/📦️packages/🦀️rust"', "semio_framework_async"),
    "semio-framework-pack-json": (f'path = "{F}/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust"', "semio_framework_pack_json"),
    "semio-framework-diagnostic": (f'path = "{F}/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust"', "semio_framework_diagnostic"),
    "semio-framework-dsl": (f'path = "{F}/🔨️modules/🗣️dsl/📦️packages/🦀️rust"', "semio_framework_dsl"),
    "semio-framework-ui-locale": (f'path = "{F}/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust"', "semio_framework_ui_locale"),
    "semio-framework-schema-composition": (f'path = "{F}/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust"', "semio_framework_schema_composition"),
    "semio-framework-schema-state": (f'path = "{F}/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust"', "semio_framework_schema_state"),
    "semio-framework-schema-registry": (f'path = "{F}/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust"', "semio_framework_schema_registry"),
    "semio-framework-2d": (f'path = "{F}/🔨️modules/◻️2d/📦️packages/🦀️rust"', "semio_framework_2d"),
    "semio-framework-value": (f'path = "{F}/🔨️modules/🌱️value/📦️packages/🦀️rust"', "semio_framework_value"),
    "number": (f'path = "{F}/🔨️modules/🔢️number/📦️packages/🦀️rust"\npackage = "semio-framework-number"', "number"),
    "geometry": (f'path = "{F}/🔨️modules/📐️geometry/📦️packages/🦀️rust"\npackage = "semio-framework-geometry"', "geometry"),
    "graph": (f'path = "{F}/🔨️modules/🕸️graph/📦️packages/🦀️rust"\npackage = "semio-framework-graph"', "graph"),
    "semio-framework-os-kernel": (f'path = "{F}/🛍️products/💻️os/📦️packages/🦀️rust"', "semio_framework_os_kernel"),
    "semio-framework-value-derive": (f'path = "{F}/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"', "semio_framework_value_derive"),
    "semio-framework-ui-contract": (f'path = "{F}/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"', "semio_framework_ui_contract"),
    "semio-framework-dispatch-macros": (f'path = "{F}/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"', "semio_framework_dispatch_macros"),
    "semio-framework-schema": (f'path = "{F}/🔨️modules/🧬️schema/📦️packages/🦀️rust"', "semio_framework_schema"),
    "pack": (f'path = "{F}/🔨️modules/🎒️pack/📦️packages/🦀️rust"\npackage = "semio-framework-pack"', "pack"),
    "base64_codec": (f'path = "{F}/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust"\npackage = "semio-framework-io-base64"', "base64_codec"),
    "semio-framework-pixels": (f'path = "{F}/🔨️modules/🔲️pixels/📦️packages/🦀️rust"', "semio_framework_pixels"),
    "semio-framework-async-macros": (f'path = "{F}/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"', "semio_framework_async_macros"),
    "serde": (None, "serde"), "serde_json": (None, "serde_json"),
}
for name in ["avi", "mp4", "gltf-model", "jpg", "json", "las", "obj", "ply", "png", "semio", "semio-base", "semio-drawing-export", "semio-drawing-import", "semio-image", "semio-image-export", "semio-image-import", "semio-mesh", "semio-mesh-export", "semio-mesh-import", "stl"]:
    EXTERNAL[f"semio-s-artifact-stdio-{name}"] = (None, f"semio_s_artifact_stdio_{name}".replace("-", "_"))
DEV_ONLY = {"png": ('"0.17.16"', "png")}
ALIASES = {"dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "framework_schema": "semio-framework-schema"}
IMPLIED = {"semio-framework-value-derive": ["semio-framework-value"], "semio-framework-dsl-record-derive": ["semio-framework-dsl-record", "semio-framework-value"], "semio-framework-os-kernel": ["semio-framework-value"]}
FAMILY = {name: spec["package"] for name, spec in config["crates"].items()}
FAMILY["aggregator"] = "semio-s-artifact-remodel-remodeling"

def root_of(name):
    if name == "aggregator":
        return os.path.join(ART, "🦀️.rs")
    return os.path.join(ART, config["crates"][name]["dir"], "🦀️.rs")

def integration_roots(name):
    """🧪️ The integration test roots a family package owns: `<package dir>/🧪️tests/<target>/🦀️.rs`."""
    if name in ("aggregator", "it-editor"):
        return []
    tests = os.path.join(os.path.dirname(root_of(name)), "🧪️tests")
    return sorted(os.path.join(tests, target, "🦀️.rs") for target in os.listdir(tests)) if os.path.isdir(tests) else []

def scan(name):
    crate = Crate(root_of(name))
    normal, dev, aliases, featured = set(), set(), set(), set()
    texts = dict(crate.text)
    for target in integration_roots(name):
        texts.update({path: text for path, text in Crate(target).text.items() if path not in texts})
    for path, text in texts.items():
        text = re.sub(r"(?m)^extern crate [a-z_]+ as [a-z_]+;\n", "", text)
        test = path in crate.test_files or "🧪️tests" in os.path.relpath(path, ART)
        words = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", text))
        target = dev if test else normal
        if test and ("protocol_laws" in words or "mutation_testing" in words):
            featured.add("semio-framework-os-kernel")
        if test and ("artifact_app_laws" in words or "history_edit_acceptance_law" in words):
            featured.add("semio-framework-plugin")
        for key, (_, ident) in EXTERNAL.items():
            if ident in words:
                target.add(key)
        for key, (_, ident) in DEV_ONLY.items():
            if ident in words and test:
                dev.add(key)
        for alias, key in ALIASES.items():
            if re.search(r"\b" + alias + r"\s*::", text):
                (dev if test else normal).add(key)
                aliases.add(alias)
        for other, package in FAMILY.items():
            if other != name and package.replace("-", "_") in words:
                target.add(package)
    for source in list(normal) + list(dev):
        for implied in IMPLIED.get(source, []):
            (normal if source in normal else dev).add(implied)
    dev -= normal
    dev |= featured
    return normal, dev, aliases, crate

def dep_line(key, dev=False):
    if key in DEV_ONLY:
        return f"{key} = {DEV_ONLY[key][0]}"
    if dev and key == "semio-framework-plugin":
        return f'{key} = {{ workspace = true, features = ["artifact-app-testing"] }}'
    if dev and key == "semio-framework-os-kernel":
        return f'{key} = {{ workspace = true, features = ["protocol-laws", "mutation-testing"] }}'
    return f"{key} = {{ workspace = true }}"

def manifest(name, normal, dev, workspace_rel, lib_path, description, role, extra=""):
    lines = ["[package]", f'workspace = "{workspace_rel}"', f'name = "{FAMILY[name]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{description}"', "",
             "[package.metadata.semio]", f'role = "{role}"', "", "[lints]", "workspace = true", "", "[lib]", f'path = "{lib_path}"', "", "[dependencies]"]
    lines += [dep_line(key) for key in sorted(normal)]
    if dev:
        lines += ["", "[dev-dependencies]"] + [dep_line(key, True) for key in sorted(dev)]
    return "\n".join(lines) + "\n" + extra

def main():
    mode = sys.argv[1]
    results = {}
    names = [name for name in config["crates"] if name != "it-editor"] + ["aggregator"]
    it_normal, it_dev, it_aliases, _ = scan("it-editor")
    for name in names:
        normal, dev, aliases, crate = scan(name)
        results[name] = (normal, dev, aliases)
    results["aggregator"][1].update((it_normal | it_dev) - results["aggregator"][0])
    results["aggregator"][1].discard("semio-s-artifact-remodel-remodeling")
    for name, (normal, dev, aliases) in results.items():
        family = sorted(k for k in normal if k.startswith("semio-s-artifact-remodel-remodeling"))
        print(f"{name}: deps={len(normal)} dev={len(dev)} family={[k.replace('semio-s-artifact-remodel-remodeling-', '') for k in family]} aliases={sorted(aliases)}")
    if mode != "apply":
        return
    for name in names:
        normal, dev, aliases = results[name]
        root = root_of(name)
        text = open(root, encoding="utf-8").read()
        for alias, key in ALIASES.items():
            if alias not in aliases and not (name == "aggregator" and alias in it_aliases):
                text = re.sub(r"(?m)^extern crate [a-z_]+ as " + alias + r";\n", "", text)
        open(root, "w", encoding="utf-8").write(text)
        if name == "aggregator":
            continue
        package_dir = os.path.dirname(root)
        workspace_rel = os.path.relpath(ART, package_dir)
        description = re.search(r"^//! \S+ (.*)$", text, re.M).group(1).replace('"', "'")
        extra = "".join(f'\n[[test]]\nname = "{re.sub(r"^[^a-z]+", "", os.path.basename(os.path.dirname(target)))}"\npath = "{os.path.relpath(target, package_dir)}"\n' for target in integration_roots(name))
        open(os.path.join(package_dir, "Cargo.toml"), "w", encoding="utf-8").write(manifest(name, normal, dev, workspace_rel, "🦀️.rs", description, "library", extra))
    it_root = root_of("it-editor")
    it_text = open(it_root, encoding="utf-8").read()
    for alias in ALIASES:
        if alias not in it_aliases:
            it_text = re.sub(r"(?m)^extern crate [a-z_]+ as " + alias + r";\n", "", it_text)
    open(it_root, "w", encoding="utf-8").write(it_text)
    normal, dev, aliases = results["aggregator"]
    agg_path = os.path.join(AGG_DIR, "Cargo.toml")
    old = open(agg_path, encoding="utf-8").read()
    tail = old[old.index("[package.metadata.semio.preparation]"):] if "[package.metadata.semio.preparation]" in old else ""
    extra = '\n[[test]]\nname = "editor"\npath = "../../🧪️tests/🔬️editor/🦀️.rs"\n' + ("\n" + tail if tail else "")
    open(agg_path, "w", encoding="utf-8").write(manifest("aggregator", normal, dev, "../..", "../../🦀️.rs", "remodel remodeling artifact: identity, definition and the standard/subset declaration of the remodeling family", "artifact", extra))
    ws = open(WS, encoding="utf-8").read()
    members = sorted(os.path.relpath(os.path.dirname(root_of(name)), ART) for name in names)
    ws = re.sub(r"members = \[[^\]]*\]", "members = [\n" + "".join(f'  "{member}",\n' for member in members) + "]", ws, count=1)
    rows = []
    for key, (body, _) in sorted(EXTERNAL.items()):
        if body and f"[workspace.dependencies.{key}]" not in ws:
            rows.append(f"[workspace.dependencies.{key}]\n{body}\n")
    for name in names:
        package = FAMILY[name]
        rel = os.path.relpath(os.path.dirname(root_of(name)), ART) if name != "aggregator" else "📦️packages/🦀️rust"
        if f"[workspace.dependencies.{package}]" not in ws:
            rows.append(f'[workspace.dependencies.{package}]\npath = "{rel}"\n')
    anchor = "[workspace.dependencies.serde]"
    ws = ws.replace(anchor, "".join(row + "\n" for row in rows) + anchor, 1)
    open(WS, "w", encoding="utf-8").write(ws)
    print("manifests written:", len(names), "workspace rows added:", len(rows))

main()
