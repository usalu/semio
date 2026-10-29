import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "cafe-chair")
B = {b["id"]: b for b in a["rig"]["bones"]}
B["leg-rear"].update(x=0.05, y=0, angleDeg=88)
B["leg-front"].update(x=0.42, y=0, angleDeg=92)
B["stretcher"].update(x=0.0542, y=0.12, angleDeg=0, lengthM=0.3616)
B["stretcher"]["mass"]["comX"] = 0.1808
B["back"].update(x=0.066, y=0.46, angleDeg=95, lengthM=0.42)
for k in ("leg-rear", "leg-front"): B[k]["lengthM"] = 0.46
B["seat"].update(x=0.03, y=0.46)

g = G()
SB, BB, ST = ("seat-beech", "beech-timber"), ("back-beech", "beech-timber"), ("steel-tube", "steel-tube")
# seat: pad with waterfall nose, apron rail, edge line, grain, screws
g.add("seat-pad", "seat", "body", 0, 4, path("M 0.012 0 L 0.40 0 C 0.416 0 0.426 -0.004 0.428 -0.014 C 0.43 -0.022 0.424 -0.028 0.414 -0.028 L 0.012 -0.028 C 0.004 -0.028 0 -0.022 0 -0.014 C 0 -0.006 0.004 0 0.012 0 Z"), *SB, tone=0.62)
g.add("seat-apron", "seat", "body", 1, 3.8, rect(0.03, -0.062, 0.36, 0.036, 0.004), *SB, tone=0.45)
g.add("seat-edge", "seat", "detail", 1, 4.1, poly([(0.006, -0.0145), (0.424, -0.0145)], False), *SB, filled=False, tone=0.3)
g.add("seat-apron-line", "seat", "detail", 2, 4.1, poly([(0.03, -0.044), (0.39, -0.044)], False), *SB, filled=False, tone=0.3)
g.add("seat-grain", "seat", "texture", 2, 4.2, path("M 0.08 -0.006 C 0.14 -0.003 0.20 -0.009 0.27 -0.005 M 0.15 -0.020 C 0.22 -0.017 0.30 -0.023 0.36 -0.019"), *SB, filled=False, tone=0.3)
for i, x in enumerate((0.05, 0.37)):
    g.add(f"seat-screw-{i}", "seat", "detail", 3, 4.3, ell(x, -0.0145, 0.003, 0.003), *ST, tone=0.2)
# back post and rails (bent beech), finial
g.add("back-post", "back", "body", 0, 3.5, cap(0, 0, 0.405, 0, 0.013, 0.009), *BB, tone=0.42)
g.add("back-finial", "back", "body", 1, 3.6, ell(0.412, 0, 0.012, 0.011), *BB, tone=0.38)
g.add("back-panel", "back", "body", 0, 4, rect(0.215, -0.020, 0.17, 0.028, 0.009), *BB, tone=0.52)
g.add("back-lower-rail", "back", "body", 1, 4, rect(0.11, -0.017, 0.06, 0.020, 0.006), *BB, tone=0.5)
g.add("back-panel-line", "back", "detail", 2, 4.1, poly([(0.23, -0.006), (0.37, -0.006)], False), *BB, filled=False, tone=0.3)
g.add("back-grain", "back", "texture", 2, 4.2, path("M 0.235 -0.013 C 0.28 -0.010 0.33 -0.016 0.375 -0.012"), *BB, filled=False, tone=0.3)
g.add("back-rivet-a", "back", "detail", 3, 4.3, ell(0.24, -0.006, 0.0025, 0.0025), *ST, tone=0.2)
g.add("back-rivet-b", "back", "detail", 3, 4.3, ell(0.36, -0.006, 0.0025, 0.0025), *ST, tone=0.2)
# legs tapered, glides, bend knuckles
for n in ("rear", "front"):
    g.add(f"leg-{n}-tube", f"leg-{n}", "body", 0, 3, cap(0.006, 0, 0.46, 0, 0.0095, 0.0125), *ST, tone=0.35)
    g.add(f"leg-foot-{n}", f"leg-{n}", "detail", 1, 3.1, rect(-0.007, -0.0115, 0.016, 0.023, 0.003), *ST, tone=0.15)
    g.add(f"leg-{n}-ferrule", f"leg-{n}", "detail", 2, 3.2, poly([(0.03, -0.0105), (0.03, 0.0105)], False), *ST, filled=False, tone=0.2)
g.add("leg-rear-knuckle", "leg-rear", "detail", 1, 3.2, ell(0.452, 0, 0.012, 0.014), *ST, tone=0.3)
g.add("stretcher-tube", "stretcher", "body", 0, 2, cap(0, 0, 0.3616, 0, 0.007), *ST, tone=0.35)
g.add("stretcher-joint-a", "stretcher", "detail", 2, 2.1, ell(0.004, 0, 0.0085, 0.0095), *ST, tone=0.25)
g.add("stretcher-joint-b", "stretcher", "detail", 2, 2.1, ell(0.3576, 0, 0.0085, 0.0095), *ST, tone=0.25)
a["geometry"]["elevation"] = g.items

occ = lambda s: [{"id": "occupant", "kind": "occupant", "title": T("Seated adult occupant (75 kg)", "Sitzende erwachsene Person (75 kg)"), "bone": "seat", "x": 0.23, "y": 0, "fxN": 0, "fyN": -735.5, "source": "authored-estimate"},
                 {"id": "occupant-back", "kind": "occupant", "title": T("Occupant leaning on the backrest", "Person lehnt am Rücken"), "bone": "back", "x": 0.28, "y": -0.014, "fxN": -60, "fyN": 0, "source": "authored-estimate"}]
upsert(a["poses"], {"id": "seated-pulled-out", "key": True, "title": T("Seated, pulled out", "Sitzend, herausgezogen"), "description": T("A 75 kg occupant has just sat on the chair, which still stands 0.25 m away from the table.", "Eine Person von 75 kg hat sich gerade gesetzt; der Stuhl steht noch 0,25 m vom Tisch entfernt."), "bones": {}, "root": {"x": -0.25, "y": 0, "angleDeg": 0}, "ground": "auto", "contacts": ["contact-rear", "contact-front"], "thermal": {}, "loads": occ(0)})
upsert(a["poses"], {"id": "seated-occupied", "title": T("Seated at the table", "Sitzend am Tisch"), "description": T("Adult occupant (75 kg, authored estimate) on the seat, a light lean of 60 N on the backrest; all four feet carry the load.", "Erwachsene Person (75 kg, Schätzwert) auf dem Sitz, leichte Anlehnung mit 60 N am Rücken; alle vier Füße tragen die Last."), "bones": {}, "ground": "auto", "contacts": ["contact-rear", "contact-front"], "thermal": {}, "loads": occ(0)})
upsert(a["clips"], {"id": "pull-out-and-sit", "title": T("Pull out and sit", "Herausziehen und Hinsetzen"), "description": T("The chair stands pulled out from the table, an adult sits down and draws it up to the table; the occupant load fades in and the four feet stay in contact throughout.", "Der Stuhl steht vom Tisch weggezogen, eine erwachsene Person setzt sich und rückt ihn an den Tisch; die Personenlast steigt an und die vier Füße bleiben durchgehend in Kontakt."),
  "durationS": 2.8, "fps": 30, "loop": False, "rootMotion": "none",
  "keys": [{"t": 0, "pose": "pulled-out", "ease": "smooth"}, {"t": 0.6, "pose": "pulled-out", "ease": "smooth"}, {"t": 1.3, "pose": "seated-pulled-out", "ease": "smooth"}, {"t": 2.4, "pose": "seated-occupied", "ease": "smooth"}, {"t": 2.8, "pose": "seated-occupied", "ease": "smooth"}],
  "contacts": [{"anchor": "contact-rear", "fromS": 0, "toS": 2.8}, {"anchor": "contact-front", "fromS": 0, "toS": 2.8}],
  "phases": [{"id": "approach", "fromS": 0, "toS": 0.6, "title": T("Chair pulled out", "Stuhl herausgezogen")}, {"id": "sit-down", "fromS": 0.6, "toS": 1.3, "title": T("Sitting down", "Hinsetzen")}, {"id": "draw-up", "fromS": 1.3, "toS": 2.4, "title": T("Drawing up to the table", "Heranrücken an den Tisch")}, {"id": "seated", "fromS": 2.4, "toS": 2.8, "title": T("Seated at the table", "Sitzend am Tisch")}],
  "events": [{"t": 1.3, "kind": "load-on", "title": T("Occupant loads the seat", "Person belastet den Sitz"), "target": "occupant"}, {"t": 2.4, "kind": "motion", "title": T("Chair reaches the table", "Stuhl erreicht den Tisch")}], "tracks": []})
a["structure"]["assumptions"] = [T("Contacts come from the pose or clip; tipping is a rigid rotation about the rear feet.", "Kontakte kommen aus Pose oder Clip; Kippen ist eine starre Drehung um die hinteren Füße."),
  T("Near and far legs coincide in elevation and are drawn once; the occupant is a 75 kg point load on the seat plus a 60 N lean on the backrest (authored estimates).", "Nahe und ferne Beine fallen im Aufriss zusammen und sind einmal gezeichnet; die Person ist eine Punktlast von 75 kg auf dem Sitz plus 60 N Anlehnung am Rücken (Schätzwerte).")]
save("objects", doc)
