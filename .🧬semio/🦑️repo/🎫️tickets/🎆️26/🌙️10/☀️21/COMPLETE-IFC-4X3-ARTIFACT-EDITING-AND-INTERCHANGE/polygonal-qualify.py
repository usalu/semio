"""🔷 Qualify polygonal physical observations and seal explicit fixture shapes."""
import json,hashlib
from pathlib import Path
p=Path(__file__).parent;data=json.loads((p/'polygonal-neutral-cases.json').read_text(encoding='utf-8'));schema=json.loads((p/'polygonal-neutral.schema.json').read_text(encoding='utf-8'))
for row in data['cases']:
 row['qualification']='normal-backend-only'if row['id']in['explicit-end-repeat','consecutive-repeat']else'geometric-contract'
 if row['qualification']=='normal-backend-only':row['expected']='qualified'
r=schema['properties']['cases']['items'];r['required'].append('qualification');r['properties']['qualification']={'enum':['normal-backend-only','geometric-contract']};r['properties']['expected']['enum'].append('qualified')
(p/'polygonal-neutral-cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8');(p/'polygonal-neutral.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
raw={'schema':data['schema'],'cases':data['cases'][:8]};(p/'polygonal-identity-cases.json').write_text(json.dumps(raw,indent=2)+'\n',encoding='utf-8');(p/'polygonal-identity.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
for n in ['polygonal-identity-cases.json','polygonal-identity.schema.json','polygonal-neutral-cases.json','polygonal-neutral.schema.json']:print('[DEBUG] '+n+' '+hashlib.sha256((p/n).read_bytes()).hexdigest())
