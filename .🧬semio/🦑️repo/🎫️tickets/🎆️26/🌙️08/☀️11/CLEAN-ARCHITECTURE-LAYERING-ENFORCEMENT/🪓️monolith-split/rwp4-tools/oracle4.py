# 🔬️ WP4 full-resolution differential for the Shell element: A = live tree with the Shell element replaced by <newtree>, B = live tree as is (or with the snapshot Shell).
# usage: oracle4.py <tag> <newtree> [--configs lib,test,wasm,wasm-test] [--baseline live|snapshot] [--fold-files]
import sys, os, re, json, collections, shutil, subprocess, time, importlib.util, concurrent.futures
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('rwp0_oracle', HERE + '/../rwp0-tools/oracle.py')
o = importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
SCR = o.SCR; ROOT = o.ROOT; ENG = o.ENG; PK = o.PK; GEN = HERE + '/../🗑️generated'
SHELL_REL = ENG + '/🧱️elements/🐚️Shell'
SHELL_TGT = SHELL_REL + '/🎯️targets/🧊️wgpu'

def run4(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-renderer-wgpu')
    t0 = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_os_renderer_wgpu', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/📚️library/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/rwp4-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - t0

def parse4(err, sb, fold):
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
        if fold: file = '<shell>' if '/🐚️Shell/' in file else file
        c[(code, msg, text, label, file)] += 1
    return c

def main():
    a = sys.argv[1:]
    tag, newtree = a[0], a[1]
    configs = ['lib']; base = 'live'; overlay = None
    for i, x in enumerate(a):
        if x == '--configs': configs = a[i + 1].split(',')
        if x == '--baseline': base = a[i + 1]
        if x == '--overlay': overlay = a[i + 1]
    fold = True
    sbn = SCR + '/rwp4-sb-' + tag
    os.makedirs(sbn, exist_ok=True)
    A, B = sbn + '/A', sbn + '/B'
    with concurrent.futures.ThreadPoolExecutor(2) as ex:
        list(ex.map(o.chain, [A, B]))
    subprocess.check_call(['rsync', '-a', '--delete', '--exclude', 'node_modules', newtree + '/', A + '/' + SHELL_TGT + '/'])
    if overlay:
        subprocess.check_call(['rsync', '-a', overlay + '/', A + '/'])
    hub = A + '/' + ENG + '/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs'
    ht = open(hub, encoding='utf-8').read()
    ext = 'extern crate semio_framework_os_renderer_wgpu_shell_vocab as shell_vocab;'
    if ext in ht:
        open(hub, 'w', encoding='utf-8').write(ht.replace(ext, '#[path = "../../../📖️shell-vocab/🦀️.rs"]\nmod shell_vocab;'))
        pp = A + '/' + SHELL_TGT + '/🦀️.rs'
        pt = open(pp, encoding='utf-8').read()
        open(pp, 'w', encoding='utf-8').write(pt.replace('pub(crate) use shell_vocab::shell_chrome_string;', 'pub(crate) use crate::shell_vocab::shell_chrome_string;'))
    if base == 'snapshot':
        st = SCR + '/apply-state'
        if os.path.isdir(st + '/ext-orig'): subprocess.check_call(['rsync', '-a', st + '/ext-orig/', B + '/'])
        shutil.copyfile(st + '/hub-orig.rs', B + '/' + ENG + '/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs')
        shutil.copyfile(st + '/root-orig.rs', B + '/' + ENG + '/🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs')
        subprocess.check_call(['rsync', '-a', '--delete', '--exclude', 'node_modules', SCR + '/orig-shell/', B + '/' + SHELL_REL + '/'])
    o.neutralize(A + '/' + ENG); o.neutralize(B + '/' + ENG)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        with concurrent.futures.ThreadPoolExecutor(2) as ex:
            fa = ex.submit(run4, A, 'A-' + name, o.CFGS[name], tag)
            fb = ex.submit(run4, B, 'B-' + name, o.CFGS[name], tag)
            (ea, ta), (eb, tb) = fa.result(), fb.result()
        ca, cb = parse4(ea, A, fold), parse4(eb, B, fold)
        new, old = ca - cb, cb - ca
        summary[name] = dict(a=sum(ca.values()), b=sum(cb.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secA=round(ta), secB=round(tb))
        print(f'== {name}: A errors={sum(ca.values())} B errors={sum(cb.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:80]: print('  +new', v, k)
        for k, v in sorted(old.items())[:80]: print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/rwp4-oracle-{tag}-summary.json', 'w'), indent=1)

if __name__ == '__main__': main()
