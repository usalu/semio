"""🧾 Brings `widthM` and `heightM` of every manifest entry to the dimensions of its asset file (the single source of truth) under the publish lock and removes a `depthM` key, which the manifest does not carry; idempotent.

Usage: `python3 sync-manifest.py [--write]`. See `../📋️silhouette-brief.md`.
"""
import json, os, pathlib, sys, time

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[7]
CATALOG = REPO / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
MANIFEST = CATALOG / "🔣️.json"
LOCK = HERE.parents[1] / "RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock"


def sync(write: bool) -> None:
    """✍️ Prints and applies every difference between an entry and its asset file."""
    manifest = json.loads(MANIFEST.read_text("utf8"))
    folders = {c["id"]: c["folder"] for c in manifest["categories"]}
    count = 0
    for entry in manifest["entries"]:
        dimensions = json.loads((CATALOG / folders[entry["category"]] / entry["id"] / "🔣️.json").read_text("utf8"))["asset"]["dimensions"]
        if "depthM" in entry:
            del entry["depthM"]
            count += 1
        for key in ("widthM", "heightM"):
            if abs(entry[key] - dimensions[key]) > 1e-9:
                print(f"{entry['id']} {key}: {entry[key]} -> {dimensions[key]}")
                entry[key] = dimensions[key]
                count += 1
    print(f"changes {count}")
    if write and count:
        MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", "utf8")


def main(write: bool) -> None:
    """🔒️ Holds the publish lock while writing."""
    if not write:
        return sync(False)
    for _ in range(600):
        try:
            os.mkdir(LOCK)
            break
        except FileExistsError:
            time.sleep(0.2)
    else:
        raise SystemExit(f"publish.lock is held: {LOCK}")
    try:
        sync(True)
    finally:
        os.rmdir(LOCK)


if __name__ == "__main__":
    main("--write" in sys.argv)
