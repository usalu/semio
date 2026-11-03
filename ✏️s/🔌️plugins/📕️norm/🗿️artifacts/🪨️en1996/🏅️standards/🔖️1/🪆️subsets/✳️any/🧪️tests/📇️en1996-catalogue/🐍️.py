"""🐍️ EN 1996 catalogue examples, oracle role: `../../🔮️oracles/⚖️compliance/🐍️.py` (mode catalogue) re-derives every hand-checked quantity through the shared norm step layer."""

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

    def oracle_example(entry, annex, numbers, texts):
        scalars, vectors = {}, {}
        for key, value in numbers.items():
            name, _, index = key.rpartition(".")
            if index.isdigit():
                vectors.setdefault(name, {})[int(index)] = value
            else:
                scalars[key] = value
        request = {"mode": "catalogue", "cases": [{"id": "feature", "annex": annex, "entry": entry, "num": scalars, "text": texts, "vec": {name: [items[i] for i in sorted(items)] for name, items in vectors.items()}}]}
        return steps.run_oracle(script, [], request)["results"]["feature"]

    return steps.norm_adapter(steps.Binding(oracle_example=oracle_example))
