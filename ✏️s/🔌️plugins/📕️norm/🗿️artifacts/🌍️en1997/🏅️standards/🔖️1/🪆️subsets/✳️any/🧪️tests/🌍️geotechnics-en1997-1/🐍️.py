"""🌍️ EN 1997 geotechnics design cases, oracle role: the groundhog and scipy quantities of `../../🔮️oracles/🌍️geotechnics/🐍️.py` for a committed case file, `<case>.<quantity>`, computed from the case inputs (the partial factors come from the NDP registry rows, never from the case)."""

import importlib.util
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
ORACLE = HERE.parents[1] / "🔮️oracles" / "🌍️geotechnics" / "🐍️.py"


def _layer():
    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def _oracle():
    spec = importlib.util.spec_from_file_location("en1997_geotechnics", ORACLE)
    module = importlib.util.module_from_spec(spec)
    sys.modules["en1997_geotechnics"] = module
    spec.loader.exec_module(module)
    return module


def computed(path):
    with open(path, encoding="utf8") as handle:
        document = json.load(handle)
    return {"%s.%s" % (row["id"], name): float(value) for row in _oracle().evaluate(document) for name, value in row["values"].items()}


def adapter():
    steps = _layer()
    return steps.norm_adapter(steps.Binding(oracle_comparison=computed))
