"""📊 Census of the WP8 crates through the canonical source counter (closure.ts)."""
import sys, os, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, common as C
FE = '️'
CLOSURE = L.HERE + '/../okwp0-tools/closure.ts'
def run(pkg, entry, features='', tag='x'):
    tmp = L.GEN + '/okwp8-census-%s.json' % tag
    env = dict(os.environ, PKG=pkg, ENTRY=entry, FEATURES=features, LIST=tmp)
    r = subprocess.run(['bun', CLOSURE], env=env, capture_output=True, text=True, cwd=L.R)
    rows = {}
    for line in r.stdout.strip().split('\n'):
        p = line.split('\t')
        if len(p) >= 9: rows[p[0]] = dict(files=int(p[2]), bytes=int(p[4]), lines=int(p[6]), embedded=int(p[8]))
    return rows, tmp
def crates():
    out = [('os-store-testing-fixtures', L.R + '/' + L.S_REL + '/🧪' + FE + 'testing/📦' + FE + 'packages/🦀' + FE + 'rust', L.R + '/' + L.S_REL + '/🧪' + FE + 'testing/🦀' + FE + '.rs', '')]
    for bk, (name, node) in C.BINS.items():
        out.append((name, C.pkg_abs(bk), C.node_abs(bk) + '/🦀' + FE + '.rs', ''))
    b = L.R + '/' + L.S_REL + '/🧪' + FE + 'testing/🎭' + FE + 'doubles'
    out.append(('os-store-testing-doubles', b + '/📦' + FE + 'packages/🦀' + FE + 'rust', b + '/🦀' + FE + '.rs', ''))
    out.append(('os-kernel', L.R + '/' + L.K_REL, L.R + '/' + L.K_REL + '/🦀' + FE + '.rs', 'deflate,sync,ureq,worker'))
    return out
if __name__ == '__main__':
    res = {}
    for name, pkg, entry, feats in crates():
        rows, tmp = run(pkg, entry, feats, name)
        res[name] = rows
        t = rows.get('test native', {}); l = rows.get('lib native', {}); w = rows.get('test wasm32-unknown-unknown', {})
        def f(r): return '%d/%d/%d' % (r.get('files', 0), r.get('bytes', 0), r.get('lines', 0)) if r else '-'
        flag = ''
        for r in (l, t, w):
            if r and (r['files'] > 28 or r['bytes'] > 230000 or r['lines'] > 3500): flag = ' OVER'
        print('%-34s lib %-22s test %-22s wasm-test %-22s%s' % (name, f(l), f(t), f(w), flag))
    json.dump(res, open(L.GEN + '/okwp8-census.json', 'w'), indent=1)
