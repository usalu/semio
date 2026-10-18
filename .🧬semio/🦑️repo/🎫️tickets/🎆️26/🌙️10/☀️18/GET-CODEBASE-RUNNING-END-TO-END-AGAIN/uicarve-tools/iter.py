"""🔁 uicarve iter: run the oracle, widen what privacy errors demand, repeat. usage: iter.py <family.json> <tag> <cfgs> [max]"""
import json, os, re, subprocess, sys
fam, tag, cfgs = sys.argv[1], sys.argv[2], sys.argv[3]
mx = int(sys.argv[4]) if len(sys.argv) > 4 else 6
G = '../🗑️generated'
name = json.load(open(fam))['name']
env = dict(os.environ, POS_PREFIX='uic')
for i in range(mx):
    label = '%s%d' % (tag, i)
    r = subprocess.run(['python3', 'posrun.py', 'run', fam, '--cfgs', cfgs, '--label', label], env=env, capture_output=True, text=True)
    open('%s/uic-%s-%s.log' % (G, name, label), 'w').write(r.stdout + r.stderr)
    errs = ['%s/uic-uic-%s-%s-cur-%s-priv.err' % (G, name.replace('uic-', ''), label, c) for c in cfgs.split(',')]
    errs = [e for e in errs if os.path.exists(e)]
    summ = [l for l in r.stdout.split('\n') if l.startswith('-- cfg') or re.match(r'\s+(privacy|resolve|type|other|trait-scope):', l)]
    print('\n'.join(summ))
    p = subprocess.run(['python3', 'pos-tools/pos2widen.py', 'plan', fam, 'cur', label] + errs, env=env, capture_output=True, text=True)
    print(p.stdout.split('\n')[0])
    m = re.match(r'(\d+) planned', p.stdout)
    if not m or int(m.group(1)) == 0: break
    a = subprocess.run(['python3', 'pos-tools/pos2widen.py', 'apply', '%s/uic-%s-widen-plan.json' % (G, label)], env=env, capture_output=True, text=True)
    print(a.stdout.split('\n')[0])
