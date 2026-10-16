"""🏗️ Cross-package lifts (plan 10.4.3 rule 2): cuts named items into `T/<destination folder>/📥️lifted/<source slug>/🦀️.rs` and mounts that file from the source module, so the module path of every item is unchanged."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
ROOT = os.environ['ROOT']; V = '️'
rd = lambda p: open(p, encoding='utf8').read()
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
CREATED = os.path.dirname(os.path.abspath(__file__)) + '/created-outside.txt'

def lift(src, specs, dest_folder, slug, mod, doc, region=None, reexport=True):
    text = rd(src)
    rest, moved = mv.cut(text, specs, strict=False)
    missing = [s for s in specs if not any(mv.matches(it, s) for it in mv.split_items(text) if it.kind != 'marker')]
    assert not missing, missing
    body = '\n\n'.join(moved)
    other = mv.idents(rest)
    body = re.sub(r'\bpub\(super\)', 'pub(crate)', body)
    body = mv.widen(body, {n for n in mv.names_private(body) if n in other})
    d = f'{os.environ["T"]}/{dest_folder}/📥{V}lifted/{slug}'
    dst = d + f'/🦀{V}.rs'
    assert not os.path.exists(dst), dst
    os.makedirs(d)
    reg = (f'\n//#region {region}\n', f'\n//#endregion {region}\n') if region else ('\n', '\n')
    w(dst, doc + '\nuse super::*;\n' + reg[0] + body + reg[1])
    relp = os.path.relpath(dst, os.path.dirname(src))
    assert len(os.path.relpath(dst, ROOT).encode()) <= 240
    mount = f'\n#[path = "{relp}"]\nmod {mod};\npub(crate) use {mod}::*;\n'
    w(src, rest.rstrip('\n') + '\n' + mount)
    rel = os.path.relpath(os.path.dirname(os.path.dirname(dst)), ROOT)
    known = [l for l in rd(CREATED).split('\n') if l]
    if rel not in known: w(CREATED, '\n'.join(known + [rel]) + '\n')
    return len(moved), sum(len(x.split('\n')) for x in moved), dst
