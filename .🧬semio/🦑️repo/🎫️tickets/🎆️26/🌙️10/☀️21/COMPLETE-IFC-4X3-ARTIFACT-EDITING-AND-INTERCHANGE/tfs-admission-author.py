import pathlib,json,copy
p=pathlib.Path(__file__).resolve().parent
v=[[0,0,0],[2,0,0],[0,1,0],[0,0,3]]
t=[[1,3,2],[1,2,4],[2,3,4],[3,1,4]]
base={'points':v,'triangles':t,'normals':None,'pnIndex':None,'closed':True,'coordinateKind':'real','indexKind':'integer','pnKind':'integer','normalKind':'real','coordinateRole':'IfcCartesianPointList3D','closedKind':'boolean'}
rows=[]
def add(id,status,reason,**kw):
 d=copy.deepcopy(base);d.update(kw);rows.append({'id':id,'source':d,'expected':status,'reason':reason})
add('closed-tetra','accept','Closed oriented tetrahedron')
add('open-single-triangle','accept','Surface, no imposed solid closure',triangles=[[1,2,3]],closed=False)
add('unknown-single-triangle','accept','Closed omitted has no affirmative closure claim',triangles=[[1,2,3]],closed=None)
add('closed-open-triangle','refuse','Informal Closed topology',triangles=[[1,2,3]])
add('closed-inward','refuse','Topological normal must be outward',triangles=[list(reversed(x)) for x in t])
add('closed-duplicate-face','refuse','Closed faces must be unique',triangles=t+[t[0]])
add('real-coordinate-index','refuse','PositiveInteger requires integer carrier',indexKind='real')
add('zero-coordinate-index','refuse','PositiveInteger lower bound',triangles=[[0,2,3]])
add('negative-coordinate-index','refuse','PositiveInteger lower bound',triangles=[[-1,2,3]])
add('index-out-of-range','refuse','Coordinate lookup absent',triangles=[[1,2,5]])
add('index-row-two','refuse','Exactly3 indices',triangles=[[1,2]])
add('empty-triangles','refuse','LIST1 lower bound',triangles=[])
add('empty-points','refuse','LIST1 lower bound',points=[])
add('point-two-components','refuse','Exactly3 coordinates',points=[[0,0],*v[1:]])
add('integer-coordinate-value','accept','IfcLengthMeasure REAL accepts integer numeric input',coordinateKind='integer')
add('foreign-coordinate-role','refuse','Coordinates exact declared point-list role',coordinateRole='IfcCartesianPointList2D')
add('closed-logical-unknown','refuse','OPTIONAL Boolean differs from Logical.UNKNOWN',closed=None,closedKind='logical')
add('pn-permutation','accept','Raw CoordIndex indexes mapping table',pnIndex=[4,3,2,1],triangles=[[4,2,3],[4,3,1],[3,2,1],[2,4,1]])
add('real-pn-entry','refuse','PnIndex PositiveInteger carrier',pnIndex=[1,2,3,4],pnKind='real')
add('unused-invalid-pn','refuse','Validate entire declared mapping including unused entries',pnIndex=[1,2,3,4,0])
add('unused-oob-pn','refuse','All mapping locations must exist',pnIndex=[1,2,3,4,5])
add('empty-pn','refuse','Optional present LIST1 not empty',pnIndex=[])
add('pn-alias-coordinate','accept','PnIndex is not UNIQUE',pnIndex=[1,2,3,4,1])
add('unused-valid-point','accept','Projection policy retains source and admits unused valid coordinates',points=v+[[8,9,10]])
add('coordinate-alias-closed','accept','Exact geometry identities may close distinct source indices',points=[v[i-1] for face in t for i in face],triangles=[[i+1,i+2,i+3] for i in range(0,12,3)])
add('degenerate-alias-triangle','refuse','Three indices alone do not establish nondegenerate geometry',points=v+[v[0]],triangles=[[1,5,2]],closed=False)
add('normals-valid','accept','Normals are per raw index, finite REAL triples',normals=[[0,0,-1],[1,0,1],[0,1,1],[-1,-1,1]])
add('normals-short','refuse','Referenced raw normal index absent',normals=[[0,0,-1]])
add('normals-empty','refuse','Optional present LIST1 lower bound',normals=[])
add('normals-two-components','refuse','Exactly3 parameter values',normals=[[0,1]]*4)
add('normals-extra-valid','accept','No formal equality cardinality rule',normals=[[0,0,-1],[1,0,1],[0,1,1],[-1,-1,1],[1,2,3]])
add('pn-normal-domain','accept','Normals use raw indices before point mapping',points=v,pnIndex=[1,2,3,4,1],triangles=[[5,3,2],[5,2,4],[2,3,4],[3,5,4]],normals=[[0,0,-1],[1,0,1],[0,1,1],[-1,-1,1],[0,0,-1]])
obj={'schema':'ifc-tfs-admission-neutral','version':1,'nativeExecution':False,'cases':rows}
(p/'tfs-admission-cases.json').write_text(json.dumps(obj,indent=2)+'\n',encoding='utf8')
triple={'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}
source={'type':'object','additionalProperties':False,'required':list(base),'properties':{'points':{'type':'array','items':{'type':'array','items':{'type':'number'}}},'triangles':{'type':'array','items':{'type':'array','items':{'type':'integer'}}},'normals':{'anyOf':[{'type':'null'},{'type':'array','items':{'type':'array','items':{'type':'number'}}}]},'pnIndex':{'anyOf':[{'type':'null'},{'type':'array','items':{'type':'integer'}}]},'closed':{'type':['boolean','null']},'coordinateKind':{'enum':['integer','real']},'indexKind':{'enum':['integer','real']},'pnKind':{'enum':['integer','real']},'normalKind':{'enum':['integer','real']},'coordinateRole':{'enum':['IfcCartesianPointList3D','IfcCartesianPointList2D']},'closedKind':{'enum':['boolean','logical']}}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['schema','version','nativeExecution','cases'],'properties':{'schema':{'const':obj['schema']},'version':{'const':1},'nativeExecution':{'const':False},'cases':{'type':'array','items':{'type':'object','additionalProperties':False,'required':['id','source','expected','reason'],'properties':{'id':{'type':'string'},'source':source,'expected':{'enum':['accept','refuse']},'reason':{'type':'string'}}}}}}
(p/'tfs-admission.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8')
print(len(rows))
