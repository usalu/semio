"""🔬️ WP2 union differential: the live renderer tree with the carved WP2 crates mounted as modules (A) against the same tree with the original WP2 element trees (B).
usage: oracle2.py <tag> [--configs lib,test,wasm,wasm-test] [--crates a,b] [--units mod=crate,...] [--seed]
Writes only below the scratchpad sandbox; every rewrite asserts that its target lies inside the sandbox."""
import collections, json, os, re, shutil, subprocess, sys, time
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rwp0-tools')
import oracle as o
from model import FOLDER
SCR = o.SCR
ORIG = SCR + '/rwp2/orig'
GEN = o.GEN
ENG, T, PK, HUBS = o.ENG, o.T, o.PK, o.HUBS
ELEMS = ['⚙️EngineCanvas', '🎞️Scenes', '🖼️IconRenderHost', '📐️Canvas2dHost']
HUB2 = HUBS + '/🎞️wp2-scenes/🦀️.rs'
ALIAS = {'element_contracts': 'crate::wp2_contracts', 'native_platform': 'crate::wp2_native_platform', 'asset_authority': 'crate::asset_authority', 'host_base': 'crate', 'canvas2d_paint': 'crate::canvas2d_paint', 'media_slots': 'crate::media_slots', 'mesh_assets': 'crate::mesh_assets', 'contracts': 'crate::wp2_contracts', 'program_bridge': 'crate::program_bridge'}
PFX = 'semio_framework_os_renderer_wgpu_'
PATHMAP = {'host_base': 'crate', 'native_platform': 'crate::wp2_native_platform', 'asset_authority': 'crate::asset_authority', 'contracts': 'crate::wp2_contracts', 'media_slots': 'crate::media_slots', 'canvas2d_paint': 'crate::canvas2d_paint'}

def guard(sb, path):
    real = os.path.realpath(path)
    assert real.startswith(os.path.realpath(sb) + '/'), (sb, path)

def children(path, text):
    base = os.path.dirname(path)
    out = []
    for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;', text):
        out.append(os.path.normpath(os.path.join(base, m.group(1))))
    for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', text):
        out.append(os.path.normpath(os.path.join(base, m.group(1))))
    return out

def closure(root):
    seen = []; stack = [root]
    while stack:
        p = stack.pop()
        if p in seen or not os.path.isfile(p): continue
        seen.append(p)
        stack += children(p, open(p, encoding='utf-8').read())
    return seen

def rewrite_owned(text, c):
    mod = 'm_' + c.replace('-', '_')
    text = re.sub(r'(?<![\w$:])crate::', 'crate::%s::' % mod, text)
    text = re.sub(r'pub\(crate\)', 'pub(in crate::%s)' % mod, text)
    return text

def rewrite_all(text, carved):
    def ext(m):
        dep = m.group(2).replace('-', '_'); alias = m.group(3)
        mod = 'm_' + dep
        if dep in carved: return '%spub(crate) use crate::%s as %s;' % (m.group(1), mod, alias)
        return '%spub(crate) use %s as %s;' % (m.group(1), ALIAS[alias], alias)
    text = re.sub(r'(?m)^(\s*(?:#\[[^\n]*\]\s*)*)extern\s+crate\s+' + PFX + r'([a-z0-9_]+)\s+as\s+(\w+);', ext, text)
    text = re.sub(PFX + r'([a-z0-9_]+)::', lambda m: 'crate::m_%s::' % m.group(1) if m.group(1) in carved else (PATHMAP[m.group(1)] + '::' if m.group(1) in PATHMAP else m.group(0)), text)
    return text

def transform(A, crates, units):
    carved = {c.replace('-', '_') for c in crates}
    for c in crates:
        rootf = '%s/%s/%s/🦀️.rs' % (A, T, FOLDER[c]); guard(A, rootf)
        rt = open(rootf, encoding='utf-8').read()
        def lifted(m):
            src = os.path.normpath(os.path.join(os.path.dirname(rootf), m.group(1) + '/🦀️.rs')); guard(A, src)
            dst = src[:-3] + '.u.rs'; shutil.copy(src, dst)
            return '#[path = "%s/🦀️.u.rs"]' % m.group(1)
        rt2 = re.sub(r'#\[path = "(📥️lifted/[^"]+)/🦀️\.rs"\]', lifted, rt)
        if rt2 != rt: open(rootf, 'w', encoding='utf-8').write(rt2)
    owned = {}
    for c in crates:
        root = '%s/%s/%s/🦀️.rs' % (A, T, FOLDER[c])
        for f in closure(root): owned.setdefault(f, c)
    for f, c in owned.items():
        guard(A, f); t = open(f, encoding='utf-8').read()
        open(f, 'w', encoding='utf-8').write(rewrite_all(rewrite_owned(t, c), carved))
    for el in ELEMS:
      for dp, dn, fn in os.walk('%s/%s/🧱️elements/%s' % (A, ENG, el)):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for name in fn:
          if not name.endswith('.rs'): continue
          p = os.path.join(dp, name)
          if p in owned: continue
          t = open(p, encoding='utf-8', errors='ignore').read()
          if PFX in t:
            guard(A, p); open(p, 'w', encoding='utf-8').write(rewrite_all(t, carved))
    hub = '%s/%s' % (A, HUB2); guard(A, hub)
    h = open(hub, encoding='utf-8').read()
    for dep in sorted(carved):
        h = re.sub(r'(?m)^extern crate ' + PFX + dep + r' as (\w+);', lambda m: 'pub(crate) use crate::m_%s as %s;' % (dep, m.group(1)), h)
    for mod, c in units.items():
        pat = re.compile(r'#\[path = "[^"]+"\]\n(pub(?:\([^)]*\))?) mod %s;\n' % mod)
        m = pat.search(h); assert m, mod
        h = h[:m.start()] + '%s use crate::m_%s as %s;\n' % (m.group(1), c.replace('-', '_'), mod) + h[m.end():]
    h += '\npub mod wp2_contracts {\n    pub use crate::element_contracts::*;\n    #[path = "%s/%s/🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs"]\n    mod admitted;\n    pub use admitted::*;\n}\npub mod wp2_native_platform {\n    #[path = "%s/%s/🖥️native-platform/📥️lifted/🎞️Scenes/🦀️.rs"]\n    mod temporal;\n    pub use temporal::*;\n}\n' % (A, T, A, T)
    for c in crates:
        h += '\n#[path = "%s/%s/%s/🦀️.rs"]\npub mod m_%s;\n' % (A, T, FOLDER[c], c.replace('-', '_'))
    open(hub, 'w', encoding='utf-8').write(h)

def baseline(B):
    for el in ELEMS:
        dst = '%s/%s/🧱️elements/%s' % (B, ENG, el); guard(B, dst)
        shutil.rmtree(dst, ignore_errors=True)
        subprocess.check_call(['rsync', '-a', '--exclude', 'node_modules', '%s/%s/🧱️elements/%s/' % (ORIG, ENG, el), dst + '/'])
    shutil.copy('%s/%s' % (ORIG, HUB2), '%s/%s' % (B, HUB2))

def run(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-renderer-wgpu')
    started = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_os_renderer_wgpu', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/📚️library/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open('%s/rwp2-oracle-%s-%s.jsonl' % (GEN, tag, label), 'w').write(p.stderr)
    return p.stderr, time.time() - started

def norm(c, sbA, mods):
    out = collections.Counter()
    for (code, msg, text, label, file), n in c.items():
        msg = re.sub(r'crate::m_[a-z0-9_]+::', 'crate::', msg); msg = re.sub(r'`m_[a-z0-9_]+`', '`crate`', msg)
        text = re.sub(r'crate::m_[a-z0-9_]+::', 'crate::', text)
        label = re.sub(r'crate::m_[a-z0-9_]+::', 'crate::', label); label = re.sub(r'`m_[a-z0-9_]+`', '`crate`', label); label = re.sub(r'(?:in|of) `crate::[A-Za-z_0-9:]*`', 'in X', label)
        out[(code, msg, text, label)] += n
    return out

def main():
    args = sys.argv[1:]; tag = args[0]
    configs = ['lib']; crates = []; units = {}
    for i, a in enumerate(args):
        if a == '--configs': configs = args[i + 1].split(',')
        if a == '--crates': crates = [x for x in args[i + 1].split(',') if x]
        if a == '--units': units = dict(x.split('=') for x in args[i + 1].split(',') if x)
        if a == '--seed': pass
    sbname = SCR + '/rwp2/sb-' + tag
    os.makedirs(sbname, exist_ok=True)
    A, B = sbname + '/A', sbname + '/B'
    o.chain(A); o.chain(B)
    baseline(B); transform(A, crates, units)
    if '--seed' in args:
        seed = [a for a in args if a.startswith('seedfile:')]
        for s in seed:
            _, f, old, new = s.split(':', 3)
            p = A + '/' + f; guard(A, p); t = open(p, encoding='utf-8').read(); assert t.count(old) == 1, (f, old); open(p, 'w', encoding='utf-8').write(t.replace(old, new))
    o.neutralize(A + '/' + ENG); o.neutralize(B + '/' + ENG)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        ea, ta = run(A, 'A-' + name, o.CFGS[name], tag); eb, tb = run(B, 'B-' + name, o.CFGS[name], tag)
        a, b = norm(o.parse(ea, A), A, crates), norm(o.parse(eb, B), B, crates)
        new, old = a - b, b - a
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print('== %s: A errors=%d B errors=%d onlyNew=%d onlyOld=%d sec=%d/%d' % (name, sum(a.values()), sum(b.values()), sum(new.values()), sum(old.values()), ta, tb), flush=True)
        for k, v in sorted(new.items())[:60]: print('  +new', v, k)
        for k, v in sorted(old.items())[:60]: print('  -old', v, k)
    json.dump(summary, open('%s/rwp2-oracle-%s-summary.json' % (GEN, tag), 'w'), indent=1)

if __name__ == '__main__': main()
