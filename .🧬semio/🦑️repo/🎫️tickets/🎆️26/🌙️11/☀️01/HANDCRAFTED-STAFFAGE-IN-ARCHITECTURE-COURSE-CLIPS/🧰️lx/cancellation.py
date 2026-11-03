"""Run the staffage Cancellation tests on Linux without manim/numpy: stub those two imports, load the real module and the real test class."""

import ast
import importlib.util
import sys
import types
import unittest
from pathlib import Path

ROOT = Path(sys.argv[1])
VISUALS = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage"

stub = types.ModuleType("manim")
for name in ("ORIGIN", "VectorizedPoint", "VGroup", "VMobject"):
    setattr(stub, name, object)
sys.modules["manim"] = stub
numpy = types.ModuleType("numpy")
numpy.ndarray = object
sys.modules["numpy"] = numpy

spec = importlib.util.spec_from_file_location("staffage_under_test", VISUALS / "🐍️.py")
S = importlib.util.module_from_spec(spec)
sys.modules["staffage_under_test"] = S
spec.loader.exec_module(S)

source = (VISUALS / "🧪️tests/🐍️.py").read_text(encoding="utf-8")
tree = ast.parse(source)
wanted = {"alive", "snapshot", "Cancellation"}
body = [node for node in tree.body if isinstance(node, (ast.FunctionDef, ast.ClassDef)) and node.name in wanted]
scope = {"S": S, "__name__": "cancellation_under_test"}
header = "import _thread, contextlib, io, json, os, re, subprocess, sys, tempfile, threading, time, unittest\nfrom pathlib import Path\nfrom unittest.mock import patch\n"
exec(compile(ast.Module(body=ast.parse(header).body + body, type_ignores=[]), "cancellation_under_test", "exec"), scope)

suite = unittest.defaultTestLoader.loadTestsFromTestCase(scope["Cancellation"])
result = unittest.TextTestRunner(verbosity=2).run(suite)
sys.exit(0 if result.wasSuccessful() else 1)
