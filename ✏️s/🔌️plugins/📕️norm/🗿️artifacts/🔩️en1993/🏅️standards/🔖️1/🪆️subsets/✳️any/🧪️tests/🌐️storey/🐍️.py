"""🐍️ EN 1993 storey stability vectors, oracle role: `../../🔮️oracles/🌐️storey/🐍️.py` (stdlib formulas for alpha_cr, phi and q_d; scipy generalised eigenproblem for the lateral stiffness model) answers every quantity through the shared norm step layer."""

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
    answers = steps.run_oracle(str(HERE.parents[1] / "🔮️oracles" / "🌐️storey" / "🐍️.py"), [])

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: answers[quantity]))
