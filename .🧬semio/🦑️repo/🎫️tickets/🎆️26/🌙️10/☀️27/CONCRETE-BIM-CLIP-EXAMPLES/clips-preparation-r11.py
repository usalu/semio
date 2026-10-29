"""🧪 Execute actual chapter preparation methods without a rendering runtime."""
import ast
import importlib
import math
from collections import Counter
from pathlib import Path
from types import SimpleNamespace

ROOT = Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
MODEL = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def progress(stage):
    """⏱️ Keep chapter preparation phases externally observable while Python is busy."""
    import json
    path=Path(__file__).parent/'🗑️generated'/'clips-preparation-r30-progress.json'
    path.write_text(json.dumps({'debug':'[DEBUG] chapter preparation','stage':stage},ensure_ascii=False),encoding='utf-8')

def methods(folder, names, extra):
    """📦 Compile unchanged chapter methods against the physical provider."""
    progress(folder+":"+",".join(names))
    path = ROOT / folder / '🐍️.py'
    tree = ast.parse(path.read_text(encoding='utf-8'))
    scene = next(node for node in tree.body if isinstance(node, ast.ClassDef))
    selected = [node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name in names]
    for method in selected:
        observed=[]
        for position,statement in enumerate(method.body):
            if not (position==0 and isinstance(statement,ast.Expr) and isinstance(statement.value,ast.Constant) and isinstance(statement.value.value,str)):
                targets=','.join(ast.unparse(t) for t in statement.targets) if isinstance(statement,ast.Assign) else type(statement).__name__
                observed.append(ast.Expr(value=ast.Call(func=ast.Name(id='progress',ctx=ast.Load()),args=[ast.Constant(value=folder+':'+method.name+':'+str(position)+':'+targets)],keywords=[])))
            observed.append(statement)
        method.body=observed
    space = {'_model': MODEL, 'math': math, 'Counter': Counter, 'SimpleNamespace': SimpleNamespace, 'progress':progress, **extra}
    exec(compile(ast.fix_missing_locations(ast.Module(body=selected, type_ignores=[])), str(path), 'exec'), space)
    return type('Scene', (), {name: space[name] for name in names} | {'language': lambda self: 'en'})()

objects = methods('🧬️objects', ['_facts'], {'SELECTED': MODEL.role('column_example'), 'WALL': 'W-S'})
facts = objects._facts()
assert facts.count == len(MODEL.type_occurrences(MODEL.role('column_type_example')))
assert facts.sel['tag'] == MODEL.role('column_example')
print('PASS object preparation: actual role, type occurrence count, quantities')
exchange = methods('🔌️exchange', ['_facts', '_objects'], {'TAG': MODEL.role('column_example'), 'SLAB': MODEL.role('floor')})
facts = exchange._facts()
assert facts.b3['tag'] == MODEL.role('column_example')
assert facts.slab['tag'] == MODEL.role('floor')
print('PASS exchange preparation: actual source, analysis, STEP, IDS')
consistency = methods('🔗️consistency', ['_prepare', 'paving_state', 'layers_of', 'name', 'weights', 'q_of'], {'LINK': 'LK-ROOF'})
consistency._prepare()
consistency.tr = lambda en, de: en
assert math.isclose(consistency.q_of(consistency.paving_old), consistency.q_old, abs_tol=1e-8)
assert math.isclose(consistency.q_of(consistency.paving_new), consistency.q_new, abs_tol=1e-8)
assert math.isclose(consistency.paving_new, .14)
assert math.isclose(consistency.paving_alt, .18)
print('PASS consistency preparation: all revisions, source supports, regional paving loads')

import numpy as np
from shapely.geometry import Polygon

mesh = MODEL.mesh(MODEL.role('column_example'), 16)
vertices = np.asarray(mesh['vertices'])
triangles = np.asarray(mesh['faces'])
volume = abs(np.einsum('ij,ij->i', vertices[triangles[:, 0]], np.cross(vertices[triangles[:, 1]], vertices[triangles[:, 2]])).sum() / 6)
rim = mesh['part_layouts'][0]['bottom']
expected = Polygon(vertices[rim, :2]).area * (vertices[:, 2].max() - vertices[:, 2].min())
assert math.isclose(volume, expected, rel_tol=1e-9)
assert len(vertices) == 32
assert len(triangles) == 60
print('PASS independent NumPy/Shapely mesh volume and actual index counts')
roof = next(slab for slab in consistency.geometry['slabs'] if slab['tag'] == MODEL.role('roof'))
source_model=MODEL.load_model()
roof_source=next(e for e in source_model['elements'] if e['tag']==MODEL.role('roof',source_model))
domain=Polygon(roof_source['geometry']['outline'])
voids=[v['outline'] for v in roof_source['geometry'].get('voids',[])]+[e['geometry']['outline'] for e in source_model['elements'] if e['class']=='IfcOpeningElement' and e['geometry'].get('slab')==roof_source['tag']]
for hole in voids:
    domain=domain.difference(Polygon(hole))
area=domain.area
density = next(row[3] for row in consistency.weights(consistency.paving_new) if row[0] == 'paving')
expected = area * density * (consistency.paving_new - consistency.paving_old)
actual = sum(consistency.reaction_new.values()) - sum(consistency.reaction_old.values())
assert math.isclose(actual, expected, rel_tol=1e-8, abs_tol=1e-5)
print('PASS independent Shapely roof area times paving increment equals total reaction increment')

source=(ROOT/'📊️information'/'🐍️.py').read_text(encoding='utf-8')
scene=next(node for node in ast.parse(source).body if isinstance(node,ast.ClassDef))
pair_method=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=='_profile_pair')
space={'np':np,'math':math,'_model':MODEL}
exec(compile(ast.fix_missing_locations(ast.Module(body=[pair_method],type_ignores=[])),'information-profile-pair','exec'),space)
info=type('Info',(),{'_profile_pair':space['_profile_pair']})()
info._m=MODEL.load_model()
info._g=MODEL.building_geometry(info._m)
info._c=MODEL.clearance(info._m)
progress("information: physical pinch distances")
for item in MODEL.path_pinches(info._m,2):
    near,contact=info._profile_pair(item)
    x,y,heading=item['pose']
    c,s=math.cos(math.radians(heading)),math.sin(math.radians(heading))
    half=np.array([info._c['car_length'],info._c['car_width']])/2
    corners=np.array([[-half[0],-half[1]],[half[0],-half[1]],[half[0],half[1]],[-half[0],half[1]]])@np.array([[c,s],[-s,c]])+[x,y]
    car=Polygon(corners)
    expected=min(car.distance(Polygon(part['outline'],part.get('holes',[]))) for part in item['parts'] if part['z0']<info._m['car']['height']['value'] and part['z1']>0)
    assert math.isclose(float(np.linalg.norm(near-contact)),expected,abs_tol=1e-8)
    assert math.isclose(expected,item['clearance'],abs_tol=2e-4)
print('PASS actual pinch diagram nearest pairs against independent GEOS contours')
bay=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=='_bay')
stop=next(i for i,node in enumerate(bay.body) if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='rv' for t in node.targets))
bay.body=bay.body[:stop]+[ast.Return(value=ast.Tuple(elts=[ast.Name(id=name,ctx=ast.Load()) for name in ('radii','ra','rb','xa','xb','car_x','lim')],ctx=ast.Load()))]
space={'np':np,'math':math,'_model':MODEL,'BAY_K':.5,'BAY_X':3.65,'BAY_Y':.85}
exec(compile(ast.fix_missing_locations(ast.Module(body=[bay],type_ignores=[])),'information-bay-preparation','exec'),space)
info._entry=MODEL.car_path(info._m)['entry_pair']
tracker=lambda value:SimpleNamespace(get_value=lambda:value)
info.st=SimpleNamespace(s=tracker(info._c['spacing']),d=tracker(info._c['obstruction_width']),w=tracker(info._c['car_width']))
radii,ra,rb,xa,xb,car_x,limits=space['_bay'](info)
assert np.allclose(radii,[.165,.14],atol=1e-10)
assert math.isclose((xb-rb-xa-ra)/.5,limits['clear_width'],abs_tol=1e-10)
assert math.isclose(car_x,(xa+ra+xb-rb)/2,abs_tol=1e-10)
print('PASS actual bay preparation excludes buried square stems and centres the car in the physical opening')

import copy
models=methods('🧩️models',['_prepare'],{'copy':copy,'np':np,'DISCIPLINES':('architecture','structure','services'),'VGroup':lambda:[]})
models._prepare()
assert models.clash['b_tag']==MODEL.role('column_example')
print('PASS actual models preparation: all disciplines, native analysis, hypothetical source-role clash')
representations=methods('👁️representations',['_setup'],{'TAG':'W-S','_engine':SimpleNamespace(VGroup=lambda:[])})
representations._setup()
assert representations.m.window==representations.m.op['fill_tag']
assert math.isclose(representations.m.q0['volume'],sum(Polygon(part['outline']).area*(part['z1']-part['z0']) for part in representations.m.wall['parts']),abs_tol=1e-7)
print('PASS actual representations preparation: native filling identity and occupied regional material volume')
info_source=(ROOT/'📊️information'/'🐍️.py').read_text(encoding='utf-8')
info_class=next(node for node in ast.parse(info_source).body if isinstance(node,ast.ClassDef))
initial=next(node for node in info_class.body if isinstance(node,ast.FunctionDef) and node.name=='present_information')
stop=next(i for i,node in enumerate(initial.body) if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='raw' for t in node.targets))
initial.body=initial.body[:stop]
helpers=[node for node in info_class.body if isinstance(node,ast.FunctionDef) and node.name in ('_xy','_at','_grid_keys')]
space={'np':np,'math':math,'_model':MODEL,'SimpleNamespace':SimpleNamespace,'ORIGIN':np.array([-3.0,0.0,0.0]),'SCALE':.22,'_engine':SimpleNamespace(ValueTracker=lambda value:SimpleNamespace(get_value=lambda:value))}
exec(compile(ast.fix_missing_locations(ast.Module(body=[initial,*helpers],type_ignores=[])),'information-preparation','exec'),space)
info_class=type('Info',(),{name:space[name] for name in ('present_information','_xy','_at','_grid_keys')}|{'language':lambda self:'en','number':lambda self,value:str(value)})
prepared_info=info_class()
prepared_info.present_information()
assert prepared_info._entry==['2','1']
assert set(prepared_info._grid_keys('2'))=={'label_x:4','label_y:A'}
assert prepared_info._grid_keys('19')==[]
print('PASS actual information preparation: source entry pair, tracker inputs and coordinate-based grid identity')
