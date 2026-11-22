"""📐 Tower server, second round pass 1, stage 2: the side cover as its own drawn part (elevation `solid` shapes on bone `cover`), mainboard and drive cage at 1:100, decisions. Writes the candidate document from catalog and source."""
import json, os
T = os.path.dirname(os.path.abspath(__file__))
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
SRC = T + "/volumes/appliances/tower-server.json"
OUT = T + "/🗑️generated/r3-tower-server/🔣️.json"
Z = float(os.environ.get("COVER_Z_SHIFT", "0"))
doc = json.load(open(A)); a = doc["asset"]
V = json.load(open(SRC))["volumes"]; a["volumes"] = V
r = lambda v: round(v, 5)

def panel(u0, u1, v0, v1): return "M %g %g L %g %g L %g %g L %g %g Z" % (u0, v0, u1, v0, u1, v1, u0, v1)
def on_cover(shape, z0, z1):
    s = {k: v for k, v in shape.items() if k not in ("kind", "x", "y", "w", "h", "r")}
    s.update({"bone": "cover", "kind": "path", "d": panel(r(0.213 - shape["y"] - shape["h"]), r(0.213 - shape["y"]), r(shape["x"] - 0.012), r(shape["x"] + shape["w"] - 0.012)), "solid": {"kind": "extruded", "sweep": "w", "z0": z0 + Z, "z1": z1 + Z}})
    return s

COVER = {"50": (-0.2, 0.213, 0.44, -0.1, -0.09), "100": (-0.2, 0.21, 0.45, -0.1, -0.085), "200": (-0.195, 0.205, 0.43, -0.1, -0.07)}
for sc, (yf, yr, top, z0, z1) in COVER.items():
    rep = a["representations"][sc]; el = rep["geometry"]["elevation"]
    if any(s["id"] == "cover" for s in el): raise SystemExit("already applied")
    side = next(s for s in el if s["id"] in ("case-side", "case"))
    new = [on_cover({"id": "cover", "bone": "case", "role": "body", "z": side["z"] + 0.05, "filled": True, "tone": side["tone"], "x": 0.012, "y": yf, "w": top - 0.012, "h": yr - yf, "region": "side-panel", "material": "chassis-steel"}, z0, z1)]
    for i, s in enumerate(el):
        if s["id"] in ("cpu-grille", "latch"): el[i] = on_cover(s, z0, z0 + 0.0008)
    el[el.index(side) + 1:el.index(side) + 1] = new
    for view in ("front", "plan"):
        rep["geometry"][view].insert(0, dict(new[0], z=2.95))
    rep["decisions"]["retained"]["en"] += " The side cover is drawn as a part of its own on its hinge bone, so the service pose shows it tilted out on its bottom edge."
    rep["decisions"]["retained"]["de"] += " Der Seitendeckel ist als eigenes Teil an seinem Scharnierknochen gezeichnet, sodass die Wartungspose ihn um die Unterkante nach außen gekippt zeigt."

b1 = V["100"]["body"]
if not any(s["id"] == "drive-cage" for s in b1):
    b1 += [
        {"id": "drive-cage", "kind": "box", "bone": "case", "material": "chassis-steel", "role": "body", "tone": 0.4, "size": [0.073, 0.145, 0.108], "transform": {"translation": [0.0635, -0.1125, 0.0]}},
    ]

BODY = {
    "50": ("The side cover is a part of its own on the bone `cover`: released by its latch it tilts out on its bottom edge (pose `service-open`) and shows the drive cage with two disk drives, the memory modules and the mainboard on its nine standoffs; the heat pipes run through a 68 mm base plate and the power supply stands on the floor of its shell.", "Der Seitendeckel ist ein eigenes Teil am Knochen `cover`: von der Verriegelung gelöst kippt er um seine Unterkante nach außen (Pose `service-open`) und zeigt den Laufwerkskäfig mit zwei Festplatten, die Speichermodule und das Mainboard auf seinen neun Abstandsbolzen; die Wärmerohre laufen durch eine 68 mm breite Bodenplatte und das Netzteil steht auf dem Boden seiner Hülle."),
    "100": ("The side cover hangs on the bone `cover` and tilts out on its bottom edge; behind it stand the cooler on the wall and the drive cage with its disks as one block; the 1.6 mm mainboard is no part at 15 mm, and the fan hub reaches its guard.", "Der Seitendeckel hängt am Knochen `cover` und kippt um seine Unterkante nach außen; dahinter stehen der Kühler an der Wand und der Laufwerkskäfig mit seinen Platten als ein Block; das 1,6 mm starke Mainboard ist bei 15 mm kein Teil, und die Lüfternabe reicht bis an ihr Schutzgitter."),
    "200": ("The side cover is a 30 mm plate on the bone `cover` with the grille opening and the latch recess; the drive face fills the bay and the fan hub stands in the fan opening.", "Der Seitendeckel ist eine 30 mm starke Platte am Knochen `cover` mit Gitteröffnung und Griffmulde; die Laufwerksfront füllt den Schacht und die Lüfternabe steht in der Lüfteröffnung."),
}
B500 = ("Body: at 75 mm the server is its case on the plinth of the four feet: the case block, the bezel as a 75 mm front slab with the opening of the intake mesh, the rear 75 mm with the openings of power supply and fan, a dark core that shows through these openings, and as 0.8 mm skins the top with the intake field of the power supply and the hinged side cover with the grille field over the processor cooler. The bezel is three times as deep as the real 25 mm because no thinner slab is a part at this scale.", "Körper: bei 75 mm ist der Server sein Gehäuse auf dem Sockel der vier Füße: der Gehäuseblock, die Blende als 75 mm tiefe Frontscheibe mit der Öffnung des Ansauggitters, die hinteren 75 mm mit den Öffnungen von Netzteil und Lüfter, ein dunkler Kern, der durch diese Öffnungen sichtbar ist, und als 0,8 mm starke Häute das Deckblech mit dem Ansaugfeld des Netzteils und der klappbare Seitendeckel mit dem Gitterfeld über dem Prozessorkühler. Die Blende ist dreimal so tief wie die wirklichen 25 mm, weil in diesem Maßstab keine dünnere Scheibe ein Teil ist.")
STRUCT = ("The side cover is a shear panel of its own, member `cover`, lying against the posts of the cover side; it carries no vertical load.", "Der Seitendeckel ist als Schubfeld ein eigener Stab `cover`, der an den Pfosten der Deckelseite anliegt; er trägt keine Vertikallast.")
for sc in ("50", "100", "200", "500"):
    d = V[sc]["decisions"]
    body = next(e for e in d if e["en"].startswith("Body")); st = next(e for e in d if e["en"].startswith("Structure"))
    if "`cover`" in st["en"]: continue
    if sc == "500": body["en"], body["de"] = B500
    else: body["en"] += " " + BODY[sc][0]; body["de"] += " " + BODY[sc][1]
    st["en"] += " " + STRUCT[0]; st["de"] += " " + STRUCT[1]
json.dump(doc, open(OUT, "w"), ensure_ascii=False, indent=2)
print("written", OUT)
