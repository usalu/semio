import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import oracle4 as o4
tag, cfg = sys.argv[1], sys.argv[2]
base = o4.SCR + '/rwp4-sb-' + tag
ea = open(f'{o4.GEN}/rwp4-oracle-{tag}-A-{cfg}.jsonl').read(); eb = open(f'{o4.GEN}/rwp4-oracle-{tag}-B-{cfg}.jsonl').read()
a, b = o4.parse4(ea, base + '/A', True), o4.parse4(eb, base + '/B', True)
new, old = a - b, b - a
import re as _re
ART = _re.compile(r'(cannot find module or crate|unresolved imports?) `(semio_framework[a-z_]*|ui_wgpu|ui_gpu|serde|serde_json|ui_contract|replication)`')
art = {k: v for k, v in new.items() if ART.search(k[1])}
new = {k: v for k, v in new.items() if k not in art}
print('artifact duplicates of unresolved external imports (sandbox only):', sum(art.values()))
print(cfg, 'A', sum(a.values()), 'B', sum(b.values()), 'onlyNew', sum(new.values()), 'onlyOld', sum(old.values()))
for k, v in sorted(new.items(), key=lambda kv: -kv[1])[:int(sys.argv[3]) if len(sys.argv) > 3 else 40]: print('+new', v, k[0], k[1][:150], '|', k[2][:60], '|', k[4][-50:])
for k, v in sorted(old.items(), key=lambda kv: -kv[1])[:10]: print('-old', v, k[0], k[1][:150], '|', k[2][:60], '|', k[4][-50:])
