"""📮 Publish a validated candidate of humans/sleeping-person: its volumes into the source, the named root keys into the catalog document.

Usage: python3 rework-sleeping-person-5-publish.py <candidate 🔣️.json> rig poses clips representations ...
The catalog file is re-read immediately before it is written and only the named keys of `asset` are replaced; run volumes-insert.py afterwards.
"""
import json
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
SOURCE = T + "/volumes/humans/sleeping-person.json"


def main(candidate, keys):
    asset = json.load(open(candidate))["asset"]
    json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w"), ensure_ascii=False)
    text = open(CATALOG).read()
    fresh = json.loads(text)
    for key in keys:
        fresh["asset"][key] = asset[key]
    first = text.find("\n")
    indent = None if first < 0 else len(text[first + 1:]) - len(text[first + 1:].lstrip(" "))
    with open(CATALOG, "w") as handle:
        json.dump(fresh, handle, ensure_ascii=False, indent=indent)
        handle.write("\n" if text.endswith("\n") else "")
    print("published", keys)


main(sys.argv[1], sys.argv[2:])
