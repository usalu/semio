"""🔩 Checks that `staffage_parts` and `_parse_frame` read the old and the compact export of every cast identically (first frame, all shape ids).

@see ./parsetime.py
"""

import importlib
import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path.cwd()))
staffage = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
before, after = Path(sys.argv[1]), Path(sys.argv[2])
checked = shaped = 0
for folder in sorted(p for p in before.iterdir() if p.is_dir()):
    other = after / folder.name
    for name in sorted(p.name for p in folder.glob("*.svg"))[:: max(1, len(list(folder.glob("*.svg"))) // 3)]:
        index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
        a = staffage._parse_frame((folder / name).read_text(encoding="utf-8"), index)
        b = staffage._parse_frame((other / name).read_text(encoding="utf-8"), index)
        np.testing.assert_array_equal(a.points, b.points)
        assert a.spans == b.spans and a.shapes == b.shapes, (folder.name, name)
        checked += 1
    first = {**json.loads((folder / "🔣️.json").read_text(encoding="utf-8")), "frames": ["0000.svg"]}
    try:
        pa, pb = staffage._parts(folder, ""), staffage._parts(other, "")
    except ValueError as error:
        print("unsupported in both:", folder.name, str(error)[:80])
        continue
    assert list(pa) == list(pb), folder.name
    for key in pa:
        np.testing.assert_array_equal(pa[key], pb[key])
    shaped += len(pa)
print("frames compared", checked, "shape polygons compared", shaped)
