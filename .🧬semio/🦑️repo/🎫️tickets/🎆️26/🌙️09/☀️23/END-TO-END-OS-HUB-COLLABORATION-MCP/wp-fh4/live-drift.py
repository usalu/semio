"""🔭️ Which of my drift-site files changed on the LIVE tree since the p2 overlay's base (live at S20's 19:37 rebase):
sha1(live) vs the pass-1 baseline (unchanged-by-pass-1 files) or the rebase BASE blob (pass-1 files).
Usage: python3 live-drift.py [family-substring]"""
import hashlib, json, sys
from pathlib import Path
ROOT = Path("/Users/ueli/Documents/semio")
HUB = ROOT / ".🧬semio/🌐hub"
base1 = json.loads((HUB / "s14-s20-overlay-faults.baseline.json").read_text())
bases = HUB / "s14-s20-overlay-faults.bases"
sites = json.loads((Path(__file__).parent / "my-sites-0.json").read_text())
needle = sys.argv[1] if len(sys.argv) > 1 else ""
files = sorted({e["path"] for e in sites["X"] + sites["Y"] if needle in e["path"]})
sha = lambda data: hashlib.sha1(data).hexdigest()
drift = []
for rel in files:
    live = ROOT / rel
    now = sha(live.read_bytes()) if live.exists() else "GONE"
    was = sha((bases / rel).read_bytes()) if (bases / rel).exists() else base1.get(rel)
    if now != was:
        drift.append(rel)
print(f"files={len(files)} live-drift={len(drift)}")
for rel in drift:
    print("  ", rel)
