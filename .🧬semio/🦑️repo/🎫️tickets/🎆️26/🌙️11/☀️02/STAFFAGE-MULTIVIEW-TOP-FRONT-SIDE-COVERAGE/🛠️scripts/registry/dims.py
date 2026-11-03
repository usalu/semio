"""📏 Read-only check of manifest heightM and widthM against the asset file dimensions (heightM, widthM, depthM); mismatches above 1 mm are listed."""

import json
import sys
from pathlib import Path

catalog_dir, output = Path(sys.argv[1]), Path(sys.argv[2])
TOLERANCE_M = 0.001

manifest = json.loads((catalog_dir / "🔣️.json").read_text(encoding="utf-8"))
folder_of = {c["id"]: c["folder"] for c in manifest["categories"]}
rows = manifest["entries"]
mismatches = []
missing = []
checked = 0


def asset_file(row: dict) -> Path | None:
    folder = catalog_dir / folder_of[row["category"]] / row["id"]
    if not folder.is_dir():
        return None
    files = [p for p in folder.iterdir() if p.suffix == ".json"]
    return files[0] if len(files) == 1 else None


for row in rows:
    path = asset_file(row)
    if path is None:
        missing.append(row["id"])
        continue
    dims = json.loads(path.read_text(encoding="utf-8"))["asset"]["dimensions"]
    checked += 1
    for manifest_key, asset_key in (("heightM", "heightM"), ("widthM", "widthM")):
        m, a = row[manifest_key], dims[asset_key]
        if abs(m - a) > TOLERANCE_M:
            mismatches.append({"id": row["id"], "field": manifest_key, "manifest": m, "asset": a, "diffMm": round((m - a) * 1000, 3)})

summary = {"rows": len(rows), "checked": checked, "missingAssetFiles": missing, "mismatches": len(mismatches), "toleranceMm": TOLERANCE_M * 1000}
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps({"generatedBy": "haiku-registry", "note": "widthM is the x extent (elevation horizontal); depthM is the lateral z extent and is not in the manifest", "summary": summary, "mismatches": mismatches}, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(json.dumps(summary, ensure_ascii=False))
for item in mismatches:
    print(f"{item['id']} {item['field']}: manifest {item['manifest']} vs asset {item['asset']} (diff {item['diffMm']} mm)")
