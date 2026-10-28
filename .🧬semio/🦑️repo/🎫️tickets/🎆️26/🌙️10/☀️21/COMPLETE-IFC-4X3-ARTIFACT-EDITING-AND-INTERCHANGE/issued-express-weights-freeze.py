"""🔐 Freeze only the new populated aggregate evidence and observed unchanged production."""
import hashlib
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
validation=base/'🧬️schema/⚖️validation'
owned=sorted((validation/'🧪️aggregate').iterdir())+[ticket/name for name in ['issued-express-weights-author.py','issued-express-weights-runtime.ts','issued-express-weights-oracle.py','issued-express-weights-freeze.py']]
refs=[validation/name for name in ['index.ts','ast.ts','compiler.ts','evaluator.ts','population.ts','control.ts','🧪️numeric/fixture.ts','🧪️numeric/cases.json','🧪️coverage/fixture.ts','🧪️coverage/cases.json','🧪️vector/cases.json','🧪️placement/cases.json']]
refs += [base/name for name in ['🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/📚️catalog/🟦️.ts','🚪️io/step.ts','🚪️io/control.ts','🧬️schema/🔺️diff/🟦️.ts']]
def observe(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observe,owned));refs=list(map(observe,refs));assert max(row['utf16'] for row in rows+refs)<=256
result=dict(owned=rows,references=refs,productionChanged=False,registrationCandidate=(validation/'🧪️aggregate/🧪️.test.ts').as_posix(),nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/issued-express-weights-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(refs),maxUtf16=max(row['utf16'] for row in rows+refs))))
