import pathlib,json,copy,math
p=pathlib.Path(__file__).resolve().parent
points=[[0,0,0],[1,0,0],[1,1,0],[0,1,0]];triangles=[[1,2,3],[1,3,4]];normals=[[0,0,1],[0,.6,.8],[.6,0,.8],[0,0,1]]
base={'points':points,'triangles':triangles,'normals':normals,'pnIndex':None,'closed':False}
rows=[]
def add(id,normalStatus,vertices,reason,**kw):
 source=copy.deepcopy(base);source.update(kw);rows.append({'id':id,'source':source,'expected':{'normalStatus':normalStatus,'vertices':vertices,'triangles':2,'reason':reason}})
add('smooth-unit','present',4,'Raw point and normal domains align')
add('pn-permutation','present',4,'Normals use raw indices and points use PnIndex',pnIndex=[3,1,4,2],triangles=[[2,4,1],[2,1,3]],normals=[normals[i-1]for i in [3,1,4,2]])
add('pn-hard-edge','present',6,'Six exact point-normal pairs preserve two source normal groups',pnIndex=[1,2,3,1,3,4],triangles=[[1,2,3],[4,5,6]],normals=[[0,0,1]]*3+[[0,1,0]]*3)
add('nonunit-normalize','present',4,'Finite nonunit source directions normalize only derived output',normals=[[0,0,5],[0,3,4],[3,0,4],[0,0,2]])
add('referenced-zero','omitted-zero',4,'One referenced zero omits complete authored NORMAL with explicit diagnostic',normals=[[0,0,0],*normals[1:]])
add('unused-zero','present',4,'Unused zero does not erase usable referenced normal domain',normals=normals+[[0,0,0]])
add('extra-valid-unused','present',4,'Unused valid normal row does not fabricate an attribute-count equality',normals=normals+[[1,2,3]])
add('missing-normals','absent',4,'No authored directions and no fabricated smoothing',normals=None)
obj={'schema':'ifc-gltf-authored-normal-neutral','version':1,'nativeExecution':False,'basis':'ifc-z-up-to-gltf-y-up:[x,z,-y]','cases':rows}
triple={'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}
source={'type':'object','additionalProperties':False,'required':list(base),'properties':{'points':{'type':'array','minItems':1,'items':triple},'triangles':{'type':'array','minItems':1,'items':{'type':'array','minItems':3,'maxItems':3,'items':{'type':'integer','minimum':1}}},'normals':{'anyOf':[{'type':'null'},{'type':'array','minItems':1,'items':triple}]},'pnIndex':{'anyOf':[{'type':'null'},{'type':'array','minItems':1,'items':{'type':'integer','minimum':1}}]},'closed':{'const':False}}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(obj),'properties':{'schema':{'const':obj['schema']},'version':{'const':1},'nativeExecution':{'const':False},'basis':{'const':obj['basis']},'cases':{'type':'array','minItems':8,'maxItems':8,'items':{'type':'object','additionalProperties':False,'required':['id','source','expected'],'properties':{'id':{'type':'string','minLength':1},'source':source,'expected':{'type':'object','additionalProperties':False,'required':['normalStatus','vertices','triangles','reason'],'properties':{'normalStatus':{'enum':['present','omitted-zero','absent']},'vertices':{'type':'integer','minimum':1},'triangles':{'const':2},'reason':{'type':'string','minLength':1}}}}}}}}
(p/'gltf-authored-normal-cases.json').write_text(json.dumps(obj,indent=2)+'\n',encoding='utf8');(p/'gltf-authored-normal.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8')
matrices=[('identity',[[1,0,0],[0,1,0],[0,0,1]],[1,2,3],True),('rotation-z90',[[0,1,0],[-1,0,0],[0,0,1]],[1,2,3],True),('reflection',[[-1,0,0],[0,1,0],[0,0,1]],[1,0,1],True),('nonuniform',[[2,0,0],[0,3,0],[0,0,4]],[1,1,1],True),('shear',[[1,0,0],[.5,1,0],[0,.25,1]],[1,2,3],True),('extreme-axis',[[1e300,0,0],[0,1e-300,0],[0,0,1]],[1,1,1],True),('extreme-balanced',[[1e300,0,0],[0,1e-300,0],[0,0,1e-100]],[1e300,1e-300,1e-100],True),('extreme-reflected',[[-1e300,0,0],[0,1e-300,0],[0,0,1e100]],[1e300,1e-300,1e100],True),('singular',[[1,0,0],[2,0,0],[0,0,1]],[1,2,3],False),('zero-normal',[[1,0,0],[0,1,0],[0,0,1]],[0,0,0],False)]
mo={'schema':'normal-covector-neutral','version':1,'nativeExecution':False,'matrixLayout':'columns','cases':[{'id':id,'columns':cols,'normal':normal,'directionAvailable':ok,'oracle':'three+mpmath' if i<5 else'mpmath'}for i,(id,cols,normal,ok)in enumerate(matrices)]}
ms={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(mo),'properties':{'schema':{'const':mo['schema']},'version':{'const':1},'nativeExecution':{'const':False},'matrixLayout':{'const':'columns'},'cases':{'type':'array','minItems':10,'maxItems':10,'items':{'type':'object','additionalProperties':False,'required':['id','columns','normal','directionAvailable','oracle'],'properties':{'id':{'type':'string'},'columns':{'type':'array','minItems':3,'maxItems':3,'items':triple},'normal':triple,'directionAvailable':{'type':'boolean'},'oracle':{'enum':['three+mpmath','mpmath']}}}}}}
expected={'identity':[1/math.sqrt(14),2/math.sqrt(14),3/math.sqrt(14)],'rotation-z90':[-2/math.sqrt(14),1/math.sqrt(14),3/math.sqrt(14)],'reflection':[-1/math.sqrt(2),0,1/math.sqrt(2)],'nonuniform':[6/math.sqrt(61),4/math.sqrt(61),3/math.sqrt(61)],'shear':[8/math.sqrt(649),12/math.sqrt(649),21/math.sqrt(649)],'extreme-axis':[0,1,1e-300],'extreme-balanced':[1/math.sqrt(3)]*3,'extreme-reflected':[-1/math.sqrt(3),1/math.sqrt(3),1/math.sqrt(3)],'singular':None,'zero-normal':None}
for row in mo['cases']:row['expectedDirection']=expected[row['id']]
ms['properties']['cases']['items']['required'].append('expectedDirection')
ms['properties']['cases']['items']['properties']['expectedDirection']={'anyOf':[{'type':'null'},{'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}]}
(p/'normal-matrix-cases.json').write_text(json.dumps(mo,indent=2)+'\n',encoding='utf8');(p/'normal-matrix.schema.json').write_text(json.dumps(ms,indent=2)+'\n',encoding='utf8')
print(json.dumps({'sourceCases':len(rows),'matrixCases':len(matrices)}))


