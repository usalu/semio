"""🧪 Register the shared exchange and controlled geometry regression selectors."""
import json,sys
from pathlib import Path
artifact=Path(sys.argv[1])
io=artifact/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests'
cases_path=io/'geometry-progress-cases.json'
schema_path=io/'geometry-progress.schema.json'
cases=json.loads(cases_path.read_text(encoding='utf-8-sig'))
schema=json.loads(schema_path.read_text(encoding='utf-8-sig'))
phases=['units','placements','representations','items','pn-indices','geometry-unbound']
faults=['error','step-error','edit-error','edit-cancelled','undefined','number']
for phase in phases:
    for endpoint in ['triangles','glb-export']:
        for fault in faults:
            row={'phase':phase,'fault':fault,'endpoint':endpoint,'expected':'reject-original'}
            if row not in cases['cases']:cases['cases'].append(row)
allowed=schema['properties']['cases']['items']['properties']['phase']['enum']
allowed.extend(phase for phase in phases if phase not in allowed)
cases_path.write_text(json.dumps(cases,ensure_ascii=False,indent=1)+'\n',encoding='utf-8')
schema_path.write_text(json.dumps(schema,ensure_ascii=False,indent=1)+'\n',encoding='utf-8')
script_path=artifact/'📦️packages/🟦️typescript/📜️script.ts'
source=script_path.read_text(encoding='utf-8')
for name in ['exchange-progress','unit-traversal']:
    selector='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/'+name+'.test.ts",'
    if selector not in source:source=source.replace(']});',selector+'\n]});')
script_path.write_text(source,encoding='utf-8')
print('[DEBUG] Registered two actual IFC4x3 selectors and extended original-caller traversal vectors')
