"""🏮 Round 3 pendant lamp: states every solid of every authored scale and list by hand and writes `volumes/appliances/pendant-lamp.json` (and returns it to the roots script)."""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-pendant-lamp-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

ROSE_M, CORD_M, BRASS, CAP_T, ENAMEL, GLASS_M = "rose-polymer", "textile-cord", "brass-lacquered", 0.5, "enamel-white", "bulb-glass"


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def th(s, region, layer=None, **extra):
    s = dict(s, region=region)
    if layer:
        s["layer"] = layer
    s.update(extra)
    return s


def st(s, member, part, **extra):
    s = dict(s, member=member, part=part)
    s.update(extra)
    return s


def plume(r1):
    return th(k.revolve("th-shade-plume", "shade", ENAMEL, "detail", 0.4, k.plume_patch(r1)), "shade", tags=["shade-plume"])


def text(path):
    return json.load(open(os.path.join(HERE, path)))


def scale50():
    body = [
        k.revolve("rose", "rose", ROSE_M, "body", 0.7, k.ROSE["50"]),
        k.revolve("rose-grip", "rose", ROSE_M, "detail", 0.7, k.ROSE_GRIP),
        k.rod("cord", "cord", CORD_M, "body", 0.15, 0.004, k.CORD_L + 0.0025, k.CORD_R, slender=True),
        k.revolve("collar", "holder", BRASS, "body", 0.45, k.COLLAR["50"]),
        k.revolve("socket", "holder", BRASS, "body", 0.45, k.SOCKET["50"]),
        k.revolve("shade", "shade", ENAMEL, "body", 0.4, k.shade50()),
        k.revolve("bulb-cap", "bulb", BRASS, "body", CAP_T, k.CAP),
        k.revolve("bulb-glass", "bulb", GLASS_M, "body", 0.9, k.glass()),
    ]
    thermal = [
        ref("th-rose", "rose", region="rose", layer="rose-shell"),
        ref("th-rose-grip", "rose-grip", region="rose", layer="rose-shell"),
        ref("th-cord", "cord", region="cord", layer="sheath"),
        ref("th-collar", "collar", region="lampholder", layer="brass-shell"),
        ref("th-socket", "socket", region="lampholder", layer="brass-shell"),
        ref("th-bulb-cap", "bulb-cap", region="lampholder", layer="brass-shell"),
        ref("th-bulb-glass", "bulb-glass", region="bulb-glass", layer="envelope"),
        ref("th-shade", "shade", region="shade", layer="shade-steel"),
        plume(0.05),
    ]
    structure = [
        st(k.capsule("support-ceiling-hook", "rose", BRASS, "body", 0.3, (0.004, 0, -0.006), (0.004, 0, 0.006), 0.004), "rose", "support", anchor="ceiling-hook"),
        st(k.capsule("member-rose-hanger", "rose", BRASS, "body", 0.3, (0.006, 0, 0), (0.022, 0, 0), 0.004), "rose", "member"),
        st(k.rod("joint-cord", "cord", BRASS, "body", 0.5, -0.006, 0.004, 0.0045), "cord", "joint"),
        st(k.rod("member-cord", "cord", CORD_M, "body", 0.15, 0.0, k.CORD_L, k.CORD_R, slender=True), "cord", "member"),
        st(k.rod("joint-holder", "holder", BRASS, "body", 0.5, -0.0005, 0.0075, 0.0045), "holder", "joint"),
        st(k.rod("member-nipple", "holder", BRASS, "body", 0.45, 0.0075, 0.036, 0.005), "holder", "member"),
        ref("member-socket", "socket", member="holder", part="member"),
        st(k.rod("joint-shade", "shade", BRASS, "body", 0.5, -0.004, 0.004, 0.009), "shade", "joint"),
        ref("member-shade", "shade", member="shade", part="member"),
        st(k.rod("joint-bulb", "bulb", BRASS, "body", 0.5, 0.004, 0.024, 0.0145), "bulb", "joint"),
        ref("member-bulb-cap", "bulb-cap", member="bulb", part="member"),
        st(k.ellipsoid("mass-rose", "rose", ROSE_M, "detail", 0.7, (0.008, 0.022, 0.022), (0.012, 0, 0)), "rose", "mass"),
        st(k.ellipsoid("mass-cord", "cord", CORD_M, "detail", 0.15, (0.015, 0.0034, 0.0034), (0.3, 0, 0)), "cord", "mass"),
        st(k.ellipsoid("mass-holder", "holder", BRASS, "detail", 0.45, (0.008, 0.008, 0.008), (0.049, 0, 0)), "holder", "mass"),
        st(k.ellipsoid("mass-shade", "shade", ENAMEL, "detail", 0.4, (0.009, 0.009, 0.009), (0.068, 0, 0)), "shade", "mass"),
        st(k.ellipsoid("mass-bulb", "bulb", GLASS_M, "detail", 0.9, (0.009, 0.009, 0.009), (0.053, 0, 0)), "bulb", "mass"),
    ]
    return body, thermal, structure


def scale100():
    body = [
        k.revolve("rose", "rose", ROSE_M, "body", 0.7, k.ROSE["100"]),
        k.rod("cord", "cord", CORD_M, "body", 0.15, 0.004, k.CORD_L + 0.002, k.CORD_R, slender=True),
        k.revolve("collar", "holder", BRASS, "body", 0.45, k.COLLAR["100"]),
        k.revolve("socket", "holder", BRASS, "body", 0.45, k.SOCKET["100"]),
        k.revolve("shade", "shade", ENAMEL, "body", 0.4, k.shade100()),
        k.revolve("bulb", "bulb", GLASS_M, "body", 0.9, k.bulb100()),
    ]
    thermal = [
        ref("th-rose", "rose", region="rose"),
        ref("th-cord", "cord", region="cord"),
        ref("th-collar", "collar", region="lampholder"),
        ref("th-socket", "socket", region="lampholder"),
        ref("th-bulb", "bulb", region="bulb-glass"),
        ref("th-shade", "shade", region="shade"),
        plume(0.066),
    ]
    holder = [(0.0, 0.0), (0.0, 0.0075), (k.SOCKET_TOP, 0.0075), (k.SOCKET_TOP, k.SOCKET_R), (k.SOCKET_MOUTH, k.SOCKET_R), (k.SOCKET_MOUTH, 0.0)]
    structure = [
        st(k.capsule("support-ceiling-hook", "rose", BRASS, "body", 0.3, (0.0075, 0, -0.003), (0.0075, 0, 0.003), 0.0075), "rose", "support", anchor="ceiling-hook"),
        st(k.rod("member-cord", "cord", CORD_M, "body", 0.15, 0.0, k.CORD_L, k.CORD_R, slender=True), "cord", "member"),
        st(k.revolve("member-holder", "holder", BRASS, "body", 0.45, holder), "holder", "member"),
        st(k.rod("joint-shade", "shade", BRASS, "body", 0.5, -0.0075, 0.0075, 0.0105), "shade", "joint"),
        ref("member-shade", "shade", member="shade", part="member"),
        st(k.rod("member-bulb", "bulb", BRASS, "body", 0.45, 0.0, k.CAP_L, k.CAP_R), "bulb", "member"),
        st(k.ellipsoid("mass-holder", "holder", BRASS, "detail", 0.45, (0.0075, 0.0075, 0.0075), (0.049, 0, 0)), "holder", "mass"),
        st(k.ellipsoid("mass-shade", "shade", ENAMEL, "detail", 0.4, (0.0075, 0.0075, 0.0075), (0.068, 0, 0)), "shade", "mass"),
        st(k.ellipsoid("mass-bulb", "bulb", GLASS_M, "detail", 0.9, (0.0075, 0.0075, 0.0075), (0.053, 0, 0)), "bulb", "mass"),
    ]
    return body, thermal, structure


def scale200():
    body = [
        k.revolve("rose", "rose", ROSE_M, "body", 0.7, k.ROSE["200"]),
        k.rod("cord", "cord", CORD_M, "body", 0.15, 0.004, k.CORD_L + 0.002, k.CORD_R, slender=True),
        k.revolve("shade", "shade", ENAMEL, "body", 0.4, k.shade200()),
        k.revolve("bulb", "bulb", GLASS_M, "body", 0.85, k.bulb200()),
    ]
    lamp = [(-0.0005, 0.0), (-0.0005, 0.0165), (k.COLLAR_H, 0.0165), (k.COLLAR_H, 0.0155), (k.SOCKET_TOP + 0.0024, 0.0155), (k.SOCKET_TOP + 0.0024, k.SOCKET_R), (k.SOCKET_MOUTH, k.SOCKET_R), (k.SOCKET_MOUTH, 0.0)]
    thermal = [
        ref("th-rose", "rose", region="rose"),
        ref("th-cord", "cord", region="cord"),
        ref("th-shade", "shade", region="shade"),
        th(k.revolve("th-lampholder", "holder", BRASS, "body", 0.45, lamp), "lampholder"),
        th(k.revolve("th-bulb", "bulb", GLASS_M, "body", 0.85, k.glass200()), "bulb-glass"),
        plume(0.088),
    ]
    structure = [
        st(k.capsule("support-ceiling-hook", "rose", BRASS, "body", 0.3, (0.015, 0, -0.005), (0.015, 0, 0.005), 0.015), "rose", "support", anchor="ceiling-hook"),
        st(k.rod("member-cord", "cord", CORD_M, "body", 0.15, 0.0, k.CORD_L, k.CORD_R, slender=True), "cord", "member"),
        st(k.rod("member-holder", "holder", BRASS, "body", 0.45, 0.0, k.BULB_AT, 0.0155), "holder", "member"),
        ref("member-shade", "shade", member="shade", part="member"),
        st(k.rod("member-bulb", "bulb", BRASS, "body", 0.45, 0.0, 0.03, 0.0155), "bulb", "member"),
        st(k.ellipsoid("mass-shade", "shade", ENAMEL, "detail", 0.4, (0.015, 0.015, 0.015), (0.068, 0, 0)), "shade", "mass"),
    ]
    return body, thermal, structure


def volumes():
    texts = text("r3-pendant-lamp-texts.json")["volumes"]
    out = {}
    for key, fn in (("50", scale50), ("100", scale100), ("200", scale200)):
        body, thermal, structure = fn()
        out[key] = {"status": "authored", "denominator": int(key), "body": body, "thermal": thermal, "structure": structure, "decisions": texts[key]}
    out["500"] = {"status": "unavailable", "denominator": 500}
    return out


if __name__ == "__main__":
    v = volumes()
    if "--write" in sys.argv:
        with open(os.path.join(HERE, "volumes", "appliances", "pendant-lamp.json"), "w") as f:
            json.dump({"volumes": v}, f, ensure_ascii=False, indent=1)
            f.write("\n")
    print(json.dumps({s: {l: len(v[s][l]) for l in ("body", "thermal", "structure")} for s in ("50", "100", "200")}))
