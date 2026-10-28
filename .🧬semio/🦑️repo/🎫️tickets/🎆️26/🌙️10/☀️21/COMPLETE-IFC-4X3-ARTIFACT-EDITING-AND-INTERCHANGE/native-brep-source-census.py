"""🧊️ Pin native Brep alternatives and their unchanged canonical Body owners."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
c = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts"
b = c / "🧱️body"

def row(path):
    """🎯️ Observe original source bytes and cross-platform path limits without copies."""
    text = str(path)
    return {"path": text, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "utf16PathUnits": len(text.encode("utf-16-le")) // 2}

owned = [b / name for name in ["🦀️.rs", "domain.rs", "clause.rs", "work.rs", "helpers.rs", "native-tests.rs", "clause-tests.rs", "advanced-tests.rs", "surface-tests.rs", "clipping-tests.rs", "brep-tests.rs", "tessellation-tests.rs", "family-tests.rs"]]
refs = [c / "types.rs", c / "native.rs", c / "preparation.rs", c / "population.rs", c.parent / "🦀️.rs"] + [c / folder / name for folder in ["🧊️brep", "💎️advanced-brep"] for name in ["definition.json", "cases.json", "provenance.json", "🟦️.ts"]]
result = {"scope": "native source only; authentic type/runtime/receipt/output proof remains false", "owned": list(map(row, owned)), "references": list(map(row, refs))}
assert all(item["utf16PathUnits"] <= 256 for item in result["owned"] + result["references"])
(ticket / "🗑️generated/native-brep-source.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
