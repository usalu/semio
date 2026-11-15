"""Author 62: elimination probe for silhouette p95 (removes body solids by id, prints p95 per view). Usage: python3 author-62-elim.py asset scale 'id1,id2' ['id3' ...]"""
import sys; sys.path.insert(0, '.')
from author_62_lib import *
A, sc = sys.argv[1], sys.argv[2]
d = load(A)
for grp in sys.argv[3:]:
    dd = copy.deepcopy(d)
    ids = set(grp.split(',')) if grp != '-' else set()
    b = dd['volumes'][sc]['body']
    dd['volumes'][sc]['body'] = [x for x in b if x['id'] not in ids and not any(x['id'].startswith(i[:-1]) for i in ids if i.endswith('*'))]
    p = scratch(A, dd, 'elim')
    out = run(['author-32-probe.ts', A, '--scale', sc, '--source', p, '--rules', 'volumes-silhouette', '--all'])
    print(grp[:60], re.findall(r'p95M:(\w+) = ([\d.]+)', out), re.findall(r'iou:(\w+) = ([\d.]+)', out))
