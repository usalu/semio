"""🔬️ Checks the exact reactor Work and Store-merge helper with governed types."""

import json
from pathlib import Path
import re
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
BASE = Path(sys.argv[1]).resolve()
WASM = BASE.parent.parent.name == "wasm32-wasip2" or "--wasm" in sys.argv
OUT = TICKET / "🗑️generated/raw-store-inbox/exact"
OUT.mkdir(parents=True, exist_ok=True)
SEARCH = OUT / "deps"
SEARCH.mkdir(exist_ok=True)
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
ARTIFACTS = {p.name: p for p in BASE.glob("*/*/out/*") if p.suffix == ".rmeta" or not WASM and p.suffix in (".so", ".dylib", ".dll")}
if WASM:
    ARTIFACTS.update((p.name, p) for p in (BASE.parents[2] / "prototype/build").glob("*/*/out/*") if p.suffix in (".so", ".dylib", ".dll"))
for name, path in ARTIFACTS.items():
    link = SEARCH / name
    if link.is_symlink():
        if link.resolve() == path:
            continue
        link.unlink()
    link.symlink_to(path)


def unique(name):
    [artifact] = [p for filename, p in ARTIFACTS.items() if filename.startswith(f"lib{name}-") and p.suffix == ".rmeta"]
    return artifact


def exact_dependencies(artifact):
    metadata = subprocess.run([*RUST, "-Zls=root", str(artifact)], check=True, capture_output=True, text=True).stdout
    identities = re.findall(r"^\d+ ([\w]+)-([a-f0-9]+) hash ", metadata, re.M)
    direct = {name: path for name, digest in identities for path in [next((ARTIFACTS.get(f"lib{name}-{digest}{ext}") for ext in (".rmeta", ".dylib", ".so", ".dll") if f"lib{name}-{digest}{ext}" in ARTIFACTS), None)] if path is not None}
    return direct


def compile(crate, source, dependencies):
    result = OUT / f"lib{crate}.rmeta"
    inputs = ["--crate-name", crate, "--crate-type=rlib", "--edition=2021",
              "--emit=metadata", "-o", str(result), str(source), f"-Ldependency={SEARCH}", f"-Ldependency={OUT}"]
    if WASM:
        inputs += ["--target=wasm32-wasip2"]
    for name, path in dependencies.items():
        assert path.is_file(), path
        inputs += ["--extern", f"{name}={path}"]
    response = OUT / f"{crate}.args"
    response.write_text("\n".join(inputs) + "\n", encoding="utf8")
    subprocess.run([*RUST, f"@{response}"], check=True)
    return result


STAGE = runpy.run_path(str(TICKET / "raw-store-inbox-stage.py"))
STAGE["stage"]()
ROOT = STAGE["ROOT"]
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]
for key in ("INBOX", "SENDER", "NATIVE"):
    path = STAGE[key]
    (OUT / path.name).write_text(POST[str(path)], encoding="utf8")
backbone = STAGE["NATIVE"].parent / "🦀️.rs"
source = (ROOT / backbone).read_text(encoding="utf8")
source = re.sub(r'(#\[path\s*=\s*)"([^"]+)"', lambda m: f'{m[1]}"{OUT / m[2] if m[2] == STAGE["NATIVE"].name else ROOT / backbone.parent / m[2]}"', source)
backbone_post = OUT / "backbone.rs"
backbone_post.write_text(source, encoding="utf8")
artifacts = exact_dependencies(unique("semio_os_store_backbone"))
backbone_api = compile("inbox_backbone", backbone_post, artifacts)
engine = unique("semio_os_store_engine")
engine_dependencies = exact_dependencies(engine)
kernel = OUT / "kernel.rs"
kernel.write_text("""
pub mod os_vcs { pub use real_vcs::*; }
pub mod os_spr { pub use installed_spr::*; }
pub mod os_store {
    pub use installed_engine::lifted_store::ArtifactStore;
    pub use installed_bundle::AtomicStoreReceive;
    pub mod backbone { pub use paid_backbone::*; }
}
""", encoding="utf8")
exports = compile("inbox_kernel", kernel, {
    "real_vcs": artifacts["semio_os_vcs"],
    "installed_spr": artifacts["semio_os_spr"],
    "installed_engine": engine,
    "installed_bundle": engine_dependencies["semio_os_store_engine_bundle"],
    "paid_backbone": backbone_api,
})
framework = engine_dependencies.get("semio_framework") or unique("semio_framework")
job = exact_dependencies(framework).get("semio_framework_job") or engine_dependencies["semio_framework_job"]
compile("inbox_actor_turn", TICKET / "raw-reactor-backbone-turn.rs", {
    "semio_os_kernel": exports,
    "semio_framework": framework,
    "semio_framework_value": artifacts["semio_framework_value"],
    "semio_framework_job": job,
})
print("[DEBUG] Type-checked exact native backbone input and actor Store fold against governed types")
