import json, sys
fixtures = sys.argv[1]
for leaf in ("🚦️change-reliability-member-id", "🧪️change-test-pairs", "🧪️change-test-theory-variation"):
    for role in ("before", "after"):
        path = f"{fixtures}/{leaf}/✅apply/🔣️.{role}.json"
        text = open(path, encoding="utf-8").read()
        document = json.loads(text)
        assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == text, path
        rows = document["permanents"]
        assert [row["id"] for row in rows] == ["G", "G"], path
        rows[0]["id"] = "G-sup"
        rows[1].update(id="G-inf", kind="gInf")
        open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
print("ok")
