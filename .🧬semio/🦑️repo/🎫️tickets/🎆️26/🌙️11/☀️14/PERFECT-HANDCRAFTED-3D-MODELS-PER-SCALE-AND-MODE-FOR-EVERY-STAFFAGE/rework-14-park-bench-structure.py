"""🌳️ Author 14, park bench: at 1:50 the contact pads of the structure list lay under the foot pads of the end frames, but the foot pads were no members, so the casting ended 3 mm above its support. The two foot pads of each end frame become members of that frame by reference to the body.

Run from the ticket folder: python3 rework-14-park-bench-structure.py
"""
import json

PATH = "volumes/furniture/park-bench.json"
document = json.load(open(PATH))
structure = [s for s in document["volumes"]["50"]["structure"] if s["id"] not in ("m-l-pads", "m-r-pads")]
for side, bone in (("l", "end-left"), ("r", "end-right")):
    index = next(i for i, s in enumerate(structure) if s["id"] == f"m-{side}-rim-3") + 1
    structure.insert(index, {"id": f"m-{side}-pads", "of": f"{side}-pads", "member": bone, "part": "member"})
document["volumes"]["50"]["structure"] = structure
decision = document["volumes"]["50"]["decisions"][-1]
OLD = ("the two foot pads of each frame are the support of its contact anchor;", "die zwei Fußplatten jedes Gestells sind das Auflager seines Kontaktankers;")
NEW = ("the two cast foot pads of each frame are members of it and stand on the contact pads of its anchor;", "die zwei angegossenen Fußplatten jedes Gestells sind seine Glieder und stehen auf den Kontaktfeldern seines Ankers;")
for language, old, new in (("en", OLD[0], NEW[0]), ("de", OLD[1], NEW[1])):
    if new not in decision[language]:
        assert old in decision[language], old
        decision[language] = decision[language].replace(old, new)
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
