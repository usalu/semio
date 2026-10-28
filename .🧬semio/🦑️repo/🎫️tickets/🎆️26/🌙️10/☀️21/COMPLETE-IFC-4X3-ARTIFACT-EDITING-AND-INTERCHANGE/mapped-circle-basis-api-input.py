"""⭕️ Issue deterministic finite binary64 pairs to the actual basis API."""
import json,math,random
from pathlib import Path
p=Path(__file__).parent;random.seed(413);cases=[('identity',[1,0,0,0,1,0]),('near-collinear',[1,1,1,1,1,math.nextafter(1,2)]),('aspect',[1,0,0,0,2**-1022,0]),('subnormal',[5e-324,0,0,0,5e-324,0]),('huge',[1e308,0,0,0,1e308,0])]
for i in range(300):
 factor=math.ldexp(1.,random.randint(-1000,1000));cases.append(('seed-'+str(i),[random.uniform(-1,1)*factor for _ in range(6)]))
(p/'🗑️generated/mapped-circle-basis-api-input.json').write_text(json.dumps([dict(id=n,u=a[:3],v=a[3:])for n,a in cases])+'\n',encoding='utf-8')
