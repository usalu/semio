"""🐍️ EN 1995 check outcome, oracle role: the independent timber utilisations of `../../🔮️oracles/🚦️check-outcome/🐍️.py` (the compliance oracle plus the design-case families). The status and utilisation of one check id of a committed document, through the shared norm step layer."""

import functools
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
ORACLE = str(HERE.parents[1] / "🔮️oracles" / "🚦️check-outcome" / "🐍️.py")


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def outcomes(steps, path):
    answers = steps.run_oracle(ORACLE, ["--json"], json.load(open(path, encoding="utf8")))
    return {row["id"]: (unverified_or(steps.pass_or_fail(row["utilization"]), row), row["utilization"]) for row in answers["checks"]}


def unverified_or(status, row):
    """🚧 A pass whose inputs read the unverified `ndp.en1995.kmod.*` or `ndp.en1995.kdef.*` rows is `unverifiedBasis`; a fail stays a fail."""
    return "unverifiedBasis" if status == "pass" and row.get("reads") else status


def adapter():
    steps = _layer()
    outcome = functools.lru_cache(maxsize=None)(lambda path: outcomes(steps, path))

    return steps.norm_adapter(steps.Binding(oracle_check=lambda fixture, check: steps.check_outcome(outcome(fixture), check)))
