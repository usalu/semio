# 🔖 Balances the //#region markers of newly lifted staging files: an `//#endregion X` whose region is not open is dropped, regions still open at the end are closed. usage: regions.py <dump.json> dry|write
import json, os, re, sys
MARK = re.compile(r'^(\s*)//#(end)?region\b\s*(.*)$')
d = json.load(open(sys.argv[1])); write = sys.argv[2] == 'write'
tot = 0
for st in d['fold']:
    if not os.path.exists(st): continue
    lines = open(st, encoding='utf8').read().split('\n')
    out = []; stack = []; dropped = 0
    for l in lines:
        m = MARK.match(l)
        if not m: out.append(l); continue
        name = m.group(3).strip()
        if not m.group(2): stack.append((name, m.group(1))); out.append(l); continue
        names = [n for n, _ in stack]
        if name in names:
            while stack and stack[-1][0] != name:
                n, ind = stack.pop(); out.append(f'{ind}//#endregion {n}')
            stack.pop(); out.append(l)
        else: dropped += 1
    added = len(stack)
    while stack:
        n, ind = stack.pop(); out.append(f'{ind}//#endregion {n}')
    if added or dropped:
        tot += 1
        print(('fix ' if write else 'would fix ') + st.split('modules/')[-1][:80], 'dropped', dropped, 'closed', added)
        if write:
            if out and out[-1] != '': out.append('')
            open(st, 'w', encoding='utf8').write('\n'.join(out))
print(tot, 'files')
