"""🌿 Writes the root fixes of the round-3 hosta clump from the candidate document into the catalog asset file: only the members named on the command line, each replaced whole in the catalog's own layout (the `dump` of `volumes-insert.py`); the file is re-read right before writing, every other member must stay equal and the member order must stay.

Usage: python3 r3-hosta-clump-roots.py <candidate 🔣️.json> <catalog 🔣️.json> <member> [<member> ...]
"""
import json
import re
import sys


def load_insert():
    source = open("volumes-insert.py", encoding="utf-8").read().replace("\nmain()\n", "\n")
    module = {}
    exec(compile(source, "volumes-insert.py", "exec"), module)
    return module


def main():
    candidate_path, catalog_path, members = sys.argv[1], sys.argv[2], sys.argv[3:]
    if "volumes" in members:
        raise SystemExit("volumes are spliced by volumes-insert.py")
    tool = load_insert()
    candidate = json.load(open(candidate_path, encoding="utf-8"))["asset"]
    text = open(catalog_path, encoding="utf-8").read()
    before = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    result = text
    for member in members:
        start, end = tool["member_range"](result, indent, member)
        result = result[:start] + indent + json.dumps(member) + ": " + tool["dump"](candidate[member], depth) + result[end:]
    after = json.loads(result)
    for key in before["asset"]:
        if key in members:
            if after["asset"][key] != candidate[key]:
                raise SystemExit("member %s did not round-trip" % key)
        elif after["asset"][key] != before["asset"][key]:
            raise SystemExit("member %s changed" % key)
    if list(after["asset"]) != list(before["asset"]):
        raise SystemExit("member order changed")
    if open(catalog_path, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while writing; retry")
    open(catalog_path, "w", encoding="utf-8").write(result)
    print("written:", ", ".join(members))


main()
