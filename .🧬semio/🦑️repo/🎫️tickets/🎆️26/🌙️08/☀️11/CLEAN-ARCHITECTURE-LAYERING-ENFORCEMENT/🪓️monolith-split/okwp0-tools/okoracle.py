# 🔬️ Full-resolution differential library for the OS kernel partition (analysis only: rustc output goes to /dev/null, every write is inside a sandbox directory).
# Sandboxes hold only the repo-relative files of the kernel closure (union of the closure rows), copied from a source tree; extern crates become empty modules and external globs become `const _: () = ();` (📓️RESOLUTION-ORACLE.md patches).
import collections, json, os, re, shutil, subprocess, time
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
GEN = os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated'
K_REL = '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
CRATE = 'semio_framework_os_kernel'
FEATS = ['deflate', 'sync', 'ureq', 'worker']
CFGS = {
    'lib': [], 'test': ['--test'], 'wasm': ['--target', 'wasm32-unknown-unknown'], 'wasm-test': ['--test', '--target', 'wasm32-unknown-unknown'], 'wasip2': ['--target', 'wasm32-wasip2'],
    'lib-nofeat': ['--cfg', 'NOFEATURES'], 'laws': [], 'laws-test': ['--test'],
}
EXTRA_FEATS = {'laws': ['protocol-laws', 'record-codec-laws', 'mutation-testing', 'durable-group-testing'], 'laws-test': ['protocol-laws', 'record-codec-laws', 'mutation-testing', 'durable-group-testing']}

def inside(path, root):
    rp = os.path.realpath(path); rr = os.path.realpath(root)
    assert rp == rr or rp.startswith(rr + os.sep), ('write outside sandbox', path, root)

def copy_files(dest, files, src_root=ROOT):
    for rel in files:
        s = os.path.join(src_root, rel)
        if not os.path.exists(s): continue
        d = os.path.join(dest, rel)
        inside(d, dest)
        os.makedirs(os.path.dirname(d), exist_ok=True)
        shutil.copy2(s, d)

def neutralize(tree):
    files, names = [], set()
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f); files.append(p)
                t = open(p, encoding='utf-8', errors='ignore').read()
                for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type|union)\s+(\w+)', t, re.M): names.add(m.group(1))
    keep = {'crate', 'super', 'self', 'std', 'core', 'alloc'} | names
    rx = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    ex = re.compile(r'^(\s*)extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?;[ \t]*$')
    for p in files:
        inside(p, tree)
        t = open(p, encoding='utf-8', errors='ignore').read()
        t2 = rx.sub(lambda m: m.group(0) if m.group(2) in keep else m.group(1) + 'const _: () = ();', t)
        out = []
        for l in t2.split('\n'):
            em = ex.match(l)
            out.append(em.group(1) + 'mod ' + (em.group(3) or em.group(2)) + ' {}' if em and not (em.group(2) == 'self') else l)
        t2 = '\n'.join(out)
        if t2 != t: open(p, 'w', encoding='utf-8').write(t2)

def run(sb, label, name, tag, root_rel=K_REL + '/🦀️.rs', extra_feats=()):
    pk = os.path.join(sb, K_REL)
    flags = list(CFGS[name])
    feats = FEATS + list(EXTRA_FEATS.get(name, [])) if name != 'lib-nofeat' else []
    if name == 'lib-nofeat': flags = []
    cfg = []
    for f in feats + list(extra_feats): cfg += ['--cfg', f'feature="{f}"']
    env = dict(os.environ, OUT_DIR=sb + '/outdir', CARGO_PKG_VERSION='0.1.0', CARGO_MANIFEST_DIR=pk, CARGO_PKG_NAME='semio-framework-os-kernel')
    started = time.time()
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', CRATE, '--error-format=json', '--emit=metadata', '-o', '/dev/null', *flags, *cfg, os.path.join(sb, root_rel)],
                       capture_output=True, text=True, env=env, cwd=pk)
    open(f'{GEN}/okwp0-oracle-{tag}-{label}-{name}.jsonl', 'w').write(p.stderr)
    return p.stderr, time.time() - started

def parse(err, sb, fold=lambda f: f):
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
            file = fold(file)
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = str(d.get('message', '')).replace(sb + '/', '').replace(ROOT + '/', '')
        m2 = re.match(r'unresolved imports (`.*`)$', msg)
        if m2:
            for name in re.findall(r'`([^`]*)`', m2.group(1)): c[(code, f'unresolved import `{name}`', '', '', file)] += 1
            continue
        c[(code, msg, text, label, file)] += 1
    return c

def diff(a, b):
    return a - b, b - a

EXT_IDENTS = ('semio_framework_', 'protocol', 'pack', 'dsl_derive', 'serde', 'serde_json', 'tokio', 'futures', 'web_sys', 'js_sys', 'wasm_bindgen', 'blake3', 'dsl', 'vcs', 'ureq', 'tokio_tungstenite', 'value_derive')
def is_external(code, msg, label):
    if 'unlinked crate' in label or 'unlinked crate' in msg: return True
    if msg.startswith('cannot find module or crate'): return True
    m = re.match(r'unresolved import `([A-Za-z_0-9]+)(?:::[^`]*)?`$', msg)
    if m and (m.group(1).startswith('semio_framework_') or m.group(1) in EXT_IDENTS): return True
    return False

def parse_keyed(err, sb, fold=lambda f: f):
    """Internal errors keyed by (code, highlighted text, file); errors that only concern an extern crate are counted apart."""
    inner = collections.Counter(); ext = collections.Counter(); examples = {}
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
            file = fold(os.path.normpath(sp.get('file_name', '').replace(sb + '/', '').replace(ROOT + '/', '')))
            label = sp.get('label') or ''
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = str(d.get('message', '')).replace(sb + '/', '').replace(ROOT + '/', '')
        msgs = [msg]
        m2 = re.match(r'unresolved imports (`.*`)$', msg)
        if m2: msgs = [f'unresolved import `{n}`' for n in re.findall(r'`([^`]*)`', m2.group(1))]
        for mm in msgs:
            if is_external(code, mm, label): ext[(code, text)] += 1
            else:
                key = (code, re.sub(r'__os_', 'os_', text), file)
                inner[key] += 1; examples.setdefault(key, mm)
    return inner, ext, examples
