"""🧪 Links the staged graph resource over the actual governed PDF COS and Pack owners."""

import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/raw-png-graph"
FIXTURE = OUT.parent / "large-picked/large-picked.png"

consumers = "--consumers" in sys.argv
if consumers:
    consumer = runpy.run_path(str(TICKET / "raw-png-graph-consumers-stage.py"))
    consumer["stage"]()
    overlay = consumer["OUT"]
else:
    graph = runpy.run_path(str(TICKET / "raw-png-graph-stage.py"))
    graph["stage"]()
    overlay = graph["PROBE"]
OUT.mkdir(parents=True, exist_ok=True)
source = FIXTURE.read_bytes()
assert len(source) == 473_683 and source[:8] == bytes.fromhex("89504e470d0a1a0a")
(OUT / "identity").write_bytes(hashlib.sha256(source).digest())
if consumers:
    sys.path.insert(0, str(OUT.parent / "paged-media-receiver/python"))
    from PIL import Image
    with Image.open(FIXTURE) as png:
        assert png.size == (521, 257)
        rgba = png.convert("RGBA")
        for mode, plane in (("rgb", rgba.convert("RGB")), ("alpha", rgba.getchannel("A"))):
            (OUT / f"{mode}-sha").write_bytes(hashlib.sha256(plane.tobytes()).digest())
postimages = json.loads(overlay.read_text(encoding="utf8"))["candidate"]
for path, content in postimages.items():
    if "📦️foreign-artifacts" in path:
        (OUT / "native.rs").write_text(content, encoding="utf8")

crate = OUT / "harness.rs"
crate.write_text(f"""
pub mod standards {{
    pub mod v1_7 {{ pub mod subsets {{ pub mod base {{
        pub mod schema {{
            pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
        }}
        pub mod io {{
            pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::error;
        }}
    }}}}}}
}}
mod native;
#[path = "{TICKET / 'raw-png-graph-law.rs'}"]
mod laws;
""", encoding="utf8")

cache = Path(sys.argv[1]).resolve()
wasm = "--wasm" in sys.argv
wasi = "--wasi" in sys.argv or wasm
target = OUT / ("probe.wasm" if wasm else "probe.rmeta" if wasi else "probe")
args = ["--test", "--edition=2021", str(crate), "-o", str(target)]
if consumers:
    args += ["--cfg=graph_consumer"]
if wasi:
    args += ["--target=wasm32-wasip2"]
    if not wasm:
        args += ["--emit=metadata"]
    dependency = OUT / "deps"
    dependency.mkdir(exist_ok=True)
    host = cache.parents[2] / "prototype/build"
    for directory, suffixes in ((cache, (".rmeta",)), (host, (".so", ".dylib", ".dll"))):
        for path in directory.glob("*/*/out/*"):
            link = dependency / path.name
            if path.suffix in suffixes and not link.exists():
                link.symlink_to(path)
    args += [f"-Ldependency={dependency}"]
for spec in sys.argv[2:]:
    if "=" not in spec:
        continue
    name, key = spec.split("=", 1)
    [meta] = cache.glob(f"*/*/out/lib{name}-{key}.rmeta")
    args += ["--extern", f"{name}={meta}"]
    if not wasi or wasm:
        args += ["--extern", f"{name}={meta.with_suffix('.rlib')}"]
if not wasi or wasm:
    args += [f"-Ldependency={path}" for path in sorted({
        p.parent for root in ({cache, host} if wasm else {cache}) for p in root.glob("*/*/out/*")
        if p.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")
    })]
response = OUT / "rustc.args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
if wasm:
    subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(target)], check=True)
elif wasi:
    print("[DEBUG] Type-checked staged native resources and exact canonical COS reopen law on WASI")
else:
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(target), "--nocapture"], check=True)
