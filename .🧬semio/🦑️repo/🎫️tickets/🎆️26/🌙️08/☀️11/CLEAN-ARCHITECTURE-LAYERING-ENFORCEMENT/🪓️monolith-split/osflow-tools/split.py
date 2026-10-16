#!/usr/bin/env python3
import json, os, re, sys, glob

ITEM_RE = re.compile(r'^((?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*)(fn|struct|enum|union|trait|type|static|const|mod)\s+(?:mut\s+)?([A-Za-z_]\w*)')
IMPL_RE = re.compile(r'^impl\b')


def strip_comments(text):
    out = []
    for l in text.split('\n'):
        s = l.lstrip()
        if s.startswith('//'):
            continue
        out.append(l)
    return '\n'.join(out)


def find_unique(lines, anchor, what):
    n = None
    if isinstance(anchor, list):
        anchor, n = anchor
    if anchor.startswith('~'):
        hits = [i for i, l in enumerate(lines) if l.startswith(anchor[1:])]
    else:
        hits = [i for i, l in enumerate(lines) if l.rstrip('\n') == anchor]
    if n is not None:
        assert len(hits) > n, f'{what}: anchor {anchor!r} has only {len(hits)} matches'
        return hits[n]
    assert len(hits) == 1, f'{what}: anchor {anchor!r} matches {len(hits)} lines'
    return hits[0]


STRUCT_RE = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?struct\s+(\w+)[^;{]*\{\s*$')
FIELD_RE = re.compile(r'^    ([A-Za-z_]\w*)\s*:')
IMPL_TYPE_RE = re.compile(r'^impl(?:<[^>]*>)?\s+(?:[\w:]+\s+for\s+)?([A-Za-z_]\w*)')


def collect_private(body_lines):
    items = []
    in_impl = False
    impl_type = None
    in_struct = None
    for idx, l in enumerate(body_lines):
        if l.startswith('impl') and IMPL_RE.match(l):
            in_impl = ' for ' not in l
            m = IMPL_TYPE_RE.match(l)
            impl_type = m.group(1) if m else None
            continue
        m = STRUCT_RE.match(l)
        if m and not l.startswith(' '):
            in_struct = m.group(1)
            if not l.startswith('pub'):
                items.append((idx, 'struct', m.group(1), 0, None))
            continue
        if l.startswith('}'):
            in_impl = False
            in_struct = None
            continue
        if in_struct:
            fm = FIELD_RE.match(l)
            if fm:
                items.append((idx, 'field', fm.group(1), 4, in_struct))
            continue
        if l[:1] not in (' ', '\t', '#', '/', '}', '\n', ''):
            if l.startswith('pub'):
                continue
            m = ITEM_RE.match(l)
            if m and m.group(3) != '_':
                items.append((idx, m.group(2), m.group(3), 0, None))
        elif in_impl and l.startswith('    ') and l[4:5] not in (' ', '#', '/', '}', ''):
            r = l[4:]
            if r.startswith('pub'):
                continue
            m = ITEM_RE.match(r)
            if m and m.group(2) in ('fn', 'const') and m.group(3) != '_':
                items.append((idx, m.group(2), m.group(3), 4, impl_type))
    return items


def main():
    spec = json.load(open(sys.argv[1]))
    dry = '--dry' in sys.argv
    parent = spec['parent']
    import hashlib
    if spec.get('sha1'):
        assert hashlib.sha1(open(parent,'rb').read()).hexdigest() == spec['sha1'], 'parent changed since spec'
    pdir = os.path.dirname(parent)
    src = open(parent, encoding='utf-8').read()
    lines = src.split('\n')
    if lines and lines[-1] == '':
        trail = True
        lines = lines[:-1]
    else:
        trail = False
    total_before = len(lines)

    segs = []
    for c in spec['children']:
        s = c['lo'] - 1
        e = c['hi'] - 1
        assert 0 <= s < e <= len(lines), c['mod']
        if c.get('chk_lo') is not None:
            assert lines[s].strip().startswith(c['chk_lo']), (c['mod'], 'lo', lines[s][:80])
        if c.get('chk_hi') is not None:
            last = [l for l in lines[s:e] if l.strip()][-1]
            assert last.strip().startswith(c['chk_hi']), (c['mod'], 'hi', last[:80])
        segs.append((s, e, c))
    for a, b in zip(segs, segs[1:]):
        assert a[1] <= b[0], f'overlap {a[2]["mod"]} {b[2]["mod"]}'

    bodies = []
    for s, e, c in segs:
        body = lines[s:e]
        if c.get('unwrap_inline'):
            assert body[0].startswith(c['start'] if isinstance(c['start'], str) else c['start'][0]) and body[-1] == '}', c['mod']
            body = [l[4:] if l.startswith('    ') else l for l in body[1:-1]]
            assert all(l.strip() == '' or not l.startswith(' ') or True for l in body)
        while body and body[0].strip() == '':
            body = body[1:]
        while body and body[-1].strip() == '':
            body = body[:-1]
        bodies.append(body)

    remainder = []
    pos = 0
    for s, e, c in segs:
        remainder += lines[pos:s]
        pos = e
    remainder += lines[pos:]

    near_texts = {}
    near_texts['__remainder__'] = strip_comments('\n'.join(remainder))
    for (s, e, c), b in zip(segs, bodies):
        near_texts[c['mod']] = strip_comments('\n'.join(b))
    far_text = ''
    for f in spec.get('far_scan', []):
        for p in glob.glob(f, recursive=True):
            if os.path.isfile(p) and p.endswith('.rs'):
                far_text += strip_comments(open(p, encoding='utf-8', errors='ignore').read()) + '\n'
    test_text = ''
    for f in spec.get('near_scan', []):
        for p in glob.glob(f, recursive=True):
            if os.path.isfile(p) and p.endswith('.rs'):
                test_text += strip_comments(open(p, encoding='utf-8', errors='ignore').read()) + '\n'

    report = []
    out_children = []
    for (s, e, c), body in zip(segs, bodies):
        body = list(body)
        wrap_h = c.get('wrap_header')
        if wrap_h:
            if c.get('wrap_start', True):
                body = [wrap_h] + body
            if c.get('wrap_end', True):
                body = body + ['}']
        items = collect_private(body)
        widened = []
        other_near = '\n'.join(t for k, t in near_texts.items() if k != c['mod']) + '\n' + test_text
        all_other = other_near
        structs_with_literals = {}
        for idx, kind, name, ind, owner in items:
            if ind == 0:
                pat = re.compile(r'(?<![A-Za-z0-9_])' + re.escape(name) + r'(?![A-Za-z0-9_])')
            elif kind == 'field':
                pat = re.compile(r'\.' + re.escape(name) + r'(?![A-Za-z0-9_])')
            else:
                pat = re.compile(r'(?:\.|::)' + re.escape(name) + r'(?![A-Za-z0-9_])')
            far = bool(far_text) and bool(pat.search(far_text))
            near = bool(pat.search(all_other))
            if kind == 'field' and owner:
                if owner not in structs_with_literals:
                    lit = re.compile(r'(?<![A-Za-z0-9_])' + re.escape(owner) + r'\s*\{')
                    selfimpl = re.compile(r'^impl(?:<[^>]*>)?\s+(?:[\w:]+\s+for\s+)?' + re.escape(owner) + r'\b[^\n]*\{[\s\S]*?\n\}', re.M)
                    uses_self = any(('Self {' in m.group(0) or 'Self{' in m.group(0)) for m in selfimpl.finditer(all_other))
                    structs_with_literals[owner] = bool(lit.search(all_other)) or uses_self
                if structs_with_literals[owner]:
                    near = True
            if far and c.get('far_vis'):
                vis = c['far_vis']
            elif near or far:
                vis = c.get('near_vis', 'pub(super)')
            else:
                continue
            l = body[idx]
            body[idx] = l[:ind] + vis + ' ' + l[ind:]
            widened.append((name if owner is None or kind == 'field' and False else (owner + '.' + name if owner else name), vis))
        tup = re.compile(r'^(pub(?:\([^)]*\))?\s+)?struct\s+(\w+)(<[^{(]*?>)?\((.*)\)(\s*where[^;]*)?;\s*$')
        for bi, l in enumerate(body):
            m = tup.match(l)
            if not m:
                continue
            tname = m.group(2)
            use = re.compile(r'(?<![A-Za-z0-9_])' + re.escape(tname) + r'\s*\(|(?<![A-Za-z0-9_])' + re.escape(tname) + r'\b[^;{]*\.0\b')
            if not use.search(all_other):
                continue
            inner = m.group(4)
            parts, depth, cur = [], 0, ''
            for ch in inner:
                if ch in '<([{': depth += 1
                if ch in '>)]}': depth -= 1
                if ch == ',' and depth == 0:
                    parts.append(cur); cur = ''
                else:
                    cur += ch
            if cur.strip():
                parts.append(cur)
            vis = c.get('near_vis', 'pub(super)')
            newparts = [(p_ if p_.strip().startswith('pub') else ' ' * (len(p_) - len(p_.lstrip())) + vis + ' ' + p_.lstrip()) for p_ in parts]
            if newparts != parts:
                body[bi] = l[:l.index('(', l.index('struct'))] + '(' + ','.join(newparts) + l[l.rindex(')'):]
                widened.append((tname + '.0', vis))
        top_pub = any(re.match(r'pub\s+(?!\()', l) for l in body if not l.startswith(' '))
        top_pubc = any(re.match(r'pub\((crate|in )', l) for l in body if not l.startswith(' '))
        top_items = [l for l in body if not l.startswith((' ', '\t', '#', '/', '}', '\n')) and l.strip() and ITEM_RE.match(re.sub(r'^pub(\([^)]*\))?\s+', '', l))]
        reexp = c.get('reexport')
        if reexp is None:
            if wrap_h and not top_items:
                reexp = 'none'
            elif top_pub:
                reexp = 'pub'
            elif top_pubc:
                reexp = 'pub(crate)'
            elif top_items:
                reexp = 'private'
            else:
                reexp = 'none'
        if reexp == 'private' and c.get('reexport_min'):
            reexp = c['reexport_min']
        hdr = ['//! ' + c['doc']] + ([] if c.get('skip_use_super') else ['', 'use super::*;'])
        for imp in c.get('imports', []):
            hdr.append(imp)
        text = '\n'.join(hdr) + '\n\n' + '\n'.join(body) + '\n'
        out_children.append((c, text, widened, reexp, len(body)))

    decl_at = {}
    for (s, e, c), (_, text, widened, reexp, nb) in zip(segs, out_children):
        d = []
        for a in c.get('mod_attrs', []):
            d.append(a)
        d.append(f'#[path = "{c["folder"]}/🦀️.rs"]')
        d.append(f'{c.get("mod_vis", "")}{" " if c.get("mod_vis") else ""}mod {c["mod"]};')
        if reexp == 'pub':
            d.append(f'pub use {c["mod"]}::*;')
        elif reexp in ('pub(crate)', 'pub(super)'):
            d.append(f'{reexp} use {c["mod"]}::*;')
        elif reexp == 'private':
            d.append(f'use {c["mod"]}::*;')
        decl_at[s] = (e, d)

    new_lines = []
    i = 0
    while i < len(lines):
        if i in decl_at:
            e, d = decl_at[i]
            new_lines += d
            i = e
        else:
            new_lines.append(lines[i])
            i += 1

    moved = sum(e - s for s, e, c in segs)
    print(f'parent before={total_before} after={len(new_lines)} moved={moved}')
    for (s, e, c), (_, text, widened, reexp, nb) in zip(segs, out_children):
        print(f'  {c["folder"]:34s} mod={c["mod"]:22s} lines={text.count(chr(10)):5d} reexp={reexp:10s} widened={len(widened)}')
    if dry:
        for (s, e, c), (_, text, widened, reexp, nb) in zip(segs, out_children):
            print(f'-- {c["mod"]} widened: ' + ', '.join(f'{n}' for n, v in widened))
        return
    for c, text, widened, reexp, nb in out_children:
        d = os.path.join(pdir, c['folder'])
        os.makedirs(d, exist_ok=True)
        p = os.path.join(d, '🦀️.rs')
        assert not os.path.exists(p), f'exists {p}'
        open(p, 'w', encoding='utf-8').write(text)
    open(parent, 'w', encoding='utf-8').write('\n'.join(new_lines) + ('\n' if trail else ''))
    log = os.path.join(os.path.dirname(sys.argv[1]), os.path.basename(sys.argv[1]) + '.widened.txt')
    with open(log, 'w', encoding='utf-8') as f:
        for c, text, widened, reexp, nb in out_children:
            f.write(f'## {c["mod"]}\n')
            for n, v in widened:
                f.write(f'{v} {n}\n')
    print('written')


main()
