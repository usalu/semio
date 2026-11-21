"""📥️ Links the exact continued Store inbox against its governed backbone and independent packet law."""

import json
from pathlib import Path
import runpy
import subprocess

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
BASE = GENERATED / "raw-store-atomic/w"
OUT = GENERATED / "raw-store-inbox"
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
STAGE = runpy.run_path(str(TICKET / "raw-store-inbox-stage.py"))
STAGE["stage"]()
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]

OUT.mkdir(parents=True, exist_ok=True)
(OUT / "inbox.rs").write_text(POST[str(STAGE["INBOX"])], encoding="utf8")
(OUT / "actor-send.rs").write_text(POST[str(STAGE["SENDER"])], encoding="utf8")
(OUT / "transport.rs").write_text(POST[str(STAGE["NATIVE"])]
    .replace('path = "📥️atomic-inbox.rs"', 'path = "inbox.rs"')
    .replace('path = "📤️atomic-actor-send.rs"', 'path = "actor-send.rs"'), encoding="utf8")
(OUT / "backbone.rs").write_text((BASE / "store-backbone.rs").read_text(encoding="utf8")
                              .replace(str(BASE / "atomic-transport.rs"), str(OUT / "transport.rs")), encoding="utf8")
library = "libstore_backbone_overlay.rlib"
response = OUT / "backbone.args"
response.write_text((BASE / "store_backbone_overlay.args").read_text(encoding="utf8")
                    .replace(str(BASE / "store-backbone.rs"), str(OUT / "backbone.rs"))
                    .replace(str(BASE / library), str(OUT / library)), encoding="utf8")
subprocess.run([*RUST, f"@{response}"], check=True)
law = OUT / "law.wasm"
response = OUT / "law.args"
input_args = (BASE / "atomic-transport-law.args").read_text(encoding="utf8")
response.write_text(input_args.replace(str(BASE / "atomic-transport-law.wasm"), str(law)).replace(str(BASE / library), str(OUT / library))
                    .replace("--cfg\ninbox\n", "") + "--cfg\ninbox\n", encoding="utf8")
subprocess.run([*RUST, f"@{response}"], check=True)
subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(law)], check=True)
