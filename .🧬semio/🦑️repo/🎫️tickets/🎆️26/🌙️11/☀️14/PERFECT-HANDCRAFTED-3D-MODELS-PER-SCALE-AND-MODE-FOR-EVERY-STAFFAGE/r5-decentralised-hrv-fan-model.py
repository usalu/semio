"""🌀 Round 5 decentralised HRV fan: keeps the round-3 hand-stated lists of 1:50 and 1:100 (`r3-decentralised-hrv-fan-volumes.py`, re-read and graded in round 5); at 1:200 seats the ceramic disc 1 mm into the tube wall (its surface lay exactly on the bore and flickered as stripes in every cut); authors the digital 1:500 anew: the inherited one drew the base tray as a 4 mm sheet with the blades as 5 mm plates and the same coincident core. Writes `volumes/appliances/decentralised-hrv-fan.json`.

Usage: python3 r5-decentralised-hrv-fan-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
TEXTS = os.path.join(HERE, "r5-decentralised-hrv-fan-texts.json")


def _module(name, file):
    """📦 Imports a sibling script as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


R3 = _module("r3hrv", "r3-decentralised-hrv-fan-volumes.py")
RC = 0.083
ZAX = [0, 0, 1]


def solid(sid, kind, bone, material, tone, role="body", **kw):
    """🧱 One solid with the common members first."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def disc(sid, bone, material, tone, a, b, r, **kw):
    """💿 A solid disc on the unit axis from w a to w b of radius r."""
    return solid(sid, "revolve", bone, material, tone, profile=[[a, 0.0], [a, r], [b, r], [b, 0.0]], axis=ZAX, origin=[0.0, 0.0, 0.0], **kw)


def seat(s):
    """🪨 Moves a core or outlet disc radius from the bore (0.082) to 1 mm inside the tube wall."""
    if s.get("bone") == "core" and s.get("kind") == "revolve":
        s = dict(s, profile=[[p[0], RC if abs(p[1] - 0.082) < 1e-9 or abs(p[1] - 0.08) < 1e-9 else p[1]] for p in s["profile"]])
    return s


def blade(sid, k):
    """🪭 One of the three 1:500 blades: a radial plan form from the 80 mm hub to the 172 mm tip circle, 15 mm thick (the 1:500 blade stands for the pitched blade's projected depth)."""
    a = math.radians(90 + 120 * k)
    pts = []
    for r, half in ((0.04, 0.012), (0.085, 0.022)):
        for s in (-1, 1):
            t = a + s * half / r
            pts.append([round(r * math.cos(t), 6), round(r * math.sin(t), 6)])
    pts = [pts[0], pts[2], pts[3], pts[1]]
    return solid(sid, "extrude", "rotor", "abs-plastic", 0.82, plane="xy", **{"from": -0.1, "to": -0.085}, profile=pts)


def plate500():
    """⬜ The base tray at 1:500: 260 x 260 x 20 mm with 20 mm corner radius and the 164 mm bore (the rim, bosses and terminal box are below the feature)."""
    out = []
    for cx, cy, a0 in ((0.11, -0.11, -90), (0.11, 0.11, 0), (-0.11, 0.11, 90), (-0.11, -0.11, 180)):
        for i in range(4):
            t = math.radians(a0 + 90 * i / 3)
            out.append([round(cx + 0.02 * math.cos(t), 6), round(cy + 0.02 * math.sin(t), 6)])
    hole = [[round(0.082 * math.cos(2 * math.pi * i / 24), 6), round(0.082 * math.sin(2 * math.pi * i / 24), 6)] for i in range(24)][::-1]
    return solid("plate", "extrude", "plate", "abs-plastic", 0.88, plane="xy", **{"from": -0.12, "to": -0.1}, profile=out, holes=[hole])


def body500():
    """🎨 1:500 body (75 mm feature): the 20 mm base tray with its bore, one tube that merges fan housing, sleeve and collar (196 / 164 mm), the ceramic as a plain disc seated in the tube, the upright bridge bar behind the hub, three broad blades and the 80 mm hub."""
    return [
        plate500(),
        solid("ring", "revolve", "ring", "abs-plastic", 0.7, profile=[[-0.1, 0.082], [0.12, 0.082], [0.12, 0.098], [-0.1, 0.098], [-0.1, 0.082]], axis=ZAX, origin=[0.0, 0.0, 0.0]),
        disc("core", "core", "ceramic-honeycomb", 0.62, -0.04, 0.06, RC),
        solid("strut", "box", "ring", "abs-plastic", 0.6, size=[0.03, 0.166, 0.03], transform={"translation": [0, 0, -0.055]}),
        blade("blade-1", 0), blade("blade-2", 1), blade("blade-3", 2),
        disc("hub", "rotor", "motor-housing", 0.55, -0.108, -0.07, 0.04),
    ]


def thermal500():
    """🌡️ 1:500 thermal: every body solid in its single layer (tray, tube and bar as casing, the ceramic as core, the blades, the hub) and the supply and exhaust outlet patches as 5 mm skins on the two faces of the ceramic."""
    refs = [("th-plate", "plate", "casing", "shell"), ("th-ring", "ring", "casing", "shell"), ("th-strut", "strut", "casing", "shell"), ("th-core", "core", "core", "wall"),
            ("th-blade-1", "blade-1", "blades", "blade"), ("th-blade-2", "blade-2", "blades", "blade"), ("th-blade-3", "blade-3", "blades", "blade"), ("th-hub", "hub", "hub", "can")]
    out = [{"id": i, "of": o, "region": r, "layer": l} for i, o, r, l in refs]
    out.append(disc("th-supply-in", "core", "ceramic-honeycomb", 0.62, -0.044, -0.039, RC, region="core", layer="wall", tags=["supply-in"]))
    out.append(disc("th-exhaust-out", "core", "ceramic-honeycomb", 0.62, 0.059, 0.064, RC, region="core", layer="wall", tags=["exhaust-out"]))
    return out


def structure500():
    """🏗️ 1:500 structure: tray, tube, bridge bar, ceramic and hub are the members themselves (references), one mass of the whole unit, the upper-left screw as a pin into the wall plug and the lower-right one as a roller on the wall face."""
    return [
        {"id": "member-plate", "of": "plate", "member": "plate", "part": "member"},
        {"id": "member-ring", "of": "ring", "member": "ring", "part": "member"},
        {"id": "member-strut", "of": "strut", "member": "ring", "part": "member"},
        {"id": "member-core", "of": "core", "member": "core", "part": "member"},
        {"id": "member-hub", "of": "hub", "member": "rotor", "part": "member"},
        solid("mass-unit", "ellipsoid", "core", "abs-plastic", 0.15, role="detail", radii=[0.02, 0.02, 0.02], transform={"translation": [0, 0, -0.009]}, member="core", part="mass"),
        solid("support-wall-screw-top", "revolve", "plate", "motor-housing", 0.25, role="detail", profile=[[-0.125, 0.0], [-0.125, 0.015], [-0.095, 0.015], [-0.095, 0.0]], axis=ZAX, origin=[-0.108, 0.108, 0.0], member="plate", part="support", anchor="wall-screw-top"),
        solid("support-wall-screw-bottom", "revolve", "plate", "motor-housing", 0.25, role="detail", profile=[[0.0, 0.0], [0.0, 0.015], [0.03, 0.015], [0.03, 0.0]], axis=[1, 0, 0], origin=[0.093, -0.108, -0.105], member="plate", part="support", anchor="wall-screw-bottom"),
    ]


def main():
    """💾 Writes the volumes source."""
    texts = json.load(open(TEXTS, encoding="utf-8"))
    s200 = R3.scale200()
    for lst in ("body", "thermal"):
        s200[lst] = [seat(s) for s in s200[lst]]
    vols = {"50": R3.scale50(), "100": R3.scale100(), "200": s200, "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": texts["500"]}}
    for s, v in vols.items():
        for lst in ("body", "thermal", "structure"):
            ids = [x["id"] for x in v[lst]]
            if len(ids) != len(set(ids)):
                raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(R3.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", R3.SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


if __name__ == "__main__":
    main()
