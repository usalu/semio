# 🔬️ WP5 full-resolution differential: A = live engine tree with the Shell element replaced by the WP5 working copy (plus the overlay of other packages' consumer files), B = live engine tree.
# usage: oracle5.py <tag> [--configs lib,test,wasm,wasm-test]
import sys, os, re, json, collections, shutil, subprocess, time, importlib.util, concurrent.futures
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('rwp0_oracle', HERE + '/../rwp0-tools/oracle.py')
o = importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
SCR = o.SCR; ROOT = o.ROOT; ENG = o.ENG; PK = o.PK; GEN = HERE + '/../🗑️generated'
SHELL_REL = ENG + '/🧱️elements/🐚️Shell'

def run5(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-renderer-wgpu')
    t0 = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_os_renderer_wgpu', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/📚️library/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/rwp5-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - t0

def parse5(err, sb):
    c = collections.Counter()
    for line in err.split('\n'):
        line = line.strip()
        if not line.startswith('{'): continue
        try: d = json.loads(line)
        except Exception: continue
        if d.get('level') != 'error' or str(d.get('message', '')).startswith('aborting due to'): continue
        code = (d.get('code') or {}).get('code') or ''
        sp = next((s for s in d.get('spans', []) if s.get('is_primary')), None)
        file = label = text = ''
        if sp:
            file = os.path.normpath(sp.get('file_name', '').replace(sb + '/', '').replace(ROOT + '/', ''))
            if o.HUBFILE.match(file): file = '<renderer-root-or-hub>'
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:100]
        msg = str(d.get('message', '')).replace(sb + '/', '').replace(ROOT + '/', '')
        if '/🐚️Shell/' in file: file = '<shell>'
        c[(code, msg, text, label, file)] += 1
    return c

def main():
    a = sys.argv[1:]
    tag = a[0]; configs = ['lib']
    for i, x in enumerate(a):
        if x == '--configs': configs = a[i + 1].split(',')
    sbn = SCR + '/rwp5-sb-' + tag
    os.makedirs(sbn, exist_ok=True)
    A, B = sbn + '/A', sbn + '/B'
    only_a = '--only-a' in a
    with concurrent.futures.ThreadPoolExecutor(2) as ex: list(ex.map(o.chain, [A] if only_a else [A, B]))
    subprocess.check_call(['rsync', '-a', '--delete', '--exclude', 'node_modules', HERE + '/../🗑️generated/rwp5/new-shell/', A + '/' + SHELL_REL + '/'])
    ov = HERE + '/../🗑️generated/rwp5/new-ext'
    if os.path.isdir(ov): subprocess.check_call(['rsync', '-a', ov + '/', A + '/'])
    if '--seed' in a:
        pp = A + '/' + SHELL_REL + '/🎯️targets/🧊️wgpu/🦀️.rs'
        t = open(pp, encoding='utf-8').read()
        for line in ('pub use staged_window_actions__tutorial_runtime::*;\n', 'pub use staged_pointer__bundle::*;\n'):
            assert t.count(line) == 1, line
            t = t.replace(line, '')
        open(pp, 'w', encoding='utf-8').write(t)
    o.neutralize(A + '/' + ENG)
    if not only_a: o.neutralize(B + '/' + ENG)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        with concurrent.futures.ThreadPoolExecutor(2) as ex:
            fa = ex.submit(run5, A, 'A-' + name, o.CFGS[name], tag)
            if only_a:
                (ea, ta) = fa.result(); eb = open(f'{GEN}/rwp5-oracle-g4-B-{name}.jsonl').read(); tb = 0; B = SCR + '/rwp5-sb-g4/B'
            else:
                fb = ex.submit(run5, B, 'B-' + name, o.CFGS[name], tag); (ea, ta), (eb, tb) = fa.result(), fb.result()
        ca, cb = parse5(ea, A), parse5(eb, B)
        new, old = ca - cb, cb - ca
        summary[name] = dict(a=sum(ca.values()), b=sum(cb.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secA=round(ta), secB=round(tb))
        print(f'== {name}: A errors={sum(ca.values())} B errors={sum(cb.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:120]: print('  +new', v, k)
        for k, v in sorted(old.items())[:60]: print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/rwp5-oracle-{tag}-summary.json', 'w'), indent=1)

if __name__ == '__main__': main()
