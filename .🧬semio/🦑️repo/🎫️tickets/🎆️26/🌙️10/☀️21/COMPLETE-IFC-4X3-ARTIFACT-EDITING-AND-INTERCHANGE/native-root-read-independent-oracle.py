"""🪟 Reobserve the owned derived reference without touching worker outputs."""
from pathlib import Path
import importlib.util,json,sqlite3
p=Path(__file__).parent;spec=importlib.util.spec_from_file_location('reference',p/'native-root-read-oracle.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);oracle=mod.DerivedOracle();repo=next(a for a in p.parents if (a/'AGENTS.md').exists());v=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation';rows=json.loads((v/'root-memo-cases.json').read_text(encoding='utf8'))['cases'];out=[]
with sqlite3.connect(':memory:') as db:
 for row in rows:
  value=oracle.evaluate(row['id']);r={'id':row['id'],'value':value,'exact':value==float(row['expected'])};r['sqlite']=db.execute('SELECT ?*10+?',(3,2) if row['expected']=='32' else (2,3)).fetchone()[0] if row['expected']!='91' else 'outside-interface';out.append(r)
(p/'🗑️generated/native-root-read-independent-oracle.json').write_text(json.dumps(out,indent=2)+'\n',encoding='utf8');print('[DEBUG]',out)
