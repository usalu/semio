import json, sys, collections
fixtures = sys.argv[1]
for leaf in ("🚦️change-reliability-member-id", "🧪️change-test-pairs", "🧪️change-test-theory-variation"):
    for role in ("before", "after"):
        path = f"{fixtures}/{leaf}/✅apply/🔣️.{role}.json"
        text = open(path, encoding="utf-8").read()
        document = json.loads(text)
        assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == text, path
        for rows in document.values():
            if not (isinstance(rows, list) and rows and isinstance(rows[0], dict) and "id" in rows[0]):
                continue
            seen = collections.Counter()
            for row in rows:
                seen[row["id"]] += 1
                if seen[row["id"]] > 1:
                    row["id"] = f"{row['id']}-{seen[row['id']]}"
        open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
print("ok")
