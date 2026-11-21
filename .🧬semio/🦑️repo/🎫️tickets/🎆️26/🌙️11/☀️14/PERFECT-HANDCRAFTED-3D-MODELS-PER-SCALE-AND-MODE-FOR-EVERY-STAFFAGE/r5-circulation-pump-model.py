"""🌀 Round 5 circulation pump: keeps the round-3 hand-stated lists of 1:50, 1:100 and 1:200 (`r3-circulation-pump-model.py`, re-read and graded in round 5) and authors the digital 1:500 anew. Round 5 found the inherited 1:500 a near copy of 1:200 drawn with an octagonal volute, an octagonal motor and a hexagonal hollow pipe; now the volute, the motor and the pipe are round 16- and 12-sided prisms (fewer triangles than the revolves of 1:200, but round to the eye), the pipe is solid (its 26 mm bore is below the 75 mm feature, C30) and the motor is one solid drum carrying the rotor. Writes `volumes/appliances/circulation-pump.json`.

Usage: python3 r5-circulation-pump-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _module(name, file):
    """📦 Imports a sibling script as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


R3 = _module("r3pump", "r3-circulation-pump-model.py")
TEXTS = os.path.join(HERE, "r5-circulation-pump-texts.json")
IRON, ALU, PA, GLASS, BRASS = R3.IRON, R3.ALU, R3.PA, R3.GLASS, R3.BRASS


def ngon(cx, cy, r, n):
    """⬣ A regular n-gon of circumradius r about (cx, cy), one corner on +x."""
    return [[round(cx + r * math.cos(2 * math.pi * i / n), 6), round(cy + r * math.sin(2 * math.pi * i / n), 6)] for i in range(n)]


def body500():
    """🎨 1:500 body: the volute with its flange as one round 96 mm drum, the motor head as one round 84 mm drum on the shaft, the pipe run with necks, nuts and tails as one solid 44 mm bar, the terminal box as a block."""
    return [
        R3.solid("casing", "extrude", "housing", IRON, tone=0.42, plane="xy", **{"from": 0.008, "to": 0.067}, profile=ngon(0.1055, -0.003, 0.048, 16)),
        R3.solid("motor-head", "extrude", "motor", ALU, tone=0.62, plane="xy", **{"from": -0.067, "to": 0.008}, profile=ngon(0.0, 0.0, 0.042, 16)),
        R3.solid("pipe", "extrude", "housing", BRASS, tone=0.6, plane="yz", **{"from": -0.006, "to": 0.214}, profile=[[round(p[1], 6), round(p[0], 6)] for p in ngon(0.035, 0.0, 0.022, 12)], sectionFaces=["-u", "+u"]),
        R3.solid("terminal-box", "box", "terminal", PA, tone=0.78, size=[0.045, 0.062, 0.064], cornerRadius=0.0, transform={"translation": [0.0125, 0.0, -0.026]}),
    ]


def thermal500():
    """🌡️ 1:500 thermal: casing, pipe, motor and terminal box as their body surfaces, the sight glass as a flush 42 mm disc on the motor front, the motor plume outlet as a skin on the lid."""
    return [
        {"id": "th-casing", "of": "casing", "region": "casing", "layer": "shell"},
        {"id": "th-pipe", "of": "pipe", "region": "casing", "layer": "shell", "sectionFaces": ["-u", "+u"]},
        {"id": "th-motor-head", "of": "motor-head", "region": "motor", "layer": "head"},
        {"id": "th-terminal-box", "of": "terminal-box", "region": "terminal-box", "layer": "box"},
        R3.solid("th-rotor-window", "revolve", "motor", GLASS, role="glazing", tone=0.18, region="rotor-window", layer="pane", sectionFaces=["+w"], origin=[0.0, 0.0, 0.0], axis=[0, 0, 1], profile=[[-0.0676, 0.0], [-0.0676, 0.021], [-0.0668, 0.021], [-0.0668, 0.0]]),
        R3.solid("th-motor-plume", "box", "terminal", PA, tone=0.8, size=[0.001, 0.062, 0.064], cornerRadius=0.0004, transform={"translation": [0.0353, 0.0, -0.026]}, region="motor", layer="head", tags=["motor-plume"]),
    ]


def structure500():
    """🏗️ 1:500 structure: the housing as one 30 mm beam between the two pipe supports, the motor as one solid 84 mm drum on the housing (stator, can and flange merged; its cavity is below the feature), the rotor shaft inside it, the terminal floor plate, the mass at the common centre, the clamp collar (pin) and the cross pin (roller)."""
    return [
        R3.solid("member-housing", "revolve", "housing", IRON, role="detail", tone=0.25, profile=[[0.0, 0.0], [0.0, 0.015], [0.208, 0.015], [0.208, 0.0]], axis=[1, 0, 0], origin=[0.0, 0.0, 0.035], member="housing", part="member"),
        R3.solid("member-motor", "extrude", "motor", ALU, role="detail", tone=0.25, plane="xy", **{"from": -0.06, "to": 0.008}, profile=ngon(0.0, 0.0, 0.042, 16), member="motor", part="member"),
        R3.solid("member-rotor", "extrude", "rotor", IRON, role="detail", tone=0.25, plane="xy", **{"from": -0.064, "to": 0.034}, profile=ngon(0.0, 0.0, 0.012, 8), member="rotor", part="member"),
        R3.solid("member-terminal", "box", "terminal", PA, role="detail", tone=0.25, size=[0.039, 0.045, 0.008], cornerRadius=0, transform={"translation": [0.013, 0.0, -0.027]}, member="terminal", part="member"),
        R3.solid("mass-pump", "ellipsoid", "housing", IRON, role="detail", tone=0.15, radii=[0.022, 0.022, 0.022], transform={"translation": [0.104, 0.005, 0.0067]}, member="housing", part="mass"),
        R3.solid("support-pipe-left", "revolve", "housing", IRON, role="detail", tone=0.25, profile=[[0.089, 0.015], [0.089, 0.03], [0.119, 0.03], [0.119, 0.015], [0.089, 0.015]], axis=[-1, 0, 0], origin=[0.104, 0.0, 0.035], member="housing", part="support", tags=["pipe-left"], anchor="pipe-left"),
        R3.solid("support-pipe-right", "revolve", "housing", IRON, role="detail", tone=0.25, profile=[[-0.02, 0.0], [-0.02, 0.015], [0.02, 0.015], [0.02, 0.0]], axis=[0, 0, 1], origin=[0.208, 0.0, 0.035], member="housing", part="support", tags=["pipe-right"], anchor="pipe-right"),
    ]


def main():
    """💾 Writes the volumes source."""
    texts = json.load(open(TEXTS, encoding="utf-8"))
    vols = {"50": R3.scale50(), "100": R3.scale100(), "200": R3.scale200(), "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": texts["500"]}}
    for s, v in vols.items():
        for lst in ("body", "thermal", "structure"):
            ids = [x["id"] for x in v[lst]]
            if len(ids) != len(set(ids)):
                raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(R3.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", R3.SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


if __name__ == "__main__":
    main()
