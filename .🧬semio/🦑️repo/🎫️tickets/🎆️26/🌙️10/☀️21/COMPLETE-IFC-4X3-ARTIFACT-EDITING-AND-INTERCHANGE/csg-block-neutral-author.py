"""🧊 Author exact CSG block dimensions and frame fixtures."""
import copy,json
from pathlib import Path
p=Path(__file__).parent;rows=[]
def add(id,dimensions=[2,1,3],position=None,unitPrefix=None,layers=None,expected='supported',bounds=None,volume=6,damage='none'):
 rows.append(dict(id=id,dimensions=dimensions,position=position or dict(origin=[0,0,0],axis=None,refDirection=None),unitPrefix=unitPrefix,mapping=layers or [],expected=expected,bounds=bounds,volume=volume,damage=damage))
add('identity',bounds=[[0,0,0],[2,1,3]])
add('offset',position=dict(origin=[-1,-.5,0],axis=None,refDirection=None),bounds=[[-1,-.5,0],[1,.5,3]])
add('rotated',position=dict(origin=[0,0,0],axis=[0,0,1],refDirection=[0,1,0]),bounds=[[-1,0,0],[0,2,3]])
add('millimetres',[2000,1000,3000],unitPrefix='MILLI',bounds=[[0,0,0],[2,1,3]])
add('mapped-reflected',layers=[dict(origin=[0,0,0],originRotation=0,target=[0,0,0],axes=[[-1,0,0],[0,1,0],[0,0,1]],scale=1,secondary=[2,3])],bounds=[[-2,0,0],[0,2,9]],volume=36)
for i in range(3):
 d=[2,1,3];d[i]=0;add('zero-dimension-'+str(i),d,expected='unsupported',volume=None)
add('negative-dimension',[-2,1,3],expected='unsupported',volume=None)
add('parallel-placement',position=dict(origin=[0,0,0],axis=[0,0,1],refDirection=[0,0,2]),expected='unsupported',volume=None)
add('wrong-root',expected='unsupported',volume=None,damage='wrong-root')
vec={'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}};nullable={'anyOf':[{'type':'null'},vec]};frame={'type':'object','additionalProperties':False,'required':['origin','axis','refDirection'],'properties':{'origin':vec,'axis':nullable,'refDirection':nullable}}
layer={'type':'object','additionalProperties':False,'required':['origin','originRotation','target','axes','scale','secondary'],'properties':{'origin':vec,'originRotation':{'type':'number'},'target':vec,'axes':{'type':'array','minItems':3,'maxItems':3,'items':nullable},'scale':{'type':['number','null']},'secondary':{'type':'array','minItems':2,'maxItems':2,'items':{'type':['number','null']}}}}
row={'type':'object','additionalProperties':False,'required':list(rows[0]),'properties':{'id':{'type':'string'},'dimensions':vec,'position':frame,'unitPrefix':{'enum':[None,'MILLI']},'mapping':{'type':'array','items':layer},'expected':{'enum':['supported','unsupported']},'bounds':{'anyOf':[{'type':'null'},{'type':'array','minItems':2,'maxItems':2,'items':vec}]},'volume':{'type':['number','null']},'damage':{'enum':['none','wrong-root']}}}
(p/'csg-block-neutral-cases.json').write_text(json.dumps({'schema':'IFC4X3_ADD2','cases':rows},indent=2)+'\n',encoding='utf-8');(p/'csg-block-neutral.schema.json').write_text(json.dumps({'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['schema','cases'],'properties':{'schema':{'const':'IFC4X3_ADD2'},'cases':{'type':'array','items':row}}},indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(rows))+' CSG block candidate rows')
