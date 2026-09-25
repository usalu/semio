import re, sys, difflib
pre, cur = sys.argv[1], sys.argv[2]
title_re = re.compile(r'^\s*(?:test|it)(?:\.if\([^)]*\)\))?\(\s*"([^"]+)"', re.M)
title_re2 = re.compile(r'^\s*test(?:\.if\(testLevelAtLeast\("[a-z]+"\)\))?\("([^"]+)"', re.M)
def bodies(path):
    s = open(path, encoding='utf-8').read()
    out = {}
    for m in title_re2.finditer(s):
        t = m.group(1); i = m.start()
        try:
            j = s.index('{', s.index('=>', m.end())); d = 0; k = j
            while k < len(s):
                c = s[k]
                if c == '{': d += 1
                elif c == '}':
                    d -= 1
                    if d == 0: break
                k += 1
        except ValueError:
            continue
        out.setdefault(t, s[i:k+1].split('\n'))
    return out
A, B = bodies(pre), bodies(cur)
sig = re.compile(r'"(?:🧪️[^"/]+/[^"]*|[^"]*component[^"]*)"')
changed = 0
for t in sorted(set(A) & set(B)):
    a, b = A[t], B[t]
    if a == b: continue
    hunks = [l for l in difflib.unified_diff(a, b, n=0, lineterm='') if l[:1] in '+-' and not l.startswith(('+++', '---'))]
    minus = [l for l in hunks if l.startswith('-') and sig.search(l)]
    if minus:
        changed += 1
        print('#####', t, len(minus))
print('laws with signature changes:', changed, 'of common', len(set(A) & set(B)))
