import sys, difflib, re
origin, current = sys.argv[1], sys.argv[2]
titles = sys.argv[3:]
def body(path, title):
    s = open(path, encoding='utf-8').read()
    i = s.find('test("' + title)
    if i < 0: return None
    depth = 0; j = s.index('{', s.index('=>', i))
    k = j
    while True:
        c = s[k]
        if c == '{': depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0: break
        k += 1
    return s[i:k+1].split('\n')
for t in titles:
    a, b = body(origin, t), body(current, t)
    print('#' * 10, t, 'origin' if a else 'NO-ORIGIN', 'current' if b else 'NO-CURRENT')
    if a and b:
        for line in difflib.unified_diff(a, b, 'origin', 'current', n=0, lineterm=''):
            print(line[:260])
