"""🪜 Hand edit of the catalog document of machinery/oil-pumpjack, second round pass 1: 2D shapes of the post ladder and the brake in the three views of every scale, the four slab bearings as structure supports. Writes a candidate (default) or the catalog file (--write); idempotent."""
import json
import pathlib
import sys

TICKET = pathlib.Path(__file__).parent
CATALOG = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/oil-pumpjack/🔣️.json")
SOURCE = TICKET / "volumes" / "machinery" / "oil-pumpjack.json"
CANDIDATE = TICKET / "🗑️generated" / "r3-oil-pumpjack" / "🔣️.json"
LEG_V0, LEG_SLOPE, LEG_U0 = 1.52922, -0.39887, -0.07326
CAP_UNDERSIDE = {"50": 3.56, "100": 3.55, "200": 3.55, "500": 3.525}
THICKNESS = {"50": 0.01, "100": 0.015, "200": 0.03, "500": 0.075}
DEPTH = {"50": 0.06, "100": 0.06, "200": 0.06, "500": 0.075}
HALF = 0.22
PREFIXES = ("ladder-", "brake-")


def leg_v(u):
    return LEG_V0 + LEG_SLOPE * (u - LEG_U0)


def shape(identity, bone, z, tone, region, role="body", material="painted-steel", **geometry):
    return {"id": identity, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, **geometry, "region": region, "material": material}


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": round(min(x0, x1), 4), "y": round(min(y0, y1), 4), "w": round(abs(x1 - x0), 4), "h": round(abs(y1 - y0), 4)}


def member(start, end, diameter):
    return {"kind": "member", "from": dict(zip("xyz", start)), "to": dict(zip("xyz", end)), "section": {"kind": "circle", "d": diameter}, "penMm": 0.2}


def shapes(scale):
    top, t, d = CAP_UNDERSIDE[scale], THICKNESS[scale], DEPTH[scale]
    coarse = scale == "500"
    rungs = [] if coarse else [0.05 + 0.28 * i for i in range(13)]
    elevation = [shape("ladder-stringer", "post", 3.28, 0.5, "steel", kind="poly", closed=True, points=[[-0.25, round(leg_v(-0.25) + 0.11, 4)], [top, round(leg_v(top) + 0.11, 4)], [top, round(leg_v(top) + 0.11 + d, 4)], [-0.25, round(leg_v(-0.25) + 0.11 + d, 4)]])]
    fine = "detail" if scale in ("50", "100") else "body"
    foot, head = [-0.25, round(leg_v(-0.25) + 0.11 + d / 2, 4)], [top, round(leg_v(top) + 0.11 + d / 2, 4)]
    stringers = [shape(f"ladder-stringer-{side}", "post", 3.28, 0.5, "steel", **member(foot + [sign * (HALF - t / 2)], head + [sign * (HALF - t / 2)], t)) for side, sign in (("l", 1), ("r", -1))]
    front, plan = list(stringers), list(stringers)
    for index, u in enumerate(rungs):
        rung = shape(f"ladder-rung-{index + 1}", "post", 3.27, 0.45, "steel", role=fine, **member([round(u, 4), round(leg_v(u) + 0.14, 4), -HALF], [round(u, 4), round(leg_v(u) + 0.14, 4), HALF], 0.02))
        front.append(rung)
        plan.append(rung)
    front.append(shape("brake-shaft", "gearbox", 8.4, 0.3, "gearbox", role=fine, material="cast-iron", **rect(0.42, 1.15, 0.81, 1.25)))
    front.append(shape("brake-drum", "gearbox", 8.45, 0.35, "gearbox", material="cast-iron", **rect(0.80, 1.01, 0.91, 1.39)))
    plan.append(shape("brake-shaft", "gearbox", 3.4, 0.3, "gearbox", role=fine, material="cast-iron", **rect(3.90, 0.42, 4.00, 0.81)))
    plan.append(shape("brake-drum", "gearbox", 3.45, 0.35, "gearbox", material="cast-iron", **rect(3.76, 0.80, 4.14, 0.91)))
    if not coarse:
        elevation.append(shape("brake-stand", "skid", 3.02, 0.4, "steel", **rect(6.90, 0.24, 7.00, 0.54)))
        elevation.append(shape("brake-lever", "skid", 3.02, 0.3, "steel", role=fine, **member([6.95, 0.50, 0.97], [7.10, 1.45, 0.97], 0.032)))
        if scale != "200":
            elevation.append(shape("brake-rod", "skid", 3.01, 0.35, "steel", role=fine, **member([6.99, 0.78, 0.97], [5.60, 1.13, 0.88], 0.016)))
        if scale != "200":
            front.append(shape("brake-band", "gearbox", 8.46, 0.28, "steel", **rect(0.805, 1.39, 0.905, 1.47)))
        front.append(shape("brake-stand", "skid", 8.5, 0.4, "steel", **rect(0.94, 0.54, 1.00, 0.84)))
        front.append(shape("brake-lever", "skid", 8.5, 0.3, "steel", role=fine, **member([6.95, 0.50, 0.97], [7.10, 1.45, 0.97], 0.032)))
        if scale != "200":
            plan.append(shape("brake-band", "gearbox", 3.46, 0.28, "steel", **rect(3.88, 0.805, 4.02, 0.905)))
        plan.append(shape("brake-stand", "skid", 3.1, 0.4, "steel", **rect(5.30, 0.94, 5.40, 1.00)))
        plan.append(shape("brake-lever", "skid", 3.5, 0.3, "steel", role=fine, **member([6.95, 0.50, 0.97], [7.10, 1.45, 0.97], 0.032)))
        if scale != "200":
            plan.append(shape("brake-rod", "skid", 3.09, 0.35, "steel", role=fine, **member([6.99, 0.78, 0.97], [5.60, 1.13, 0.88], 0.016)))
    return {"elevation": elevation, "front": front, "plan": plan}


document = json.loads(CATALOG.read_text())
asset = document["asset"]
for scale, representation in asset["representations"].items():
    for view, added in shapes(scale).items():
        kept = [s for s in representation["geometry"][view] if not s["id"].startswith(PREFIXES)]
        representation["geometry"][view] = kept + added
asset["structure"]["supports"] = ["contact-a-l", "contact-a-r", "contact-b-l", "contact-b-r"]
if "--write" in sys.argv:
    CATALOG.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("catalog written")
else:
    asset["volumes"] = json.loads(SOURCE.read_text())["volumes"]
    CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
    CANDIDATE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("candidate", CANDIDATE)
