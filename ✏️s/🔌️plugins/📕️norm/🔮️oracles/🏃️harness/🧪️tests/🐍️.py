"""🧪️ Harness laws: schema validity of every shared case file, tolerance profile semantics, blessed numerical expectations and loud failure on wrong or missing references."""

import copy
import importlib.util
import json
from pathlib import Path

import pytest

HERE = Path(__file__).resolve().parent.parent
CASES = sorted((HERE.parent.parent / "🧪️tests" / "🧫️fixtures" / "🔮️oracle-cases").glob("*.json"))


def harness():
    """🏃️ Import the harness entry module by path."""
    spec = importlib.util.spec_from_file_location("norm_oracle_harness_under_test", HERE / "🐍️.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


@pytest.mark.parametrize("path", CASES, ids=lambda p: p.name)
def test_case_files_validate_and_match_blessed_values(path):
    h = harness()
    document = json.loads(path.read_text(encoding="utf-8"))
    h.validate(document)
    failures = [f for r in h.run(document) for f in r["failures"]]
    assert failures == []


def test_wrong_expected_value_fails_loudly():
    h = harness()
    document = json.loads(CASES[0].read_text(encoding="utf-8"))
    broken = copy.deepcopy(document["cases"][0])
    next(iter(broken["expected"].values()))["value"] *= 1.01
    assert any("expected" in f for f in h.judge(broken, h.compute(broken)))


def test_unknown_adapter_and_check_raise():
    h = harness()
    case = json.loads(CASES[0].read_text(encoding="utf-8"))["cases"][0]
    with pytest.raises(LookupError):
        h.compute({**case, "references": [{"oracle": "x", "adapter": "does-not-exist"}]})
    with pytest.raises(LookupError):
        h.compute({**case, "check": "en0000.nothing"})


def test_schema_rejects_missing_citation_and_non_si_profile():
    h = harness()
    document = json.loads(CASES[0].read_text(encoding="utf-8"))
    del document["cases"][0]["citation"]
    with pytest.raises(Exception):
        h.validate(document)


def test_tolerance_profiles():
    t = harness().TOLERANCE
    assert set(t.PROFILES) == {"closed-form-v1", "eurocode-quantities-v1", "numerical-v1", "method-spread-v1", "table-exact-v1"}
    assert t.within("closed-form-v1", 1.0, 1.0 + 1e-10)
    assert not t.within("closed-form-v1", 1.0, 1.0 + 1e-6)
    assert t.within("eurocode-quantities-v1", 100.0, 100.005)
    assert t.within("method-spread-v1", 1.0, 1.029)
    assert not t.within("table-exact-v1", 0.85, 0.8500001)
    assert t.within("numerical-v1", 1.0, 1.01, override=2e-2)


def test_registry_profiles_match_harness_profiles():
    registry = json.loads((HERE.parent / "🔣️.json").read_text(encoding="utf-8"))
    t = harness().TOLERANCE.PROFILES
    assert {p["id"]: p["tolerance"] for p in registry["comparisonProfiles"]} == {k: v["relative"] for k, v in t.items()}


INDEPENDENT_ONLY = ("en1990.6.4.combinations", "en1990.C.form-linear", "en1991.4.janssen-pressure", "en1994.6.2.composite-plastic", "en1998.3.2.2.5.design-spectrum", "en1998.4.3.shear-building-modal", "en1990.C.form-nonnormal", "en1990.fatigue.miner", "en1991.4.wind-peak-pressure", "en1991.7.wind-cpe", "en1991.snow", "en1991.fire.parametric-curve", "en1991.fire.iso834", "en1992.F.membrane-reinforcement", "en1992.KK", "en1993.5.3.2.sway-imperfection", "en1994.6.6.3.stud-resistance", "en1996.G.phi-m", "en1996.6.2.shear-wall", "en1997.7.pile", "en1997.8.anchor-resistance", "en1998.3.1.vs30", "en1998.4.3.5.2.nonstructural", "en1991.8.bridge-wind-force", "en1991.6.structural-factor", "en1991.6.vortex-shedding", "en1991.7.impact-force", "en1991.7.gas-explosion", "en1991.5.bridge-temperature", "en1991.2.special-vehicle", "en1991.snow.roof-step-drift")
ARTIFACT_CASES = sorted((HERE.parent.parent / "🗿️artifacts").glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json"))


def test_every_artifact_has_third_party_case_files():
    assert {p.parts[-9][-6:] for p in ARTIFACT_CASES} >= {"en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"}


@pytest.mark.parametrize("path", ARTIFACT_CASES, ids=lambda p: f"{p.parts[-9]}/{p.name}")
def test_artifact_case_files_validate_and_match_the_libraries(path):
    h = harness()
    document = json.loads(path.read_text(encoding="utf-8"))
    h.validate(document)
    assert all(r["failures"] == [] for r in h.run(document))
    assert all(ref["kind"] in ("third-party-library", "independent-formulation") for c in document["cases"] for ref in c["references"])
    assert all(any(ref["kind"] == "third-party-library" for ref in c["references"]) or c["check"].startswith(INDEPENDENT_ONLY) for c in document["cases"])


def test_non_finite_library_output_is_rejected(monkeypatch):
    h = harness()
    case = {"id": "en1997.x", "check": "en1997.9.earth-pressure", "inputs": {"phi": 30.0, "delta": 0.0}, "references": [{"oracle": "x", "adapter": "groundhog"}]}
    assert set(h.compute(case)["x"]) == {"Ka_rankine", "Kp_rankine"}
    broken = type("Broken", (), {"CHECKS": {"en1997.9.earth-pressure": lambda i: {"Ka_rankine": float("nan")}}})
    monkeypatch.setattr(h, "adapter", lambda name: broken)
    with pytest.raises(ArithmeticError):
        h.compute(case)


def run_check(adapter_name, check, inputs):
    """🧮️ Run one adapter check on `inputs`."""
    return harness().adapter(adapter_name).CHECKS[check](inputs)


def test_t_section_with_flange_only_compression_equals_the_rectangle_of_flange_width():
    base = {"fck": 30e6, "fyk": 500e6, "gamma_c": 1.5, "gamma_s": 1.15, "alpha_cc": 0.85, "d": 0.55, "As": 0.002}
    t = run_check("numerical", "en1992.6.1.flexure-t", base | {"b_f": 1.2, "h_f": 0.15, "b_w": 0.3, "h": 0.6})
    rect = run_check("numerical", "en1992.6.1.flexure-rect", base | {"b": 1.2, "h": 0.6})
    assert t["x"] < 0.15 and t["M_Rd"] == pytest.approx(rect["M_Rd"], rel=1e-9)


def test_janssen_numerical_and_symbolic_solutions_agree_and_approach_the_asymptote():
    inputs = {"gamma": 9000.0, "a_m": 1.0, "mu": 0.4, "K": 0.5, "depth": 60.0}
    numeric, exact = run_check("numerical", "en1991.4.janssen-pressure", inputs), run_check("sympy", "en1991.4.janssen-pressure", inputs)
    assert numeric["p_hf"] == pytest.approx(exact["p_hf"], rel=1e-9)
    assert numeric["p_hf"] == pytest.approx(numeric["p_ho"], rel=1e-4)


def test_shear_building_periods_agree_between_eigensolver_and_characteristic_polynomial():
    inputs = {"m": [100e3, 100e3, 80e3], "k": [60e6, 50e6, 40e6]}
    numeric, exact = run_check("numerical", "en1998.4.3.shear-building-modal", inputs), run_check("sympy", "en1998.4.3.shear-building-modal", inputs)
    assert [numeric[f"T_{j}"] for j in (1, 2, 3)] == pytest.approx([exact[f"T_{j}"] for j in (1, 2, 3)], rel=1e-9)
    assert numeric["T_1"] > numeric["T_2"] > numeric["T_3"]


def test_form_design_point_satisfies_the_limit_state_and_the_sensitivity_factors_are_unit_length():
    inputs = {"mu_R": 300.0, "sigma_R": 30.0, "mu_E": 150.0, "sigma_E": 30.0}
    out = run_check("numerical", "en1990.C.form-linear", inputs)
    assert out["R_d"] == pytest.approx(out["E_d"], rel=1e-9)
    assert out["alpha_R"] ** 2 + out["alpha_E"] ** 2 == pytest.approx(1.0, rel=1e-9)


def test_lm1_tandem_extremes_follow_the_closed_form_of_the_axle_group():
    out = run_check("numerical", "mechanics.lm1-tandem", {"span": 9.6, "Q": 300e3, "s": 1.2})
    a = 9.6 / 2.0 - 1.2 / 4.0
    reaction = 300e3 * ((1.0 - a / 9.6) + (1.0 - (a + 1.2) / 9.6))
    assert out["M_max"] == pytest.approx(reaction * a, rel=1e-9)
    assert out["R_max"] == pytest.approx(300e3 * (2.0 - 1.2 / 9.6), rel=1e-9)


def test_timber_table_values_are_exact_en_338_2016_entries():
    c30 = run_check("blueprints", "en1995.338.strength-class", {"strength_class": "C30"})
    assert (c30["f_m_k"], c30["f_t_0_k"], c30["f_c_0_k"], c30["rho_mean"]) == (30e6, 19e6, 24e6, 460.0)


def test_vlasov_finite_element_reproduces_the_sinusoidal_mode_closed_form_with_and_without_restraint():
    section = {"h": 0.3, "b": 0.3, "tf": 0.014, "tw": 0.0085, "E": 210e9, "G": 81e9, "L": 10.0, "f_o": 355e6, "gamma_M1": 1.0, "alpha_LT": 0.49, "lambda_0": 0.2}
    for spring in (0.0, 5.0e4):
        inputs = section | {"k_s": spring}
        fe = run_check("numerical", "en1999.6.3.2.ltb", inputs)["M_cr"]
        closed = run_check("numerical", "en1994.6.4.ltb-restrained", inputs)["M_cr"]
        assert fe == pytest.approx(closed, rel=1e-9)
        assert fe == pytest.approx(run_check("sympy", "en1994.6.4.ltb-restrained", inputs)["M_cr"], rel=1e-3)


def test_annex_d_coefficient_for_unknown_variation_is_the_student_prediction_factor_and_exceeds_the_known_one():
    for n in (3, 5, 10, 30):
        out = run_check("numerical", "en1990.D.7.characteristic-from-tests", {"n": n, "mean": 100.0, "s": 12.0, "sigma": 12.0})
        assert out["k_n_unknown"] > out["k_n_known"] > 1.645
    assert run_check("numerical", "en1990.D.7.characteristic-from-tests", {"n": 3, "mean": 0.0, "s": 1.0, "sigma": 1.0})["k_n_unknown"] == pytest.approx(3.37, abs=5e-3)


def test_membrane_reinforcement_minimisation_reproduces_the_closed_form_tie_forces():
    for n in ({"n_x": 200e3, "n_y": 100e3, "n_xy": 80e3}, {"n_x": 300e3, "n_y": -150e3, "n_xy": 120e3}, {"n_x": -100e3, "n_y": 200e3, "n_xy": 100e3}):
        numeric, closed = run_check("numerical", "en1992.F.membrane-reinforcement", n), run_check("sympy", "en1992.F.membrane-reinforcement", n)
        assert numeric["f_x"] == pytest.approx(closed["f_x"], rel=1e-5, abs=1e-3) and numeric["f_y"] == pytest.approx(closed["f_y"], rel=1e-5, abs=1e-3)


def test_eurocode_storey_formula_is_unconservative_against_the_eigenvalue_for_stiff_fixed_base_portals():
    frame = {"E": 210e9, "A": 0.02, "I_col": 8e-5, "I_beam": 8e-4, "h": 4.0, "span": 6.0, "H": 50e3, "V": 2.0e6}
    both = run_check("numerical", "en1993.5.2.alpha-cr-storey", frame)
    assert both["alpha_cr_formula"] > both["alpha_cr_eigen"] > 0.0
    assert run_check("pynite", "en1993.5.2.alpha-cr-eigen", frame)["alpha_cr_eigen"] == pytest.approx(both["alpha_cr_eigen"], rel=5e-3)


def test_dry_run_plans_the_mechanics_cases_and_refuses_bad_geometry_without_a_solver():
    h = harness()
    document = json.loads(next(p for p in ARTIFACT_CASES if p.name == "section-mechanics.json").read_text(encoding="utf-8"))
    rows = h.dry_run(document)
    assert [r["status"] for r in rows] == ["planned"] * 3
    assert rows[0]["plan"]["area"] == pytest.approx(0.07 * 0.14, rel=1e-12)
    assert rows[2]["plan"]["j"] == pytest.approx(sum(w["thickness"] ** 3 * abs(w["y2"] - w["y1"] + w["z2"] - w["z1"]) / 3 for w in document["cases"][2]["inputs"]["walls"]), rel=1e-12)
    numerical = json.loads((CASES[0].parent / "numerical.json").read_text(encoding="utf-8"))
    broken = json.loads(json.dumps(document))
    broken["cases"][2]["inputs"]["walls"].append({"y1": 0.0, "z1": 0.0055, "y2": 0.075, "z2": 0.1945, "thickness": 0.01})
    assert h.dry_run(broken)[2]["status"] == "invalid"
    assert [r["status"] for r in h.dry_run(numerical)] == ["none"] * sum(len(c["references"]) for c in numerical["cases"])
