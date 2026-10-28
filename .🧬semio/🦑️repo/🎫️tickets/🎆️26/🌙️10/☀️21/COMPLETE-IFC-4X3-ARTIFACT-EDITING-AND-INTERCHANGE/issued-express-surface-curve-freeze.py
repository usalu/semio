"""🔐 Freeze new source-linked surface inputs without changing held production references."""
import hashlib
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
validation=base/'🧬️schema/⚖️validation'
owned=sorted((validation/'🧪️surface-curve').iterdir())+[ticket/name for name in ['issued-express-surface-curve-plan.md','issued-express-surface-curve-author.py','issued-express-surface-curve-oracle.py','issued-express-surface-curve-runtime.ts','issued-express-surface-curve-freeze.py','issued-express-surface-curve-implementation.md']]
refs=[validation/name for name in ['index.ts','ast.ts','parser.ts','compiler.ts','control.ts','read-scope.ts','evaluator.ts','population.ts','🧪️numeric/fixture.ts','🧪️placement/fixture.ts','🧪️coverage/cases.json','🧪️numeric/cases.json','🧪️vector/cases.json','🧪️placement/cases.json','🧪️aggregate/cases.json','🧪️vector-arithmetic/cases.json','🧪️projection/cases.json']]
refs += [base/name for name in ['🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/📚️catalog/🟦️.ts','🚪️io/step.ts','🚪️io/control.ts','🧬️schema/🔺️diff/🟦️.ts']]
def observe(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observe,owned));references=list(map(observe,refs));assert max(row['utf16'] for row in rows+references)<=256
oracle=json.loads((ticket/'🗑️generated/issued-express-surface-curve-oracle.json').read_text(encoding='utf-8'));assert oracle['casesSha256']==hashlib.sha256((validation/'🧪️surface-curve/cases.json').read_bytes()).hexdigest();assert oracle['sourceSha256']==hashlib.sha256((validation/'🧪️surface-curve/source.ifc').read_bytes()).hexdigest()
result=dict(owned=rows,references=references,productionChanged=False,registrationCandidate=(validation/'🧪️surface-curve/🧪️.test.ts').as_posix(),nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/issued-express-surface-curve-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(references),maxUtf16=max(row['utf16'] for row in rows+references))))
