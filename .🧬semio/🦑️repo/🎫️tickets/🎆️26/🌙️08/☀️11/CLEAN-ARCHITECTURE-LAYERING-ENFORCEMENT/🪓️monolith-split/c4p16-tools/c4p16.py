#!/usr/bin/env python3
import os, re, sys, json, hashlib, collections, shutil, subprocess

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
from rsitems import lex, items as rs_items, name_kind
import widen2 as W

REPO = '/Users/ueli/Documents/semio'
SCR = os.environ.get('C4P16_SCR', '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p16')
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
    def __init__(self, text, off=0):
        self.text = text
        self.off = off
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
            first_doc = None
            while j <= e and (ORPH.match(lines[j]) or lines[j].strip() == '' or lines[j].lstrip().startswith('//')):
                if ORPH.match(lines[j]):
                    last = j
                elif first_doc is None and lines[j].lstrip().startswith('///'):
                    first_doc = j
                j += 1
            if last is not None:
                s = first_doc if first_doc is not None and first_doc < last else last + 1
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
            elif covered[L] is None and line.strip() != '' and not ORPH.match(line):
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
            s1, e1 = it['s'] + 1 + doc.off, it['e'] + 1 + doc.off
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


def dedent_body(lines, n):
    text = '\n'.join(lines)
    cont = set()
    for k, x, sl, el, si in lex(text):
        if k in ('str', 'comment') and el > sl:
            cont.update(range(sl + 1, el + 1))
    pad = ' ' * n
    out = []
    for i, l in enumerate(lines):
        if i in cont:
            out.append(l)
        elif l.startswith(pad):
            out.append(l[n:])
        else:
            assert l.strip() == '', ('under-indented body line', i, l[:80])
            out.append('')
    return out


def build(doc, spec):
    owner = {}
    plans = []
    unwrap_items = {}
    bodies = []
    for k, u in enumerate(spec.get('unwrap', [])):
        cands = [ii for ii, it in enumerate(doc.items) if it['kind'] == 'mod' and it['name'] == u['name'] and not doc.lines[it['e']].rstrip().endswith(';')]
        assert len(cands) == 1, ('unwrap candidate', u['name'], cands)
        ii = cands[0]
        it = doc.items[ii]
        rx = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+%s\s*\{\s*$' % re.escape(u['name']))
        hdr = None
        for L in range(it['s'], it['e'] + 1):
            m = rx.match(doc.lines[L])
            if m:
                hdr = L
                vis = (m.group(1) or '').strip()
                break
        assert hdr is not None, ('unwrap header', u['name'])
        assert doc.lines[it['e']].strip() == '}', ('unwrap close', u['name'])
        assert it['balanced'], ('unwrap item unbalanced regions', u['name'])
        unwrap_items[ii] = (k, hdr, vis)
        body = dedent_body(doc.lines[hdr + 1:it['e']], u.get('dedent', 4))
        bodies.append((u, '\n'.join(body), hdr + 1 + doc.off))
    for ci, c in enumerate(spec['children']):
        idxs = select(doc, c['ranges'], c['folder'])
        keep = []
        for ii in idxs:
            it = doc.items[ii]
            if it['kind'] in PINNED or (it['kind'] == 'mod' and doc.lines[it['e']].rstrip().endswith(';')):
                continue
            assert ii not in owner and ii not in unwrap_items, ('item in two children', ii, it['name'])
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
    if not plans:
        pass
    elif spec.get('wiring', 'block') == 'block':
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
    repl = {}
    for ii, (k, hdr, vis) in unwrap_items.items():
        u = spec['unwrap'][k]
        it = doc.items[ii]
        repl[hdr] = ['#[path = "%s/🦀️.rs"]' % u['folder'], '%smod %s;' % (vis + ' ' if vis else '', u['name'])]
        for L in range(hdr, it['e'] + 1):
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
        if L in repl:
            out.extend(repl[L])
            continue
        if L in wiring:
            wl = ['']
            for cj in wiring[L]:
                c = spec['children'][cj]
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
    return ptxt, child_files, plans, bodies


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


INC = re.compile(r'(include_str!|include_bytes!)\(\s*"([^"\\]+)"')


def rewrite_includes(text, depth):
    def sub(m):
        path = m.group(2)
        if path.startswith('/') or ':' in path:
            return m.group(0)
        return '%s("%s%s"' % (m.group(1), '../' * depth, path)
    return INC.sub(sub, text)


def native_empty_regions(text):
    lines = text.split('\n')
    stack = []
    names = []
    for i, l in enumerate(lines):
        m = RGN.match(l)
        if not m:
            continue
        if not m.group(1):
            stack.append(i)
        else:
            j = stack.pop()
            if all(x.strip() == '' for x in lines[j + 1:i]):
                names.append(lines[j].strip().split('#region', 1)[1].strip())
    return tuple(names)


def unit_files(sp, prefix=''):
    """relative file paths (parent first) of a spec, recursively"""
    out = [prefix + '🦀️.rs']
    for c in sp['children']:
        out.append(prefix + c['folder'] + '/🦀️.rs')
    for u in sp.get('unwrap', []):
        out.extend(unit_files(u, prefix + u['folder'] + '/'))
    return out


def stage_unit(text, sp, ext, depth, top):
    doc = Doc(text, sp.get('off', 0))
    ptxt, child_files, plans, bodies = build(doc, sp)
    rep = []
    child_files = widen_children(ptxt, child_files, ext, rep, sp.get('nest', False))
    files = {}
    for f, t in list(child_files.items()):
        t = rewrite_includes(t, depth + f.count('/') + 1)
        for old_, new_ in sp.get('patch', {}).get(f, []):
            assert t.count(old_) == 1, (f, old_[:80], t.count(old_))
            t = t.replace(old_, new_)
        files[f + '/🦀️.rs'] = t
    for old_, new_ in sp.get('patch_parent', []):
        assert ptxt.count(old_) == 1, ('parent', old_[:80], ptxt.count(old_))
        ptxt = ptxt.replace(old_, new_)
    ptxt, nrem = prune_empty_regions(ptxt, tuple(sp.get('keep_empty', ())) + native_empty_regions(text))
    ptxt = collapse(ptxt.split('\n'))
    ptxt = rewrite_includes(ptxt, depth) if depth else ptxt
    if not top:
        ptxt = sp['doc'] + '\n\n' + ptxt.lstrip('\n')
    files['🦀️.rs'] = ptxt + ('\n' if not ptxt.endswith('\n') else '')
    for u, body, off in bodies:
        usp = dict(u)
        usp['off'] = off
        sub, subrep = stage_unit(body, usp, [], depth + 1, False)
        for f, t in sub.items():
            files[u['folder'] + '/' + f] = t
        rep.extend((u['folder'] + '/' + r[0],) + tuple(r[1:]) for r in subrep)
    return files, rep


def load_orig(sp):
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    if sp.get('pre'):
        orig = sp['pre'](orig)
    return orig


def stage(tag, spec_module):
    sp = spec_module.SPEC
    orig = load_orig(sp)
    ext = [open(os.path.join(REPO, p), encoding='utf-8').read() for p in sp.get('external', [])]
    files, rep = stage_unit(orig, sp, ext, 0, True)
    d = os.path.join(SCR, 'stage', tag)
    shutil.rmtree(d, ignore_errors=True)
    for rel, t in files.items():
        fp = os.path.join(d, rel)
        os.makedirs(os.path.dirname(fp), exist_ok=True)
        open(fp, 'w', encoding='utf-8').write(t)
    with open(os.path.join(SCR, 'stage', tag + '.widen.tsv'), 'w', encoding='utf-8') as fh:
        for r in rep:
            fh.write('\t'.join(r) + '\n')
    print('staged', tag, {k: v.count('\n') for k, v in files.items()}, 'widened', len(rep))
    return d


def norm_line(l):
    return re.sub(r'(include_(?:str|bytes)!\()"(?:\.\./)+', r'\1"', l.strip().replace('pub(in super::super) ', '').replace('pub(super) ', ''))


def lossless(tag, spec_module):
    sp = spec_module.SPEC
    orig = load_orig(sp)
    d = os.path.join(SCR, 'stage', tag) if len(sys.argv) < 4 else sys.argv[3]
    rels = unit_files(sp)
    files = [os.path.join(d, r) for r in rels]
    O = collections.Counter(norm_line(l) for l in orig.split('\n') if l.strip())
    N = collections.Counter()
    for f in files:
        N += collections.Counter(norm_line(l) for l in open(f, encoding='utf-8').read().split('\n') if l.strip())
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
        elif re.match(r'^(pub(\((crate|super)\))? )?mod [a-z0-9_]+;$', l):
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
    worst = 0
    for rel in unit_files(sp)[1:]:
        r = os.path.join(base, rel)
        n = len(r.encode('utf-8'))
        worst = max(worst, n)
        nabs = len((REPO + '/' + r).encode('utf-8'))
        print('path rel-bytes', n, 'abs-bytes', nabs, 'abs-chars', len(REPO + '/' + r), r[-70:])
        if n > 240 or len(REPO + '/' + r) > 256:
            print('  TOO LONG (repo-relative bytes > 240 or absolute chars > 256)')
            ok = False
    print('worst rel-bytes', worst)
    return ok


def install(tag, spec_module):
    sp = spec_module.SPEC
    d = os.path.join(SCR, 'stage', tag)
    target = os.path.join(REPO, sp['file'])
    cur = open(target, 'rb').read()
    orig = open(os.path.join(T, 'orig', sp['orig']), 'rb').read()
    assert cur == orig, 'repo file differs from the recorded original; refusing to overwrite'
    base = os.path.dirname(target)
    rels = unit_files(sp)
    for rel in rels[1:]:
        assert not os.path.exists(os.path.join(base, rel)), ('child exists', rel)
    folders = sorted({os.path.dirname(r) for r in rels[1:]})
    for fo in folders:
        os.makedirs(os.path.join(base, fo), exist_ok=True)
    for rel in rels[1:]:
        shutil.copyfile(os.path.join(d, rel), os.path.join(base, rel))
    tmp = target + '.pfa-tmp'
    shutil.copyfile(os.path.join(d, '🦀️.rs'), tmp)
    os.replace(tmp, target)
    print('installed', tag, len(rels) - 1, 'children')


if __name__ == '__main__':
    import importlib
    cmd = sys.argv[1]
    tag = sys.argv[2]
    if cmd == 'list':
        sp = importlib.import_module('specs.' + tag).SPEC
        listing(Doc(load_orig(sp)))
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
