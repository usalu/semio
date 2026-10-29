"""🧮 Run physical analysis regressions with independent geometry and numerical references."""
import importlib

suite = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.📏️physical")
for name in sorted(vars(suite)):
    if name.startswith("test_"):
        getattr(suite, name)()
        print("PASS", name)
