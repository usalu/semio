import json, sys
for p in sys.argv[1:]:
    lines = open(p, encoding='utf-8').read().split('\n')
    try:
        a = lines.index('{')
        b = max(i for i, l in enumerate(lines) if l == '}')
        d = json.loads('\n'.join(lines[a:b + 1]))
    except Exception as e:
        print(p.split('/')[-1], 'NO JSON', lines[-3:])
        continue
    ex = [l for l in lines if l.startswith('exit=')]
    L = d['layout']
    print('%-28s exit %-6s crates %2d baseCrates %d native %d/%d only %d | test %d/%d only %d | admission %d shared %d foreign %d healedBase %d' % (p.split('checkapplied-')[-1], ex[-1] if ex else '?', d['crates'], d['baseCrates'], L['native']['base'], L['native']['combined'], L['native']['onlyCombined'], L['--test']['base'], L['--test']['combined'], L['--test']['onlyCombined'], len(d['admission']), len(d['shared']), len(d['sharedForeign']), len(d.get('healedBase', []))))
