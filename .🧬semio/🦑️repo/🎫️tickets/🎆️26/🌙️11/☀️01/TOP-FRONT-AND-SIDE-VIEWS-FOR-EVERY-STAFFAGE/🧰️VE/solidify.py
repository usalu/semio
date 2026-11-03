"""One-off patch of the builders: replace helper-bone / followTip bindings by solid extruded shapes on the original bones."""
import re

LIB = '''

# ---------------------------------------------------------------- solid (extruded) rebinding
GEOM_KEYS = {"kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "d", "solid"}


def ext(profile, z0, z1):
    g = dict(profile)
    g.pop("followTip", None)
    g["solid"] = {"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}
    return g


def rebind(shapes, rules):
    """rules: id -> None (drop) | (bone, geometry[, overrides])"""
    out = []
    for s in shapes:
        rule = rules.get(s["id"], 0)
        if rule == 0:
            out.append(s)
            continue
        if rule is None:
            continue
        new = dict(rule[1])
        new.update({k: v for k, v in s.items() if k not in GEOM_KEYS})
        new["bone"] = rule[0]
        if len(rule) > 2:
            new.update(rule[2])
        out.append(new)
    return out


def apply_rules(asset, rules):
    for scale, views in rules.items():
        for view, table in views.items():
            geo = asset["representations"][scale]["geometry"]
            geo[view] = rebind(geo[view], table)
'''
s = open("lib.py", encoding="utf-8").read()
if "def rebind" not in s:
    open("lib.py", "a", encoding="utf-8").write(LIB)

RULES = {}
RULES["build_kick.py"] = '''
STEM = cap(0, 0, 0.8, 0, 0.019, 0.015)
BLADE = cap(0, 0, 0.03, -0.192, 0.0085)


def kick(grip_r, lever=True):
    f = {"stem": ("stem", ext(STEM, -0.019, 0.019)), "fork-blade": ("fork", ext(BLADE, 0.045, 0.062)),
         "bar": ("stem", ext(rect(0.785, -0.014, 0.03, 0.028, 0.012), -0.19, 0.19)),
         "grip-l": ("stem", ext(ell(0.8, 0, grip_r), 0.12, 0.215)), "grip-r": ("stem", ext(ell(0.8, 0, grip_r), -0.215, -0.12))}
    if lever:
        f["brake-lever"] = ("stem", ext(cap(0.8036, -0.0106, 0.7762, -0.0976, 0.008), 0.16, 0.18))
    return f


apply_rules(a, {
    "50": {"front": kick(0.028), "plan": kick(0.028)},
    "100": {"front": kick(0.028, False), "plan": kick(0.028, False)},
    "200": {
        "front": {"fork-stem": ("stem", ext(cap(0, 0, 0.8, 0, 0.025), -0.025, 0.025)), "fork-blade": ("fork", ext(BLADE, 0.0, 0.06)), "bars": ("stem", ext(rect(0.77, -0.029, 0.06, 0.058, 0.02), -0.215, 0.215))},
        "plan": {"fork-stem": ("stem", ext(cap(0, 0, 0.8, 0, 0.022), -0.022, 0.022)), "fork-blade": ("fork", ext(cap(0, 0, 0.03, -0.192, 0.012), 0.028, 0.052)), "bars": ("stem", ext(rect(0.77, -0.029, 0.06, 0.058, 0.02), -0.215, 0.215))},
    },
})
'''
RULES["build_bike.py"] = '''
STEM = cap(0, 0, -0.259, 0.133, 0.0125)
BAR = cap(-0.259, 0.133, -0.329, 0.108, 0.0125)
FORK = cap(0.048, -0.132, 0.261, -0.587, 0.007)
CRN = cap(0, 0, 0.17, 0, 0.012)
CRF = cap(0, 0, -0.17, 0, 0.012)
STD = cap(0, 0, 0.32, 0, 0.007)


def common(arms=True):
    f = {"fork-l": ("steer", ext(FORK, 0.038, 0.052)), "fork-r": ("steer", ext(FORK, -0.052, -0.038)),
         "stand-l": ("stand", ext(STD, 0.068, 0.082)), "stand-r": ("stand", ext(STD, -0.082, -0.068))}
    if arms:
        f["arm-n"] = ("crank", ext(CRN, -0.08, -0.06))
        f["arm-f"] = ("crank", ext(CRF, 0.06, 0.08))
    return f


GRIP = ell(-0.329, 0.108, 0.016)
GRIPP = ell(-0.329, 0.108, 0.018)
SWEEP = cap(-0.259, 0.133, -0.309, 0.115, 0.0125)
f50 = common()
f50.update({"stem-low": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-high": None, "handlebar": ("steer", ext(BAR, -0.31, 0.31)),
            "grip-l": ("steer", ext(GRIP, 0.22, 0.31)), "grip-r": ("steer", ext(GRIP, -0.31, -0.22))})
p50 = common()
p50.update({"stem-head": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-bar": None,
            "handlebar-l": ("steer", ext(SWEEP, 0.0, 0.26), {"filled": True}), "handlebar-r": ("steer", ext(SWEEP, -0.26, 0.0), {"filled": True}),
            "grip-l": ("steer", ext(GRIPP, 0.26, 0.31)), "grip-r": ("steer", ext(GRIPP, -0.31, -0.26))})
f100 = common(False)
f100.update({"stem": ("steer", ext(STEM, -0.0125, 0.0125)), "handlebar": ("steer", ext(BAR, -0.31, 0.31))})
p100 = common()
p100.update({"stem-head": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-bar": None,
             "crossbar": ("steer", ext(ell(-0.259, 0.133, 0.0125), -0.18, 0.18)),
             "handlebar-l": ("steer", ext(SWEEP, 0.18, 0.26)), "handlebar-r": ("steer", ext(SWEEP, -0.26, -0.18)),
             "grip-l": ("steer", ext(GRIPP, 0.26, 0.31)), "grip-r": ("steer", ext(GRIPP, -0.31, -0.26))})
f200 = {"fork": ("steer", ext(FORK, -0.052, 0.052)), "steer-column": ("steer", ext(cap(0, -0.132, -0.259, 0.133, 0.02), -0.02, 0.02)),
        "handlebar": ("steer", ext(BAR, -0.31, 0.31)), "stand": ("stand", ext(STD, -0.082, 0.082))}
p200 = {"fork": ("steer", ext(FORK, 0.033, 0.057)), "fork-r": ("steer", ext(FORK, -0.057, -0.033)),
        "steer-column": ("steer", ext(cap(0, -0.132, -0.259, 0.133, 0.0125), -0.0125, 0.0125)),
        "handlebar": ("steer", ext(ell(-0.259, 0.133, 0.0125), -0.19, 0.19)),
        "sweep-l": ("steer", ext(cap(-0.259, 0.133, -0.329, 0.108, 0.016), 0.19, 0.31)), "sweep-r": ("steer", ext(cap(-0.259, 0.133, -0.329, 0.108, 0.016), -0.31, -0.19)),
        "stand": ("stand", ext(STD, 0.065, 0.085)), "stand-r": ("stand", ext(STD, -0.085, -0.065))}
apply_rules(a, {"50": {"front": f50, "plan": p50}, "100": {"front": f100, "plan": p100}, "200": {"front": f200, "plan": p200}})
'''
RULES["build_cargo.py"] = '''
CRN = cap(0, 0, 0.17, 0, 0.014)
CRF = cap(0, 0, -0.17, 0, 0.014)
STD = cap(0, 0, 0.3433, 0, 0.01)
PA = rect(0.12, -0.012, 0.1, 0.024, 0.004)
PB = rect(-0.19, -0.01, 0.06, 0.02, 0.004)
ARMS = {"arm-n": ("crank", ext(CRN, -0.115, -0.085)), "arm-f": ("crank", ext(CRF, 0.085, 0.115)),
        "pedal-a": ("crank", ext(PA, -0.19, -0.1)), "pedal-b": ("crank", ext(PB, 0.1, 0.19))}
LEGS = {"stand-leg-l": ("stand", ext(STD, 0.11, 0.13)), "stand-leg-r": ("stand", ext(STD, -0.13, -0.11)),
        "stand-foot": ("stand", ext(cap(0.3, 0, 0.3433, 0, 0.017), -0.15, 0.15))}
f50 = dict(ARMS)
f50.update(LEGS)
p50 = dict(ARMS)
p50.update(LEGS)
p50.pop("stand-foot")
f100 = {"pedal-a": ARMS["pedal-a"], "pedal-b": ARMS["pedal-b"], "stand-leg-l": LEGS["stand-leg-l"], "stand-leg-r": LEGS["stand-leg-r"]}
p100 = {"crank-arms": ARMS["arm-n"], "crank-arms-f": ARMS["arm-f"], "stand-leg-l": LEGS["stand-leg-l"], "stand-leg-r": LEGS["stand-leg-r"]}
f200 = {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.015), -0.13, 0.13))}
p200 = {"stand-l": ("stand", ext(cap(0, 0, 0.3433, 0, 0.012), 0.11, 0.13)), "stand-r": ("stand", ext(cap(0, 0, 0.3433, 0, 0.012), -0.13, -0.11))}
apply_rules(a, {"50": {"front": f50, "plan": p50}, "100": {"front": f100, "plan": p100}, "200": {"front": f200, "plan": p200},
                "500": {"front": {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.02), -0.13, 0.13))}, "plan": {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.03), 0.09, 0.15))}}})
'''
RULES["build_ms.py"] = '''
STD = cap(0, 0, 0.335, 0, 0.012)
LEGF = {"stand-leg-l": ("stand", ext(STD, 0.095, 0.125)), "stand-leg-r": ("stand", ext(STD, -0.125, -0.095))}


def seat(z, r=0.05):
    return ("seat", ext(rect(0, -0.12, 0.58, 0.195, r), -z, z))


apply_rules(a, {
    "50": {"front": LEGF, "plan": dict(LEGF, **{"seat-l": seat(0.155), "seat-c": None, "seat-r": None})},
    "100": {"front": LEGF, "plan": dict(LEGF, **{"seat-l": seat(0.16), "seat-r": None})},
    "200": {"front": {"stand": ("stand", ext(STD, -0.125, 0.125))}, "plan": {"stand-l": LEGF["stand-leg-l"], "stand-r": LEGF["stand-leg-r"], "seat": seat(0.15, 0.06)}},
    "500": {"plan": {"seat": seat(0.15, 0.06)}},
})
'''
RULES["build_tram.py"] = '''
def pan(r, hw):
    return {"pan-lower": ("pan-lower", ext(cap(0, 0, 1.9, 0, r), -r, r)), "pan-upper": ("pan-upper", ext(cap(0, 0, 1.9, 0, r), -r, r)),
            "pan-head": ("pan-head", ext(cap(-0.28, 0, 0.28, 0, 0.03), -hw, hw))}


apply_rules(a, {"50": {"front": pan(0.03, 0.6), "plan": pan(0.03, 0.6)}, "100": {"front": pan(0.035, 0.6), "plan": pan(0.035, 0.6)},
                "200": {"front": pan(0.05, 0.6), "plan": pan(0.05, 0.6)}, "500": {"front": pan(0.06, 0.6), "plan": pan(0.06, 0.6)}})
'''
RULES["build_car.py"] = '''
BON = poly([(-0.05, -0.07), (0.9, -0.07), (0.94, 0.03), (-0.05, 0.02)])
HAT = poly([(-0.05, -0.05), (0.8, -0.05), (0.8, 0.03), (0.74, 0.095), (-0.05, 0.08)])
HGL = poly([(0.03, -0.03), (0.45, -0.015), (0.45, 0.05), (0.03, 0.055)])


def plan_rules(nb, nh):
    t = {"bonnet-0": ("bonnet", ext(BON, -0.68, 0.68)), "hatch-0": ("hatch", ext(HAT, -0.7, 0.7))}
    t.update({f"bonnet-{i}": None for i in range(1, nb)})
    t.update({f"hatch-{i}": None for i in range(1, nh)})
    return t


front_rules = {"bonnet-panel": ("bonnet", ext(BON, -0.7, 0.7)), "hatch-panel": ("hatch", ext(HAT, -0.58, 0.58)), "hatch-glass": ("hatch", ext(HGL, -0.48, 0.48))}
apply_rules(a, {"50": {"front": front_rules, "plan": plan_rules(5, 5)}, "100": {"front": front_rules, "plan": plan_rules(3, 4)},
                "200": {"front": front_rules, "plan": plan_rules(3, 3)}, "500": {"front": front_rules, "plan": plan_rules(3, 3)}})
'''

for name, rules in RULES.items():
    s = open(name, encoding="utf-8").read()
    if "apply_rules(" not in s:
        s = s.replace("\nsave(AID, doc)", rules + "\nsave(AID, doc)")
    if name == "build_kick.py":
        s = re.sub(r'a\["rig"\]\["bones"\]\.append\(\{.*?\}\)\n', "", s, flags=re.S, count=1)
    if name == "build_bike.py":
        s = re.sub(r"def bone\(.*?\n\n\n", "\n", s, flags=re.S, count=1)
        s = re.sub(r'bone\("[^\n]*\n', "", s)
    if name == "build_cargo.py":
        s = re.sub(r"for id, en, de, ang in .*?\n\n", "\n", s, flags=re.S, count=1)
    open(name, "w", encoding="utf-8").write(s)
print("patched")
