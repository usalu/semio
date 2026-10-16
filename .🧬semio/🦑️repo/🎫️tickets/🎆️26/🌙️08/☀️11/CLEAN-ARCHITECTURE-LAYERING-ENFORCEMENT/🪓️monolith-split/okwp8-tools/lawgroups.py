"""🧩 Lists every exact-law group addressing the kernel lib: file, line, laws, and the companion crate each law now lives in."""
import sys, os, re, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, testnames
census = json.load(open(L.GEN + '/okwp8-census.json'))
name2crate = collections.defaultdict(list)
seen = set()
for crate in census:
    if crate == 'os-kernel': continue
    d = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
    for r in d['test native']:
        if r['kind'] != 'embedded' and r['path'].endswith('.rs'):
            p = L.R + '/' + r['path']
            if p in seen: continue
            seen.add(p)
            for n in testnames.names(open(p, encoding='utf8').read()): name2crate[n].append(crate)
def groups(t):
    out = []
    for m in re.finditer(r'package:\s*"semio-framework-os-kernel"', t):
        s = t.rfind('{', 0, m.start())
        d = 0; e = s
        while e < len(t):
            if t[e] == '{': d += 1
            elif t[e] == '}':
                d -= 1
                if d == 0: break
            e += 1
        out.append((s, e + 1))
    return out
if __name__ == '__main__':
    files = subprocess.run(['rg', '-l', '-F', 'package: "semio-framework-os-kernel"', '--glob', '*.ts', '--glob', '!node_modules', '--glob', '!.🧬semio', L.R], capture_output=True, text=True).stdout.split('\n')
    for f in files:
        if not f: continue
        t = open(f, encoding='utf8').read()
        for s, e in groups(t):
            g = t[s:e]
            line = t.count('\n', 0, s) + 1
            laws = re.findall(r'"([A-Za-z_][A-Za-z0-9_:]*)"', g)
            laws = [x for x in laws if x not in ('semio-framework-os-kernel',) and x not in ('lib', 'sync')]
            print('###', f.replace(L.R + '/', '')[-80:], line, 'nlaws', len(laws), 'varlaws' if 'laws }' in g or 'laws:' not in g else '')
            for x in laws[:40]:
                last = x.split('::')[-1]
                cs = name2crate.get(last)
                print('    ', x[:100], cs if cs else 'KERNEL?')
