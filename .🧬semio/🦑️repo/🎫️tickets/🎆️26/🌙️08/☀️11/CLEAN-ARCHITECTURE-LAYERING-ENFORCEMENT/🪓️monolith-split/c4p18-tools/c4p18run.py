#!/usr/bin/env python3
"""usage: c4p18run.py <stage|lossless|paths|install|verify|list> <tag>  (specs in specs/<tag>.py)
A spec either splits a file into item-level children (spec['children'], ranges in original file lines)
or lifts inline `mod x { .. }` blocks into file modules (spec['modules']; a module may carry an 'inner' item-level split)."""
import os, re, sys, json, collections, shutil, importlib, hashlib

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p18 as pfa
from rsitems import lex

REPO = pfa.REPO
SCR = pfa.SCR
INC = re.compile(r'(include_str!|include_bytes!|include!)\(\s*"([^"\\]+)"')
EMO = '️'


def rewrite_includes(text, depth):
    def sub(m):
        path = m.group(2)
        if path.startswith('/') or ':' in path:
            return m.group(0)
        return '%s("%s%s"' % (m.group(1), '../' * depth, path)
    return INC.sub(sub, text)


def norm_line(l):
    l = l.strip().replace('pub(in super::super) ', '').replace('pub(super) ', '')
    return re.sub(r'(include_(?:str|bytes)!|include!)\("(?:\.\./)+', r'\1("', l)


def dedent_body(body_lines):
    text = '\n'.join(body_lines)
    cont = set()
    for k, x, sl, el, si in lex(text):
        if k == 'str' and el > sl:
            for L in range(sl + 1, el + 1):
                cont.add(L)
    out = []
    for i, l in enumerate(body_lines):
        if i in cont:
            out.append(l)
        elif l.strip() == '':
            out.append('')
        else:
            assert l.startswith('    '), ('body line not indented by 4 spaces', i + 1, l[:80])
            out.append(l[4:])
    return out


def read_orig(sp):
    t = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    if sp.get('pre'):
        t = sp['pre'](t)
    return t


def split_text(doc, spec, ext_texts, depth, who):
    ptxt, child_files, plans = pfa.build(doc, spec)
    rep = []
    child_files = pfa.widen_children(ptxt, child_files, ext_texts, rep, spec.get('nest', False))
    for f, t in list(child_files.items()):
        t = rewrite_includes(t, f.count('/') + 1 + depth)
        for old_, new_ in spec.get('patch', {}).get(f, []):
            assert t.count(old_) == 1, (who, f, old_[:80], t.count(old_))
            t = t.replace(old_, new_)
        child_files[f] = t
    for old_, new_ in spec.get('patch_parent', []):
        assert ptxt.count(old_) == 1, (who, 'parent', old_[:80], ptxt.count(old_))
        ptxt = ptxt.replace(old_, new_)
    ptxt, nrem = pfa.prune_empty_regions(ptxt, spec.get('keep_empty', ()))
    ptxt = pfa.collapse(ptxt.split('\n'))
    ptxt = rewrite_includes(ptxt, depth) if depth else ptxt
    return ptxt, child_files, rep


def ext_texts_of(sp):
    return [open(os.path.join(REPO, p), encoding='utf-8').read() for p in sp.get('external', [])]


def stage_files(sp):
    """returns (files: relpath -> text, meta)"""
    orig = read_orig(sp)
    files = {}
    meta = dict(declared_missing=[], declared_extra=[], widened=[])
    if sp.get('modules'):
        doc = pfa.Doc(orig)
        lines = list(doc.lines)
        repl = []
        for m in sp['modules']:
            hits = [it for it in doc.items if it['kind'] == 'mod' and it['name'] == m['name'] and not doc.lines[it['e']].rstrip().endswith(';')]
            assert len(hits) == 1, ('module not found / ambiguous', m['name'], len(hits))
            it = hits[0]
            s, e = it['s'], it['e']
            k = None
            for L in range(s, e + 1):
                if re.match(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+%s\s*\{\s*$' % re.escape(m['name']), lines[L]):
                    k = L
                    break
            assert k is not None, ('module header not found', m['name'])
            assert lines[e].strip() == '}', ('module does not end with a lone brace', m['name'], lines[e])
            body = dedent_body(lines[k + 1:e])
            attrs = lines[s:k]
            header = lines[k]
            folder = m['folder']
            mount = list(attrs) + ['#[path = "%s/🦀️.rs"]' % folder, header.rstrip()[:-1].rstrip() + ';']
            meta['declared_missing'] += [header, lines[e]]
            meta['declared_extra'] += mount[len(attrs):]
            body_text = '\n'.join(body)
            if m.get('inner'):
                inner = dict(m['inner'])
                inner['children'] = [dict(c, ranges=[(a - (k + 1) + 0, b - (k + 1) + 0) for a, b in c['ranges']]) for c in inner['children']]
                bdoc = pfa.Doc(body_text)
                ptxt_b, cf, rep = split_text(bdoc, inner, ext_texts_of(inner), 1, m['name'])
                for r in rep:
                    meta['widened'].append((m['name'],) + tuple(r))
                files['%s/🦀️.rs' % folder] = ptxt_b + '\n'
                for f, t in cf.items():
                    files['%s/%s/🦀️.rs' % (folder, f)] = t
            else:
                files['%s/🦀️.rs' % folder] = rewrite_includes(body_text, 1).rstrip('\n') + '\n'
            if m.get('doc'):
                cur = files['%s/🦀️.rs' % folder]
                files['%s/🦀️.rs' % folder] = m['doc'] + ('\n//!\n' if cur.startswith('//!') else '\n\n') + cur
            repl.append((s, e, mount))
        out = []
        L = 0
        repl.sort()
        ri = 0
        while L < len(lines):
            if ri < len(repl) and L == repl[ri][0]:
                out.extend(repl[ri][2])
                L = repl[ri][1] + 1
                ri += 1
                continue
            out.append(lines[L])
            L += 1
        parent = pfa.collapse(out)
        if sp.get('patch_parent'):
            for old_, new_ in sp['patch_parent']:
                assert parent.count(old_) == 1, ('parent', old_[:80], parent.count(old_))
                parent = parent.replace(old_, new_)
        files['🦀️.rs'] = parent.rstrip('\n') + '\n'
    else:
        doc = pfa.Doc(orig)
        ptxt, cf, rep = split_text(doc, sp, ext_texts_of(sp), 0, 'file')
        for r in rep:
            meta['widened'].append(('',) + tuple(r))
        files['🦀️.rs'] = ptxt.rstrip('\n') + '\n'
        for f, t in cf.items():
            files['%s/🦀️.rs' % f] = t
    return files, meta


def stage_dir(tag):
    return os.path.join(SCR, 'stage', tag)


def cmd_stage(tag, sp):
    files, meta = stage_files(sp)
    d = stage_dir(tag)
    shutil.rmtree(d, ignore_errors=True)
    for rel, t in files.items():
        p = os.path.join(d, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(t)
    json.dump(meta, open(d + '.meta.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    sizes = {rel: t.count('\n') for rel, t in files.items()}
    print('staged', tag, 'files', len(files), 'widened', len(meta['widened']))
    for rel, n in sorted(sizes.items()):
        print('  %5d %s' % (n, rel))
    return files


def read_stage(tag):
    d = stage_dir(tag)
    files = {}
    for dp, dn, fn in os.walk(d):
        for f in fn:
            p = os.path.join(dp, f)
            files[os.path.relpath(p, d)] = open(p, encoding='utf-8').read()
    return files, json.load(open(d + '.meta.json', encoding='utf-8'))


def cmd_lossless(tag, sp, files=None, meta=None):
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    if files is None:
        files, meta = read_stage(tag)
    O = collections.Counter(norm_line(l) for l in orig.split('\n') if l.strip())
    N = collections.Counter()
    for rel, t in files.items():
        N += collections.Counter(norm_line(l) for l in t.split('\n') if l.strip())
    miss = O - N
    extra = N - O
    for l in meta.get('declared_missing', []):
        k = norm_line(l)
        if miss.get(k):
            miss[k] -= 1
    miss = +miss
    declared_extra = collections.Counter(norm_line(l) for l in meta.get('declared_extra', []) + sp.get('expect_extra', []))
    pre_extra = collections.Counter(sp.get('pre_extra', []))
    print('files', len(files), 'orig non-blank', sum(O.values()), 'new non-blank', sum(N.values()))
    print('missing', sum(miss.values()))
    for l, c in sorted(miss.items()):
        print('  -', c, l[:200])
    allowed = []
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
    print('extra', sum(extra.values()))
    for t2, c, l in allowed:
        print('  +', t2 or 'UNEXPLAINED', c, l[:160])
    bad = [a for a in allowed if not a[0]]
    rr = re.compile(r'^\s*//\s?#region\b', re.M)
    re_ = re.compile(r'^\s*//\s?#endregion\b', re.M)
    mo = len(rr.findall(orig))
    mn = sum(len(rr.findall(t)) for t in files.values())
    eo = len(re_.findall(orig))
    en = sum(len(re_.findall(t)) for t in files.values())
    for rel, t in files.items():
        a, b = len(rr.findall(t)), len(re_.findall(t))
        if a != b:
            print('  UNBALANCED regions in', rel, a, b)
            bad.append(rel)
    print('regions orig/new', mo, mn, 'endregions orig/new', eo, en)
    ok = not miss and not bad and mn == en
    print('LOSSLESS', 'OK' if ok else 'FAIL', tag)
    return ok


def abs_target(sp):
    return os.path.join(REPO, sp['file'])


def cmd_paths(tag, sp):
    base = os.path.dirname(sp['file'])
    files, _ = read_stage(tag)
    ok = True
    worst = 0
    for rel in files:
        if rel == '🦀️.rs':
            continue
        for seg in rel.split('/')[:-1]:
            if not re.match(r'^[^\x00-\x7f\ufe0f]+\ufe0f[a-z0-9]+(-[a-z0-9]+)*$', seg) or seg.count('\ufe0f') != 1:
                print('  BAD FOLDER NAME', repr(seg))
                ok = False
        full = os.path.join(base, rel)
        n = len(full.encode('utf-8'))
        na = len((REPO + '/' + full))
        worst = max(worst, n)
        if n > 240 or na > 256:
            print('  TOO LONG', n, na, full[-90:])
            ok = False
    print('paths', tag, 'longest repo-relative bytes', worst, 'OK' if ok else 'FAIL')
    return ok


def cmd_refs(tag, sp):
    files, _ = read_stage(tag)
    base = os.path.dirname(abs_target(sp))
    staged = {os.path.normpath(os.path.join(base, rel)) for rel in files}
    rx = re.compile(r'(?:include_str!|include_bytes!|include!)\(\s*"([^"\\]+)"|#\[\s*path\s*=\s*"([^"]+)"\s*\]')
    bad = 0
    n = 0
    for rel, text in files.items():
        d = os.path.dirname(os.path.join(base, rel))
        for m in rx.finditer(text):
            r = m.group(1) or m.group(2)
            if r == '.' or r.startswith('/'):
                continue
            n += 1
            tgt = os.path.normpath(os.path.join(d, r))
            if tgt in staged or os.path.exists(tgt):
                continue
            print('  UNRESOLVED', rel, '->', r)
            bad += 1
    print('refs', tag, 'checked', n, 'unresolved', bad)
    return bad == 0


def cmd_install(tag, sp):
    files, meta = read_stage(tag)
    target = abs_target(sp)
    cur = open(target, 'rb').read()
    orig = open(os.path.join(T, 'orig', sp['orig']), 'rb').read()
    assert cur == orig, 'repo file differs from the recorded original; refusing to overwrite'
    base = os.path.dirname(target)
    new = [rel for rel in files if rel != '🦀️.rs']
    for rel in new:
        assert not os.path.exists(os.path.join(base, rel)), ('child exists', rel)
    for rel in new:
        p = os.path.join(base, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(files[rel])
    tmp = target + '.c4p18-tmp'
    open(tmp, 'w', encoding='utf-8').write(files['🦀️.rs'])
    os.replace(tmp, target)
    print('installed', tag, len(new), 'children')


def cmd_verify(tag, sp):
    files, meta = read_stage(tag)
    base = os.path.dirname(abs_target(sp))
    bad = 0
    for rel, t in files.items():
        p = os.path.join(base, rel)
        try:
            cur = open(p, encoding='utf-8').read()
        except FileNotFoundError:
            print('  MISSING', rel)
            bad += 1
            continue
        if cur != t:
            print('  DIFFERS', rel)
            bad += 1
    print('verify', tag, 'files', len(files), 'differ', bad)
    return bad == 0


if __name__ == '__main__':
    cmd, tag = sys.argv[1], sys.argv[2]
    sp = importlib.import_module('specs.' + tag).SPEC
    r = True
    if cmd == 'stage':
        cmd_stage(tag, sp)
    elif cmd == 'lossless':
        r = cmd_lossless(tag, sp)
    elif cmd == 'paths':
        r = cmd_paths(tag, sp)
    elif cmd == 'refs':
        r = cmd_refs(tag, sp)
    elif cmd == 'install':
        cmd_install(tag, sp)
    elif cmd == 'verify':
        r = cmd_verify(tag, sp)
    sys.exit(0 if r else 1)
