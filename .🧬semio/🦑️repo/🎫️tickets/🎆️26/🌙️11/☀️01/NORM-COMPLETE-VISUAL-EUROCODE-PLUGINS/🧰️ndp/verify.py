"""NDP registry verification against third-party libraries and the target specs.

Run: /home/deck/semio/.venv/bin/python -I verify.py REGISTRY.json OUTDIR [--apply]

Policy
- A row is verified only when every numeric value of the row is confirmed by an independent source.
  Independent = a third-party library in the venv that embeds the Eurocode table or constant.
  The target specs are project-written from memory (spec section 0.1), so they are only reported as
  a consistency check of the transcription and never verify a row.
- Library scope "product": product-standard table or fixed constant of the code body (EN 338, E, nu, alpha_T).
  The national annex does not change it, so recommended and naDe must both equal the library value.
- Library scope "en": the library holds the EN recommended value only. If the row also carries a
  national value the row stays unverified and is marked "EN confirmed, NA open" in `evidence`.
- Never auto-change a value. Differences are reported as conflicts.
"""
import importlib.metadata as md, inspect, json, math, pathlib, re, sys
from collections import Counter, defaultdict

TODAY = "2026-11-03"
TICKET = pathlib.Path(__file__).resolve().parent.parent
SPECS = TICKET / "📐️specs"
UNIT = {"": 1.0, "MPa": 1e6, "N/mm2": 1e6, "GPa": 1e9, "kg/m3": 1.0, "1/K": 1.0, "mm": 1e-3, "m": 1.0}


def norm(value, unit):
    return float(value) * UNIT.get(unit, 1.0) if isinstance(value, (int, float)) else None


def same(a, b):
    return a is not None and b is not None and math.isclose(a, b, rel_tol=1e-9, abs_tol=1e-12)


def ver(pkg):
    return f"{pkg} {md.version(pkg)}"


FACTS = defaultdict(list)


def fact(rid, value, unit, scope, cite, inferred=False):
    FACTS[rid].append(dict(value=value, unit=unit, scope=scope, cite=cite, inferred=inferred))


def en338():
    from blueprints.codes.eurocode.en_338_2016.chapter_5_classification_of_structural_timber import table_1, table_3
    props = {"fmk": "f_m_k", "ft0k": "f_t_0_k", "ft90k": "f_t_90_k", "fc0k": "f_c_0_k", "fc90k": "f_c_90_k", "fvk": "f_v_k",
             "e0mean": "e_m_0_mean", "e005": "e_m_0_k", "e90mean": "e_m_90_mean", "gmean": "g_mean", "rhok": "rho_k", "rhomean": "rho_mean"}
    units = {"rhok": "kg/m3", "rhomean": "kg/m3"}
    v = ver("blue-prints")
    for enum_cls, table_cls, tmod in [(table_1.SoftwoodStrengthClassBending, table_1.Table1StrengthClassesSoftwoodBending, "table_1"),
                                      (table_3.HardwoodStrengthClass, table_3.Table3StrengthClassesHardwoodBending, "table_3")]:
        for member in enum_cls:
            row = table_cls(member)
            for key, attr in props.items():
                rid = f"ndp.en1995.strength.{member.value.lower()}.{key}"
                cite = f"{v} blueprints.codes.eurocode.en_338_2016.chapter_5_classification_of_structural_timber.{tmod}.{table_cls.__name__}({enum_cls.__name__}.{member.name}).{attr} (EN 338:2016 Table {tmod[-1]})"
                fact(rid, getattr(row, attr), units.get(key, "N/mm2"), "product", cite)


def structuralcodes():
    from structuralcodes.codes.ec2_2004 import shear, _section_7_3_crack_control as cc
    from structuralcodes.materials.concrete import ConcreteEC2_2004
    from structuralcodes.materials.reinforcement import ReinforcementEC2_2004
    v = ver("structuralcodes")
    con = ConcreteEC2_2004(fck=30)
    rebar = ReinforcementEC2_2004(fyk=500, Es=200000, density=7850, ftk=540, epsuk=0.07)
    p = "structuralcodes.materials.concrete._concreteEC2_2004.ConcreteEC2_2004"
    fact("ndp.en1992.m-01.gamma-c-persistent", con.gamma_c, "", "en", f"{v} {p}.gamma_c (EN 1992-1-1 2.4.2.4 recommended)")
    fact("ndp.en1992.m-01.alpha-cc-persistent", con.alpha_cc, "", "en", f"{v} {p}.alpha_cc (EN 1992-1-1 3.1.6(1) recommended)")
    fact("ndp.en1992.m-01.alpha-ct", con.alpha_ct, "", "en", f"{v} {p}.alpha_ct (EN 1992-1-1 3.1.6(2) recommended)")
    fact("ndp.en1992.gamma-s", rebar.gamma_s, "", "en", f"{v} structuralcodes.materials.reinforcement._reinforcementEC2_2004.ReinforcementEC2_2004.gamma_s (EN 1992-1-1 2.4.2.4 recommended)")
    sig = inspect.signature(shear.VRdc).parameters
    fact("ndp.en1992.k1-shear", sig["k1"].default, "", "en", f"{v} structuralcodes.codes.ec2_2004.shear.VRdc, default k1 (EN 1992-1-1 Eq. 6.2)")
    numerator = float(re.search(r"CRdc or ([0-9.]+) / gamma_c", inspect.getsource(shear.VRdc)).group(1))
    fact("ndp.en1992.crd-c-shear", numerator, "", "en", f"{v} structuralcodes.codes.ec2_2004.shear.VRdc, default CRdc = {numerator} / gamma_c (EN 1992-1-1 Eq. 6.2)")
    k = (1 + math.sqrt(200 / 100.0))
    k = min(k, 2.0)
    fact("ndp.en1992.vmin-shear", shear.vmin(1.0, 100.0) / k ** 1.5, "", "en", f"{v} structuralcodes.codes.ec2_2004.shear.vmin, coefficient of k^1.5 sqrt(fck) (EN 1992-1-1 Eq. 6.3N)")
    fact("ndp.en1992.kt-short", cc.kt("short") if hasattr(cc, "kt") else None, "", "en", f"{v} structuralcodes.codes.ec2_2004._section_7_3_crack_control.kt('short') (EN 1992-1-1 7.3.4(2))")
    fact("ndp.en1992.kt-long", cc.kt("long") if hasattr(cc, "kt") else None, "", "en", f"{v} structuralcodes.codes.ec2_2004._section_7_3_crack_control.kt('long') (EN 1992-1-1 7.3.4(2))")
    fact("ndp.en1992.k3-crack", cc.k3(), "", "en", f"{v} structuralcodes.codes.ec2_2004._section_7_3_crack_control.k3 (EN 1992-1-1 Eq. 7.11)")
    fact("ndp.en1992.k4-crack", cc.k4(), "", "en", f"{v} structuralcodes.codes.ec2_2004._section_7_3_crack_control.k4 (EN 1992-1-1 Eq. 7.11)")


def steel():
    from blueprints.materials import steel as s
    v = ver("blue-prints")
    fact("ndp.en1993.1-1.e-modulus", s.STEEL_YOUNG_MODULUS, "MPa", "product", f"{v} blueprints.materials.steel.STEEL_YOUNG_MODULUS (EN 1993-1-1 3.2.6)")
    fact("ndp.en1993.1-1.poisson", s.STEEL_POISSON_RATIO, "", "product", f"{v} blueprints.materials.steel.STEEL_POISSON_RATIO (EN 1993-1-1 3.2.6)")
    fact("ndp.en1993.1-1.alpha-t", s.STEEL_THERMAL_COEFFICIENT, "1/K", "product", f"{v} blueprints.materials.steel.STEEL_THERMAL_COEFFICIENT (EN 1993-1-1 3.2.6)")
    from blueprints.checks.eurocode.steel.strength_tension import CheckStrengthTensionClass1234 as C
    fact("ndp.en1993.gamma-m0", C.__dataclass_fields__["gamma_m0"].default, "", "en", f"{v} blueprints.checks.eurocode.steel.strength_tension.CheckStrengthTensionClass1234.gamma_m0 default (EN 1993-1-1 6.1(1) recommended)")


def groundhog():
    from groundhog.standards.eurocode7.factors import Eurocode7_factoring_STR_GEO as F
    f = F()
    v = ver("groundhog")
    p = "groundhog.standards.eurocode7.factors.Eurocode7_factoring_STR_GEO"
    for s, tag in [("A1", "a1"), ("A2", "a2")]:
        fact(f"ndp.en1997.{tag}.g", f.factors_actions[s]["Permanent unfavourable"], "", "en", f"{v} {p}.factors_actions['{s}']['Permanent unfavourable'] (EN 1997-1 Table A.3)")
        fact(f"ndp.en1997.{tag}.q", f.factors_actions[s]["Variable unfavourable"], "", "en", f"{v} {p}.factors_actions['{s}']['Variable unfavourable'] (EN 1997-1 Table A.3)")
    soil = {"c": "Effective cohesion", "cu": "Undrained shear strength", "gamma": "Weight density", "phi": "Angle of shearing resistance"}
    for s in ("M1", "M2"):
        for key, name in soil.items():
            fact(f"ndp.en1997.{s.lower()}.{key}", f.factors_soil[s][name], "", "en", f"{v} {p}.factors_soil['{s}']['{name}'] (EN 1997-1 Table A.4)")
    res = {"b": ("Driven pile", "Base"), "s": ("Driven pile", "Shaft"), "t": ("Driven pile", "Total compression"), "st": ("Driven pile", "Shaft tension"),
           "a": ("Prestressed anchorage", "Temporary"), "rv": ("Retaining structure", "Bearing capacity"), "rh": ("Retaining structure", "Sliding resistance"), "re": ("Retaining structure", "Earth resistance")}
    for s in ("R1", "R2", "R3", "R4"):
        for key, (kind, comp) in res.items():
            value = f.factors_resistance[s][kind][comp]
            if not math.isnan(value):
                fact(f"ndp.en1997.{s.lower()}.{key}", value, "", "en", f"{v} {p}.factors_resistance['{s}']['{kind}']['{comp}'] (EN 1997-1 Tables A.5-A.14; registry row names no pile type, mapping inferred)", inferred=True)
    cf = f.correlation_factors
    for tag, mean, mn, ns in [("static", "ksi_1", "ksi_2", {1: "1", 2: "2", 3: "3", 4: "4", 5: ">=5"}), ("ground", "ksi_3", "ksi_4", {n: str(n) for n in (1, 2, 3, 4, 5, 7, 10)}),
                              ("dynamic", "ksi_5", "ksi_6", {n: f">={n}" for n in (2, 5, 10, 15, 20)})]:
        for n, key in ns.items():
            for idx, name in ((1, mean), (2, mn)):
                number = {"static": idx, "ground": idx + 2, "dynamic": idx + 4}[tag]
                fact(f"ndp.en1997.xi.{tag}.n{n}.{number}", cf[name][key], "", "en", f"{v} {p}.correlation_factors['{name}']['{key}'] (EN 1997-1 Table A.{ {'static': 9, 'ground': 10, 'dynamic': 11}[tag] })")


for build in (en338, structuralcodes, steel, groundhog):
    build()


def spec_line(ref):
    m = re.search(r"([\w-]+\.md)[: ]+(?:line )?(\d+)", ref or "")
    if not m or not (SPECS / m.group(1)).exists():
        return None
    lines = (SPECS / m.group(1)).read_text().splitlines()
    n = int(m.group(2))
    return (m.group(1), n, lines[n - 1]) if 0 < n <= len(lines) else None


def tokens(value):
    out = {f"{value:g}", f"{value:.1f}", f"{value:.2f}", f"{value:.3f}".rstrip("0").rstrip(".")}
    if abs(value) >= 1000 and float(value).is_integer():
        out |= {f"{int(value):,}".replace(",", " "), f"{int(value):,}".replace(",", " "), f"{int(value)}"}
    return out


def in_line(value, text):
    return any(re.search(rf"(?<![\d.]){re.escape(t)}(?![\d])", text) for t in tokens(value))


def classify(e):
    rec, na, unit = norm(e["recommended"], e["unit"]), norm(e["naDe"], e["unit"]), e["unit"]
    cands = FACTS.get(e["id"])
    if not cands:
        return "no_source", None
    c = cands[0]
    if c["value"] is None:
        return "no_source", None
    want = norm(c["value"], c["unit"])
    ok_rec = same(rec, want)
    if rec is None:
        return "no_source", None
    if not ok_rec:
        return ("candidate_conflict" if c["inferred"] else "conflict"), c
    if c["scope"] == "product":
        if na is not None and not same(na, want):
            return "conflict", c
        return "verified", c
    if e["naDe"] is None:
        return ("en_confirmed_na_open" if c["inferred"] else "verified"), c
    return "en_confirmed_na_open", c


def main():
    src, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    apply = "--apply" in sys.argv
    doc = json.loads(src.read_text())
    out.mkdir(parents=True, exist_ok=True)
    res = []
    for e in doc["entries"]:
        cls, c = classify(e)
        spec = spec_line(e["specRef"])
        sp = None
        if spec and e["recommended"] is not None:
            sp = in_line(e["recommended"], spec[2]) and (e["naDe"] is None or in_line(e["naDe"], spec[2]))
        res.append(dict(id=e["id"], code=e["code"], status=e["status"], cls=cls, rec=e["recommended"], na=e["naDe"], unit=e["unit"],
                        lib=None if not c else dict(value=c["value"], unit=c["unit"], scope=c["scope"], cite=c["cite"], inferred=c["inferred"]),
                        spec=None if not spec else dict(file=spec[0], line=spec[1], consistent=sp)))
    (out / "results.json").write_text(json.dumps(res, ensure_ascii=False, indent=1))
    count = Counter((r["code"], r["status"], r["cls"]) for r in res)
    for k, v in sorted(count.items()):
        print(k, v)
    if apply:
        byid = {r["id"]: r for r in res}
        changed = Counter()
        for e in doc["entries"]:
            r = byid[e["id"]]
            if not r["lib"] or r["cls"] not in ("verified", "en_confirmed_na_open"):
                continue
            note = f"ndp-verify {TODAY}: " + ("VERIFIED" if r["cls"] == "verified" else "EN value confirmed, NA-DE value still open") + f" by {r['lib']['cite']}; library value {r['lib']['value']} {r['lib']['unit']}".rstrip()
            if note in e["evidence"]:
                continue
            e["evidence"] = (e["evidence"] + " | " if e["evidence"] else "") + note
            if r["cls"] == "verified":
                if e["status"] == "unverified":
                    changed["newly_verified"] += 1
                e["status"] = "verified"
                if e["source"]["edition"] == "to be recorded at verification":
                    e["source"]["edition"] = "EN 338:2016" if ".strength." in e["id"] else "EN edition of the cited library table"
            else:
                changed["en_confirmed_note"] += 1
        src.write_text(json.dumps(doc, ensure_ascii=False, indent=1))
        print("applied", dict(changed))


if __name__ == "__main__":
    main()
