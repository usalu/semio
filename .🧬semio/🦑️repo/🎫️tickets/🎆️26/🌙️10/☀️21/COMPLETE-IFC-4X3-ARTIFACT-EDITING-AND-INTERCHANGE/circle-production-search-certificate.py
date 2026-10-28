"""⭕ Verify actual chosen production counts against independent ideal circle sagitta."""
import json,sys
from pathlib import Path
t=Path(__file__).resolve().parent;sys.path.insert(0,str(t/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=100;source=json.loads((t/'🗑️generated/geometry-query/circle-production-search.json').read_text());rows=[]
for row in source['rows']:
 n=row['actual'];rows.append({'nominal':row['nominal'],'side':row['side'],'actual':n,'refused':n is None,'idealChordPass':None if n is None else 2*mp.sin(mp.pi/(2*n))**2<=mp.mpf(row['absolute'])})
(t/'🗑️generated/geometry-query/circle-production-search-certificate.json').write_text(json.dumps({'library':mp.__version__,'precision':100,'rows':rows},indent=2));print('[DEBUG] '+json.dumps({'cases':len(rows),'accepted':sum(not r['refused']for r in rows),'falseAdmission':sum(r['idealChordPass']is False for r in rows)}))
