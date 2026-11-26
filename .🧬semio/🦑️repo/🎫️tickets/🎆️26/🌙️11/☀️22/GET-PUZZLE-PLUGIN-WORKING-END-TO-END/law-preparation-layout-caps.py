import os
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
work = ticket / "🗑️generated" / "law-preparation-layout-caps"
work.mkdir(parents=True, exist_ok=True)
value = root / "🧰️framework/🔨️modules/🌱️value"
parts = {
    "semio-framework-value": "",
    "semio-framework-value-retained-clone": "🧬️retained-clone/🧬️contract",
    "semio-framework-value-derive": "✨️derive",
    "semio-framework-value-paged": "📦️paged",
    "semio-framework-value-paged-list": "📋️list",
    "semio-framework-value-persistent": "📋️list/🔗️persistent",
    "semio-framework-value-retirement": "♻️retirement",
    "semio-framework-value-testing-effect-bank": "🧪️testing/📥️admission/🎟️effect",
    "semio-framework-value-testing-allocator": "🧪️testing/💾️allocator",
    "semio-framework-value-clone-source": "🧬️retained-clone/🔗️source",
    "semio-framework-value-clone-source-testing": "🧬️retained-clone/🔗️source/🧪️testing",
}
dependencies = "\n".join(f'{name} = {{ path = "{value / path / "📦️packages/🦀️rust"}" }}' for name, path in parts.items())
manifest = work / "Cargo.toml"
manifest.write_text(f'''
[workspace]
[package]
name = "law-preparation-layout-caps"
version = "0.0.0"
edition = "2021"
[dependencies]
serde = {{ version = "1.0.219", features = ["derive"] }}
{dependencies}
[[test]]
name = "caps"
path = "{ticket / "law-preparation-layout-caps.rs"}"
''')
artifacts = ticket / "🗑️generated/law-framework-only"
toolchain = root / ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
os.environ.update(
    CARGO_HOME=str(root / ".cache/b/r/m/cargo/home"),
    ZIG_GLOBAL_CACHE_DIR=str(artifacts / "zig"),
    ZIG_LOCAL_CACHE_DIR=str(artifacts / "zig-local"),
    RUSTC=str(toolchain / "rustc"),
    RUSTC_WRAPPER="",
    CARGO_BUILD_BUILD_DIR=str(artifacts / "b"),
    CARGO_TARGET_DIR=str(artifacts / "t"),
    RUST_MIN_STACK="134217728",
    CARGO_PROFILE_TEST_DEBUG="0",
    CARGO_INCREMENTAL="0",
    RUSTFLAGS="-Z threads=1",
)
print("[DEBUG] Building funded nested-clone checks in an isolated dependency graph", flush=True)
raise SystemExit(subprocess.run([toolchain / "cargo", "test", "--offline", "-j1", "--manifest-path", manifest,
                                  "--test", "caps", "--", "--nocapture"], cwd=root).returncode)

