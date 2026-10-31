"""🔬 Capture exact neutral mitre edge incidence before a sustainable clipping repair."""
import importlib,json,sys
from pathlib import Path
from collections import Counter
sys.path.insert(0,str(Path.cwd()));test=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧪️tests.🐍️');f=test.api._surface_finishes();out=Path(__file__).parent/'🗑️generated/native-r92';rows=[]
for end in ([2,2,0],[2,-2,0]):
 joint=f.skirting_joint([0,0,0],[2,0,0],end,.015);p=joint['plane'];parts=[]
 for a,b,n,plane in [([0,0,0],[2+joint['extension'],0,0],[0,-1,0],p),([2,-(1 if end[1]>0 else -1)*joint['extension'],0],end,[1 if end[1]>0 else -1,0,0],{'normal':[-v for v in p['normal']],'offset':-p['offset']})]:
  stock=f.skirting_stock(a,b,n,test.CASE['skirting']['section'],'MAT-PAINTED-WOOD');part=f.clip_stock(stock,[plane]);inc=Counter(tuple(sorted((a,b))) for face in part['mesh']['faces'] for a,b in zip(face,face[1:]+face[:1]));parts.append({'part':part,'edges':[(k,v) for k,v in inc.items() if v!=2]})
 rows.append({'end':end,'joint':joint,'parts':parts})
(out/'skirting-mitre-neutral.json').write_text(json.dumps(rows,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps([{ 'end':r['end'],'edges':[p['edges'] for p in r['parts']]} for r in rows]),flush=True)
