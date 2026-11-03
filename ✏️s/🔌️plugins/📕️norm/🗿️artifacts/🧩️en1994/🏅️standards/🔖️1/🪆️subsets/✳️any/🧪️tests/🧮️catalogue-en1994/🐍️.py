"""🐍️ EN 1994 catalogue worked examples, oracle role: the stdlib-only oracle `../../🔮️oracles/🧮️catalogue/🐍️.py` re-derives every published quantity (strip integration, double integration of the curvature, RK4 time stepping) through the shared norm step layer."""

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
    oracle = steps.load_module("catalogue_oracle", str(HERE.parents[1] / "🔮️oracles" / "🧮️catalogue" / "🐍️.py"))
    values = oracle.vectors()

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda example, quantity: values[quantity]))
