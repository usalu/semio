"""✍️ Round 3 Labrador: writes corrected root members into the catalog asset file. Re-reads the file right before writing, replaces only the top-level members of `asset` named in the given JSON files (same expanded two-space style as the file), checks that every other member is byte-identical and writes atomically.
Usage: python3 r3-labrador-dog-apply.py <asset 🔣️.json> <members.json> [<members.json> ...]"""
import json, os, re, sys

def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\": index += 1
            elif char == '"': quoted = False
        elif char == '"': quoted = True
        elif char in "{[": depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0: return index + 1
        elif depth == 0 and char in ",\n": return index
        index += 1
    raise SystemExit("unbalanced")

def main():
    path = sys.argv[1]
    members = {}
    for f in sys.argv[2:]: members.update(json.load(open(f, encoding="utf-8")))
    text = open(path, encoding="utf-8").read()
    before = json.loads(text)
    for key, value in members.items():
        found = re.search(r'\n    "' + re.escape(key) + r'": ', text)
        if found is None: raise SystemExit("no member " + key)
        end = member_end(text, found.end())
        body = "\n".join(("    " + line if n else line) for n, line in enumerate(json.dumps(value, indent=2, ensure_ascii=False).split("\n")))
        text = text[: found.end()] + body + text[end:]
    after = json.loads(text)
    for key in before["asset"]:
        if key in members:
            if after["asset"][key] != members[key]: raise SystemExit("member not written: " + key)
        elif after["asset"][key] != before["asset"][key]: raise SystemExit("foreign member changed: " + key)
    if list(after["asset"].keys()) != list(before["asset"].keys()): raise SystemExit("member order changed")
    tmp = path + ".r3-labrador-dog.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.replace(tmp, path)
    print("written", ", ".join(members.keys()))

main()
