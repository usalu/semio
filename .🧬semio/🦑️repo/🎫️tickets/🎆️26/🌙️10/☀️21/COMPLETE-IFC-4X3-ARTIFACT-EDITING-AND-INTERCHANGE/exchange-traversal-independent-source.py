import json,hashlib
from pathlib import Path
p=Path(__file__).parent
m=json.loads((p/'🗑️generated/exchange-traversal-source.json').read_text(encoding='utf-8-sig'));rows=[dict(kind=r['kind'],path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest().lower()==r['sha256'].lower())for r in m];assert all(r['matches']for r in rows if r['kind']=='owned')
before=json.loads((p/'🗑️generated/bim-cold-scan-independent-before-v2.json').read_text());after=json.loads((p/'🗑️generated/bim-cold-scan-independent-after.json').read_text());f0=json.loads((p/'🗑️generated/fem-cold-scan-independent-before.json').read_text());f1=json.loads((p/'🗑️generated/fem-cold-scan-independent-after.json').read_text());result=dict(rows=rows,bim=dict(beforeScans=len(before['scans']),afterScans=len(after['scans']),projectionExact=before['before']==after['before'],editedExact=before['after']==after['after'],inverseExact=after['inverseExact']),fem=dict(beforeScans=len(f0['scans']),afterScans=len(f1['scans']),resultExact=f0['result']==f1['result']))
assert result['bim']['projectionExact']and result['bim']['editedExact']and result['fem']['resultExact']
(p/'🗑️generated/exchange-traversal-independent-source.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps({k:v for k,v in result.items()if k!='rows'}))
