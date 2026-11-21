"""🔥 Round 5 gas boiler: keeps the round-3 hand-stated lists of 1:50 and 1:100 (`r3-gas-boiler-model.py`, re-read and graded in round 5), replaces the 1:200 thermal internals by their real forms and authors the digital 1:500 anew (crisp hood block, solid flue drum, round vessel lens), then writes `volumes/appliances/gas-boiler.json`.

Usage: python3 r5-gas-boiler-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _module(name, file):
    """📦 Imports a sibling script as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


R3 = _module("r3boiler", "r3-gas-boiler-model.py")
K = R3.K
SOURCE = R3.SOURCE
TEXTS = os.path.join(HERE, "r5-gas-boiler-texts.json")
PS, BR, PC, GL, BM, FP, GS, SS, AL = R3.PS, R3.BR, R3.PC, R3.GL, R3.BM, R3.FP, R3.GS, R3.SS, R3.AL
WX, WY = K.WIN
ZAX, YAX, XAX = R3.ZAX, R3.YAX, R3.XAX
LENS200 = [(0.065, 0.0), (0.07, 0.09), (0.085, 0.14), (0.105, 0.158), (0.125, 0.14), (0.14, 0.09), (0.145, 0.0)]
LENS500 = [(0.066, 0.0), (0.075, 0.12), (0.105, 0.158), (0.135, 0.12), (0.144, 0.0)]


def thermal200():
    """🌡️ 1:200 thermal: the round-3 list with the expansion vessel as its real 0.32 m lens (a revolve, no longer a square slab) and the premix fan as its drum."""
    out = []
    for s in R3.thermal200():
        if s["id"] == "th-vessel":
            s = K.th(K.rev("th-vessel", "casing", PS, "body", 0.3, LENS200, (0.0, 0.40, 0), ZAX), "expansion-vessel", "shell")
        elif s["id"] == "th-fan":
            s = K.th(K.rev("th-fan", "casing", AL, "body", 0.6, [(0.0, 0.0), (0.0, 0.046), (0.07, 0.046), (0.07, 0.0)], (-0.205, 0.60, -0.05), XAX), "fan", "housing")
        out.append(s)
    return out


def body200():
    """🎨 1:200 body: the round-3 list with the control band as a 30 mm strip that stands 15 mm proud of the hood face and runs 15 mm into it, as the 1:200 front and plan now draw it (printable to 1:350)."""
    return [K.ext("panel", "casing", PC, "body", 0.35, "xy", K.rrect(-0.2, 0.065, 0.2, 0.21, 0.0), -0.165, -0.135) if s["id"] == "panel" else s for s in R3.body200()]


def structure200():
    """🏗️ 1:200 structure: the round-3 load path with the sections generalised to the 30 mm feature: the chassis as a channel 30 mm deep at the wall, the rail and the rest strip as 30 mm bars at their anchors."""
    t = 0.0015
    xo = K.XW - 0.0015
    zc = K.ZW - 0.03
    u = [(zc, -xo), (K.ZW, -xo), (K.ZW, xo), (zc, xo), (zc, xo - t), (K.ZW - t, xo - t), (K.ZW - t, -xo + t), (zc, -xo + t)]
    swap = {
        "member-chassis": K.st(K.ext("member-chassis", "casing", GS, "body", 0.7, "zx", u, K.Y0 + 0.0015, K.Y1 - 0.0015), "casing", "member"),
        "support-wall-bracket": K.st(K.box("support-wall-bracket", "casing", GS, "body", 0.3, (0.2, 0.03, 0.03), (0.0, 0.74, K.ZW - 0.0155)), "casing", "support", anchor="wall-bracket"),
        "support-wall-rest": K.st(K.box("support-wall-rest", "casing", GS, "body", 0.3, (0.3, 0.03, 0.03), (0.0, 0.075, K.ZW - 0.0155)), "casing", "support", anchor="wall-rest"),
    }
    return [swap.get(s["id"], s) for s in R3.structure200()]


def body500():
    """🎨 1:500 body (75 mm feature): the hood as one crisp block from the front face to the wall (the 14 mm edge radius and the 20 mm chassis band are far below the feature), the control band merged flush into a darker front strip 15 mm proud as at 1:200, the flue adapter as one solid 125 mm drum and the five stubs as one connection bar."""
    return [
        K.ext("casing", "casing", PS, "body", 0.88, "xy", K.rrect(-K.XW, K.Y0, K.XW, K.Y1, 0.0), -0.15, K.ZW),
        K.box("panel", "casing", PC, "body", 0.35, (0.40, 0.145, 0.016), (0.0, 0.1375, -0.157)),
        K.rev("collar", "collar", FP, "body", 0.6, [(-0.002, 0.0), (-0.002, 0.0625), (0.05, 0.0625), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX),
        K.box("stubs", "connections", BR, "body", 0.6, (0.34, 0.052, 0.04), (0.0, 0.026, K.STUB_Z)),
    ]


def thermal500():
    """🌡️ 1:500 thermal: hood, control strip, the window as a flush disc, one burner block in the heat-exchanger drum, the flue drum with its outlet patch in the top face, the connection bar, and one solid per hidden region in its real place: the exchanger drum with the collector, the fan drum, the vessel lens behind, the hydraulic block below."""
    return [
        K.ref("th-casing", "casing", region="casing", layer="sheet"),
        K.ref("th-panel", "panel", region="controls", layer="panel"),
        K.th(K.rev("th-sight-glass", "casing", GL, "glazing", 0.85, [(-0.154, 0.0), (-0.154, 0.0375), (-0.15, 0.0375), (-0.15, 0.0)], (WX, WY, 0), ZAX, sectionFaces=["+w", "-u", "+u", "-v", "+v"]), "sight-glass", "glass"),
        K.th(K.box("th-burner", "casing", BM, "body", 0.9, (0.075, 0.04, 0.03), (0.0, 0.55, -0.11)), "burner", "mesh"),
        K.ref("th-collar", "collar", region="flue-collar", layer="pipe"),
        K.th(K.rev("th-flue-gas", "collar", FP, "detail", 0.5, [(0.046, 0.0), (0.046, 0.0375), (0.0505, 0.0375), (0.0505, 0.0)], (0, 0, K.COLLAR_Z), YAX), "flue-collar", None, tags=["flue-gas"]),
        K.ref("th-stubs", "stubs", region="connections", layer="tube"),
        K.th(K.rev("th-hx", "casing", SS, "body", 0.5, [(-0.14, 0.0), (-0.14, 0.115), (0.043, 0.115), (0.043, 0.0)], (WX, WY, 0), ZAX), "heat-exchanger", "wall"),
        K.th(K.rev("th-fan", "casing", AL, "body", 0.6, [(0.0, 0.0), (0.0, 0.0375), (0.075, 0.0375), (0.075, 0.0)], (-0.21, 0.60, -0.05), XAX), "fan", "housing"),
        K.th(K.rev("th-vessel", "casing", PS, "body", 0.3, LENS500, (0.0, 0.36, 0), ZAX), "expansion-vessel", "shell"),
        K.th(K.box("th-hydraulics", "casing", BR, "body", 0.36, (0.38, 0.12, 0.09), (0.0, 0.13, 0.04)), "hydraulics", "body"),
    ]


def structure500():
    """🏗️ 1:500 structure: the galvanised chassis as one 1.5 mm channel to the wall (the real member), one bracket shelf carrying the heat-exchanger drum, the flue collector and adapter as one tube up through the hood top, the connection bar on the bottom edge, rail and rest at the wall, and the mass at the centre of mass."""
    t = 0.0015
    xo = K.XW - 0.0015
    u = [(K.ZSEAM, -xo), (K.ZW, -xo), (K.ZW, xo), (K.ZSEAM, xo), (K.ZSEAM, xo - t), (K.ZW - t, xo - t), (K.ZW - t, -xo + t), (K.ZSEAM, -xo + t)]
    return [
        K.st(K.ext("member-chassis", "casing", GS, "body", 0.7, "zx", u, K.Y0 + 0.0015, K.Y1 - 0.0015), "casing", "member"),
        K.st(K.box("member-hx-brackets", "casing", GS, "body", 0.7, (0.23, 0.02, 0.13), (0.0, 0.437, 0.108)), "casing", "member"),
        K.st(K.rev("member-hx", "casing", SS, "body", 0.5, [(-0.14, 0.0), (-0.14, 0.118), (0.043, 0.118), (0.043, 0.0)], (WX, WY, 0), ZAX), "casing", "member"),
        K.st(K.rev("member-collar", "collar", FP, "body", 0.5, [(-0.12, 0.0), (-0.12, 0.04), (0.0, 0.04), (0.0, 0.0625), (0.05, 0.0625), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX), "collar", "member"),
        K.st(K.box("member-connections", "connections", BR, "body", 0.34, (0.34, 0.06, 0.03), (0.0, 0.03, K.STUB_Z)), "connections", "member"),
        K.st(K.box("support-wall-bracket", "casing", GS, "body", 0.3, (0.2, 0.03, 0.017), (0.0, 0.74, K.ZW - 0.009)), "casing", "support", anchor="wall-bracket"),
        K.st(K.box("support-wall-rest", "casing", GS, "body", 0.3, (0.3, 0.03, 0.016), (0.0, 0.075, K.ZW - 0.009)), "casing", "support", anchor="wall-rest"),
        K.st(K.ell("mass-casing", "casing", PS, "detail", 0.5, (0.04, 0.04, 0.04), (0.0, 0.42, 0.0)), "casing", "mass"),
    ]


def main():
    """💾 Writes the volumes source."""
    r3t = json.load(open(R3.TEXTS, encoding="utf-8"))
    r5t = json.load(open(TEXTS, encoding="utf-8"))
    vols = {
        "50": {"status": "authored", "denominator": 50, "body": R3.body50(), "thermal": R3.thermal50(), "structure": R3.structure50(), "decisions": r3t["50"]},
        "100": {"status": "authored", "denominator": 100, "body": R3.body100(), "thermal": R3.thermal100(), "structure": R3.structure100(), "decisions": r3t["100"]},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": r5t["200"]},
        "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": r5t["500"]},
    }
    for s, v in vols.items():
        for lst in ("body", "thermal", "structure"):
            ids = [x["id"] for x in v[lst]]
            if len(ids) != len(set(ids)):
                raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


if __name__ == "__main__":
    main()
