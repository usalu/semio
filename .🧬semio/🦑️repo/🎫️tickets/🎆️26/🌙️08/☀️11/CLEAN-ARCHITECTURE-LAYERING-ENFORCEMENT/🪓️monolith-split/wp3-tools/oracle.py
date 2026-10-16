# 🔬️ WP0 full-resolution differential: current engine tree (A) against the same tree with the WP0 root edits reverted (B).
# usage: oracle.py <tag> [--configs lib,test,wasm,wasm-test] [--baseline snapshot|inverse] [--jobs N]
# A = live engine copy; B = A with the renderer root and the library root restored (snapshot) or un-hubbed (inverse) and the hub folder removed.
# Both get the sandbox patches of 📓️RESOLUTION-ORACLE.md (extern crates become empty modules, external glob imports become `const _: () = ();`).
import collections, json, os, re, shutil, subprocess, sys, time
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
GEN = os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated'
ORIG = SCR + '/wp3-orig'
OWNED = [l for l in open(SCR + '/wp3-owned-paths.txt', encoding='utf8').read().split('\n') if l]
ENG0 = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
SINGLES = {'kernel-core': ENG0 + '/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs', 'kernel-replay-client': ENG0 + '/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs'}
CREATED = [l for l in open(os.path.dirname(os.path.abspath(__file__)) + '/created-outside.txt', encoding='utf8').read().split('\n') if l] if os.path.exists(os.path.dirname(os.path.abspath(__file__)) + '/created-outside.txt') else []
ENG = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
ENG0 = ENG
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
    # B = live tree with every WP3-owned path restored from the snapshot taken before the first edit
    for rel in OWNED:
        dst = sb + '/' + rel
        if os.path.isdir(dst) and not os.path.islink(dst): shutil.rmtree(dst)
        elif os.path.exists(dst): os.remove(dst)
        s = ORIG + '/' + rel
        if os.path.isdir(s): shutil.copytree(s, dst)
        else:
            os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copy2(s, dst)
    shutil.copy2(ORIG + '/kernel-core.rs', sb + '/' + SINGLES['kernel-core'])
    shutil.copy2(ORIG + '/kernel-replay-client.rs', sb + '/' + SINGLES['kernel-replay-client'])
    for rel in CREATED:
        p = sb + '/' + rel
        if os.path.isdir(p): shutil.rmtree(p)
        elif os.path.exists(p): os.remove(p)

OUTRMETA = SCR + '/wp3-oracle-out/x-' + str(os.getpid()) + '.rmeta'
os.makedirs(SCR + '/wp3-oracle-out', exist_ok=True)
def run(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-renderer-wgpu')
    started = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_os_renderer_wgpu', '--error-format=json', '--emit=metadata', '-o', OUTRMETA, *extra, pk + '/📚️library/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/rwp3-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
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
        c[(code, os.path.basename(msg) if msg.startswith("couldn't read") else msg, text, label, '')] += 1
    return c

def main():
    args = sys.argv[1:]
    tag = args[0]
    configs = ['lib', 'test', 'wasm', 'wasm-test']
    mode = 'snapshot'
    for i, a in enumerate(args):
        if a == '--configs': configs = args[i + 1].split(',')
        if a == '--baseline': mode = args[i + 1]
    sbname = SCR + '/rwp3-sb-' + tag + '-' + '_'.join(configs)
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
    wp3seeds = [s for s in os.environ.get('WP3_SEEDS', '').split(',') if s]
    V = '\ufe0f'
    for s in wp3seeds:
        RR = A + '/' + T + f'/🧊{V}renderer'
        if s == 'glob':
            pth = RR + f'/🛎{V}frame-actions/🦀{V}.rs'; x = open(pth, encoding='utf8').read(); y = 'pub(crate) use interaction_state::*;\n'
            assert x.count(y) == 1; open(pth, 'w', encoding='utf8').write(x.replace(y, ''))
        if s == 'private':
            pth = RR + f'/🛎{V}frame-actions/🦀{V}.rs'; x = open(pth, encoding='utf8').read(); y = 'pub(crate) const WORLD3D_DEADLINE_ID_BYTES: usize = 256;'
            assert x.count(y) == 1; open(pth, 'w', encoding='utf8').write(x.replace(y, 'const WORLD3D_DEADLINE_ID_BYTES: usize = 256;'))
        if s == 'generic':
            pth = RR + f'/📮{V}mailbox-impl/🦀{V}.rs'; x = open(pth, encoding='utf8').read(); y = 'runtime.interaction_mut()'
            assert x.count(y) > 3; open(pth, 'w', encoding='utf8').write(x.replace(y, 'runtime.interaction.as_mut()', 1))
        print('seeded', s, flush=True)
    import re as _re
    for sb in (A, B):
        hubp = sb + '/' + HUBS + '/🌱️wp1-foundation/🦀️.rs'
        if os.path.exists(hubp) and os.environ.get('WP3_DROP_BROKEN_HOSTS', '1') == '1':
            x = open(hubp, encoding='utf8').read()
            x2 = _re.sub(r'(?:#\[cfg[^\n]*\]\n)*mod \w+_host \{[^}]*\}\n', '', x)
            if x2 != x: open(hubp, 'w', encoding='utf8').write(x2); print('dropped broken WP1 test host in', sb[-3:], flush=True)
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
    json.dump(summary, open(f'{GEN}/rwp3-oracle-{tag}-summary.json', 'w'), indent=1)

def reparse(tag, name, configs=None):
    base = SCR + '/rwp3-sb-' + tag + '-' + (configs or name)
    A, B = base + '/A', base + '/B'
    if not os.path.isdir(A): A, B = SCR + '/rwp3-sb/A', SCR + '/rwp3-sb/B'
    ea = open(f'{GEN}/rwp3-oracle-{tag}-A-{name}.jsonl').read(); eb = open(f'{GEN}/rwp3-oracle-{tag}-B-{name}.jsonl').read()
    a, b = parse(ea, A), parse(eb, B)
    new, old = a - b, b - a
    print(f'== {name}: A(current) errors={sum(a.values())} B(baseline) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())}')
    for k, v in sorted(new.items())[:40]: print('  +new', v, k)
    for k, v in sorted(old.items())[:40]: print('  -old', v, k)

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == 'reparse': reparse(sys.argv[2], sys.argv[3])
    else: main()
