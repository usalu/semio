"""🔧 City car, second round pass 1: the nose is opened into scuttle, two front wings and a front end, and the engine bay under the bonnet is furnished (rails, engine, gearbox, radiator, battery) at every scale; every number is stated here by hand."""
import copy
from rework_09_lib import Source

ROT = [0.70711, 0, 0.70711, 0]
WING = {50: 0.14, 100: 0.14, 200: 0.14, 500: 0.15}
PAINT = 0.00008


def at_x(sections, x):
    """📏 The section of a loft along x at station x (linear between its neighbours)."""
    for a, b in zip(sections, sections[1:]):
        if a["at"][0] <= x <= b["at"][0]:
            t = (x - a["at"][0]) / (b["at"][0] - a["at"][0])
            mix = lambda k: round(a[k] + t * (b[k] - a[k]), 5)
            return {"at": [x, round(a["at"][1] + t * (b["at"][1] - a["at"][1]), 5), 0], "rx": mix("rx"), "ry": mix("ry"), "exponent": mix("exponent"), "rotation": ROT}
    raise SystemExit(f"no station {x}")


def wing(id, old, side, half):
    """🛡️ A front wing: the old outer face and crown, `half` wide to each side of its centre line."""
    stations = [0.99] + [s["at"][0] for s in old if 1.0 < s["at"][0] < 1.7] + [1.7]
    sections = []
    for x in stations:
        s = at_x(old, x)
        sections.append({"at": [x, s["at"][1], round(side * (s["rx"] - half), 5)], "rx": half, "ry": s["ry"], "exponent": 6, "rotation": ROT})
    return {"id": id, "kind": "loft", "bone": "chassis", "material": "paint", "role": "body", "tone": 0.62, "sections": sections, "capStart": True, "capEnd": True}


def sheet(solid):
    """🧻 The steel sheet under the paint film of a painted loft."""
    inner = copy.deepcopy(solid)
    inner["id"] = "th-" + solid["id"] + "-sheet"
    for s in inner["sections"]:
        s["rx"], s["ry"] = round(s["rx"] - PAINT, 5), round(s["ry"] - PAINT, 5)
    inner.update(region="body-panels", layer="sheet")
    return inner


def box(id, size, at, radius, material="engine-metal", tone=0.2, role="detail"):
    return {"id": id, "kind": "box", "bone": "chassis", "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": at}, "cornerRadius": radius}


src = Source("vehicles/city-car")
for scale in (50, 100, 200, 500):
    master = "hull" if scale == 500 else "nose"
    old = src.get(scale, "body", master)["sections"]
    rear = [s for s in old if s["at"][0] <= (0.9 if scale == 500 else 0.97)] + [at_x(old, 1.02)]
    front = [at_x(old, 1.66)] + [s for s in old if s["at"][0] >= 1.7]
    base = {k: v for k, v in src.get(scale, "body", master).items() if k != "sections"}
    if scale == 500:
        src.put(scale, "body", {**base, "sections": rear})
        nose = src.put(scale, "body", {**base, "id": "nose", "sections": front}, after="hull")
        scuttle = None
    else:
        scuttle = src.put(scale, "body", {**base, "id": "scuttle", "sections": rear}, after="nose")
        nose = src.put(scale, "body", {**base, "sections": front})
    wl = src.put(scale, "body", wing("wing-l", old, 1, WING[scale]), after="nose")
    wr = src.put(scale, "body", wing("wing-r", old, -1, WING[scale]), after="wing-l")
    radius = {50: 0.015, 100: 0.015, 200: 0.03, 500: 0.03}[scale]
    parts = []
    if scale < 500:
        rail = {50: 0.07, 100: 0.07, 200: 0.1}[scale]
        zr = {50: 0.42, 100: 0.42, 200: 0.40}[scale]
        parts += [box("front-rail-l", [1.0, 0.1, rail], [1.36, 0.47, zr], radius, "paint", 0.5), box("front-rail-r", [1.0, 0.1, rail], [1.36, 0.47, -zr], radius, "paint", 0.5)]
        parts.append(box("radiator", [0.05, 0.36, 0.62], [1.665, 0.56, 0], 0.01 if scale < 200 else 0.02, "engine-metal", 0.15))
    for th in ("th-engine-block", "th-gearbox", "th-exhaust-manifold", "th-engine"):
        solid = src.get(scale, "thermal", th)
        if solid:
            body = {k: v for k, v in solid.items() if k != "region"}
            body["id"] = th[3:]
            parts.append(body)
    if scale <= 100:
        parts.append(box("engine-mount", [0.1, 0.06, 0.12], [1.36, 0.55, 0.36], 0.015, "cast-iron", 0.3))
        parts.append(box("battery", [0.18, 0.19, 0.25], [1.55, 0.7, -0.435], 0.015, "plastic-trim", 0.1))
        parts.append(box("battery-tray", [0.2, 0.09, 0.07], [1.55, 0.565, -0.42], 0.015, "paint", 0.5))
    if scale == 50:
        parts.append(box("valve-cover", [0.2, 0.05, 0.34], [1.36, 0.77, 0.13], 0.02, "plastic-trim", 0.1))
        parts.append(box("air-box", [0.24, 0.16, 0.24], [1.31, 0.68, -0.27], 0.03, "plastic-trim", 0.1))
        src.drop(50, "body", "engine-cover")
        z = [-0.42, -0.38, -0.2, 0, 0.2, 0.38, 0.42]
        x = [1.946, 1.953, 1.958, 1.958, 1.958, 1.953, 1.946]
        src.put(50, "body", {"id": "intake", "kind": "loft", "bone": "chassis", "material": "plastic-trim", "role": "detail", "tone": 0.1, "sections": [{"at": [a, 0.37, b], "rx": 0.012, "ry": 0.045, "exponent": 4} for a, b in zip(x, z)], "capStart": True, "capEnd": True})
    last = "wing-r"
    for part in parts:
        src.put(scale, "body", part, after=last)
        last = part["id"]
    if scale == 50:
        src.drop(50, "thermal", "th-nose-sheet", "th-nose-paint")
        for solid in (nose, scuttle, wl, wr):
            src.put(50, "thermal", sheet(solid))
            src.put(50, "thermal", {"id": "th-" + solid["id"] + "-paint", "of": solid["id"], "region": "body-panels", "layer": "paint"})
    else:
        for solid in (nose, scuttle, wl, wr):
            if solid and not src.get(scale, "thermal", "th-" + solid["id"]):
                src.put(scale, "thermal", {"id": "th-" + solid["id"], "of": solid["id"], "region": "body-panels"})
    src.decide(
        scale,
        "The front end is no longer one closed lump: scuttle, two front wings and the front end with the slam panel leave the engine bay open under the bonnet, and the bay holds the powertrain of the thermal model (engine, gearbox) on the front rails, so the bonnet-open pose shows a bay instead of a second bonnet." if scale < 500 else "The hull ends at the scuttle; two front wings and the front end leave the engine bay open under the bonnet, with the engine block of the thermal model standing on the underbody, so the bonnet-open pose shows a bay.",
        "Der Vorderwagen ist kein geschlossener Klotz mehr: Spritzwand, zwei Kotflügel und die Front mit dem Schlossträger lassen den Motorraum unter der Haube offen, und darin steht der Antrieb des thermischen Modells (Motor, Getriebe) auf den Längsträgern, sodass die Pose mit offener Haube einen Motorraum statt einer zweiten Haube zeigt." if scale < 500 else "Der Rumpf endet an der Spritzwand; zwei Kotflügel und die Front lassen den Motorraum unter der Haube offen, der Motorblock des thermischen Modells steht auf dem Unterboden, sodass die Pose mit offener Haube einen Motorraum zeigt.",
    )
src.save()
