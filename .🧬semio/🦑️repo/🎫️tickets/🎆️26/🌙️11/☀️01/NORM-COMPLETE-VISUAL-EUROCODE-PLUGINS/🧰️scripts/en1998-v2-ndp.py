"""EN 1998 v2: registers the unverified constants of the Annex A chord rotation, Annex B liquefaction screen, Annex F bearing surface and pile expressions in the shared NDP registry (`⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`), idempotently, then regenerates `🗃️table.rs` with `🧰️ndp/codegen.py`.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v2-ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
PRE = "ndp.en1998."
DOC = "DIN EN 1998-1/NA:2023-11"
SPEC = "wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition"

ROWS = [
    ("en3.annex-a.theta-y.base", "EN 1998-3 A.10b", "θ_y constant term", 0.0013, 0.0013, "", "0.0013 (1 + 1.5 h/L_V)"),
    ("en3.annex-a.theta-y.slenderness", "EN 1998-3 A.10b", "θ_y h/L_V factor", 1.5, 1.5, "", "0.0013 (1 + 1.5 h/L_V)"),
    ("en3.annex-a.theta-y.bar-slip", "EN 1998-3 A.10b", "θ_y bar slip factor", 0.13, 0.13, "", "0.13 φ_y d_b f_y/√f_c"),
    ("en3.annex-a.theta-um.base", "EN 1998-3 A.1", "θ_um base value", 0.016, 0.016, "", "0.016 (1/γ_el)"),
    ("en3.annex-a.theta-um.axial", "EN 1998-3 A.1", "θ_um axial load base", 0.3, 0.3, "", "0.3^ν"),
    ("en3.annex-a.theta-um.reinforcement", "EN 1998-3 A.1", "θ_um reinforcement exponent", 0.225, 0.225, "", "[max(0.01, ω')/max(0.01, ω) f_c]^0.225"),
    ("en3.annex-a.theta-um.shear-span", "EN 1998-3 A.1", "θ_um shear span exponent", 0.35, 0.35, "", "(L_V/h)^0.35"),
    ("en3.annex-a.theta-um.confinement", "EN 1998-3 A.1", "θ_um confinement base", 25.0, 25.0, "", "25^(α ρ_sx f_yw/f_c)"),
    ("en3.annex-a.theta-um.diagonal", "EN 1998-3 A.1", "θ_um diagonal reinforcement base", 1.25, 1.25, "", "1.25^(100 ρ_d)"),
    ("en3.annex-a.steel-modulus-mpa", "EN 1992-1-1 3.2.7(4)", "E_s", 200000.0, 200000.0, "MPa", "E_s = 200 GPa"),
    ("en5.liquefaction.annex-b.csr-factor", "EN 1998-5 Annex B", "τ_e factor", 0.65, 0.65, "", "τ_e/σ'_v0 = 0.65 (a_g S/g)(σ_v0/σ'_v0), no depth reduction r_d (Youd 2001 applies r_d)"),
    ("en5.liquefaction.screen-ag-s-g", "EN 1998-5 4.1.4", "a_g S screening limit in g", 0.1, 0.15, "", "liquefaction may be neglected below a_g S/g; the NA-DE value is marked [V] in the spec"),
    ("en5.liquefaction.screen-depth-m", "EN 1998-5 4.1.4", "screening depth", 15.0, 15.0, "m", "layers deeper than 15 m"),
    ("en5.annex-f.gamma-rd", "EN 1998-5 Annex F", "γ_Rd", 1.0, 1.0, "", "γ_Rd = 1.0 sand, 1.15 loose, 1.5 sensitive clay"),
    ("en5.annex-f.sand.a", "EN 1998-5 Table F.1", "a", 0.92, 0.92, "", ""),
    ("en5.annex-f.sand.b", "EN 1998-5 Table F.1", "b", 1.25, 1.25, "", ""),
    ("en5.annex-f.sand.c", "EN 1998-5 Table F.1", "c", 0.92, 0.92, "", ""),
    ("en5.annex-f.sand.d", "EN 1998-5 Table F.1", "d", 1.25, 1.25, "", ""),
    ("en5.annex-f.sand.e", "EN 1998-5 Table F.1", "e", 1.0, 1.0, "", "the spec GE-06 lists 0.92"),
    ("en5.annex-f.sand.f", "EN 1998-5 Table F.1", "f", 1.0, 1.0, "", "the spec GE-06 lists 0.41"),
    ("en5.annex-f.sand.m", "EN 1998-5 Table F.1", "m", 0.96, 0.96, "", ""),
    ("en5.annex-f.sand.k", "EN 1998-5 Table F.1", "k", 1.0, 1.0, "", "the spec GE-06 lists 0.39"),
    ("en5.annex-f.sand.k-prime", "EN 1998-5 Table F.1", "k'", 0.39, 0.39, "", ""),
    ("en5.annex-f.sand.c-t", "EN 1998-5 Table F.1", "c_T", 2.14, 2.14, "", ""),
    ("en5.annex-f.sand.c-t-prime", "EN 1998-5 Table F.1", "c'_T", 1.81, 1.81, "", ""),
    ("en5.annex-f.sand.c-m", "EN 1998-5 Table F.1", "c_M", 2.57, 2.57, "", ""),
    ("en5.annex-f.sand.c-m-prime", "EN 1998-5 Table F.1", "c'_M", 1.85, 1.85, "", ""),
    ("en5.annex-f.sand.beta", "EN 1998-5 Table F.1", "β", 2.9, 2.9, "", "the spec GE-06 lists 2.57"),
    ("en5.annex-f.sand.gamma", "EN 1998-5 Table F.1", "γ", 2.8, 2.8, "", "the spec GE-06 lists 4.0"),
    ("en5.pile.stiffness-contrast", "EN 1998-5 5.4.2", "G_2/G_1 for kinematic interaction", 6.0, 6.0, "", "kinematic bending where the shear modulus ratio of adjacent layers exceeds this"),
    ("ref.pile.nikolaou-2001.coefficient", "Nikolaou et al. 2001", "M = c τ_c d³ ...", 0.042, 0.042, "", "reference basis, EN 1998-5 gives no closed form"),
    ("ref.pile.nikolaou-2001.length-exponent", "Nikolaou et al. 2001", "(L/d)^p", 0.30, 0.30, "", "reference basis"),
    ("ref.pile.nikolaou-2001.modulus-exponent", "Nikolaou et al. 2001", "(E_p/E_1)^q", 0.65, 0.65, "", "reference basis"),
    ("ref.pile.nikolaou-2001.velocity-exponent", "Nikolaou et al. 2001", "(v_s2/v_s1)^r", 0.50, 0.50, "", "reference basis"),
]


def entry(row):
    key, clause, symbol, rec, na, unit, note = row
    return {"id": PRE + key, "code": "EN 1998", "part": "", "clause": clause, "symbol": symbol, "recommended": rec, "naDe": na, "unit": unit,
            "source": {"document": DOC, "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": SPEC,
            "asWritten": "%s = %s as used by the artifact" % (symbol, rec), "evidence": note}


def main():
    path = NDP + "🔣️.json"
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
    known = {e["id"] for e in data["entries"]}
    added = [entry(r) for r in ROWS if PRE + r[0] not in known]
    if not added:
        print("registry already complete")
        return
    data["entries"] = sorted(data["entries"] + added, key=lambda e: e["id"])
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
    subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
    print("added", len(added))


main()
