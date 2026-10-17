import sys, re
items = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:use|mod|fn|const|static|struct|enum|trait|impl|type|extern|macro_rules!|unsafe|async)\b|^#\[')
bad = []
n = 0
for line in open(sys.argv[1], encoding='utf8'):
    path = line.rstrip('\n')
    try:
        text = open(path, encoding='utf8').read()
    except Exception:
        continue
    n += 1
    seen_item = False
    in_block = False
    for i, l in enumerate(text.split('\n')[:400], 1):
        s = l.strip()
        if in_block:
            if '*/' in s: in_block = False
            continue
        if s.startswith('/*'):
            if '*/' not in s: in_block = True
            continue
        if not s or s.startswith('//'):
            continue
        if s.startswith('#!['):
            if seen_item:
                bad.append((path, i))
                break
            continue
        if items.match(s):
            seen_item = True
        elif seen_item:
            continue
print('checked', n, 'bad', len(bad))
for p, i in bad:
    print(i, p)
