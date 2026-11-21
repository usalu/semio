"""🧪 Links guarded independent-vector laws through the governed causal runtime."""

import json
from pathlib import Path
import re
import runpy
import struct
import subprocess

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "raw-hub-atomic"
BASE = GENERATED / "raw-store-atomic/w"
STAGE = runpy.run_path(str(TICKET / "raw-hub-atomic-stage.py"))
STAGE["stage"]()
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]

OUT.mkdir(parents=True, exist_ok=True)
for i, path in enumerate(sorted((GENERATED / "raw-spr-transport").glob("*.part"))):
    (OUT / str(i)).write_bytes(struct.pack("<BBQ", 0, 8, 82) + path.read_bytes())
edit = b"edit-png-82"
(OUT / "descriptor").write_bytes(struct.pack("<4sHH", b"SAE\x01", 15, len(edit)) + edit)
prepared = (GENERATED / "raw-spr-transport" / "prepared").read_bytes()
(OUT / "checkpoint").write_bytes(struct.pack("<4sBI", bytes.fromhex("80004702"), 1, len(prepared)) + prepared)
causal = STAGE["CAUSAL"]
ROOT = STAGE["ROOT"]
for product, file in [(STAGE["HUB_CODEC"], "hub.rs"), (causal.with_name("🧩️atomic-group.rs"), "group.rs")]:
    content = POST.get(str(product)) or (ROOT / product).read_text(encoding="utf8")
    (OUT / file).write_text(content, encoding="utf8")
links = {"🏪️atomic-hub.rs": OUT / "hub.rs", "🧩️atomic-group.rs": OUT / "group.rs"}
def resolve(match):
    return f'#[path = "{links.get(match[1], ROOT / causal.parent / match[1])}"]'
(OUT / "binary.rs").write_text(re.sub(r'#\[path\s*=\s*"([^"]+)"\]', resolve, POST[str(causal)]), encoding="utf8")
(OUT / "causal.rs").write_text('pub use real_causal::*;\n#[path = "binary.rs"] pub mod binary;\n', encoding="utf8")
old = (BASE / "causal_source_overlay.args").read_text(encoding="utf8")
name = "hub_causal"
library = OUT / f"lib{name}.rlib"
response = OUT / "causal.args"
response.write_text(old.replace(str(BASE / "causal.rs"), str(OUT / "causal.rs"))
                    .replace(str(BASE / "libcausal_source_overlay.rlib"), str(library))
                    .replace("causal_source_overlay", name), encoding="utf8")
subprocess.run([*RUST, f"@{response}"], check=True)
law = OUT / "law.wasm"
response = OUT / "law.args"
baseline = (BASE / "atomic-transport-law.args").read_text(encoding="utf8").splitlines()
flags = [line for line in baseline if line.startswith("-Ldependency=")]
response.write_text("\n".join(["--target=wasm32-wasip2", "--edition=2021", str(TICKET / "raw-hub-atomic-law.rs"),
                               "-o", str(law), "--extern", f"{name}={library}", *flags]) + "\n", encoding="utf8")
subprocess.run([*RUST, f"@{response}"], check=True)
subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(law)], check=True)
