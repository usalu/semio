"""⚡ Compare exact sparse-bound candidates with the original dense scalar predicate."""
import ast
import hashlib
import importlib
import inspect
import json
from pathlib import Path
import time
from types import SimpleNamespace
import numpy as np
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
CASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/🔣️.json'
def identity():
    """🔐 Bind the actual candidate helper, portable controls, benchmark and numerical runtime."""
    return {'helper':hashlib.sha256(ast.dump(ast.parse(inspect.getsource(V._overlap_candidates)),include_attributes=False).encode()).hexdigest(),'case':hashlib.sha256(CASE.read_bytes()).hexdigest(),'benchmark':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'numpy':np.__version__}
def dense(first,second,strict,margin):
    """🔬 Evaluate the original ordered scalar bound rejection without changing its arithmetic."""
    for i,a in enumerate(first):
        for j,b in enumerate(second):
            reject=((a.upper<=b.lower+margin).any() or (b.upper<=a.lower+margin).any()) if strict else ((a.upper<b.lower).any() or (b.upper<a.lower).any())
            if not reject:
                yield i,j
before=identity()
case=json.loads(CASE.read_text(encoding='utf-8'))['overlap_candidates']
rows=[]
for fixture in case['cases']:
    bounds=lambda values:[SimpleNamespace(lower=np.asarray(box[:2]),upper=np.asarray(box[2:])) for box in values]
    first,second=bounds(fixture['first']),bounds(fixture['second'])
    for strict in (False,True):
        margin=case['margin'] if strict else 0
        started=time.perf_counter()
        expected=list(dense(first,second,strict,margin))
        dense_seconds=time.perf_counter()-started
        started=time.perf_counter()
        actual=list(V._overlap_candidates(first,second,strict=strict,margin=margin))
        sparse_seconds=time.perf_counter()-started
        if actual!=expected or [list(pair) for pair in actual]!=fixture['strict' if strict else 'inclusive']:
            raise AssertionError('Exact ordered candidate difference: '+fixture['name'])
        rows.append({'case':fixture['name'],'strict':strict,'dense_pairs':len(first)*len(second),'actual_candidates':len(actual),'dense_seconds':dense_seconds,'sparse_seconds':sparse_seconds})
after=identity()
result={'before':before,'after':after,'unchanged':before==after,'rows':rows}
(TICKET/'🗑️generated/overlap-candidate-benchmark.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(result,ensure_ascii=False),flush=True)
raise SystemExit(0 if before==after else 1)
