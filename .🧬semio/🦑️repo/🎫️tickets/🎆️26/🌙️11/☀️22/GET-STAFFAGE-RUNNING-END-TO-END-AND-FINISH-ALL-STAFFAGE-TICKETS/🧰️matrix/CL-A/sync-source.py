"""🧵 Keeps the volume sources of the 3D ticket equal to the catalog after the closing repair: `python3 sync-source.py <asset id>…` writes `asset.volumes` of the catalog file into `…/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes/animals/<id>.json` in the indentation and final newline that file has (its chain `volumes-insert.py` and the `rework-*.py` scripts refuse or undo a catalog whose volumes differ from the source). Idempotent."""
import glob, json, sys

R = "/Users/ueli/Documents/semio"
T14 = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
for asset in sys.argv[1:]:
    catalog = json.load(open(glob.glob(f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{asset}/🔣️.json")[0], encoding="utf-8"))["asset"]["volumes"]
    path = f"{T14}/volumes/animals/{asset}.json"
    raw = open(path, encoding="utf-8").read()
    source = json.loads(raw)
    if source["volumes"] == catalog:
        print(asset, "source equal to the catalog: unchanged")
        continue
    source["volumes"] = catalog
    open(path, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False, indent=1) + ("\n" if raw.endswith("\n") else ""))
    print(asset, "source written")
