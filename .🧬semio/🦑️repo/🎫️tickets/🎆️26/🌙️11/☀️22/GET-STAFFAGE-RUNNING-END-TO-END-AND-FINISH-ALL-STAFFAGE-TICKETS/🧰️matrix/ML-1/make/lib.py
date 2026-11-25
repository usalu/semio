"""🧰️ Shared helpers of the spec writers of ML-1: the catalog document of an asset and the writer of `spec/<id>.json`."""
import glob, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def asset(slug):
    """📖️ The asset of a catalog document."""
    (path,) = glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")
    with open(path, encoding="utf-8") as file:
        return json.load(file)["asset"]


def write(slug, spec):
    """💾️ Writes the spec of an asset and says how many ops it holds."""
    with open(os.path.join(HERE, "..", "spec", f"{slug}.json"), "w", encoding="utf-8") as file:
        json.dump(spec, file, ensure_ascii=False, indent=1)
        file.write("\n")
    print(f"{slug}: {len(spec.get('ops', []))} ops")


def r6(value):
    """🔢️ A number rounded to the micrometre."""
    return round(value, 6) + 0
