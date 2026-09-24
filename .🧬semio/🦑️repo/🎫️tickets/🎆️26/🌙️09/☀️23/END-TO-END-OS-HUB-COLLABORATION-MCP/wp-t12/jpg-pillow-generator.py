#!/usr/bin/env python3
"""🏭️ jpg jfif-1.01 baseline: T11 removed Pillow as this capability's JUDGE (it cannot see DHT/DAC) but seven committed
`third-party-generated` fixtures still name it as their GENERATOR, so the contract reports `fixture-generator-unregistered`.
Pillow did write those bytes, so it is registered again under a generator-only id and capability, the seven fixtures point
at it, and the dependency ledger row for Pillow lists it. Usage: jpg-pillow-generator.py [--write]"""
import json, sys
from pathlib import Path
root = Path("/Users/ueli/Documents/semio")
path = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🔮️oracles/🔣️.json"
ledger_path = root / "🔒️dependencies.json"
write = "--write" in sys.argv
OLD, NEW, CAPABILITY = "pillow-jpg-jfif-1-01-baseline-mutate-reader", "pillow-jpg-jfif-1-01-baseline-fixture-generator", "jpg-jfif-1-01-baseline-fixture-generate"
text = path.read_text(encoding="utf-8")
doc = json.loads(text)
entry = {
    "id": NEW,
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "Pillow",
    "version": "12.2.0",
    "engine": {"family": "pillow", "implementation": "Pillow 12.2.0 JpegImagePlugin encoder", "version": "12.2.0"},
    "capabilities": [CAPABILITY],
    "comparisonProfiles": ["exact-bytes-v1"],
    "license": "MIT-CMU",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "homepage": "https://python-pillow.org",
    "rationale": "🏭️ A GENERATOR, never a judge. Pillow 12.2.0's own JPEG encoder wrote the seven `third-party-generated` before/after pairs below (its `subsampling=`, `qtables=`, mode and `progressive=` save parameters), so it is their provenance and nothing else. It discharges no mutation capability: the marker-level judge of `jpg-jfif-1-01-baseline-mutate` is `libjpeg-jpg-jfif-1-01-baseline-marker-cli`, because Pillow's JPEG plugin skips the DHT and DAC markers and so can never read those kinds' axes.",
}
if not any(o["id"] == NEW for o in doc["oracles"]):
    doc["oracles"].append(entry)
moved = 0
for fixture in doc["fixtureManifests"]:
    generator = fixture.get("generator")
    if generator and generator.get("oracle") == OLD:
        generator["oracle"] = NEW
        moved += 1
ledger_text = ledger_path.read_text(encoding="utf-8")
ledger = json.loads(ledger_text)
row = next(e for e in ledger["entries"] if e["ecosystem"] == "python" and e["name"] == "Pillow")
if NEW not in row["oracleIds"]:
    row["oracleIds"] = sorted(row["oracleIds"] + [NEW])
    row["capabilities"] = sorted(row["capabilities"] + [CAPABILITY])
print(f"fixtures re-pointed: {moved}; oracle registered: {NEW}; ledger row {row['name']} {row['version']}")
if write:
    path.write_text(json.dumps(doc, ensure_ascii=False, indent=1 if text.startswith('{\n "') else 2) + "\n", encoding="utf-8")
    ledger_path.write_text(json.dumps(ledger, ensure_ascii=False, indent=1 if ledger_text.startswith('{\n "') else 2) + "\n", encoding="utf-8")
