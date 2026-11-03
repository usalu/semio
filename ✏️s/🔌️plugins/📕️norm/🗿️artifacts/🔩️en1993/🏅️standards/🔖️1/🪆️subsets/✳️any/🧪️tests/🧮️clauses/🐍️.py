"""🐍️ EN 1993 clause vectors, oracle role: `../../🔮️oracles/🧮️clauses/🐍️.py` (quadratic-root buckling factor, Galerkin M_cr, Heun fire integration, third-party `rainflow` package) answers every worked-example quantity E01 to E11 through the shared norm step layer."""

import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


ORACLE_KEYS = {
    "chi_z": "e01_chi_z",
    "Nb_Rd_z_na_de": "e01_nbrd_z_de",
    "Mcr_closed_form": "e02_mcr_closed",
    "chi_LT_6.3.2.2": "e02_chi_lt_234",
    "chi_LT_mod_6.3.2.3": "e02_chi_lt_mod",
    "Mcr_fe_uniform": "e02_mcr_uniform_galerkin",
    "Mcr_fe_udl": "e02_mcr_udl_galerkin",
    "b_eff": "e04_beff",
    "V_bw_Rd": "e07_vbw",
    "F_b_Rd": "e08_fb",
    "F_T2_Rd": "e10_ft2",
    "damage": "e11_damage",
    "rainflow_cycles": "rainflow_count",
    "rainflow_cycles_detail": "rainflow_count_fatigue_detail",
    "theta_a_300s": ("e12_theta", 0),
    "theta_a_600s": ("e12_theta", 1),
    "theta_a_900s": ("e12_theta", 2),
    "phi_sway": "e13_phi",
}


def _value(answers, quantity):
    key = ORACLE_KEYS[quantity]
    return answers[key[0]][key[1]] if isinstance(key, tuple) else answers[key]


def adapter():
    steps = _layer()
    script = str(HERE.parents[1] / "🔮️oracles" / "🧮️clauses" / "🐍️.py")
    answers = steps.run_oracle(script, [])

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: _value(answers, quantity)))
