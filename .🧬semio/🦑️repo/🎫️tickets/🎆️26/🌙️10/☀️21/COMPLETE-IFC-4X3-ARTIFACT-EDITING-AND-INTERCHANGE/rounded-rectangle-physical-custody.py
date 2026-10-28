"""▢ Add raw header custody and separate repaired-profile math without changing STEP bytes."""
from pathlib import Path
import json,re,hashlib,sys
p=Path(__file__).parent;path=p/'🗑️generated/rounded-rectangle-independent-physical.json';data=json.loads(path.read_text(encoding='utf8'));manifest=json.loads((p/'🗑️generated/rounded-rectangle-actual/emitted.json').read_text(encoding='utf8'));by={r['id']:r for r in manifest['records']}
for r in data['records']:
 raw=Path(by[r['id']]['path']).read_bytes();text=raw.decode('utf8');r['rawSchemaHeader']=re.search(r'FILE_SCHEMA\s*\(.*?;',text,re.S).group(0);r['finalHashMatches']=hashlib.sha256(raw).hexdigest()==r['sha256']
sys.path.insert(0,str(p/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=100;repair=next(r for r in data['records'] if r['id']=='integer-radius-canonical-real-repair');x=mp.mpf(repair['profile']['XDim']);y=mp.mpf(repair['profile']['YDim']);radius=mp.mpf(repair['profile']['RoundingRadius']);depth=mp.mpf(repair['solid']['depth']);N=72
repair['separateHighPrecisionMath']={'digits':100,'expectedChords':72,'expectedBoundary':76,'logicalValues':1356,'idealVolume':mp.nstr((x*y-(4-mp.pi)*radius**2)*depth,90),'polygonVolume':mp.nstr((x*y-4*radius**2+N*radius**2*mp.sin(2*mp.pi/N)/2)*depth,90),'origin':'raw actual REAL dimensions, independent formulas; manifest null unchanged'}
path.write_text(json.dumps(data,indent=2,default=str)+'\n',encoding='utf8');print('[DEBUG]',len(data['records']),sum(r['finalHashMatches'] for r in data['records']),sorted(set(r['rawSchemaHeader'] for r in data['records'])))
