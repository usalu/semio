"""🧪 Run final independent physical domains and record exact semantic input identity."""
import ast
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import time
import unittest

sys.stdout.reconfigure(encoding='utf-8',errors='backslashreplace')
sys.stderr.reconfigure(encoding='utf-8',errors='backslashreplace')

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
DOMAINS=('shared','🚶️circulation','📦️casing','🧱️panel','💊️detail','🪟️window','🧭️poses','📍️survey','🪟️unit','🔧️mounts','💡️light')

def fingerprints():
    """🔐 Canonical AST/JSON identity for every actual local model and oracle input."""
    result={}
    for path in sorted(BASE.rglob('*')):
        if path.suffix not in ('.py','.json','.cjs') or '__pycache__' in path.parts:
            continue
        text=path.read_text(encoding='utf8')
        canonical=ast.dump(ast.parse(text),include_attributes=False) if path.suffix=='.py' else json.dumps(json.loads(text),sort_keys=True,separators=(',',':'),ensure_ascii=False) if path.suffix=='.json' else text
        result[path.relative_to(ROOT).as_posix()]=hashlib.sha256(canonical.encode()).hexdigest()
    return result

before=fingerprints()
results=[]
for index,domain in enumerate(DOMAINS):
    path=BASE/'🧪️tests'/'🐍️.py' if domain=='shared' else BASE/'🧪️tests'/domain/'🐍️.py'
    print('[DEBUG] physical domain '+domain,flush=True)
    spec=importlib.util.spec_from_file_location('physical_domain_'+str(index),path)
    module=importlib.util.module_from_spec(spec)
    sys.modules[spec.name]=module
    spec.loader.exec_module(module)
    started=time.monotonic()
    result=unittest.TextTestRunner(stream=sys.stdout,verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(module))
    results.append({'domain':domain,'tests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'success':result.wasSuccessful(),'seconds':time.monotonic()-started})
after=fingerprints()
record={'source_before':before,'source_after':after,'unchanged':before==after,'domains':results,'success':before==after and all(row['success'] for row in results)}
out=Path(__file__).parent/'🗑️generated/physical-domains-final.json'
out.write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({'success':record['success'],'unchanged':record['unchanged'],'tests':sum(row['tests'] for row in results)}),flush=True)
raise SystemExit(0 if record['success'] else 1)
