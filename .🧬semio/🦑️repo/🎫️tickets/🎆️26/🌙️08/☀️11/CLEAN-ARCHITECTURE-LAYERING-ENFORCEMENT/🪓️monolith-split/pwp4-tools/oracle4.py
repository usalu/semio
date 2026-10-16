# 🔬️ WP4 one-unit differential: A = live plugin tree + overlay dir(s); B = live plugin tree (+ optional baseline overlay). usage: oracle4.py TAG --a DIR [--b DIR] [--configs lib,test,...] [--seed NAME]
import sys, os, re, shutil, json, collections, subprocess, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../pwp0-tools')
import oracle as o
from common import *
SCRP = SCR
PK = o.PK; PLUGREL = o.PLUG
def chain(sb):
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb + '/' + PLUGREL)
    real = o.ROOT + '/' + PLUGREL
    ex = []
    for e in o.EXCLUDE + ['dist', '*.rmeta']: ex += ['--exclude', e]
    r = subprocess.run(['rsync', '-a', *ex, real + '/', sb + '/' + PLUGREL + '/'], capture_output=True, text=True)
    if r.returncode not in (0, 23, 24): raise RuntimeError(r.stderr[:500])
    os.makedirs(sb + '/' + PK, exist_ok=True)
    for f in ('🦀️.rs', 'Cargo.toml'): shutil.copy(o.ROOT + '/' + PK + '/' + f, sb + '/' + PK + '/' + f)
def overlay(sb, d):
    for dp, dn, fn in os.walk(d):
        for f in fn:
            src = os.path.join(dp, f); rel = os.path.relpath(src, d)
            dst = os.path.join(sb, PLUGREL, rel)
            os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(src, dst)
def run(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-plugin')
    t0 = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_plugin', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/pwp4-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - t0
def neutralize_pub(tree):
    rx = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?)extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?;[ \t]*$', re.M)
    n = 0
    for dp, dn, fn in os.walk(tree):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if not f.endswith('.rs'): continue
            p = os.path.join(dp, f)
            s = open(p, encoding='utf-8', errors='ignore').read()
            s2 = rx.sub(lambda m: m.group(1) + m.group(2) + 'mod ' + (m.group(4) or m.group(3)) + ' {}', s)
            if s2 != s: open(p, 'w', encoding='utf-8').write(s2); n += 1
    return n
def fold(c, keep_file):
    if keep_file: return c
    out = collections.Counter()
    for k, v in c.items(): out[k[:4]] += v
    return out
a_flags = set()
def main():
    global a_flags
    a = sys.argv[1:]; tag = a[0]
    a_flags = {x[2:] for x in a if x.startswith('--') and x[2:] in ('file',)}
    adir = a[a.index('--a') + 1] if '--a' in a else None
    bdir = a[a.index('--b') + 1] if '--b' in a else None
    configs = a[a.index('--configs') + 1].split(',') if '--configs' in a else ['lib', 'test', 'wasip2', 'lib-features', 'test-features']
    seed = a[a.index('--seed') + 1] if '--seed' in a else None
    base = SCR + '/sb-' + tag; shutil.rmtree(base, ignore_errors=True); os.makedirs(base)
    A, B = base + '/A', base + '/B'
    chain(A); chain(B)
    if adir: overlay(A, adir)
    if bdir: overlay(B, bdir)
    if seed: do_seed(A, seed)
    o.neutralize(A + '/' + PLUGREL); o.neutralize(B + '/' + PLUGREL)
    neutralize_pub(A + '/' + PLUGREL); neutralize_pub(B + '/' + PLUGREL)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        ea, ta = run(A, 'A-' + name, o.CFGS[name], tag); eb, tb = run(B, 'B-' + name, o.CFGS[name], tag)
        pa, pb = fold(o.parse(ea, A), 'file' in a_flags), fold(o.parse(eb, B), 'file' in a_flags)
        new, old = pa - pb, pb - pa
        summary[name] = dict(a=sum(pa.values()), b=sum(pb.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print(f'== {name}: A errors={sum(pa.values())} B errors={sum(pb.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:60]: print('  +new', v, k)
        for k, v in sorted(old.items())[:30]: print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/pwp4-oracle-{tag}-summary.json', 'w'), indent=1)
def do_seed(A, seed):
    root = A + '/' + PLUGREL
    if seed == 'private':     # make one piece item private again
        p = root + '/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs'
        t = open(p, encoding='utf-8').read(); assert 'pub struct PluginRuntime<' in t
        open(p, 'w', encoding='utf-8').write(t.replace('pub struct PluginRuntime<', 'struct PluginRuntime<', 1)); print('seeded: PluginRuntime private')
    if seed == 'mount':       # break a residual mount
        p = root + '/⚛️reactor/🏃️runtime/🦀️.rs'
        t = open(p, encoding='utf-8').read(); i = t.index('pub use life_app_cell::*;')
        open(p, 'w', encoding='utf-8').write(t[:i] + t[i + len('pub use life_app_cell::*;'):]); print('seeded: app_cell re-export removed')
if __name__ == '__main__': main()
