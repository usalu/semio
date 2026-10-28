"""🔧 Run focused energy checks with the ticket-owned Rust toolchain."""
from pathlib import Path
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
env = os.environ.copy()
env.update({"RUSTUP_HOME": str(generated / "rustup"), "CARGO_HOME": str(generated / "cargo"), "RUSTC": str(generated / "cargo/bin/rustc"), "RUSTC_WRAPPER": "", "RUSTC_WORKSPACE_WRAPPER": "", "CARGO_TARGET_DIR": str(generated / "target"), "CARGO_BUILD_BUILD_DIR": str(generated / "build"), "CC": str(generated / "cc"), "CXX": str(generated / "c++"), "AR": str(generated / "ar"), "RUSTFLAGS": "-Cdebuginfo=0 -Clinker=" + str(generated / "cc"), "CARGO_BUILD_JOBS": "2", "ZIG_GLOBAL_CACHE_DIR": str(generated / "zig-cache"), "ZIG_LOCAL_CACHE_DIR": str(generated / "zig-local"), "PATH": str(generated / "cargo/bin") + os.pathsep + str(generated / "bun-linux-x64") + os.pathsep + env["PATH"]})
raise SystemExit(subprocess.call([str(generated / "cargo/bin/cargo"), *sys.argv[1:]], env=env))
