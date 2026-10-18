"""📋 Inserts paths into a byte-sorted JSON string list (line-oriented, keeps the file's formatting). usage: addlist.py <file> <line-inside-list (1-based)> <path>... ; asserts the list is byte-sorted before and after and no duplicates."""
import sys, re, json
f, ln = sys.argv[1], int(sys.argv[2])
new = sys.argv[3:]
lines = open(f, encoding='utf-8').read().split('\n')
i = ln - 1
while not lines[i].rstrip().endswith('['):
    i -= 1
j = i + 1
while not lines[j].strip().startswith(']'):
    j += 1
body = lines[i + 1:j]
items = []
for l in body:
    m = re.match(r'^(\s*)("(?:[^"\\]|\\.)*")(,?)\s*$', l)
    assert m, ('unexpected list line', l[:100])
    items.append((m.group(1), json.loads(m.group(2))))
indent = items[0][0]
vals = [v for _, v in items]
key = lambda s: s.encode('utf-8')
assert vals == sorted(vals, key=key), 'list not byte sorted before'
for n in new:
    assert n not in vals, ('already present', n)
allv = sorted(vals + new, key=key)
out = [indent + json.dumps(v, ensure_ascii=False) + (',' if k < len(allv) - 1 else '') for k, v in enumerate(allv)]
lines[i + 1:j] = out
open(f, 'w', encoding='utf-8').write('\n'.join(lines))
print('list opened at', i + 1, 'items', len(vals), '->', len(allv))
