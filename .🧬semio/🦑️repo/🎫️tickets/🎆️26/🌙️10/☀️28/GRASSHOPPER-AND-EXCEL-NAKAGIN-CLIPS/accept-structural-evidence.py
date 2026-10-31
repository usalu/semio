"""🏗️ Revalidate current structural proof inputs without rendering or rerunning geometry."""
import ast,hashlib,json
from pathlib import Path
T=Path(__file__).resolve().parent
R=next(p for p in T.parents if (p/'🧰️framework').is_dir())
record=json.loads((T/'🗑️generated/structural-final/result.json').read_text(encoding='utf8'))
expected={'📏️dimensions':13,'🗼️core':12,'🌉️bridge':7,'🏬️podium':10,'🏭️roof':10}
rows=record['domains']
valid=len(rows)==len(expected) and len({row['domain'] for row in rows})==len(expected) and {row['domain'] for row in rows}==set(expected) and all(row['tests']==expected[row['domain']] and row['success'] and row['failures']==row['errors']==0 for row in rows)
def fingerprint(path):
    """🔐 Reproduce the original structural semantic hash policy exactly."""
    text=path.read_text(encoding='utf8')
    value=ast.dump(ast.parse(text),include_attributes=False) if path.suffix=='.py' else json.dumps(json.loads(text),ensure_ascii=False,sort_keys=True,separators=(',',':'))
    return hashlib.sha256(value.encode()).hexdigest()
current={key:fingerprint(R/key) for key in record['source_fingerprints']}
stable=len(current)==16 and record['source_fingerprints']==record['source_fingerprints_end']==current
result={'success':valid and stable,'tests':sum(row['tests'] for row in rows),'complete_domain_contract':valid,'source_unchanged':stable,'source_before':record['source_fingerprints'],'source_after':record['source_fingerprints_end'],'source_current':current,'validator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
(T/'🗑️generated/structural-current-acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({key:result[key] for key in ('success','tests','complete_domain_contract','source_unchanged')}),flush=True)
raise SystemExit(0 if result['success'] else 1)
