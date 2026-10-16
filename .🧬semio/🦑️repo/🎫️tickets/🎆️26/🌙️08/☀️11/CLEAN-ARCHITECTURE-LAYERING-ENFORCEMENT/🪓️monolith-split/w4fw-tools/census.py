import sys, json, subprocess, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cfg import *
def run(R, out, extra_rows=()):
    spec = [dict(id=c['id'], root=c['root'], dir=c['node'] + '/📦️packages/🦀️rust') for c in CLUSTERS.values()] + list(extra_rows)
    sp = out + '.spec.json'
    json.dump(spec, open(sp, 'w'), ensure_ascii=False)
    subprocess.check_call(['bun', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'closure.ts'), R.rstrip('/'), sp, out])
    return json.load(open(out))
if __name__ == '__main__':
    R = sys.argv[1]; res = run(R, sys.argv[2])
    for r in res:
        if 'error' in r: print(r['id'], r['cfg'], 'ERROR', r['error'][:150])
        else: print(f"{r['id']:34s} {r['cfg']:4s} files {r['files']:3d} bytes {r['bytes']:7d} lines {r['lines']:5d} {'OVER' if (r['files']>32 or r['bytes']>262144 or r['lines']>4000) else ('>target' if (r['files']>28 or r['bytes']>230000 or r['lines']>3500) else '')}")
