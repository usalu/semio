"""🎯️ EN 1997 derived characteristic values, oracle role: the independent Python ground derivation (`../../🔮️oracles/🎯️ground-derivation/🐍️.py`) answers every worked example of the committed fixture through the shared norm step layer; `<layer>.<parameter>.<field>` names a derived value (`n`, `mean`, `kn`, `characteristic`)."""

import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = HERE.parent / "🧫️fixtures" / "🎯️ground-derivation" / "🔣️.json"


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
    script = str(HERE.parents[1] / "🔮️oracles" / "🎯️ground-derivation" / "🐍️.py")
    derived = {}

    def oracle_vector(example, quantity):
        if not derived:
            derived.update(steps.run_oracle(script, [str(FIXTURE)]))
        key, field = quantity.rsplit(".", 1)
        return derived[example][key][field]

    return steps.norm_adapter(steps.Binding(oracle_vector=oracle_vector))
