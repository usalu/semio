from pathlib import Path
import json
I=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any')
for p in (I/'🧬️schema').rglob('🔣️.json'):
 try:d=json.loads(p.read_text(encoding='utf-8'))
 except:continue
 if 'IfcValue' in d.get('$defs',{}):print(str(p.relative_to(I)),json.dumps(d['$defs']['IfcValue'])[:1800])
