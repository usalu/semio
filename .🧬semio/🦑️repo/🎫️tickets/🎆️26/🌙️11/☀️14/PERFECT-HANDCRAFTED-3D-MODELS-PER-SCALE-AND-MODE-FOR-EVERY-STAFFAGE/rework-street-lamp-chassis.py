"""🦴 Hand edit of `objects/street-lamp` (second round, pass 1): at 1:500 the die-cast luminaire body itself is the head member.

Usage: python3 rework-street-lamp-chassis.py
The separate hexagonal chassis prism inside the 1:500 head (three stations, flat ends) is replaced by a reference to the head loft: at a 75 mm feature the casting that carries the gear and clamps the spigot is one part. Writes the source; publish with `volumes-insert.py`.
"""
import json
import os
import types

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/street-lamp.json")
layout = open(os.path.join(T, "volumes-insert.py")).read()
insert = types.SimpleNamespace()
exec("import json\n" + layout[layout.index("def scalar"):layout.index("def member_end")], insert.__dict__)
source = json.load(open(SOURCE))
model = source["volumes"]["500"]
rows = model["structure"]
index = next(i for i, row in enumerate(rows) if row["id"] == "member-head-chassis")
rows[index] = {"id": "member-head-chassis", "of": "head", "member": "head", "part": "member"}
for decision in model["decisions"]:
    decision["en"] = decision["en"].replace("a chassis loft in the head", "the die-cast head itself as its member")
    decision["de"] = decision["de"].replace("ein Rahmenloft im Kopf", "der Druckgusskopf selbst als sein Stab")
open(SOURCE, "w").write(insert.dump(source, 0) + "\n")
print([d["en"][:160] for d in model["decisions"] if d["en"].startswith("Structure")])
