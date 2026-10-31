"""⚡ Replay the production sparse-support test without loading the drawing engine."""
import ast
from pathlib import Path
import sys
from types import SimpleNamespace
import unittest
import numpy as np
import json
sys.dont_write_bytecode=True
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
MODEL=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
source=ast.parse((MODEL/'🎨️visuals/🐍️.py').read_text(encoding='utf-8'))
namespace={'np':np,'C':json.loads((MODEL/'🧪️cases/👁️occlusion/🔣️.json').read_text(encoding='utf-8'))}
helpers=[node for node in source.body if isinstance(node,ast.FunctionDef) and node.name=='_candidate_support']
exec(compile(ast.Module(body=helpers,type_ignores=[]),'sparse-support-production','exec'),namespace)
namespace['V']=SimpleNamespace(**{node.name:namespace[node.name] for node in helpers})
tests=ast.parse((MODEL/'🧪️tests/👁️occlusion/🐍️.py').read_text(encoding='utf-8'))
case=next(node for node in tests.body if isinstance(node,ast.ClassDef) and node.name=='Occlusion')
method=next(node for node in case.body if isinstance(node,ast.FunctionDef) and node.name=='test_sparse_support_intervals_match_portable_values_and_dense_numpy_oracle')
exec(compile(ast.Module(body=[method],type_ignores=[]),'sparse-support-permanent-test','exec'),namespace)
probe=type('SparseSupport',(unittest.TestCase,),{method.name:namespace[method.name]})
result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(probe))
raise SystemExit(0 if result.wasSuccessful() else 1)
