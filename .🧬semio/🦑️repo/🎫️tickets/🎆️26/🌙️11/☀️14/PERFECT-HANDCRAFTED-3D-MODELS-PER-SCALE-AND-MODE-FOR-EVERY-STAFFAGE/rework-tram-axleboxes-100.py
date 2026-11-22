"""🚋 Tram 1:100: trailing-bogie axle boxes (250 mm, outside the wheels) with their primary pads in the body, and the journal - axle box / gearbox - primary - side beam chain in the structure."""
import copy, json, sys

PATH = "volumes/vehicles/tram.json"
doc = json.load(open(PATH))
body, st = doc["volumes"]["100"]["body"], doc["volumes"]["100"]["structure"]
if any(s.get("id") == "axle-box-r-l-1" for s in body):
    sys.exit("already applied")
ids = lambda items: [s.get("id") for s in items]
gear = next(s for s in body if s["id"] == "gearbox-f-l-1")
pad = next(s for s in body if s["id"] == "primary-f-l-1")
for side, sz in (("l", 1), ("r", -1)):
    for n, x in (("1", -0.9), ("2", 0.9)):
        box = copy.deepcopy(gear) | {"id": f"axle-box-r-{side}-{n}", "bone": "bogie-r", "profile": [[0, 0.0], [0, 0.125], [0.135, 0.125], [0.135, 0.0]], "axis": [0, 0, sz], "origin": [x, 0.0, sz * 0.825]}
        spring = copy.deepcopy(pad) | {"id": f"primary-r-{side}-{n}", "bone": "bogie-r", "size": [0.2, 0.07, 0.15], "transform": {"translation": [x, 0.155, sz * 0.915]}}
        at = ids(body).index(f"stub-axle-ws-r2-r") + 1
        body[at:at] = [box, spring]
        for bg, hub in (("f", "gearbox"), ("r", "axle-box")):
            at = ids(st).index(f"st-bogie-beam-{bg}-{side}") + 1
            st[at:at] = [{"id": f"st-{hub}-{bg}-{side}-{n}", "of": f"{hub}-{bg}-{side}-{n}", "member": f"bogie-{bg}", "part": "member"}, {"id": f"st-primary-{bg}-{side}-{n}", "of": f"primary-{bg}-{side}-{n}", "member": f"bogie-{bg}", "part": "joint"}]
            ws = f"ws-{bg}{n}"
            wheel = next(s for s in st if s["id"] == f"st-wheel-{ws}-{side}")
            journal = {k: v for k, v in next(s for s in st if s["id"] == "pivot-f").items()} | {"id": f"axle-journal-{ws}-{side}", "bone": ws, "member": ws, "part": "member", "profile": [[0, 0.0], [0, 0.06], [0.39, 0.06], [0.39, 0.0]], "axis": [0, 0, sz], "origin": [0.0, 0.0, sz * 0.56]}
            st.insert(st.index(wheel) + 1, journal)
json.dump(doc, open(PATH, "w"), ensure_ascii=False)
print("applied")
