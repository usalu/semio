"""🐍️ EN 1990 factor overrides, E90.10 to E90.12 and the Fig. 6.10 rail frequency limits, oracle role: an independent restatement of the annex policy, X_d = eta X_k / gamma_m, a_d = a_nom +- delta a, alpha_cr against the first-order limit and the closed Fig. 6.10 power laws, read from the committed documents through the shared norm step layer."""

import functools
import importlib.util
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent

def _limit_rule():
    for parent in pathlib.Path(__file__).resolve().parents:
        path = parent / "🔮️oracles" / "🏃️harness" / "📏️tolerance" / "🐍️.py"
        if path.is_file():
            spec = importlib.util.spec_from_file_location("norm_limit_rule", path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            return module
    raise FileNotFoundError("oracle tolerance module")


LIMIT = _limit_rule()

ALLOWED = {"En": {"gamma", "psi0", "psi1", "psi2", "xi"}, "De": {"psi0", "psi1", "psi2"}}
ANNEX_VALUE = {("psi0", "wind"): 0.6, ("gamma", "office"): 1.5}
ALPHA_CR_FIRST_ORDER = 10.0


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def override_rows(document):
    categories = {variable["id"]: variable["category"] for variable in document["variables"]}
    rows = {}
    for row in document["factorOverrides"]:
        kind, target, value = row["factor"], row["target"].strip(), row["value"]
        if kind not in ALLOWED[document["annex"]]:
            status = "fail"
        elif target and target not in categories:
            status = "fail"
        elif not row["justification"].strip():
            status = "fail"
        elif (kind, categories.get(target)) in ANNEX_VALUE and ANNEX_VALUE[(kind, categories[target])] != value:
            status = "warning"
        else:
            raise AssertionError("the restated policy has no annex value for %s on %s" % (kind, target))
        rows["en1990.override.%s" % row["id"]] = (status, 0.0)
    return rows


def member_rows(document):
    rows = {}
    for member in document["members"]:
        name = member["id"]
        xk, partial = member["materialStrengthK"], member["materialPartialFactor"]
        if xk > 0 and partial > 0 and member["materialDesignStrength"] > 0:
            eta = member["materialConversionFactor"] or 1.0
            utilisation = member["materialDesignStrength"] / (eta * xk / partial)
            rows["en1990.e10.material.%s" % name] = (LIMIT.limit_status(utilisation), utilisation)
        nominal, tolerance, design = member["nominalDimension"], abs(member["dimensionTolerance"]), member["designDimension"]
        if nominal > 0 and tolerance > 0 and design > 0:
            deviation = abs(design - nominal)
            utilisation = tolerance / deviation
            rows["en1990.e11.geometry.%s" % name] = ("pass" if deviation >= tolerance else "fail", max(utilisation, 1.0) if deviation < tolerance else utilisation)
        alpha = member["memberAlphaCr"]
        if alpha > 0:
            if member["secondOrderAnalysis"] >= 0.5:
                rows["en1990.e12.analysis.%s" % name] = ("info", 0.0)
            else:
                rows["en1990.e12.analysis.%s" % name] = ("pass" if alpha >= ALPHA_CR_FIRST_ORDER else "fail", max(ALPHA_CR_FIRST_ORDER / alpha, 1.0) if alpha < ALPHA_CR_FIRST_ORDER else ALPHA_CR_FIRST_ORDER / alpha)
    return rows


def frequency_limits(span):
    lower = 80.0 / span if span <= 20.0 else 23.58 * span**-0.592
    return {"lower-limit-hz": lower, "upper-limit-hz": 94.76 * span**-0.748}


def adapter():
    steps = _layer()

    @functools.lru_cache(maxsize=None)
    def outcome(path):
        document = json.load(open(path, encoding="utf8"))
        return {**override_rows(document), **member_rows(document)}

    def vector(name, quantity):
        return frequency_limits(float(name.rsplit("-", 1)[1]))[quantity]

    return steps.norm_adapter(steps.Binding(oracle_check=lambda fixture, check: steps.check_outcome(outcome(fixture), check), oracle_vector=vector))
