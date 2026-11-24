"""↩️ Withdraws the forced band differences of the first pass of batch MB-2 after the coordinator's ruling (D47: a view that is physically the same drawing in two bands is allowlisted, not redrawn): restores the copied views of `bicycle-stand`, `tealight`, `used-tv-sideboard` (plan 1:500), `used-shoe-cabinet` (plan 1:500), `heat-pump-outdoor-unit` (plan 1:500 casing) from the band they were equal to and removes the notes of those ops. Idempotent; run once, the ops of these changes are gone from `ops/`."""
import glob, json
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def edit(slug, change):
    """✍️ Applies a change to the asset of a catalog document and writes it back in its indentation."""
    path = glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")[0]
    text = open(path).read()
    second = text.split("\n", 2)[1]
    doc = json.loads(text)
    change(doc["asset"]["representations"])
    out = json.dumps(doc, ensure_ascii=False, indent=len(second) - len(second.lstrip())) + "\n"
    if out != text:
        open(path, "w").write(out)
    print(slug, "restored" if out != text else "unchanged")


def unnote(r, scale, decision, *starts):
    """🧽️ Removes appended sentences that begin with one of the given words."""
    for lang in ("en", "de"):
        text = r[scale]["decisions"][decision][lang]
        for start in starts:
            at = text.find(" " + start)
            if at >= 0:
                text = text[:at]
        r[scale]["decisions"][decision][lang] = text


def copy(r, view, source, target):
    """📋️ Makes the view of a band the copy of another band's."""
    r[target]["geometry"][view] = json.loads(json.dumps(r[source]["geometry"][view]))


def bicycle(r):
    for view in ("front", "plan"):
        copy(r, view, "50", "100")
        copy(r, view, "50", "200")
    unnote(r, "100", "merged", "Front and plan: the tube radius", "Vorderansicht und Draufsicht: der Rohrradius")
    unnote(r, "200", "merged", "Front and plan: leg, bends", "Vorderansicht und Draufsicht: Bein, Bögen")


def tealight(r):
    cup = next(s for s in r["50"]["geometry"]["plan"] if s["id"] == "cup")
    for scale in ("100", "200"):
        plan = r[scale]["geometry"]["plan"]
        top = next(s for s in plan if s["id"] == "wax-top")
        top.update(rx=0.0178, ry=0.0178, regions=["wax", "wick"])
        flame = next((s for s in plan if s["id"] == "flame"), None) or json.loads(json.dumps(next(s for s in r["100"]["geometry"]["plan"] if s["id"] == "flame")))
        r[scale]["geometry"]["plan"] = [json.loads(json.dumps(cup)), top, flame]
        unnote(r, scale, "merged", "Plan view: rim and wax are one disc", "Draufsicht: Rand und Wachs sind eine Scheibe")
    unnote(r, "200", "omitted", "Plan view: the flame disc", "Draufsicht: die Flammenscheibe")


def put(r, scale, view, source, sid, after):
    """📌️ Puts back a shape a band lost, as the copy of another band's, after a named shape."""
    shapes = r[scale]["geometry"][view]
    if any(s["id"] == sid for s in shapes):
        return
    twin = json.loads(json.dumps(next(s for s in r[source]["geometry"][view] if s["id"] == sid)))
    shapes.insert(next(i for i, s in enumerate(shapes) if s["id"] == after) + 1, twin)


def sideboard(r):
    put(r, "500", "plan", "200", "door-r", "door-l")
    unnote(r, "500", "omitted", "Plan: the right door", "Draufsicht: die rechte Tür")


def cabinet(r):
    put(r, "500", "plan", "200", "back", "blanket")
    next(s for s in r["500"]["geometry"]["plan"] if s["id"] == "frame")["points"] = [[0.003, 0.0], [0.154, 0.0], [0.154, 1.45], [0.003, 1.45]]
    unnote(r, "500", "merged", "Plan: back and carcass are one block", "Draufsicht: Rückwand und Korpus sind ein Block")


def heat_pump(r):
    casing = next(s for s in r["500"]["geometry"]["plan"] if s["id"] == "casing")
    casing.update(x=-0.215, y=-0.55, w=0.43, h=1.1)
    unnote(r, "500", "merged", "Plan: casing and cover share", "Draufsicht: Gehäuse und Deckel haben")


edit("bicycle-stand", bicycle)
edit("tealight", tealight)
edit("used-tv-sideboard", sideboard)
edit("used-shoe-cabinet", cabinet)
edit("heat-pump-outdoor-unit", heat_pump)
