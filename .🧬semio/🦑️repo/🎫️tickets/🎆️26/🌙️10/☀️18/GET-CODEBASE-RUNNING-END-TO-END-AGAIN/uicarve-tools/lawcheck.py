import os, re, sys
R = '/Users/ueli/Documents/semio'
roots = [R + '/🧰️framework/🔨️modules/🖱️ui', R + '/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer']
skip = {'node_modules', 'dist', 'target', '🗑️generated'}
sources = []; cases = []
for rt in roots:
    for dp, dn, fn in os.walk(rt):
        dn[:] = [d for d in dn if d not in skip]
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f); sources.append(p)
                par = os.path.basename(os.path.dirname(os.path.dirname(p))); nm = os.path.basename(dp)
                if par == '🧪️tests' and nm.startswith('🔬️') and 'wgpu' in nm and f == '🦀️.rs': cases.append(p)
mounts = []
for s in sources:
    try: t = open(s, encoding='utf8').read()
    except Exception: continue
    for line in t.split('\n'):
        if line.lstrip().startswith('//'): continue
        for m in re.finditer(r'(?:#\[path|include!)[^"\n]*"([^"]+\.rs)"', line):
            lit = m.group(1)
            d = os.path.dirname(s); stem = os.path.splitext(os.path.basename(s))[0]
            for cand in (os.path.join(d, lit), os.path.join(d, stem, lit)):
                if os.path.exists(cand): mounts.append((s, os.path.realpath(cand))); break
            else: mounts.append((s, None, lit))
bad = 0
for c in cases:
    k = [m for m in mounts if len(m) == 2 and m[1] == os.path.realpath(c)]
    if len(k) != 1: print('MOUNTED', len(k), c[-80:], [x[0][-50:] for x in k]); bad += 1
dang = [m for m in mounts if len(m) == 3]
for m in dang: print('DANGLING', m[0][-70:], m[2][-60:]); bad += 1
print('cases', len(cases), 'bad', bad)
