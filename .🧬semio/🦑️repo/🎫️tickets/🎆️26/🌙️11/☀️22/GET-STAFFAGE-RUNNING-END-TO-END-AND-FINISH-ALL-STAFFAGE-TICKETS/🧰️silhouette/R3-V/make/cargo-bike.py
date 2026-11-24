"""🚲️ Writes the hand-made repair ops of `cargo-bike` into ops/cargo-bike.json (the first step of make/cargo-bike.sh).

Brake discs, chain ring and chain guard (1:50 to 1:200): the body carries these 2 to 4 mm discs as `texture` solids (a sheet seen flat, no part of the silhouette); the drawing had them as `detail` shapes, which the guard reads as the edge of a body of 180 mm inside the open wheel and as a fill of the opening between chain, chain stay and motor. The drawn discs are `texture` like the body and like the drawn brake track and spokes.

Body, rear hub: the boss that carries the brake disc was 48 mm across; a six-bolt disc mount is 58 mm, and with 48 mm the opening between boss, drop-out and stays measured 903 mm2 in the plan, on the very size (900 mm2 at 1:50) from which the guard counts an opening, so body and drawing fell on different sides of it. The boss is 58 mm at every scale that models it.

Side elevation of 1:50, stand feet: the rubber feet are round bars of 24 mm across the bike; they were drawn as bars of 24 x 52 mm standing upright and are drawn as their round section. Plan of 1:50, rear hub: one cylinder of 66 mm over the whole 135 mm, the body is a stepped hub (28 mm stubs, the 66 mm shell, the 58 mm disc mount); it is drawn as four cylinders (the 4 mm flanges are below 0.2 mm on paper and stay undrawn); at 1:100 one cylinder of the shell diameter, 105 mm long (a shorter one would leave strips below 0.2 mm on paper beside the mudguard). Front view of 1:50: the bell was the box of its dome (52 x 30 mm); it is drawn as two cylinders about its axis (52 and 38 mm), which follow the dome. The chain stays and seat stays taper towards the drop-out (23 to 17 mm and 19 to 15 mm); the 1:50 plan drew them 20 and 17 mm thick throughout and draws them at the thickness of the drop-out end, where they frame the opening beside the hub. Front view of 1:100: the two stand feet are drawn (round bars of 24 mm), and the two rear box rails, which the bike on its stand shows 5 and 10 mm below the front one. Plan of 1:100: the three cross bars of the carrier between its ends are drawn as 0.15 mm strokes (15 mm tubes), and the crank axle is drawn (it closes the slit between battery and crank arm).
Side elevation of 1:100 and 1:200, carrier: the body has two legs a side that meet at the drop-out as a V; the drawing had one leg between them. Both legs are drawn (the two sides lie behind each other)."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, ".."))
from lib import asset, shape, solid

SLUG = "cargo-bike"
a = asset(SLUG)
ops = []
for scale in ("50", "100", "200"):
    textures = {s["id"] for s in a["volumes"][scale]["body"] if s.get("texture")}
    for view in ("elevation", "front", "plan"):
        for s in a["representations"][scale]["geometry"][view]:
            if s["id"] in textures and s["role"] != "texture":
                ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"role": "texture"}})
for foot, (z0, z1) in (("stand-foot-l", (0.168, 0.244)), ("stand-foot-r", (-0.244, -0.168))):
    old = shape(a, 50, "elevation", foot)
    s = solid(a, 50, foot)
    ops.append({"op": "shape", "scale": "50", "view": "elevation", "id": foot, "shape": {**{k: v for k, v in old.items() if k not in ("points", "closed", "kind")}, "kind": "ellipse", "cx": s["p0"][0], "cy": s["p0"][1], "rx": s["r0"], "ry": s["r0"], "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}})
for scale in ("50", "100", "200", "500"):
    hubs = [s for s in a["volumes"][scale].get("body", []) if s["id"] == "hub-r" and [0.04, 0.024] in s.get("profile", []) and [0.052, 0.024] in s["profile"]]
    for s in hubs:
        ops.append({"op": "solid-set", "scale": scale, "id": "hub-r", "set": {"profile": [[p[0], 0.029] if p in ([0.04, 0.024], [0.052, 0.024]) else p for p in s["profile"]]}})
hub = shape(a, 50, "plan", "hub-r")
ops.append({"op": "shape-remove", "scale": "50", "view": "plan", "id": "hub-r"})
for n, (r, z0, z1) in enumerate(((0.014, -0.0675, -0.042), (0.033, -0.042, 0.04), (0.029, 0.04, 0.052), (0.014, 0.052, 0.0675)), 1):
    ops.append({"op": "shape-add", "scale": "50", "view": "plan", "after": "axle-r", "shape": {**hub, "id": f"hub-r-{n}", "z": round(hub["z"] + n / 10, 2), "rx": r, "ry": r, "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}})
for sid, d in (("chainstay-l", 0.017), ("chainstay-r", 0.017), ("seatstay-l", 0.015), ("seatstay-r", 0.015)):
    ops.append({"op": "shape-set", "scale": "50", "view": "plan", "id": sid, "set": {"section": {"kind": "circle", "d": d}}})
ops.append({"op": "shape-set", "scale": "100", "view": "plan", "id": "hub-r", "set": {"rx": 0.033, "ry": 0.033, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.0525, "z1": 0.0525}}})
bell, dome = shape(a, 50, "front", "bell"), solid(a, 50, "bell")
ops.append({"op": "shape-remove", "scale": "50", "view": "front", "id": "bell"})
(u0, v0, w0) = dome["origin"]
for n, (r, a0, a1) in enumerate(((0.026, 0, 0.013), (0.019, 0.013, 0.027)), 1):
    ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": "display", "shape": {**{k: v for k, v in bell.items() if k not in ("points", "closed", "kind", "solid")}, "kind": "ellipse", "id": f"bell-{n}", "z": round(bell["z"] + n / 10, 2), "cx": v0, "cy": w0, "rx": r, "ry": r, "solid": {"kind": "extruded", "sweep": "u", "z0": round(u0 + a0, 5), "z1": round(u0 + a1, 5)}}})
rail = shape(a, 100, "front", "box-rail-3")
for n in (1, 2):
    ops.append({"op": "shape-add", "scale": "100", "view": "front", "after": "box-rail-3", "shape": {**rail, "id": f"box-rail-{n}", "z": round(rail["z"] - 0.3 + n / 10, 2), "points": shape(a, 100, "plan", f"box-rail-{n}")["points"]}})
axle = solid(a, 100, "crank-axle")
ops.append({"op": "shape-add", "scale": "100", "view": "plan", "after": "hub-f", "shape": {**shape(a, 100, "plan", "hub-f"), "id": "crank-axle", "bone": "crank", "z": 5.5, "tone": 0.6, "rx": axle["profile"][0][1], "ry": axle["profile"][0][1], "solid": {"kind": "extruded", "sweep": "w", "z0": axle["origin"][2], "z1": round(axle["origin"][2] + axle["profile"][1][0], 5)}}})
hoop = shape(a, 100, "plan", "carrier-hoop")
for n in (2, 3, 4):
    s = solid(a, 100, f"carrier-bar-{n}")
    ops.append({"op": "shape-add", "scale": "100", "view": "plan", "after": "carrier-hoop", "shape": {"kind": "member", "id": f"carrier-bar-{n}", "bone": "frame", "role": "detail", "z": round(hoop["z"] + n / 100, 2), "filled": False, "tone": hoop["tone"], "region": hoop.get("region"), "material": hoop["material"], "penMm": 0.15, "from": dict(zip("xyz", s["p0"])), "to": dict(zip("xyz", s["p1"])), "section": {"kind": "circle", "d": 2 * s["r0"]}}})
pivot = shape(a, 100, "front", "stand-pivot")
for foot, (z0, z1) in (("stand-foot-l", (0.168, 0.244)), ("stand-foot-r", (-0.244, -0.168))):
    s = solid(a, 100, foot)
    ops.append({"op": "shape-add", "scale": "100", "view": "front", "after": "stand-pivot", "shape": {**pivot, "id": foot, "tone": 0.1, "material": "rubber", "z": round(pivot["z"] + 0.2, 2), "cx": s["p0"][0], "cy": s["p0"][1], "rx": s["r0"], "ry": s["r0"], "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}})
for scale in ("100", "200"):
    old = shape(a, scale, "elevation", "carrier-leg-l")
    for side in ("l", "r"):
        ops.append({"op": "shape-remove", "scale": scale, "view": "elevation", "id": f"carrier-leg-{side}"})
    for leg in ("a", "b"):
        s = solid(a, scale, f"carrier-leg-{leg}-l")
        ops.append({"op": "shape-add", "scale": scale, "view": "elevation", "after": "carrier-hoop", "shape": {**old, "id": f"carrier-leg-{leg}", "z": round(old["z"] + (0.1 if leg == "a" else 0.2), 2), "points": [s["p0"][:2], s["p1"][:2]]}})
subprocess.run([sys.executable, os.path.join(HERE, "..", "hand.py"), SLUG], input=json.dumps(ops), text=True, check=True)
