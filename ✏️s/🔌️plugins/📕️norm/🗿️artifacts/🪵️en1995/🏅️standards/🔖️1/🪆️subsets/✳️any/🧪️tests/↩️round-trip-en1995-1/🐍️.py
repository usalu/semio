"""🐍️ EN 1995 mutation round trip, oracle role: the shared mutation vocabulary engine (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution`) applies each committed mutation and its own computed inverse independently of the Rust producer, through the shared norm step layer."""

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
    mutate = steps.load_module("en1995_mutate_adapter", str(HERE.parent / "🪵️mutate-en1995-1" / "🐍️.py"))

    return steps.norm_adapter(steps.Binding(oracle_round_trip=steps.vocabulary_round_trip(mutate.KINDS)))
