from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_input.ts');s=p.read_text(encoding='utf8').replace("current.kind==='primitive'&&current.name==='STRING'","current.kind==='named'&&current.name==='STRING'").replace("current.kind==='primitive'&&current.name==='INTEGER'","current.kind==='named'&&current.name==='INTEGER'");p.open('r+b').write(s.encode())
