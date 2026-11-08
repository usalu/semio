"""🔬 Fleet-1108 library probes: executable and documented facts of third-party Eurocode libraries for NDP registry ids.

Imported by `fleet1108_ndp.py`; run with `/home/deck/semio/.venv/bin/python -I`. Every probe returns (value, unit, scope, kind, cite)
where kind is `executed` (the library computes the number) or `documented` (the library docstring states the recommended value).
"""
import importlib.metadata as md, inspect, re

BP, SC = f"blue-prints {md.version('blue-prints')}", f"structuralcodes {md.version('structuralcodes')}"
EC2 = "blueprints.codes.eurocode.en_1992_1_1_2004"


def documented(text, pattern):
    return float(re.search(pattern, text).group(1))


def probes():
    out = {}

    def add(rid, value, scope, kind, cite, unit=""):
        out[rid] = dict(value=value, unit=unit, scope=scope, kind=kind, cite=cite)

    from structuralcodes.codes.ec2_2004 import shear
    sig = inspect.signature(shear._theta).parameters
    add("ndp.en1992.cot-theta-max", sig["cot_max"].default, "en", "executed", f"{SC} structuralcodes.codes.ec2_2004.shear._theta default cot_max (EN 1992-1-1 Eq. 6.7N)")
    add("ndp.en1992.cot-theta-min", sig["cot_min"].default, "en", "executed", f"{SC} structuralcodes.codes.ec2_2004.shear._theta default cot_min (EN 1992-1-1 Eq. 6.7N)")
    add("ndp.en1992.nu1-en-factor", shear.v(0.0), "en", "executed", f"{SC} structuralcodes.codes.ec2_2004.shear.v(0) (EN 1992-1-1 Eq. 6.6N)")
    add("ndp.en1992.nu1-en-divisor", 250.0, "en", "executed", f"{SC} structuralcodes.codes.ec2_2004.shear.v: v(125)/v(0) = {shear.v(125.0) / shear.v(0.0)} (EN 1992-1-1 Eq. 6.6N, 1 - f_ck/250)") if abs(shear.v(125.0) / shear.v(0.0) - 0.5) < 1e-12 else None
    add("ndp.en1992.nu1-strut-factor", shear.v1(30.0), "en", "executed", f"{SC} structuralcodes.codes.ec2_2004.shear.v1(30) (EN 1992-1-1 Eq. 6.10aN)")

    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_8_detailing_of_reinforcement_and_prestressing_tendons.formula_8_2 import Form8Dot2UltimateBondStress as Bond
    add("ndp.en1992.fbd-eta", float(Bond(eta_1=1.0, eta_2=1.0, f_ctd=1.0)), "en", "executed", f"{BP} {EC2}.chapter_8_detailing_of_reinforcement_and_prestressing_tendons.formula_8_2.Form8Dot2UltimateBondStress(1, 1, 1) (EN 1992-1-1 Eq. 8.2)")

    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_6_ultimate_limit_state import formula_6_47 as f647, formula_6_52 as f652
    doc647 = inspect.getsource(f647)
    add("ndp.en1992.crd-c-punch", documented(doc647, r"is \$([0-9.]+) / \\gamma_c\$, for \$v_\{min\}"), "en", "documented", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_47.Form6Dot47PunchingShearResistance docstring, recommended C_Rd,c = 0.18/gamma_c (EN 1992-1-1 6.4.4(1))")
    add("ndp.en1992.k1-punch", documented(doc647, r"\$k_1\$ is \$([0-9.]+)\$"), "en", "documented", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_47.Form6Dot47PunchingShearResistance docstring, recommended k_1 (EN 1992-1-1 6.4.4(1))")
    cs = f652.Form6Dot52PunchingShearResistance
    base = float(cs(v_rd_c=1.0, d=1.0, s_r=1.0, a_sw=0.0, f_ywd_ef=0.0, u_1=1.0, alpha=90.0))
    asw = float(cs(v_rd_c=0.0, d=1.0, s_r=1.0, a_sw=1.0, f_ywd_ef=1.0, u_1=1.0, alpha=90.0))
    add("ndp.en1992.punch-cs-vrdc-factor", base, "en", "executed", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_52.Form6Dot52PunchingShearResistance (EN 1992-1-1 Eq. 6.52, factor of v_Rd,c)")
    add("ndp.en1992.punch-cs-asw-factor", asw, "en", "executed", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_52.Form6Dot52PunchingShearResistance (EN 1992-1-1 Eq. 6.52, factor of d/s_r A_sw f_ywd,ef/(u_1 d))")
    ef = f652.Form6Dot52Sub1EffectiveYieldStrength
    f0 = float(ef(d=0.0, f_ywd=1e9))
    add("ndp.en1992.punch-fywd-ef-base", f0, "en", "executed", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_52.Form6Dot52Sub1EffectiveYieldStrength(d=0) (EN 1992-1-1 Eq. 6.52sub1)", "MPa")
    add("ndp.en1992.punch-fywd-ef-slope", (float(ef(d=1000.0, f_ywd=1e9)) - f0) / 1000.0, "en", "executed", f"{BP} {EC2}.chapter_6_ultimate_limit_state.formula_6_52.Form6Dot52Sub1EffectiveYieldStrength (EN 1992-1-1 Eq. 6.52sub1, slope in d)")

    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_4_durability_and_cover.constants import NominalConcreteCoverConstants
    add("ndp.en1992.delta-c-dev", float(NominalConcreteCoverConstants().DEFAULT_DELTA_C_DEV), "en", "executed", f"{BP} {EC2}.chapter_4_durability_and_cover.constants.NominalConcreteCoverConstants().DEFAULT_DELTA_C_DEV (EN 1992-1-1 4.4.1.3(1))", "mm")

    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_4_durability_and_cover.table_4_3 import Table4Dot3ConcreteStructuralClass as T43
    from blueprints.materials.concrete import ConcreteMaterial, ConcreteStrengthClass
    def level(exposure, name):
        value = T43(exposure_classes=[exposure], design_working_life=50, concrete_material=ConcreteMaterial(ConcreteStrengthClass(name)), plate_geometry=False, quality_control=False)
        return int(getattr(value, "value", value))

    classes = ["C12/15", "C16/20", "C20/25", "C25/30", "C30/37", "C35/45", "C40/50", "C45/55", "C50/60"]
    for key, exposure in {"xc1": "XC1", "xc2-xc3": "XC2", "xc4-xd1-xd2-xs1": "XC4", "xd3-xs2-xs3": "XD3"}.items():
        levels = [level(exposure, c) for c in classes]
        first = next(c for c, level in zip(classes, levels) if level != levels[0])
        add(f"ndp.en1992.d-04.class-fck.{key}", float(ConcreteMaterial(ConcreteStrengthClass(first)).f_ck), "en", "executed", f"{BP} {EC2}.chapter_4_durability_and_cover.table_4_3.Table4Dot3ConcreteStructuralClass(['{exposure}'], 50 years, {first}) lowers the structural class (EN 1992-1-1 Table 4.3N)", "MPa")

    import blueprints.codes.eurocode.en_1992_1_1_2004.chapter_5_structural_analysis.formula_5_47 as f547, blueprints.codes.eurocode.en_1992_1_1_2004.chapter_5_structural_analysis.formula_5_48 as f548
    for tag, module, symbol in (("sup", f547, "r_sup"), ("inf", f548, "r_inf")):
        text = inspect.getsource(module)
        pre = documented(text, rf"recommended value is ([0-9.]+) for pre-tensioning or unbounded tendons")
        post = documented(text, rf"tendons,\s*([0-9.]+) for post-tensioning with bonded tendons")
        add(f"ndp.en1992.p-r-{tag}.pre-or-unbonded", pre, "en", "documented", f"{BP} {EC2}.chapter_5_structural_analysis.{module.__name__.rsplit('.', 1)[1]} docstring, recommended {symbol} for pre-tensioning or unbonded tendons (EN 1992-1-1 5.10.9(1))")
        add(f"ndp.en1992.p-r-{tag}.post-bonded", post, "en", "documented", f"{BP} {EC2}.chapter_5_structural_analysis.{module.__name__.rsplit('.', 1)[1]} docstring, recommended {symbol} for post-tensioning with bonded tendons (EN 1992-1-1 5.10.9(1))")

    import blueprints.materials.reinforcement_steel as rs
    add("ndp.en1992.gamma-s-acc", documented(inspect.getsource(rs), r"Accidental design situations\s+\$\\gamma_s = ([0-9.]+)\$"), "en", "documented", f"{BP} blueprints.materials.reinforcement_steel.ReinforcementSteelMaterial docstring, accidental gamma_s (EN 1992-1-1 Table 2.1N)")
    import blueprints.codes.eurocode.en_1992_1_1_2004.chapter_5_structural_analysis.formula_5_1 as f51
    add("ndp.en1992.imperfection-theta0-divisor", documented(inspect.getsource(f51), r"recommended value is 1/([0-9.]+)"), "en", "documented", f"{BP} {EC2}.chapter_5_structural_analysis.formula_5_1 docstring, recommended theta_0 = 1/200 (EN 1992-1-1 5.2(5))")
    return {k: v for k, v in out.items() if v is not None}
