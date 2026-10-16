# 🚀 Applies an overlay {path: text} to the repository only if every target still equals the text it was built from (guarded, all-or-nothing), and records the touched files.
import json, os, sys, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
R = '/Users/ueli/Documents/semio'
TOUCHED = HERE + '/touched.txt'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.exists(p) else None
def apply(path, dry=False):
    d = json.load(open(path)); out = d['out']; hashes = d.get('hashes', {})
    for p in out:
        assert p.startswith(R + '/'), p
        if p in hashes: assert sha(p) == hashes[p], ('changed since build', p)
        elif os.path.exists(p): raise SystemExit(('no recorded hash for existing file', p))
    if dry: print('dry ok', len(out)); return
    for p, t in out.items():
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf8').write(t)
    seen = set(open(TOUCHED, encoding='utf8').read().split('\n')) if os.path.exists(TOUCHED) else set()
    with open(TOUCHED, 'a', encoding='utf8') as f:
        for p in out:
            rel = os.path.relpath(p, R)
            if rel not in seen: f.write(rel + '\n')
    print('applied', len(out))
if __name__ == '__main__': apply(sys.argv[1], dry=(len(sys.argv) > 2 and sys.argv[2] == 'dry'))
