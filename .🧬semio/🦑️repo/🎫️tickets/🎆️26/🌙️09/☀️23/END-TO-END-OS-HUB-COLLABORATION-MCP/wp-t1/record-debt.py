"""🔒️ Inserts one `productionDebt` record into each named oracle, right after its `testOnly`, keeping the file's own formatting."""
import json, re, sys
spec = json.load(open(sys.argv[1], encoding="utf-8"))
for item in spec:
    path, oid, debt = item["path"], item["oracle"], item["debt"]
    text = open(path, encoding="utf-8").read()
    start = text.index(f'"id": "{oid}"')
    match = re.compile(r'\n([ \t]*)"testOnly": true,').search(text, start)
    updated = text[:match.end()] + f'\n{match.group(1)}"productionDebt": {json.dumps(debt, ensure_ascii=False)},' + text[match.end():]
    oracle = [entry for entry in json.loads(updated)["oracles"] if entry["id"] == oid][0]
    assert oracle["productionDebt"] == debt, oid
    open(path, "w", encoding="utf-8").write(updated)
    print("recorded", oid)
