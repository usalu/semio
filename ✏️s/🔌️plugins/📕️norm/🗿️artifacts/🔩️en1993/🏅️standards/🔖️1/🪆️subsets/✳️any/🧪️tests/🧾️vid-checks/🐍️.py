"""🐍️ EN 1993 catalogue-gap checks, oracle role: `../../🔮️oracles/🧾️vids/🐍️.py` (numpy closed forms and the third-party `sectionproperties` finite element torsion and warping constants) answers every quantity through the shared norm step layer."""

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
    answers = steps.run_oracle(str(HERE.parents[1] / "🔮️oracles" / "🧾️vids" / "🐍️.py"), [])

    return steps.norm_adapter(steps.Binding(oracle_vector=lambda vector, quantity: answers[quantity]))
