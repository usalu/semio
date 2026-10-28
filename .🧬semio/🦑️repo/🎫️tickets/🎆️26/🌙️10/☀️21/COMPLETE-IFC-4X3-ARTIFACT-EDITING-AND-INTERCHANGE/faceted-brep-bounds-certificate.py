import json,math
from pathlib import Path
p=Path(__file__).resolve().parent
v=json.loads((p/'faceted-brep-geometry-vectors.json').read_text(encoding='utf-8-sig'))
o=json.loads((p/'🗑️generated/faceted-brep-vectors/oracle.json').read_text(encoding='utf-8'));rows=[]
for r in v['cases']:
 if r['expected']=='supported':
  actual=next(x for x in o['rows']if x['id']==r['id'])
  if r['geometry']['voids']:
   rows.append(dict(id=r['id'],normalCompoundGeometry=False,separateCertificate='void-shell-certificate.json'));continue
  assert all(math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-10)for side,expected in zip(actual['geometry']['bounds'],r['bounds'])for a,b in zip(side,expected))
  rows.append(dict(id=r['id'],analyticBoundsMatched=True))
(p/'🗑️generated/faceted-brep-vectors/bounds-certificate.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8');print('[DEBUG] explicit SI bounds checked against normal geometry; void compound separately labeled')
