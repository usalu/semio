"""Hand-designed front elevation of tilt-window (0.9 m wide, 1.25 m high; x from the centre, y from the floor-side sill).

python window_front.py <asset.json>

The blind frame (sill, jambs, head) is fixed. The sash members (stiles, rails, pane, spacer) carry followTip: the hinge at the bottom stays, the head of the
sash drops by 1.2 m * (1 - cos(tilt)) and opens a gap against the head of the frame.
"""
import json
import sys


def r5(v):
    return round(v, 5)


def rect(id, bone, role, z, filled, tone, x, y, w, h, region, material=None, tip=False, pen=None):
    shape = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "region": region}
    if material:
        shape["material"] = material
    if pen:
        shape["penMm"] = pen
    shape.update({"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)})
    if tip:
        shape["followTip"] = True
    return shape


def front(scale):
    sill = rect("sill", "sill", "body", 1, True, 0.6, -0.47, 0.0, 0.94, 0.05, "frame-sill", "timber")
    if scale == 200:
        return [
            sill,
            rect("frame-outline", "sill", "detail", 2, False, 0.5, -0.45, 0.05, 0.9, 1.2, "frame-timber"),
            rect("pane", "sash", "glazing", 3, True, 0.9, -0.4, 0.05, 0.8, 1.15, "pane-glass", tip=True),
            rect("sash-outline", "sash", "body", 4, False, 0.5, -0.4, 0.05, 0.8, 1.15, "frame-timber", tip=True),
        ]
    jamb = 0.05 if scale == 50 else 0.055
    stile = 0.045 if scale == 50 else 0.05
    shapes = [
        sill,
        rect("jamb-left", "sill", "detail", 2, True, 0.6, -0.45, 0.05, jamb, 1.15, "frame-timber", "timber"),
        rect("jamb-right", "sill", "detail", 2, True, 0.6, 0.45 - jamb, 0.05, jamb, 1.15, "frame-timber", "timber"),
        rect("head", "sill", "detail", 2, True, 0.6, -0.45, 1.2, 0.9, 0.05, "frame-timber", "timber"),
        rect("stile-left", "sash", "detail", 3, True, 0.6, -0.45 + jamb, 0.05, stile, 1.15, "frame-timber", "timber", tip=True),
        rect("stile-right", "sash", "detail", 3, True, 0.6, 0.45 - jamb - stile, 0.05, stile, 1.15, "frame-timber", "timber", tip=True),
        rect("rail-bottom", "sash", "detail", 3, True, 0.6, -0.45 + jamb + stile, 0.05, 0.9 - 2 * (jamb + stile), stile, "frame-timber", "timber", tip=True),
        rect("rail-top", "sash", "detail", 3, True, 0.6, -0.45 + jamb + stile, 1.2 - stile, 0.9 - 2 * (jamb + stile), stile, "frame-timber", "timber", tip=True),
        rect("pane", "sash", "glazing", 3.5, True, 0.9, -0.45 + jamb + stile, 0.05 + stile, 0.9 - 2 * (jamb + stile), 1.15 - 2 * stile, "pane-glass", "glass", tip=True),
    ]
    if scale == 50:
        inset = 0.012
        x0 = -0.45 + jamb + stile + inset
        shapes.append(rect("pane-spacer", "sash", "glazing", 4, False, 0.5, x0, 0.05 + stile + inset, 0.9 - 2 * (jamb + stile) - 2 * inset, 1.15 - 2 * stile - 2 * inset, "pane-edge", "edge-zone", tip=True))
    shapes.append({"id": "handle", "bone": "sash", "role": "detail", "z": 5, "filled": True, "tone": 0.2, "region": "frame-timber", "kind": "capsule", "x0": r5(0.45 - jamb - stile / 2), "y0": 0.55, "x1": r5(0.45 - jamb - stile / 2), "y1": 0.66, "r0": 0.01, "r1": 0.01})
    return shapes


def tx(en, de):
    return {"en": en, "de": de}


ADD = {
    "50": tx(" Front elevation: the blind frame (sill, two jambs, head) and the sash inside it (two stiles, two rails, the pane with its spacer line and the handle on the right stile); the sash is hinged at the bottom, so its head drops against the frame head by 1.2 m x (1 - cos of the tilt) and the gap above it shows the tilt (0.04 m at 15 degrees, 0.16 m at 30 degrees).", " Vorderansicht: der Blendrahmen (Sohlstück, zwei Pfosten, Kopf) und darin der Flügel (zwei Stiele, zwei Riegel, die Scheibe mit Abstandhalterlinie und der Griff am rechten Stiel); der Flügel ist unten angeschlagen, sein Kopf sinkt gegen den Rahmenkopf um 1,2 m x (1 - cos der Kippung), und der Spalt darüber zeigt die Kippung (0,04 m bei 15 Grad, 0,16 m bei 30 Grad)."),
    "100": tx(" Front elevation: blind frame, sash members, pane and handle; the head gap shows the tilt.", " Vorderansicht: Blendrahmen, Flügelhölzer, Scheibe und Griff; der Kopfspalt zeigt die Kippung."),
    "200": tx(" Front elevation: sill, frame outline, sash outline and pane; the head gap shows the tilt.", " Vorderansicht: Sohlstück, Rahmenkontur, Flügelkontur und Scheibe; der Kopfspalt zeigt die Kippung."),
}


def main():
    path = sys.argv[1]
    doc = json.load(open(path, encoding="utf-8"))
    asset = doc["asset"]
    for key in ("200", "100", "50"):
        rep = asset["representations"][key]
        rep["geometry"]["front"] = front(int(key))
        rep["decisions"]["silhouette"] = {lang: rep["decisions"]["silhouette"][lang] + ADD[key][lang] for lang in ("en", "de")}
    asset["presentation"]["viewpoints"] = ["elevation", "front"]
    json.dump(doc, open(path, "w", encoding="utf-8", newline="\n"), ensure_ascii=False, indent=1)


main()
