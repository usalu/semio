"""🧨️ Pin exact late observer failure boundaries and neutral regression inputs."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
io = base / "🚪️io"

def row(path):
    """🧷️ Observe exact original bytes and UTF-16 path limits without source copies."""
    text = str(path)
    return {"path": text, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "utf16PathUnits": len(text.encode("utf-16-le")) // 2}

owned = [io / name for name in ["control.ts", "step.ts", "step-writer.ts"]] + [io / "🧪️tests" / name for name in ["control-failure.test.ts", "control-failure-cases.json", "control-failure.schema.json"]]
refs = [io / name for name in ["gltf.ts", "gltf-source.ts", "gltf-json.ts", "gltf-json-reader.ts"]] + [io / "🧪️tests" / name for name in ["control-authority.test.ts", "control.test.ts", "gltf-framing.test.ts"]] + [base / "🧬️schema/🔺️diff/🟦️.ts", ticket / "late-original-failure-tsconfig.json"]
result = {"owned": list(map(row, owned)), "references": list(map(row, refs)), "scope": "TypeScript late observer identity; native and full MVD readiness unproved"}
assert all(item["utf16PathUnits"] <= 256 for item in result["owned"] + result["references"])
(ticket / "🗑️generated/late-original-failure-source.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
