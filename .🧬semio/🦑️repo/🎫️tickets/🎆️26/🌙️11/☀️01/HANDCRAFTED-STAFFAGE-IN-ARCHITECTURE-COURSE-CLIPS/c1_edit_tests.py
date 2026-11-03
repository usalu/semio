import json
import re
from pathlib import Path

v = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests")
p = v / "🐍️.py"
s = p.read_text(encoding="utf-8")


def cut(start, end):
    global s
    a = s.index(start)
    b = s.index(end) if end else len(s)
    s = s[:a] + s[b:]


cut("    def test_standing_person_is_one_opaque_closed_silhouette", "    def test_house_material_has_real_wall_thickness")
cut("    def test_radiator_and_person_share_a_plausible_building_scale", "    def test_room_material_meets_at_corners")
cut("    def test_pendant_lamp_has_solid_shade_and_connected_bulb", "    def test_silhouette_rasters_match")
cut("    def test_silhouette_rasters_match", '\n\nif __name__ == "__main__":')
s = s.replace('"""🧪 Architectural silhouettes and dimensions against Manim geometry."""', '"""🧪 Architectural section drawings and dimensions against Manim geometry."""')
s = s.rstrip("\n") + "\n" if False else s
Path("C:/git/semio/.tmp_c1_tests.py").write_text(s, encoding="utf-8", newline="")

c = v / "🔣️.cases.json"
d = json.loads(c.read_text(encoding="utf-8"))
for k in ("standing", "seated", "radiator", "lamp", "raster"):
    d.pop(k, None)
c.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")

sc = v / "🔣️scenes.json"
print(sc.read_text(encoding="utf-8")[:1200])
