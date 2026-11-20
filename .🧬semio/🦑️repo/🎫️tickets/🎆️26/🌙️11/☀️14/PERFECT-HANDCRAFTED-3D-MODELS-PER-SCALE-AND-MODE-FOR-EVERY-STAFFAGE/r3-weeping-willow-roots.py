"""🌱 Writes the root fixes of the round-3 weeping willow from the candidate document into the catalog asset file: the members named on the command line (`rig`, `anchors`, `poses`, `representations`, `thermal`), nothing else; the file is re-read right before writing and every other member must stay equal.

Usage: python3 r3-weeping-willow-roots.py <candidate 🔣️.json> <catalog 🔣️.json> <member> [<member> ...]
"""
import json
import sys


def load_insert():
    source = open("volumes-insert.py", encoding="utf-8").read().replace("\nmain()\n", "\n")
    module = {}
    exec(compile(source, "volumes-insert.py", "exec"), module)
    return module


def member_range(tool, text, member):
    key = '\n    "%s": ' % member
    start = text.index(key) + 1
    value = start + len(key) - 1
    if text[value] in "{[":
        return start, tool["member_end"](text, value)
    end = value
    while text[end] not in ",\n":
        end += 1
    return start, end


def main():
    candidate_path, catalog_path, members = sys.argv[1], sys.argv[2], sys.argv[3:]
    tool = load_insert()
    candidate = json.load(open(candidate_path, encoding="utf-8"))["asset"]
    text = open(catalog_path, encoding="utf-8").read()
    before = json.loads(text)
    result = text
    for member in members:
        start, end = member_range(tool, result, member)
        if member == "representations":
            block = '    "representations": ' + tool["dump"](candidate[member], 2)
        else:
            block = '    "%s": ' % member + json.dumps(candidate[member], ensure_ascii=False, indent=2).replace("\n", "\n    ")
        result = result[:start] + block + result[end:]
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
