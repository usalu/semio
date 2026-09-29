"""🔎️ Prints my drift sites (`ledger-1.json`) with context from the p2 overlay: python3 show.py <path-substring> [before] [after]"""
import json, sys
from pathlib import Path
OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
rows = json.load(open(Path(__file__).parent / "ledger-1.json"))
needle = sys.argv[1]
before = int(sys.argv[2]) if len(sys.argv) > 2 else 2
after = int(sys.argv[3]) if len(sys.argv) > 3 else 2
last = None
for e in rows:
    if needle not in e["path"]:
        continue
    lines = (OVERLAY / e["path"]).read_text().split("\n")
    if e["path"] != last:
        print("=====", e["path"].split("🔌️plugins/")[-1])
        last = e["path"]
    lo = max(0, e["line"] - 1 - before)
    hi = min(len(lines), e["line"] + after)
    print(f"--- L{e['line']} {e.get('kind')} {e.get('method')} {e.get('code') or e.get('why')}")
    for i in range(lo, hi):
        print(f"{i+1:5d}| {lines[i][:260]}")
