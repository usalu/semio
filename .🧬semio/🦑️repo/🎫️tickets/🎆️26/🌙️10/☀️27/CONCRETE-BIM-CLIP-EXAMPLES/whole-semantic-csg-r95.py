"""🧪 Require authoritative semantic capture and geometric CSG exclusions through the actual whole-model auditor."""
import hashlib,importlib,importlib.util,json,math,sys,time,traceback
from copy import deepcopy
from pathlib import Path
sys.dont_write_bytecode=True
R=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(R));T=Path(__file__).parent;D=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference';paths=[Path(__file__),T/'whole-expanded-interference-r92.py',T/'analytic-difference-intersections-r95.json',D/'🧬️.schema.json',D/'🧪️tests/🔣️.json',D/'🧪️tests/🔬️scipy.py']
def hashes():
    """🔐 Bind actual auditor, shared schema and independent semantic adversaries."""
    return{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
def native(operand):
    """📦 Supply the actual native finite/profile carrier format, never a flattened Boolean approximation."""
    if operand.get('operator')=='difference':return{'csg':deepcopy(operand)}
    if operand['kind']=='prism':return{key:deepcopy(operand[key])for key in('outline','holes','z0','z1')}
    if operand['kind']=='polyhedron':return{'mesh':{'vertices':deepcopy(operand['vertices']),'faces':deepcopy(operand['faces'])}}
    p=operand['profile'];cx,cy=operand['axis'];shape=p['shape'];profile={'shape':shape,'rotation':operand['rotation'],'voids':[{'offset':h['offset'],'diameter':{'value':2*h['radius']}}for h in p['voids']]}
    if shape=='circle':
        radius=p['radius'];profile['diameter']={'value':2*radius};outline=[[cx+radius,cy],[cx,cy+radius],[cx-radius,cy],[cx,cy-radius]]
    elif shape=='ellipse':
        rx,ry=p['radii'];profile.update(width={'value':2*rx},depth={'value':2*ry});outline=[[cx-rx,cy-ry],[cx+rx,cy-ry],[cx+rx,cy+ry],[cx-rx,cy+ry]]
    elif shape=='rectangle':
        w,d=p['width'],p['depth'];profile.update(width={'value':w},depth={'value':d});outline=[[cx-w,cy-d],[cx+w,cy-d],[cx+w,cy+d],[cx-w,cy+d]]
    elif shape=='polygon':
        profile['outline']=p['outline'];outline=[[cx+x,cy+y]for x,y in p['outline']]
    else:raise ValueError('Unknown original source profile')
    return{'axis':[cx,cy],'profile':profile,'outline':outline,'z0':operand['z0'],'z1':operand['z1']}
before=hashes();spec=importlib.util.spec_from_file_location('whole_semantic_audit_r95',paths[1]);A=importlib.util.module_from_spec(spec);spec.loader.exec_module(A);shared=json.loads(paths[4].read_text(encoding='utf8'));pairs=json.loads(paths[2].read_text(encoding='utf8'));rows=[];start=time.monotonic()
for c in shared['cases']:
    row={'id':'owned/'+c['id'],'pass':False}
    try:
        solid=A.rawsolid(c['part']);assert solid.semantic==c['part']['csg'];stock=c['part']['csg']['stock'];result=A.intersection(solid,A.rawsolid(native(stock)));expected=c['expected']
        if expected.get('owned_volume')==0:assert result['volume']==0 and result['error']==0 and result['method']=='exact_semantic_exclusion'
        elif 'owned_volume'in expected:assert math.isclose(result['volume'],expected['owned_volume'],rel_tol=2e-12,abs_tol=1e-13),(result,expected)
        if expected.get('owned_volume_positive'):assert result['volume']>0
        for cutter in c['part']['csg']['cutters']:
            excluded=A.intersection(solid,A.rawsolid(native(cutter['operand'])));assert excluded['volume']==0 and excluded['error']==0 and excluded['method']=='exact_semantic_exclusion',excluded
        row['direct_cutters_excluded']=len(c['part']['csg']['cutters']);row['result']=result;row['pass']=True
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
for c in pairs['cases']:
    row={'id':'intersection/'+c['id'],'pass':False}
    try:
        owner=c['left']['cutters'][0];actual_guid=c.get('owner_guid_actual',owner.get('owner_guid','NEUTRAL_OWNER'));left=A.Solid({'geometry':native(c['left']),'tag':'DONOR','guid':'NEUTRAL_DONOR'});right=A.Solid({'geometry':native(c['right']),'tag':owner.get('owner','BOUNDARY'),'guid':actual_guid});result=A.intersection(left,right);row['actual_owner_guid']=actual_guid
        if c.get('expected_zero'):assert result['volume']==0 and result['error']==0 and result['method']=='exact_semantic_exclusion',result
        if 'expected_volume'in c:assert math.isclose(result['volume'],c['expected_volume'],rel_tol=2e-12,abs_tol=c.get('absolute_tolerance',1e-13)),result
        if 'positive_range'in c:assert c['positive_range'][0]<result['volume']<c['positive_range'][1]and result['error']<result['volume']/8,result
        row['result']=result;row['pass']=True
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
row={'id':'actual_capture_retains_semantic_tree_without_display_authority','pass':False};folder=T/'🗑️generated/semantic-capture-controls-r95';folder.mkdir(exist_ok=True);source=folder/'neutral-source.json';part=deepcopy(shared['cases'][0]['part']);part['display']={'purpose':'display_only','parts':[{'outline':[[0,0],[99,0],[99,99]],'z0':0,'z1':99}]};obj={'tag':'NEUTRAL-CSG','guid':'0NEUTRAL00000000000000','class':'IfcBeam','storey':'neutral','parts':[part]};curtain={'tag':'NEUTRAL-CURTAIN','guid':'0NEUTRAL00000000000001','class':'IfcCurtainWall','storey':'neutral','parts':[]};model={'elements':[{k:o[k]for k in('tag','guid','class','storey')}for o in(obj,curtain)]};source.write_text(json.dumps(model),encoding='utf8');api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');saved=(A.G,A.SOURCE,A.producer_hashes,A.loaded_hashes,api.building_geometry)
try:
    A.G=folder;A.SOURCE=source;A.producer_hashes=lambda:{source.relative_to(R).as_posix():hashlib.sha256(source.read_bytes()).hexdigest()};A.loaded_hashes=lambda:{};api.building_geometry=lambda model:{'beams':[obj],'ramp':{'ramps':[]},'curtain_wall':curtain};capture=A.capture();assert len(capture['records'])==1;record=capture['records'][0];assert record['geometry']=={'csg':part['csg']},record['geometry'];assert record['material']==part['material']and record['role']==part['role'];row['pass']=True
except Exception:row['error']=traceback.format_exc()
finally:A.G,A.SOURCE,A.producer_hashes,A.loaded_hashes,api.building_geometry=saved
rows.append(row);after=hashes();out={'pass':all(r['pass']for r in rows)and before==after,'stable':before==after,'cases':len(rows),'rows':rows,'before':before,'after':after,'seconds':time.monotonic()-start,'scope':'Neutral exact semantic geometry and isolated synthetic capture only; no current whole-house acceptance.'};(T/'🗑️generated/whole-semantic-csg-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('rows','before','after')}),flush=True);print('[DEBUG] failing_cases='+str([r['id']for r in rows if not r['pass']]),flush=True);raise SystemExit(not out['pass'])
