"""🛤️ Wheelchair ramp, second round pass 1: handrail returns at 1:100 and 1:200, kerbs at 1:500, in 3D and 2D.

Usage: python3 rework-wr-wheelchair-ramp-returns-kerbs.py candidate|write
`candidate` writes the whole asset document to 🗑️generated/r3-wheelchair-ramp/🔣️.json,
`write` writes the volumes source and the 2D representations of the catalog file.
"""
import json
import re
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
SOURCE = T / "volumes/components/wheelchair-ramp.json"
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/wheelchair-ramp/🔣️.json")
CANDIDATE = T / "🗑️generated/r3-wheelchair-ramp/🔣️.json"

RAIL_STATIONS = {"100": [0, 1, 3, 5, 7, 8, 9, 10, 11, 12, 14, 16, 18, 19], "200": [0, 1, 4, 7, 8, 9, 10, 11, 12, 15, 18, 19]}
BAND_100 = [0, 1, 3, 5, 7, 8, 9, 10, 11, 12, 14, 16, 18, 19, 20, 21, 24, 27, 28, 29, 30, 31, 32, 35, 38, 39]
RETURNS_200 = {"handrail-return-start": [[0.0212, 0.85], [0.0212, 0.75]], "handrail-return-end": [[9.2788, 1.45], [9.2788, 1.35]]}
BEND_STATIONS_100 = [0, 1, 3, 5, 7, 8]
KERB_PROFILE = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.7], [10.2, 0.7], [6.6, 0.4], [5.1, 0.4], [1.5, 0.1]]
MASS_500 = [[1.5, 0], [1.5, 0.1], [5.1, 0.4], [6.6, 0.4], [10.2, 0.7], [11.7, 0.7], [11.7, 0]]
POST_FEET_500 = [[0.4, 0.10833], [2.1, 0.25], [3.8, 0.39167], [5.5, 0.40833], [7.2, 0.55], [8.9, 0.69167]]


def by_id(items, identifier):
    return next(item for item in items if item["id"] == identifier)


def swap(text, old, new):
    assert old in text, old
    return text.replace(old, new)


def reword(entry, pairs):
    for language, old, new in pairs:
        entry[language] = swap(entry[language], old, new)


def volumes(v):
    fine = {side: by_id(v["50"]["body"], f"rail-{side}") for side in "ab"}
    for side in "ab":
        assert len(fine[side]["sections"]) == 20
    for scale, stations in RAIL_STATIONS.items():
        for side in "ab":
            rail = by_id(v[scale]["body"], f"rail-{side}")
            rail["sections"] = [json.loads(json.dumps(fine[side]["sections"][i])) for i in stations]
    members = v["100"]["structure"]
    for side in "ab":
        if any(s["id"] == f"m-rail-{side}-bend-start" for s in members):
            continue
        for end in ("start", "end"):
            straight = by_id(members, f"m-rail-{side}-ext-{end}")
            model = by_id(v["50"]["structure"], f"m-rail-{side}-ext-{end}")
            straight["profile"], straight["origin"] = json.loads(json.dumps(model["profile"])), list(model["origin"])
            bend = json.loads(json.dumps(by_id(v["50"]["structure"], f"m-rail-{side}-bend-{end}")))
            assert len(bend["sections"]) == 9
            bend["sections"] = [bend["sections"][i] for i in BEND_STATIONS_100]
            members.insert(members.index(by_id(members, f"m-rail-{side}-ext-end")) + 1, bend)
    coarse = v["500"]
    slab = next(i for i, s in enumerate(coarse["body"]) if s["id"] == "slab")
    if not any(s["id"] == "kerb-a" for s in coarse["body"]):
        coarse["body"][slab + 1:slab + 1] = [
            {"id": "kerb-a", "kind": "extrude", "bone": "body", "material": "concrete", "role": "body", "tone": 0.62, "profile": KERB_PROFILE, "plane": "xy", "from": 0.6, "to": 0.7},
            {"id": "kerb-b", "kind": "extrude", "bone": "body", "material": "concrete", "role": "body", "tone": 0.62, "profile": KERB_PROFILE, "plane": "xy", "from": -0.7, "to": -0.6},
        ]
        wall = next(i for i, s in enumerate(coarse["structure"]) if s["id"] == "m-wall-b")
        coarse["structure"][wall + 1:wall + 1] = [
            {"id": "m-kerb-a", "of": "kerb-a", "member": "body", "part": "member"},
            {"id": "m-kerb-b", "of": "kerb-b", "member": "body", "part": "member"},
        ]
    for side in "ab":
        for run, foot in (("run-1", 0.10833), ("run-2", 0.40833)):
            post = by_id(coarse["body"], f"post-{side}-{run}")
            post["profile"] = [[0, 0], [0, 0.0375], [0.733, 0.0375], [0.733, 0]]
            post["origin"][1] = foot
            by_id(coarse["structure"], f"j-foot-{side}-{run}")["origin"][1] = foot
        index = next(i for i, s in enumerate(coarse["thermal"]) if s["id"] == f"th-kerb-{side}")
        coarse["thermal"][index] = {"id": f"th-kerb-{side}", "of": f"kerb-{side}", "region": "kerb", "sectionFaces": ["-v", "-u", "+u"]}
    d = v["100"]["decisions"][0]
    if "plain ends" in d["en"]:
        reword(d, [
            ("en", "each handrail as one tube with plain ends (the bends are below what this scale draws)", "each handrail as one tube whose 0.30 m extensions turn down into the 100 mm leg through a 60 mm bend drawn in three chords (the end caps are below what this scale draws)"),
            ("de", "jeder Handlauf als ein Rohr mit glatten Enden (die Bögen liegen unter dem, was dieser Maßstab zeichnet)", "jeder Handlauf als ein Rohr, dessen 0,30 m lange Verlängerungen über einen in drei Sehnen gezeichneten 60-mm-Bogen in den 100 mm langen Schenkel nach unten laufen (die Endkappen liegen unter dem, was dieser Maßstab zeichnet)"),
        ])
        reword(v["200"]["decisions"][0], [
            ("en", "(`slender`, drawn as lines) on their", "(`slender`, drawn as lines) with the turned-down return of each end as two chords, on their"),
            ("de", "(`slender`, als Linien gezeichnet) auf ihren", "(`slender`, als Linien gezeichnet) mit der nach unten geführten Rücklenkung jedes Endes als zwei Sehnen, auf ihren"),
        ])
        reword(coarse["decisions"][0], [
            ("en", "the wedge with the kerbs merged into it, the two handrails as true tubes (`slender`) and the twelve posts at the size floor of 75 mm, standing on the slab edge so that no rail floats", "the wedge and the two 100 mm kerb strips (above the 75 mm feature, so they stay), the two handrails as true tubes (`slender`) with plain ends (the 100 mm return is one feature long and left out) and the twelve posts at the size floor of 75 mm, standing on the kerb top as built"),
            ("de", "der Keil mit den darin aufgegangenen Borden, die zwei Handläufe als wahre Rohre (`slender`) und die zwölf Pfosten an der Größenuntergrenze von 75 mm, auf dem Plattenrand stehend, sodass kein Handlauf schwebt", "der Keil und die zwei Bordstreifen von 100 mm (über dem Merkmal von 75 mm, also bleiben sie), die zwei Handläufe als wahre Rohre (`slender`) mit glatten Enden (die Rücklenkung von 100 mm ist ein Merkmal lang und entfällt) und die zwölf Pfosten an der Größenuntergrenze von 75 mm, wie gebaut auf der Bordoberseite stehend"),
        ])
        reword(coarse["decisions"][1], [
            ("en", "the kerbs as their own 100 mm strips (kept for the region although the body merges them)", "the kerb strips of the body with their three free faces"),
            ("de", "die Borde als eigene Streifen 100 mm (für die Region behalten, obwohl der Körper sie aufnimmt)", "die Bordstreifen des Körpers mit ihren drei freien Seiten"),
        ])
        reword(coarse["decisions"][2], [
            ("en", "rails and posts as bars", "the kerbs cast onto the slab edges, rails and posts as bars"),
            ("de", "Handläufe und Pfosten als Stäbe", "die an die Plattenränder anbetonierten Borde, Handläufe und Pfosten als Stäbe"),
        ])


def drawings(r):
    fine = by_id(r["50"]["geometry"]["elevation"], "handrail")["points"]
    assert len(fine) == 40
    by_id(r["100"]["geometry"]["elevation"], "handrail")["points"] = [list(fine[i]) for i in BAND_100]
    for side in "ab":
        band = by_id(r["100"]["geometry"]["front"], f"handrail-{side}")
        band["y"], band["h"] = 0.75, 0.7212
    strokes = r["200"]["geometry"]["elevation"]
    rail = next(i for i, s in enumerate(strokes) if s["id"] == "handrail")
    if not any(s["id"] == "handrail-return-start" for s in strokes):
        strokes[rail + 1:rail + 1] = [{**strokes[rail], "id": identifier, "points": points} for identifier, points in RETURNS_200.items()]
    elevation = r["500"]["geometry"]["elevation"]
    by_id(elevation, "ramp")["points"] = MASS_500
    for i, foot in enumerate(POST_FEET_500):
        by_id(elevation, f"post-{i}")["points"][0] = foot
    d100, d200, d500 = (r[s]["decisions"] for s in ("100", "200", "500"))
    if "rail returns" in d100["omitted"]["en"]:
        reword(d100["omitted"], [("en", ", rail returns.", ", the end caps of the rail returns."), ("de", ", Handlaufrücklenkungen.", ", die Endkappen der Handlaufrücklenkungen.")])
        reword(d100["retained"], [("en", "three joints.", "three joints, and the turned-down return at both ends of the rail."), ("de", "drei Fugen.", "drei Fugen und die nach unten geführte Rücklenkung an beiden Handlaufenden.")])
        reword(d200["omitted"], [("en", "post base plates, rail returns, stipple", "post base plates, stipple"), ("de", "Pfostenfußplatten, Handlaufrücklenkungen, Punktierung", "Pfostenfußplatten, Punktierung")])
        reword(d200["retained"], [("en", "with extensions, six posts", "with extensions and turned-down returns (0.10 m = 0.5 mm), six posts"), ("de", "mit Verlängerungen, sechs Pfosten", "mit Verlängerungen und nach unten geführten Rücklenkungen (0,10 m = 0,5 mm), sechs Pfosten")])
        reword(d200["silhouette"], [("en", "a handrail line and six post lines.", "a handrail line with a short return stroke at each end and six post lines."), ("de", "eine Handlauflinie und sechs Pfostenlinien.", "eine Handlauflinie mit einem kurzen Rücklenkungsstrich an jedem Ende und sechs Pfostenlinien.")])
        reword(d500["silhouette"], [
            ("en", "and one rail line.", "with the kerb band on top, one rail line and six post lines."),
            ("de", "und eine Handlauflinie.", "mit dem Radabweiserband obenauf, eine Handlauflinie und sechs Pfostenlinien."),
        ])
        reword(d500["retained"], [
            ("en", "0.6 m high at the real 1:12 slope", "0.6 m high (0.7 m with the kerb) at the real 1:12 slope"),
            ("de", "0,6 m hoch mit der echten Steigung 1:12", "0,6 m hoch (0,7 m mit dem Radabweiser) mit der echten Steigung 1:12"),
        ])
        d500["merged"] = {
            "en": "Surface, side faces and the wheel-guard kerb are one filled mass listing all three regions; in the side elevation the mass rises to the kerb top (0.10 m = 0.2 mm), in the front and the plan the 0.10 m kerb squares and strips are left out.",
            "de": "Oberfläche, Seitenflächen und Radabweiser sind eine gefüllte Masse mit allen drei Regionen; in der Seitenansicht reicht die Masse bis zur Oberkante des Radabweisers (0,10 m = 0,2 mm), in Vorderansicht und Draufsicht entfallen die 0,10 m großen Quadrate und Streifen des Radabweisers.",
        }
        d500["omitted"] = {
            "en": "Slab joints, rail returns, stipple, and in the front and the plan the posts; the side elevation starts with the 0.10 m end of the kerb at the foot of the first run.",
            "de": "Plattenfugen, Handlaufrücklenkungen, Punktierung und in Vorderansicht und Draufsicht die Pfosten; die Seitenansicht beginnt mit dem 0,10 m hohen Bordende am Fuß des ersten Laufs.",
        }


def main():
    mode = sys.argv[1]
    source = json.loads(SOURCE.read_text())
    text = CATALOG.read_text()
    document = json.loads(text)
    volumes(source["volumes"])
    drawings(document["asset"]["representations"])
    if mode == "candidate":
        document["asset"]["volumes"] = source["volumes"]
        CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
        CANDIDATE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
        print(CANDIDATE)
    elif mode == "write":
        splice = {}
        exec((T / "volumes-insert.py").read_text().rsplit("main()", 1)[0], splice)
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        start, end = splice["member_range"](text, indent, "representations")
        body = indent + '"representations": ' + splice["dump"](document["asset"]["representations"], len(indent) // 2)
        print("representations byte-identical before the edit:", indent + '"representations": ' + splice["dump"](json.loads(text)["asset"]["representations"], len(indent) // 2) == text[start:end])
        SOURCE.write_text(json.dumps(source, ensure_ascii=False, indent=1) + "\n")
        CATALOG.write_text(text[:start] + body + text[end:])
        print("written", SOURCE, CATALOG)


main()
