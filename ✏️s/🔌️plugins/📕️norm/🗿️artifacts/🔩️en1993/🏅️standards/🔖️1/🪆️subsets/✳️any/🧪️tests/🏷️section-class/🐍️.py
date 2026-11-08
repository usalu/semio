"""🐍️ EN 1993 section classes, shear resistance and Annex A, oracle role: `🧮️reference/🐍️.py` (closed forms from the standard, section area from `sectionproperties`) answers every quantity."""

import importlib.util
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent


def _layer():
    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def adapter():
    steps = _layer()
    answers = steps.run_oracle(str(HERE / "🧮️reference" / "🐍️.py"), [])
    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: answers[quantity]))
