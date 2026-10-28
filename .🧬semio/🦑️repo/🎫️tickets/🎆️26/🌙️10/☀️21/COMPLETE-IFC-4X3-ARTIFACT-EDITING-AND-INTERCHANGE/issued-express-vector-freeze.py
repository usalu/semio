"""🔐 Freeze only the new unregistered vector evidence and observed immutable production references."""
import hashlib
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
validation=base/'🧬️schema/⚖️validation'
owned=sorted((validation/'🧪️vector').iterdir())+[ticket/name for name in ['issued-express-vector-author.py','issued-express-vector-runtime.ts','issued-express-vector-oracle.py','issued-express-vector-freeze.py']]
refs=[validation/name for name in ['index.ts','ast.ts','compiler.ts','evaluator.ts','population.ts','control.ts','🧪️numeric/fixture.ts','🧪️numeric/cases.json','🧪️coverage/fixture.ts','🧪️coverage/cases.json']]
refs += [base/name for name in ['🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/📚️catalog/🟦️.ts','🚪️io/step.ts','🚪️io/control.ts','🧬️schema/🔺️diff/🟦️.ts']]
def observe(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observe,owned));refs=list(map(observe,refs));assert max(row['utf16'] for row in rows+refs)<=256
result=dict(owned=rows,references=refs,productionChanged=False,registrationCandidate=(validation/'🧪️vector/🧪️.test.ts').as_posix(),nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/issued-express-vector-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(refs),maxUtf16=max(row['utf16'] for row in rows+refs))))
