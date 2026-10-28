from pathlib import Path
import json
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
for p in [A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔣️.json',A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json']:
 d=json.loads(p.read_text(encoding='utf-8'))
 print(json.dumps(d.get('$defs',{}).get('StepValue',d.get('definitions',{}).get('Part21Value',{})),ensure_ascii=True)[:2000])
