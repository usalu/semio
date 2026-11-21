"""🧰 Round 5 O02 (storage-crate-stack, waste-bin, chalkboard, cooking-pot, drinking-fountain): shared helpers to load a round-3 module, write a volumes source, build a candidate document and write changed catalog members in their plain layout.

See ./r5-brief.md and ./r3-tools.md.
"""
import importlib.util
import json
import os
import re

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/%s/🔣️.json"


def module(path, name=None):
    """📦 Loads a python file of the ticket folder as a module."""
    spec = importlib.util.spec_from_file_location(name or os.path.basename(path).replace("-", "_").replace(".py", ""), os.path.join(T, path))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def member(source, key):
    """🔎 Start and end offsets of the value of an asset member `key` (indent 4) in the catalog text."""
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=2, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


def catalog(slug):
    """📖 The current catalog document of an asset (re-read every call)."""
    return json.load(open(CATALOG % slug, encoding="utf-8"))


def write_source(slug, volumes):
    """💾 Writes the volumes source `volumes/objects/<slug>.json`."""
    path = os.path.join(T, "volumes", "objects", slug + ".json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return path


def write_candidate(slug, document):
    """🧪 Writes a candidate document under `🗑️generated/r5-<slug>/🔣️.json` for `r3-validate.ts --file`."""
    out = os.path.join(T, "🗑️generated", "r5-" + slug)
    os.makedirs(out, exist_ok=True)
    path = os.path.join(out, "🔣️.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
    return path


def apply_members(slug, asset, keys):
    """✍️ Writes the given members of `asset` into the freshly re-read catalog file, refusing a member that is not in the plain layout and any unexpected change."""
    fresh = open(CATALOG % slug, encoding="utf-8").read()
    before = json.loads(fresh)
    for key in keys:
        start, end = member(fresh, key)
        if dumped(before["asset"][key]) != fresh[start:end]:
            raise SystemExit("member %s is not in the plain layout, not written" % key)
        fresh = fresh[:start] + dumped(asset[key]) + fresh[end:]
    check = json.loads(fresh)
    for key in check["asset"]:
        if key not in keys and check["asset"][key] != before["asset"][key]:
            raise SystemExit("member %s changed unexpectedly" % key)
    with open(CATALOG % slug, "w", encoding="utf-8") as f:
        f.write(fresh)
    return keys
