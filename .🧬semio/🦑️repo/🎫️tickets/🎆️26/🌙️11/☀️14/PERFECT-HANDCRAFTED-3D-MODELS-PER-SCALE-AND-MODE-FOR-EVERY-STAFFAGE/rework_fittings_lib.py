"""🧷 Helpers of the fittings rework (2026-11-22 night): load and save a volumes source in the style it has, look solids up by id."""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))


def path(asset):
    """📍 Path of the volumes source of `category/slug`."""
    return os.path.join(T, "volumes", asset + ".json")


def load(asset):
    """📖 Reads the source and remembers whether it is indented."""
    with open(path(asset), encoding="utf8") as f:
        text = f.read()
    return json.loads(text), text.startswith("{\n")


def save(asset, document, indented):
    """💾 Writes the source back in the style it had."""
    with open(path(asset), "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1 if indented else None)
        f.write("\n")


def by_id(solids):
    """🔎 Index of a list of solids by id."""
    return {solid["id"]: solid for solid in solids}


def drop(solids, ids):
    """✂️ Removes the named solids from the list in place and insists that each was there."""
    missing = set(ids) - {solid["id"] for solid in solids}
    if missing:
        raise SystemExit("not in the list (already applied?): " + ", ".join(sorted(missing)))
    solids[:] = [solid for solid in solids if solid["id"] not in ids]


def say(scale_volumes, en, de):
    """🗣️ Appends one decision."""
    scale_volumes["decisions"].append({"en": en, "de": de})


import re

from rework_19_lib import dump, member_end

FOLDER = {"components": "🧱️components", "objects": "🪑️objects", "vehicles": "🚜️vehicles"}


def catalog(asset):
    """🗂️ Path of the catalog file of `category/slug`."""
    category, slug = asset.split("/")
    return os.path.join(T.split("/.🧬semio/")[0], "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", FOLDER[category], slug, "🔣️.json")


def read_asset(asset):
    """📗 The catalog asset as it is now."""
    with open(catalog(asset), encoding="utf8") as f:
        return json.load(f)["asset"]


def spliced(asset, members):
    """🪡 Text of the catalog file with the named root members replaced, every other member byte-identical, and the text it was made from."""
    with open(catalog(asset), encoding="utf8") as f:
        text = f.read()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    result = text
    for key, value in members.items():
        found = re.search(r'\n' + indent + '"' + key + r'"\s*:\s*', result)
        if not found:
            raise SystemExit("no member " + key)
        result = result[: found.end()] + dump(value, len(indent) // 2) + result[member_end(result, found.end()):]
    before, after = json.loads(text)["asset"], json.loads(result)["asset"]
    for key in before:
        if (after[key] != members[key]) if key in members else (after[key] != before[key]):
            raise SystemExit("splice went wrong at " + key)
    return result, text


def candidate(asset, members):
    """🧪 Writes the spliced document as a candidate under 🗑️generated and returns its path."""
    result, _ = spliced(asset, members)
    folder = os.path.join(T, "🗑️generated", "r3-" + asset.split("/")[1])
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf8") as f:
        f.write(result)
    return os.path.join(folder, "🔣️.json")


def write_members(asset, members):
    """✍️ Replaces the named root members in the catalog file (re-read right before the write)."""
    result, text = spliced(asset, members)
    with open(catalog(asset), encoding="utf8") as f:
        if f.read() != text:
            raise SystemExit("asset changed while writing; retry")
    with open(catalog(asset), "w", encoding="utf8") as f:
        f.write(result)
    print("written:", ", ".join(members))
