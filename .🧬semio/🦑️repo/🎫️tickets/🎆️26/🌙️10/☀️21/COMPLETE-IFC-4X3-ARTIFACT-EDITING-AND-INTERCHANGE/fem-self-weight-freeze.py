"""🔐 Freeze the exact self-weight source and original-control metadata used for independent review."""
import hashlib
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
fem=repo/'✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any'
owned=sorted((base/'🚪️io/fem/self-weight').iterdir())
owned += [base/'🚪️io'/name for name in ['fem.ts','native_fem_load_build.rs','native_fem_load_import.rs','native_exchange.rs','fem/nodal/native-export-tests.rs','🧪️tests/fem.test.ts']]
owned += [fem/'🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs']
owned += [ticket/name for name in ['fem-self-weight-author.py','fem-self-weight-runtime.ts','fem-self-weight-oracle.py','fem-self-weight-freeze.py']]
references=[base/name for name in ['🚪️io/relations.ts','🚪️io/control.ts','🚪️io/geometry.ts','🚪️io/external-source.ts','🚪️io/step.ts','🚪️io/step-writer.ts','🚪️io/native_fem_owned.rs','🚪️io/native_fem_load_query.rs','🚪️io/native_fem_build.rs','🚪️io/fem/nodal/native-support.rs','🚪️io/fem/nodal/native-tests.rs','🚪️io/🧪️tests/fem-isolated-fixtures.ts','🚪️io/🧪️tests/structural.ifc','🚪️io/🧪️tests/structural.provenance.json','🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/🔺️diff/🟦️.ts','🧬️schema/🧬️mutations/🟦️.ts']]
references += [fem/name for name in ['🧬️schema/📸️snapshot/🟦️.ts','🧬️schema/🏗️structure/🦀️.rs','✏️editor/📌️panels/🔍️inspection/🦀️.rs','✏️editor/📋️manifest/🦀️.rs']]
references += [repo/'✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/⚙️engine/🧊️3d/🦀️.rs']
def observation(path):
    return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observation,owned));refs=list(map(observation,references))
assert max(row['utf16'] for row in rows+refs)<=256
physical=Path(ticket/'🗑️generated/fem-self-weight')
result=dict(owned=rows,references=refs,physical=[observation(path) for path in sorted(physical.glob('*.ifc'))],registrationCandidate=(base/'🚪️io/fem/self-weight/self-weight.test.ts').as_posix(),nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/fem-self-weight-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(refs),physical=len(result['physical']),maxUtf16=max(row['utf16'] for row in rows+refs))))
