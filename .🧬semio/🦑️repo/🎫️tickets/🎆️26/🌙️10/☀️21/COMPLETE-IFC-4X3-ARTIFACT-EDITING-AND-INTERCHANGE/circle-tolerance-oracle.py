"""⭕ Independent high precision circle tessellation bounds without new dependency installs."""
import json,sys,ctypes
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
sys.path.insert(0,str(t/'🗑️generated/determinant'))
try:
 import mpmath as mp
except ImportError:
 out=t/'🗑️generated/geometry-query/circle-oracle.json';out.write_text(json.dumps({'available':False,'reason':'Existing Python interface has no mpmath; no install performed'}));print('[DEBUG] mpmath unavailable; no numeric oracle claim');sys.exit(0)
mp.mp.dps=100;rows=[]
for name,radius,depth,absolute,relative,maximum in [('official-opening','0.05','0.2','0','0.001',4096),('metre','1','3','0','0.001',4096),('absolute-small','0.05','0.2','0.000001','0',4096),('tight-refusal','1','1','1e-30','0',4096),('huge','1e300','1e-300','0','0.001',4096),('subnormal','4.9406564584124654e-324','1','0','0.001',4096)]:
 r=mp.mpf(radius);d=mp.mpf(depth);tol=max(mp.mpf(absolute),r*mp.mpf(relative));count=None
 for n in range(4,maximum+1,4):
  if 2*r*mp.sin(mp.pi/(2*n))**2<=tol:count=n;break
 row={'id':name,'radius':radius,'depth':depth,'absolute':absolute,'relative':relative,'maxSegments':maximum,'segments':count,'scope':'ideal analytic sampling only; floating representability and finite output checked separately'}
 if count:
  sagitta=2*r*mp.sin(mp.pi/(2*count))**2;ratio=1-count*mp.sin(2*mp.pi/count)/(2*mp.pi);area=mp.pi*r*r;row.update(sagitta=mp.nstr(sagitta,80),areaRelativeDeficit=mp.nstr(ratio,80),areaDeficitUpper=mp.nstr(2*mp.pi**2/(3*count**2),80),analyticVolume=mp.nstr(area*d,80),volumeDeficit=mp.nstr(area*d*ratio,80))
 rows.append(row)
out=t/'🗑️generated/geometry-query/circle-oracle.json';out.write_text(json.dumps({'library':'mpmath','version':mp.__version__,'dps':mp.mp.dps,'available':True,'rows':rows},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'id':r['id'],'segments':r['segments']}for r in rows]))

