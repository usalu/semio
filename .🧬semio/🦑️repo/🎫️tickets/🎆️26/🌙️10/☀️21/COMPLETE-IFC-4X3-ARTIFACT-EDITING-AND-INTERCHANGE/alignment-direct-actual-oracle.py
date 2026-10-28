"""🚆 Compare actual public endpoint outputs with independent physical attribute identities."""
import ctypes,sys,json,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
out=t/'🗑️generated/alignment-direct-actual';owned=json.loads((out/'owned.json').read_text());root=t
while not(root/'✏️s').exists():root=root.parent
base=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official'
def observe(f,request):
 target=request['targets'][0];s=f.by_id(int(target['segment']));p=s.DesignParameters;length=getattr(p,'SegmentLength',getattr(p,'HorizontalLength',None));paths=[]
 if s.Representation:
  for rep in s.Representation.Representations:
   for c in rep.Items:
    v=getattr(c,'SegmentLength',None);paths.append({'definition':s.Representation.id(),'representation':rep.id(),'item':c.id(),'type':c.is_a(),'branch':v.is_a() if hasattr(v,'is_a')else None,'value':v.wrappedValue if hasattr(v,'wrappedValue')else repr(v)})
 return {'parameterId':p.id(),'parameterType':p.is_a(),'businessLength':length,'paths':paths}
rows=[]
for r in owned['rows']:
 row={'id':r['id'],'endpoint':r['actual'],'written':r['written']}
 if not r['written']:row['externalComparison']='not representable by actual STEP writer';rows.append(row);continue
 path=out/(r['id']+'.ifc');data=path.read_bytes();assert hashlib.sha256(data).hexdigest()==r['sha256'];f=ifcopenshell.open(str(path));row['header']=list(f.header.file_schema.schema_identifiers)
 try:row['observation']=observe(f,r['request'])
 except Exception as e:row['observationError']=str(e)
 log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);row['expressDiagnostics']=[{'attribute':v.get('attribute'),'message':str(v.get('message'))}for v in log.statements];rows.append(row)
originals=[]
for r in owned['originals']:
 path=base/r['file'];assert hashlib.sha256(path.read_bytes()).hexdigest()==r['sha256'];f=ifcopenshell.open(str(path));obs=observe(f,{'targets':[{'segment':r['segment']}]});paths=obs['paths'];expected='pass' if len(paths)==1 and paths[0]['branch']in('IfcLengthMeasure','IfcParameterValue')and paths[0]['value']==0 and obs['businessLength']==0 else 'incomplete';originals.append({'file':r['file'],'segment':r['segment'],'sha256':r['sha256'],'header':list(f.header.file_schema.schema_identifiers),'observation':obs,'independentExpected':expected,'endpoint':r['status'],'match':expected==r['status'],'curveMatch':r.get('curve')==str(paths[0]['item'])if len(paths)==1 else r.get('curve')is None})
(out/'independent.json').write_text(json.dumps({'library':ifcopenshell.version,'python':sys.version,'rows':rows,'originals':originals,'nativeExecution':False},indent=2));print('[DEBUG] '+json.dumps({'rows':len(rows),'written':sum(r['written']for r in rows),'zeroExpress':[r['id']for r in rows if r['written']and not r.get('expressDiagnostics')],'originalMatches':sum(r['match']and r['curveMatch']for r in originals)}))