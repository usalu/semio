"""🏃️ Corrects the thermal model of runner-with-blade-prosthesis: the running metabolic rate is the heat of the stated oxygen uptake in the solver's met (58.15 W/m2 of skin), not the ACSM MET count, and the reference wind is the running speed in calm air: python3 fix-runner-thermal.py"""
import sys

sys.path.insert(0, __file__.rsplit("/", 2)[0] + "/🧰️three-views")
from tv2_doc import Doc, catalog_file

OLD, NEW = 11.5, 9.25
doc = Doc(catalog_file("runner-with-blade-prosthesis"))
if doc.get("asset", "thermal", "environment", "windMS") != 1.5:
    raise SystemExit("already applied")
for index, pose in enumerate(doc.get("asset", "poses")):
    if pose["thermal"].get("activityMet") == OLD:
        doc.set(("asset", "poses", index, "thermal", "activityMet"), NEW)
for c, clip in enumerate(doc.get("asset", "clips")):
    for t, track in enumerate(clip.get("tracks", [])):
        if track["target"] == "activityMet":
            for k, key in enumerate(track["keys"]):
                if key["v"] == OLD:
                    doc.set(("asset", "clips", c, "tracks", t, "keys", k, "v"), NEW)
doc.set(("asset", "thermal", "environment", "windMS"), 3.1)
for p, pattern in enumerate(doc.get("asset", "thermal", "respiration", "patterns")):
    if pattern["id"] == "running":
        doc.set(("asset", "thermal", "respiration", "patterns", p, "title"), {"en": "Running at 3.1 m/s, about 955 W", "de": "Laufen mit 3,1 m/s, etwa 955 W"})
assumptions = doc.get("asset", "thermal", "assumptions")
index = [i for i, a in enumerate(assumptions) if a["en"].startswith("Level running at 3.1 m/s")]
if len(index) != 1:
    raise SystemExit("running assumption not found")
doc.set(("asset", "thermal", "assumptions", index[0]), {
    "en": "Level running at 3.1 m/s (184 m/min) gives 0.2 x 184 + 3.5 = 40 ml/(kg min) of oxygen after the ACSM running equation, 11.5 MET of 3.5 ml/(kg min). For 68.5 kg that is 2.74 L/min, and with the energy equivalent of oxygen of ISO 8996 (5.81 Wh/L at a respiratory quotient of 0.95) a metabolic rate of 955 W. The model counts activity in met of 58.15 W per m2 of skin (103.3 W for 1.776 m2), so running is 9.25 met; the MET count per body mass is not the met per skin area. Ventilation of about 70 L/min (36 per minute, 1.95 L) is an authored estimate for that load. Runners with blades are reported to run at a similar cost (Weyand et al. 2009); no extra cost is applied.",
    "de": "Ebenes Laufen mit 3,1 m/s (184 m/min) ergibt 0,2 x 184 + 3,5 = 40 ml/(kg min) Sauerstoff nach der ACSM-Laufgleichung, 11,5 MET zu 3,5 ml/(kg min). Für 68,5 kg sind das 2,74 L/min und mit dem Energieäquivalent des Sauerstoffs nach ISO 8996 (5,81 Wh/L bei einem respiratorischen Quotienten von 0,95) ein Energieumsatz von 955 W. Das Modell zählt die Aktivität in met zu 58,15 W je m2 Haut (103,3 W für 1,776 m2), Laufen ist also 9,25 met; die MET-Zahl je Körpermasse ist nicht das met je Hautfläche. Eine Lüftung von etwa 70 L/min (36 je Minute, 1,95 L) ist für diese Last geschätzt. Läufer mit Prothesenfeder laufen berichtet bei ähnlichem Aufwand (Weyand et al. 2009); ein Mehraufwand wird nicht angesetzt.",
})
sun = [i for i, a in enumerate(doc.get("asset", "thermal", "assumptions")) if a["en"].startswith("Sunny morning")]
doc.insert(("asset", "thermal", "assumptions"), sun[0] + 1, {
    "en": "The reference wind of 3.1 m/s from ahead is the running speed in calm air: the figure runs in place while ground and air pass (treadmill frame), so the air moves over the skin at the speed of the run.",
    "de": "Der Bezugswind von 3,1 m/s von vorn ist die Laufgeschwindigkeit bei Windstille: Die Figur läuft am Ort, Boden und Luft ziehen vorbei (Laufbandsystem), die Luft streicht also mit der Laufgeschwindigkeit über die Haut.",
})
references = doc.get("asset", "provenance", "references")
if not any(r["id"] == "iso-8996-2021" for r in references):
    doc.insert(("asset", "provenance", "references"), len(references), {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate: energy equivalent of oxygen EE = (0.23 RQ + 0.77) x 5.88 Wh per litre."})
doc.save()
print("runner thermal corrected")
