"""Sandbox differential oracle for the os-infinite world carve (analysis only; -Zno-codegen, nothing consumed)."""
import os, re, shutil, subprocess, sys, collections, json, pickle
R = "/Users/ueli/Documents/semio"
SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
FW = "🧰️framework"
INFREL = FW + "/🛍️products/💻️os/🔨️modules/♾️infinite"
TERREL = FW + "/🔨️modules/🗺️surface/🏔️terrain"
LIB = INFREL + "/📦️packages/🦀️rust/🦀️.rs"
TOOLCHAIN = "+nightly-2026-07-20"

def mksb(tag, infinite_src):
    root = f"{SCR}/sb-{tag}"
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(root + "/" + os.path.dirname(INFREL), exist_ok=True)
    ign = shutil.ignore_patterns('node_modules')
    shutil.copytree(infinite_src, f"{root}/{INFREL}", symlinks=True, ignore=ign)
    os.makedirs(root + "/" + os.path.dirname(TERREL), exist_ok=True)
    shutil.copytree(f"{R}/{TERREL}", f"{root}/{TERREL}", symlinks=True, ignore=ign)
    return root

def neutralize(root, extra_roots=()):
    files = []
    mods = set()
    for base in [f"{root}/{INFREL}", f"{root}/{TERREL}"] + list(extra_roots):
        for dp, dn, fn in os.walk(base):
            for f in fn:
                if f.endswith('.rs'):
                    p = os.path.join(dp, f); files.append(p)
                    for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)', open(p, encoding='utf-8', errors='ignore').read(), re.M): mods.add(m.group(1))
    keep = {'crate', 'super', 'self', 'infinite'} | mods
    rx = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    rxe = re.compile(r'^\s*extern crate [A-Za-z_]\w*(?: as [A-Za-z_]\w*)?;[ \t]*$', re.M)
    n = 0
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read()
        def sub(m):
            return m.group(1) + 'const _: () = ();' if m.group(2) not in keep else m.group(0)
        t2 = rx.sub(sub, t)
        if p.endswith('📦️packages/🦀️rust/🦀️.rs'):
            t2 = re.sub(r'^extern crate semio_framework_os_kernel as dsl;[ \t]*$', '', t2, flags=re.M)
            t2 = re.sub(r'^extern crate semio_framework_async as wasm_bindgen_futures;[ \t]*$', '', t2, flags=re.M)
        if t2 != t:
            open(p, 'w', encoding='utf-8').write(t2); n += 1
    return n

def run(root, lib, extra, crate='infinite_probe'):
    out = SCR + "/out"
    os.makedirs(out, exist_ok=True)
    env = dict(os.environ); env['CARGO_MANIFEST_DIR'] = os.path.dirname(root + '/' + lib)
    p = subprocess.run(["rustc", TOOLCHAIN, "-Zno-codegen", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--out-dir", out, "--error-format=json", "--json=diagnostic-short", *extra, root + "/" + lib],
                       capture_output=True, text=True, cwd=os.path.dirname(root + "/" + lib), env=env)
    for f in os.listdir(out):
        try: os.remove(os.path.join(out, f))
        except Exception: pass
    return p.stderr

def norm(stderr):
    out = collections.Counter()
    for ln in stderr.splitlines():
        if not ln.startswith('{'): continue
        try: d = json.loads(ln)
        except Exception: continue
        if d.get('level') not in ('error',): continue
        spans = [s for s in d.get('spans', []) if s.get('is_primary')]
        txt = lab = ''
        if spans:
            s = spans[0]
            txt = ' '.join(' '.join(t['text'] for t in s.get('text', [])).split())
            lab = s.get('label') or ''
        code = (d.get('code') or {}).get('code') or ''
        msg = re.sub(r'\s+', ' ', d['message'])
        msg = re.sub(re.escape(SCR) + r'/sb-[a-z0-9]+/', 'ROOT/', msg)
        if EXTFILTER:
            if msg.startswith("couldn't read"): continue
            if re.search(r'unresolved imports? `?(?!crate::semio_framework_os_infinite_world)', msg) or re.search(r'cannot find module or crate `(?!semio_framework_os_infinite_world)', msg):
                if 'no `' not in lab: continue
        out[(code, msg, '', lab)] += 1
    return out

EXTFILTER = False
CFGS = {"lib": [], "test": ["--test"], "wasm": ["--target", "wasm32-unknown-unknown"], "p2": ["--target", "wasm32-wasip2"], "testp2": ["--test", "--target", "wasm32-wasip2"]}

def runall(root, tags=None, lib=LIB, crate='infinite_probe'):
    res = {}
    for tag, extra in CFGS.items():
        if tags and tag not in tags: continue
        err = run(root, lib, extra, crate)
        open(f"{SCR}/err-{os.path.basename(root)}-{tag}.json", 'w').write(err)
        res[tag] = norm(err)
    return res

def compare(a, b, show=40, label=''):
    bad = 0
    for tag in a:
        x, y = a[tag], b[tag]
        only_a = x - y; only_b = y - x
        print(f"== {tag}: base errors {sum(x.values())} new errors {sum(y.values())} only-base {sum(only_a.values())} only-new {sum(only_b.values())}")
        for k, v in sorted(only_b.items())[:show]: print("  +new", v, k[0], k[1][:120], '|', k[2][:80], '|', k[3][:40])
        for k, v in sorted(only_a.items())[:show]: print("  -base", v, k[0], k[1][:120], '|', k[2][:80], '|', k[3][:40])
        bad += sum(only_a.values()) + sum(only_b.values())
    print("TOTAL DIFF", bad)
    return bad

if __name__ == '__main__':
    cmd = sys.argv[1]
    if cmd == 'base':
        src = sys.argv[2]
        root = mksb('base', src)
        print('neutralized files', neutralize(root))
        res = runall(root)
        pickle.dump(res, open(SCR + '/base.pkl', 'wb'))
        for t, c in res.items(): print(t, sum(c.values()))
    elif cmd == 'cur':
        src = sys.argv[2]
        root = mksb('cur', src)
        print('neutralized files', neutralize(root))
        res = runall(root)
        base = pickle.load(open(SCR + '/base.pkl', 'rb'))
        compare(base, res)
