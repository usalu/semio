# ✂️ WP4 step S0: cuts the ShellState members (Appendix B) and the WP5-bound free items out of the 20 Shell chunk files into staging modules.
# usage: s0.py <out-dir>   (writes the full new Shell element tree into <out-dir>, a sandbox outside the repo; nothing in the repo is touched)
import re, sys, os, json, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from assign import *

CRATE_FOLDER = {}
for m in re.finditer(r"^\| \d+ \| `(shell-[a-z-]+)` \| `([^`]+)` \|", plan, re.M):
    CRATE_FOLDER[m.group(1)] = m.group(2)

DROP_NAMES = {'HOST_IO_REQUEST_TRACE', 'HostIoRequestTraceGuard', 'host_io_request_trace_label', 'host_io_call'}
DROPPED = []

VIS_RX = re.compile(r'^(pub(?:\s*\([^)]*\))?\s+)')
def widen_head(line):
    m = VIS_RX.match(line.lstrip())
    ind = line[:len(line) - len(line.lstrip())]
    body = line.lstrip()
    if m:
        v = m.group(1).strip()
        if v == 'pub' or v.replace(' ', '') in ('pub(crate)', 'pub(super)'): return line
        if v.replace(' ', '') == 'pub(self)': return ind + 'pub(super) ' + body[len(m.group(1)):]
        return line
    return ind + 'pub(super) ' + body

class Region:
    def __init__(self, idx, text, name): self.idx = idx; self.text = text; self.name = name; self.end_text = None; self.end_idx = None

def regions_of(src):
    stack = []; snap = {}; allr = []
    for i, l in enumerate(src.lines):
        snap[i] = list(stack)
        m = REGION.match(l)
        if m and src.bl[i].strip() == '':
            if m.group(1) == 'region':
                r = Region(i, l, m.group(2).strip()); stack.append(r); allr.append(r)
            else:
                assert stack, ('unbalanced endregion', i)
                r = stack.pop(); r.end_text = l; r.end_idx = i
    assert not stack, 'unbalanced region'
    return snap, allr

def transition(cur, new, out):
    k = 0
    while k < len(cur) and k < len(new) and cur[k] is new[k]: k += 1
    for r in reversed(cur[k:]): out.append(r.end_text)
    for r in new[k:]: out.append(r.text)
    return list(new)

def ident_set(text): return set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', text))

def file_uses(f):
    src = f['src']; out = []
    if f['slug'] == '🦀️.rs': return out
    for u in f['units']:
        if u.kind == 'use':
            t = '\n'.join(src.lines[u.first:u.last + 1])
            if re.fullmatch(r'\s*use super::\*;\s*', t): continue
            out.append(t)
    return out

def parent_vis(parent_text):
    vis = {}
    for m in re.finditer(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod (\w+);\n(pub(?:\([a-z]+\))? )?use (\w+)::\*;', parent_text):
        vis[m.group(1)] = (m.group(3) or '').strip()
    return vis

def main(outdir):
    home, files, items = analyse()
    import gravity
    overrides = {n: c for n, c in home.items() if n in ('tutorial_capture_camera_pose','tutorial_apply_camera_pose','tutorial_capture_ui_snapshot','tutorial_apply_ui_snapshot','tutorial_apply_ui_change_to_shell','tutorial_resolve_gesture_point','render_tutorial_gesture_overlay_node','tutorial_recorder_sample','persist_ui_preferences')}
    _f, _i, bytype, impls, h, RKg, _u, _su = gravity.run(overrides=overrides, loaded=(home, files, items))
    INF = 10**6
    keep = set()
    for f in files:
        for u in f['units']:
            if u.kind == 'mod' and u.name in ('labels', 'refusals') and f['slug'].startswith('⏪️'): keep.add(f['slug'])
    ptext = open(SH + '/🦀️.rs', encoding='utf-8').read()
    modnames = set(re.findall(r'#\[path = "[^"]+/🦀️\.rs"\]\n(?:pub(?:\([a-z]+\))? )?mod (\w+);', ptext))
    qual = collections.defaultdict(set)
    for f in files:
        bt = f['src'].b
        for m in re.finditer(r'(?<![\w])(' + '|'.join(sorted(modnames)) + r')\s*::\s*(?:\{([^}]*)\}|(\w+))', bt):
            for nm in re.findall(r'\w+', m.group(2) or m.group(3) or ''): qual[m.group(1)].add(nm)
    slug2mod = {}
    for m in re.finditer(r'#\[path = "([^"]+)/🦀️\.rs"\]\n(?:pub(?:\([a-z]+\))? )?mod (\w+);', ptext): slug2mod[m.group(1)] = m.group(2)
    childmods = collections.defaultdict(list)
    for f in files:
        for u in f['units']:
            if u.kind == 'mod' and u.name and not u.name.endswith('tests') and not u.name.endswith('_tests'):
                head = '\n'.join(f['src'].lines[u.first:u.code + 1])
                if 'cfg(test)' not in head and 'cfg(all(test' not in head: childmods[f['slug']].append(u.name)
    for u in items:
        if u.is_member: continue
        u.crate = None
    for n, us in bytype.items():
        for u in us:
            r = h[id(u)]
            if r >= INF: continue
            if u.f['slug'] in keep and re.search(r'\b(labels|refusals)::', u.text or ''): continue
            if u.name in qual.get(slug2mod.get(u.f['slug']), ()): continue
            if re.search(r'(?<![\w])(' + '|'.join(childmods[u.f['slug']]) + r')\s*::', u.text or '') if childmods[u.f['slug']] else False: continue
            u.crate = RANK[r]
    DROPPED.clear()
    for f in files:
        for u in f['units']:
            if not u.state_impl and u.kind not in ('use', 'mod') and (u.name in DROP_NAMES):
                u.crate = None; DROPPED.append((f, u))
    for n, ims in impls.items():
        tus = bytype[n]
        for im in ims:
            same = [t_ for t_ in tus if t_.f['slug'] == im.f['slug']] or tus
            im.crate = getattr(same[0], 'crate', None)
            if n in DROP_NAMES: DROPPED.append((im.f, im))
    # destination per unit
    for f in files:
        f['staged'] = collections.defaultdict(list)   # crate -> list of pieces (ordered)
    stats = collections.Counter()
    repo_shell = SH
    ppath = SH + '/🦀️.rs'
    ptext = open(ppath, encoding='utf-8').read()
    pvis = parent_vis(ptext)
    modules = []   # (crate, slug, relpath, modname, has_free)
    new_text = {}  # relpath(abs original path) -> new text
    stage_text = {}  # relpath under Shell root -> text
    widen_defs = []  # filled later
    # pass 1: build staging file bodies (without widening), remember per-unit placement
    placement = {}   # id(unit) -> module id string
    for f in files:
        src = f['src']
        snap, allr = regions_of(src)
        f['snap'] = snap; f['allr'] = allr
        crates = collections.OrderedDict()
        for u in f['units']:
            if u.state_impl:
                for m in u.members: crates.setdefault(m.crate, None)
            elif getattr(u, 'crate', None): crates.setdefault(u.crate, None)
        f['crates'] = list(crates)
    out = {}
    for f in files:
        src = f['src']; snap = f['snap']
        uses = file_uses(f)
        for c in f['crates']:
            body = []
            cur_r = []
            pieces = 0; free = False
            for u in f['units']:
                if u.state_impl:
                    ms = [m for m in u.members if m.crate == c]
                    if not ms: continue
                    outer = [r for r in snap[ms[0].first] if r.idx < u.code]
                    for m in ms:
                        assert [r for r in snap[m.first] if r.idx < u.code] == outer, (f['slug'], m.name)
                        for r in snap[m.first]:
                            if r.idx < u.code: assert r.end_idx > u.last, ('region crosses impl', f['slug'], r.text)
                            else: assert r.end_idx < u.last, ('region crosses impl', f['slug'], r.text)
                    cur_r = transition(cur_r, outer, body)
                    head = src.lines[u.first:u.code + 1]
                    assert head[-1].rstrip().endswith('{'), (f['slug'], head[-1])
                    body += head
                    inner_cur = []
                    first_m = True
                    for m in ms:
                        inner = [r for r in snap[m.first] if r.idx >= u.code]
                        if not first_m: body.append('')
                        first_m = False
                        inner_cur = transition(inner_cur, inner, body)
                        lines = list(src.lines[m.first:m.last + 1])
                        lines[m.code - m.first] = widen_head(lines[m.code - m.first])
                        body += m.lead + lines + m.trail
                        placement[id(m)] = (f['slug'], c)
                        pieces += 1
                    inner_cur = transition(inner_cur, [], body)
                    body.append('}')
                    body.append('')
                elif getattr(u, 'crate', None) == c:
                    outer = list(snap[u.first])
                    cur_r = transition(cur_r, outer, body)
                    lines = list(src.lines[u.first:u.last + 1])
                    lines[u.code - u.first] = lines[u.code - u.first]
                    body += lines
                    body.append('')
                    placement[id(u)] = (f['slug'], c)
                    pieces += 1; free = True
            cur_r = transition(cur_r, [], body)
            while body and body[-1] == '': body.pop()
            out[(f['slug'], c)] = dict(body=body, free=free, pieces=pieces, uses=uses)
    return home, files, items, out, placement, ptext, pvis, childmods

# ---- writer -----------------------------------------------------------------------------------------------------------

REL_LIT = re.compile(r'((?:include_str!|include_bytes!|include!)\s*\(\s*|#\[\s*path\s*=\s*)"([^"\n]+)"')

def rebase_paths(text, old_dir, new_dir):
    def sub(m):
        lit = m.group(2)
        if lit.startswith('/') or not (lit.startswith('.') or '/' in lit or lit.endswith('.rs') or lit.endswith('.json')): return m.group(0)
        absolute = os.path.normpath(os.path.join(old_dir, lit))
        return m.group(1) + '"' + os.path.relpath(absolute, new_dir) + '"'
    return REL_LIT.sub(sub, text)

def first_emoji(folder):
    return folder[:2] if len(folder) > 1 and folder[1] == '️' else folder[:1]

def snake(s):
    return re.sub(r'[^a-z0-9]+', '_', re.sub(r'[^\x00-\x7f]', '', s).lower()).strip('_')

def ascii_slug(s): return re.sub(r'[^\x00-\x7f]', '', s)

def build(outdir):
    home, files, items, out, placement, ptext, pvis, childmods = main(None)
    root = os.path.realpath(outdir)
    assert root.startswith('/private/tmp/') or root.startswith('/tmp/'), root
    if os.path.exists(root): shutil.rmtree(root)
    os.makedirs(root)
    subprocess_rsync(SH, root)
    new_files = {}   # abs path under root -> text
    removed_by_file = collections.defaultdict(list)
    for f in files:
        for u in f['units']:
            if u.state_impl:
                assert all(id(m) in placement for m in u.members), (f['slug'], 'unplaced member')
                removed_by_file[f['slug']].append((u.first, u.last))
            elif id(u) in placement:
                removed_by_file[f['slug']].append((u.first, u.last))
    orig_mod = {}
    for m in re.finditer(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod (\w+);', ptext): orig_mod[m.group(1)] = m.group(2)
    f_has_free = {}
    for f in files:
        f_has_free[f['slug']] = any((not u.state_impl) and u.kind not in ('use', 'mod') and id(u) not in placement for u in f['units'])
    for f_, u_ in DROPPED:
        removed_by_file[f_['slug']].append((u_.first, u_.last))
    # staging files
    mounts = []
    widen_mods = set()
    for (slug, c), v in sorted(out.items(), key=lambda kv: (RANK.index(kv[0][1]), kv[0][0])):
        folder = CRATE_FOLDER[c].replace('shell-', '', 1)
        sdir = '🐚️shell-parent' if slug == '🦀️.rs' else slug
        rel = f'📥️staging/{folder}/{sdir}/🦀️.rs'
        sdir0 = '🐚️shell-parent' if slug == '🦀️.rs' else slug
        old_dir = SH_LIVE if slug == '🦀️.rs' else os.path.join(SH_LIVE, slug)
        new_dir = os.path.join(SH_LIVE, '📥️staging', CRATE_FOLDER[c].replace('shell-', '', 1), sdir0)
        body = rebase_paths('\n'.join(v['body']), old_dir, new_dir)
        ids = ident_set(body)
        uses = []
        for ut in v['uses']:
            cand_ids = ident_set(re.sub(r'\b(use|crate|super|self|pub|as|std)\b', ' ', ut))
            if cand_ids & ids: uses.append(ut)
        emoji = first_emoji(folder)
        sl = ascii_slug(sdir).strip('-')
        extra_imports = []
        if slug not in pvis and f_has_free.get(slug) and slug != '🦀️.rs':
            extra_imports.append(f'use super::{orig_mod[slug]}::*;')
        for cm in childmods.get(slug, ()):
            if slug != '🦀️.rs' and re.search(r'(?<![\w])' + cm + r'\s*::', body):
                extra_imports.append(f'use super::{orig_mod[slug]}::{cm};')
                widen_mods.add((slug, cm))
        head = [f'//! {emoji} `{c}` staging of `{sl}`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.',
                '//!',
                '//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.',
                '', 'use super::*;'] + extra_imports
        text = '\n'.join(head + uses + [''] + [body]) + '\n'
        p = os.path.join(root, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        new_files[p] = text
        mods = f'staged_{snake(c.replace("shell-", ""))}__{snake(sl)}'
        full = (SH_LIVE + '/' + rel)[len(R) + 1:]
        assert len(full.encode()) <= 240, (len(full.encode()), full)
        orig_vis = None
        for k, vv in pvis.items():
            if k == slug: orig_vis = vv
        mounts.append((c, rel, mods, v['free'], orig_vis))
    # original files
    for f in files:
        if f['slug'] not in removed_by_file: continue
        src = f['src']
        rem = removed_by_file[f['slug']]
        dead = set()
        for a, b in rem:
            dead.update(range(a, b + 1))
        outl = []; gap = False
        for i, l in enumerate(src.lines):
            if i in dead: gap = True; continue
            if gap and l.strip() == '' and outl and outl[-1].strip() == '': continue
            if l.strip() != '': gap = False
            outl.append(l)
        changed = True
        while changed:
            changed = False
            for i, l in enumerate(outl):
                m = REGION.match(l)
                if m and m.group(1) == 'region':
                    j = i + 1
                    while j < len(outl) and outl[j].strip() == '': j += 1
                    if j < len(outl):
                        m2 = REGION.match(outl[j])
                        if m2 and m2.group(1) == 'endregion' and m2.group(2).strip() == m.group(2).strip():
                            del outl[i:j + 1]; changed = True; break
        while outl and outl[-1].strip() == '': outl.pop()
        txt = '\n'.join(outl) + '\n'
        if not re.search(r'(?m)^use super::\*;', txt) and f['slug'] != '🦀️.rs':
            txt, n2 = re.subn(r'(?m)^use super::\{[^}]*\};', 'use super::*;', txt, count=1)
            assert n2 == 1, ('no super import to widen', f['slug'])
        for (sl_, cm_) in widen_mods:
            if sl_ == f['slug']:
                txt, n_ = re.subn(r'(?m)^(mod ' + cm_ + r';)', r'pub(super) \1', txt)
                if n_ == 0: txt, n_ = re.subn(r'(?m)^(    mod ' + cm_ + r';)', r'    pub(super) mod ' + cm_ + ';', txt)
                assert n_ >= 1, ('mod decl not found', sl_, cm_)
        new_files[os.path.join(root, os.path.relpath(f['path'], SH))] = txt
    # parent
    block = ['//#region 📥️RendererWp4Staging', '// 🪓️ Step S0 of PLAN-RENDERER-WGPU-CRATES.md: the ShellState members and WP5-bound free items by crate, one module per (crate, chunk).']
    for c, rel, mods, free, vis in mounts:
        block.append(f'#[path = "{rel}"]')
        block.append(f'mod {mods};')
        if free:
            block.append(f'{(vis + " ") if vis else "pub(crate) "}use {mods}::*;')
    block.append('pub(crate) use semio_framework_os_renderer_wgpu_host_base::host_io_call;')
    block += ['#[cfg(test)]', '#[path = "../../🧪️tests/🧰️section-fixtures/🦀️.rs"]', 'pub(crate) mod section_fixtures;']
    block.append('//#endregion 📥️RendererWp4Staging')
    anchor = '#[cfg(test)]\n/// 🔎️ The whole Shell module as the one text'
    ptext = new_files.get(os.path.join(root, '🦀️.rs'), ptext)
    assert ptext.count(anchor) == 1
    pt = ptext.replace(anchor, '\n'.join(block) + '\n\n' + anchor)
    a2 = '    include_str!("🎨️theme-file-import-and-accessibility/🦀️.rs")\n);'
    assert pt.count(a2) == 1
    inc = ',\n'.join(f'    include_str!("{rel}")' for c, rel, mods, free, vis in mounts)
    pt = pt.replace(a2, '    include_str!("🎨️theme-file-import-and-accessibility/🦀️.rs"),\n' + inc + '\n);')
    new_files[os.path.join(root, '🦀️.rs')] = pt
    from widen import apply_widening
    tests = {}
    for f in files:
        d = os.path.dirname(f['path'])
        key = os.path.join(root, os.path.relpath(f['path'], SH))
        lst = []
        for u in f['units']:
            if u.kind == 'mod':
                head = '\n'.join(f['src'].lines[u.first:u.code + 1])
                m = re.search(r'#\[path\s*=\s*"([^"]+)"\]', head)
                if m and m.group(1).startswith('../'):
                    tp = os.path.normpath(os.path.join(d, m.group(1)))
                    if os.path.exists(tp): lst.append(open(tp, encoding='utf-8').read())
        tests[key] = lst
    parent_key = os.path.join(root, '🦀️.rs')
    allf = {}
    for f in files:
        allf[os.path.join(root, os.path.relpath(f['path'], SH))] = f['src'].raw
    allf.update(new_files)
    wnew, report, detail = apply_widening(allf, tests)
    new_files.update(wnew)
    print('widened', dict(report), len(detail))
    json.dump([[os.path.relpath(a, root), b, c] for a, b, c in detail], open(GEN + '/rwp4-s0-widened.json', 'w'), ensure_ascii=False, indent=0)
    return root, files, new_files, mounts, out, placement

def subprocess_rsync(a, b):
    import subprocess
    subprocess.check_call(['rsync', '-a', '--exclude', 'node_modules', a + '/', b + '/'])

if __name__ == '__main__' and len(sys.argv) > 2 and sys.argv[1] == 'build':
    root, files, new_files, mounts, out, placement = build(sys.argv[2])
    for p, t in new_files.items():
        assert os.path.realpath(p).startswith(root + '/'), p
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(t)
    print(len(new_files), 'files written to', root, len(mounts), 'mounts')
