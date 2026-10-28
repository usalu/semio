from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_input_tests.rs');s=p.read_text(encoding='utf8').rstrip()+'\n#[path = "type_multiple_input_tests.rs"]\nmod multiple;\n';p.open('r+b').write(s.encode())
