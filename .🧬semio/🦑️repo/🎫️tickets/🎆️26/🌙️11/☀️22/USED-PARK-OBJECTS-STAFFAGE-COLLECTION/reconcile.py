import json, glob, os, subprocess, sys
"""🧭️ Republishes every used-park-objects asset whose catalog file exists but whose manifest row or collection membership is missing."""
K = os.path.dirname(os.path.abspath(__file__))
C = os.path.join(K, '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog')
slugs = [l.split('|')[2].strip() for l in open(os.path.join(K, 'assign.md')) if l.startswith('| ') and l[2:4].isdigit()]
m = json.load(open(os.path.join(C, '🔣️.json')))
ids = {e['id'] for e in m['entries']}
coll = next((c['assets'] for c in m['collections'] if c['id'] == 'used-park-objects'), [])
files = {os.path.basename(os.path.dirname(p)) for p in glob.glob(os.path.join(C, '*/used-*/🔣️.json'))}
todo = [s for s in slugs if s in files and (s not in ids or s not in coll) and os.path.exists(os.path.join(K, 'assets', s, 'distinction.json'))]
print('[DEBUG] missing', todo)
for s in todo:
    r = subprocess.run([sys.argv[1], 'publish.ts', s], cwd=K, capture_output=True, text=True)
    print(s, r.returncode, (r.stdout + r.stderr).strip().splitlines()[-1:] )
