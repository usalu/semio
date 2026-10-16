"""🧪️ Compares two ts_gate.py results: per file the failing test names that are new after the change, and the ones that were fixed. usage: python3 ts_diff.py before.json after.json"""
import json, sys
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
new = 0; fixed = 0
for f in sorted(set(a) | set(b)):
    x, y = a.get(f), b.get(f)
    if not x or not y:
        print('MISSING', f[-80:]); continue
    nf = sorted(set(y['fails']) - set(x['fails'])); ff = sorted(set(x['fails']) - set(y['fails']))
    cnt = (x['passed'], x['failed'], y['passed'], y['failed'])
    if nf or ff or cnt[:2] != cnt[2:]:
        print(f.split('🔌️plugin/')[-1][:95], 'before pass/fail', cnt[:2], 'after', cnt[2:], 'NEW:', nf, 'FIXED:', ff)
    new += len(nf); fixed += len(ff)
print('files', len(b), 'new failing tests', new, 'fixed', fixed, '| totals before pass/fail', sum(v['passed'] or 0 for v in a.values()), sum(v['failed'] or 0 for v in a.values()), 'after', sum(v['passed'] or 0 for v in b.values()), sum(v['failed'] or 0 for v in b.values()))
