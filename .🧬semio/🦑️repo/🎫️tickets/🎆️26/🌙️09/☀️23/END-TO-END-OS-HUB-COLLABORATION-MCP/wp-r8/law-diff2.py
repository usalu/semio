import sys, difflib, re
pre, cur = sys.argv[1], sys.argv[2]
def body(path, title):
    s = open(path, encoding='utf-8').read()
    m = re.search(r'^\s*test(?:\.if\(testLevelAtLeast\("[a-z]+"\)\))?\("' + re.escape(title), s, re.M)
    if not m: return None
    i = m.start(); j = s.index('{', s.index('=>', m.end())); d = 0; k = j
    while k < len(s):
        c = s[k]
        if c == '{': d += 1
        elif c == '}':
            d -= 1
            if d == 0: break
        k += 1
    return s[i:k+1].split('\n')
for t in sys.argv[3:]:
    a, b = body(pre, t), body(cur, t)
    print('#' * 8, t, bool(a), bool(b))
    if a and b:
        for l in difflib.unified_diff(a, b, 'pre', 'cur', n=0, lineterm=''): print(l[:300])
