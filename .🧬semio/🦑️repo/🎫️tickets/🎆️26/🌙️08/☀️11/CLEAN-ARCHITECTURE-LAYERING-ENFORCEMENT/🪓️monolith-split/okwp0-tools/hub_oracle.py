# 🔬️ Hub differential: A = closure files with the hubbed root, B = same files with the original root. usage: hub_oracle.py <tag> [configs] [--seed]
import json, os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okoracle as O, hubsplit as H

def main():
    tag = sys.argv[1]
    configs = sys.argv[2].split(',') if len(sys.argv) > 2 and not sys.argv[2].startswith('--') else ['lib', 'test', 'wasm', 'wasm-test', 'wasip2']
    files = json.load(open(O.SCR + '/orig-paths.json'))
    orig_root = open(O.SCR + '/orig/' + O.K_REL + '/🦀️.rs', encoding='utf-8').read()
    new_root, hubs = H.forward(orig_root)
    base = O.SCR + '/ok-sb-' + tag
    shutil.rmtree(base, ignore_errors=True)
    A, B = base + '/A', base + '/B'
    for sb in (A, B):
        os.makedirs(sb); O.copy_files(sb, files); os.makedirs(sb + '/outdir')
    rootA, rootB = A + '/' + O.K_REL + '/🦀️.rs', B + '/' + O.K_REL + '/🦀️.rs'
    if '--inline' in sys.argv:
        import re
        un = re.compile(r'(#\[\s*path\s*=\s*"|include!\(\s*")\.\./\.\./')
        t = new_root
        for n, h in hubs.items():
            body = '\n'.join(l for l in h.split('\n') if not l.startswith('// ' + H.HUBS[n][1] + ' HUB') and not l.startswith('// This file') and not l.startswith('// its `#[path]`') and not l.startswith('// Carving a crate'))
            body = un.sub(lambda m: m.group(1), body)
            inc = 'include!("📥️mounts/' + H.HUBS[n][0] + '/🦀️.rs");'
            assert t.count(inc) == 1
            t = t.replace(inc, body)
        open(rootB, 'w', encoding='utf-8').write(t)
    else: open(rootB, 'w', encoding='utf-8').write(orig_root)
    open(rootA, 'w', encoding='utf-8').write(new_root)
    for n, h in hubs.items():
        d = A + '/' + O.K_REL + '/📥️mounts/' + H.HUBS[n][0]
        os.makedirs(d); open(d + '/🦀️.rs', 'w', encoding='utf-8').write(h)
    if '--seed' in sys.argv:
        p = A + '/' + O.K_REL + '/📥️mounts/' + H.HUBS[1][0] + '/🦀️.rs'
        s = open(p, encoding='utf-8').read()
        a = 'pub mod list_delta;'
        assert s.count(a) == 1
        s = s.replace(a, 'pub mod list_delta_absent;')
        s = s.replace('pub use crate::os_dsl::grammar', 'pub use crate::os_dsl::grammar')
        open(p, 'w', encoding='utf-8').write(s)
        p2 = A + '/' + O.K_REL + '/🦀️.rs'
        s = open(p2, encoding='utf-8').read()
        a = 'pub use crate::os_pack::*;\n'
        assert s.count(a) == 1
        open(p2, 'w', encoding='utf-8').write(s.replace(a, ''))
        print('seeded: list_delta mount renamed, os_pack glob removed')
    O.neutralize(A); O.neutralize(B)
    fold = lambda f: '<kernel-root-or-hub>' if f.startswith(O.K_REL + '/🦀️.rs') or f.startswith(O.K_REL + '/📥️mounts/') else f
    summary = {}
    for name in configs:
        ea, ta = O.run(A, 'A', name, tag); eb, tb = O.run(B, 'B', name, tag)
        a, b = O.parse(ea, A, fold), O.parse(eb, B, fold)
        new, old = O.diff(a, b)
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print(f'== {name}: A(hubbed) errors={sum(a.values())} B(original) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:30]: print('  +new', v, k)
        for k, v in sorted(old.items())[:30]: print('  -old', v, k)
    json.dump(summary, open(f'{O.GEN}/okwp0-oracle-{tag}-summary.json', 'w'), indent=1)

main()
