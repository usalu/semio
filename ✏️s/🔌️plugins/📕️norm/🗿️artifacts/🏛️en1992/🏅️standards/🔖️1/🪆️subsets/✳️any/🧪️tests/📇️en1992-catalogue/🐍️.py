"""🐍️ EN 1992 catalogue examples, oracle role: `../../🔮️oracles/⚖️compliance/🐍️.py --catalogue` (line protocol `entry|case|annex=..|n:k=v|s:k=v`) re-derives every hand-checked output through the shared norm step layer."""

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
    script = str(HERE.parents[1] / "🔮️oracles" / "⚖️compliance" / "🐍️.py")

    def oracle_example(entry, annex, numbers, texts):
        line = "|".join([entry, "feature", "annex=" + annex] + ["n:%s=%r" % item for item in numbers.items()] + ["s:%s=%s" % item for item in texts.items()])
        answers = {}
        for row in steps.run_oracle_text(script, ["--catalogue"], line + "\n").splitlines():
            _, _, key, value = row.split("|")
            answers[key] = float(value)
        return answers

    return steps.norm_adapter(steps.Binding(oracle_example=oracle_example))
