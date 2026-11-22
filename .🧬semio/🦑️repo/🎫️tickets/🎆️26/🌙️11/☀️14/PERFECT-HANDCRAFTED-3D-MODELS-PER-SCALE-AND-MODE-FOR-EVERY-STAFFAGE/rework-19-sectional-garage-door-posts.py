"""🚪 Rework 19, sectional garage door: the end bearing plates of the spring line no longer hang in the air. Each stands on a wall angle 45 x 30 x 2.5 mm that continues the jamb fixing from the flag bracket (top 2.14 m) to the bearing plate (bottom 2.66 m); 1:500 gets the bearing plates it lacked. Hand-chosen numbers, at 1:500 post and plate are one folded piece; applied to the source and to the 2D drawings of the catalog asset."""
import json, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
import rework_19_lib as L

SRC = "volumes/components/sectional-garage-door.json"
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sectional-garage-door/🔣️.json"
Y0, Y1 = 2.14, 2.66
POST_L = [[1.205, -0.075], [1.25, -0.075], [1.25, -0.045], [1.2475, -0.045], [1.2475, -0.0725], [1.205, -0.0725]]
POST_R = [[-1.205, -0.0725], [-1.2475, -0.0725], [-1.2475, -0.045], [-1.25, -0.045], [-1.25, -0.075], [-1.205, -0.075]]
PLATE_L = [[1.205, -0.075], [1.25, -0.075], [1.25, 0.13], [1.2475, 0.13], [1.2475, -0.0725], [1.205, -0.0725]]
PLATE_R = [[-1.205, -0.0725], [-1.2475, -0.0725], [-1.2475, 0.13], [-1.25, 0.13], [-1.25, -0.075], [-1.205, -0.075]]


def solid(i, profile, a, b, tone):
    return {"id": i, "kind": "extrude", "bone": "track", "material": "steel", "role": "body", "tone": tone, "profile": profile, "plane": "zx", "from": a, "to": b}


def add(lst, item, after):
    if any(x["id"] == item["id"] for x in lst):
        return
    k = next((n for n, x in enumerate(lst) if x["id"] == after), len(lst) - 1)
    lst.insert(k + 1, item)


doc = json.load(open(SRC))
for s, v in doc["volumes"].items():
    if s == "500":
        for key in ("body", "thermal", "structure"):
            v[key] = [x for x in v[key] if "bearing-plate" not in x["id"] and "bearing-post" not in x["id"]]
        new = [solid("bearing-post-l", PLATE_L, Y0, 2.84, 0.38), solid("bearing-post-r", PLATE_R, Y0, 2.84, 0.38)]
    else:
        new = [solid("bearing-post-l", POST_L, Y0, Y1, 0.35), solid("bearing-post-r", POST_R, Y0, Y1, 0.35)]
    for n in new:
        side = n["id"][-1]
        add(v["body"], n, "flag-bracket-" + side)
        add(v["thermal"], {"id": "th-" + n["id"], "of": n["id"], "region": "track-steel", "layer": "track-wall"}, "th-flag-bracket-" + side)
        add(v["structure"], {"id": "m-" + n["id"], "of": n["id"], "member": "track", "part": "member"}, "m-flag-bracket-" + side)
    note = {"en": "The end bearing plates of the spring line stand on wall angles 45 x 30 x 2.5 mm that continue the jamb fixing from the flag brackets upwards, so the shaft load has a drawn path into the jamb and nothing of the balance line hangs free.", "de": "Die Endlagerplatten der Federlinie stehen auf Wandwinkeln 45 x 30 x 2,5 mm, die die Zargenbefestigung von den Bogenwinkeln nach oben fortsetzen; die Wellenlast hat so einen gezeichneten Weg in die Zarge und nichts an der Ausgleichslinie hängt frei."}
    if not any(d["en"] == note["en"] for d in v["decisions"]):
        v["decisions"].append(note)
json.dump(doc, open(SRC, "w"), ensure_ascii=False, indent=1)

if "--roots" in sys.argv:
    a = L.load_asset("sectional-garage-door")
    for s, rep in a["representations"].items():
        g = rep.get("geometry")
        if not g:
            continue
        ref = next(x for x in g["elevation"] if x["id"] == "flag-bracket")
        e = dict(ref, id="bearing-post", x=-0.075, y=Y0, w=0.03, h=Y1 - Y0)
        if not any(x["id"] == "bearing-post" for x in g["elevation"]):
            g["elevation"].insert(g["elevation"].index(ref) + 1, e)
        fl = next((x for x in g["front"] if x["id"] == "flag-bracket-left"), None)
        if fl and not any(x["id"] == "bearing-post-left" for x in g["front"]):
            k = g["front"].index(fl)
            g["front"].insert(k + 1, dict(fl, id="bearing-post-left", x=1.205, y=Y0, w=0.045, h=Y1 - Y0))
            g["front"].insert(k + 2, dict(fl, id="bearing-post-right", x=-1.25, y=Y0, w=0.045, h=Y1 - Y0))
    L.write_members("sectional-garage-door", {"representations": a["representations"]})
