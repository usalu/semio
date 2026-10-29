"""🔬 Inspect independent GEOS versus published nine-decimal clipped-grid coordinates."""
import ast
from pathlib import Path
import numpy as np
from scipy.spatial import cKDTree
path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py')
tree=ast.parse(path.read_text())
cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='BimAnalysisContract')
method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='test_mesh_nodes_match_independent_shapely_clipped_grid')
method.body.append(ast.Assign(targets=[ast.Attribute(value=ast.Name(id='self',ctx=ast.Load()),attr='node_sets',ctx=ast.Store())],value=ast.Tuple(elts=[ast.Name(id='actual',ctx=ast.Load()),ast.Name(id='expected',ctx=ast.Load())],ctx=ast.Load())))
space=namespace['BimAnalysisContract'].test_mesh_nodes_match_independent_shapely_clipped_grid.__globals__.copy()
exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),str(path),'exec'),space)
instance=namespace['BimAnalysisContract']()
space[method.name](instance)
actual,expected=(np.asarray(list(s),dtype=float) for s in instance.node_sets)
a=cKDTree(actual).query(expected)[0]
b=cKDTree(expected).query(actual)[0]
print('NODE_PRECISION',len(actual),len(expected),'max',float(a.max()),float(b.max()),'p99',float(np.quantile(a,.99)),float(np.quantile(b,.99)))
