from pathlib import Path
roots=[Path('✏️s')]
for root in roots:
 for p in root.rglob('*.rs'):
  s=p.read_text(encoding='utf-8')
  for e,v in [('Part21Value','Str'),('IfcValue','String'),('StepValue','String')]:
   if f'{e}::{v}'in s and f'{e}::Derived'in s and f'{e}::Binary'not in s and '🧪️tests'not in str(p)and 'tests.rs'not in str(p):print(e,str(p))
