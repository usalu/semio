"""🧮 Content-leaf coverage of an artifact: every leaf of the snapshot JSON Schema against the `SetFieldRule` paths of its edit rules.

`python -I leaf_coverage.py <artifact>...` walks `🧬️schema/📸️snapshot/🔣️.json` (resolving `$ref` and nullable `anyOf`), counts scalar, enum and scalar-array leaves, drops identity `id` leaves and prints the leaves no set-field kind addresses.
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

EMOJI = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️"}


def leaves(schema):
    defs = schema.get("$defs") or schema.get("definitions") or {}

    def resolve(node):
        while True:
            if "$ref" in node:
                node = defs[node["$ref"].split("/")[-1]]
            elif "anyOf" in node or "oneOf" in node:
                node = [x for x in node.get("anyOf") or node["oneOf"] if x.get("type") != "null"][0]
            else:
                return node

    out = []

    def walk(node, path, seen):
        node = resolve(node)
        if node.get("type") == "object" and "properties" in node:
            key = json.dumps(node, sort_keys=True)[:200]
            if key in seen and len(path) > 8:
                return
            for name, child in node["properties"].items():
                walk(child, path + [name], seen | {key})
        elif node.get("type") == "array":
            item = resolve(node["items"])
            if item.get("type") == "object" and "properties" in item:
                walk(item, path[:-1] + [path[-1] + "[]"], seen)
            else:
                out.append(".".join(path))
        else:
            out.append(".".join(path))

    walk(schema, [], set())
    return out


def main():
    for short in sys.argv[1:]:
        base = NORM + "/🗿️artifacts/" + EMOJI[short] + short + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
        schema = json.load(open(base + "/📸️snapshot/🔣️.json"))
        rules = rd(base + "/🧬️mutations/🧭️edit-rules/🦀️.rs")
        covered = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', rules))
        every = leaves(schema)
        content = [l for l in every if not l.endswith(".id") and l != "id"]
        missing = [l for l in content if l not in covered]
        print("%s: %d leaves, %d content leaves, %d set-field paths, %d content leaves without a kind" % (short, len(every), len(content), len(covered), len(missing)))
        for leaf in missing:
            print("   ", leaf)


main()
