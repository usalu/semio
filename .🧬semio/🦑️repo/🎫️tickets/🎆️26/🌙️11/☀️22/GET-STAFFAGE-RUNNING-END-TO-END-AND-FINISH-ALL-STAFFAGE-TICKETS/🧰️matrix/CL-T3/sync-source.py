"""🧵 Keeps the volume source of the 3D ticket equal to the catalog after the repair of lane CL-T3: `python3 sync-source.py equal <asset id>` exits 0 when `…/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes/<category>/<id>.json` holds `asset.volumes` of the catalog file (its chain `volumes-insert.py` and the `rework-*.py` scripts refuse or undo a catalog whose volumes differ from the source), `python3 sync-source.py write <asset id>` writes the catalog volumes into it in the indentation and final newline that file has. A source that differed before the repair is another lane's state and is left alone by `prove.sh`. Idempotent."""
import glob, json, sys

R = "/Users/ueli/Documents/semio"
T14 = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
mode, asset = sys.argv[1:3]
catalog = json.load(open(glob.glob(f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{asset}/🔣️.json")[0], encoding="utf-8"))["asset"]["volumes"]
paths = glob.glob(f"{T14}/volumes/*/{asset}.json")
if not paths:
    print(asset, "has no volume source")
    sys.exit(2)
raw = open(paths[0], encoding="utf-8").read()
source = json.loads(raw)
if source["volumes"] == catalog:
    print(asset, "source equal to the catalog")
    sys.exit(0)
if mode == "equal":
    print(asset, "source differs from the catalog")
    sys.exit(1)
source["volumes"] = catalog
open(paths[0], "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False, indent=1) + ("\n" if raw.endswith("\n") else ""))
print(asset, "source written")
