from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub('"wwr": 0.3, "overhang": 0, "insulation": 0.16}},\n    {"id": "ribbon"', '"wwr": 0.2, "overhang": 0, "insulation": 0.16}},\n    {"id": "ribbon"')
sub('"wwr": 0.5, "overhang": 0.6, "insulation": 0.16}},\n    {"id": "glass_shaded"', '"wwr": 0.5, "overhang": 0.6, "insulation": 0.16}},\n    {"id": "glass_shaded"')
sub('{"orientation": 180, "wwr": 0.8, "overhang": 1.2, "insulation": 0.16}},\n    {"id": "punched_insulated"', '{"orientation": 180, "wwr": 0.75, "overhang": 1.2, "insulation": 0.16}},\n    {"id": "punched_insulated"')
sub('"wwr": 0.3, "overhang": 0, "insulation": 0.3}},', '"wwr": 0.2, "overhang": 0, "insulation": 0.25}},')
sub('"wwr": 0.8, "overhang": 0, "insulation": 0.16}},\n    {"id": "glass_premium"', '"wwr": 0.75, "overhang": 0, "insulation": 0.16}},\n    {"id": "glass_premium"')
sub('"wwr": 0.8, "overhang": 1.2, "insulation": 0.3}}', '"wwr": 0.75, "overhang": 1.2, "insulation": 0.3}}')
sub('"cost_heavy": {"electricity": 0.2, "cost": 0.6, "daylight": 0.13333333333333333, "carbon": 0.06666666666666667}', '"cost_heavy": {"electricity": 0.22857142857142856, "cost": 0.6, "daylight": 0.11428571428571428, "carbon": 0.05714285714285714}')
p.write_text(s, encoding="utf-8")
