from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_scalar_input.test.ts');s=p.read_text(encoding='utf8');s=s.replace("const current=request('IFCDERIVEDUNITELEMENT');current.args={path:","const current={...request('IFCDERIVEDUNITELEMENT'),args:{path:");s=s.replace("value:payload}]}};await expect", "value:payload}]}}};await expect");data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
