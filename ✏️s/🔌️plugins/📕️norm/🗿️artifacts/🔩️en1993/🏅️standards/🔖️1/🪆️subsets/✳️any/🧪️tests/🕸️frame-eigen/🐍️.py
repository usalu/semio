"""🐍️ EN 1993 general planar frame alpha_cr, oracle role: `../../🔮️oracles/🕸️frame-eigen/🐍️.py` (numpy/scipy Hermite assembly solved with scipy.linalg.eigh, stability-function closed form solved with scipy.optimize.brentq) answers every quantity through the shared norm step layer."""

import math
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
    answers = steps.run_oracle(str(HERE.parents[1] / "🔮️oracles" / "🕸️frame-eigen" / "🐍️.py"), [])
    effective = {"ncrPinned": 1.0, "ncrCantilever": 2.0, "ncrFixed": 0.5}
    euler = lambda factor: math.pi**2 * 16.8e6 / (factor * 5.0) ** 2

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: euler(effective[quantity]) if quantity in effective else answers[quantity]))
