from pathlib import Path
import json
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
p=D/'🧪️tests/type-value-select-input.json'
law=json.loads(p.read_text(encoding='utf-8'))
new=[{'id':'array-'+name,'member':'IFCCOMPLEXNUMBER','items':[{'kind':'aggregate','value':[{'kind':'real','value':{'bits':word}}for word in words]}],'domain':valid,'physical':True}for name,words,valid in [('one',['0'],False),('two',['0','9223372036854775808'],True),('three',['0','0','0'],False)]]
new.append({'id':'angle-five','member':'IFCCOMPOUNDPLANEANGLEMEASURE','items':[{'kind':'aggregate','value':[{'kind':'integer','value':str(i)}for i in [1,2,3,4,5]]}],'domain':False,'physical':True})
assert not any(row['id']in {item['id']for item in law['cases']}for row in new)
law['cases']+=new
p.write_text(json.dumps(law,ensure_ascii=False,indent=1)+'\n',encoding='utf-8')
p=T/'type-value-select-oracle.py'
s=p.read_text(encoding='utf-8').replace("(T/'type-value-select-input.json').read_text(encoding='utf-8-sig')", "(T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-value-select-input.json').read_text(encoding='utf-8')")
p.write_text(s,encoding='utf-8',newline='\n')
