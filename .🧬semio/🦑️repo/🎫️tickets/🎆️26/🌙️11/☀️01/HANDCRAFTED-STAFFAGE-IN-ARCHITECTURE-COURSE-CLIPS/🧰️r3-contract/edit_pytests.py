import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
edit(sys.argv[2], [
('''def pieces(group):''', '''def stamped(index: dict, cast: str) -> dict:
    """🔖 Add the provenance the exporter writes for a manifest cast to a hand-made index."""
    provenance = S.expected_provenance(cast)
    provenance["asset"]["category"] = "humans"
    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}
    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}


def pieces(group):'''),
('''json.dumps({**CASE["index"], "cast": cast, "frames": []})''', '''json.dumps(stamped({**CASE["index"], "frames": []}, cast))'''),
('''        (out / name / S._INDEX).write_text(json.dumps({**spec["index"], "cast": name}), encoding="utf-8")
S._run''', '''        (out / name / S._INDEX).write_text(json.dumps(stamped(spec["index"], name)), encoding="utf-8")
S._run'''),
('''(out / name / S._INDEX).write_text(json.dumps({**self.SPEC["index"], "cast": name}), encoding="utf-8")''', '''(out / name / S._INDEX).write_text(json.dumps(stamped(self.SPEC["index"], name)), encoding="utf-8")'''),
('''(folder / S._INDEX).write_text(json.dumps(self.SPEC["index"]), encoding="utf-8")''', '''(folder / S._INDEX).write_text(json.dumps(stamped(self.SPEC["index"], cast)), encoding="utf-8")'''),
('''WORKER = """
import hashlib, importlib, json, sys, time
from pathlib import Path
S = importlib.import_module(sys.argv[1])
''', '''WORKER = """
import hashlib, importlib, json, sys, time
from pathlib import Path
S = importlib.import_module(sys.argv[1])
def stamped(index, cast):
    provenance = S.expected_provenance(cast)
    provenance["asset"]["category"] = "humans"
    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}
    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}
'''),
])
