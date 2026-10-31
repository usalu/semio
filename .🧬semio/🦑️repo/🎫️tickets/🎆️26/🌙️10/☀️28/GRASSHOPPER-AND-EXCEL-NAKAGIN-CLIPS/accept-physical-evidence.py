"""🔐 Verify exact physical gate inputs while retaining unrelated rendering drift as evidence."""
import ast
import hashlib
import json
from pathlib import Path

TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
record=json.loads((TICKET/'🗑️generated/physical-domains-final.json').read_text(encoding='utf8'))
for endpoint in ('source_before','source_after'):
    record[endpoint]={key.replace('\\','/'):value for key,value in record[endpoint].items()}
contract=json.loads((TICKET/'physical-acceptance-cases.json').read_text(encoding='utf8'))

def valid_domains(rows, expected):
    """🧪 Require every expected physical domain exactly once, with its exact successful test count."""
    if not isinstance(rows,list) or len(rows)!=len(expected) or any(not isinstance(row,dict) for row in rows):
        return False
    names=[row.get('domain') for row in rows]
    if any(not isinstance(name,str) for name in names) or len(set(names))!=len(names) or set(names)!=set(expected):
        return False
    return all(type(row.get('tests')) is int and row['tests']==expected[row['domain']] and type(row.get('failures')) is int and row['failures']==0 and type(row.get('errors')) is int and row['errors']==0 and row.get('success') is True for row in rows)


def physical(path):
    """📦 Keep all model, schema and physical oracles; rendering has its own independent gate."""
    return path not in contract['unconsumed_paths']

def fingerprint(path):
    """🧬 Reproduce the original semantic endpoint hashing exactly."""
    text=path.read_text(encoding='utf8')
    canonical=ast.dump(ast.parse(text),include_attributes=False) if path.suffix=='.py' else json.dumps(json.loads(text),sort_keys=True,separators=(',',':'),ensure_ascii=False) if path.suffix=='.json' else text
    return hashlib.sha256(canonical.encode()).hexdigest()

before={k:v for k,v in record['source_before'].items() if physical(k)}
after={k:v for k,v in record['source_after'].items() if physical(k)}
current_all={k:fingerprint(ROOT/k) for k in record['source_after']}
current={k:v for k,v in current_all.items() if physical(k)}
changed=[k for k in set(record['source_before'])|set(record['source_after']) if record['source_before'].get(k)!=record['source_after'].get(k)]
valid=valid_domains(record.get('domains'),contract['expected_domains'])
success=valid and before==after==current
result={'success':success,'tests':sum(row['tests'] for row in record['domains']),'broad_base_unchanged':record['unchanged'],'physical_dependencies_unchanged':before==after,'physical_dependencies_current':after==current,'source_before':before,'source_after':after,'source_current':current,'unconsumed_rendering_changes':changed,'exact_unconsumed_paths':contract['unconsumed_paths'],'drift_during_run':changed,'drift_after_run':[k for k in current_all if current_all[k]!=record['source_after'][k]],'complete_domain_contract':valid,'domains':record['domains']}
result['physical_map_sha256']=hashlib.sha256(json.dumps(current,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
(TICKET/'🗑️generated/physical-final-acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({k:result[k] for k in ('success','tests','broad_base_unchanged','physical_dependencies_unchanged','physical_dependencies_current','physical_map_sha256')}),flush=True)
raise SystemExit(0 if success else 1)
