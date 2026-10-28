import pathlib,json,math
p=pathlib.Path(__file__).resolve().parent
base={'primitive':'cylinder','height':3,'radius':1,'position':{'location':[0,0,0],'axis':[0,0,1],'refDirection':[1,0,0]},'unitPrefix':None}
rows=[]
def add(id,target,damage,formal,admission,word=None):
 rows.append({'id':id,'base':base,'damage':{'target':target,'operation':damage,'realWord':word},'expected':{'formal':formal,'admission':admission}})
for target in ['primitive','root','position','point','direction']:
 add(target+'-extra-arity',target,'appendUnset','invalid-arity','refuse')
 add(target+'-missing-arity',target,'removeLast','invalid-arity','refuse')
add('primitive-complex','primitive','complexConstituent','not-asserted','refuse-selected-declaration')
add('root-complex','root','complexConstituent','not-asserted','refuse-selected-declaration')
add('missing-position','primitive','unsetPosition','invalid-required','refuse')
add('foreign-position','primitive','pointPosition','invalid-role','refuse')
for name,word in [('positive-infinity','9218868437227405312'),('negative-infinity','18442240474082181120'),('quiet-nan','9221120237041090560')]:
 add(name,'radius','realBits','nonphysical-source','refuse-nonfinite',word)
add('finite-translated-collapse','position','translateAll1e20','valid-physical','refuse-projected-degeneracy')
obj={'schema':'ifc-csg-circular-supplemental','version':1,'nativeExecution':False,'cases':rows}
(p/'csg-round-supplemental-cases.json').write_text(json.dumps(obj,indent=2)+'\n',encoding='utf8')
def closed(props):return {'type':'object','additionalProperties':False,'required':list(props),'properties':props}
vec={'type':'array','items':{'type':'number'},'minItems':3,'maxItems':3}
frame=closed({'location':vec,'axis':{'anyOf':[vec,{'type':'null'}]},'refDirection':{'anyOf':[vec,{'type':'null'}]}})
baseSchema=closed({'primitive':{'const':'cylinder'},'height':{'const':3},'radius':{'const':1},'position':frame,'unitPrefix':{'type':'null'}})
row=closed({'id':{'type':'string','minLength':1},'base':baseSchema,'damage':closed({'target':{'enum':['primitive','root','position','point','direction']},'operation':{'enum':['appendUnset','removeLast','complexConstituent','unsetPosition','pointPosition','realBits','translateAll1e20']},'realWord':{'anyOf':[{'type':'string','pattern':'^[0-9]+$'},{'type':'null'}]}}),'expected':closed({'formal':{'enum':['invalid-arity','not-asserted','invalid-required','invalid-role','nonphysical-source','valid-physical']},'admission':{'enum':['refuse','refuse-selected-declaration','refuse-nonfinite','refuse-projected-degeneracy']}})})
schema={'$schema':'https://json-schema.org/draft/2020-12/schema',**closed({'schema':{'const':obj['schema']},'version':{'const':1},'nativeExecution':{'const':False},'cases':{'type':'array','minItems':18,'maxItems':18,'items':row}})}
(p/'csg-round-supplemental.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8')

