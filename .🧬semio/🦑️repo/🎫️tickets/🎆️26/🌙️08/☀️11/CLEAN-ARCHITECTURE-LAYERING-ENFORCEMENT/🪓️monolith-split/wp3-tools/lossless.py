"""♻️ Losslessness: the multiset of non-blank lines of every WP3-owned .rs/.ts file (and the lift files) before the first edit against now; prints what was lost and what is new."""
import os, collections, sys
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
ROOT = '/Users/ueli/Documents/semio'
def lines(paths):
    c = collections.Counter()
    for p in paths:
        try:
            for l in open(p, encoding='utf8').read().split('\n'):
                if l.strip(): c[l.rstrip()] += 1
        except Exception: pass
    return c
def walk(base):
    out = []
    for dp, dn, fn in os.walk(base):
        for f in fn:
            if f.endswith(('.rs', '.ts', '.tsx')): out.append(os.path.join(dp, f))
    return out
owned = [l for l in open(SCR + '/wp3-owned-paths.txt', encoding='utf8').read().split('\n') if l]
before_files, after_files = [], []
for rel in owned:
    b = os.path.join(SCR, 'wp3-orig', rel); a = os.path.join(ROOT, rel)
    before_files += walk(b) if os.path.isdir(b) else [b]
    after_files += walk(a) if os.path.isdir(a) else [a]
for rel in [l for l in open(os.path.dirname(os.path.abspath(__file__)) + '/created-outside.txt', encoding='utf8').read().split('\n') if l]:
    after_files += walk(os.path.join(ROOT, rel))
pass
V = '️'
E = ROOT + '/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime'
pass
B, A = lines(before_files), lines(after_files)
lost = B - A; new = A - B
print('before lines', sum(B.values()), 'after lines', sum(A.values()), 'lost', sum(lost.values()), 'new', sum(new.values()))
if '-v' in sys.argv:
    print('--- lost'); [print(v, k[:200]) for k, v in sorted(lost.items(), key=lambda x: -x[1])[:400]]
    print('--- new'); [print(v, k[:200]) for k, v in sorted(new.items(), key=lambda x: -x[1])[:400]]
