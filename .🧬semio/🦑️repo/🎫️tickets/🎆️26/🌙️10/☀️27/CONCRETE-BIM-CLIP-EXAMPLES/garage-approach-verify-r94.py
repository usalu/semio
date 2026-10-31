"""🔬 Prove one unpublished garage surface against native, GEOS, SciPy and public quantity controls."""
from __future__ import annotations
import importlib,importlib.util,json,math,sys,time,traceback,hashlib
from fractions import Fraction as Q
from pathlib import Path
from collections import Counter
from unittest.mock import patch
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,Point
ROOT=Path.cwd();TICKET=Path(__file__).parent;sys.path.insert(0,str(ROOT));OUT=TICKET/'🗑️generated/native-r92'
spec=importlib.util.spec_from_file_location('garage_author',TICKET/'garage-approach-authoring-r94.py');author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author)
API=author.API;CASE=author.CASE;START=time.monotonic();candidate_path=OUT/'garage-approach-candidate-r94.json';candidate=json.loads(candidate_path.read_text(encoding='utf-8'))


def oracle_volume(part,plane):
    """🔬 Integrate actual Float64 vertices in an exact local affine frame before the independent SciPy hull."""
    vertices=[[Q(v) for v in point] for point in API.part_vertices(part)];ring=[[Q(v) for v in point] for point in part['outline']];a,b,_=plane
    u=max(([q[0]-p[0],q[1]-p[1]] for p,q in zip(ring,ring[1:]+ring[:1])),key=lambda v:v[0]*v[0]+v[1]*v[1]);det=u[0]*u[0]+u[1]*u[1];origin=vertices[0]
    coordinates=[[(x-origin[0])*u[0]+(y-origin[1])*u[1],-(x-origin[0])*u[1]+(y-origin[1])*u[0],z-origin[2]-a*(x-origin[0])-b*(y-origin[1])] for x,y,z in vertices]
    low=[min(point[k] for point in coordinates) for k in range(3)];ranges=[max(point[k] for point in coordinates)-low[k] for k in range(3)]
    if min(ranges)<=0:raise AssertionError('A native cell loses one positive dimension')
    normalized=np.asarray([[float((point[k]-low[k])/ranges[k]) for k in range(3)] for point in coordinates],float)
    return ConvexHull(normalized).volume*float(ranges[0]*ranges[1]*ranges[2]/det)


def prove():
    """🛡️ Require physical geometry and actual native partition proof before any public additive quantity."""
    from jsonschema import Draft202012Validator
    patches,error=author.construction();surfaces={row['name']:row['plane'] for row in patches};surfaces['flat']=(Q(0),Q(0),author.q(CASE['reference_elevation']))
    edge_owners={}
    for row in patches:
        for a,b in zip(row['ring'],row['ring'][1:]+row['ring'][:1]):edge_owners.setdefault(tuple(sorted((tuple(a),tuple(b)))),[]).append(row['plane'])
    for edge,owners in edge_owners.items():
        if len(owners)>2:raise AssertionError('Overowned surface patch edge')
        target=owners[1] if len(owners)==2 else surfaces['flat']
        for point in (*edge,tuple((a+b)/2 for a,b in zip(*edge))):
            assert sum(v*q for v,q in zip(owners[0],(*point,Q(1))))==sum(v*q for v,q in zip(target,(*point,Q(1))))
    model=API.load_model();obj=author.apply_candidate(model,candidate);schema=json.loads((Path(API.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf-8'))
    Draft202012Validator({'$defs':schema['$defs'],'$ref':'#/$defs/element'}).validate(obj)
    actual=[];volumes=[];oracles=[];areas=[];max_height_error=0.;tiny=1.;source_areas=[]
    assert len(candidate['parts'])==len(candidate['cells'])
    for index,(part,row) in enumerate(zip(candidate['parts'],candidate['cells'])):
        author.guard();vertices=API.part_vertices(part);faces=API.part_faces(part)
        try:native=API._polyhedron_part(vertices,faces)
        except Exception:
            (OUT/'garage-approach-rejected-part-r94.json').write_text(json.dumps({'index':index,'row':row,'part':part,'error':traceback.format_exc()},ensure_ascii=True,indent=2),encoding='utf-8');raise
        edges=Counter()
        for face in faces:
            for a,b in zip(face,face[1:]+face[:1]):
                assert tuple(vertices[a])!=tuple(vertices[b]),('collapsed native edge',index,a,b)
                edges[tuple(sorted((a,b)))]+=1
        assert edges and set(edges.values())=={2},('open native cell',index)
        volume=API._polyhedron_volume(native);independent=oracle_volume(native,surfaces[row['patch']]);assert volume>0 and independent>0
        assert math.isclose(volume,independent,rel_tol=2e-10,abs_tol=0),('native/scipy volume',index,volume,independent)
        polygon=Polygon(native['outline']);assert polygon.is_valid and polygon.area>0 and Polygon(CASE['stock']).covers(polygon)
        pair={}
        for x,y,z in vertices:pair.setdefault((x,y),[]).append(z)
        a,b,c=surfaces[row['patch']]
        for (x,y),values in pair.items():
            expected=float(a*Q(x)+b*Q(y)+c);max_height_error=max(max_height_error,abs(max(values)-expected));assert math.isclose(max(values)-min(values),CASE['depth'],abs_tol=CASE['comparison']['coordinate'])
        actual.append({**part,**native});volumes.append(volume);oracles.append(independent);areas.append(polygon.area);tiny=min(tiny,volume)
        if index%250==0:print('[DEBUG] garage native '+str(index)+'/'+str(len(candidate['parts'])),flush=True)
    original_area=math.fsum(Polygon(ring).area for ring in author.INPUT['reserved_cells']);actual_area=math.fsum(areas);expected_volume=original_area*CASE['depth']
    assert math.isclose(actual_area,original_area,abs_tol=1e-9,rel_tol=0),(actual_area,original_area)
    assert math.isclose(math.fsum(volumes),expected_volume,abs_tol=CASE['comparison']['volume'],rel_tol=0)
    assert math.isclose(math.fsum(oracles),expected_volume,abs_tol=CASE['comparison']['volume'],rel_tol=0)
    assert max_height_error<=CASE['comparison']['coordinate']
    events=[];partitioned=API._parts_disjoint(actual,progress=events.append,cancelled=lambda:(OUT/'garage-approach-cancel').exists())
    assert partitioned,'Actual published XY/Z cells are not proven disjoint; quantity fast path must not be waived'
    tests=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌿️gravel')
    with patch.object(API,'load_model',return_value=model):
        for name in sorted(dir(tests)):
            if name.startswith('test_') and callable(getattr(tests,name)):getattr(tests,name)()
    union=API._occupied_volume;calls=[]
    def forbidden_union(parts):
        calls.append(len(parts));raise AssertionError('A proven candidate unexpectedly invokes expensive generic union')
    try:
        API._occupied_volume=forbidden_union;before=time.monotonic();quantity=API.quantities(obj['tag'],model);quantity_seconds=time.monotonic()-before
    finally:API._occupied_volume=union
    assert not calls and math.isclose(quantity['volume'],expected_volume,abs_tol=CASE['comparison']['volume'],rel_tol=0)
    assert math.isclose(quantity['material_quantities'][0]['volume'],expected_volume,abs_tol=CASE['comparison']['volume'],rel_tol=0)
    report={'success':True,'candidate_sha256':hashlib.sha256(candidate_path.read_bytes()).hexdigest(),'parts':len(actual),'minimum_native_volume':tiny,'original_geos_area':original_area,'actual_geos_area':actual_area,'native_volume':math.fsum(volumes),'scipy_volume':math.fsum(oracles),'expected_volume':expected_volume,'exact_patch_C0':True,'maximum_native_height_error':max_height_error,'maximum_declared_facet_error':CASE['approach']['maximum_vertical_facet_error'],'constructed_facet_bound':error,'actual_native_parts_disjoint':partitioned,'public_quantity_volume':quantity['volume'],'public_quantity_seconds':quantity_seconds,'generic_union_calls':len(calls),'seconds':time.monotonic()-START,'peak_rss_bytes':author.PEAK,'publication':'NOT PUBLISHED; independent ENDE SDK still required'}
    (OUT/'garage-approach-physical-green-r94.json').write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf-8');print('[DEBUG] garage physical '+json.dumps(report),flush=True)


if __name__=='__main__':
    try:prove()
    except BaseException:
        (OUT/'garage-approach-physical-failure-r94.json').write_text(json.dumps({'error':traceback.format_exc(),'elapsed':time.monotonic()-START},ensure_ascii=True,indent=2),encoding='utf-8');raise
