from pathlib import Path
import os
import subprocess
import sys
import re

root = Path.cwd()
ticket = Path(__file__).parent
product_ticket = next((root / ".🧬semio").glob("*/🎫*/🎆26/*/*/GIS-TERRAIN-AND-OVERLAYS-END-TO-END"))
lane = product_ticket / "🗑️generated/n/debug/build"
output = ticket / "🗑️generated"
test = root / "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/📦️packages/🦀️rust"
dependencies = (
    "semio-framework-value-retirement-contract",
    "semio-framework-value-control-contract",
    "semio-framework-value-retained-clone",
    "semio-framework-value-retirement",
    "semio-framework-value-factory",
    "semio-framework-value-clone-source-testing",
    "serde_json",
    "semio-framework-value-testing-allocator",
    "semio-framework-value-testing-effect-bank",
)
rlibs = ("6141b90a634cec99", "2e46b9496aa1264e", "423379dce1cbc90a", "f76fc74c5dce2cd2", "ee38b5f14ed2765d", "22fb4dfde18a6492", "f0fe5b73b196585e", "577b44bb111c978c", "12cfcc2afa136b00")
toolchain = re.search(r"(?m)^channel = ([^\n]+)$", (root / "rust-toolchain.toml").read_text()).group(1).strip(chr(34))
binary = output / f"native-viewport-slot-{sys.argv[1]}"
rust = ["repo", "tools", "exec", "--light", "rustup", "--", "run", toolchain, "rustc"]
rust += ["--crate-name", "semio_framework_value_original_slot", "--edition=2021", "--test", str(test / "🦀️.rs"), "-o", str(binary)]
for name, fingerprint in zip(dependencies, rlibs):
    crate = name.replace("-", "_")
    library = lane / name / fingerprint / "out" / f"lib{crate}-{fingerprint}.rlib"
    assert library.is_file(), library
    metadata = library.with_suffix(".rmeta")
    rust += ["--extern", f"{crate}={metadata}", "--extern", f"{crate}={library}"]
library_directory = output / "vl"
library_directory.mkdir(exist_ok=True)
for library in lane.glob("*/*/out/lib*"):
    if library.suffix in {".rlib", ".rmeta", ".dylib"}:
        link = library_directory / library.name
        if not link.is_symlink():
            link.symlink_to(library)
rust.append(f"-Ldependency={library_directory}")
env = dict(os.environ, RUST_MIN_STACK="67108864", CARGO_MANIFEST_DIR=str(test))
with (output / f"native-viewport-slot-{sys.argv[1]}-direct.log").open("w") as log:
    print("[DEBUG] Linking the original-slot tests against the captured native target", file=log, flush=True)
    result = subprocess.run(rust, cwd=root, env=env, stdout=log, stderr=log)
    if result.returncode == 0:
        result = subprocess.run([str(binary), *sys.argv[2:]], cwd=root, env=env, stdout=log, stderr=log)
raise SystemExit(result.returncode)
