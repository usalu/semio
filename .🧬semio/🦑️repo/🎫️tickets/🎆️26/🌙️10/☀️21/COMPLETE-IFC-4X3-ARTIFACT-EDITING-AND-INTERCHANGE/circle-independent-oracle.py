"""⭕ Independent full EXPRESS, physical mesh and ideal high precision circle checks."""
import ctypes,json,sys,math,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep);sys.path.insert(0,str(t/'🗑️generated/determinant'))
import ifcopenshell,ifcopenshell.validate,ifcopenshell.geom,mpmath as mp
mp.mp.dps=100;out=t/'🗑️generated/circle-extrusion';owned=json.loads((out/'owned.json').read_text());repo=t
while not(repo/'✏️s').exists():repo=repo.parent
official=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/slab-openings.ifc'
def measure(p,idx):
 volume=0.
 for i in range(0,len(idx),3):
  a,b,c=(p[idx[i+j]]for j in range(3));volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6
 return {'bounds':[[min(v[k]for v in p)for k in range(3)],[max(v[k]for v in p)for k in range(3)]],'signedVolume':volume,'vertices':len(p),'triangles':len(idx)//3}
rows=[]
for row in owned['rows']:
 path=official if row['id']=='official-opening-325'else out/(row['id']+'.ifc');f=ifcopenshell.open(str(path));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);diagnostics=[{'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements];settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);shape=ifcopenshell.geom.create_shape(settings,f.by_id(row['product']));v=list(shape.geometry.verts);independent=measure([v[i:i+3]for i in range(0,len(v),3)],list(shape.geometry.faces));actual=measure(row['positions'],row['indices']);case=row.get('case',{'radius':50,'depth':200,'millimetres':True,'direction':[0,0,1]});scale=mp.mpf('.001')if case.get('millimetres')else mp.mpf(1);radius=mp.mpf(case['radius'])*scale;depth=mp.mpf(case['depth'])*scale;direction=[mp.mpf(x)for x in case['direction']];height=depth*abs(direction[2])/mp.sqrt(sum(x*x for x in direction));n=len(row['positions'])//2;policy=case.get('sampling',{});tol=max(mp.mpf(policy.get('absoluteChordErrorMetres',0)),radius*mp.mpf(policy.get('relativeChordError',.001)));sagitta=2*radius*mp.sin(mp.pi/(2*n))**2;analytic=mp.pi*radius**2*height;ideal=n*radius**2*mp.sin(2*mp.pi/n)*height/2;ratio=1-ideal/analytic;rows.append({'id':row['id'],'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'diagnostics':diagnostics,'actual':actual,'independent':independent,'segments':n,'sagitta':mp.nstr(sagitta,80),'requestedError':mp.nstr(tol,80),'chordPass':sagitta<=tol,'analyticVolume':mp.nstr(analytic,80),'idealPolygonVolume':mp.nstr(ideal,80),'actualIdealVolumeError':float(abs(mp.mpf(actual['signedVolume'])-ideal)),'areaRelativeDeficit':mp.nstr(ratio,80),'relativeDeficitUpper':mp.nstr(2*mp.pi**2/(3*n*n),80),'areaBoundPass':ratio<=2*mp.pi**2/(3*n*n)})
(out/'independent.json').write_text(json.dumps({'ifcopenshell':ifcopenshell.version,'mpmath':mp.__version__,'dps':mp.mp.dps,'rows':rows,'scope':'Circle polygon ideal sampling; external facet count not normative; native unexecuted'},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'id':r['id'],'diagnostics':len(r['diagnostics']),'segments':r['segments'],'chord':r['chordPass'],'areaBound':r['areaBoundPass'],'volumeError':r['actualIdealVolumeError']}for r in rows]))
