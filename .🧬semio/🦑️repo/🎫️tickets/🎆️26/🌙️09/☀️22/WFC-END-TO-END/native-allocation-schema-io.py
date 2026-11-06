from pathlib import Path
import copy,json

owner=json.loads(Path('🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/schema.json').read_text())
p=Path('🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json');s=json.loads(p.read_text());target=s['$defs']['NativeSnapshotFailure'];target['properties']['allocation']=copy.deepcopy(owner['properties']['allocation']);target['required'].append('allocation');p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
p=Path('🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🧪️tests/🟦️.test.ts');s=p.read_text();s=s.replace('},receiptRefusal:null,contextRefusal:', '},allocation:null,receiptRefusal:null,contextRefusal:');p.write_text(s)
