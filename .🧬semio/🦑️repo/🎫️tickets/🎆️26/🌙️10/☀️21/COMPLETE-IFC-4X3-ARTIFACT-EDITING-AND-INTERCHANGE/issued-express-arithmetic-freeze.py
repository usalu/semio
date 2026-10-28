"""🔐 Freeze the arithmetic/read-scope correction and exact unchanged source references."""
import hashlib
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
validation=base/'🧬️schema/⚖️validation'
owned=sorted((validation/'🧪️vector-arithmetic').iterdir())+[validation/name for name in ['read-scope.ts','evaluator.ts','population.ts']]
owned += [ticket/name for name in ['issued-express-vector-arithmetic-author.py','issued-express-isolation-author.py','issued-express-arithmetic-oracle.py','issued-express-arithmetic-runtime.ts','issued-express-arithmetic-freeze.py','issued-express-vector-arithmetic-plan.md','issued-express-arithmetic-isolation-before.md','issued-express-read-scope-plan.md','issued-express-arithmetic-implementation.md']]
refs=[validation/name for name in ['index.ts','ast.ts','parser.ts','compiler.ts','control.ts','🧪️numeric/fixture.ts','🧪️coverage/cases.json','🧪️numeric/cases.json','🧪️vector/cases.json','🧪️vector/source.ifc','🧪️vector/fixture.ts','🧪️placement/cases.json','🧪️placement/fixture.ts','🧪️aggregate/cases.json','validation.test.ts']]
refs += [base/name for name in ['🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/📚️catalog/🟦️.ts','🚪️io/step.ts','🚪️io/control.ts','🧬️schema/🔺️diff/🟦️.ts']]
def observe(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observe,owned));references=list(map(observe,refs));assert max(row['utf16'] for row in rows+references)<=256
result=dict(owned=rows,references=references,productionChanged=True,registrationCandidates=[(validation/'🧪️vector-arithmetic'/name).as_posix() for name in ['🧪️.test.ts','isolation.test.ts']],nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/issued-express-arithmetic-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(references),maxUtf16=max(row['utf16'] for row in rows+references))))
