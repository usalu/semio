"""🌡️ tp: print the thermal block and clip/pose timing of catalog assets: thermal.py <slug> ..."""
import glob
import json
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
S = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for slug in sys.argv[1:]:
    hits = glob.glob(str(S / "*" / slug / "🔣️.json"))
    asset = json.load(open(hits[0], encoding="utf8"))["asset"]
    th = asset.get("thermal", {})
    print("=====", slug, asset["dimensions"])
    for key in ("environment", "operating", "passengers", "emitters", "assumptions"):
        print(key, json.dumps(th.get(key), ensure_ascii=False))
    for r in th.get("regions", []):
        print("region", json.dumps({k: v for k, v in r.items() if k not in ("polygon", "outline", "shape")}, ensure_ascii=False)[:700])
    for c in asset.get("clips", []):
        print("clip", json.dumps({k: v for k, v in c.items() if k not in ("tracks", "keyframes")}, ensure_ascii=False)[:500])
    for p in asset.get("poses", []):
        print("pose", json.dumps({k: v for k, v in p.items() if k not in ("bones", "transforms")}, ensure_ascii=False)[:300])
