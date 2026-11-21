"""🔌️ Checks the live Stdio PDF assembly against its actual emitted WASI dependencies."""

import difflib
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated/hub-assembly"
out.mkdir(exist_ok=True)
run = Path(sys.argv[1])
root = Path.cwd()
base = root / "🌎️hub/🧩️compositions/🗄️stdio"
pdf = base / "🧩️extensions/📘️pdf/🦀️.rs"
stdio = base / "🔌️plugin/🦀️.rs"

def candidate(path, app):
    original = path.read_text()
    before = f"Plugin::<{app}>::builder("
    after = f"PluginBuilder::<_, {app}>::new("
    assert original.count(before) + original.count(after) == 1, path
    output = original.replace(before, after)
    if "PluginBuilder" not in output.partition("semio_framework_dispatch_macros::dyn_enum_close!")[0]:
        output = "use semio_plugin::PluginBuilder;\n" + output if not output.startswith("//!") else output.replace("\n\n", "\n\nuse semio_plugin::PluginBuilder;\n", 1)
    if path == pdf and "use semio_plugin::__value as semio_framework_value;" not in output:
        output = output.replace("use semio_plugin::__semio_dispatch_PluginApp;", "use semio_plugin::{__semio_dispatch_PluginApp, __value as semio_framework_value};")
    return original, output

changes = [candidate(pdf, "StdioPdfApps"), candidate(stdio, "StdioApps")]
for path, (original, corrected) in zip([pdf, stdio], changes):
    relative = str(path.relative_to(root))
    if corrected != original:
        print("[DEBUG] staging", relative, hashlib.sha256(path.read_bytes()).hexdigest())
    (out / ("pdf.patch" if path == pdf else "stdio.patch")).write_text(
        "".join(difflib.unified_diff(original.splitlines(keepends=True), corrected.splitlines(keepends=True), "a/" + relative, "b/" + relative)))
if "--stage-only" in sys.argv:
    raise SystemExit

g = run / "l"
units = json.loads((g / "unit-graph-prototype-00.json").read_text())["units"]
def package(i):
    return units[i]["pkg_id"].split("#")[-1].split("@")[0]

target = next(i for i, r in enumerate(units) if package(i) == "semio-hub-stdio-pdf" and r["platform"] == "wasm32-wasip2")
wanted = set()
pending = [d["index"] for d in units[target]["dependencies"]]
while pending:
    i = pending.pop()
    if i in wanted:
        continue
    wanted.add(i)
    pending.extend(d["index"] for d in units[i]["dependencies"])
keys = {(units[i]["pkg_id"], units[i]["target"]["name"], units[i]["platform"] == "wasm32-wasip2"): i for i in wanted if units[i]["mode"] == "build"}
products = {}
for log in g.glob("cargo-*.jsonl"):
    for line in log.open():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if item.get("reason") != "compiler-artifact":
            continue
        paths = [Path(value) for value in item["filenames"]]
        key = (item["package_id"], item["target"]["name"], any("/wasm32-wasip2/" in str(path) for path in paths))
        if key not in keys:
            continue
        eligible = [path for path in paths if path.suffix in {".rmeta", ".rlib", ".dylib", ".so", ".dll"}]
        if eligible:
            products[keys[key]] = max(eligible, key=lambda path: path.suffix != ".rlib")
deps = out / "deps"
deps.mkdir(exist_ok=True)
missing = {path.name for path in products.values() if not path.exists()}
objects = {}
if missing:
    store_owner = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts"
    query = f'const p=await import("./{store_owner}"); console.log(p.casObjectPathV1(p.casRootV1(),"0".repeat(64)));'
    example = Path(subprocess.check_output(["repo", "tools", "exec", "bun", "--light", "--", "-e", query], text=True).strip())
    cache = example.parent.parent
    for file in Path(".cache/b/r/m/u/e").glob("*/*/*.json"):
        if not missing:
            break
        try:
            raw = file.read_text()
        except FileNotFoundError:
            continue
        if not missing.intersection(re.findall(r'"name":"([^"]+)"', raw)):
            continue
        receipt = json.loads(raw)
        for row in receipt["outputs"]:
            if row["name"] not in missing or row.get("template"):
                continue
            digest = row["sha256"]
            blob = cache / digest[:2] / digest[2:]
            if blob.exists() and blob.stat().st_size == row["bytes"]:
                objects[row["name"]] = blob
                missing.remove(row["name"])
links = {}
for i, path in products.items():
    owner = path if path.exists() else objects.get(path.name)
    if owner is None:
        continue
    link = deps / path.name
    if link.is_symlink():
        if link.resolve() != owner.resolve():
            assert not link.exists(), link
            link.unlink()
    if not link.is_symlink():
        link.symlink_to(owner)
    links[i] = link
print("[DEBUG] exact artifacts", len(links), "CAS artifacts", len(objects), "missing", len(missing))
toolchain = subprocess.check_output(["repo", "tools", "exec", "rustup", "--light", "--", "show", "active-toolchain"], text=True).split()[0]
command = ["repo", "tools", "exec", "rustup", "--light", "--", "run", toolchain, "rustc",
           "--crate-name", "semio_hub_stdio_pdf", "--crate-type", "lib", "--edition", "2024",
           "--target", "wasm32-wasip2", "--emit", "metadata", "-o", str(out / "pdf.rmeta"),
           "-L", "dependency=" + str(deps)]
for dep in units[target]["dependencies"]:
    i = dep["index"]
    assert i in links, (package(i), dep)
    command.extend(["--extern", f"{dep.get('extern_crate_name') or units[i]['target']['name']}={links[i]}"])
env = dict(os.environ, CARGO_PKG_VERSION="0.1.0", CARGO_MANIFEST_DIR=str(pdf.parent / "📦️packages/🦀️rust"))
mode = "before" if "--before" in sys.argv else "after"
result = subprocess.run(command + ["-"], input=changes[0][0 if mode == "before" else 1], env=env, text=True, capture_output=True)
(out / (mode + ".log")).write_text(result.stderr)
if mode == "before":
    assert result.returncode and "named `builder`" in result.stderr and "semio_framework_value" in result.stderr, result.stderr
    print("[DEBUG] Captured WASI dependencies reproduced the two PDF hub errors")
else:
    if result.returncode:
        print(result.stderr)
        raise SystemExit(result.returncode)
    print("[DEBUG] Full corrected PDF hub assembly compiled as WASI metadata")
