"""🪢 Replay portable production dependency ordering against NetworkX without drawing."""
import ast
import heapq
import json
from pathlib import Path
import sys
from types import SimpleNamespace
import unittest
sys.dont_write_bytecode=True
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
source=ast.parse((BASE/'🎨️visuals/🐍️.py').read_text(encoding='utf-8'))
functions=[node for node in source.body if isinstance(node,ast.FunctionDef) and node.name in ('_cyclic_components','_dependency_order')]
namespace={'heapq':heapq,'C':json.loads((BASE/'🧪️cases/👁️occlusion/🔣️.json').read_text(encoding='utf-8'))}
exec(compile(ast.Module(body=functions,type_ignores=[]),'production-dependency-functions','exec'),namespace)
namespace['V']=SimpleNamespace(**{node.name:namespace[node.name] for node in functions})
module=ast.parse((BASE/'🧪️tests/👁️occlusion/🐍️.py').read_text(encoding='utf-8'))
case=next(node for node in module.body if isinstance(node,ast.ClassDef) and node.name=='Occlusion')
method=next(node for node in case.body if isinstance(node,ast.FunctionDef) and node.name=='test_dependency_order_preserves_residual_component_dependencies')
exec(compile(ast.Module(body=[method],type_ignores=[]),'permanent-dependency-test','exec'),namespace)
probe=type('DependencyOrder',(unittest.TestCase,),{method.name:namespace[method.name]})
result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(probe))
raise SystemExit(0 if result.wasSuccessful() else 1)
