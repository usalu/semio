"""🔐 Record exact occurrence-property implementation, canonical native test mounts and shared source observations."""
import hashlib
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
ifc=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
bim=repo/'✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any'
owned=[ifc/'🚪️io'/name for name in ['bim.ts','relations.ts','🧪️tests/traversal.test.ts','🧪️tests/traversal-cases.json','🧪️tests/traversal.schema.json']]
owned += [ifc/'🚪️io/bim/properties'/name for name in ['🟦️.ts','fixture.ts','cases.json','schema.json','properties.test.ts','properties.feature','relation-cases.json','relation.schema.json','text-cases.json','text.schema.json']]
owned += [bim/name for name in ['🧬️schema/🧬️mutations/🦀️.rs','🧬️schema/🧬️mutations/🧪️tests/occurrence.rs','🧬️schema/🧬️mutations/🧪️tests/occurrence.json','🧬️schema/🧬️mutations/🧪️tests/occurrence.schema.json','✏️editor/📌️panels/🔍️properties/🦀️.rs','✏️editor/📌️panels/🔍️properties/🧪️tests/🔬️unit/🦀️.rs','✏️editor/📌️panels/🔍️properties/🧪️tests/🔬️unit/occurrence.rs']]
owned += [bim/'✏️editor/📌️panels/🔍️properties/🏷️data/🦀️.rs']
owned += [ticket/name for name in ['bim-occurrence-properties-author.py','bim-occurrence-command-author.py','bim-occurrence-properties-runtime.ts','bim-occurrence-properties-oracle.py','bim-occurrence-properties-freeze.py']]
references=[ifc/name for name in ['🚪️io/control.ts','🚪️io/external-source.ts','🚪️io/source-rows.ts','🚪️io/step.ts','🚪️io/step-writer.ts','🚪️io/bim/phases/🟦️.ts','🧬️schema/📚️catalog/🟦️.ts','🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🧬️schema/🔺️diff/🟦️.ts','🧬️schema/🧬️mutations/🟦️.ts','🧪️fixtures/official/linear-placement-of-signal.ifc']]
references += [bim/name for name in ['🧬️schema/🔣️.json','🧬️schema/🟦️.ts','🧬️schema/📸️snapshot/💠️values/🦀️.rs','🧬️schema/🧬️mutations/🧾️set-element-property/🔺️diff/🦀️.rs','🧬️schema/🧬️mutations/🧾️set-element-property/↩️inverse/🦀️.rs','🧬️schema/🧬️mutations/🫧️remove-element-property/🔺️diff/🦀️.rs','✏️editor/🎮️commands/🧾️set-property/🦀️.rs']]
def observation(path):return dict(path=path.as_posix(),sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),utf16=len(str(path).encode('utf-16-le'))//2)
rows=list(map(observation,owned));refs=list(map(observation,references))
assert max(row['utf16'] for row in rows+refs)<=256
result=dict(owned=rows,references=refs,rootOwnedContext=[observation(ifc/'🚪️io/geometry.ts')],registrationCandidate=(ifc/'🚪️io/bim/properties/properties.test.ts').as_posix(),nativeExecuted=False,nativeTypechecked=False,semanticPublicationEnabled=False,capabilityEnabled=False)
(ticket/'🗑️generated/bim-properties-source.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owned=len(rows),references=len(refs),rootOwnedContext=1,maxUtf16=max(row['utf16'] for row in rows+refs))))
