"""🧪️ Type-generate the combined actor WIT package and check the raw transport ABI."""

import json
from pathlib import Path
import subprocess
from runpy import run_path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUTPUT = TICKET / "🗑️generated"
SCHEMA = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema")

run_path(str(TICKET / "raw-reactor-stage.py"))["stage"]()
overlay = json.loads((OUTPUT / "raw-reactor-overlay.json").read_bytes())["candidate"]
wire = OUTPUT / "rm-wit"
declarations = OUTPUT / "rm-types"
wire.mkdir(exist_ok=True)
for source in sorted((ROOT / SCHEMA).glob("*.wit")):
    neutral = source.name.removeprefix("📜️").removeprefix("-")
    body = overlay.get(str(source.relative_to(ROOT)))
    (wire / (neutral if neutral != ".wit" else "actor.wit")).write_text(
        source.read_text(encoding="utf8") if body is None else body,
        encoding="utf8",
    )
subprocess.run(["repo", "tools", "exec", "jco", "--", "types", str(wire), "--world-name", "actor", "--strict", "-o", str(declarations)], check=True)
types = "\n".join(path.read_text(encoding="utf8") for path in declarations.rglob("*.d.ts"))
for witness in ("originalMediaAck", "original-media-control", "pageIndex", "baseRevision", "owner: bigint", "seq: bigint"):
    if witness not in types:
        raise ValueError(f"Unrecognized raw command ABI field: {witness}")
print("[DEBUG] Generated and checked the full actor WIT control and receipt declarations")
