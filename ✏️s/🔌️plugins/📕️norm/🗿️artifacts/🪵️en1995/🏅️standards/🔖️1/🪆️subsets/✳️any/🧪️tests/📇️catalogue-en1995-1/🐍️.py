"""🐍️ EN 1995 catalogue fixture case, oracle role: the independent Python formulation (`../../🔮️oracles/⚖️compliance/🐍️.py --catalogue`) answers every committed catalogue case through the shared norm step layer. The Rust subject adapter beside this file judges the kernel against the same answers."""

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


def adapter():
    steps = _layer()
    script = str(HERE.parents[1] / "🔮️oracles" / "⚖️compliance" / "🐍️.py")

    def fixture_cases(path):
        with open(path, encoding="utf8") as handle:
            return json.load(handle)["cases"]

    def oracle_case(case):
        return steps.run_oracle(script, ["--catalogue"], {"cases": [case]})["cases"][0]["values"]

    return steps.norm_adapter(steps.Binding(fixture_cases=fixture_cases, oracle_case=oracle_case))
