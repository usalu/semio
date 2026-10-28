"""🔐 Record exact numeric source custody and the selected owner union without claiming total branch coverage."""
import hashlib
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
validation=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation'
old=json.loads((ticket/'🗑️generated/issued-owner-source.json').read_text(encoding='utf-8'))
def observation(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper())
owned=[validation/'🧪️numeric'/name for name in ['cases.json','cases.schema.json','fixture.ts','🧪️.test.ts','coverage.feature']]+[ticket/name for name in ['issued-express-numeric-author.py','issued-express-numeric-oracle.py','issued-express-numeric-runtime.ts','issued-express-numeric-freeze.py']]
references=[Path(row['path']) for row in old['references']]+[validation/'parser.ts',validation/'🧪️coverage/fixture.ts']
previous={row['path']:row['sha256'].upper() for row in old['references']}
observed=[observation(path) for path in references]
changed=[row['path'] for row in observed if row['path'] in previous and row['sha256']!=previous[row['path']]]
rows=json.loads((validation/'🧪️numeric/cases.json').read_text(encoding='utf-8'))['functions']
prior=json.loads((validation/'🧪️coverage/cases.json').read_text(encoding='utf-8'))
selected={row['owner'] for row in prior['functions']+rows}
allowners=json.loads((validation/'🧪️coverage/owners.json').read_text(encoding='utf-8'))['owners']
census=dict(scope='selected-meaningful-boundaries-not-total-branch-coverage',functionOwners=48,globalOwners=2,firstSelectedFunctions=len({row['owner'] for row in prior['functions']}),additionalSelectedFunctions=len({row['owner'] for row in rows}),unionSelectedFunctions=len(selected),otherExistingEvidenceUnscored=[row['owner'] for row in allowners if row['kind']=='function' and row['owner'] not in selected],newOwners=[dict(owner=name,cases=[row['id'] for row in rows if row['owner']==name],sourceStartLine=next(row['sourceStartLine'] for row in allowners if row['owner']==name),totalBranchCoverage=False) for name in sorted({row['owner'] for row in rows})])
paths=owned+references
assert all(len(str(path))<=256 for path in paths)
assert not changed,changed
result=dict(owned=[observation(path) for path in owned],references=observed,productionChanged=False,nativeExecuted=False,capabilityEnabled=False,registrationCandidate=(validation/'🧪️numeric/🧪️.test.ts').as_posix(),census=census)
(ticket/'🗑️generated/issued-numeric-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(owned),references=len(references),cases=len(rows),remainingUnscored=len(census['otherExistingEvidenceUnscored']),maxPath=max(map(lambda path:len(str(path)),paths)),productionChanged=False)))
