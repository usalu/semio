# 🧪️ WP4 TS readers against the candidate sandbox (A) and the baseline sandbox (B) of an oracle run: vitest (wgpu config) and bun test, per-test status compared.
import json, os, re, subprocess, sys, importlib.util
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('v', HERE + '/../rwp0-tools/ts_vitest.py'); v = importlib.util.module_from_spec(spec)
sys.path.insert(0, HERE + '/../rwp0-tools')
spec.loader.exec_module(v)
spec2 = importlib.util.spec_from_file_location('c', HERE + '/../rwp0-tools/ts_compare.py'); c = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(c)
o = v.o
GEN = HERE + '/../🗑️generated'
def main(tag):
    base = o.SCR + '/rwp4-sb-' + tag
    A, B = base + '/A', base + '/B'
    v.FILES.append('🧬️shell-component-decomposition')
    for sb in (A, B):
        cf = sb + '/' + v.CONFIG
        t = open(cf, encoding='utf-8').read()
        if 'shell-component-decomposition' not in t:
            i = t.index('include: ['); j = t.index(']', i)
            open(cf, 'w', encoding='utf-8').write(t[:j] + ', "🧪️tests/🧬️shell-component-decomposition/🟦️.ts"' + t[j:])
    ra, ea = v.run(A, 'w4-' + tag + '-A'); rb, eb = v.run(B, 'w4-' + tag + '-B')
    print('vitest A err', ea, 'B err', eb)
    same = 0
    for k in sorted(set(ra or {}) | set(rb or {})):
        if (ra or {}).get(k) == (rb or {}).get(k):
            same += 1
            st = (ra or {}).get(k) or {}
            print('SAME', k, 'pass', sum(1 for x in st.values() if x == 'passed'), 'fail', sum(1 for x in st.values() if x != 'passed'))
        else:
            print('DIFF', k, 'A', (ra or {}).get(k), 'B', (rb or {}).get(k))
    print('vitest files', len(set(ra or {}) | set(rb or {})), 'same', same)
    la, lb = f'{GEN}/rwp4-bun-{tag}-A.log', f'{GEN}/rwp4-bun-{tag}-B.log'
    open(la, 'w').close(); open(lb, 'w').close()
    for t in c.tests:
        a = c.run(A, t, la); b = c.run(B, t, lb)
        print(('SAME ' if a == b else 'DIFF ') + t, 'A', a, 'B', b, flush=True)
if __name__ == '__main__':
    main(sys.argv[1])
