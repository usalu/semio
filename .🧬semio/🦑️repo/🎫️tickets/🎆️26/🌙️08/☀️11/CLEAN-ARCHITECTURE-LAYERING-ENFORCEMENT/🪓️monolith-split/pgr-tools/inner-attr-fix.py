import sys, re, difflib
ITEM = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:mod|fn|const|static|struct|enum|trait|impl|type|macro_rules!|unsafe|async)\b|^#\[(?:path|derive|cfg_attr|macro_use)')
LEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:use|extern|mod|fn|const|static|struct|enum|trait|impl|type|macro_rules!|unsafe|async)\b|^#\[')
def fix(text):
    lines = text.split('\n')
    n = len(lines)
    limit = min(n, 400)
    first = None
    in_block = False
    for i in range(n):
        s = lines[i].strip()
        if in_block:
            if '*/' in s: in_block = False
            continue
        if s.startswith('/*'):
            if '*/' not in s: in_block = True
            continue
        if not s or (s.startswith('//') and not s.startswith('///')) or s.startswith('#!['):
            continue
        first = i
        break
    if first is None:
        return text, 0
    lift = []
    stop = None
    for i in range(first, n):
        l = lines[i]
        if ITEM.match(l) and stop is None:
            stop = i
        if re.match(r'^#!\[.*\]\s*$', l) and i < limit:
            lift.append(i)
        elif l.startswith('//!') and (stop is None):
            lift.append(i)
    if not lift:
        return text, 0
    moved = [lines[i] for i in lift]
    rest = [l for i, l in enumerate(lines) if i not in set(lift)]
    out = rest[:first] + moved + rest[first:]
    return '\n'.join(out), len(lift)
apply = sys.argv[1] == 'apply'
for p in sys.argv[2:]:
    text = open(p, encoding='utf8').read()
    new, k = fix(text)
    if k == 0:
        print('NOCHANGE', p); continue
    print('LIFT', k, p)
    if apply:
        open(p, 'w', encoding='utf8').write(new)
    else:
        for l in difflib.unified_diff(text.split('\n')[:40], new.split('\n')[:40], lineterm='', n=0):
            print('   ', l[:150])
