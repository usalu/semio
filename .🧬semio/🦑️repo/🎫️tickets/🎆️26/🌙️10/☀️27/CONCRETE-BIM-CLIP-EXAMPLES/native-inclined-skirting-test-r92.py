"""🧪 Verify oblique occupied sections and sloped stock against independent GEOS/SciPy."""
import importlib,json,runpy,sys
import numpy as np
from pathlib import Path
from copy import deepcopy
from fractions import Fraction
from shapely.geometry import Polygon
from scipy.spatial import ConvexHull
from jsonschema import Draft202012Validator
root=Path.cwd();base=Path(__file__).parent
compile((base/'native-skirting-build-r92.py').read_text(encoding='utf8'),str(base/'native-skirting-build-r92.py'),'exec')
sys.path.insert(0,str(root))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
f=api._surface_finishes();helper=runpy.run_path(str(base/'native-inclined-skirting-r92.py'))
cases=json.loads((base/'inclined-skirting-cases-r92.json').read_text())
Draft202012Validator({'type':'object','required':['cases'],'properties':{'cases':{'type':'array','minItems':2,'items':{'type':'object','required':['id','bounds','plane','above','expected']}}}}).validate(cases)
case=cases['raw_reference'];raw=deepcopy(case['part']);raw['mesh']['vertices']=[[Fraction(v) for v in p] for p in raw['mesh']['vertices']];before=deepcopy(raw)
envelope=helper['native_envelope'](api,raw);assert raw==before and envelope['mesh'] is raw['mesh']
wire=helper['source_part'](envelope);assert set(wire)=={'mesh'} and wire['mesh'] is raw['mesh'];assert api.part_vertices(wire)==api.part_vertices(envelope)
assert [envelope['z0'],envelope['z1']]==case['bounds'];assert Polygon(envelope['outline']).symmetric_difference(Polygon(case['footprint'])).area==0
for case in cases['collinear']:
 actual=helper['stock_boundary'](case['points']);assert actual==case['expected'];assert Polygon(actual).symmetric_difference(Polygon(case['points'])).area==0
for case in cases['placement']:assert list(helper['placement_modes'](case['kind']))==case['expected']
for case in cases['cases']:
 x0,y0,z0,x1,y1,z1=case['bounds'];part={'outline':[[x0,y0],[x1,y0],[x1,y1],[x0,y1]],'z0':z0,'z1':z1};before=deepcopy(part)
 shape=helper['section_xy'](api,part,helper['affine_plane'](case['plane']),case['above']);expected=Polygon(case['expected'])
 assert abs(shape.area-expected.area)<max(1e-30,expected.area*1e-14),(case['id'],shape.wkt)
 assert shape.symmetric_difference(expected).area<max(1e-30,expected.area*1e-14)
 assert part==before
case=cases['band'];actual=helper['occupied_band'](api,case['part'],case['low'],case['high']);expected=Polygon(case['expected']);assert actual.symmetric_difference(expected).area<1e-15
case=cases['horizontal_concave'];actual=helper['inclined_occupied'](api,case['part'],helper['affine_plane'](case['plane']),case['height']);assert actual.symmetric_difference(Polygon(case['part']['outline'])).area==0
flat={'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':0,'z1':1}
plane=helper['affine_plane']([[0,0,0],[1,0,0],[0,1,.5]])
actual=helper['inclined_occupied'](api,flat,plane,.09)
assert actual.symmetric_difference(Polygon(flat['outline'])).area<1e-15
stock=f.skirting_stock([0,0,0],[0,2,1],[1,0,0],{'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8},'timber',back_clearance=.0001)
stock=helper['seat_stock'](f,stock,helper['affine_plane']([[0,0,0],[0,2,1],[1,0,0]]))
count=len(stock._origin['mesh']['vertices'])//2
for index in (0,1,count,count+1):
 p=stock._origin['mesh']['vertices'][index];assert p[2]*2==p[1]
api._polyhedron_part(stock['mesh']['vertices'],stock['mesh']['faces'])
assert abs(api._polyhedron_volume(stock)-ConvexHull(api.part_vertices(stock)).volume)<1e-15

case=cases['extension_only'];extension_stock=f.skirting_stock(*case['axis'],case['normal'],{'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8},'timber',extensions=case['extensions']);assert len(helper['fit_support'](f,extension_stock,Polygon(case['support']),case['axis'][0],np.asarray(case['axis'][1])-case['axis'][0]))==case['expected_parts']
case=cases['joint_fitting'];caps=[{'normal':[0,-1,0],'offset':0,'point':[0,0,0]},{'normal':[0,1,0],'offset':2,'point':[0,2,0]}]
assert helper['fitted_joints'](caps,0)==caps
stock=f.skirting_stock(*case['axis'],case['normal'],{'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8},'timber')
part=f.clip_stock(stock,helper['fitted_joints'](caps,case['clearance']),exact=True)
assert abs(min(p[1] for p in api.part_vertices(part))-case['expected_bounds'][0])<1e-15 and abs(max(p[1] for p in api.part_vertices(part))-case['expected_bounds'][1])<1e-15
assert ConvexHull(api.part_vertices(part)).volume>0
tiny=f.skirting_stock([0,0,0],[0,case['tiny_length'],0],case['normal'],{'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8},'timber');tiny_caps=[{'normal':[0,-1,0],'offset':0,'point':[0,0,0]},{'normal':[0,1,0],'offset':case['tiny_length'],'point':[0,case['tiny_length'],0]}]
assert f.clip_stock(tiny,helper['fitted_joints'](tiny_caps,0),exact=True) is not None
assert f.clip_stock(tiny,helper['fitted_joints'](tiny_caps,case['clearance']),exact=True) is None
case=cases['collinear_face'];assert helper['height'](helper['affine_plane'](case['points']),*case['probe'])==case['height']
case=cases['flight_plane'];assert helper['flight_face_plane'](case['wearing'],case['flight']) is not None;assert helper['flight_face_plane'](case['decoy'],case['flight']) is None;assert helper['flight_face_plane'](case['vertical'],case['flight']) is None
case=cases['edge_host'];assert helper['host_near_support'](Polygon(case['host']),Polygon(case['support']).buffer(-.00005,join_style=2),case['thickness'])==case['expected']
case=cases['full_section']
stock=f.skirting_stock(*case['axis'],case['normal'],{'thickness':.015,'height':.09,'round_radius':.002,'round_segments':8},'timber',back_clearance=.0001)
space=Polygon(case['support']);direction=np.asarray(case['axis'][1])-case['axis'][0]
parts=helper['fit_support'](f,stock,space,case['axis'][0],direction);assert len(parts)==1
assert abs(min(v[1] for v in parts[0]['mesh']['vertices'])-case['kept_y'][0])<1e-15
assert abs(max(v[1] for v in parts[0]['mesh']['vertices'])-case['kept_y'][1])<1e-15
base=ConvexHull(api.part_vertices(stock)).volume
assert abs(ConvexHull(api.part_vertices(parts[0])).volume-base*case['volume_ratio'])<1e-15
print('[DEBUG] actual band/section + full nominal stock fit SciPy/GEOS GREEN',flush=True)
