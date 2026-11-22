"""🧾 Finisher A row proof: compares every given manifest row file with its catalog asset by the rules of the product's `validateManifest` (poses without key poses, clips, symbols, exports, completion, per-scale status, height and width within 1 mm, power, analytics) and with the key set and order of the existing manifest entries.

Usage: `python3 rows.py <id>...` (prints one line per id; exit 1 on any mismatch).
See `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts` (`validateManifest`).
"""
import json, pathlib, sys

TICKET = pathlib.Path(__file__).resolve().parent.parent.parent
CATALOG = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
KEYS = ["id", "category", "title", "heightM", "widthM", "powerW", "poses", "clips", "representations", "symbols", "analytics", "exports", "distinction", "completion"]


def power(asset: dict) -> float:
    regions = sum(r.get("powerW", 0) or 0 for r in asset["thermal"]["regions"])
    return regions + sum(o.get("powerW", 0) or 0 for o in asset["thermal"].get("operating", []))


def problems(ident: str) -> list[str]:
    row_path = TICKET / "📥️manifest-rows" / f"{ident}.json"
    asset_path = CATALOG / "🌳️vegetation" / ident / "🔣️.json"
    if not asset_path.is_file():
        return ["no asset"]
    if not row_path.is_file():
        return ["no row"]
    row = json.loads(row_path.read_text("utf8"))
    asset = json.loads(asset_path.read_text("utf8"))["asset"]
    out = []
    if list(row.keys()) != KEYS:
        out.append(f"keys {list(row.keys())}")
    if row.get("id") != asset["id"] or asset["id"] != ident:
        out.append("id")
    if row.get("category") != asset["category"]:
        out.append("category")
    if row.get("title") != asset["title"]:
        out.append(f"title {row.get('title')} != {asset['title']}")
    for field in ("heightM", "widthM"):
        if not abs(row.get(field, -1) - asset["dimensions"][field]) <= 0.001:
            out.append(f"{field} {row.get(field)} != {asset['dimensions'][field]}")
    if abs(row.get("powerW", -1) - power(asset)) > 0.5:
        out.append(f"powerW {row.get('powerW')} != {power(asset)}")
    if sorted(row.get("poses", [])) != sorted(p["id"] for p in asset["poses"] if not p.get("key")):
        out.append("poses")
    if row.get("poses") != [p["id"] for p in asset["poses"] if not p.get("key")]:
        out.append("pose order")
    if row.get("clips") != [c["id"] for c in asset["clips"]]:
        out.append("clips")
    if row.get("symbols") != [s["id"] for s in asset["symbols"]]:
        out.append("symbols")
    if row.get("exports") != asset["exports"]:
        out.append("exports")
    if row.get("completion") != asset["completion"]:
        out.append("completion")
    if row.get("representations") != {k: asset["representations"][k]["status"] for k in ("50", "100", "200", "500")}:
        out.append("representations")
    analytics = (["thermal"] if asset["thermal"]["regions"] else []) + (["transport"] if asset["thermal"]["emitters"] else []) + ["structural"]
    if row.get("analytics") != analytics:
        out.append(f"analytics {row.get('analytics')}")
    for lang in ("en", "de"):
        if not str(row.get("distinction", {}).get(lang, "")).strip():
            out.append(f"distinction.{lang}")
    return out


def main() -> None:
    bad = False
    for ident in sys.argv[1:]:
        found = problems(ident)
        bad = bad or bool(found)
        print(f"{ident}: {'row matches asset' if not found else '; '.join(found)}")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
