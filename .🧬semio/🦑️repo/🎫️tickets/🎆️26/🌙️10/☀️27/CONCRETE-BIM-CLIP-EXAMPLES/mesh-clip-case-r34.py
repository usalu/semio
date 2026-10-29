"""🔬 Retain an actual mesh cell whose intermediate rounding departs from GEOS."""
import ast,json,math
from pathlib import Path
import numpy as np
from scipy.spatial import cKDTree
from shapely.geometry import Polygon,box
path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py')
tree=ast.parse(path.read_text())
cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='BimAnalysisContract')
method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='test_mesh_nodes_match_independent_shapely_clipped_grid')
method.body=[node for node in method.body if not (isinstance(node,ast.Expr) and isinstance(node.value,ast.Call) and isinstance(node.value.func,ast.Attribute) and node.value.func.attr=='assertLessEqual')]
method.body.append(ast.Assign(targets=[ast.Attribute(value=ast.Name(id='self',ctx=ast.Load()),attr='node_sets',ctx=ast.Store())],value=ast.Tuple(elts=[ast.Name(id='actual',ctx=ast.Load()),ast.Name(id='expected',ctx=ast.Load())],ctx=ast.Load())))
space=namespace['BimAnalysisContract'].test_mesh_nodes_match_independent_shapely_clipped_grid.__globals__.copy()
exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),str(path),'exec'),space)
instance=namespace['BimAnalysisContract']()
space[method.name](instance)
actual,expected=(np.asarray(list(s),dtype=float) for s in instance.node_sets)
distance,nearest=cKDTree(expected).query(actual)
worst_index=int(np.argmax(distance))
worst=actual[worst_index]
node=next(n for n in instance.analysis['nodes'] if np.array_equal(n['position'],worst))
by_id={n['id']:n for n in instance.analysis['nodes']}
slab=instance.floor()
xs,ys=(instance.analysis['mesh'][key] for key in ('x_lines','y_lines'))
cases=[]
for shell in instance.analysis['shells']:
    if node['id'] not in shell['nodes']:
        continue
    _,part_index,j,i=shell['id'].split('-')
    part_index,j,i=map(int,(part_index,j,i))
    part=slab['parts'][part_index]
    lo_x,hi_x,lo_y,hi_y=xs[i],xs[i+1],ys[j],ys[j+1]
    clip=[[lo_x,lo_y],[hi_x,lo_y],[hi_x,hi_y],[lo_x,hi_y]]
    result=Polygon(part['outline']).intersection(box(lo_x,lo_y,hi_x,hi_y))
    rings=[list(map(list,shape.exterior.coords)) for shape in getattr(result,'geoms',[result]) if shape.geom_type=='Polygon']
    cases.append({'shell_id':shell['id'],'part_index':part_index,'source_slab':slab['tag'],'subject':part['outline'],'clip':clip,'expected_geos_rings':rings,'expected_geos_area':result.area,'published_actual':[by_id[n]['position'] for n in shell['nodes']],'mid_surface':shell['mid_surface']})
fixture={'source_provider':provider_hash,'publication_decimals':9,'publication_half_cell_radius_3d':math.sqrt(3)*.5e-9,'worst_actual_node':list(worst),'nearest_geos_node':list(expected[nearest[worst_index]]),'observed_distance':float(distance.max()),'cases':cases}
(Path(__file__).parent/'mesh-clip-r34.json').write_text(json.dumps(fixture,ensure_ascii=False,indent=2))
print('MESH_CLIP_CASE',fixture['worst_actual_node'],fixture['nearest_geos_node'],fixture['observed_distance'],len(cases),[(c['shell_id'],len(c['subject'])) for c in cases])
