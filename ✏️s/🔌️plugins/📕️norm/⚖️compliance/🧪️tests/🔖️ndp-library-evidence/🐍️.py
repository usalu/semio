import json, math, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
NORM = HERE.parent.parent.parent
REGISTRY = NORM / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json"
FIXTURES = HERE.parent / "🧫️fixtures"
LIBRARY = re.compile(r"\b(blue-prints|structuralcodes|groundhog|concreteproperties|sectionproperties) \d+\.\d+")
SCALE = {"": 1.0, "MPa": 1e6, "N/mm2": 1e6, "GPa": 1e9, "kg/m3": 1.0, "1/K": 1.0, "mm": 1e-3, "m": 1.0}

def registry():
    return {e["id"]: e for e in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]}

def en_verified(e):
    per = e.get("annexStatus")
    return e["status"] == "verified" or bool(per and per["en"]["status"] == "verified")

def any_verified(e):
    per = e.get("annexStatus")
    return e["status"] == "verified" or bool(per and (per["en"]["status"] == "verified" or per["naDe"]["status"] == "verified"))

def verified_flags_are_library_backed(reg):
    for e in reg.values():
        per = e.get("annexStatus")
        if e["status"] == "verified":
            assert LIBRARY.search(e["evidence"]), e["id"]
        if per:
            for side in ("en", "naDe"):
                if per[side]["status"] == "verified":
                    assert LIBRARY.search(per[side]["evidence"]), (e["id"], side)
            assert (e["status"] == "verified") == (per["en"]["status"] == "verified" and per["naDe"]["status"] == "verified"), e["id"]

def library_fixture_equals_registry(reg):
    rows = json.loads((FIXTURES / "🔖️ndp-library-evidence" / "🔣️.json").read_text(encoding="utf-8"))["rows"]
    assert len(rows) > 100
    for r in rows:
        e = reg[r["id"]]
        assert math.isclose(e["recommended"] * SCALE.get(e["unit"], 1.0), r["value"] * SCALE.get(r["unit"], 1.0), rel_tol=1e-9, abs_tol=1e-12), r["id"]
        assert en_verified(e), r["id"]
        evidence = e["evidence"] + " " + (e.get("annexStatus") or {}).get("en", {}).get("evidence", "")
        assert r["library"] in evidence, (r["id"], r["library"])
        assert r["kind"] in ("executed", "documented")
    return len(rows)

def live_probe(reg):
    try:
        import inspect
        from structuralcodes.codes.ec2_2004 import shear
        from blueprints.codes.eurocode.en_338_2016.chapter_5_classification_of_structural_timber import table_1
        from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_4_durability_and_cover.table_4_3 import Table4Dot3ConcreteStructuralClass as T43
        from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_8_detailing_of_reinforcement_and_prestressing_tendons.formula_8_2 import Form8Dot2UltimateBondStress as Bond
        from blueprints.materials.concrete import ConcreteMaterial, ConcreteStrengthClass
    except ImportError:
        return "skipped (libraries not importable)"
    c24 = table_1.Table1StrengthClassesSoftwoodBending(table_1.SoftwoodStrengthClassBending.C24)
    assert c24.f_t_0_k == reg["ndp.en1995.strength.c24.ft0k"]["recommended"]
    assert inspect.signature(shear.VRdc).parameters["k1"].default == reg["ndp.en1992.k1-shear"]["recommended"]
    assert inspect.signature(shear._theta).parameters["cot_max"].default == reg["ndp.en1992.cot-theta-max"]["recommended"]
    assert inspect.signature(shear._theta).parameters["cot_min"].default == reg["ndp.en1992.cot-theta-min"]["recommended"]
    assert float(Bond(eta_1=1.0, eta_2=1.0, f_ctd=1.0)) == reg["ndp.en1992.fbd-eta"]["recommended"]
    threshold = float(ConcreteMaterial(ConcreteStrengthClass("C30/37")).f_ck)
    levels = {c: int(T43(exposure_classes=["XC1"], design_working_life=50, concrete_material=ConcreteMaterial(ConcreteStrengthClass(c)), plate_geometry=False, quality_control=False)) for c in ("C25/30", "C30/37")}
    assert levels["C30/37"] == levels["C25/30"] - 1 and threshold == reg["ndp.en1992.d-04.class-fck.xc1"]["recommended"]
    return "recomputed"

def ledger_keeps_basis(reg):
    rows = json.loads((FIXTURES / "🔖️ndp-conflict-ledger" / "🔣️.json").read_text(encoding="utf-8"))["rows"]
    assert len({r["id"] for r in rows}) == len(rows)
    for r in rows:
        e = reg[r["id"]]
        assert (e["recommended"], e["naDe"]) == (r["en"], r["naDe"]), (r["id"], e["recommended"], e["naDe"])
        if r["basis"] == "library":
            assert en_verified(e), r["id"]
        else:
            assert not any_verified(e), (r["id"], r["basis"])
    return len(rows)

def main():
    reg = registry()
    verified_flags_are_library_backed(reg)
    n = library_fixture_equals_registry(reg)
    live = live_probe(reg)
    m = ledger_keeps_basis(reg)
    synthetic = dict(reg["ndp.en1992.k1-shear"], status="verified", evidence="no library")
    try:
        verified_flags_are_library_backed({"x": synthetic})
        raise SystemExit("negative control did not fail")
    except AssertionError:
        pass
    print(f"ndp library evidence ok: {n} library rows, {m} ledger rows, live probe {live}")

if __name__ == "__main__":
    main()
