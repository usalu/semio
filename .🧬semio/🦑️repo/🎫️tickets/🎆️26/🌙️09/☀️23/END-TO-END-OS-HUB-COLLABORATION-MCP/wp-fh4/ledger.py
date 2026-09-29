"""📒️ Ledger re-run for FH4's families on the p2 overlay: S20's own `p2-codemod.convert` over every Rust file of the named
plugins, nothing written; prints the drift sites left (and writes them to `ledger-<tag>.json` here when a tag is given).
Usage: python3 ledger.py [tag] [plugin-dir…]   (default plugin dirs: P2-X + P2-Y)"""
import importlib.util
import json
import os
import sys
from pathlib import Path

HERE = Path(__file__).parent
OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
FAMILIES = ["🀄️wfc", "📕️norm", "📸️remodel", "🖍️draw", "📐️cad", "🖨️raster", "🏗️fem", "📋️forms", "🪐️space", "🌍️gis", "📏️layout", "🗄️stdio"]
spec = importlib.util.spec_from_file_location("p2codemod", "/Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-p2/p2-codemod.py")
codemod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(codemod)
args = sys.argv[1:]
tag = args.pop(0) if args and not args[0].startswith(("🀄", "📕", "📸", "🖍", "📐", "🖨", "🏗", "📋", "🪐", "🌍", "📏", "🗄")) else None
families = args or FAMILIES
ledger: list[dict] = []
for family in families:
    for current, dirs, names in os.walk(OVERLAY / "✏️s/🔌️plugins" / family):
        dirs[:] = [d for d in dirs if d not in codemod.SKIP_DIRS and d != "dist"]
        for name in names:
            if name != "🦀️.rs":
                continue
            path = Path(current) / name
            text = path.read_text()
            if "Mutation" in text:
                codemod.convert(str(path.relative_to(OVERLAY)), text, ledger)
counts: dict[str, int] = {}
for entry in ledger:
    family = entry["path"].split("/")[2]
    counts[family] = counts.get(family, 0) + 1
print(f"drift sites left: {len(ledger)} {counts}")
for entry in ledger:
    print(f"  {entry['path'].split('🔌️plugins/')[1][:150]}:{entry['line']} {entry['kind']} {entry.get('method', '')} {entry.get('code') or entry.get('why')}")
if tag:
    (HERE / f"ledger-{tag}.json").write_text(json.dumps(ledger, ensure_ascii=False, indent=1))
