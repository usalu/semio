"""🔀 Atomic switch of the board carve into the repository: python3 bswitch.py <apply-b dir> [dry]."""
import sys, os, shutil, hashlib, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bfinal, bpoint, breg
from bcrates import CRATES, crate_pkg

R = "/Users/ueli/Documents/semio/"
INF = R + "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/"
SC = bfinal.SC

def rs_files(root):
    out = []
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d != 'node_modules']
        out += [os.path.join(dp, f) for f in fn if f.endswith('.rs')]
    return out

def new_files(apply):
    out = {}
    for c in CRATES:
        base = f"{apply}/INF/{c[2]}"
        for dp, dn, fn in os.walk(base):
            for f in fn:
                p = os.path.join(dp, f)
                out[os.path.relpath(p, apply + '/INF')] = p
    return out

def check_preconditions(apply):
    snap = SC + '/snapshot-board'
    def md5(root):
        h = hashlib.md5()
        for dp, dn, fn in sorted(os.walk(root)):
            dn[:] = sorted(d for d in dn if d != 'node_modules')
            for f in sorted(fn):
                p = os.path.join(dp, f); h.update(os.path.relpath(p, root).encode()); h.update(open(p, 'rb').read())
        return h.hexdigest()
    a = md5(snap + '/🎲️board'); b = md5(INF + '🎲️board')
    same_facade = open(snap + '/📦️packages/🦀️rust/🦀️.rs', 'rb').read() == open(INF + '📦️packages/🦀️rust/🦀️.rs', 'rb').read()
    return a == b, same_facade

def delete_set(apply, newrel):
    keep_roots = []
    for dp, dn, fn in os.walk(INF + '🎲️board'):
        if 'Cargo.toml' in fn and dp.endswith('/📦️packages/🦀️rust'): keep_roots.append(os.path.dirname(os.path.dirname(dp)) + '/')
    dels = []
    for p in rs_files(INF + '🎲️board'):
        rel = os.path.relpath(p, INF)
        if rel in newrel: continue
        if any(p.startswith(k) for k in keep_roots): continue
        dels.append(p)
    return dels, keep_roots

def patch_facade_manifest():
    p = INF + '📦️packages/🦀️rust/Cargo.toml'; t = open(p, encoding='utf-8').read()
    anchor = 'semio-framework-os-infinite-directed-presentation = { path = "../../🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/📦️packages/🦀️rust" }\n'
    assert t.count(anchor) == 1
    add = ''.join(f'{crate_pkg(c[0])} = {{ path = "../../{c[2]}/📦️packages/🦀️rust" }}\n' for c in CRATES if f'{crate_pkg(c[0])} =' not in t)
    comment = "# 🎲️ The board family lives in producer crates (schema, engine, normal model/scene/focus/gestures/fill/host, dag model/graph/host); `src/🦀️.rs` re-exports them under the former `board` paths.\n"
    if add: open(p, 'w', encoding='utf-8').write(t.replace(anchor, anchor + comment + add, 1))
    return add.count('\n')

FIXTURE_EDITS = [
 (INF + "🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧫️fixtures/🔗️wire-edit/🔣️.json",
  '"rust": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs"',
  '"rust": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🧪️tests/🔗️wire-edit/🦀️.rs"'),
 (R + "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🧫️fixtures/🔌️port-types/🔣️.json",
  "♾️infinite/🎲️board/🦀️.rs — Handle::value_types", "♾️infinite/🎲️board/⚙️engine/🦀️.rs — Handle::value_types"),
]

def patch_receivers():
    p = INF + "🎲️board/🚪️io/📝️text/🎯️dag-input/🧪️tests/🧫️fixtures/receivers.json"
    t = open(p, encoding='utf-8').read(); P = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/"
    old_sources = [
     f'    "{P}🧬️schema/🎯️dag-input/🦀️.rs",\n    "{P}🚪️io/📝️text/🎯️dag-input/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs",\n',
    ]
    new_sources = f'    "{P}🧬️schema/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🗃️model/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🕸️graph/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🧭️host/🦀️.rs",\n    "{P}🔌️ports/➡️directed/🕸️dag/🧭️host/🧪️tests/🔬️dag-unit/🦀️.rs",\n'
    assert t.count(old_sources[0]) == 1
    t = t.replace(old_sources[0], new_sources)
    sel = f'    "{P}🚪️io/📝️text/🎯️dag-input/📤️selection/🦀️.rs",\n'
    assert t.count(sel) == 1
    t = t.replace(sel, '')
    old = f'"file": "{P}🔌️ports/➡️directed/🕸️dag/🦀️.rs"'
    assert t.count(old) == 1
    t = t.replace(old, f'"file": "{P}🔌️ports/➡️directed/🕸️dag/🕸️graph/🦀️.rs"')
    open(p, 'w', encoding='utf-8').write(t)

def main(apply, dry):
    same_board, same_facade = check_preconditions(apply)
    print('live board tree unchanged since snapshot:', same_board, '| facade lib unchanged:', same_facade)
    if not (same_board and same_facade): raise SystemExit('live tree changed; regenerate')
    nf = new_files(apply)
    newrel = {k for k in nf if k.endswith('.rs')}
    dels, keeps = delete_set(apply, newrel)
    print('new files', len(nf), 'rs deletions', len(dels), 'kept crate roots', len(keeps))
    ext, used, unres = bfinal.externals()
    drift = [f for f in ext if not os.path.exists(f"{apply}/EXT/{f}") or open(f"{apply}/EXT/{f}", encoding='utf-8').read() != ext[f]]
    print('external files', len(ext), 'drift vs apply tree', len(drift), 'unresolved', unres)
    if dry: return
    log = {'created': [], 'updated': [], 'removed': [], 'external': []}
    for rel, src in sorted(nf.items()):
        dst = INF + rel
        existed = os.path.exists(dst)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)
        log['updated' if existed else 'created'].append(os.path.relpath(dst, R))
    for p in dels:
        os.remove(p); log['removed'].append(os.path.relpath(p, R))
    for dp, dn, fn in sorted(os.walk(INF + '🎲️board'), reverse=True):
        if not os.listdir(dp):
            os.rmdir(dp)
    shutil.copyfile(apply + '/INF/📦️packages/🦀️rust/🦀️.rs', INF + '📦️packages/🦀️rust/🦀️.rs')
    log['updated'].append(os.path.relpath(INF + '📦️packages/🦀️rust/🦀️.rs', R))
    print('facade manifest deps added', patch_facade_manifest())
    for f, nt in ext.items():
        open(R + f, 'w', encoding='utf-8').write(nt); log['external'].append(f)
    for path, old, new in FIXTURE_EDITS:
        t = open(path, encoding='utf-8').read(); assert t.count(old) == 1, path; open(path, 'w', encoding='utf-8').write(t.replace(old, new))
    patch_receivers()
    print('pointers', bpoint.main(False))
    cs = breg.crates()
    print('members', breg.reg_members(cs), 'owners', breg.reg_owners(cs), 'launch', breg.reg_launch(cs))
    json.dump(log, open(SC + '/../board-switch-log.json', 'w'), ensure_ascii=False, indent=1)
    print('done', {k: len(v) for k, v in log.items()})

if __name__ == '__main__':
    main(sys.argv[1], len(sys.argv) > 2 and sys.argv[2] == 'dry')
