from pathlib import Path
import json

for name,fragment in [('🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/schema.json','#/properties/receiptRefusal'),('🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json','#/$defs/NativeSnapshotFailure/properties/receiptRefusal')]:
 p=Path(name);s=json.loads(p.read_text());target=s if 'properties/receiptRefusal'==fragment[2:] else s['$defs']['NativeSnapshotFailure'];variants=target['properties']['allocation']['anyOf'][1]['oneOf']
 for variant in variants:
  for field in variant['required']:
   if field not in ('type','actual'):variant['properties'][field]={'$ref':fragment}
 p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
