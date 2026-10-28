import json
from pathlib import Path
p=Path(__file__).parent
vec={'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}
layer={'type':'object','additionalProperties':False,'required':['origin','originRotation','target','axes','scale','secondary'],'properties':{'origin':{'$ref':'#/$defs/vec'},'originRotation':{'type':'number'},'target':{'$ref':'#/$defs/vec'},'axes':{'type':'array','minItems':3,'maxItems':3,'items':{'anyOf':[{'$ref':'#/$defs/vec'},{'type':'null'}]}},'scale':{'type':['number','null']},'secondary':{'type':'array','minItems':2,'maxItems':2,'items':{'type':['number','null']}}}}
bounds={'type':'array','minItems':2,'maxItems':2,'items':{'$ref':'#/$defs/vec'}}
row={'type':'object','additionalProperties':False,'required':['id','layers','repeatedSource','expectedBounds','expectedVolume'],'properties':{'id':{'type':'string','minLength':1},'layers':{'type':'array','minItems':1,'maxItems':2,'items':{'$ref':'#/$defs/layer'}},'repeatedSource':{'type':'boolean'},'expectedBounds':bounds,'expectedVolume':{'type':'number','exclusiveMinimum':0}}}
refusal={'type':'object','additionalProperties':False,'required':['id','layers','damage'],'properties':{'id':{'type':'string','minLength':1},'layers':{'type':'array','minItems':1,'maxItems':2,'items':{'$ref':'#/$defs/layer'}},'damage':{'enum':[None,'cycle','two-dimensional-operator']}}}
s={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','baseBounds','layerOrder','cases','refusals'],'properties':{'version':{'const':1},'baseBounds':bounds,'layerOrder':{'const':'inner-to-outer'},'cases':{'type':'array','minItems':8,'maxItems':8,'items':row},'refusals':{'type':'array','minItems':5,'maxItems':5,'items':refusal}},'$defs':{'vec':vec,'layer':layer}}
(p/'mapping-neutral.schema.json').write_text(json.dumps(s,indent=2)+'\n',encoding='utf-8')
