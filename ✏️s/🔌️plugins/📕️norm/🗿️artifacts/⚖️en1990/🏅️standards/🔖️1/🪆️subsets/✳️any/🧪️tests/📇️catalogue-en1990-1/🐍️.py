"""🐍️ EN 1990 catalogue fixture case, oracle role: `../../🔮️oracles/⚖️compliance/🐍️.py catalogue <fixture>` answers every committed catalogue case through the shared norm step layer."""

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
    cache = {}

    def fixture_cases(path):
        cache["path"] = path
        with open(path, encoding="utf8") as handle:
            return json.load(handle)

    def oracle_case(case):
        if "all" not in cache:
            cache["all"] = steps.run_oracle(script, ["catalogue", cache["path"]])
        pairs = cache["all"][case["id"]]
        return {"%s.%s" % (key, side): pair[position] for key, pair in pairs.items() if not key.startswith("_") for position, side in enumerate(("demand", "capacity"))}

    return steps.norm_adapter(steps.Binding(fixture_cases=fixture_cases, oracle_case=oracle_case))
