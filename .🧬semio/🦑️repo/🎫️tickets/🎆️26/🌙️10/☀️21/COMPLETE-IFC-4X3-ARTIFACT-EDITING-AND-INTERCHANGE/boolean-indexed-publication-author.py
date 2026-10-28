"""📦 Author source-neutral indexed publication conservation laws, no producer imports."""
from pathlib import Path
import json,hashlib,sqlite3
p=Path(__file__).parent;repo=next(a for a in p.parents if (a/'AGENTS.md').exists());source=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/owned-workspace-cases.json';raw=source.read_bytes();cube=json.loads(raw)['source'];v=len(cube['vertices']);t=len(cube['triangles']);original=3*(v+t);flat=3*t;cases=[]
def program(id,limit,prefix,outcome):
 peak=original+flat if limit>=original+flat else original
 return {'id':id,'vertexCount':v,'triangleCount':t,'originalSlots':original,'flatSlots':flat,'limit':limit,'copiedPrefix':prefix,'outcome':outcome,'expectedPeak':peak,'expectedUsed':0 if outcome=='published' else original,'originalHandleRetained':outcome!='published','actualPartialLength':prefix if limit>=original+flat else 0}
cases.append(program('cube-publish-96',96,36,'published'));cases.append(program('cube-denied-95',95,0,'limit-refusal'))
for prefix in [0,1,2,3,17,35,36]:
 for outcome in ['return','cancel','throw-undefined','throw-error']:cases.append(program('cube-'+outcome+'-prefix'+str(prefix),96,prefix,outcome))
cases.append({'id':'empty-publish','vertexCount':0,'triangleCount':0,'originalSlots':0,'flatSlots':0,'limit':0,'copiedPrefix':0,'outcome':'published','expectedPeak':0,'expectedUsed':0,'originalHandleRetained':False,'actualPartialLength':0})
data={'scope':'owned-indexed-publication-conservation','producerExecuted':False,'metric':'logical-numeric-slots','sourceSha256':hashlib.sha256(raw).hexdigest(),'originalSource':str(source.relative_to(repo)).replace('\\','/'),'cases':cases}
(p/'boolean-indexed-publication-cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf8')
fields={'id':{'type':'string'},**{k:{'type':'integer','minimum':0}for k in ['vertexCount','triangleCount','originalSlots','flatSlots','limit','copiedPrefix','expectedPeak','expectedUsed','actualPartialLength']},'outcome':{'enum':['published','limit-refusal','return','cancel','throw-undefined','throw-error']},'originalHandleRetained':{'type':'boolean'}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(data),'properties':{'scope':{'const':data['scope']},'producerExecuted':{'const':False},'metric':{'const':data['metric']},'sourceSha256':{'type':'string','pattern':'^[a-f0-9]{64}$'},'originalSource':{'type':'string'},'cases':{'type':'array','minItems':31,'maxItems':31,'items':{'type':'object','additionalProperties':False,'required':list(fields),'properties':fields}}}}
(p/'boolean-indexed-publication.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8');observations=[]
for row in cases:
 with sqlite3.connect(':memory:') as db:
  db.execute('CREATE TABLE leases(owner TEXT PRIMARY KEY,slots INTEGER NOT NULL CHECK(slots>=0))');db.execute('INSERT INTO leases VALUES(?,?)',('original',row['originalSlots']));peak=row['originalSlots'];admitted=row['originalSlots']+row['flatSlots']<=row['limit']
  if admitted:db.execute('INSERT INTO leases VALUES(?,?)',('flat',row['flatSlots']));peak=row['originalSlots']+row['flatSlots']
  if admitted and row['outcome']=='published':db.execute('DELETE FROM leases')
  else:db.execute("DELETE FROM leases WHERE owner='flat'")
  used=db.execute('SELECT COALESCE(SUM(slots),0) FROM leases').fetchone()[0];retained=db.execute("SELECT COUNT(*) FROM leases WHERE owner='original'").fetchone()[0]>0;observations.append({'id':row['id'],'admitted':admitted,'used':used,'peak':peak,'originalHandleRetained':retained,'exact':used==row['expectedUsed'] and peak==row['expectedPeak'] and retained==row['originalHandleRetained']})
(p/'🗑️generated/boolean-indexed-publication-sqlite.json').write_text(json.dumps({'sqlite':sqlite3.sqlite_version,'producerExecuted':False,'cases':observations},indent=2)+'\n',encoding='utf8');assert all(x['exact']for x in observations);assert hashlib.sha256(source.read_bytes()).hexdigest()==data['sourceSha256'];print('[DEBUG]',len(cases),original,flat,all(x['exact']for x in observations))
