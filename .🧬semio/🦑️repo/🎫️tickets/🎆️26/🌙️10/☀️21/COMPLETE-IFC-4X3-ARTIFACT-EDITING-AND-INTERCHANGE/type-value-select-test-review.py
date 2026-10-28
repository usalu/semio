from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_value_select_input.test.ts')
s=p.read_text(encoding='utf-8-sig').replace("{kind:'text',format:'step-binary'}","{kind:'text',format:'text',pattern:'^[0-3][0-9A-F]*$'}")
p.write_text(s,encoding='utf-8',newline='\n')
