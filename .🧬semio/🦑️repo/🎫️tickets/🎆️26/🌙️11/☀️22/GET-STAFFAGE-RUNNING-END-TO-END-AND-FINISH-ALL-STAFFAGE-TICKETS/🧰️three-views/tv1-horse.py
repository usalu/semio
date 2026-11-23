"""🐴️ Horse, three-views sweep (pose standing-rest, neck lowered by 32 degrees): the front crest and neck drawings were carried as flat plates at the middle depth of a same-named side shape, so they sank less than the side elevation. `python3 tv1-horse.py [target json]` writes the fix to the catalog file (or to a candidate file)."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("horse"))

for old, new in (("mane-1", "crest-base"), ("mane-2", "crest-mid"), ("mane-3", "crest-poll")):
    path, _ = doc.shape(50, "front", old)
    doc.set(path + ("id",), new)

path, neck = doc.shape(200, "front", "neck-2", "neck-2")
lower = dict(neck)
lower["points"] = [[-0.0309, 0.263], [0.0351, 0.263], [0.0771, 0.2813], [0.0981, 0.3088], [0.1041, 0.3394], [0.1041, 0.474], [0.0968, 0.5692], [0.0944, 0.6], [0.06, 0.625], [0.0, 0.635], [-0.06, 0.625], [-0.093, 0.6], [-0.0956, 0.5692], [-0.1059, 0.4496], [-0.0999, 0.3088], [-0.0669, 0.2752]]
upper = dict(neck)
upper["id"] = "neck-3"
upper["bone"] = "neck-3"
upper["points"] = [[-0.06, 0.545], [0.0, 0.535], [0.06, 0.545], [0.0975, 0.56], [0.0944, 0.6], [0.0801, 0.7861], [0.0351, 0.8442], [-0.0009, 0.8564], [-0.0369, 0.8442], [-0.0759, 0.7983], [-0.093, 0.6], [-0.096, 0.56]]
doc.many(path, [lower, upper])

for old, new in (("neck-a", "neck-girth-a"), ("neck-b", "neck-girth-b")):
    path, _ = doc.shape(500, "front", old)
    doc.set(path + ("id",), new)

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
