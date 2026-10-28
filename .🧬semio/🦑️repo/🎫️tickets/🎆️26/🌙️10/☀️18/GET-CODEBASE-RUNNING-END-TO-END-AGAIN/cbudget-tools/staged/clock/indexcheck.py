#!/usr/bin/env python3
"""⏲️ Law of the derived schema index after batch `clock`: python3 -I indexcheck.py <root>. Against the base copy the index gained exactly the id of the contention policy schema, every id of the process root stays sorted, and the path it names holds a schema with that $id."""
import json, os, sys
here = os.path.dirname(os.path.abspath(__file__)); root = sys.argv[1]
paths = json.load(open(os.path.join(here, "paths.json"), encoding="utf8"))
rel = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"
new = json.load(open(os.path.join(root, rel), encoding="utf8")); base = json.load(open(os.path.join(here, "base", "07.json"), encoding="utf8"))
owner = "🧰️framework/🔨️modules/🏃️process"
added = {k: v for k, v in new["roots"][owner].items() if k not in base["roots"][owner]}
schema_rel = paths["new"]["02.json"][0]
assert added == {"https://json.schemas.assets.semio-tech.com/framework/process/testing/budget/contention.json": schema_rel}, added
assert {k: v for k, v in base["roots"].items() if k != owner} == {k: v for k, v in new["roots"].items() if k != owner}
assert {k: v for k, v in base["roots"][owner].items()} == {k: v for k, v in new["roots"][owner].items() if k in base["roots"][owner]}
keys = list(new["roots"][owner]); assert keys == sorted(keys), "ids out of order"
assert json.load(open(os.path.join(root, schema_rel), encoding="utf8"))["$id"] == next(iter(added))
assert open(os.path.join(root, rel), encoding="utf8").read() == json.dumps(new, ensure_ascii=False, indent=2) + "\n", "index is not in the generator's format"
print("index ok")
