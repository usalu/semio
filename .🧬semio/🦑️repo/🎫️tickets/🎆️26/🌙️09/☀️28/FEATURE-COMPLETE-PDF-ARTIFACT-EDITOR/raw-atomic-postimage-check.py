"""🧾 Verifies guarded source sets and parses each exact Rust production postimage."""

import hashlib
import json
from pathlib import Path
import runpy
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/raw-atomic-postimages"
OUT.mkdir(parents=True, exist_ok=True)
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
for script in ("raw-hub-socket-stage.py", "raw-reactor-inbound-stage.py"):
    stage = runpy.run_path(str(TICKET / script))
    stage["stage"]()
    overlay = json.loads(stage["OUT"].read_text(encoding="utf8"))
    for path, sha in overlay["preimages"].items():
        source = ROOT / path
        actual = hashlib.sha256(source.read_bytes()).hexdigest() if source.exists() else None
        assert actual == sha, path
        assert len(str(source)) <= 256, path
    modules = []
    for path, source in overlay["candidate"].items():
        if not path.endswith(".rs"):
            continue
        modules.append(f"mod source_{len(modules)} {{\n{source}\n}}")
    snapshot = OUT / (Path(script).stem + ".rs")
    snapshot.write_text("\n".join(modules) + "\n", encoding="utf8")
    subprocess.run([*RUST, "--edition=2021", "-Zparse-crate-root-only=yes", str(snapshot)], check=True)
    print(f"[DEBUG] Parsed {len(modules)} exact Rust files from {script}; all guarded source hashes and path bounds match")
