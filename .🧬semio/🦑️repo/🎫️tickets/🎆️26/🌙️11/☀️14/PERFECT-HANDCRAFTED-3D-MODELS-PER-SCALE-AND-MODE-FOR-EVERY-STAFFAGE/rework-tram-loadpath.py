"""🚋 Tram pass 1 (second round): closes the load path by hand. Every number is chosen against the dumped geometry (see rework/assets/vehicles--tram.md).

Body: articulation arm seated under both floors, trailing-bogie primary springs down onto the axle boxes, primary pads over the gearboxes (1:100, 1:200), pantograph coupling-rod pins and drive rod.
Structure: pantograph feet on two reinforced roof bows, shaft bearings, cant rails deep enough to take the bows, secondary springs and bolsters, axle boxes and primaries, traction link, door headers and hangers, cab bulkhead posts, header ring, B-posts, cross-bearers out of the wheel envelope."""
import copy, json, sys

PATH = "volumes/vehicles/tram.json"
doc = json.load(open(PATH))
V = doc["volumes"]
if any(s.get("id") == "end-pan-bow-1" for s in V["50"]["structure"]):
    sys.exit("already applied")


def get(scale, lst, sid):
    return next(s for s in V[scale][lst] if s.get("id") == sid)


def has(scale, lst, sid):
    return any(s.get("id") == sid for s in V[scale][lst])


def add(scale, lst, after, solid):
    assert not has(scale, lst, solid["id"]), solid["id"]
    items = V[scale][lst]
    at = next((i for i, s in enumerate(items) if s.get("id") == after), len(items) - 1)
    items.insert(at + 1, solid)


def box(scale, lst, template, sid, bone, size, at, part=None, title=None, rotation=None):
    s = copy.deepcopy(get(scale, lst, template))
    for k in ("of", "profile", "plane", "from", "to", "holes", "axis", "origin", "cornerRadius", "p0", "p1", "r0", "r1", "slender", "sheet", "members", "repeat"):
        s.pop(k, None)
    s.update({"id": sid, "bone": bone, "kind": "box", "size": size, "transform": {"translation": at, **({"rotation": rotation} if rotation else {})}})
    if lst == "structure":
        s.update({"member": bone, "part": part or "member"})
    if title and "title" in s:
        s["title"] = title
    return s


def cyl(scale, lst, template, sid, bone, radius, length, origin, axis, part=None):
    s = box(scale, lst, template, sid, bone, [0, 0, 0], [0, 0, 0], part)
    del s["size"], s["transform"]
    s.update({"kind": "revolve", "profile": [[0, 0.0], [0, radius], [length, radius], [length, 0.0]], "axis": axis, "origin": origin})
    return s


def ref(scale, sid, of, bone, part):
    assert has(scale, "body", of), of
    return {"id": sid, "of": of, "member": bone, "part": part}


SIDES = (("l", 1), ("r", -1))

# ---------- body ----------
for sc in ("50", "100"):
    arm = get(sc, "body", "articulation-arm")
    assert arm["size"] == [0.92, 0.07, 0.28]
    arm["size"] = [0.92, 0.085, 0.28]
    arm["transform"]["translation"] = [0.0, -0.0575, 0.0]

for side, sz in SIDES:
    for n, x in (("1", -0.9), ("2", 0.9)):
        p = get("50", "body", f"primary-r-{side}-{n}")
        assert p["size"] == [0.18, 0.04, 0.15]
        p["size"] = [0.18, 0.09, 0.15]
        p["transform"]["translation"][1] = 0.16
        for sc, size, y in (("100", [0.2, 0.06, 0.15], 0.165), ("200", [0.24, 0.08, 0.17], 0.16)):
            add(sc, "body", f"gearbox-f-{side}-{n}", box(sc, "body", "bogie-beam-f-l", f"primary-f-{side}-{n}", "bogie-f", size, [x, y, sz * 0.915]) | {"material": "rubber", "role": "detail", "tone": 0.12})

T = "pan-knee"
add("50", "body", "pan-lower-rod", cyl("50", "body", T, "pan-rod-pin-base", "pan-lower", 0.022, 0.115, [0.22, -0.045, 0.0], [0, 0, 1]))
add("50", "body", "pan-rod-pin-base", cyl("50", "body", T, "pan-rod-pin-knee", "pan-lower", 0.02, 0.115, [1.84, -0.035, 0.0], [0, 0, 1]))
drive = get("50", "body", "pan-drive")
assert drive["profile"][2] == [0.6, 0.05]
drive["profile"] = [[0, 0.0], [0, 0.05], [0.78, 0.05], [0.78, 0.0]]
add("50", "body", "pan-drive", cyl("50", "body", T, "pan-drive-rod", "body-end", 0.015, 0.36, [2.6, 3.537, 0.17], [1, 0, 0]))

# ---------- structure ----------
BOW = {"50": 0.03, "100": 0.035, "200": 0.04, "500": 0.05}
CANT = {"100": (0.12, 3.04), "200": (0.14, 3.04), "500": (0.16, 3.03)}
for sc in ("50", "100", "200", "500"):
    st = "structure"
    half = BOW[sc]
    for n, x in (("1", 2.35), ("2", 3.75)):
        bow = copy.deepcopy(get(sc, st, "end-roof-bow-2"))
        bow.update({"id": f"end-pan-bow-{n}", "from": round(x - half, 4), "to": round(x + half, 4)})
        add(sc, st, "end-roof-bow-2", bow)
    tb = "end-cross-bearer-1"
    if sc == "500":
        for n, x in (("1", 2.35), ("2", 3.75)):
            add(sc, st, "st-pan-base", box(sc, st, tb, f"st-pan-foot-{n}", "body-end", [0.1, 0.13, 0.94], [x, 3.335, 0.0]))
    else:
        low = 3.32 if sc == "200" else 3.335 if sc == "50" else 3.33
        for n, x in (("1", 2.35), ("2", 3.75)):
            for side, sz in SIDES:
                last = "st-pan-frame-cross-2" if sc == "50" else "st-pan-frame-r"
                add(sc, st, last, cyl(sc, st, tb, f"st-pan-foot-{n}-{side}", "body-end", 0.05, round(3.395 - low, 3), [x, low, sz * 0.42], [0, 1, 0]))
                add(sc, st, f"st-pan-foot-{n}-{side}", ref(sc, f"st-pan-insulator-{n}-{side}", f"pan-insulator-{n}-{side}", "body-end", "joint"))
    if sc in CANT:
        h, y = CANT[sc]
        for mod, yy in (("end", y), ("mid", round(-(y - 0.3), 4))):
            for side, _ in SIDES:
                c = get(sc, st, f"{mod}-cant-rail-{side}")
                c["size"][1] = h
                c["transform"]["translation"][1] = yy
        # bolsters over the bogies and the secondary springs that carry them
        for bg, bx, length in (("r", 1.9, 2.6), ("f", 7.19, 2.58)):
            for side, sz in SIDES:
                add(sc, st, "end-cant-rail-r", box(sc, st, tb, f"end-bolster-{bg}-{side}", "body-end", [length, 0.1, 0.345], [bx, 0.75, sz * 0.9725]))
                for n, x in (("1", -0.3), ("2", 0.3)):
                    sid = f"st-spring-{bg}-{side}-{n}"
                    if has(sc, "body", f"spring-{bg}-{side}-{n}"):
                        add(sc, st, f"pivot-{bg}", ref(sc, sid, f"spring-{bg}-{side}-{n}", f"bogie-{bg}", "joint"))
                    else:
                        top, hh, rr = (0.30, 0.10, 0.1) if sc == "500" else (0.32, 0.08, 0.09)
                        add(sc, st, f"pivot-{bg}", cyl(sc, st, tb, sid, f"bogie-{bg}", rr, hh, [x, top, sz * 0.9], [0, 1, 0], "joint"))
        # cab: bulkhead posts from floor to waist ring, header ring over the A-posts, B-posts to the cant rails
        t = {"100": 0.08, "200": 0.08, "500": 0.1}[sc]
        for side, sz in SIDES:
            add(sc, st, "cab-ring", box(sc, st, tb, f"cab-bulkhead-post-{side}", "body-end", [0.08 if sc != "500" else 0.1, 0.86, t], [8.5, 0.71, sz * 1.13]))
            b = copy.deepcopy(get("50", st, f"cab-b-post-{side}"))
            b["size"] = [b["size"][0], t, t]
            add(sc, st, f"cab-post-{side}", b)
        hd = copy.deepcopy(get("50", st, "cab-header-ring"))
        hd.update({"from": 2.63, "to": 2.72 if sc != "500" else 2.73})
        add(sc, st, "cab-ring", hd)

# 1:50: pantograph bearings, axle boxes and primaries, upper articulation link into the end rings, door hangers
st = "structure"
for side, _ in SIDES:
    add("50", st, "st-pan-shaft", ref("50", f"st-pan-bearing-{side}", f"pan-bearing-{side}", "body-end", "joint"))
    for n in ("1", "2"):
        add("50", st, f"st-bogie-beam-f-{side}", ref("50", f"st-gearbox-f-{side}-{n}", f"gearbox-f-{side}-{n}", "bogie-f", "member"))
        add("50", st, f"st-bogie-beam-f-{side}", ref("50", f"st-primary-f-{side}-{n}", f"primary-f-{side}-{n}", "bogie-f", "joint"))
        add("50", st, f"st-bogie-beam-r-{side}", ref("50", f"st-axle-box-r-{side}-{n}", f"axle-box-r-{side}-{n}", "bogie-r", "member"))
        add("50", st, f"st-bogie-beam-r-{side}", ref("50", f"st-primary-r-{side}-{n}", f"primary-r-{side}-{n}", "bogie-r", "joint"))
up = get("50", st, "articulation-upper")
assert up["transform"]["translation"] == [0.0, 2.815, 0.0]
up["transform"]["translation"] = [0.0, 2.925, 0.0]
get("50", st, "articulation-upper-pin")["origin"] = [0, 2.86, 0.0]
for d in ("a1", "a2", "b1", "b2"):
    c = get("50", st, f"leaf-carriage-{d}")
    assert c["size"] == [0.2, 0.07, 0.04], c["size"]
    c["size"] = [0.2, 0.09, 0.1]
    t = c["transform"]["translation"]
    c["transform"]["translation"] = [t[0], round(t[1] - 0.01, 5), round(t[2] - 0.03, 5)]

# 1:100 and 1:200: pantograph head, door headers and hangers, traction link / drop chords, cross-bearers clear of the wheels, cab floor frame
for sc in ("100", "200"):
    tb = "end-cross-bearer-1"
    if sc == "100":
        add(sc, st, "st-pan-strip-2", ref(sc, "st-pan-apex", "pan-apex", "pan-head", "member"))
        for bg in ("f", "r"):
            add(sc, st, f"st-bogie-cross-{bg}-2", box(sc, st, tb, f"st-traction-link-{bg}", f"bogie-{bg}", [1.0, 0.07, 0.12], [0.0, -0.085, 0.0]))
    else:
        for bg in ("f", "r"):
            for side, sz in SIDES:
                add(sc, st, f"st-bogie-cross-{bg}", box(sc, st, tb, f"st-bogie-chord-{bg}-{side}", f"bogie-{bg}", [1.1, 0.14, 0.1], [0.0, -0.11, sz * 0.85]))
    zp, tp = (-1.1005, 0.07) if sc == "100" else (-1.115, 0.08)
    add(sc, st, "end-cant-rail-r", box(sc, st, tb, "end-door-header", "body-end", [1.47, 0.08, tp], [4.15, 2.5, zp]))
    add(sc, st, "mid-cant-rail-r", box(sc, st, "mid-cross-bearer-2", "mid-door-header", "body-mid", [1.47, 0.08, tp], [3.35, -2.2, zp]))
    for d in ("a1", "a2", "b1", "b2"):
        size, z = ([0.2, 0.09, 0.12], -1.14) if sc == "100" else ([0.24, 0.09, 0.13], -1.145)
        add(sc, st, f"st-leaf-{d}-rail-top", box(sc, st, tb, f"leaf-carriage-{d}", f"door-{d}", size, [0.325, 2.165, z], "joint"))
    b2, b4 = get(sc, st, "end-cross-bearer-2"), get(sc, st, "end-cross-bearer-4")
    b2["transform"]["translation"][0] = 3.26
    b4["transform"]["translation"][0] = 8.53
    fl = copy.deepcopy(get("50", st, "cab-floor-frame"))
    fl.update({"from": 0.282, "to": 0.348})
    add(sc, st, "cab-ring", fl)

json.dump(doc, open(PATH, "w"), ensure_ascii=False)
print("applied")
