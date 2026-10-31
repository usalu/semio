"""🛝️ Independently validate every saved guard cell before source publication."""
import importlib,json,math,sys,time
from collections import Counter
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';start=time.monotonic()
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');I=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔻️volume');result={'GUARD-RM-1':[],'GUARD-RM-2':[]};volume={k:0. for k in result}
for row in json.loads((out/'ramp-guards-prevalidation-r92.json').read_text(encoding='utf8')):
    tag,p=row['guard'],row['part']
    try:native=api._polyhedron_part(p['mesh']['vertices'],p['mesh']['faces'])
    except ValueError:
        (out/'ramp-guard-native-failure-r92.json').write_text(json.dumps(row,ensure_ascii=False),encoding='utf8');raise
    expected=I.independent_volume(p['mesh']['vertices']);actual=api._polyhedron_volume(native)
    if not (expected>0 and actual>0 and math.isclose(actual,expected,rel_tol=1e-7,abs_tol=1e-24)):
        (out/'ramp-guard-affine-failure-r92.json').write_text(json.dumps({'tag':tag,'part':p,'expected':expected,'actual':actual},ensure_ascii=False),encoding='utf8')
        raise AssertionError((tag,p['role'],actual,expected))
    edges=Counter(tuple(sorted((a,b))) for f in p['mesh']['faces'] for a,b in zip(f,f[1:]+f[:1]));assert set(edges.values())=={2}
    result[tag].append({k:p[k] for k in ('mesh','material','role')});volume[tag]+=actual
report={'guards':result,'volume':volume,'seconds':time.monotonic()-start,'qualification':['Published saved exact-construction geometry independently conditioned by an invertible exact affine map before SciPy convex integration','No source vertex, face or positive material discarded']}
(out/'ramp-guards-candidates-r92.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'parts':{k:len(v) for k,v in result.items()},'volume':volume,'seconds':report['seconds']}))
