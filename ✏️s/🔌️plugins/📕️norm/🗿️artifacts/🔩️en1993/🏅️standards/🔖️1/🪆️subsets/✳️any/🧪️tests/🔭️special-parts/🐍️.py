"""🐍️ EN 1993 special-part clause vectors and wired checks, oracle role: `../../🔮️oracles/🧮️special-parts/🐍️.py` (numpy and scipy: brentq buckling factors, fixed-point distortional iteration, linprog base plate equilibrium, quad Janssen integrals, interp1d tables) answers every quantity through the shared norm step layer."""

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
    script = str(HERE.parents[1] / "🔮️oracles" / "🧮️special-parts" / "🐍️.py")
    answers = steps.run_oracle(script, [])

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: answers[quantity], oracle_utilisation=lambda subject, annex, check, quantity: answers[quantity]))
