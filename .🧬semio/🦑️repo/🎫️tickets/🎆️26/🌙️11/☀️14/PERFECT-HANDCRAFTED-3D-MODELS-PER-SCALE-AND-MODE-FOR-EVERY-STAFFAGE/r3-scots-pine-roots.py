"""🌲 Writes the root fixes of the round-3 Scots pine from the candidate document into the catalog asset file: the members named on the command line (`rig` = only the lateral offsets `z` of bones, `anchors`, `representations`, `thermal`, `symbols`), nothing else; the file is re-read right before writing and every other member must stay equal.

Usage: python3 r3-scots-pine-roots.py <candidate 🔣️.json> <catalog 🔣️.json> <member> [<member> ...]
"""
import importlib.util
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
    tool = load_insert()
    candidate = json.load(open(candidate_path, encoding="utf-8"))["asset"]
    text = open(catalog_path, encoding="utf-8").read()
    before = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    result = text
    for member in members:
        start, end = tool["member_range"](result, indent, member)
        if member == "rig":
            block = result[start:end]
            for bone in candidate["rig"]["bones"]:
                found = re.search(r'("id": "' + re.escape(bone["id"]) + r'",\n(?:.*\n)*?)( +)"y": ([^\n]*),\n(?: +"z": [^\n]*,\n)?', block)
                if not found:
                    raise SystemExit("bone %s not found" % bone["id"])
                z = "" if "z" not in bone else found.group(2) + '"z": ' + json.dumps(bone["z"]) + ",\n"
                block = block[: found.start()] + found.group(1) + found.group(2) + '"y": ' + found.group(3) + ",\n" + z + block[found.end() :]
            result = result[:start] + block + result[end:]
        else:
            result = result[:start] + indent + '"%s": ' % member + tool["dump"](candidate[member], depth) + result[end:]
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
