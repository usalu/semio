"""🗺️ Pin the canonical admitted source ordinal index without copying source."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
io = base / "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"

def row(path):
    """📌️ Observe actual bytes and cross-platform path lengths without source copies."""
    text = str(path)
    return {"path": text, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "utf16PathUnits": len(text.encode("utf-16-le")) // 2}

owned = [io / "geometry.ts"] + [io / "🧪️tests" / name for name in ["geometry-index.test.ts", "geometry-index-cases.json", "geometry-index.schema.json"]]
refs = [base / "📚️catalog/🔎️query/🟦️.ts", io / "control.ts", io / "exchange-source.ts", ticket / "geometry-source-index-tsconfig.json"]
result = {"owned": list(map(row, owned)), "references": list(map(row, refs)), "scope": "canonical JS capture index; BIM/FEM are concurrent worker-owned consumers; no native execution proof"}
assert all(item["utf16PathUnits"] <= 256 for item in result["owned"] + result["references"])
(ticket / "🗑️generated/geometry-source-index.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
