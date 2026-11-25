import json,sys

def key(row):
    text=row["kind"]+"\u0000"+row["path"]+"\u0000"+json.dumps(row.get("fact"),ensure_ascii=False,separators=(",",":"))
    return text.encode("utf-16-be")

def sorted_rows(rows):
    return all(key(rows[i])<=key(rows[i+1]) for i in range(len(rows)-1))

bundle=json.load(open(sys.argv[1],encoding="utf8"))
print(json.dumps({
    "rowsSorted":sorted_rows(bundle["rows"]),
    "layersSorted":[sorted_rows(layer) for layer in bundle["layers"]],
    "factsOnly":sorted(bundle["sidecar"].keys())==["facts"],
}))
