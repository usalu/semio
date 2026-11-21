"""Build and emit the actual mounted Trinity composition from the ticket-local lock."""

from pathlib import Path
import os
import re
import subprocess

root = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
owner = generated / "nd/trinity"
target = generated / "native-mcp"
lock = (owner / "Cargo.lock").read_text()
assert 'name = "flate2"\nversion = "1.1.9"' in lock
assert 'name = "semio-hub-trinity"\nversion = "0.1.0"\ndependencies = [\n "semio-framework",' in lock
toolchain = re.search(
    r'^channel\s*=\s*"([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.M
)[1]
rustc = subprocess.check_output(
    ["rustup", "which", "--toolchain", toolchain, "rustc"], text=True
).strip()
environment = {
    **os.environ,
    "SEMIO_BUILD_MODE": "prototype",
    "CARGO_HOME": str(root / ".cache/b/r/m/cargo/home"),
    "CARGO_TARGET_DIR": str(target),
    "CARGO_BUILD_BUILD_DIR": str(target),
    "CARGO_INCREMENTAL": "0",
    "CARGO_PROFILE_DEV_DEBUG": "0",
    "RUSTC": rustc,
    "RUSTC_WRAPPER": "",
    "RUSTFLAGS": "-Zthreads=1",
}
for key in ("CARGO_BUILD_TARGET_DIR", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WORKSPACE_WRAPPER"):
    environment.pop(key, None)
subprocess.run(
    [
        "rustup", "run", toolchain, "cargo", "build", "--locked", "--offline",
        "--manifest-path", str(owner / "Cargo.toml"), "--features", "trinity", "-j", "1",
    ],
    env=environment, check=True,
)
executable = target / "debug" / ("mcp-native-descriptor.exe" if os.name == "nt" else "mcp-native-descriptor")
subprocess.run(
    [str(executable), "trinity", str(generated / "descriptor-refresh/trinity")],
    check=True,
)
