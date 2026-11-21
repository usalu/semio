import json
from pathlib import Path
import os
import subprocess
import sys

workspace = Path.cwd()
out = Path(__file__).resolve().parent / "🗑️generated" / "construction-native"
out.mkdir(exist_ok=True)
target = out / "t"
build = out / "b"
job = workspace / "🧰️framework/🔨️modules/🧵️job"
modules = workspace / "🧰️framework/🔨️modules"
packages = {
    "semio-framework-job": job,
    "semio-framework-value": modules / "🌱️value",
    "semio-framework-value-retained-clone": modules / "🌱️value/🧬️retained-clone/🧬️contract",
    "semio-framework-value-native-owned": modules / "🌱️value/🛬️decode/📦️owner",
    "semio-framework-value-testing-allocator": modules / "🌱️value/🧪️testing/💾️allocator",
    "semio-framework-test-fixtures": modules / "🧪️test/🧲️fixture-reader",
    "semio-framework-trace": modules / "📊️trace",
}
if "--presentation" in sys.argv:
    packages.update({
        "semio-framework-tool-run": modules / "⏯️tool-run",
        "semio-framework-tool-run-presentation": modules / "⏯️tool-run/📊️progress/🪟️presentation",
        "semio-framework-tool-run-entities": modules / "⏯️tool-run/👥️entities",
        "semio-framework-tool-run-tick-wire": modules / "⏯️tool-run/🎞️tick",
    })
if any(not (p / "📦️packages/🦀️rust/Cargo.toml").is_file() for p in packages.values()):
    for path in modules.glob("**/Cargo.toml"):
        if "📦️packages" not in str(path):
            continue
        raw = path.read_text()
        for name in packages:
            if f'name = "{name}"\n' in raw:
                packages[name] = path.parent.parent.parent
manifest = ["[package]", 'name = "construction-verification"', 'version = "0.0.0"', 'edition = "2021"', "[workspace]", "[lib]", 'path = "lib.rs"', "[dependencies]", 'serde_json = "1"']
for name, path in packages.items():
    metadata = path / "📦️packages/🦀️rust"
    if not (metadata / "Cargo.toml").is_file():
        raise RuntimeError(f"No manifest for {name}: {metadata}")
    feature = ', features = ["testkit"]' if name == "semio-framework-job" else ""
    manifest.append(name + " = { path = " + json.dumps(str(metadata), ensure_ascii=False) + feature + " }")
manifest.extend(("[profile.dev]", "debug = 0", "incremental = false"))
(out / "Cargo.toml").write_text("\n".join(manifest) + "\n")
(out / "lib.rs").write_text("")
toolchain, = (workspace / ".cache/b/t/m/r").glob("*/toolchains/nightly-2026-07-20-*")
env = {**os.environ, "CARGO_HOME": str(workspace / ".cache/b/r/m/cargo/home"), "CARGO_INCREMENTAL": "0"}
args = ["repo", "tools", "exec", "cargo", "--", "build", "--offline", "--jobs", "1", "--manifest-path", str(out / "Cargo.toml"),
        "--target-dir", str(target), "--config", f'build.build-dir="{build}"', "--config", f'build.rustc="{toolchain / "bin/rustc"}"',
        "--config", 'build.rustc-wrapper=""', "--message-format=json"]
with (out / "cargo.jsonl").open("w") as log:
    subprocess.run(args, stdout=log, env=env, check=True)
