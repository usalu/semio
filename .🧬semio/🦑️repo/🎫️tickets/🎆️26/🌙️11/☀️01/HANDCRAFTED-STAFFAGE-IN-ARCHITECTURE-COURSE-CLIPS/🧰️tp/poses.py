import glob, json, sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
S = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
slug = sys.argv[1]
asset = json.load(open(glob.glob(str(S / "*" / slug / "🔣️.json"))[0], encoding="utf8"))["asset"]
want = sys.argv[2:]
for p in asset.get("poses", []):
    if not want or p["id"] in want:
        print("POSE", p["id"], json.dumps(p.get("thermal"), ensure_ascii=False))
for c in asset.get("clips", []):
    if not want or c["id"] in want:
        print("CLIP", c["id"], c["durationS"], json.dumps(c.get("keys"), ensure_ascii=False)[:1500])
