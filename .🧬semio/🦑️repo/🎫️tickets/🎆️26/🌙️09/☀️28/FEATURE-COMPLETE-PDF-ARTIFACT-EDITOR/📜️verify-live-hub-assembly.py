"""🔬 Type-checks the reviewed Hub PDF assembly against one coherent existing WASI lane."""

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

lane = Path(sys.argv[1]).resolve()
ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/hub-assembly"
root = Path.cwd()
config = root / "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf"
source = config / "🦀️.rs"
live = source.read_text()
before = live.replace("PluginAssemblyError, PluginBuilder", "PluginAssemblyError").replace("PluginBuilder::<_, StdioPdfApps>::new(", "Plugin::<StdioPdfApps>::builder(")
receipt = json.loads((output / "reviewed-hub-assembly.json").read_text())[str(source.relative_to(root))]
assert hashlib.sha256(live.encode()).hexdigest() == receipt["postimage"]
assert hashlib.sha256(before.encode()).hexdigest() == receipt["preimage"]
target = lane / "wasm32-wasip2/prototype/build"
host = lane / "prototype/build"
search = output / ("lane-" + lane.name)
search.mkdir(exist_ok=True)
files = [p for group in [target, host] for p in group.glob("*/*/out/lib*") if p.suffix in {".rmeta", ".dylib", ".so", ".dll", ".rlib"}]
for p in files:
    link = search / p.name
    if not link.is_symlink():
        link.symlink_to(p)
    else:
        assert p == link.resolve(), link

def library(package):
    root = host if package == "semio_framework_dispatch_macros" else target
    suffix = ".dylib" if root == host else ".rmeta"
    candidates = [p for p in files if p.is_relative_to(root) and p.name.startswith("lib" + package + "-") and p.suffix == suffix]
    assert len(candidates) == 1, (package, candidates)
    return candidates[0]

toolchain = subprocess.check_output(["repo", "tools", "exec", "rustup", "--light", "--", "show", "active-toolchain"], text=True).split()[0]
compiler = ["repo", "tools", "exec", "rustup", "--light", "--", "run", toolchain, "rustc",
            "--crate-name", "semio_hub_stdio_pdf", "--crate-type=lib", "--edition=2024",
            "--target", "wasm32-wasip2", "--emit=metadata", "-o", str(output/"reviewed-pdf.rmeta"),
            ]
for directory in dict.fromkeys(p.parent for p in files):
    compiler += ["-L", "dependency=" + str(directory)]
deps = ["semio_framework", "pack", "semio_framework_dispatch_macros", "semio_framework_job",
        "semio_os_kernel", "semio_plugin", "semio_s_artifact_stdio_pdf", "semio_s_artifact_stdio_contract"]
for dep in deps:
    compiler += ["--extern", f"{dep}={library(dep)}"]
env = dict(os.environ, CARGO_PKG_VERSION="0.1.0", RUSTC_LOG="rustc_metadata::locator=info", CARGO_MANIFEST_DIR=str(config / "📦️packages/🦀️rust"))
for name, text, extra in [("prior", before, []), ("reviewed", live, ["--extern", f"semio_framework_value={library('semio_framework_value')}"])]:
    result = subprocess.run(compiler + extra + ["-"], input=text, env=env, text=True, capture_output=True)
    (output / f"{name}-pdf-wasi.log").write_text(result.stderr)
    if name == "prior":
        assert result.returncode and "named `builder`" in result.stderr and "semio_framework_value" in result.stderr, result.stderr[-3700:]
        print("[DEBUG] The existing WASI lane reproduced both original PDF hub failures")
    elif result.returncode:
        print(result.stderr)
        raise SystemExit(result.returncode)
    else:
        print("[DEBUG] The exact reviewed Hub PDF postimage compiled to WASI metadata")
