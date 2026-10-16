"""Stitched sandbox for the multi-crate world carve: all new crates become modules of one analysis crate."""
import sys, os, re, shutil, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import woracle
from woracle import *
woracle.EXTFILTER = True
from wplan import CRATES
from wgen import crate_ident

def stitch(tag, src, with_tests=True):
    root = mksb(tag, src)
    inf = f"{root}/{INFREL}"
    lib = f"{root}/{LIB}"
    t = open(lib, encoding='utf-8').read()
    old = re.search(r'#\[cfg\(any\(test, not\(all\(target_arch = "wasm32", target_env = "p2"\)\)\)\)\]\n#\[path = "\.\./\.\./🌍️world/🦀️\.rs"\]\npub mod world;\n', t)
    assert old, 'world mount not found'
    mounts = ''.join(f'#[path = "../../🌍️world/{folder}/🦀️.rs"]\npub mod {crate_ident(cid)};\n' for cid, bands, folder in CRATES)
    tmods = []
    for cid, bands, folder in CRATES:
        tdir = f"{inf}/🌍️world/{folder}/🧪️tests"
        if with_tests and os.path.isdir(tdir):
            for d in sorted(os.listdir(tdir)):
                if os.path.exists(f"{tdir}/{d}/🦀️.rs"):
                    name = f"t_{cid}_{len(tmods)}"
                    tmods.append((name, folder, d, cid))
                    mounts += f'#[cfg(test)]\n#[path = "../../🌍️world/{folder}/🧪️tests/{d}/🦀️.rs"]\nmod {name};\n'
    facade = open(f"{inf}/🌍️world/🦀️.rs", encoding='utf-8').read()
    facade = re.sub(r'pub use (semio_framework_os_infinite_world_)', r'pub use crate::\1', facade)
    facade = '\n'.join(l for l in facade.split('\n') if not l.startswith('//!'))
    mounts += '#[cfg(test)]\n#[path = "../../🌍️world/♻️resources/🧪️tests/🦀️.rs"]\nmod resources_test;\n'
    t = t[:old.start()] + mounts + '#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]\npub mod world {\n' + facade + '\n}\n' + t[old.end():]
    open(lib, 'w', encoding='utf-8').write(t)
    testfiles = {f"{inf}/🌍️world/{folder}/🧪️tests/{d}/🦀️.rs": name for name, folder, d, cid in tmods}
    rp = f"{inf}/🌍️world/♻️resources/🧪️tests/🦀️.rs"
    rs = open(rp, encoding='utf-8').read().replace('semio_framework_os_infinite::world::', 'crate::world::')
    open(rp, 'w', encoding='utf-8').write(rs)
    for cid, bands, folder in CRATES:
        base = f"{inf}/🌍️world/{folder}"
        mod = crate_ident(cid)
        for dp, dn, fn in os.walk(base):
            for f in fn:
                if not f.endswith('.rs'): continue
                p = os.path.join(dp, f)
                s = open(p, encoding='utf-8').read()
                s = re.sub(r'\buse (semio_framework_os_infinite_world_)', r'use crate::\1', s)
                s = re.sub(r'(?<![A-Za-z0-9_:])(semio_framework_os_infinite_world_[a-z_]+)::', r'crate::\1::', s)
                owner = testfiles.get(p, mod)
                s = s.replace('pub(crate)', f'pub(in crate::{owner})')
                open(p, 'w', encoding='utf-8').write(s)
    return root

if __name__ == '__main__':
    src = sys.argv[1]; tags = sys.argv[2].split(',') if len(sys.argv) > 2 else None
    root = stitch('w2', src)
    print('neutralized', neutralize(root))
    woracle.CFGS['ktest'] = ['--test', '--cfg', 'feature="testkit"']
    woracle.CFGS['ktestp2'] = ['--test', '--cfg', 'feature="testkit"', '--target', 'wasm32-wasip2']
    res = runall(root, tags)
    base = {t: woracle.norm(open(f"{SCR}/err-sb-base-{t.lstrip('k')}.json").read()) for t in res}
    compare(base, res, show=60)
