"""🧪 cs: builds the cast manifest for the style contact sheets (changed assets and archive references)."""
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[7]
S = ROOT / "🎓️teaching" / "🏛️architecture" / "🧍️staffage"
CAT = S / "🗂️catalog"
OUT = HERE.parent / "🗑️generated" / "cs"
REFS = ["cyclist", "seated-reader", "park-bench", "bookshelf", "heat-pump-outdoor-unit", "single-bed", "three-seat-sofa", "stepladder"]
ENV = {"humans": "office-neutral", "appliances": "office-neutral", "furniture": "office-neutral", "objects": "office-neutral"}
VIEWS = [("dark-line", "dark"), ("light-line", "light"), ("thermal", "dark")]


def changed():
    out = subprocess.run(["git", "-c", "core.quotepath=false", "status", "--porcelain", "-u", "--", str(CAT.relative_to(ROOT))], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    ids = []
    for line in out.splitlines():
        p = line[3:].strip('"')
        parts = p.split("/")
        if parts[-1].endswith(".json") and len(parts) >= 3 and parts[-2] != "🗂️catalog":
            if next(CAT.glob(f"*/{parts[-2]}/🔣️.json"), None) is not None:
                ids.append(parts[-2])
    return sorted(set(ids))


def find(asset_id):
    for path in CAT.glob(f"*/{asset_id}/🔣️.json"):
        return path
    raise SystemExit(f"missing {asset_id}")


def main():
    ids = changed()
    groups = {"new": ids, "ref": REFS}
    casts = {}
    meta = {}
    for kind, items in groups.items():
        for a in items:
            doc = json.loads(find(a).read_text(encoding="utf-8"))["asset"]
            pose = doc["poses"][0]["id"]
            env = ENV.get(doc["category"], "winter-overcast")
            meta[a] = {"kind": kind, "category": doc["category"], "pose": pose}
            for view, theme in VIEWS:
                for scale in (50, 100):
                    cid = f"{a}-{view}-{scale}"[:64]
                    c = {"asset": a, "pose": pose, "view": view, "scale": scale, "theme": theme}
                    if view == "thermal":
                        c["environment"] = env
                    casts[cid] = c
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "manifest.json").write_text(json.dumps({"version": 1, "casts": casts}, ensure_ascii=False, indent=1), encoding="utf-8")
    (OUT / "meta.json").write_text(json.dumps(meta, ensure_ascii=False, indent=1), encoding="utf-8")
    print(len(ids), "changed", ids, len(casts), "casts")


main()
