"""🐍️ EN 1999 oracle comparison, oracle role: the blessed quantities of a committed oracle case file, `<case>.<quantity>`, without the checks the production registry has no evaluator for and the documented known differences (read from the `oracle_wiring` source beside the subject, so the two sides cannot drift apart)."""

import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
WIRING = HERE.parents[1] / "🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs"


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def _registry():
    text = WIRING.read_text(encoding="utf8")
    checks = set(re.findall(r'Adapter \{\s*check: "([^"]+)"', text))
    known = set(re.findall(r'KnownDifference \{\s*case: "([^"]+)",\s*quantity: "([^"]+)"', text))
    return checks, known


def blessed(path):
    checks, known = _registry()
    with open(path, encoding="utf8") as handle:
        document = json.load(handle)
    return {
        "%s.%s" % (case["id"], name): float(entry["value"])
        for case in document["cases"]
        if case["check"] in checks
        for name, entry in case["expected"].items()
        if (case["id"], name) not in known
    }


def adapter():
    steps = _layer()
    return steps.norm_adapter(steps.Binding(oracle_comparison=blessed))
