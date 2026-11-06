from pathlib import Path
import copy,json

p=Path('🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/schema.json')
s=json.loads(p.read_text()); error=copy.deepcopy(s['properties']['receiptRefusal']); actual={'anyOf':[{'type':'null'},{'type':'integer','minimum':0}]}
variants=[]
for name, fields in [('encode',['primary','extentRefusal','forwardedRefusal','scopedRefusal']),('decode',['primary','extentRefusal','settlementRefusal'])]:
 props={'type':{'const':name},'actual':actual};props.update({field:copy.deepcopy(error) for field in fields})
 variants.append({'type':'object','additionalProperties':False,'required':['type',*fields,'actual'],'properties':props})
s['properties']['allocation']={'anyOf':[{'type':'null'},{'oneOf':variants}]}
s['required'].append('allocation')
p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
