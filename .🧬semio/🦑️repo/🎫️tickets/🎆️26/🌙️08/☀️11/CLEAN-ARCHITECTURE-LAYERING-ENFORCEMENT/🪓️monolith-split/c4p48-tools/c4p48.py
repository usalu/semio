#!/usr/bin/env python3
import os, re, sys, json, hashlib, collections, shutil, subprocess

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
from rsitems import lex, items as rs_items, name_kind
import widen2 as W

REPO = '/Users/ueli/Documents/semio'
SCR = os.environ.get('C4P48_SCR', '/Users/ueli/.cache/c4p48')
ORPH = re.compile(r'^\s*(//\s?#(end)?region\b|#!\[|//!)')
MARK = re.compile(r'^(\s*)//\s?#(end)?region\b(.*)$')
PINNED = ('use', 'crate')


def ident_set(text):
    s = set()
    for k, x, sl, el, si in lex(text):
        if k == 'id':
            s.add(x)
        elif k == 'str':
            for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*)', x):
                s.add(m.group(1))
    return s


class Doc:
    def __init__(self, text):
        self.text = text
        self.lines = text.split('\n')
        self.items = []
        self.regions = []
        self._parse()

    def _parse(self):
        lines = self.lines
        raw = rs_items(self.text)
        prev_e = -1
        its = []
        for r in raw:
            s = r['first_tok_line']
            e = r['end_line']
            j = s
            last = None
            while j <= e and (ORPH.match(lines[j]) or lines[j].strip() == '' or lines[j].lstrip().startswith('//')):
                if ORPH.match(lines[j]):
                    last = j
                j += 1
            if last is not None:
                s = last + 1
            while s <= e and lines[s].strip() == '':
                s += 1
            if s <= prev_e:
                s = prev_e + 1
            assert s <= e, ('empty item span', r['first_tok_line'] + 1, e + 1)
            try:
                vis, kind, name = name_kind(r['toks'])
            except Exception as ex:
                vis, kind, name = '', '?', ''
            its.append(dict(s=s, e=e, vis=vis, kind=kind, name=name, attrs=r['attrs']))
            prev_e = e
        self.items = its
        covered = [None] * len(lines)
        for ii, it in enumerate(its):
            for L in range(it['s'], it['e'] + 1):
                assert covered[L] is None, ('overlap', L + 1)
                covered[L] = ii
        stack = []
        region_at = [None] * (len(lines) + 1)
        for L, line in enumerate(lines):
            region_at[L] = tuple(stack)
            m = MARK.match(line)
            if m and covered[L] is None or (m and covered[L] is not None):
                if m.group(2) is None:
                    rid = len(self.regions)
                    self.regions.append(dict(id=rid, open=line.strip(), close=None, name=m.group(3).strip()))
                    stack.append(rid)
                else:
                    assert stack, ('endregion without region', L + 1)
                    rid = stack.pop()
                    self.regions[rid]['close'] = line.strip()
            elif covered[L] is None and line.strip() != '' and not ORPH.match(line) and not line.lstrip().startswith('//'):
                raise AssertionError(('uncovered non-blank line', L + 1, line[:80]))
        region_at[len(lines)] = tuple(stack)
        assert not stack, ('unbalanced regions at EOF', stack)
        for it in its:
            it['stack'] = region_at[it['s']]
            it['balanced'] = region_at[it['e'] + 1] == it['stack']
        self.covered = covered


def listing(doc):
    for ii, it in enumerate(doc.items):
        print('%d\t%d-%d\t%d\t%s\t%s\t%s\t%s' % (ii, it['s'] + 1, it['e'] + 1, it['e'] - it['s'] + 1, it['vis'], it['kind'], it['name'][:60], ' '.join(it['attrs'])[:50]))


def select(doc, ranges, who):
    chosen = []
    for a, b in ranges:
        for ii, it in enumerate(doc.items):
            s1, e1 = it['s'] + 1, it['e'] + 1
            if e1 < a or s1 > b:
                continue
            assert a <= s1 and e1 <= b, (who, 'range cuts an item', (a, b), (s1, e1), it['name'])
            chosen.append(ii)
    return sorted(set(chosen))


def vis_rank(it, nest=False):
    v = it['vis']
    if v == 'pub':
        return 4
    if v == 'pub(crate)':
        return 3
    if v == 'pub(super)' and nest:
        return 2
    return 1


def build(doc, spec):
    owner = {}
    plans = []
    for ci, c in enumerate(spec['children']):
        idxs = select(doc, c['ranges'], c['folder'])
        keep = []
        for ii in idxs:
            it = doc.items[ii]
            if it['kind'] in PINNED or (it['kind'] == 'mod' and doc.lines[it['e']].rstrip().endswith(';')):
                continue
            assert ii not in owner, ('item in two children', ii, it['name'])
            assert it['balanced'], ('moved item with unbalanced markers', it['s'] + 1, it['e'] + 1, it['name'])
            owner[ii] = ci
            keep.append(ii)
        assert keep, ('empty child', c['folder'])
        plans.append(keep)
    lines = doc.lines
    child_files = {}
    for ci, c in enumerate(spec['children']):
        out = [c['doc'], '']
        if c.get('glob', True):
            out.append('use super::*;')
        for u in c.get('uses', []):
            out.append(u)
        out.append('')
        cur = []
        first = True
        for ii in plans[ci]:
            it = doc.items[ii]
            target = list(it['stack'])
            p = 0
            while p < len(cur) and p < len(target) and cur[p] == target[p]:
                p += 1
            closers = [doc.regions[r]['close'] for r in reversed(cur[p:])]
            openers = [doc.regions[r]['open'] for r in target[p:]]
            if closers:
                while out and out[-1] == '':
                    out.pop()
                out.extend(closers)
            if not first or out[-1] != '':
                if out[-1] != '':
                    out.append('')
            out.extend(openers)
            out.extend(lines[it['s']:it['e'] + 1])
            cur = target
            first = False
        closers = [doc.regions[r]['close'] for r in reversed(cur)]
        out.extend(closers)
        while out and out[-1] == '':
            out.pop()
        child_files[c['folder']] = '\n'.join(out) + '\n'
    wiring = {}
    if spec.get('wiring', 'block') == 'block':
        wiring[min(doc.items[plans[ci][0]]['s'] for ci in range(len(plans)))] = list(range(len(plans)))
    else:
        for ci, c in enumerate(spec['children']):
            first_it = doc.items[plans[ci][0]]
            wiring[first_it['s']] = [ci]
    skip = set()
    for ii, ci in owner.items():
        it = doc.items[ii]
        for L in range(it['s'], it['e'] + 1):
            skip.add(L)
    out = []
    reex = {}
    for ci, c in enumerate(spec['children']):
        best = 1
        cfgs = set()
        named = 0
        for ii in plans[ci]:
            it = doc.items[ii]
            k = it['kind']
            if k in ('impl', 'macro', 'include') or not it['name'] and k in ('impl',):
                continue
            if k == 'impl':
                continue
            if it['name'] == '' and k not in ('mod',):
                r = 4
            else:
                r = vis_rank(it, spec.get('nest', False))
            named += 1
            best = max(best, r)
            cfgs.add(tuple(a for a in it['attrs'] if a.startswith('#[cfg(')))
        reex[ci] = (named, best, cfgs)
    for L, line in enumerate(lines):
        if L in wiring:
            wl = ['']
            for cj in wiring[L]:
                c = spec['children'][cj]
                if c.get('cfg_mod'):
                    wl.append(c['cfg_mod'] + TAG)
                wl.extend(['#[path = "%s/🦀️.rs"]' % c['folder'] + TAG, '%smod %s;' % (c.get('modvis', '') + (' ' if c.get('modvis') else ''), c['mod']) + TAG])
                named, best, cfgs = reex[cj]
                if c.get('reexport', True) and named:
                    v = {4: 'pub use', 3: 'pub(crate) use', 2: 'pub(super) use', 1: 'use'}[best]
                    if len(cfgs) == 1 and list(cfgs)[0] and all(len(x) == 1 for x in cfgs):
                        wl.append(list(cfgs)[0][0] + TAG)
                    wl.append('%s %s::*;' % (v, c['mod']) + TAG)
                wl.append('')
            out.extend(wl)
        if L in skip:
            continue
        out.append(line)
    ptxt = collapse(out)
    return ptxt, child_files, plans


def collapse(lines):
    out = []
    blank = 0
    for l in lines:
        if l.strip() == '':
            blank += 1
            if blank > 1:
                continue
            out.append('')
        else:
            blank = 0
            out.append(l)
    return '\n'.join(out)


TAG = '\x00W'
RGN = re.compile(r'^\s*//\s?#(end)?region\b')


def prune_empty_regions(text, keep=()):
    lines = text.split('\n')
    changed = True
    removed = 0
    while changed:
        changed = False
        stack = []
        for i, l in enumerate(lines):
            m = RGN.match(l)
            if not m:
                continue
            if not m.group(1):
                stack.append(i)
            else:
                j = stack.pop()
                body = lines[j + 1:i]
                if all(b.strip() == '' or b.endswith(TAG) for b in body) and not any(lines[j].strip().endswith(k) for k in keep):
                    del lines[i]
                    del lines[j]
                    removed += 1
                    changed = True
                    break
    return '\n'.join(lines).replace(TAG, ''), removed


def widen_children(ptxt, child_files, external_texts, report=None, nest=False):
    all_ids = {'__parent__': W.ident_set(ptxt)}
    for f, t in child_files.items():
        all_ids[f] = W.ident_set(t)
    ext = set()
    for t in external_texts:
        ext |= W.ident_set(t)
    res = {}
    for f, t in child_files.items():
        others = set(ext)
        for g, s in all_ids.items():
            if g != f:
                others |= s
        ed = W.plan(t, others, nest)
        res[f] = W.apply(t, ed)
        if report is not None:
            for e in ed:
                report.append((f,) + tuple(e[3]))
    return res


SUP = re.compile(r'(?<!\w)(?<!::)super::(?!\*)')


def rebase_super(text, who):
    out = []
    for l in text.split('\n'):
        s = l.lstrip()
        if s.startswith('//') or s.startswith('use super::*;'):
            out.append(l)
            continue
        assert 'super::super::' not in l, (who, 'already rebased path', l[:100])
        out.append(SUP.sub('super::super::', l))
    return '\n'.join(out)


INC = re.compile(r'(include_str!|include_bytes!)\(\s*"([^"\\]+)"')


def rewrite_includes(text, depth):
    def sub(m):
        path = m.group(2)
        if path.startswith('/') or ':' in path:
            return m.group(0)
        return '%s("%s%s"' % (m.group(1), '../' * depth, path)
    return INC.sub(sub, text)


HDR = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z0-9_]+)\s*\{\s*$')


def inline_body(lines, it, indent, who):
    h = None
    for L in range(it['s'], it['e'] + 1):
        if HDR.match(lines[L]):
            h = L
            break
    assert h is not None, (who, 'no inline mod header', it['s'] + 1)
    assert lines[it['e']].strip() == '}', (who, 'mod does not end with a closing brace', it['e'] + 1)
    ded = []
    for l in lines[h + 1:it['e']]:
        if l.strip() == '':
            ded.append('')
        else:
            assert l.startswith(' ' * indent), (who, 'inner line not indented', l[:60])
            ded.append(l[indent:])
    while ded and ded[0] == '':
        ded.pop(0)
    while ded and ded[-1] == '':
        ded.pop()
    pre = lines[it['s']:h]
    head = HDR.match(lines[h]).group(2).strip() + ';'
    return pre, head, ded, lines[h]


def layer(doc, children_spec, ext, nest, depth_base, rep):
    ptxt, child_files, plans = build(doc, children_spec)
    child_files = widen_children(ptxt, child_files, ext, rep, nest)
    for f, t in list(child_files.items()):
        child_files[f] = rewrite_includes(t, f.count('/') + 1 + depth_base)
        if children_spec.get('rebase_super'):
            child_files[f] = rebase_super(child_files[f], f)
    return ptxt, child_files


def stage(tag, spec_module):
    os.makedirs(os.path.join(SCR, 'stage'), exist_ok=True)
    sp = spec_module.SPEC
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    if sp.get('pre'):
        orig = sp['pre'](orig)
    doc = Doc(orig)
    ext = []
    for p in sp.get('external', []):
        ext.append(open(os.path.join(REPO, p), encoding='utf-8').read())
    rep = []
    files = {}
    inn = sp.get('inner')
    if inn:
        cands = [i for i, it in enumerate(doc.items) if it['kind'] == 'mod' and it['name'] == inn['mod'] and not doc.lines[it['e']].rstrip().endswith(';')]
        assert len(cands) == 1, ('inline mod not unique', inn['mod'], cands)
        it = doc.items[cands[0]]
        assert it['balanced'], 'inline mod with unbalanced markers'
        pre, head, body, hdr_line = inline_body(doc.lines, it, inn.get('indent', 0), tag)
        inner_doc = Doc('\n'.join(body))
        ptxt_in, grand = layer(inner_doc, inn, [], inn.get('nest', False), 1, rep)
        for f, t in grand.items():
            files[inn['folder'] + '/' + f] = t
        shell = inn['doc'] + '\n\n' + ptxt_in.lstrip('\n')
        shell, _ = prune_empty_regions(shell, ())
        shell = collapse(shell.split('\n'))
        files[inn['folder']] = shell if shell.endswith('\n') else shell + '\n'
        wl = pre + ['#[path = "%s/🦀️.rs"]' % inn['folder'], head]
        lines = doc.lines[:it['s']] + wl + doc.lines[it['e'] + 1:]
        outer_doc = Doc('\n'.join(lines))
    else:
        outer_doc = doc
    ptxt, child_files = layer(outer_doc, sp, ext, sp.get('nest', False), 0, rep)
    files.update(child_files)
    for f, lst in sp.get('patch', {}).items():
        for old_, new_ in lst:
            t = files[f]
            assert t.count(old_) == 1, (f, old_[:80], t.count(old_))
            files[f] = t.replace(old_, new_)
    for f_, ln_, old_, new_ in sp.get('vis_edits', []):
        lines_ = files[f_].split('\n')
        assert lines_[ln_ - 1] == old_, (f_, ln_, lines_[ln_ - 1], old_)
        lines_[ln_ - 1] = new_
        files[f_] = '\n'.join(lines_)
    for old_, new_ in sp.get('patch_parent', []):
        assert ptxt.count(old_) == 1, ('parent', old_[:80], ptxt.count(old_))
        ptxt = ptxt.replace(old_, new_)
    ptxt, nrem = prune_empty_regions(ptxt, sp.get('keep_empty', ()))
    ptxt = collapse(ptxt.split('\n'))
    d = os.path.join(SCR, 'stage', tag)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    open(os.path.join(d, '🦀️.rs'), 'w', encoding='utf-8').write(ptxt + ('\n' if not ptxt.endswith('\n') else ''))
    for f, t in files.items():
        os.makedirs(os.path.join(d, f))
        open(os.path.join(d, f, '🦀️.rs'), 'w', encoding='utf-8').write(t)
    with open(os.path.join(SCR, 'stage', tag + '.widen.tsv'), 'w', encoding='utf-8') as fh:
        for r in rep:
            fh.write('\t'.join(r) + '\n')
    sizes = {f: t.count('\n') for f, t in files.items()}
    print('staged', tag, 'parent', ptxt.count('\n') + 1, 'children', sizes, 'widened', len(rep))
    return d


def rel_folders(sp):
    out = [c['folder'] for c in sp['children']]
    inn = sp.get('inner')
    if inn:
        out.append(inn['folder'])
        out.extend(inn['folder'] + '/' + c['folder'] for c in inn['children'])
    return out


def norm_line(l):
    return re.sub(r'(include_(?:str|bytes)!\()"(?:\.\./)+', r'\1"', l.strip().replace('pub(in super::super) ', '').replace('pub(super) ', '').replace('super::super::', 'super::'))


def lossless(tag, spec_module):
    sp = spec_module.SPEC
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    d = os.path.join(SCR, 'stage', tag) if len(sys.argv) < 4 else sys.argv[3]
    files = [os.path.join(d, '🦀️.rs')] + [os.path.join(d, f, '🦀️.rs') for f in rel_folders(sp)]
    O = collections.Counter(norm_line(l) for l in orig.split('\n') if l.strip())
    vis_map = {norm_line(n_): norm_line(o_) for _, _, o_, n_ in sp.get('vis_edits', [])}
    N = collections.Counter()
    per = {}
    for f in files:
        c = collections.Counter(vis_map.get(norm_line(l), norm_line(l)) for l in open(f, encoding='utf-8').read().split('\n') if l.strip())
        per[f] = c
        N += c
    miss = O - N
    extra = N - O
    for l in sp.get('expect_missing', []):
        k = norm_line(l)
        if miss.get(k):
            print('  declared-missing', k[:120])
            miss[k] -= 1
    miss = +miss
    declared_extra = collections.Counter(norm_line(l) for l in sp.get('expect_extra', []))
    print('files', len(files), 'orig non-blank', sum(O.values()), 'new non-blank', sum(N.values()))
    print('missing', sum(miss.values()))
    for l, c in sorted(miss.items()):
        print('  -', c, l[:200])
    print('extra', sum(extra.values()))
    allowed = []
    pre_extra = collections.Counter(sp.get('pre_extra', []))
    for l, c in sorted(extra.items()):
        tag2 = ''
        if pre_extra.get(l, 0) >= c:
            tag2 = 'pre-edit'
        elif declared_extra.get(l, 0) >= c:
            tag2 = 'declared'
        elif l.startswith('//!'):
            tag2 = 'doc'
        elif l == 'use super::*;':
            tag2 = 'glob'
        elif re.match(r'^#\[path = ".*/🦀️\.rs"\]$', l):
            tag2 = 'path'
        elif re.match(r'^(pub )?mod [a-z0-9_]+;$', l):
            tag2 = 'mod'
        elif re.match(r'^(pub(\((crate|super)\))? )?use [a-z0-9_]+::\*;$', l):
            tag2 = 'reexport'
        elif re.match(r'^//\s?#(end)?region', l):
            tag2 = 'marker'
        elif l.startswith('#[cfg('):
            tag2 = 'cfg-on-reexport'
        allowed.append((tag2, c, l))
    for t2, c, l in allowed:
        print('  +', t2 or 'UNEXPLAINED', c, l[:160])
    bad = [a for a in allowed if not a[0]]
    rr = re.compile(r'^\s*//\s?#region\b', re.M)
    re_ = re.compile(r'^\s*//\s?#endregion\b', re.M)
    mo = len(rr.findall(orig))
    mn = sum(len(rr.findall(open(f, encoding='utf-8').read())) for f in files)
    eo = len(re_.findall(orig))
    en = sum(len(re_.findall(open(f, encoding='utf-8').read())) for f in files)
    print('regions orig/new', mo, mn, 'endregions orig/new', eo, en)
    ok = not miss and not bad and mn == en
    print('LOSSLESS', 'OK' if ok else 'FAIL')
    return ok


def paths_check(tag, spec_module):
    sp = spec_module.SPEC
    base = os.path.dirname(sp['file'])
    ok = True
    for f in rel_folders(sp):
        rel = os.path.join(base, f, '🦀️.rs')
        n = len(rel.encode('utf-8'))
        nabs = len((REPO + '/' + rel).encode('utf-8'))
        print('path rel-bytes', n, 'abs-bytes', nabs, 'abs-chars', len(REPO + '/' + rel), rel[-70:])
        if n > 240 or len(REPO + '/' + rel) > 256:
            print('  TOO LONG (repo-relative bytes > 240 or absolute chars > 256)')
            ok = False
    return ok


def install(tag, spec_module):
    sp = spec_module.SPEC
    d = os.path.join(SCR, 'stage', tag)
    target = os.path.join(REPO, sp['file'])
    cur = open(target, 'rb').read()
    orig = open(os.path.join(T, 'orig', sp['orig']), 'rb').read()
    assert cur == orig, 'repo file differs from the recorded original; refusing to overwrite'
    base = os.path.dirname(target)
    for f in rel_folders(sp):
        cd = os.path.join(base, f)
        assert not os.path.exists(cd), ('child folder exists', cd)
    for f in rel_folders(sp):
        cd = os.path.join(base, f)
        os.makedirs(cd)
        shutil.copyfile(os.path.join(d, f, '🦀️.rs'), os.path.join(cd, '🦀️.rs'))
    tmp = target + '.pfa-tmp'
    shutil.copyfile(os.path.join(d, '🦀️.rs'), tmp)
    os.replace(tmp, target)
    print('installed', tag)


if __name__ == '__main__':
    import importlib.util
    cmd = sys.argv[1]
    tag = sys.argv[2]
    if cmd == 'list':
        sp = importlib.import_module('specs.' + tag).SPEC
        txt = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
        if sp.get('pre'):
            txt = sp['pre'](txt)
        listing(Doc(txt))
        sys.exit(0)
    mod = importlib.import_module('specs.' + tag)
    if cmd == 'stage':
        stage(tag, mod)
    elif cmd == 'lossless':
        sys.exit(0 if lossless(tag, mod) else 1)
    elif cmd == 'paths':
        sys.exit(0 if paths_check(tag, mod) else 1)
    elif cmd == 'install':
        install(tag, mod)
