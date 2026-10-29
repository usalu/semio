"""🧵 Verify actual native STEP token annotation without constructing geometry."""
import importlib,json
from pathlib import Path
TICKET=Path(__file__).parent
native=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
case=json.loads((TICKET/'step-empty-token-r62.json').read_text())
rows=[]
for example in case['cases']:
    writer=native._Writer(1,[])
    writer.raw(['IFCTEXT(',native._str(example['value'],'value'),')'])
    assert writer.lines==[example['serialized']]
    invalid=[span for span in writer.spans if not 0<=span['start']<span['end']<=len(writer.lines[span['line']])]
    rows.append({'id':example['id'],'serialized':writer.lines[0],'spans':writer.spans,'invalid':invalid})
(TICKET/'🗑️generated/step-empty-token-r62.json').write_text(json.dumps(rows))
print('STEP_TOKEN_CASES',json.dumps(rows))
assert not any(row['invalid'] for row in rows)
