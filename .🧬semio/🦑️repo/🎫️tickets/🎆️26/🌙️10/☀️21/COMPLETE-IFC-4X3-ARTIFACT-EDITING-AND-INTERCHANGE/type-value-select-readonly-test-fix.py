from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_value_select_input.test.ts')
s=p.read_text(encoding='utf-8')
a="current.page={fieldOffset:'0',diagnosticOffset:'0',choice:{path:['entities','0','args','0'],offset:'93'}};form="
b="current={...current,page:{fieldOffset:'0',diagnosticOffset:'0',choice:{path:['entities','0','args','0'],offset:'93'}}};form="
assert s.count(a)==1
s=s.replace(a,b)
p.write_text(s,encoding='utf-8',newline='\n')
