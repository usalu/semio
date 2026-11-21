"""🏮 Round 5 pendant lamp: keeps the hand-stated 1:50, 1:100 and 1:200 lists of `r3-pendant-lamp-model.py` and states the 1:500 massing anew by hand (dome shell, not a cone), then writes `volumes/appliances/pendant-lamp.json`."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model3", os.path.join(HERE, "r3-pendant-lamp-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
k = M.k

def shade500():
    """⛑️ The dome of the 1:50 Bezier at six stations as a 4 mm shell with its neck and a square-cut rim: smooth, never a cone."""
    outer = k.dome_points(5)
    inner = list(reversed(k.dome_points(5, 1.0, 0.004)))
    return [(-0.03, 0.0), (-0.03, 0.0155), (0.0, 0.0155)] + outer + inner + [(0.004, 0.0)]


def scale500():
    body = [
        k.revolve("rose", "rose", M.ROSE_M, "body", 0.7, [(0.0, 0.0), (0.0, 0.05), (0.024, 0.05), (0.03, 0.044), (0.03, 0.0)]),
        k.rod("cord", "cord", M.CORD_M, "body", 0.15, 0.004, k.CORD_L + 0.002, k.CORD_R, slender=True),
        k.revolve("shade", "shade", M.ENAMEL, "body", 0.4, shade500()),
        k.revolve("bulb", "bulb", M.GLASS_M, "body", 0.85, [(-0.035, 0.0), (-0.035, 0.02), (0.026, 0.02), (0.026, 0.014), (0.05, 0.027), (0.074, 0.03), (0.098, 0.016), (0.105, 0.0)]),
    ]
    lamp = [(-0.0005, 0.0), (-0.0005, 0.02), (k.SOCKET_MOUTH, 0.02), (k.SOCKET_MOUTH, 0.0)]
    thermal = [
        M.ref("th-rose", "rose", region="rose"),
        M.ref("th-cord", "cord", region="cord"),
        M.ref("th-shade", "shade", region="shade"),
        M.th(k.revolve("th-lampholder", "holder", M.BRASS, "body", 0.45, lamp), "lampholder"),
        M.th(k.revolve("th-bulb", "bulb", M.GLASS_M, "body", 0.85, [(0.026, 0.0), (0.026, 0.014), (0.05, 0.027), (0.074, 0.03), (0.098, 0.016), (0.105, 0.0)]), "bulb-glass"),
        M.plume(0.08),
    ]
    structure = [
        M.st(k.capsule("support-ceiling-hook", "rose", M.BRASS, "body", 0.3, (0.012, 0, -0.008), (0.012, 0, 0.008), 0.012), "rose", "support", anchor="ceiling-hook"),
        M.st(k.rod("member-cord", "cord", M.CORD_M, "body", 0.15, 0.0, k.CORD_L, k.CORD_R, slender=True), "cord", "member"),
        M.st(k.rod("member-holder", "holder", M.BRASS, "body", 0.45, 0.0, k.SOCKET_MOUTH, 0.02), "holder", "member"),
        M.ref("member-shade", "shade", member="shade", part="member"),
        M.st(k.rod("member-bulb", "bulb", M.BRASS, "body", 0.45, 0.0, k.CAP_L, 0.0145), "bulb", "member"),
        M.st(k.ellipsoid("mass-shade", "shade", M.ENAMEL, "detail", 0.4, (0.02, 0.02, 0.02), (0.068, 0, 0)), "shade", "mass"),
    ]
    return body, thermal, structure


def main():
    v = M.volumes()
    b, t, s = scale500()
    v["500"] = {"status": "authored", "denominator": 500, "body": b, "thermal": t, "structure": s, "decisions": json.load(open(os.path.join(HERE, "r5-pendant-lamp-texts.json"), encoding="utf-8"))["500"]}
    with open(os.path.join(HERE, "volumes", "appliances", "pendant-lamp.json"), "w") as f:
        json.dump({"volumes": v}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print({d: {l: len(v[d][l]) for l in ("body", "thermal", "structure")} for d in v})


if __name__ == "__main__":
    main()
