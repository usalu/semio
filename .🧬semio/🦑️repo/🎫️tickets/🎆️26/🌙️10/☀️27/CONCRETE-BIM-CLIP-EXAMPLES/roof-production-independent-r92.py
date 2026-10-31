"""🧮 Independently validate current production exact fallback on both captured roof cases."""
import hashlib,importlib,json,sys,time
from fractions import Fraction as F
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');an=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️');p=Path(__file__).parent;cases=json.loads((p/'roof-exact-review-cases-r92.json').read_text(encoding='utf8'))['cases'];original=api._prism_residuals;counts={False:0,True:0};records=[];start=time.monotonic();before={name:hashlib.sha256(Path(m.__file__).read_bytes()).hexdigest()for name,m in [('api',api),('analysis',an)]}
def orient(a,b,c):return(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
def on(a,b,c):return orient(a,b,c)==0 and all(min(a[k],b[k])<=c[k]<=max(a[k],b[k]) for k in [0,1])
def simple(raw):
 r=[tuple(F(x) for x in v) for v in raw]
 if len(r)<3 or len(set(r))!=len(r):return False
 area=sum(orient(r[0],a,b) for a,b in zip(r[1:-1],r[2:]))
 if area==0:return False
 for i,b in enumerate(r):
  a,c=r[i-1],r[(i+1)%len(r)]
  if orient(a,b,c)==0 and sum((b[k]-a[k])*(c[k]-b[k]) for k in [0,1])<0:return False
 for i,a in enumerate(r):
  b=r[(i+1)%len(r)]
  for j in range(i+1,len(r)):
   if j==i+1 or(i==0 and j==len(r)-1):continue
   c,d=r[j],r[(j+1)%len(r)];s,t,u,v=orient(a,b,c),orient(a,b,d),orient(c,d,a),orient(c,d,b)
   if s*t<0 and u*v<0 or on(a,b,c) or on(a,b,d) or on(c,d,a) or on(c,d,b):return False
 return True


def traced(outline,cutters,exact=False):
 counts[exact]+=1
 return original(outline,cutters,exact)
api._prism_residuals=traced
try:
 for c in cases:
  mark=time.monotonic();counts={False:0,True:0};out=api._subtract_prisms(c['parts'],c['cutters']);expected=Polygon(c['parts'][0]['outline']).difference(unary_union([Polygon(x['outline'])for x in c['cutters']]));bad=[i for i,r in enumerate(out)if not simple(r['outline'])or not Polygon(r['outline']).is_valid];roundbad=[i for i,r in enumerate(out)if not simple([[round(x,9),round(y,9)]for x,y in r['outline']])];area=sum(Polygon(r['outline']).area for r in out);records.append({'id':c['id'],'cutters':len(c['cutters']),'parts':len(out),'invalid':bad,'raw_9decimal_ring_diagnostic_invalid':roundbad,'fast_calls':counts[False],'exact_replays':counts[True],'area':area,'expected_geos':expected.area,'difference':area-expected.area,'seconds':time.monotonic()-mark});assert not bad
 controls=[]
 for w,h in [(2e-9,.0025),(2e-9,2e-9)]:
  ring=[[19.,0],[19.+w,0],[19.+w,h],[19.,h]];controls.append({'width':w,'height':h,'exact_simple':simple(ring),'production_simple':an._simple(ring),'note':'Simple-ring validity only; existingresidualarea filtersareseparate'});assert simple(ring)and an._simple(ring)
finally:api._prism_residuals=original
result={'scope':'CurrentproductiondefaultAPI, independentexactringpredicate+GEOS; raw9decimaldiagnosticnotanactualSDKserializationcertificate','cases':records,'controls':controls,'producers_before':before,'producers_after':{name:hashlib.sha256(Path(m.__file__).read_bytes()).hexdigest()for name,m in [('api',api),('analysis',an)]},'seconds':time.monotonic()-start};(p/'🗑️generated/roof-production-independent-r92.json').write_text(json.dumps(result,indent=2));print(json.dumps(result),flush=True)
