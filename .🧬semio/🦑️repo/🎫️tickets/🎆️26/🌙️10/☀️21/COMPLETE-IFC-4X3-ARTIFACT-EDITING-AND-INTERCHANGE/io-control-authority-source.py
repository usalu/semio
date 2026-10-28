"""🔐️ Pin original IO control capture and actual public consumer evidence."""
from pathlib import Path
import hashlib
import json
ticket = Path(__file__).parent
base = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
io = base / "🚪️io"
def row(path):
    """🧷️ Observe source bytes and platform path limits without source copies."""
    text = str(path)
    return {"path":text,"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(text.encode("utf-16-le"))//2}
owned = [io / "control.ts"] + [io / "🧪️tests" / name for name in ["control-authority.test.ts","control-authority-cases.json","control-authority.schema.json"]]
refs = [io / name for name in ["step.ts","step-writer.ts","gltf.ts","gltf-source.ts","gltf-json.ts","gltf-json-reader.ts","geometry/control.ts"]] + [base / "🧬️schema/👁️mvd/🧩️concepts/🧱️body/🟦️.ts", ticket / "io-control-authority-tsconfig.json"]
result = {"owned":list(map(row,owned)),"references":list(map(row,refs)),"scope":"actual TypeScript IO capture; no native output proof"}
assert all(item["utf16PathUnits"]<=256 for item in result["owned"]+result["references"])
(ticket / "🗑️generated/io-control-authority-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf-8")
