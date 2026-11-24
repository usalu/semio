"""🧰️ Helpers of the ops writers of batch MB-2 (`make/<id>.py`): read the catalog document of an asset and write `ops/<id>.json`, one op per line. A writer reads only what its own ops do not change, so writing and applying again gives the same file."""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def asset(slug):
    """📖️ The asset of the catalog document of an id."""
    return json.load(open(glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")[0]))["asset"]


def shape(a, scale, view, sid):
    """🔎️ A copy of a drawn shape by id."""
    return json.loads(json.dumps(next(s for s in a["representations"][str(scale)]["geometry"][view] if s["id"] == sid)))


def like(base, drop=(), **changes):
    """🧬️ A copy of a shape without the members named in `drop` and with the given members set."""
    out = {k: v for k, v in base.items() if k not in drop}
    out.update(changes)
    return out


GEOMETRY = ("kind", "x", "y", "w", "h", "r", "points", "closed", "d", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "followTip")


def write(slug, ops):
    """📤️ Writes the ops of an asset."""
    os.makedirs(f"{HERE}/ops", exist_ok=True)
    open(f"{HERE}/ops/{slug}.json", "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
    print(f"{slug}: {len(ops)} ops written")
