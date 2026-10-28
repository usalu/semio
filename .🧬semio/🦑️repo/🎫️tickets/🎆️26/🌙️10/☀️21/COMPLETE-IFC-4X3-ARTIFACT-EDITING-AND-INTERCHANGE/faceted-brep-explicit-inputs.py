import json
from pathlib import Path
p=Path(__file__).resolve().parent
v=json.loads((p/'faceted-brep-geometry-vectors.json').read_text())
for r in v['cases']:
 m=r.pop('mode');outer=[[0,0],[2,0],[2,1],[0,1]];holes=[];height=3
 if m=='concave':outer=[[0,0],[3,0],[3,1],[1,1],[1,3],[0,3]];height=2
 if m=='hole':outer=[[0,0],[6,0],[6,6],[0,6]];holes=[[[2,2],[2,4],[4,4],[4,2]]];height=2
 r['geometry']={'outer':outer,'holes':holes,'base':0,'height':height,'voids':[{'outer':[[.5,.25],[1.5,.25],[1.5,.75],[.5,.75]],'holes':[],'base':1,'height':1,'reverse':True}]if m=='voids'else[]}
 r['unit']={'prefix':'MILLI'if m=='mm'else None,'coordinateScale':1000 if m=='mm'else 1,'siScale':.001 if m=='mm'else 1}
 r['placement']={'origin':[10,20,30]if m=='rigid'else[0,0,0],'rotation':.3 if m=='rigid'else 0}
 r['orientation']={'reversePolygon':m=='false','bound':m!='false'}
 r['modifier']={'kind':'point-coordinates','ordinal':0,'coordinates':[0,0,.2]}if m=='nonplanar'else{'kind':'polygon-order','ordinal':0,'order':[0,2,1,3]}if m=='crossed'else{'kind':'point-coordinates','ordinal':0,'dimensions':2}if m=='point2d'else{'kind':m if m in ['no-outer','two-outer','empty-bounds','wrong-bound','wrong-loop','wrong-point']else'none'}
(p/'faceted-brep-geometry-vectors.json').write_text(json.dumps(v,indent=2)+'\n')
number={'type':'number'};point={'type':'array','minItems':2,'maxItems':2,'items':number};ring={'type':'array','minItems':3,'items':point};rings={'type':'array','items':ring};obj=lambda props,required=None:{'type':'object','additionalProperties':False,'required':required or list(props),'properties':props}
void=obj({'outer':ring,'holes':rings,'base':number,'height':{'type':'number','exclusiveMinimum':0},'reverse':{'type':'boolean'}})
geometry=obj({'outer':ring,'holes':rings,'base':number,'height':{'type':'number','exclusiveMinimum':0},'voids':{'type':'array','items':void}})
modifier={'oneOf':[obj({'kind':{'enum':['none','no-outer','two-outer','empty-bounds','wrong-bound','wrong-loop','wrong-point']}}),obj({'kind':{'const':'polygon-order'},'ordinal':{'type':'integer','minimum':0},'order':{'type':'array','minItems':3,'items':{'type':'integer','minimum':0}}}),obj({'kind':{'const':'point-coordinates'},'ordinal':{'type':'integer','minimum':0},'coordinates':{'type':'array','minItems':3,'maxItems':3,'items':number}}),obj({'kind':{'const':'point-coordinates'},'ordinal':{'type':'integer','minimum':0},'dimensions':{'const':2}})]}
row=obj({'id':{'type':'string','pattern':'^[a-z][a-z0-9-]*$'},'expected':{'enum':['supported','unsupported']},'geometry':geometry,'unit':obj({'prefix':{'enum':[None,'MILLI']},'coordinateScale':{'type':'number','exclusiveMinimum':0},'siScale':{'type':'number','exclusiveMinimum':0}}),'placement':obj({'origin':{'type':'array','minItems':3,'maxItems':3,'items':number},'rotation':number}),'orientation':obj({'reversePolygon':{'type':'boolean'},'bound':{'type':'boolean'}}),'modifier':modifier,'volume':{'type':'number','exclusiveMinimum':0},'bounds':{'type':'array','minItems':2,'maxItems':2,'items':{'type':'array','minItems':3,'maxItems':3,'items':number}}},['id','expected','geometry','unit','placement','orientation','modifier'])
s={'$schema':'https://json-schema.org/draft/2020-12/schema',**obj({'schema':{'const':'ifc-faceted-brep-geometry-vectors-v1'},'nativeExecution':{'const':False},'cases':{'type':'array','minItems':16,'maxItems':16,'items':row}})}
(p/'faceted-brep-geometry-vectors.schema.json').write_text(json.dumps(s,indent=2)+'\n')

