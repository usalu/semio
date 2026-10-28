"""🔷 Author explicit polygonal fixture inputs without production helpers."""
import copy,json
from pathlib import Path
p=Path(__file__).parent;data=json.loads((p/'polygonal-identity-cases.json').read_text(encoding='utf-8'));cases=data['cases']
for r in cases:
 r['expected']='supported';r['bounds']=[[0,0,0],[2,1,3]];r['volume']=6 if r['closed'] else None;r['damage']='none'
base=cases[0]
def add(id,change,expected='unsupported'):
 r=copy.deepcopy(base);r['id']=id;r['expected']=expected;change(r);cases.append(r)
add('closed-missing-face',lambda r:r['faces'].pop())
add('zero-index',lambda r:r['faces'][0]['outer'].__setitem__(0,0))
add('negative-index',lambda r:r['faces'][0]['outer'].__setitem__(0,-1))
add('fractional-index',lambda r:r['faces'][0]['outer'].__setitem__(0,1.5))
add('out-of-range-index',lambda r:r['faces'][0]['outer'].__setitem__(0,9))
add('pnindex-out-of-range',lambda r:r.update(pnIndex=[1,2,3,4,5,6,7,9]))
add('nonplanar-face',lambda r:r['coordinates'][0].__setitem__(2,.25))
add('self-crossing-face',lambda r:r['faces'][0].update(outer=[1,3,2,4]))
add('duplicate-face-reference',lambda r:r.update(damage='duplicate-face-reference'))
add('wrong-face-role',lambda r:r.update(damage='wrong-face-role'))
add('wrong-coordinate-role',lambda r:r.update(damage='wrong-coordinate-role'))
outer=[[0,0],[4,0],[4,4],[0,4]];hole=[[1,1],[1,3],[3,3],[3,1]];coords=[[x,y,z]for z in[0,2]for x,y in outer+hole];faces=[dict(outer=[4,3,2,1],holes=[[8,7,6,5]]),dict(outer=[9,10,11,12],holes=[[13,14,15,16]])]
for ring in [[1,2,3,4],[5,6,7,8]]:
 for a,b in zip(ring,ring[1:]+ring[:1]):faces.append(dict(outer=[a,b,b+8,a+8],holes=[]))
cases.append(dict(id='through-hole',coordinates=coords,faces=faces,pnIndex=None,closed=True,expected='supported',bounds=[[0,0,0],[4,4,2]],volume=24,damage='none'))
(p/'polygonal-neutral-cases.json').write_text(json.dumps(dict(schema='IFC4X3_ADD2',cases=cases),indent=2)+'\n',encoding='utf-8')
vector=dict(type='array',minItems=3,maxItems=3,items=dict(type='number'));index=dict(type='number');loop=dict(type='array',items=index)
face=dict(type='object',additionalProperties=False,required=['outer','holes'],properties=dict(outer=loop,holes=dict(type='array',items=loop)))
row=dict(type='object',additionalProperties=False,required=list(cases[0]),properties=dict(id=dict(type='string',minLength=1),coordinates=dict(type='array',items=vector),faces=dict(type='array',items=face),pnIndex=dict(anyOf=[dict(type='null'),loop]),closed=dict(type=['boolean','null']),expected=dict(enum=['supported','unsupported']),bounds=dict(type='array',minItems=2,maxItems=2,items=vector),volume=dict(type=['number','null']),damage=dict(enum=['none','duplicate-face-reference','wrong-face-role','wrong-coordinate-role'])))
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['schema','cases'],'properties':{'schema':{'const':'IFC4X3_ADD2'},'cases':{'type':'array','items':row}}}
(p/'polygonal-neutral.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(cases))+' explicit polygonal rows authored')
