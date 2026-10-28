from pathlib import Path
import json
p=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE')
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-enum-input.schema.json')
s=json.loads(f.read_text(encoding='utf8'))
def closed(properties,required=None):return {'type':'object','additionalProperties':False,'required':required or list(properties),'properties':properties}
text={'type':'string'};unsigned={'type':'string','pattern':'^(0|[1-9][0-9]{0,19})$'};ref={'$ref':'#/definitions/value'};values={'type':'array','items':ref}
kinds=[]
for kind in ['unset','derived']:kinds.append(closed({'kind':{'const':kind}}))
for kind,value in [('string',text),('enum',text),('integer',{'type':'string','pattern':'^(0|-?[1-9][0-9]{0,18})$'}),('reference',{'type':'string','pattern':'^[1-9][0-9]{0,19}$'}),('real',closed({'bits':unsigned})),('aggregate',values)]:kinds.append(closed({'kind':{'const':kind},'value':value}))
s['definitions']={'value':{'oneOf':kinds},'part':closed({'name':text,'args':values}),'entity':closed({'id':{'type':'string','pattern':'^[1-9][0-9]{0,19}$'},'name':text,'args':values,'complex':{'type':'array','items':{'$ref':'#/definitions/part'}}})}
s['properties']['sources']['additionalProperties']['properties']['entities']['items']={'$ref':'#/definitions/entity'}
s['properties']['cases']['items']['properties']['args']=values
f.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
