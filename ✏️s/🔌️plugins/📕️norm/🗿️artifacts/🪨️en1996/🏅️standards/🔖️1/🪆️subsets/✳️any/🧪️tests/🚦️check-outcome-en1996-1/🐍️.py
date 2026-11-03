"""🐍️ EN 1996 check outcome, oracle role: the independent per-wall utilisations of `../../🔮️oracles/🚦️check-outcome/🐍️.py --json`. The status and utilisation of one check id of a committed document, through the shared norm step layer."""

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
    def status(row):
        verdict = steps.pass_or_fail(row["utilization"])
        advisory = ".6.1.3.area." in row["id"] or any(f".8.6.chase-{kind}." in row["id"] for kind in ("depth", "width", "distance", "cumulative"))
        return "warning" if verdict == "fail" and advisory else verdict

    return {row["id"]: (status(row), row["utilization"]) for row in answers["checks"]}


def adapter():
    steps = _layer()
    outcome = functools.lru_cache(maxsize=None)(lambda path: outcomes(steps, path))

    return steps.norm_adapter(steps.Binding(oracle_check=lambda fixture, check: steps.check_outcome(outcome(fixture), check)))
