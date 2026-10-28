"""🧭 Inventory unscored direct owners without treating syntactic branches as coverage."""
import hashlib
import inspect
import json
import re
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
exp=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/IFC4X3_ADD2.exp'
text=exp.read_text(encoding='utf-8')
remaining=json.loads((ticket/'🗑️generated/issued-express-arithmetic-runtime.json').read_text(encoding='utf-8'))['coverage']['remainingOwners']
functions={match.group(1).upper():match for match in re.finditer(r'^FUNCTION\s+(Ifc\w+)\b.*?^END_FUNCTION;',text,re.M|re.S)}
rows=[]
for owner in remaining:
    match=functions[owner];body=match.group(0);helper=next((getattr(rules,name) for name in dir(rules) if name.upper()==owner),None)
    rows.append(dict(owner=owner,firstLine=text.count('\n',0,match.start())+1,lastLine=text.count('\n',0,match.end())+1,bodySha256=hashlib.sha256(body.encode()).hexdigest(),syntacticTokens={word:len(re.findall(r'\b'+word+r'\b',body,re.I)) for word in ['IF','CASE','REPEAT','QUERY']},calledOwners=sorted({name.upper() for name in re.findall(r'\b(Ifc\w+)\s*\(',body) if name.upper() in functions and name.upper()!=owner}),helperAvailable=helper is not None,helperSha256=hashlib.sha256(inspect.getsource(helper).encode()).hexdigest() if helper else None,directBranchCoverageComplete=False))
result=dict(expSha256=hashlib.sha256(exp.read_bytes()).hexdigest(),ifcopenshell=ifcopenshell.version,directOwnerCount=34,remaining=rows,syntacticCountsAreCoverage=False,absenceClaim=False)
(ticket/'🗑️generated/issued-express-remaining-census.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(remaining=len(rows),helpers=sum(row['helperAvailable'] for row in rows))))
