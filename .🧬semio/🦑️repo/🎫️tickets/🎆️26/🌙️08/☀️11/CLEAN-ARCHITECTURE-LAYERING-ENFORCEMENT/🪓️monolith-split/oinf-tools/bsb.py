"""Stitched sandbox for the board carve: world crates + board crates become modules of one analysis crate."""
import sys, os, re, shutil, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import collections, woracle
from woracle import *
woracle.EXTFILTER = True
import wplan, wgen
from wplan import CRATES as WCRATES
from wgen import crate_ident as wident
import bcrates
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"

def prepare_src(base_infinite, crates_root, dst):
    """base infinite tree with the board module tree replaced by the generated board crates (lib roots only)."""
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(base_infinite, dst, symlinks=True, ignore=shutil.ignore_patterns('node_modules'))
    shutil.rmtree(dst + '/🎲️board', ignore_errors=True)
    for cid, bands, folder in bcrates.CRATES:
        os.makedirs(f"{dst}/{folder}", exist_ok=True)
        shutil.copyfile(f"{crates_root}/{folder}/🦀️.rs", f"{dst}/{folder}/🦀️.rs")
        if os.path.isdir(f"{crates_root}/{folder}/🧪️tests"): shutil.copytree(f"{crates_root}/{folder}/🧪️tests", f"{dst}/{folder}/🧪️tests", dirs_exist_ok=True)
    lib = dst + '/📦️packages/🦀️rust/🦀️.rs'
    fl = f"{crates_root}/📦️packages/🦀️rust/🦀️.rs"
    if os.path.exists(fl): shutil.copyfile(fl, lib)
    else:
        t = open(lib, encoding='utf-8').read()
        a = t.index('#[path = "."]\npub mod board {')
        open(lib, 'w', encoding='utf-8').write(t[:a])

def stitch(tag, src, with_tests=False):
    root = mksb(tag, src)
    inf = f"{root}/{INFREL}"
    lib = f"{root}/{LIB}"
    t = open(lib, encoding='utf-8').read()
    old = re.search(r'#\[cfg\(any\(test, not\(all\(target_arch = "wasm32", target_env = "p2"\)\)\)\)\]\n#\[path = "\.\./\.\./🌍️world/🦀️\.rs"\]\npub mod world;\n', t)
    assert old, 'world mount not found'
    mounts = ''.join(f'#[path = "../../🌍️world/{folder}/🦀️.rs"]\npub mod {wident(cid)};\n' for cid, bands, folder in WCRATES)
    bmounts = ''.join(f'#[path = "../../{folder}/🦀️.rs"]\npub mod {bcrates.crate_ident(cid)};\n' for cid, bands, folder in bcrates.CRATES)
    btests = {}
    if with_tests:
        for cid, bands, folder in bcrates.CRATES:
            tdir = f"{inf}/{folder}/🧪️tests"
            if os.path.isdir(tdir):
                for d in sorted(os.listdir(tdir)):
                    if os.path.exists(f"{tdir}/{d}/🦀️.rs"):
                        name = f"bt_{cid.replace('-', '_')}_{len(btests)}"
                        btests[f"{tdir}/{d}/🦀️.rs"] = name
                        bmounts += f'#[cfg(test)]\n#[path = "../../{folder}/🧪️tests/{d}/🦀️.rs"]\nmod {name};\n'
    facade = open(f"{inf}/🌍️world/🦀️.rs", encoding='utf-8').read()
    facade = re.sub(r'pub use (semio_framework_os_infinite_world_)', r'pub use crate::\1', facade)
    facade = '\n'.join(l for l in facade.split('\n') if not l.startswith('//!'))
    t = re.sub(r'(?m)^(\s*)pub use (semio_framework_os_infinite_board_)', r'\1pub use crate::\2', t)
    t = t[:old.start()] + mounts + bmounts + '#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]\npub mod world {\n' + facade + '\n}\n' + t[old.end():]
    open(lib, 'w', encoding='utf-8').write(t)
    for cid, bands, folder in WCRATES:
        base = f"{inf}/🌍️world/{folder}"
        mod = wident(cid)
        for dp, dn, fn in os.walk(base):
            for f in fn:
                if not f.endswith('.rs'): continue
                p = os.path.join(dp, f)
                s = open(p, encoding='utf-8').read()
                s = re.sub(r'\buse (semio_framework_os_infinite_world_)', r'use crate::\1', s)
                s = re.sub(r'(?<![A-Za-z0-9_:])(semio_framework_os_infinite_world_[a-z_]+)::', r'crate::\1::', s)
                s = s.replace('pub(crate)', f'pub(in crate::{mod})')
                open(p, 'w', encoding='utf-8').write(s)
    for cid, bands, folder in bcrates.CRATES:
        p = f"{inf}/{folder}/🦀️.rs"
        mod = bcrates.crate_ident(cid)
        s = open(p, encoding='utf-8').read()
        s = re.sub(r'\buse (semio_framework_os_infinite_board_)', r'use crate::\1', s)
        s = re.sub(r'(?<![A-Za-z0-9_:])(semio_framework_os_infinite_board_[a-z_]+)::', r'crate::\1::', s)
        s = s.replace('pub(crate)', f'pub(in crate::{mod})')
        s = re.sub(r'(?m)^#!\[.*\]\s*\n', '', s)
        open(p, 'w', encoding='utf-8').write(s)
    for p, name in btests.items():
        s = open(p, encoding='utf-8').read()
        s = re.sub(r'\buse (semio_framework_os_infinite_board_)', r'use crate::\1', s)
        s = re.sub(r'(?<![A-Za-z0-9_:])(semio_framework_os_infinite_board_[a-z_]+)::', r'crate::\1::', s)
        s = s.replace('pub(crate)', f'pub(in crate::{name})')
        open(p, 'w', encoding='utf-8').write(s)
    return root

if __name__ == '__main__':
    crates_root = sys.argv[1]; tags = sys.argv[2].split(',') if len(sys.argv) > 2 else None
    prep = SC + '/b-prepared'
    prepare_src(SC + '/snapshot-board', crates_root, prep)
    withtests = any(x.startswith('k') for x in (tags or []))
    root = stitch('b4', prep, with_tests=withtests)
    print('neutralized', neutralize(root))
    woracle.CFGS['ktest'] = ['--test', '--cfg', 'feature="testkit"']
    res = runall(root, tags)
    def nolabel(c):
        o = collections.Counter()
        for (code, msg, a, lab), n in c.items(): o[(code, msg, '', '')] += n
        return o
    base = {t: nolabel(woracle.norm(open(f"{SC}/err-sb-baseB-{t}.json").read())) for t in res}
    res = {t: nolabel(c) for t, c in res.items()}
    compare(base, res, show=60)
