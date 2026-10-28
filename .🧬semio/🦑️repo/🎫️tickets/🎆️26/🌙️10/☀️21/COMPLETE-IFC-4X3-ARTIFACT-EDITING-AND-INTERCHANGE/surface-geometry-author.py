"""🫧 Author explicit face-based surface groups and bound orientation inputs."""
import copy,json
from pathlib import Path
p=Path(__file__).parent
points=[[0,0,0],[2,0,0],[2,1,0],[0,1,0],[0,0,3],[2,0,3],[2,1,3],[0,1,3]];box=[[3,2,1,0],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7],[4,5,6,7]];cases=[]
def faces(loops,orientation=True,outer=True):return[dict(bounds=[dict(loop=l,orientation=orientation,outer=outer)])for l in loops]
def add(id,coords,fs,groups,area,expected='supported',damage='none'):
 cases.append(dict(id=id,points=coords,faces=fs,groups=groups,expected=expected,area=area,damage=damage))
g=lambda ids,kind='IFCCONNECTEDFACESET':[dict(kind=kind,faces=ids)]
add('single-face',points[:4],faces([[0,1,2,3]]),g([0]),2)
add('open-shell-face',points[:4],faces([[0,1,2,3]]),g([0],'IFCOPENSHELL'),2)
add('closed-box-concrete-set',points,faces(box),g(list(range(6))),22)
add('closed-shell-box',points,faces(box),g(list(range(6)),'IFCCLOSEDSHELL'),22)
add('open-box',points,faces(box[:-1]),g(list(range(5)),'IFCOPENSHELL'),20)
coords=points[:4]+[[x+3,y,z]for x,y,z in points[:4]];fs=faces([[0,1,2,3],[4,5,6,7]])
add('disconnected-separate-groups',coords,fs,[dict(kind='IFCCONNECTEDFACESET',faces=[0]),dict(kind='IFCCONNECTEDFACESET',faces=[1])],4)
add('disconnected-one-group',coords,fs,g([0,1]),None,'unsupported')
add('adjacent-open-faces',[[0,0,0],[1,0,0],[2,0,0],[0,1,0],[1,1,0],[2,1,0]],faces([[0,1,4,3],[1,2,5,4]]),g([0,1]),2)
coords=[[0,0,0],[4,0,0],[4,4,0],[0,4,0],[1,1,0],[1,3,0],[3,3,0],[3,1,0]];fs=[dict(bounds=[dict(loop=[0,1,2,3],orientation=True,outer=True),dict(loop=[4,5,6,7],orientation=True,outer=False)])]
add('face-hole',coords,fs,g([0]),12)
rev=copy.deepcopy(fs)
for b in rev[0]['bounds']:b['loop'].reverse();b['orientation']=False
add('false-orientation-equivalent',coords,rev,g([0]),12)
add('unmarked-sole-bound',points[:4],faces([[0,1,2,3]],outer=False),g([0]),2)
add('closed-shell-open',points,faces(box[:-1]),g(list(range(5)),'IFCCLOSEDSHELL'),None,'unsupported')
add('missing-orientation',points[:4],faces([[0,1,2,3]],orientation=None),g([0]),None,'unsupported')
add('nonplanar',[[0,0,.25],[2,0,0],[2,1,0],[0,1,0]],faces([[0,1,2,3]]),g([0]),None,'unsupported')
add('crossing',points[:4],faces([[0,2,1,3]]),g([0]),None,'unsupported')
add('duplicate-face-reference',points[:4],faces([[0,1,2,3]]),g([0,0]),None,'unsupported')
add('duplicate-group-reference',points[:4],faces([[0,1,2,3]]),g([0]),None,'unsupported','duplicate-group-reference')
for damage in ['wrong-bound','wrong-loop','wrong-point','empty-groups']:
 add(damage,points[:4],faces([[0,1,2,3]]),g([0]),None,'unsupported',damage)
row=dict(type='object',additionalProperties=False,required=list(cases[0]),properties=dict(id=dict(type='string'),points=dict(type='array',items=dict(type='array',minItems=3,maxItems=3,items=dict(type='number'))),faces=dict(type='array',items=dict(type='object',additionalProperties=False,required=['bounds'],properties=dict(bounds=dict(type='array',items=dict(type='object',additionalProperties=False,required=['loop','orientation','outer'],properties=dict(loop=dict(type='array',items=dict(type='integer',minimum=0)),orientation=dict(type=['boolean','null']),outer=dict(type='boolean'))))))),groups=dict(type='array',items=dict(type='object',additionalProperties=False,required=['kind','faces'],properties=dict(kind=dict(enum=['IFCCONNECTEDFACESET','IFCOPENSHELL','IFCCLOSEDSHELL']),faces=dict(type='array',items=dict(type='integer',minimum=0))))),expected=dict(enum=['supported','unsupported']),area=dict(type=['number','null']),damage=dict(enum=['none','duplicate-group-reference','wrong-bound','wrong-loop','wrong-point','empty-groups'])))
(p/'surface-geometry-cases.json').write_text(json.dumps(dict(schema='IFC4X3_ADD2',cases=cases),indent=2)+'\n',encoding='utf-8');(p/'surface-geometry.schema.json').write_text(json.dumps({'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['schema','cases'],'properties':{'schema':{'const':'IFC4X3_ADD2'},'cases':{'type':'array','items':row}}},indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(cases))+' explicit face-based surface rows')
