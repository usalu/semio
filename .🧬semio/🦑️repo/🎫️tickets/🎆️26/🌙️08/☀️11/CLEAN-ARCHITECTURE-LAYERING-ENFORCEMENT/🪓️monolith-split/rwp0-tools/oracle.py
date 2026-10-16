# 🔬️ WP0 full-resolution differential: current engine tree (A) against the same tree with the WP0 root edits reverted (B).
# usage: oracle.py <tag> [--configs lib,test,wasm,wasm-test] [--baseline snapshot|inverse] [--jobs N]
# A = live engine copy; B = A with the renderer root and the library root restored (snapshot) or un-hubbed (inverse) and the hub folder removed.
# Both get the sandbox patches of 📓️RESOLUTION-ORACLE.md (extern crates become empty modules, external glob imports become `const _: () = ();`).
import collections, json, os, re, shutil, subprocess, sys, time
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
GEN = os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated'
ENG = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
T = ENG + '/🎯️targets/🧊️wgpu'
R_ROOT = T + '/🧊️renderer/🦀️.rs'
LIB_ROOT = T + '/📦️packages/🦀️rust/📚️library/🦀️.rs'
PK = T + '/📦️packages/🦀️rust'
HUBS = T + '/🧊️renderer/📥️mounts'
CFGS = {'lib': [], 'test': ['--test'], 'wasm': ['--target', 'wasm32-unknown-unknown'], 'wasm-test': ['--test', '--target', 'wasm32-unknown-unknown'], 'wasip2': ['--target', 'wasm32-wasip2']}

def neutralize(tree):
    files, names = [], set()
    for dp, dn, fn in os.walk(tree):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f); files.append(p)
                t = open(p, encoding='utf-8', errors='ignore').read()
                for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type|union)\s+(\w+)', t, re.M): names.add(m.group(1))
    keep = {'crate', 'super', 'self', 'std', 'core', 'alloc'} | names
    rx = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    ex = re.compile(r'^(\s*)extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?;[ \t]*$')
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read()
        t2 = rx.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + 'const _: () = ();', t)
        out = []
        for l in t2.split('\n'):
            em = ex.match(l)
            out.append(em.group(1) + 'mod ' + (em.group(3) or em.group(2)) + ' {}' if em else l)
        t2 = '\n'.join(out)
        if t2 != t: open(p, 'w', encoding='utf-8').write(t2)

def chain(sb):
    shutil.rmtree(sb, ignore_errors=True)
    real, box = ROOT, sb
    parts = ENG.split('/')
    os.makedirs(sb)
    for i, part in enumerate(parts):
        for e in os.listdir(real):
            if e == part: continue
            os.symlink(real + '/' + e, box + '/' + e)
        real, box = real + '/' + part, box + '/' + part
        os.makedirs(box, exist_ok=True)
    subprocess.check_call(['rsync', '-a', '--exclude', 'node_modules', '--exclude', 'target', real + '/', box + '/'])

REBASE = re.compile(r'(#\[\s*path\s*=\s*\s*"|include!\(\s*")\.\./\.\./')
def unhub(root_text, hub_dir):
    def sub(m):
        rel = m.group(1)
        text = open(hub_dir + '/' + rel + '/🦀️.rs', encoding='utf-8').read()
        text = REBASE.sub(lambda mm: mm.group(1), text)
        return '\n'.join(l for l in text.split('\n') if not l.startswith('// 🧭️ HUB')) + '\n'
    return re.sub(r'^include!\("📥️mounts/([^"/]+)/🦀️\.rs"\);[ \t]*\n', sub, root_text, flags=re.M)

def baseline(sb, mode):
    eng = sb + '/' + ENG
    if mode == 'snapshot':
        shutil.copy(SCR + '/orig/renderer-root.rs', sb + '/' + R_ROOT)
        shutil.copy(SCR + '/orig/library-root.rs', sb + '/' + LIB_ROOT)
    else:
        p = sb + '/' + R_ROOT
        t = open(p, encoding='utf-8').read()
        dup = '#[cfg(not(target_arch = "wasm32"))]\n#[path = "../⌨️native-entrypoint/📦️modules/🦀️.rs"]\nmod native_runtime_modules;\n'
        text = unhub(t, sb + '/' + HUBS)
        assert text.count(dup) == 1
        open(p, 'w', encoding='utf-8').write(text.replace(dup, ''))
        shutil.copy(SCR + '/orig/library-root.rs', sb + '/' + LIB_ROOT)
    shutil.rmtree(sb + '/' + HUBS, ignore_errors=True)

def run(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-renderer-wgpu')
    started = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_os_renderer_wgpu', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/📚️library/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/rwp0-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - started

HUBFILE = re.compile(r'^' + re.escape(T) + r'/🧊️renderer/(?:📥️mounts/[^/]+/)?🦀️\.rs$')
def parse(err, sb):
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
            if HUBFILE.match(file): file = '<renderer-root-or-hub>'
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = str(d.get('message', '')).replace(sb + '/', '').replace(ROOT + '/', '')
        msg = re.sub(r'[^`]*?(?=🧰️framework)', '', msg, count=0) if False else msg
        c[(code, os.path.normpath(msg) if msg.startswith("couldn't read") else msg, text, label, file)] += 1
    return c

def main():
    args = sys.argv[1:]
    tag = args[0]
    configs = ['lib', 'test', 'wasm', 'wasm-test']
    mode = 'snapshot'
    for i, a in enumerate(args):
        if a == '--configs': configs = args[i + 1].split(',')
        if a == '--baseline': mode = args[i + 1]
    sbname = SCR + '/rwp0-sb-' + tag + '-' + '_'.join(configs)
    os.makedirs(sbname, exist_ok=True)
    A, B = sbname + '/A', sbname + '/B'
    chain(A); chain(B)
    if 'selfcheck' not in tag: baseline(B, mode)
    seeds = {a.split(':')[1] for a in args if a.startswith('seed:')} | ({'path', 'glob', 'import'} if '--seed' in args else set())
    if seeds:
        for name, (pth, a, b) in {
            'glob': (A + '/' + HUBS + '/🧵️wp3-frame-core/🦀️.rs', 'pub(crate) use frame_actions::*;\n', ''),
            'path': (A + '/' + HUBS + '/🎞️wp2-scenes/🦀️.rs', '🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod scenes;', '🎯️targets/🧊️wgpu/📛️absent/🦀️.rs"]\npub mod scenes;'),
            'import': (A + '/' + R_ROOT, 'use semio_framework_os_renderer_camera as camera_storage;\n', ''),
        }.items():
            if name not in seeds: continue
            s = open(pth, encoding='utf-8').read()
            assert s.count(a) == 1, (pth, a)
            open(pth, 'w', encoding='utf-8').write(s.replace(a, b))
        print('seeded:', sorted(seeds), flush=True)
    neutralize(A + '/' + ENG); neutralize(B + '/' + ENG)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        ea, ta = run(A, 'A-' + name, CFGS[name], tag)
        eb, tb = run(B, 'B-' + name, CFGS[name], tag)
        a, b = parse(ea, A), parse(eb, B)
        new, old = a - b, b - a
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print(f'== {name}: A(current) errors={sum(a.values())} B(baseline) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:40]: print('  +new', v, k)
        for k, v in sorted(old.items())[:40]: print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/rwp0-oracle-{tag}-summary.json', 'w'), indent=1)

def reparse(tag, name, configs=None):
    base = SCR + '/rwp0-sb-' + tag + '-' + (configs or name)
    A, B = base + '/A', base + '/B'
    if not os.path.isdir(A): A, B = SCR + '/rwp0-sb/A', SCR + '/rwp0-sb/B'
    ea = open(f'{GEN}/rwp0-oracle-{tag}-A-{name}.jsonl').read(); eb = open(f'{GEN}/rwp0-oracle-{tag}-B-{name}.jsonl').read()
    a, b = parse(ea, A), parse(eb, B)
    new, old = a - b, b - a
    print(f'== {name}: A(current) errors={sum(a.values())} B(baseline) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())}')
    for k, v in sorted(new.items())[:40]: print('  +new', v, k)
    for k, v in sorted(old.items())[:40]: print('  -old', v, k)

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == 'reparse': reparse(sys.argv[2], sys.argv[3])
    else: main()
