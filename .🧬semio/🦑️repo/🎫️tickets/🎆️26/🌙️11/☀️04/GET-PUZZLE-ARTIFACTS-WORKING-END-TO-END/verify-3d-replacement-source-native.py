import argparse
import os
import subprocess
import sys
from pathlib import Path


ticket = Path(__file__).resolve().parent
repository = ticket.parents[6]
generated = ticket / "🗑️generated"
parser = argparse.ArgumentParser(description="Verify the current replacement Source against a warm native dependency graph.")
parser.add_argument("--deps", type=Path, default=generated / "3d-history-report-field-probe" / "deps")
parser.add_argument("--native-build", type=Path, default=generated / "cargo-puzzle-clipboard-build" / "debug" / "build")
parser.add_argument("--execute", action="store_true", help="Link and execute the focused native witness.")
options = parser.parse_args()
deps = options.deps.resolve()
native_build = options.native_build.resolve()
if not native_build.is_dir():
    parser.error(f"Native cached build directory is absent: {native_build}")

source = ticket / "3d-replacement-source-native-witness.rs"
args = [
    os.environ.get("RUSTC", "rustc"),
    "--edition=2021", "--test", "-Cdebuginfo=0", "-Zthreads=1",
    "--crate-name", "replacement_source_probe", "-L", f"dependency={deps}",
]
if sys.platform.startswith("linux"):
    args.extend(("-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options"))
for name in (
    "semio_os_store_operation_wire", "semio_os_store_one_item_publication", "semio_os_spr",
    "semio_framework_job", "semio_framework_value", "semio_framework_value_native_owned",
    "semio_framework_pack_json", "semio_plugin_testing_laws", "semio_plugin_time_travel",
):
    libraries = sorted(deps.glob(f"lib{name}-*.rlib"))
    if len(libraries) != 1 or not libraries[0].with_suffix(".rmeta").is_file():
        parser.error(f"Expected one cached .rlib and .rmeta pair for {name} in {deps}")
    lib = libraries[0]
    args.extend(("--extern", f"{name}={lib}", "--extern", f"{name}={lib.with_suffix('.rmeta')}"))

for output in sorted(native_build.glob("*/*/out")):
    if any(path.suffix in (".a", ".lib") for path in output.iterdir()):
        args.extend(("-L", f"native={output}"))
if not options.execute:
    print(f"Found the native dependency pairs and linker directory for {source.name}.")
    sys.exit(0)

generated.mkdir(exist_ok=True)
environment = dict(os.environ)
cargo_home = repository / ".cache" / "b" / "r" / "m" / "cargo" / "home"
if cargo_home.is_dir():
    environment.setdefault("CARGO_HOME", str(cargo_home))
environment.setdefault("ZIG_GLOBAL_CACHE_DIR", str(generated / "zig-clipboard" / "global"))
environment.setdefault("ZIG_LOCAL_CACHE_DIR", str(generated / "zig-clipboard" / "local"))
environment.setdefault("RUST_MIN_STACK", "134217728")
binary = generated / ("3d-replacement-source-native-witness.exe" if os.name == "nt" else "3d-replacement-source-native-witness")
subprocess.run(args + ["-o", str(binary), str(source)], cwd=repository, env=environment, check=True)
subprocess.run([binary, "--nocapture"], cwd=repository, env=environment, check=True)
