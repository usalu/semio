"""🧪 uicarve wgpu test relocation: paint reference crate, owner test mounts -> companion crates, orphaned root tests -> laws."""
import json, os, re, sys, collections, subprocess
sys.path.insert(0, 'lib')
import carve, rs, peel, crategen
from carve import R, UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
step = sys.argv[1]
if step == 'peel-paintref':
    rep = peel.peel(W + '/🖌️paint/🦀️.rs', lambda it: it.kind in ('fn', 'const', 'static', 'struct', 'enum', 'type') and any(a.replace(' ', '') == '#[cfg(test)]' for a in it.attrs),
        W + '/🖌️paint/🔬️reference/🦀️.rs', '🔬️ Immediate-mode reference paint pipeline: the golden oracle the retained paint laws compare against.', 'crate::wgpu::paint', 'crate::wgpu::paint_reference',
        strip_cfgs=('#[cfg(test)]',), widen_all=True)
    P = json.load(open('lists/wgpu-peels.json'))
    P['paint_reference'] = rep
    json.dump(P, open('lists/wgpu-peels.json', 'w'), indent=1, ensure_ascii=False)
    print(len(rep['moved']), 'moved; used_from_rest', rep['used_from_rest'], 'used_by_rest', rep['used_by_rest'])
    sys.exit()
spec = carve.Spec('specs/wgpu.json')
OWNER_MOUNTS = {
 'input': ('📥️input', ['tests']),
 'layout': ('🧮️layout', ['tree_row_rect_tests', 'table_row_grid_tests']),
 'mounted': ('📌️mounted_layout/🎯️hit', ['retained_hit_target_tests']),
 'events': ('⚡️events', ['tests', 'control_commit_tests', 'scrub_press_tests']),
 'paint': ('🖌️paint', ['tests']),
 'engine': ('⚙️engine', ['tests', 'retained_document_hostile_fixtures', 'retained_section_collapse_tests', 'retained_select_origin_tests', 'original_presented_source_tests']),
}
def rel(frm_file, to_file):
    return os.path.relpath(to_file, os.path.dirname(frm_file))
def remove_mounts(path, names):
    src = rd(path)
    items = peel.top_items(src)
    metas = []; edits = []
    for it in items:
        if it.kind == 'mod' and it.name in names:
            pm = re.search(r'#\[path\s*=\s*"([^"]+)"\]', it.text)
            metas.append((it.name, os.path.normpath(os.path.join(os.path.dirname(path), pm.group(1)))))
            keep = [l for l in it.text.split('\n') if re.match(r'\s*//\s*#(end)?region\b', l)]
            edits.append((it.start, it.end, '\n'.join(keep)))
    new = rs.apply_edits(src, edits)
    wr(path, new)
    return metas
def header_uses_text(path):
    s = rd(path)
    out = []
    for u in peel.header_uses(s):
        out.append(s[u.astart:u.end])
    return out
def retarget(text, mount_names, owner_ident):
    toks = rs.lex(text); masked = rs.mask(text, toks)
    edits = []; use_spans = []
    for u in rs.find_uses(text, toks):
        use_spans.append((u.astart, u.end))
        leaves = rs.flatten_tree(u.tree); ch = False; out = []
        for p, a in leaves:
            segs = p.split('::')
            if segs[0] == 'crate' and len(segs) >= 2 and segs[1] not in mount_names:
                segs[0] = owner_ident; ch = True
            out.append(('::'.join(segs), a))
        if ch: edits.append((u.astart, u.end, rs.emit_use(u.vis, out, u.attrs)))
    for a, b, segs in rs.path_occurrences(masked, {'crate'}):
        if any(x <= a and b <= y for x, y in use_spans): continue
        if len(segs) >= 2 and segs[1] not in mount_names:
            edits.append((a, b, '::'.join([owner_ident] + segs[1:])))
    return rs.apply_edits(text, edits) if edits else text
GLOB = {'input': [], 'layout': ['layout'], 'mounted': ['hit']}
if step in ('companions', 'companions2'):
    for key in (('events', 'paint', 'engine') if step == 'companions' else ('input', 'layout', 'mounted')):
        d, names = OWNER_MOUNTS[key]
        owner_path = W + '/' + d + '/🦀️.rs'
        metas = remove_mounts(owner_path, names)
        lk = key + '-laws'
        c = spec.crates[lk]; root = spec.root_file(c)
        owner_ident = spec.crates[key]['ident']
        hdr = header_uses_text(owner_path)
        txt = '//! 🥣️ %s.\n\n#![cfg(test)]\n\n' % c['desc'][0].upper().__add__(c['desc'][1:]) if False else '//! 🥣️ ' + c['desc'] + '.\n\n#![cfg(test)]\n\n'
        txt += '\n'.join(hdr) + '\nuse %s::*;\n' % '::'.join([owner_ident] + GLOB.get(key, []))
        if key == 'engine': txt += 'use semio_framework_ui_wgpu_engine_surface::*;\n'
        txt += '\n'
        for name, p in metas:
            txt += '#[path = "%s"]\nmod %s;\n\n' % (rel(root, p), name)
        wr(root, txt)
        names_set = {n for n, _ in metas}
        files = carve.reachable(root)
        n = 0
        for f in files:
            if f == root or not f.endswith('.rs'): continue
            old = rd(f); new = retarget(old, names_set, owner_ident)
            if new != old: wr(f, new); n += 1
        # owner root: drop cfg(test)-only imports that only served the removed tests
        print(key, 'mounts', [m for m, _ in metas], 'retargeted files', n)
    sys.exit()
if step == 'laws':
    c = spec.crates['laws']; root = spec.root_file(c)
    old = rd('lists/wgpu-old-root.rs.txt')
    mounts_ = [(n, p) for n, p, a, pub in carve.mounts_from_text(old, W + '/🦀️.rs')] if hasattr(carve, 'mounts_from_text') else None
    sys.exit()
