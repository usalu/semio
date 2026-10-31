"""🔎 Validate the bounded coal-bay floor against independent native material and wearing support."""
import hashlib,importlib,importlib.util,json,sys,time
from pathlib import Path
from unittest.mock import patch
import jsonschema
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
sys.path.insert(0,str(Path.cwd()))
B=Path(__file__).parent;O=B/'🗑️generated/native-r92'
spec=importlib.util.spec_from_file_location('coal_author',B/'coal-bay-floor-author-r94.py');H=importlib.util.module_from_spec(spec);spec.loader.exec_module(H)
A=H.A;started=time.perf_counter();m=A.load_model();candidate=json.loads((O/'coal-bay-floor-candidate-r94.json').read_text())
assert hashlib.sha256(A._source().encode('utf-8')).hexdigest()==candidate['source_sha256']
old={tag:A._solid_geometry(m,A._find(m,tag))['parts'] for tag in candidate['additions']}
H.apply(m,candidate)
schema=json.loads(Path(A.__file__).with_name('🧬️schema').joinpath('🔣️.schema.json').read_text(encoding='utf-8'))
validator=jsonschema.Draft202012Validator(schema)
allparts=[];native=0.;oracle=0.
for tag,parts in candidate['additions'].items():
    e=A._find(m,tag)
    jsonschema.Draft202012Validator({'$ref':'#/$defs/element','$defs':schema['$defs']}).validate(e)
    for p in parts:
        vertices=np.asarray(A.part_vertices(p),dtype=float)
        native_part=A._polyhedron_part(vertices.tolist(),A.part_faces(p))
        origin=vertices.min(axis=0);scale=np.ptp(vertices,axis=0)
        assert np.all(scale>0)
        v=float(ConvexHull((vertices-origin)/scale).volume*np.prod(scale))
        volume=A._polyhedron_volume(native_part)
        assert abs(v-volume)<max(1e-15,v*1e-11),(v,volume)
        native+=volume;oracle+=v;allparts.append(p)
all_new=unary_union([Polygon(p['outline']) for p in allparts if p['z0']==-.13])
assert abs(all_new.area-candidate['target_area'])<1e-12
geometry=json.loads((O/'skirting-geometry.json').read_text())['geometry']
collisions=[]
for group in ['slabs','walls','screens','beams','pilotis','details','furnishings','services']:
    for owner in geometry[group]:
        for j,q in enumerate(owner['parts']):
            if q['z0']>=0 or q['z1']<=-.13 or 'outline' not in q:
                continue
            shape=Polygon(q['outline'],q.get('holes',[]))
            if not shape.intersects(all_new):
                continue
            for i,p in enumerate(allparts):
                dz=min(p['z1'],q['z1'])-max(p['z0'],q['z0'])
                if dz<=0:
                    continue
                volume=Polygon(p['outline']).intersection(shape).area*dz
                if volume>1e-14:
                    collisions.append([group,owner['tag'],j,i,volume])
assert not collisions,collisions
for i,p in enumerate(allparts):
    for j,q in enumerate(allparts[:i]):
        dz=min(p['z1'],q['z1'])-max(p['z0'],q['z0'])
        if dz>0:
            assert Polygon(p['outline']).intersection(Polygon(q['outline'])).area*dz<1e-14,(i,j)
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪵️coal-bay')
with patch.object(T.A,'load_model',lambda:m):
    T.test_coal_bay_indoor_finished_floor_supports_all_cupboard_feet_and_front()
plate=next(p for p in A._find(m,'SL-GROUND-CELLAR')['geometry']['regions'] if p['tag']=='R-SL-GROUND-COAL-CEM-1')
cover=A._find(m,'COVER-COAL')
assert plate['top_offset']['value']==cover['geometry']['top_offset']['value']==-.13
plate_poly=Polygon(plate['outline']);assert all_new.difference(plate_poly).area<1e-12
volume_slices=0.
z=sorted({p[k] for p in allparts for k in ['z0','z1']})
for z0,z1 in zip(z,z[1:]):
    polygons=[Polygon(p['outline']) for p in allparts if p['z0']<=(z0+z1)/2<=p['z1']]
    volume_slices+=unary_union(polygons).area*(z1-z0)
assert abs(volume_slices-native)<1e-12,(volume_slices,native)
report={'success':True,'source_sha256':candidate['source_sha256'],'candidate_sha256':hashlib.sha256((O/'coal-bay-floor-candidate-r94.json').read_bytes()).hexdigest(),'parts':len(allparts),'target_area':candidate['target_area'],'native_volume':native,'scipy_volume':oracle,'geos_strata_volume':volume_slices,'actual_cupboard_feet_and_front':'PASS','retained_plate_and_cover_top':-.13,'positive_native_owner_overlaps':collisions,'duration_seconds':time.perf_counter()-started}
(O/'coal-bay-floor-physical-r94.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps(report),flush=True)
