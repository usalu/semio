"""W3-THIRDPARTY-3: generates third-party-w3-3.json for EN 1998 (design and elastic spectrum per ground type with the registry Table 3.2/3.3 parameters, shear-frame periods and participation, modal base shear with SRSS and CQC) and restores the cases dropped by W3-THIRDPARTY-2 (unequal-span delta_0 via PyNite on a finer mesh, off-grid tandem/LM71 positions on a finer travel step).

Run: /home/deck/semio/.venv/bin/python -I w3-thirdparty-3-gen.py [--write] [en1998 en1994 en1991]
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
ART = ROOT / "🗿️artifacts"
FOLDER = {"en1998": "🫨️en1998", "en1991": "🏋️en1991", "en1994": "🧩️en1994", "en1996": "🪨️en1996"}
TEMPLATES = {}
for f in sorted(ART.glob("*/" + SUB + "/third-party*.json")):
    if f.name in ("third-party-w3-3.json",):
        continue
    for c in json.loads(f.read_text(encoding="utf8"))["cases"]:
        TEMPLATES[c["id"]] = c
doc = json.loads((ROOT / "⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json").read_text(encoding="utf8"))


def rows(o):
    if isinstance(o, dict):
        if str(o.get("id", "")).startswith(("ndp.en1998.t32.", "ndp.en1998.t33.")):
            yield o
        for v in o.values():
            yield from rows(v)
    elif isinstance(o, list):
        for v in o:
            yield from rows(v)


NDP = {r["id"]: r["recommended"] for r in rows(doc)}
CITE = {"standard": "EN 1998-1", "edition": "2004", "clause": "3.2.2.5(4) Eq. 3.13-3.16; 3.2.2.2 Eq. 3.2-3.5; parameters Table 3.2 / 3.3 of the registry (status unverified)", "annex": "en"}
NUM, SYM = ({"oracle": "norm-eurocode-scipy-numerics", "adapter": "numerical", "kind": "independent-formulation"}, {"oracle": "norm-eurocode-sympy", "adapter": "sympy", "kind": "independent-formulation"})


def quantity(unit, profile="numerical-v1"):
    return {"value": 0.0, "unit": unit, "profile": profile}


def spectrum_cases():
    out = []
    qs = {"a": 1.5, "b": 2.0, "c": 3.0, "d": 4.0, "e": 1.5}
    for table, label in (("t32", "type1"), ("t33", "type2")):
        for cls in "abcde":
            p = {k: NDP[f"ndp.en1998.{table}.{cls}.{k}"] for k in ("s", "tb", "tc", "td")}
            for tag, t in {"rising": 0.5 * p["tb"], "plateau": 0.5 * (p["tb"] + p["tc"]), "velocity": 0.5 * (p["tc"] + p["td"]), "displacement": 1.5 * p["td"], "floor": 6.0}.items():
                out.append({"id": f"en1998.3.2.2.5.w3-3-spectrum-{label}-ground-{cls}-{tag}", "check": "en1998.3.2.2.5.design-spectrum", "citation": CITE, "inputs": {"a_g": 1.5, "S": p["s"], "T_B": p["tb"], "T_C": p["tc"], "T_D": p["td"], "q": qs[cls], "beta": 0.2, "T": t}, "expected": {"S_d": quantity("m/s^2", "eurocode-quantities-v1"), "S_e": quantity("m/s^2", "eurocode-quantities-v1")}, "references": [SYM, NUM]})
    return out


FRAMES = {
    "two-storey-stiff-base": ([150000.0, 120000.0], [90000000.0, 60000000.0]),
    "three-storey-soft-first": ([100000.0, 100000.0, 100000.0], [30000000.0, 80000000.0, 80000000.0]),
    "four-storey-uniform": ([200000.0] * 4, [150000000.0] * 4),
    "five-storey-tapered": ([300000.0, 280000.0, 260000.0, 240000.0, 200000.0], [220000000.0, 200000000.0, 170000000.0, 140000000.0, 100000000.0]),
}


def modal_cases():
    out = []
    for name, (m, k) in FRAMES.items():
        n = len(m)
        out.append({"id": f"en1998.4.3.3.w3-3-shear-building-{name}", "check": "en1998.4.3.shear-building-modal", "citation": {"standard": "EN 1998-1", "edition": "2004", "clause": "4.3.3.1 modal response spectrum analysis; 4.3.3.2.2 Eq. 4.6 vs eigenvalue periods"}, "inputs": {"m": m, "k": k}, "expected": {f"T_{j + 1}": quantity("s") for j in range(n)} | {"Gamma_1": quantity("1"), "m_eff_1_fraction": quantity("1")}, "references": [NUM]})
        out.append({"id": f"en1998.4.3.3.w3-3-shear-building-{name}-periods", "check": "en1998.4.3.shear-building-modal", "citation": {"standard": "EN 1998-1", "edition": "2004", "clause": "4.3.3.1 modal response spectrum analysis periods"}, "inputs": {"m": m, "k": k}, "expected": {f"T_{j + 1}": quantity("s") for j in range(n)}, "references": [NUM, SYM]})
    for name, (m, k), xi, q, cls in (("two-storey", FRAMES["two-storey-stiff-base"], 0.05, 2.0, "b"), ("three-storey", FRAMES["three-storey-soft-first"], 0.05, 3.0, "c"), ("four-storey-low-damping", FRAMES["four-storey-uniform"], 0.02, 2.0, "d"), ("five-storey", FRAMES["five-storey-tapered"], 0.05, 4.0, "b"), ("five-storey-high-damping", FRAMES["five-storey-tapered"], 0.10, 1.5, "e")):
        p = {key: NDP[f"ndp.en1998.t32.{cls}.{key}"] for key in ("s", "tb", "tc", "td")}
        n = len(m)
        out.append({"id": f"en1998.4.3.3.3.w3-3-modal-base-shear-{name}", "check": "en1998.4.3.3.3.modal-base-shear", "citation": {"standard": "EN 1998-1", "edition": "2004", "clause": "4.3.3.1 modal response spectrum analysis; 4.3.3.3.2 combination of modal responses (SRSS, CQC); effective modal mass 4.3.3.1(3)"}, "inputs": {"m": m, "k": k, "a_g": 1.5, "S": p["s"], "T_B": p["tb"], "T_C": p["tc"], "T_D": p["td"], "q": q, "beta": 0.2, "xi": xi}, "expected": {f"F_b_{j + 1}": quantity("N") for j in range(n)} | {"V_srss": quantity("N"), "V_cqc": quantity("N")}, "references": [NUM, SYM]})
    return out


def clone(template, cid, refs=None, **inputs):
    t = TEMPLATES[template]
    c = copy.deepcopy(t)
    c["id"], c["inputs"] = cid, t["inputs"] | inputs
    if refs:
        c["references"] = refs
    for q in c["expected"].values():
        q["value"] = 0.0
    return c


def fresh(cid, check, clause, inputs, outputs, refs, unit="1"):
    return {"id": cid, "check": check, "citation": {"standard": "EN 1996-1-1", "edition": "2005", "clause": clause}, "inputs": inputs, "expected": {k: quantity(u) for k, u in outputs.items()}, "references": refs}


def en1991_cases():
    out = []
    for n, span in {"10_0": 10.0, "17_5": 17.5}.items():
        out.append(clone("en1991.mechanics.lm1-tandem-span-6_0", f"en1991.mechanics.w3-3-lm1-tandem-span-{n}-fine-step", span=span, step=0.1))
    for n, (span, x) in {"20_0-x-10_0": (20.0, 10.0), "15_0-x-7_5": (15.0, 7.5), "21_0-x-10_5": (21.0, 10.5)}.items():
        out.append(clone("en1991.mechanics.tandem-section-envelope-span-12_0-x-3_6", f"en1991.mechanics.w3-3-tandem-section-span-{n}-fine-step", span=span, x=x, step=0.05))
    for n, span in {"10_0": 10.0, "14_0": 14.0}.items():
        out.append(clone("en1991.mechanics.rail-lm71-mid-moment-span-24_0", f"en1991.mechanics.w3-3-rail-lm71-span-{n}-fine-step", span=span, step=0.1))
    return out


def en1994_cases():
    pynite = next(r for r in TEMPLATES["en1994.mechanics.two-span-udl"]["references"] if r["adapter"] == "pynite")
    out = []
    for n, spans in {"5-8": [5.0, 8.0], "7-4": [7.0, 4.0]}.items():
        out.append(clone("en1994.mechanics.two-span-udl", f"en1994.mechanics.w3-3-two-span-unequal-{n}-fine-mesh-delta", [pynite], spans=spans, w=[12000.0, 12000.0], mesh=40))
    return out


YL = "en1996.E.yield-line-pressure"
SCIPY = [NUM]
CITE_E = "Annex E Johansen yield-line mechanics (stand-in of the unkeyed coefficient table, M-31)"


def en1996_cases():
    out = []
    panels = {"ssss-square": (1, 1, 1, 1, 3.0, 3.0, 1200.0, 1200.0), "ssss-wall": (1, 1, 1, 1, 4.0, 2.5, 1500.0, 3000.0), "xxxx-wall": (2, 2, 2, 2, 4.0, 2.5, 1500.0, 3000.0), "bottom-fixed": (1, 2, 1, 1, 3.6, 2.8, 900.0, 1800.0), "sides-fixed": (1, 1, 2, 2, 5.0, 2.6, 1000.0, 2500.0), "top-simple-three-fixed": (1, 2, 2, 2, 3.0, 4.0, 1800.0, 900.0)}
    for n, (t, b, l, r, length, height, m1, m2) in panels.items():
        out.append(fresh(f"en1996.E.w3-3-yield-line-{n}", YL, CITE_E, {"top": t, "bottom": b, "left": l, "right": r, "l": length, "h": height, "m1": m1, "m2": m2}, {"w_Rd": "Pa"}, SCIPY))
    for n, (t, b, l, r, mu, ratio) in {"ssss-mu-0.5-h-1": (1, 1, 1, 1, 0.5, 1.0), "ssss-mu-0.25-h-0.75": (1, 1, 1, 1, 0.25, 0.75), "xxxx-mu-1-h-1.5": (2, 2, 2, 2, 1.0, 1.5), "sxss-mu-0.4-h-0.5": (1, 2, 1, 1, 0.4, 0.5)}.items():
        out.append(fresh(f"en1996.E.w3-3-alpha1-standin-{n}", "en1996.E.alpha1-standin", CITE_E, {"top": t, "bottom": b, "left": l, "right": r, "mu": mu, "h_over_l": ratio}, {"alpha1": "1"}, SCIPY))
    for n, kw in {"short-stiff": dict(h_ef=2.0, t=0.3, e_mk=0.03), "tall-thin": dict(h_ef=3.2, t=0.15, e_mk=0.02), "high-modulus": dict(E=8000000000.0, f_k=8000000.0, h_ef=2.8, t=0.2, e_mk=0.04), "low-eccentricity": dict(h_ef=2.6, t=0.2, e_mk=0.005)}.items():
        out.append(clone("en1996.G.phi-m-lambda-0.41", f"en1996.G.w3-3-phi-m-{n}", **kw))
    for n, kw in {"short-wall": dict(l=1.5, e=0.1, N_Ed=150000.0), "long-wall-high-load": dict(l=6.0, e=0.5, N_Ed=900000.0), "wide-eccentric": dict(l=4.0, e=1.2, N_Ed=400000.0)}.items():
        out.append(clone("en1996.6.2.shear-wall-concentric", f"en1996.6.2.w3-3-shear-wall-{n}", **kw))
    return out


CASES = {"en1998": spectrum_cases() + modal_cases(), "en1991": en1991_cases(), "en1994": en1994_cases(), "en1996": en1996_cases()}


def main(argv):
    rc = 0
    only = [a for a in argv if a in CASES]
    for code, cases in CASES.items():
        if only and code not in only:
            continue
        d = {"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": code, "annex": "de", "cases": cases}
        H.validate(d)
        results = H.run(d, bless=True)
        bad = [r for r in results if r["failures"]]
        for r in bad:
            rc = 1
            print("SPREAD", r["id"], *r["failures"], sep="\n    ")
        print(code, len(cases), "cases", len(cases) - len(bad), "agree", flush=True)
        if "--write" in argv:
            (ART / FOLDER[code] / SUB / "third-party-w3-3.json").write_text(json.dumps(d, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
