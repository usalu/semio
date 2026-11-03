"""✂️ pp: splice the single-bone rig and the three-view geometry blocks into tower_server.py."""
from pathlib import Path

here = Path(__file__).parent
lines = (here / "tower_server.py").read_text(encoding="utf-8").split("\n")
geometry = (here / "geometry_block.py").read_text(encoding="utf-8").rstrip("\n").split("\n")
views = (here / "views_block.py").read_text(encoding="utf-8").rstrip("\n").split("\n")


def index(prefix, start=0):
    return next(i for i in range(start, len(lines)) if lines[i].startswith(prefix))


b0, b1 = index("bones = ["), index("level_title")
lines[b0:b1] = geometry + [""]
r0, r1 = index("def rect("), index("def decisions")
lines[r0:r1] = views + [""]
text = "\n".join(lines)
text = text.replace('"geometry": {"elevation": g50}', '"geometry": {"elevation": el50, "front": fr50, "plan": pl50}')
text = text.replace('"geometry": {"elevation": g100}', '"geometry": {"elevation": el100, "front": fr100, "plan": pl100}')
text = text.replace('"geometry": {"elevation": g200}', '"geometry": {"elevation": el200, "front": fr200, "plan": pl200}')
text = text.replace(', "viewpoints": ["elevation"]}', "}")
text = text.replace('"rig": {"kind": "mechanical", "root": "base", "bones": bones}', '"rig": {"kind": "mechanical", "root": "case", "bones": bones}')
text = text.replace("*[s for s in []]", "")
(here / "tower_server.py").write_text(text, encoding="utf-8")
print("spliced")
