"""🧮 Review both retained roof cases and tiny positive controls without production edits."""
import hashlib,importlib,json,sys,time
from fractions import Fraction as F
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');an=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️');p=Path(__file__).parent;cases=json.loads((p/'🗑️generated/roof-residual-inputs-r92.json').read_text(encoding='utf8'));oldsub,oldloops=an._subtract,an._loops

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

def replay(parts,cutters):
 pp=[{**r,'outline':[[F(x),F(y)]for x,y in r['outline']]}for r in parts];cc=[{**r,'outline':[[F(x),F(y)]for x,y in r['outline']]}for r in cutters];result=api._subtract_prisms(pp,cc);return[{**r,'outline':[[float(x),float(y)]for x,y in r['outline']]}for r in result]
def exact(piece,void,digits=9,tolerance=an.EPS):return oldsub(piece,void,digits=None,tolerance=tolerance)
records=[];controls=[];producers={x:hashlib.sha256(Path(m.__file__).read_bytes()).hexdigest()for x,m in [('api',api),('analysis',an)]};start=time.monotonic();an._subtract=exact;an._loops=lambda pieces,tolerance=an.EPS:[]
try:
 for i,c in enumerate(cases):
  begin=time.monotonic();out=replay(c['input_parts'],c['cutters']);expected=Polygon(c['input_parts'][0]['outline']).difference(unary_union([Polygon(x['outline'])for x in c['cutters']]));invalid=[j for j,r in enumerate(out)if not simple(r['outline']) or not Polygon(r['outline']).is_valid];area=sum(Polygon(r['outline']).area for r in out);records.append({'case':i,'cutters':len(c['cutters']),'original_inputs_valid':all(simple(r['outline'])and Polygon(r['outline']).is_valid for r in c['input_parts']+c['cutters']),'captured_bad_ring_exact_simple':simple(c['invalid_output']['outline']),'captured_bad_ring_geos_valid':Polygon(c['invalid_output']['outline']).is_valid,'published_parts':len(out),'invalid_published':invalid,'area':area,'expected_geos_area':expected.area,'area_difference':area-expected.area,'seconds':time.monotonic()-begin});assert not invalid
 tiny={'outline':[[19.,0.],[19.+2e-9,0.],[19.+2e-9,.0025],[19.,.0025]],'z0':0.,'z1':2e-9};bar={'outline':[[18.9,.0008],[19.1,.0008],[19.1,.0017],[18.9,.0017]],'z0':0.,'z1':2e-9}
 for name,cs in [('two_nm_positive_stock',[]),('two_nm_stock_cut',[bar])]:
  out=replay([tiny],cs);expected=Polygon(tiny['outline']).difference(unary_union([Polygon(c['outline'])for c in cs]));area=sum(Polygon(r['outline']).area for r in out);controls.append({'id':name,'parts':len(out),'all_exact_simple':all(simple(r['outline'])for r in out),'area':area,'volume':area*2e-9,'expected_area':expected.area,'difference':area-expected.area});assert out and all(simple(r['outline'])for r in out)and area>0 and abs(area-expected.area)<1e-20
finally:an._subtract=oldsub;an._loops=oldloops
out={'scope':'Read-only2case exactreplay review; sourcealgorithmexisting1e-12areafilter remains and isnotgeneralall-positivearea guarantee','producers_before':producers,'producers_after':{x:hashlib.sha256(Path(m.__file__).read_bytes()).hexdigest()for x,m in [('api',api),('analysis',an)]},'roof_cases':records,'tiny_controls':controls,'seconds':time.monotonic()-start};(p/'🗑️generated/roof-exact-independent-r92.json').write_text(json.dumps(out,indent=2));print(json.dumps(out),flush=True)
