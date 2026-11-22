"""🔗 Author pass 1, second chisel on machinery/concrete-mixer-truck: every part of the 1:50 model joined to what carries it (cross members into the rail webs, tank, battery, fender, spare chute, ladder and lamp brackets, hopper brackets, rear frame gussets, funnel collar, chute fork, dampers and torque rods on their mounts) and the wheel, brake and axle joints of the coarser scales.

Usage: python3 rework-10-concrete-mixer-truck-p2.py   (source only; publish with volumes-insert.py)
"""
exec(open("rework-10-lib.py").read())
S = Source("concrete-mixer-truck")
P, G = "paint-steel", "galvanised-steel"


def has(scale, sid, lst="body"):
    return any(s["id"] == sid for s in S.v[str(scale)][lst])


def struct(scale, of, member, part="member"):
    lst = S.v[str(scale)]["structure"]
    if not any(s.get("of") == of for s in lst):
        lst.append({"id": "st-" + of, "of": of, "member": member, "part": part})


def sides(fn):
    return [fn(side, sign) for side, sign in (("l", 1), ("r", -1))]


# 1:50 ------------------------------------------------------------------
for n in range(1, 8):
    S.get(50, f"frame-cross-{n}")["size"] = [0.09, 0.2, 0.752]
S.get(50, "air-tank-bracket")["size"] = [0.08, 0.5, 0.26]
S.put(50, [box("battery-bracket", "chassis", P, [0.3, 0.06, 0.13], [0.5, 0.88, 0.44], tone=0.3),
           box("fuel-tank-bracket-1", "chassis", P, [0.05, 0.06, 0.155], [0.1, 0.87, 0.4525], tone=0.3),
           box("fuel-tank-bracket-2", "chassis", P, [0.05, 0.06, 0.155], [-0.42, 0.87, 0.4525], tone=0.3)], region="chassis")
S.put(50, sides(lambda s, k: {"id": f"fender-2-stay-{s}", "kind": "capsule", "bone": "chassis", "material": P, "role": "detail", "tone": 0.3, "p0": [1.23, 1.0, k * 0.36], "p1": [1.23, 1.12, k * 0.87], "r0": 0.02, "r1": 0.02})
      + [box(f"fender-rear-stay-{s}-{i}", "chassis", P, [0.05, 0.04, 0.2], [x, 1.12, k * 0.47], tone=0.3) for s, k in (("l", 1), ("r", -1)) for i, x in ((1, -1.275), (2, -2.675))], region="running-gear")
for s, k in (("l", 1), ("r", -1)):
    S.get(50, f"mud-flap-{s}")["transform"]["translation"] = [-3.29, 0.44, k * 0.92]
    S.get(50, f"tail-lamp-{s}")["transform"]["translation"] = [-4.455, 0.8, k * 0.98]
    for n, x0 in ((1, 3.025), (2, 1.23)):
        d = S.get(50, f"damper-{n}-{s}")
        d["p0"], d["p1"] = [x0 + 0.055, 0.56, k * 0.5], [x0 + 0.3, 1.02, k * 0.405]
    for i, (x0, x1) in ((1, (-1.325, -1.95)), (2, (-2.625, -2.0))):
        r = S.get(50, f"torque-rod-{s}-{i}")
        r["p0"], r["p1"] = [x0, 0.44, k * 0.42], [x1, 0.52, k * 0.42]
    st = S.get(50, f"swivel-stay-{s}")
    st["p1"] = [-3.95, 2.02, k * 0.22]
    hs = S.get(50, f"platform-stay") if s == "l" else None
    if hs:
        hs["p0"] = [-3.75, 2.5, 0.57]
S.get(50, "tail-lamp-beam")["size"] = [0.1, 0.1, 2.36]
S.get(50, "tail-lamp-beam")["transform"]["translation"] = [-4.38, 0.8, 0]
S.get(50, "chute-hinge-1")["transform"]["translation"] = [1.23, -0.01, -0.262]
S.get(50, "chute-hinge-2")["transform"]["translation"] = [1.23, -0.01, 0.262]
S.put(50, sides(lambda s, k: box(f"torque-rod-bracket-{s}", "chassis", P, [0.2, 0.34, 0.06], [-1.975, 0.6, k * 0.42], tone=0.3)), region="running-gear")
S.put(50, [box(f"spare-chute-cradle-{s}-{i}", "chassis", P, [0.05, 0.05, 0.3], [x, 1.15, k * 0.97], tone=0.3) for s, k in (("l", 1), ("r", -1)) for i, x in ((1, -1.7), (2, -2.35))], region="chute")
S.put(50, sides(lambda s, k: box(f"hopper-bracket-{s}", "chassis", P, [0.06, 0.05, 0.16], [-3.98, 3.38, k * 0.46], tone=0.35))
      + sides(lambda s, k: box(f"rear-frame-gusset-{s}", "chassis", P, [0.3, 0.04, 0.3], [-3.51, 1.17, k * 0.44], tone=0.4))
      + [box("ladder-foot", "chassis", P, [0.2, 0.03, 0.46], [-4.29, 0.8, 1.01], tone=0.35)], region="superstructure")
for s in ("l", "r"):
    struct(50, f"rear-frame-gusset-{s}", "chassis")
    struct(50, f"torque-rod-bracket-{s}", "chassis")


def chute_carry(scale):
    S.put(scale, [{"id": "funnel-collar", "kind": "revolve", "bone": "chassis", "material": G, "role": "body", "tone": 0.45, "profile": [[0, 0.165], [0, 0.2], [0.09, 0.2], [0.09, 0.165], [0, 0.165]], "axis": [0, 1, 0], "origin": [-3.95, 2.025, 0]}]
          + sides(lambda s, k: box(f"chute-fork-{s}", "chute-swivel", G, [0.13, 0.06, 0.03], [-0.015, 0, k * 0.25], tone=0.42)), region="chute")
    for s in ("l", "r"):
        struct(scale, f"chute-fork-{s}", "chute-swivel")
    if scale != 50:
        S.put(scale, sides(lambda s, k: {"id": f"swivel-stay-{s}", "kind": "capsule", "bone": "chassis", "material": P, "role": "detail", "tone": 0.3, "p0": [-3.8, 2.55, k * 0.5], "p1": [-3.95, 2.02, k * 0.22], "r0": 0.022, "r1": 0.022}), region="superstructure")
        for s in ("l", "r"):
            struct(scale, f"swivel-stay-{s}", "chassis")
    for j in S.v[str(scale)]["structure"]:
        if j["id"] == "joint-chute-swivel":
            j["profile"] = [[0.02, 0], [0.02, 0.18], [0.08, 0.18], [0.08, 0]]


for sc in (50, 100, 200):
    chute_carry(sc)

# wheels, brakes, axle joints ---------------------------------------------
for sc in (50, 100, 200):
    for n in (1, 2):
        for s in ("l", "r"):
            S.get(sc, f"brake-drum-{n}-{s}")["profile"] = [[0, 0.21], [0.215, 0.21]]
    for j in S.v[str(sc)]["structure"]:
        if j["id"].startswith(("joint-wheel-3", "joint-wheel-4")):
            j["origin"] = [0, 0, 0.64 if j["axis"][2] > 0 else -0.64]
for sc in (50, 100):
    for n in (3, 4):
        for s in ("l", "r"):
            S.get(sc, f"hub-{n}-{s}-outer")["profile"][0] = [-0.11, 0.15]
for n in (3, 4):
    for s in ("l", "r"):
        S.get(200, f"rim-{n}-{s}-outer")["profile"] = [[-0.185, 0.26], [-0.185, 0.31], [0.13, 0.31], [0.13, 0.26], [-0.185, 0.26]]
        S.get(200, f"rim-{n}-{s}-inner")["profile"] = [[-0.13, 0.26], [-0.13, 0.31], [0.185, 0.31], [0.185, 0.26], [-0.13, 0.26]]
        S.get(200, f"hub-{n}-{s}-outer")["profile"] = [[-0.29, 0], [-0.29, 0.15], [-0.145, 0.15], [0.03, 0.115], [0.054, 0.08], [0.054, 0]]
        S.get(500, f"wheel-disc-{n}-{s}-inner")["profile"] = [[0.06, 0], [0.06, 0.29], [0.175, 0.29], [0.175, 0]]
for n, x0 in ((1, 3.025), (2, 1.23)):
    S.put(500, sides(lambda s, k: box(f"spring-{n}-{s}", "chassis", P, [1.1, 0.24, 0.1], [x0, 0.69, k * 0.34], tone=0.28)), region="running-gear")
S.put(500, sides(lambda s, k: box(f"bogie-spring-{s}", "chassis", P, [1.55, 0.22, 0.1], [-1.975, 0.7, k * 0.34], tone=0.28)), region="running-gear")
for s in ("l", "r"):
    for sid in (f"spring-1-{s}", f"spring-2-{s}", f"bogie-spring-{s}"):
        struct(500, sid, "chassis")
    for n in (1, 2, 3, 4):
        struct(500, f"brake-drum-{n}-{s}", f"wheel-{n}")

# 1:100 front springs on hangers ------------------------------------------
c = lambda dx: 0.035 * (dx / 0.74) ** 2
for n, x0 in ((1, 3.025), (2, 1.23)):
    top = [[x0 + dx, 0.665 + c(dx)] for dx in (-0.74, -0.37, 0, 0.37, 0.74)]
    half = [[0.74, 0.64 + c(0.74)], [0.5, 0.64 + c(0.5)], [0.5, 0.6 + c(0.5)], [0.25, 0.6 + c(0.25)], [0.25, 0.565]]
    prof = top + [[x0 + dx, y] for dx, y in half] + [[x0 - dx, y] for dx, y in half[::-1]]
    for s, k in (("l", 1), ("r", -1)):
        S.put(100, [ext(f"spring-{n}-{s}", "chassis", P, prof, "xy", k * 0.34 - 0.045, k * 0.34 + 0.045, tone=0.28, role="detail")])
        S.put(100, [box(f"spring-hanger-{n}-{s}-{i}", "chassis", P, [0.1, 0.13, 0.1], [x0 + dx, 0.745, k * 0.34], tone=0.28) for i, dx in ((1, -0.74), (2, 0.74))], region="running-gear")
        for i in (1, 2):
            struct(100, f"spring-hanger-{n}-{s}-{i}", "chassis")


# second round: exhaust, intake, cooler, visor, folding chute pin; 1:100 brackets of the parts that equal 1:50 ------
def carriers(scale):
    parts = [box("silencer-bracket", "chassis", P, [0.3, 0.06, 0.17], [2.09, 0.9, -0.455], tone=0.3),
             box("air-filter-bracket", "chassis", P, [0.3, 0.06, 0.22], [2.09, 0.95, 0.48], tone=0.3),
             box("fuel-tank-bracket-1", "chassis", P, [0.05, 0.06, 0.155], [0.1, 0.87, 0.4525], tone=0.3),
             box("fuel-tank-bracket-2", "chassis", P, [0.05, 0.06, 0.155], [-0.42, 0.87, 0.4525], tone=0.3),
             box("air-tank-bracket", "chassis", P, [0.08, 0.5, 0.26], [-0.09, 0.8, -0.5], tone=0.3)]
    S.put(scale, parts, region="chassis")


carriers(50)
carriers(100)
S.put(50, [box("oil-cooler-bracket", "chassis", P, [0.2, 0.05, 0.06], [1.98, 1.5, 0.465], tone=0.3)], region="drum-drive")
S.put(50, sides(lambda s, k: box(f"sun-visor-bracket-{s}", "chassis", P, [0.07, 0.02, 0.04], [4.385, 2.9, k * 0.8], tone=0.25)), region="cab")
S.put(50, [{"id": "chute-ext-hinge-pin", "kind": "revolve", "bone": "chute", "material": "cast-iron", "role": "detail", "tone": 0.3, "profile": [[0, 0], [0, 0.012], [0.56, 0.012], [0.56, 0]], "axis": [0, 0, 1], "origin": [1.25, 0, -0.28]}], region="chute")
S.put(50, [{"id": "exhaust-outlet", "kind": "capsule", "bone": "chassis", "material": "exhaust-steel", "role": "detail", "tone": 0.3, "p0": [2.22, 0.93, -0.95], "p1": [2.37, 1.04, -0.95], "r0": 0.055, "r1": 0.055}], region="exhaust-stack")
S.put(50, [box("stack-clamp", "chassis", P, [0.14, 0.04, 0.2], [2.53, 2.3, -1.02], tone=0.3)], region="exhaust-stack")
struct(50, "chute-ext-hinge-pin", "chute-ext", "joint")

S.decide(50, "Every part is joined to what carries it: the cross members run into the rail webs, battery box, fuel tank and air tanks sit on brackets off the rail, fenders on stays, the spare chutes in cradles on the rear fenders, the lamp beam on the underrun brackets, the ladder on a foot off that beam, the hopper on brackets of its stays, the rear frame on gussets over the subframe, the funnel in a collar on the swivel ring and the trough in a fork under it; dampers and torque rods end on their mounts.", "Jedes Teil ist mit dem verbunden, was es trägt: Die Querträger laufen in die Stege der Längsträger, Batteriekasten, Kraftstofftank und Luftkessel sitzen auf Konsolen am Rahmen, Kotflügel auf Streben, die Ersatzschurren in Schalen auf den hinteren Kotflügeln, der Leuchtenträger auf den Unterfahrschutzkonsolen, die Leiter auf einem Fuß an diesem Träger, der Trichter auf Konsolen seiner Stützen, der Heckrahmen auf Knotenblechen über dem Hilfsrahmen, der Auslauf in einem Kragen auf dem Drehkranz und der Trog in einer Gabel darunter; Stoßdämpfer und Lenker enden an ihren Lagern.") if not any(d["en"].startswith("Every part is joined") for d in S.v["50"]["decisions"]) else None
S.save()
