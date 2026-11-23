"""🔭️ Static view coverage of the staffage catalog: per asset every representation, symbol and alternative with its three views, viewpoint lists, unavailable causes, volumes, poses and clips; prints a JSON summary."""
import collections
import json
import pathlib
import sys

STAFFAGE = pathlib.Path(__file__).resolve().parents[8] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage"
VIEWS = ("elevation", "front", "plan")


def measure(root: pathlib.Path) -> dict:
    """🧮️ Count every static coverage fact of the catalog below `root`."""
    manifest = json.loads((root / "🗂️catalog" / "🔣️.json").read_text(encoding="utf-8"))
    folders = {category["id"]: category["folder"] for category in manifest["categories"]}
    summary = collections.Counter()
    missing: dict[str, list[str]] = collections.defaultdict(list)
    categories = collections.Counter()
    files = {path.parent.name for path in (root / "🗂️catalog").glob("*/*/🔣️.json")}
    entries = {entry["id"] for entry in manifest["entries"]}
    for entry in manifest["entries"]:
        asset = json.loads((root / "🗂️catalog" / folders[entry["category"]] / entry["id"] / "🔣️.json").read_text(encoding="utf-8"))["asset"]
        categories[entry["category"]] += 1
        summary["assets"] += 1
        summary["poses"] += len(asset["poses"])
        summary["clips"] += len(asset["clips"])
        if "viewpoints" in asset["presentation"]:
            missing["presentation.viewpoints"].append(entry["id"])
        if asset["completion"] != "authored":
            missing["completion"].append(entry["id"])
        if not asset.get("volumes"):
            missing["volumes"].append(entry["id"])
        if not asset.get("thermal"):
            missing["thermal"].append(entry["id"])
        if not asset.get("structure"):
            missing["structure"].append(entry["id"])
        for key, representation in asset["representations"].items():
            summary[f"representation.{representation['status']}"] += 1
            if representation["status"] == "authored":
                for view in VIEWS:
                    if not representation["geometry"].get(view):
                        missing[f"authored.{view}"].append(f"{entry['id']}@{key}")
            else:
                if representation.get("cause") not in ("below-printable-feature", "no-physical-depiction"):
                    missing["unavailable.cause"].append(f"{entry['id']}@{key}")
                for index, alternative in enumerate(representation.get("alternatives", [])):
                    summary[f"alternative.{alternative.get('kind')}"] += 1
                    geometry = alternative.get("geometry")
                    if geometry is not None:
                        for view in VIEWS:
                            if not geometry.get(view):
                                missing[f"alternative.{view}"].append(f"{entry['id']}@{key}#{index}")
        for symbol in asset.get("symbols", []):
            summary["symbols"] += 1
            for view in VIEWS:
                if not symbol["geometry"].get(view):
                    missing[f"symbol.{view}"].append(f"{entry['id']}:{symbol['id']}")
    return {"categories": dict(categories), "summary": dict(summary), "foldersWithoutEntry": sorted(files - entries), "entriesWithoutFolder": sorted(entries - files), "missing": {key: value for key, value in missing.items()}}


if __name__ == "__main__":
    json.dump(measure(pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else STAFFAGE), sys.stdout, ensure_ascii=False, indent=2)
