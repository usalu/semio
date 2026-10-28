from pathlib import Path
import importlib.util,json,collections
p=Path(__file__).parent
spec=importlib.util.spec_from_file_location('oracle',p/'issued-express-surface-curve-oracle.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
g=m.GeneratedSurfaceCurveOracle();s=m.SqliteSurfaceCurveOracle()
a=[g.observe(c) for c in m.data['functions']];b=[s.observe(c) for c in m.data['functions']]
r={'functions':a,'relational':b,'summary':{'helpers':dict(collections.Counter(x['status'] for x in a)),'sqlite':dict(collections.Counter(x['status'] for x in b))}}
(p/'🗑️generated/issued-surface-curve-independent-oracle.json').write_text(json.dumps(r,indent=2,default=str)+'\n',encoding='utf8');print(r['summary'])
