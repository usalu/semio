"""🏗️ Independent SQL residency transitions for owned mesh construction."""
from pathlib import Path
import json,sqlite3
p=Path(__file__).parent;repo=next(a for a in p.parents if (a/'AGENTS.md').exists());file=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/owned-workspace-cases.json';data=json.loads(file.read_text(encoding='utf8'));out=[]
for program in data['programs']:
 with sqlite3.connect(':memory:') as db:
  db.execute('CREATE TABLE residency(owner TEXT PRIMARY KEY, slots INTEGER)');peak=0
  for step in program['steps']:
   used=db.execute('SELECT COALESCE(SUM(slots),0) FROM residency').fetchone()[0]
   if step['operation']=='builder' and used+60<=program['limit']:db.execute('INSERT INTO residency VALUES(?,60)',(step['owner'],))
   if step['operation']=='release':db.execute('DELETE FROM residency WHERE owner=?',(step['owner'],))
   used=db.execute('SELECT COALESCE(SUM(slots),0) FROM residency').fetchone()[0];peak=max(peak,used);out.append({'program':program['id'],'operation':step['operation'],'used':used,'peak':peak,'exact':used==step['expectedUsed'] and peak==step['expectedPeak']})
(p/'🗑️generated/boolean-workspace-producer-independent-sqlite.json').write_text(json.dumps({'sqlite':sqlite3.sqlite_version,'transitions':out},indent=2)+'\n',encoding='utf8');print('[DEBUG]',len(out),all(x['exact']for x in out))
