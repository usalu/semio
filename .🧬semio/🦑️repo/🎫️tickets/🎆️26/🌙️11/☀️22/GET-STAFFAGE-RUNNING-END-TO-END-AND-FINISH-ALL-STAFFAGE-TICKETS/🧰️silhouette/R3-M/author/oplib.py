"""🧰️ Shared helpers of the authoring scripts of the repair ops of batch R3-M: read a catalog asset, find shapes and solids, write `ops/<id>.json` one op per line."""
import glob, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def load(ident):
    """📥️ The asset of an id: the pre-repair copy `$SAM_SOURCE/<id>.json` when there is one, else the catalog file."""
    source = os.path.join(os.environ.get("SAM_SOURCE", ""), f"{ident}.json")
    return json.load(open(source if os.path.exists(source) else glob.glob(f"{CATALOG}/*/{ident}/🔣️.json")[0]))["asset"]


def shape(asset, scale, view, ident):
    """🔎️ One drawn shape."""
    return next(s for s in asset["representations"][str(scale)]["geometry"][view] if s.get("id") == ident)


def solid(asset, scale, ident, kind="body"):
    """🔎️ One volume solid."""
    return next(s for s in asset["volumes"][str(scale)][kind] if s.get("id") == ident)


def write(ident, ops):
    """📤️ Writes the ops of an asset."""
    with open(os.path.join(HERE, "..", "ops", f"{ident}.json"), "w") as f:
        f.write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
    print(f"{ident}: {len(ops)} ops")
