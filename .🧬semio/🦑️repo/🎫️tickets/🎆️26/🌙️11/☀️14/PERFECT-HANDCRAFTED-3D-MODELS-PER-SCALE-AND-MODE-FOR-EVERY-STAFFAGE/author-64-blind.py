"""🪟 Author 64: external-venetian-blind 1:500 modelled anew against the 1:500 drawing: every fourth lamella (0, 4, 8, 12, 16) as a 75 mm bar like the five drawn strokes, the other twelve stay as thermal-only plates so the lamella surface area is not lost."""
import json, os, sys, time, copy
T = os.path.dirname(os.path.abspath(__file__))
path = f"{T}/volumes/components/external-venetian-blind.json"
if (time.time() - os.path.getmtime(path)) / 60 < 20 and "--force-age" not in sys.argv:
    print("SKIP"); sys.exit()
raw = open(path).read()
src = json.loads(raw)
v = src["volumes"]["500"]
keep = {"slat-0", "slat-4", "slat-8", "slat-12", "slat-16"}
dropped = [x for x in v["body"] if x["id"].startswith("slat-") and x["id"] not in keep]
if not dropped:
    print("already"); sys.exit()
v["body"] = [x for x in v["body"] if x not in dropped]
ids = {x["id"] for x in dropped}
new = []
for t in v["thermal"]:
    if t.get("of") in ids:
        solid = copy.deepcopy(next(x for x in dropped if x["id"] == t["of"]))
        solid["id"] = t["id"]
        solid["role"] = "body"
        for k in ("region", "layer", "sectionFaces"):
            if k in t:
                solid[k] = t[k]
        new.append(solid)
    else:
        new.append(t)
v["thermal"] = new
v["decisions"] = [
 {"en": "Body: the 1:500 drawing strokes only every fourth lamella (0, 4, 8, 12, 16) and the bottom rail, so the body is the wall sill and head, the 50 mm glass slab, the 150 x 210 mm box, the two 60 mm guide bars, five 75 mm lamella bars and the 75 mm bottom rail; the twelve lamellae in between are no longer drawn and have no body solid.",
  "de": "Körper: die Zeichnung 1:500 strichelt nur jede vierte Lamelle (0, 4, 8, 12, 16) und die Endschiene, daher besteht der Körper aus Wandsockel und -sturz, der 50-mm-Glasplatte, dem Kasten 150 x 210 mm, den zwei 60-mm-Führungsstäben, fünf 75-mm-Lamellenbalken und der 75-mm-Endschiene; die zwölf Lamellen dazwischen sind nicht mehr gezeichnet und haben keinen Körper."},
 {"en": "Thermal: all seventeen lamellae keep their two broad faces in the lamellae region (5 by reference to the body bars, 12 as thermal-only plates on their own bones), because the exchange surface of the blind is its full lamella count whatever the drawing shows; the wall, glass, box and rails reference the body as before. Structure: members for the wall leaf, the box and both guides, a 75 mm foot block; the lamellae carry no load.",
  "de": "Thermik: alle siebzehn Lamellen behalten ihre zwei breiten Flächen in der Region Lamellen (5 per Verweis auf die Körperbalken, 12 als reine Thermikplatten an ihren eigenen Bones), weil die Austauschfläche des Behangs ihre volle Lamellenzahl ist, was immer die Zeichnung zeigt; Wand, Glas, Kasten und Schienen verweisen wie bisher auf den Körper. Tragwerk: Stäbe für Wandschale, Kasten und beide Führungen, ein 75-mm-Fußblock; die Lamellen tragen nichts."},
 {"en": "Minimal: five lamella bars with the bottom rail are the least that still reads as a lowered blind in the 1:500 drawing; fewer would lose the rhythm of the strokes, more would merge into a block at the 75 mm feature.",
  "de": "Minimal: fünf Lamellenbalken mit der Endschiene sind das Wenigste, das in der Zeichnung 1:500 noch als herabgelassener Behang liest; weniger verlöre den Rhythmus der Striche, mehr würde bei der Einzelheit 75 mm zu einem Block verschmelzen."},
]
out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
assert open(path).read() == raw
open(path, "w").write(out)
print("WROTE", len(dropped), "slats dropped from body")
