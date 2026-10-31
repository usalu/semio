"""🔬 Compare independently decoded native operands with original authority and the permanent SciPy/GEOS oracle."""
import copy,hashlib,importlib,json,math,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/native-analytic-difference-r95';S=O/'samples.json';N=O/'native-trees.json';M=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference/🧪️tests/🔬️scipy.py';samples=json.loads(S.read_text(encoding='utf8'));native=json.loads(N.read_text(encoding='utf8'));paths={S,N,M,Path(__file__)}


def hashes():
    """🔐 Bind native parser, selected code, original source schema/vectors and independent analytic oracle bytes."""
    result={str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}
    for path,value in {**native['source_inputs'],**native['validator_inputs']}.items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==value,path
    assert native['input_sha256']==hashlib.sha256(S.read_bytes()).hexdigest()
    return result


def binary(node):
    """🌳 Normalize sequential source differences into the exact native left-associated Boolean tree."""
    if node.get('operator')!='difference':return copy.deepcopy(node)
    result=binary(node['stock'])
    for cutter in node['cutters']:result={'operator':'difference','stock':result,'cutters':[{**copy.deepcopy(cutter),'operand':binary(cutter['operand'])}]}
    return result


def canonical(node,native=False):
    """📐 Compare exact native parameters and boundary topology, with no coordinate tolerance or hull substitution."""
    node=binary(node)
    if node.get('operator')=='difference':return {'operator':'difference','stock':canonical(node['stock'],native),'cutters':[{**{key:value for key,value in cutter.items() if key!='operand'},'operand':canonical(cutter['operand'],native)} for cutter in node['cutters']]}
    if node['kind']=='polyhedron':
        points=sorted(tuple(point) for point in node['vertices']);indices={point:index for index,point in enumerate(points)};faces=[]
        for face in node['faces']:
            ring=[indices[tuple(node['vertices'][index])] for index in face];faces.append(min(tuple(ring[index:]+ring[:index]) for index in range(len(ring))))
        return {'kind':'polyhedron','vertices':points,'faces':sorted(faces)}
    if node['kind']=='profile_extrusion':
        if native:
            orientation=node.pop('_native_orientation');assert orientation['profile']==[1,0];basis=orientation['extrusion']
        else:angle=math.radians(node['rotation']);basis=[math.cos(angle),math.sin(angle)]
        node.pop('rotation');node['orientation']=basis
    return node


def authority(node):
    """🧹 Remove observer-only native direction diagnostics while keeping the decoded physical expression unchanged."""
    result={key:copy.deepcopy(value) for key,value in node.items() if key!='_native_orientation'}
    if result.get('operator')=='difference':result['stock']=authority(result['stock']);result['cutters']=[{**row,'operand':authority(row['operand'])} for row in result['cutters']]
    return result


before=hashes();oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🧪️tests.🔬️scipy');expected={(row['case'],row['scope']):row for row in samples['samples']};rows=[];started=time.monotonic()
for item in native['rows']:
    row={'case':item['case'],'scope':item['scope']};original=expected[(item['case'],item['scope'])]
    try:
        assert canonical(item['operand'],True)==canonical(original['operand']),item['case']+' exact typed source parameter/topology mismatch'
        first,error_first=oracle.volume(original['operand']);second,error_second=oracle.volume(authority(item['operand']));bound=8*(error_first+error_second)+64*math.ulp(max(abs(first),abs(second)));assert abs(first-second)<=bound,(first,second,bound)
        if item['scope']=='Body':
            target=original['expected']
            if 'owned_volume' in target:assert abs(second-target['owned_volume'])<=bound+64*math.ulp(abs(target['owned_volume'])),(item['case'],second,target)
            if target.get('owned_volume_positive'):assert second>0
        row.update(success=True,exact_typed_parameters=True,source_volume=first,native_operand_volume=second,numerical_error_bound=bound)
    except Exception:row.update(success=False,error=traceback.format_exc())
    rows.append(row);print('[DEBUG] Native analytic oracle',row['case'],row['scope'],row['success'],flush=True)
after=hashes();report={'success':all(row['success'] for row in rows),'stable':before==after,'before':before,'after':after,'rows':rows,'seconds':time.monotonic()-started,'scope':'Native IFC SDK decoded typed analytic trees, exact source parameter/topology equality and independent real-curve SciPy/GEOS volume. No tessellation is used as material authority.'};(O/'independent-native-oracle.json').write_text(json.dumps(report,indent=2),encoding='utf8');assert report['success'] and report['stable']
