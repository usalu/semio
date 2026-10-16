# 🔬️ WP3 full-resolution differential of the plugin crate: current tree (A) against the same tree with every WP3-touched path restored from the pre-edit snapshot (B). usage: oracle3.py <tag> [--configs lib,test,wasip2,wasip2-test,lib-features,test-features] [--seed:<name>] [--noise]
# Both trees get the sandbox patches of 📓️RESOLUTION-ORACLE.md (extern crates become empty modules, external glob imports become `const _: () = ();`).
import collections, json, os, re, shutil, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pwp7'
GEN = ROOT + '/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated'
PLUG = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
PK = PLUG + '/📦️packages/🦀️rust'
MOUNTS_DIR = '🪝️mounts'
HUBS = PLUG + '/' + MOUNTS_DIR
FEATS = ['artifact-app-testing', 'media-turn-testing', 'reserved-producer-testing', 'original-actor-context-testing']
FEATARGS = [x for f in FEATS for x in ('--cfg', 'feature="%s"' % f)]
CFGS = {'lib': [], 'test': ['--test'], 'wasip2': ['--target', 'wasm32-wasip2'], 'wasip2-test': ['--test', '--target', 'wasm32-wasip2'], 'lib-features': FEATARGS, 'test-features': ['--test'] + FEATARGS, 'wasm': ['--target', 'wasm32-unknown-unknown']}
EXCLUDE = ['node_modules', 'target', '.cache', '🖥️host', '🧠️interpreter', '🏇️mounted-owner', '🏛️ownership', '🧬️guest-contract', '🖨️describe', '📦️packages', '.🧬semio', '🌐️browser-bundle', '🏗️build']

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
    ex = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?)extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?;[ \t]*$')
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read()
        t2 = rx.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + 'const _: () = ();', t)
        out = []
        for l in t2.split('\n'):
            em = ex.match(l)
            out.append(em.group(1) + em.group(2) + 'mod ' + (em.group(4) or em.group(3)) + ' {}' if em else l)
        t2 = '\n'.join(out)
        if t2 != t: open(p, 'w', encoding='utf-8').write(t2)

def chain(sb):
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb + '/' + PLUG)
    real = ROOT + '/' + PLUG
    ex = []
    for e in EXCLUDE: ex += ['--exclude', e]
    subprocess.check_call(['rsync', '-a', *ex, real + '/', sb + '/' + PLUG + '/'])
    os.makedirs(sb + '/' + PK, exist_ok=True)
    for f in ('🦀️.rs', 'Cargo.toml'): shutil.copy(ROOT + '/' + PK + '/' + f, sb + '/' + PK + '/' + f)
    # sibling crates referenced by `#[path]`/include from the plugin tree
    # (none are needed for name resolution: external crates are neutralised)

REBASE = re.compile(r'(#\[\s*path\s*=\s*\s*")\.\./\.\./')
def unhub(root_text, hub_dir):
    def sub(m):
        rel = m.group(1)
        text = open(hub_dir + '/' + rel + '/🦀️.rs', encoding='utf-8').read()
        text = REBASE.sub(lambda mm: mm.group(1), text)
        return '\n'.join(l for l in text.split('\n') if not l.startswith(('// ' + rel[:1], '// This file is spliced', '// its `#[path]`', '// Carving a crate removes')) or True) + '\n'
    return re.sub(r'^include!\("' + re.escape(MOUNTS_DIR) + r'/([^"/]+)/🦀️\.rs"\);[ \t]*\n', sub, root_text, flags=re.M)

def baseline_unused(sb, mode):
    root = sb + '/' + PLUG + '/🦀️.rs'
    if mode == 'snapshot':
        shutil.copy(SCR + '/orig/plugin-root.rs', root)
    else:
        t = open(root, encoding='utf-8').read()
        open(root, 'w', encoding='utf-8').write(unhub(t, sb + '/' + HUBS))
    shutil.rmtree(sb + '/' + HUBS, ignore_errors=True)

def run(sb, label, extra, tag):
    pk = sb + '/' + PK
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-plugin')
    started = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'semio_framework_plugin', '--error-format=json', '--emit=metadata', '-o', '/dev/null', *extra, pk + '/🦀️.rs'], capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/pwp7-oracle-{tag}-{label}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - started

HUBFILE = re.compile(r'^' + re.escape(PLUG) + r'/(?:' + re.escape(MOUNTS_DIR) + r'/[^/]+/)?🦀️\.rs$')
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
            if HUBFILE.match(file): file = '<plugin-root-or-hub>'
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = str(d.get('message', '')).replace(sb + '/', '').replace(ROOT + '/', '')
        m2 = re.match(r"couldn't read `([^`]*)`(.*)", msg, re.S)
        if m2: msg = "couldn't read `" + os.path.normpath(m2.group(1)) + '`' + m2.group(2)
        c[(code, msg, text, label, '' if os.environ.get('FOLD', '1') == '1' else file)] += 1
    return c


SNAP = GEN + '/pwp7-snap'
def overlay(sb):
    allt = json.load(open(GEN + '/pwp7-touched.json', encoding='utf-8'))
    created = set(); modified = set(); removed = set()
    for st in allt:
        created |= set(st['created']); modified |= set(st['modified']); removed |= set(st['removed'])
    for rel in created - removed:
        p = os.path.join(sb, PLUG, rel)
        if os.path.exists(p): os.remove(p)
    for rel in (modified | removed) - (created - removed):
        src_ = os.path.join(SNAP, rel); dst = os.path.join(sb, PLUG, rel)
        if not os.path.exists(src_): continue
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(src_, dst)

INC = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)')
def stub_missing_includes(tree):
    n = 0
    for dp, dn, fn in os.walk(tree):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if not f.endswith('.rs'): continue
            p = os.path.join(dp, f)
            try: t = open(p, encoding='utf-8', errors='ignore').read()
            except Exception: continue
            for m in INC.finditer(t):
                q = os.path.normpath(os.path.join(dp, m.group(1)))
                if not os.path.exists(q) and q.startswith(SCR):
                    os.makedirs(os.path.dirname(q), exist_ok=True)
                    open(q, 'w').write('{}\n' if q.endswith('.json') else '')
                    n += 1
    return n

def seed_hook(A, seeds):
    if not seeds: return
    spec = json.load(open(GEN + '/pwp6-seeds.json', encoding='utf-8'))
    for s in seeds:
        d = spec[s.split(':', 1)[1]]
        pth = os.path.join(A, PLUG, d['file']); txt = open(pth, encoding='utf-8').read()
        assert txt.count(d['old']) >= 1, (s, d['file'])
        open(pth, 'w', encoding='utf-8').write(txt.replace(d['old'], d['new'], 1))
    print('seeded:', seeds, flush=True)

def main():
    args = sys.argv[1:]
    tag = args[0]
    configs = ['lib', 'test', 'wasip2', 'lib-features', 'test-features']
    for i, a in enumerate(args):
        if a == '--configs': configs = args[i + 1].split(',')
    sbname = SCR + '/pwp7-sb-' + tag
    os.makedirs(sbname, exist_ok=True)
    A, B = sbname + '/A', sbname + '/B'
    chain(A); chain(B)
    overlay(B)
    neutralize(A + '/' + PLUG); neutralize(B + '/' + PLUG)
    print('stubbed missing include targets:', stub_missing_includes(A + '/' + PLUG), stub_missing_includes(B + '/' + PLUG), flush=True)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    summary = {}
    for name in configs:
        from concurrent.futures import ThreadPoolExecutor
        with ThreadPoolExecutor(2) as ex:
            fa = ex.submit(run, A, 'A-' + name, CFGS[name], tag); fb = ex.submit(run, B, 'B-' + name, CFGS[name], tag)
            ea, ta = fa.result(); eb, tb = fb.result()
        a, b = parse(ea, A), parse(eb, B)
        new, old = a - b, b - a
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print(f'== {name}: A(current) errors={sum(a.values())} B(baseline) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:60]: print('  +new', v, k)
        for k, v in sorted(old.items())[:60]: print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/pwp7-oracle-{tag}-summary.json', 'w'), indent=1)

if __name__ == '__main__': main()
