"""W3-THIRDPARTY-2: generates third-party-w3-2.json for EN 1990, 1991, 1993, 1994, 1995, 1998 and 1999 by cloning the shape of an existing wired case (same check, same quantities, same references) with new inputs; values are blessed by the first reference through the oracle harness, which fails on any spread between the references.

Run: /home/deck/semio/.venv/bin/python -I w3-thirdparty-2-gen.py [--write] [en1990 ...]
"""

import copy
import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
SPEC = importlib.util.spec_from_file_location("h", ROOT / "🔮️oracles/🏃️harness/🐍️.py")
H = importlib.util.module_from_spec(SPEC)
sys.modules["h"] = H
SPEC.loader.exec_module(H)
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
FOLDER = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1998": "🫨️en1998", "en1999": "🪶️en1999"}
ART = ROOT / "🗿️artifacts"
TEMPLATES = {}
for f in sorted(ART.glob("*/" + SUB + "/third-party*.json")):
    if f.name == "third-party-w3-2.json":
        continue
    for c in json.loads(f.read_text(encoding="utf8"))["cases"]:
        TEMPLATES[c["id"]] = c


def clone(code, template, cid, clause, **inputs):
    """A case with the expected shape and references of `template` and `inputs` overriding its inputs."""
    t = TEMPLATES[template]
    c = copy.deepcopy(t)
    c["id"], c["inputs"] = cid, t["inputs"] | inputs
    c["citation"] = t["citation"] | ({"clause": clause} if clause else {})
    for q in c["expected"].values():
        q["value"] = 0.0
    return c


def rolled(code, name, h, b, tf, tw, r, tpl="en1993.section.heb240"):
    return clone(code, tpl, f"{code}.section.{name}", None, h=h, b=b, tf=tf, tw=tw, r=r)


E1990, E1991, E1993, E1994, E1995, E1998, E1999 = ([] for _ in range(7))
ENV = "en1990.mechanics.unequal-spans-gamma-arrangement-envelope"
for n, (spans, g, q, gi) in {"equal-short": ([4.0, 4.0], 8000, 5000, 1.0), "long-second-span": ([6.0, 9.0], 18000, 12000, 1.0), "short-second-span": ([8.0, 5.0], 10000, 30000, 1.0), "gamma-g-inf-09": ([7.0, 7.0], 14000, 20000, 0.9)}.items():
    E1990.append(clone("en1990", ENV, f"en1990.mechanics.w3-2-span-envelope-{n}", None, spans=spans, g=g, q=q, gamma_G_inf=gi))
for n, span in {"4_2": 4.2, "12_0": 12.0, "20_4": 20.4}.items():
    E1991.append(clone("en1991", "en1991.mechanics.lm1-tandem-span-6_0", f"en1991.mechanics.w3-2-lm1-tandem-span-{n}", None, span=span))
for n, (span, Q) in {"20_0": (20.0, 250000.0), "32_0": (32.0, 250000.0)}.items():
    E1991.append(clone("en1991", "en1991.mechanics.rail-lm71-mid-moment-span-24_0", f"en1991.mechanics.w3-2-rail-lm71-span-{n}", None, span=span, Q=Q))
for n, (span, x) in {"9_0-x-2_7": (9.0, 2.7), "18_0-x-5_4": (18.0, 5.4), "24_0-x-12_0": (24.0, 12.0)}.items():
    E1991.append(clone("en1991", "en1991.mechanics.tandem-section-envelope-span-12_0-x-3_6", f"en1991.mechanics.w3-2-tandem-section-span-{n}", None, span=span, x=x))
for name, dims in {"hea200": (0.19, 0.2, 0.01, 0.0065, 0.018), "heb400": (0.4, 0.3, 0.024, 0.0135, 0.027), "ipe200": (0.2, 0.1, 0.0085, 0.0056, 0.012), "ipe450": (0.45, 0.19, 0.0146, 0.0094, 0.021), "ipe600": (0.6, 0.22, 0.019, 0.012, 0.024)}.items():
    E1993.append(rolled("en1993", name, *dims))
E1993.append(clone("en1993", "en1993.section.rhs-200x100x8", "en1993.section.rhs-300x200x10", None, h=0.3, b=0.2, t=0.01, r_out=0.02))
E1993.append(clone("en1993", "en1993.section.rhs-200x100x8", "en1993.section.rhs-100x100x5", None, h=0.1, b=0.1, t=0.005, r_out=0.01))
E1993.append(clone("en1993", "en1993.section.chs-139.7x5", "en1993.section.chs-273x8", None, d=0.273, t=0.008))
E1993.append(clone("en1993", "en1993.section.chs-139.7x5", "en1993.section.chs-60.3x3.2", None, d=0.0603, t=0.0032))
E1993.append(clone("en1993", "en1993.section.channel-pfc200", "en1993.section.channel-pfc300", None, h=0.3, b=0.1, tf=0.0165, tw=0.01, r=0.016))
E1993.append(clone("en1993", "en1993.section.angle-l100x10", "en1993.section.angle-l150x12", None, h=0.15, b=0.15, t=0.012, r_root=0.016, r_toe=0.008))
E1993.append(clone("en1993", "en1993.section.mono-i-plate-girder", "en1993.section.mono-i-light-girder", None, h=0.5, b_t=0.2, b_b=0.25, t_ft=0.012, t_fb=0.016, tw=0.008))
for n, kw in {"tall": dict(h=5.0, span=8.0, H=25000.0), "stiff-beam": dict(h=3.5, span=6.0, H=10000.0, I_beam=0.0016712), "pinned-like-beam": dict(h=4.0, span=10.0, H=5000.0, I_beam=0.00004178)}.items():
    E1993.append(clone("en1993", "en1993.mechanics.portal-sway", f"en1993.mechanics.w3-2-portal-sway-{n}", None, **kw))
for n, kw in {"alpha-cr-eigen-tall": dict(h=5.0, span=8.0), "alpha-cr-eigen-heavy-gravity": dict(V=4000000.0, I_beam=0.0004)}.items():
    E1993.append(clone("en1993", "en1993.5.2.alpha-cr-eigen-flexible-beam", f"en1993.5.2.w3-2-{n}", None, **kw))
CE = "en1994.5.4.composite-elastic-ipe300-short-term"
for n, kw in {"ipe400-short": dict(h=0.4, b=0.18, tf=0.0135, tw=0.0086, r=0.0, b_eff=2.5, h_c=0.15), "ipe500-long": dict(h=0.5, b=0.2, tf=0.016, tw=0.0102, r=0.0, b_eff=3.0, h_c=0.14, E_c=11000000000.0), "hea300-rounded-c25": dict(h=0.29, b=0.3, tf=0.014, tw=0.0085, r=0.027, b_eff=1.8, h_c=0.12, E_c=31000000000.0)}.items():
    E1994.append(clone("en1994", CE if "rounded" not in n else "en1994.5.4.composite-elastic-ipe300-rounded", f"en1994.5.4.w3-2-composite-elastic-{n}", None, **kw))
BC = "en1994.mechanics.two-span-udl"
for n, kw in {"unequal-spans-udl": dict(spans=[5.0, 8.0], w=[12000.0, 12000.0]), "pattern-second-span": dict(spans=[6.0, 6.0], w=[0.0, 18000.0])}.items():
    E1994.append(clone("en1994", BC if "udl" in n else "en1994.mechanics.two-span-pattern-first-span", f"en1994.mechanics.w3-2-two-span-{n}", None, **kw))
    if "udl" in n:
        E1994[-1]["expected"].pop("delta_0")
E1994.append(clone("en1994", "en1994.mechanics.ss-beam-deflection", "en1994.mechanics.w3-2-ss-beam-long", None, spans=[9.0], w=[22000.0]))
E1994.append(clone("en1994", ENV, "en1994.mechanics.w3-2-span-envelope-long", None, E=210000000000.0, A=0.00538, I=8.356e-05, spans=[9.0, 9.0], g=25000.0, q=18000.0))
for n, kw in {"unequal-spans-udl": dict(spans=[4.0, 5.5], w=[3000.0, 3000.0]), "pattern-second-span": dict(spans=[4.5, 4.5], w=[0.0, 4000.0])}.items():
    E1995.append(clone("en1995", "en1995.mechanics.two-span-udl" if "udl" in n else "en1995.mechanics.two-span-pattern-first-span", f"en1995.mechanics.w3-2-two-span-{n}", None, **kw))
E1995.append(clone("en1995", "en1995.mechanics.ss-beam-deflection", "en1995.mechanics.w3-2-ss-beam-long", None, spans=[5.5], w=[2500.0]))
E1995.append(clone("en1995", "en1995.mechanics.two-span-midspan-forces", "en1995.mechanics.w3-2-two-span-midspan-asymmetric", None, spans=[4.0, 6.0], P_mid=[5000.0, 8000.0]))
for n, kw in {"short": dict(length=3.0, ei_l=60000.0, m=90.0), "long": dict(length=7.5, ei_l=480000.0, m=210.0), "light": dict(length=5.0, ei_l=200000.0, m=60.0)}.items():
    E1995.append(clone("en1995", "en1995.7.5.natural-frequency", f"en1995.7.5.w3-2-natural-frequency-{n}", None, **kw))
E1995.append(clone("en1995", ENV, "en1995.mechanics.w3-2-span-envelope-timber-unequal", None, E=11000000000.0, A=0.03, I=0.000225, spans=[4.0, 5.5], g=1200.0, q=3000.0))
LF2, LF3 = "en1998.mechanics.lateral-force-frame-two-storey", "en1998.mechanics.lateral-force-frame-three-storey"
E1998.append(clone("en1998", LF2, "en1998.mechanics.w3-2-lateral-frame-two-storey-wide", None, h=[3.5, 3.5], span=9.0, F=[30000.0, 55000.0]))
E1998.append(clone("en1998", LF2, "en1998.mechanics.w3-2-lateral-frame-two-storey-soft-beam", None, I_beam=0.0003, F=[45000.0, 70000.0]))
E1998.append(clone("en1998", LF3, "en1998.mechanics.w3-2-lateral-frame-three-storey-tall-ground", None, h=[5.0, 3.5, 3.5], F=[25000.0, 60000.0, 95000.0]))
E1998.append(clone("en1998", LF3, "en1998.mechanics.w3-2-lateral-frame-three-storey-flexible-beam", None, I_beam=0.0006, span=8.0))
FB = "en1999.6.3.1.flexural-buckling-class-a-L4"
for n, kw in {"short-column": dict(L=1.2), "slender-column": dict(L=7.0), "class-b-wide-flange": dict(h=0.15, b=0.15, tf=0.012, tw=0.008, L=3.0, alpha=0.32, lambda_0=0.0), "high-strength-alloy": dict(f_o=300000000.0, L=3.5)}.items():
    E1999.append(clone("en1999", FB, f"en1999.6.3.1.w3-2-flexural-buckling-{n}", None, **kw))
E1999.append(clone("en1999", "en1999.section.i-extruded-200x100", "en1999.section.i-extruded-300x150", None, h=0.3, b_t=0.15, b_b=0.15, t_ft=0.014, t_fb=0.014, tw=0.008))
E1999.append(clone("en1999", "en1999.section.i-extruded-200x100", "en1999.section.i-mono-extruded", None, h=0.25, b_t=0.08, b_b=0.14, t_ft=0.008, t_fb=0.012, tw=0.007))
E1999.append(clone("en1999", "en1999.section.chs-extruded-100x5", "en1999.section.chs-extruded-160x8", None, d=0.16, t=0.008))
E1999.append(clone("en1999", "en1999.section.channel-extruded-100", "en1999.section.channel-extruded-200", None, h=0.2, b=0.08, tf=0.01, tw=0.007, r=0.006))
E1999.append(clone("en1999", "en1999.section.tee-extruded-60", "en1999.section.tee-extruded-100", None, h=0.1, b=0.1, tf=0.008, tw=0.007, r=0.004))
E1999.append(clone("en1999", "en1999.section.angle-extruded-60x6", "en1999.section.angle-extruded-100x8", None, h=0.1, b=0.1, t=0.008, r_root=0.008, r_toe=0.004))
CASES = {"en1990": E1990, "en1991": E1991, "en1993": E1993, "en1994": E1994, "en1995": E1995, "en1998": E1998, "en1999": E1999}


def main(argv):
    rc = 0
    only = [a for a in argv if a in CASES]
    for code, cases in CASES.items():
        if only and code not in only:
            continue
        doc = {"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": code, "annex": "de", "cases": cases}
        H.validate(doc)
        results = H.run(doc, bless=True)
        bad = [r for r in results if r["failures"]]
        for r in bad:
            rc = 1
            print("SPREAD", r["id"], *r["failures"], sep="\n    ")
        print(code, len(cases), "cases", len(cases) - len(bad), "agree", flush=True)
        if "--write" in argv:
            (ART / FOLDER[code] / SUB / "third-party-w3-2.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
