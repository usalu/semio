import json
from pathlib import Path
import sqlite3

drawing = Path("✏️s/🔌️plugins/🖍️draw")
owner = json.loads(next(drawing.glob("**/📋️native-owner/🔣️.json")).read_text(encoding="utf-8"))
source = next(drawing.glob("**/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"))
document = json.loads(source.read_text(encoding="utf-8"))
db = sqlite3.connect(":memory:")
query = """
    WITH RECURSIVE tree(node, path, rank) AS (
      SELECT value, json_array(CAST(key AS INTEGER)), printf('%08d', key)
      FROM json_each(?1)
      UNION ALL
      SELECT child.value, json_insert(tree.path, '$[#]', CAST(child.key AS INTEGER)),
             tree.rank || ':' || printf('%08d', child.key)
      FROM tree, json_each(tree.node, '$.children') child
    )
    SELECT json_extract(node, '$.name'), path FROM tree
    WHERE json_extract(node, '$.id') = ?2
    ORDER BY rank LIMIT 1
"""

def keyed(node):
    return {
        "id": node["id"].encode().hex(),
        "name": node["name"].encode().hex(),
        "children": [keyed(child) for child in node.get("children", [])],
    }

def first(layers, target):
    row = db.execute(query, (json.dumps([keyed(layer) for layer in layers]), target.encode().hex())).fetchone()
    return (bytes.fromhex(row[0]).decode(), json.loads(row[1])) if row else (None, None)

layers = document["layers"]
later = dict(layers[1]["children"][0], name="Later Duplicate")
for case in owner["lookup"]["cases"]:
    assert first(layers + [later], case["target"]) == (case["name"], case["path"])

repeat = owner["repeat"]
long = owner["text"] * repeat + "\0"
layers[1]["children"][0]["name"] = long
layers.extend((dict(layers[0], id=long, name="Long Identity"), dict(layers[0], name="Later Duplicate")))
deep = dict(layers[0], id="deep-rename", name="Deep Original")
deep_case = owner["rename"]["cases"][-1]
for depth in range(len(deep_case["path"]) - 1):
    deep = dict(layers[1], id=f"nested-{depth}", children=[deep])
layers.append(deep)
for case in owner["rename"]["cases"]:
    target = long if case.get("repeatTarget") else case["target"]
    authored = case["newName"] * (repeat if case.get("repeatNewName") else 1)
    if case["disposition"] == "no-op" and case.get("repeatNewName") or "suffix" in case:
        authored += "\0"
    authored += case.get("suffix", "")
    previous, path = first(layers, target)
    expected_name = long if case["inverseName"] == "long" else case["inverseName"]
    assert previous == expected_name
    assert db.execute(
        "SELECT CASE WHEN ?1 IS NULL THEN 'missing' WHEN ?1=?2 THEN 'no-op' ELSE 'changed' END",
        (previous, authored),
    ).fetchone()[0] == case["disposition"]
    assert path == case["path"]
