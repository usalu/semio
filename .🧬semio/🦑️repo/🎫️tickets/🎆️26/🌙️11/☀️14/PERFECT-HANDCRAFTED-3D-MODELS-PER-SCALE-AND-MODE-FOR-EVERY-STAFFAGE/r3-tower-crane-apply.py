"""🖊️ Writes the root fixes of `machinery/tower-crane` from the validated candidate (🗑️generated/r3-tower-crane/🔣️.json) into the catalog file: re-reads the catalog right before writing and replaces, by text surgery in the file's own style, only the members that the round changes (dimensions, rig, anchors, representations, symbols, thermal); `volumes` is spliced afterwards by volumes-insert.py.

Usage: python3 r3-tower-crane-apply.py
@see ./r3-tower-crane-jsontext.py
@see ./r3-tower-crane-build.py
"""
import json
import os
from importlib import util

HERE = os.path.dirname(os.path.abspath(__file__))
_s = util.spec_from_file_location("jt", os.path.join(HERE, "r3-tower-crane-jsontext.py"))
J = util.module_from_spec(_s)
_s.loader.exec_module(J)
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
CANDIDATE = os.path.join(HERE, "🗑️generated/r3-tower-crane/🔣️.json")
MEMBERS = ("dimensions", "rig", "anchors", "representations", "symbols", "thermal")

if __name__ == "__main__":
    new = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    old = json.loads(text)["asset"]
    changed = []
    for key in MEMBERS:
        if old[key] != new[key]:
            text = J.replace(text, key, new[key])
            changed.append(key)
    after = json.loads(text)["asset"]
    assert list(after) == list(old), "member order changed"
    assert all(after[k] == (new[k] if k in MEMBERS else old[k]) for k in old), "unexpected change"
    open(CATALOG, "w", encoding="utf-8").write(text)
    print("changed:", ", ".join(changed) or "nothing")
