import collections, glob, json, sys
artifacts, *families = sys.argv[1:]
def fix(node):
    changed = False
    if isinstance(node, list):
        if node and all(isinstance(row, dict) and "id" in row for row in node):
            seen = collections.Counter()
            for row in node:
                seen[row["id"]] += 1
                if seen[row["id"]] > 1:
                    row["id"] = f"{row['id']}-{seen[row['id']]}"
                    changed = True
        for row in node:
            changed |= fix(row)
    elif isinstance(node, dict):
        for value in node.values():
            changed |= fix(value)
    return changed
count = 0
for family in families:
    for role in ("before", "after"):
        for path in glob.glob(f"{artifacts}/{family}/🏅️standards/*/🪆️subsets/*/🧪️tests/🧫️fixtures/🧬️mutations/*/*/🔣️.{role}.json"):
            text = open(path, encoding="utf-8").read()
            document = json.loads(text)
            assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == text, path
            if fix(document):
                open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
                count += 1
print(count)
