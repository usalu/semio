"""🌬️ Round 5, appliances/ceiling-diffuser: 1:200 and 1:500 authored anew (open spigot tube, flange plate, frustum envelope; emitters as outlet patches lying on the cone face under the outer slot; structure with the corner posts that carry the cone pack). Rewrites only `volumes.200` and `volumes.500` of `volumes/appliances/ceiling-diffuser.json`; the helpers are the round-3 ones (definitions only, the round-3 writer is not run).

Usage: python3 r5-ceiling-diffuser-coarse.py
"""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
OUT = T / "volumes" / "appliances" / "ceiling-diffuser.json"
_src = (T / "r3-ceiling-diffuser-model.py").read_text(encoding="utf-8")
H = {"__file__": str(T / "r3-ceiling-diffuser-model.py")}
exec(compile(_src[: _src.index("\nvolumes = {")], "r3-ceiling-diffuser-model.py", "exec"), H)


def patch(id, side, x, y, along, normal, drop, length=0.40):
    """💨 An outlet patch lying on the cone face under the outer slot, half sunk into the sheet (D2)."""
    s = H["emitter"](id, side * x, y, along, normal, length)
    a = side * math.atan2(drop, 0.127) / 2
    s["transform"]["rotation"] = [0, 0, H["r6"](math.sin(a)), H["r6"](math.cos(a))]
    return s


def posts(size, ylo, yhi, d):
    """🧱 The four corner posts (merged spacers) that carry the cone pack from the plate."""
    s = H["base"]("member-posts", "powder-coated-steel", "detail", 0.6)
    s.update({"kind": "box", "size": [size, H["r6"](yhi - ylo), size], "transform": {"translation": [d, H["r6"]((ylo + yhi) / 2), d]},
              "repeat": {"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": H["r6"](math.pi / 2), "origin": [0, 0, 0]}, "member": "body", "part": "member"})
    return s


def mass(r, ry, y):
    """⚫ The mass of the whole diffuser at its centre of mass."""
    s = H["base"]("mass-body", "powder-coated-steel", "detail", 0.15)
    s.update({"kind": "ellipsoid", "radii": [r, ry, r], "transform": {"translation": [0, y, 0]}, "member": "body", "part": "mass"})
    return s


def sheet(id, half, y0, y1, hole):
    """🟥 A thin thermal sheet of the plate region at its real 0.8 mm (C20): the plenum back at the ceiling or the discharge baffle in the outer slot."""
    s = H["base"](id, "powder-coated-steel", "body", 0.88)
    s.update({"kind": "extrude", "plane": "zx", "from": y0, "to": y1, "profile": [[-half, -half], [half, -half], [half, half], [-half, half]], "region": "plate", "layer": "sheet"})
    if hole:
        s["holes"] = [hole]
    return s


def scale200():
    """🔎 1:200: open spigot tube, the 30 mm flange plate, one cone envelope, outlet patches on the cone face, four corner posts."""
    text, ref, support = H["text"], H["ref"], H["support"]
    body = [
        H["revolve"]("collar", H["collar_tube"](0.1245, 0.0007, False), "galvanised-steel", "body", 0.5),
        H["plate"]("flange", 0.2975, 0.006, H["circle"](0.1245), -0.03, 0.0, faces=["+v"]),
        H["loft"]("cone", [(-0.07, 0.135, 12), (-0.03, 0.262, 12)], 0.72),
    ]
    frame = H["plate"]("th-flange", 0.2975, 0.006, H["circle"](0.1245, n=32), -0.012, 0.0, faces=["+v"])
    frame.update({"region": "plate", "layer": "sheet"})
    thermal = [frame, sheet("th-plenum-back", 0.287, 0.0, 0.0008, list(reversed(H["circle"](0.1245, n=32)))),
               sheet("th-discharge-baffle", 0.235, -0.011, -0.0102, [[-0.135, -0.135], [-0.135, 0.135], [0.135, 0.135], [0.135, -0.135]]),
               ref("th-cone", "cone", region="plate", layer="sheet"), ref("th-collar", "collar", region="collar", layer="tube"),
               patch("jet-left", -1, 0.236, -0.0385, 0.03, 0.03, 0.040), patch("jet-right", 1, 0.236, -0.0385, 0.03, 0.03, 0.040)]
    structure = [ref("member-flange", "flange", member="body", part="member"), posts(0.03, -0.06, -0.03, 0.15), mass(0.06, 0.015, -0.02),
                 support("ceiling-hanger-left", "pin", -0.28, 0.03, -0.03, 0.03), support("ceiling-hanger-right", "roller", 0.28, 0.03, 0, 0)]
    decisions = [
        text("Own model for 1:200: a 30 mm flange plate (border, back plate and slots at the 30 mm feature) with the spigot hole, one cone mass that generalises the three steps to their envelope from 0.262 m half width under the flange to the 0.27 m square centre cone 70 mm below the ceiling, and the spigot as its open 0.7 mm tube (C30: the 0.25 m bore is far above the feature).",
             "Eigenes Modell für 1:200: eine Flanschplatte von 30 mm (Rahmen, Rückplatte und Schlitze auf dem 30-mm-Merkmal) mit dem Stutzenloch, eine Kegelmasse, die die drei Stufen zu ihrer Hülle verallgemeinert, von 0,262 m Halbbreite unter dem Flansch bis zum 0,27 m breiten Mittelkegel 70 mm unter der Decke, und der Stutzen als sein offenes Rohr von 0,7 mm (C30: die Bohrung von 0,25 m liegt weit über dem Merkmal)."),
        text("Thermal: the sheets keep their real 0.8 mm (C20): the 12 mm border frame, the plenum back at the ceiling and the discharge baffle in the outer slot between frame and cone, with references to the cone and the spigot; the jets are 30 mm outlet patches lying on the cone face under the outer slot, half sunk into the sheet. Structure: the flange plate carries the load to the pin and the roller (30 mm); the spacers merge into four 30 mm corner posts that carry the cone pack; the joint merges into the plate, one mass sits at the centre of mass.",
             "Thermik: die Bleche behalten ihre echten 0,8 mm (C20): der Randrahmen von 12 mm, die Plenumrückwand an der Decke und das Leitblech im äußeren Schlitz zwischen Rahmen und Kegel, dazu Verweise auf Kegel und Stutzen; die Strahlen sind 30-mm-Austrittsflächen, die unter dem äußeren Schlitz auf der Kegelfläche liegen, halb ins Blech gesenkt. Tragwerk: die Flanschplatte trägt die Last zum Festlager und zur Rolle (30 mm); die Abstandshalter werden zu vier Eckstützen von 30 mm, die das Kegelpaket tragen; das Gelenk geht in der Platte auf, eine Masse liegt im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    """🔎 1:500: open spigot tube, a 12 mm plate with sharp corners, the cone as one tapered frustum, outlet patches, posts, mass and the two hangers."""
    text, ref, support = H["text"], H["ref"], H["support"]
    cone = H["base"]("cone", "powder-coated-steel", "body", 0.72)
    cone.update({"kind": "extrude", "plane": "zx", "from": -0.07, "to": -0.012, "profile": [[-0.135, -0.135], [0.135, -0.135], [0.135, 0.135], [-0.135, 0.135]], "taperTo": [1, 1.94]})
    flange = H["base"]("flange", "powder-coated-steel", "body", 0.88)
    flange.update({"kind": "extrude", "plane": "zx", "from": -0.012, "to": 0.0, "profile": [[-0.2975, -0.2975], [0.2975, -0.2975], [0.2975, 0.2975], [-0.2975, 0.2975]],
                   "holes": [list(reversed(H["circle"](0.1245, n=32)))], "sectionFaces": ["+v"]})
    body = [H["revolve"]("collar", [(0.0, 0.1245), (0.05, 0.1245), (0.05, 0.1225), (0.0, 0.1225), (0.0, 0.1245)], "galvanised-steel", "body", 0.5), flange, cone]
    shell = dict(cone, id="th-cone", holes=[[[-0.131, -0.131], [-0.131, 0.131], [0.131, 0.131], [0.131, -0.131]]], region="plate", layer="sheet")
    thermal = [ref("th-flange", "flange", region="plate", layer="sheet"), sheet("th-plenum-back", 0.287, 0.0, 0.0008, list(reversed(H["circle"](0.1245, n=32)))),
               shell, sheet("th-centre-dish", 0.131, -0.07, -0.0692, None), ref("th-collar", "collar", region="collar", layer="tube"),
               patch("jet-left", -1, 0.24, -0.022, 0.04, 0.012, 0.058), patch("jet-right", 1, 0.24, -0.022, 0.04, 0.012, 0.058)]
    structure = [ref("member-flange", "flange", member="body", part="member"), posts(0.02, -0.04, -0.012, 0.12), mass(0.06, 0.012, -0.02),
                 support("ceiling-hanger-left", "pin", -0.28, 0.02, -0.012, 0.02), support("ceiling-hanger-right", "roller", 0.28, 0.02, 0, 0)]
    decisions = [
        text("Own model for 1:500, a crisp massing: the 0.595 m square plate 12 mm thick with sharp corners and the spigot hole, the frustums as one square frustum from 0.262 m half width under the plate to the 0.27 m square face 70 mm below the ceiling, and the spigot as its open tube.",
             "Eigenes Modell für 1:500, eine klare Massenform: die quadratische Platte von 0,595 m, 12 mm stark mit scharfen Ecken und dem Stutzenloch, die gestuften Kegel als ein quadratischer Kegelstumpf von 0,262 m Halbbreite unter der Platte bis zur 0,27 m breiten Front 70 mm unter der Decke, und der Stutzen als sein offenes Rohr."),
        text("Thermal: references to plate and spigot, the 0.8 mm plenum back at the ceiling, the frustum as its open pressed shell with the centre dish (the plenum under the spigot reaches its inside, as in reality), and the two outlet patches on the frustum face under the outer slot. Structure: the plate carries the load to the two hangers, four corner posts carry the cone pack, one mass at the centre of mass; the joint merges into the plate.",
             "Thermik: Verweise auf Platte und Stutzen, die Plenumrückwand von 0,8 mm an der Decke, der Kegelstumpf als offene gepresste Schale mit der Mittelschale (das Plenum unter dem Stutzen erreicht ihr Inneres wie in Wirklichkeit) und die zwei Austrittsflächen auf der Stumpffläche unter dem äußeren Schlitz. Tragwerk: die Platte trägt die Last zu den zwei Haltern, vier Eckstützen tragen das Kegelpaket, eine Masse im Schwerpunkt; das Gelenk geht in der Platte auf."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


doc = json.loads(OUT.read_text(encoding="utf-8"))
doc["volumes"]["200"] = scale200()
doc["volumes"]["500"] = scale500()
OUT.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("written 1:200 and 1:500", OUT)
