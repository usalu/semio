"""🧮️ SQLite corroborates prepaid numeric capacity independently of the future geometric producer."""
from pathlib import Path
import json,sqlite3
T=Path(__file__).parent
class CapacityOracle:
 def __init__(self,limit,mesh):
  self.db=sqlite3.connect(':memory:');self.limit=limit;self.mesh=mesh;self.used=0;self.peak=0;self.owners={}
 def scalar(self,query,*args):
  return self.db.execute(query,args).fetchone()[0]
 def step(self,step):
  op=step['operation'];owner=step['owner']
  if op=='builder':
   needed=self.scalar('SELECT 3*?+3*?',len(self.mesh['vertices']),len(self.mesh['triangles']))
   if not self.scalar('SELECT ?<=?-?',needed,self.limit,self.used):return 'limit'
   if owner in self.owners:raise AssertionError('unique original owner')
   self.owners[owner]={'values':needed,'points':0,'triangles':0}
   self.used=self.scalar('SELECT ?+?',self.used,needed);self.peak=self.scalar('SELECT max(?,?)',self.peak,self.used)
  elif op in ('points','triangles'):
   record=self.owners[owner];record[op]=step['rows'];assert record[op]<=len(self.mesh['vertices'if op=='points'else'triangles'])
   assert self.scalar('SELECT 3*?+3*?<=?',record['points'],record['triangles'],record['values'])
  elif op=='seal':
   record=self.owners[owner]
   if record['points']!=len(self.mesh['vertices'])or record['triangles']!=len(self.mesh['triangles']):return 'incomplete'
  elif op=='release':
   self.used=self.scalar('SELECT ?-?',self.used,self.owners.pop(owner)['values'])
  else:raise AssertionError('closed neutral command')
  return None
 def close(self):self.db.close()
law=json.loads((T/'boolean-owned-mesh-programs.json').read_text(encoding='utf-8-sig'));results=[]
for program in law['programs']:
 oracle=CapacityOracle(program['limit'],law['source'])
 try:
  for step in program['steps']:
   refusal=oracle.step(step)
   assert refusal==step.get('refusal'),(program['id'],step,refusal)
   assert(oracle.used,oracle.peak)==(step['expectedUsed'],step['expectedPeak']),(program['id'],step,oracle.used,oracle.peak)
  assert oracle.used==0
  results.append({'id':program['id'],'steps':len(program['steps']),'peak':oracle.peak,'exact':True})
 finally:oracle.close()
(T/'🗑️generated/boolean-owned-mesh-oracle.json').write_text(json.dumps({'engine':'SQLite '+sqlite3.sqlite_version,'scope':'prepaid capacity conservation only','geometryCertificate':False,'programs':results},indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'programs':len(results),'steps':sum(row['steps']for row in results),'exact':True,'geometryCertificate':False}))
