"""🧾 Compares the manifest rows of the PW-10 … PW-17 plants with their shipped catalog assets (shape of the existing manifest entries, values of the asset) and prints every difference.

Usage: `python3 rowcheck.py [<id> ...]`. See `../../📋️brief.md`.
"""
import json, pathlib, sys

TICKET = pathlib.Path(__file__).resolve().parents[2]
STAFFAGE = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage"
VEG = STAFFAGE / "🗂️catalog/🌳️vegetation"
KEYS = ["id", "category", "title", "heightM", "widthM", "powerW", "poses", "clips", "representations", "symbols", "analytics", "exports", "distinction", "completion"]
IDS = "windmill-palm banana-plant tree-fern saguaro-cactus prickly-pear yucca common-lilac hydrangea rhododendron boxwood-ball privet-hedge cherry-laurel lavender-bush rosemary-bush shrub-rose pampas-grass common-reed sunflower wisteria-pergola grapevine-row maize-row potted-monstera fiddle-leaf-fig snake-plant".split()

for slug in sys.argv[1:] or IDS:
    asset_file, row_file = VEG / slug / "🔣️.json", TICKET / "📥️manifest-rows" / f"{slug}.json"
    if not asset_file.exists() or not row_file.exists():
        print(f"{slug}: asset {'yes' if asset_file.exists() else 'NO'} row {'yes' if row_file.exists() else 'NO'}")
        continue
    a, row = json.loads(asset_file.read_text("utf8"))["asset"], json.loads(row_file.read_text("utf8"))
    want = {
        "id": a["id"], "category": a["category"], "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"],
        "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]],
        "representations": {k: a["representations"][k]["status"] for k in ["50", "100", "200", "500"]},
        "symbols": [s["id"] for s in a.get("symbols", [])], "exports": a["exports"], "completion": a["completion"],
    }
    diffs = [f"keys {list(row)}"] if list(row) != KEYS else []
    diffs += [f"{k}: row {json.dumps(row.get(k), ensure_ascii=False)[:160]} asset {json.dumps(v, ensure_ascii=False)[:160]}" for k, v in want.items() if row.get(k) != v]
    if row.get("powerW") != 0: diffs.append(f"powerW {row.get('powerW')}")
    if set(row.get("distinction", {})) != {"en", "de"}: diffs.append("distinction languages")
    print(f"{slug}: {'ok' if not diffs else ' | '.join(diffs)}  [{a['dimensions']}] analytics {row.get('analytics')}")
