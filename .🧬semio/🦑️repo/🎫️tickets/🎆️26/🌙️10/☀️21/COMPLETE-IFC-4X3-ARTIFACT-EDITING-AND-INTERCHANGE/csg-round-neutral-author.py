import pathlib,json,copy,math
p=pathlib.Path(__file__).resolve().parent
identity={'location':[0,0,0],'axis':None,'refDirection':None}
base={'primitive':'cylinder','height':3,'radius':1,'heightKind':'real','radiusKind':'real','position':identity,'productPlacement':identity,'unitPrefix':None,'mapping':None,'policy':{'absoluteChordErrorMetres':0,'relativeChordError':.001,'maxSegments':4096},'damage':'none'}
rows=[]
def add(id,**kw):
 s=copy.deepcopy(base);bounds=kw.pop('bounds',[[-1,-1,0],[1,1,3]]);n=kw.pop('segments',72);factor=kw.pop('determinant',1);status=kw.pop('status','supported');reason=kw.pop('reason','Exact declared circular CSG primitive');s.update(kw);scale=.001 if s['unitPrefix']=='MILLI'else 1;volume=None if status!='supported'else n*.5*math.sin(2*math.pi/n)*(s['radius']*scale)**2*(s['height']*scale)*factor/(3 if s['primitive']=='cone'else 1);ideal=None if status!='supported'else math.pi*(s['radius']*scale)**2*(s['height']*scale)*factor/(3 if s['primitive']=='cone'else 1)
 rows.append({'id':id,'source':s,'expected':{'status':status,'segments':n if status=='supported'else None,'bounds':bounds if status=='supported'else None,'polygonVolume':volume,'idealVolume':ideal,'reason':reason}})
add('cylinder-default');add('cone-default',primitive='cone')
add('cylinder-positioned',position={'location':[2,3,4],'axis':[0,0,1],'refDirection':[0,1,0]},bounds=[[1,2,4],[3,4,7]])
add('cone-tilted',primitive='cone',position={'location':[2,3,4],'axis':[1,0,0],'refDirection':[0,1,0]},bounds=[[2,2,3],[5,4,5]])
add('cylinder-millimetres',height=3000,radius=1000,unitPrefix='MILLI');add('cone-millimetres',primitive='cone',height=3000,radius=1000,unitPrefix='MILLI')
add('cylinder-mapped-reflected',mapping={'origin':copy.deepcopy(identity),'target':[5,-2,7],'axes':[[-1,0,0],[0,1,0],[0,0,1]],'scale':2,'secondary':[3,4]},bounds=[[3,-5,7],[7,1,19]],determinant=24)
add('cone-mapped-origin',primitive='cone',mapping={'origin':{'location':[1,2,0],'axis':[0,0,1],'refDirection':[0,1,0]},'target':[3,4,5],'axes':[[1,0,0],[0,1,0],[0,0,1]],'scale':1,'secondary':[1,1]},bounds=[[3,5,5],[5,7,8]])
add('cylinder-absolute-tight',segments=224,policy={'absoluteChordErrorMetres':.0001,'relativeChordError':0,'maxSegments':4096})
add('cone-absolute-coarse',primitive='cone',segments=8,policy={'absoluteChordErrorMetres':.1,'relativeChordError':0,'maxSegments':4096})
add('cylinder-integer-carriers',heightKind='integer',radiusKind='integer')
add('height-zero',height=0,status='refuse',reason='IfcPositiveLengthMeasure WHERE false')
add('height-negative',height=-3,status='refuse',reason='IfcPositiveLengthMeasure WHERE false')
add('radius-zero',radius=0,status='refuse',reason='IfcPositiveLengthMeasure WHERE false')
add('radius-negative',radius=-1,status='refuse',reason='IfcPositiveLengthMeasure WHERE false')
add('radius-string',radiusKind='string',status='refuse',reason='Invalid REAL defined-domain carrier')
add('foreign-position',damage='foreign-position',status='refuse',reason='Position requires actual Axis2Placement3D')
add('segment-cap',policy={'absoluteChordErrorMetres':0,'relativeChordError':.001,'maxSegments':4},status='refuse',reason='Source valid; original sampling grant cannot certify tolerance')
add('subnormal-radius',radius=math.ulp(0.),status='refuse',reason='Source positive; selected finite normal SI/precision admission refuses')
add('mapping-zero-scale',mapping={'origin':copy.deepcopy(identity),'target':[0,0,0],'axes':[[1,0,0],[0,1,0],[0,0,1]],'scale':0,'secondary':[1,1]},status='refuse',reason='TransformationOperator ScaleGreaterZero false')
obj={'schema':'ifc-csg-circular-primitive-neutral','version':1,'nativeExecution':False,'matrixComposition':'product*mappingTarget*mappingOrigin*primitivePosition','cases':rows}
triple={'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}
frame={'type':'object','additionalProperties':False,'required':['location','axis','refDirection'],'properties':{'location':triple,'axis':{'anyOf':[triple,{'type':'null'}]},'refDirection':{'anyOf':[triple,{'type':'null'}]}}}
policy={'type':'object','additionalProperties':False,'required':['absoluteChordErrorMetres','relativeChordError','maxSegments'],'properties':{'absoluteChordErrorMetres':{'type':'number','minimum':0},'relativeChordError':{'type':'number','minimum':0,'maximum':1},'maxSegments':{'type':'integer','minimum':4,'maximum':65536}}}
mapshape={'type':'object','additionalProperties':False,'required':['origin','target','axes','scale','secondary'],'properties':{'origin':frame,'target':triple,'axes':{'type':'array','minItems':3,'maxItems':3,'items':triple},'scale':{'type':'number'},'secondary':{'type':'array','minItems':2,'maxItems':2,'items':{'type':'number'}}}}
source={'type':'object','additionalProperties':False,'required':list(base),'properties':{'primitive':{'enum':['cylinder','cone']},'height':{'type':'number'},'radius':{'type':'number'},'heightKind':{'enum':['real','integer','string']},'radiusKind':{'enum':['real','integer','string']},'position':frame,'productPlacement':frame,'unitPrefix':{'enum':[None,'MILLI']},'mapping':{'anyOf':[{'type':'null'},mapshape]},'policy':policy,'damage':{'enum':['none','foreign-position']}}}
expected={'type':'object','additionalProperties':False,'required':['status','segments','bounds','polygonVolume','idealVolume','reason'],'properties':{'status':{'enum':['supported','refuse']},'segments':{'type':['integer','null'],'minimum':4},'bounds':{'anyOf':[{'type':'null'},{'type':'array','minItems':2,'maxItems':2,'items':triple}]},'polygonVolume':{'type':['number','null']},'idealVolume':{'type':['number','null']},'reason':{'type':'string','minLength':1}}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(obj),'properties':{'schema':{'const':obj['schema']},'version':{'const':1},'nativeExecution':{'const':False},'matrixComposition':{'const':obj['matrixComposition']},'cases':{'type':'array','minItems':20,'maxItems':20,'items':{'type':'object','additionalProperties':False,'required':['id','source','expected'],'properties':{'id':{'type':'string','minLength':1},'source':source,'expected':expected}}}}}
(p/'csg-round-cases.json').write_text(json.dumps(obj,indent=2)+'\n',encoding='utf8');(p/'csg-round.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8');print(json.dumps({'cases':len(rows),'supported':sum(x['expected']['status']=='supported'for x in rows)}))

