#!/usr/bin/env python3
"""🔬 Rust lexer and top-level item splitter for the os-infinite crate carve (ticket tool, not part of the repo build)."""
import re, os, sys, json

LOCAL_ROOTS = None
IDENT_START = re.compile(r'[A-Za-z_\u0080-\U0010ffff]')
ID_RE = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


class Tok:
    __slots__ = ('k', 's', 'a', 'b')

    def __init__(self, k, s, a, b):
        self.k, self.s, self.a, self.b = k, s, a, b

    def __repr__(self):
        return f'{self.k}:{self.s!r}@{self.a}'


def lex(text):
    toks = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c in ' \t\r\n':
            i += 1
            continue
        if c == '/' and text.startswith('//', i):
            j = text.find('\n', i)
            j = n if j < 0 else j
            toks.append(Tok('comment', text[i:j], i, j))
            i = j
            continue
        if c == '/' and text.startswith('/*', i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith('/*', j):
                    depth += 1
                    j += 2
                elif text.startswith('*/', j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            toks.append(Tok('comment', text[i:j], i, j))
            i = j
            continue
        if c == 'r' or c == 'b':
            m = re.match(r'b?r(#*)"', text[i:i + 40])
            if m:
                hashes = m.group(1)
                end = '"' + hashes
                j = text.find(end, i + m.end())
                j = n if j < 0 else j + len(end)
                toks.append(Tok('str', text[i:j], i, j))
                i = j
                continue
            if c == 'b' and text.startswith('b"', i):
                j = i + 2
                while j < n and text[j] != '"':
                    j += 2 if text[j] == '\\' else 1
                toks.append(Tok('str', text[i:j + 1], i, j + 1))
                i = j + 1
                continue
            if c == 'b' and text.startswith("b'", i):
                m2 = re.match(r"b'(\\.[^']*|[^\\'])'", text[i:i + 12])
                if m2:
                    toks.append(Tok('char', m2.group(0), i, i + m2.end()))
                    i += m2.end()
                    continue
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == '\\' else 1
            toks.append(Tok('str', text[i:j + 1], i, j + 1))
            i = j + 1
            continue
        if c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'])'", text[i:i + 14])
            if m:
                toks.append(Tok('char', m.group(0), i, i + m.end()))
                i += m.end()
                continue
            m = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", text[i:])
            if m:
                toks.append(Tok('life', m.group(0), i, i + m.end()))
                i += m.end()
                continue
            toks.append(Tok('p', c, i, i + 1))
            i += 1
            continue
        if c.isdigit():
            m = re.match(r'[0-9][0-9A-Za-z_]*(\.[0-9][0-9A-Za-z_]*)?', text[i:])
            s = m.group(0)
            if re.match(r'^[0-9][0-9_]*[eE]$', s) and text[i + len(s):i + len(s) + 1] in '+-':
                m2 = re.match(r'[+-][0-9_]+', text[i + len(s):])
                if m2:
                    s += m2.group(0)
            toks.append(Tok('num', s, i, i + len(s)))
            i += len(s)
            continue
        m = ID_RE.match(text, i)
        if m:
            s = m.group(0)
            if s == 'r' and text.startswith('#', i + 1):
                m3 = ID_RE.match(text, i + 2)
                if m3:
                    s = 'r#' + m3.group(0)
            toks.append(Tok('id', s, i, i + len(s)))
            i += len(s)
            continue
        if ord(c) > 127:
            j = i
            while j < n and ord(text[j]) > 127:
                j += 1
            toks.append(Tok('id', text[i:j], i, j))
            i = j
            continue
        toks.append(Tok('p', c, i, i + 1))
        i += 1
    return toks


OPEN = {'(': ')', '[': ']', '{': '}'}
CLOSE = {')', ']', '}'}

ITEM_KW = {'fn', 'struct', 'enum', 'union', 'trait', 'type', 'static', 'const', 'mod', 'impl', 'use', 'extern', 'macro_rules'}


class Item:
    def __init__(self):
        self.kind = None
        self.name = None
        self.start = 0
        self.end = 0
        self.text = ''
        self.lead = ''
        self.trail = ''
        self.attrs = []
        self.vis = ''
        self.cfg = []
        self.impl_ty = None
        self.impl_trait = None
        self.idents = set()
        self.members = []

    def __repr__(self):
        return f'<{self.kind} {self.name} {self.impl_ty}>'


def split_items(text):
    """Return (items, tail). Each item.lead is the trivia (comments/blank lines) before it (starting after the previous item's end line)."""
    toks = lex(text)
    code = [t for t in toks if t.k != 'comment']
    items = []
    i, n = 0, len(code)
    prev_end = 0
    while i < n:
        j = i
        attrs = []
        attr_strs = set()
        while j < n and code[j].s == '#' and code[j].k == 'p':
            k = j + 1
            if k < n and code[k].s == '!':
                k += 1
            assert code[k].s == '[', (code[k], text[code[k].a - 50:code[k].a + 50])
            depth = 0
            while True:
                if code[k].s == '[':
                    depth += 1
                elif code[k].s == ']':
                    depth -= 1
                    if depth == 0:
                        break
                k += 1
            attrs.append(text[code[j].a:code[k].b])
            for q in range(j, k + 1):
                if code[q].k == 'str':
                    lit = code[q].s.strip('"')
                    if ID_RE.fullmatch(lit): attr_strs.add(lit)
            j = k + 1
        if j >= n:
            if attrs:
                it = Item()
                it.kind = 'attr-only'
                it.start = code[i].a
                it.end = code[n - 1].b
                it.attrs = attrs
                items.append(it)
            i = n
            break
        s0 = code[i].a
        h = j
        vis = ''
        vis_a = vis_b = None
        head_a = code[j].a
        if code[h].s == 'pub' and code[h].k == 'id':
            h += 1
            if h < n and code[h].s == '(':
                d = 0
                while True:
                    if code[h].s == '(':
                        d += 1
                    elif code[h].s == ')':
                        d -= 1
                        if d == 0:
                            break
                    h += 1
                h += 1
            vis = text[code[j].a:code[h - 1].b]
            vis_a, vis_b = code[j].a, code[h - 1].b
        head_a = code[h].a
        qual = []
        while h < n and code[h].k == 'id' and code[h].s in ('async', 'const', 'unsafe', 'default') and not (code[h].s == 'const' and h + 1 < n and (code[h + 1].s == '_' or (code[h + 1].k == 'id' and code[h + 2].s == ':' if h + 2 < n else False))):
            qual.append(code[h].s)
            h += 1
        if h < n and code[h].s == 'extern' and h + 1 < n and code[h + 1].k == 'str':
            qual.append('extern')
            h += 2
        kw = code[h].s
        it = Item()
        it.attrs = attrs
        it.attr_strs = attr_strs
        it.vis = vis
        it.vis_a = vis_a; it.vis_b = vis_b; it.head_a = head_a
        it.cfg = [a for a in attrs if re.match(r'#\[\s*cfg\s*\(', a)]
        e = h
        macro_call = False
        if kw in ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'static', 'const', 'mod', 'impl', 'use', 'extern', 'macro_rules', 'unsafe'):
            it.kind = kw
        else:
            m_ = h
            while m_ + 2 < n and code[m_].k == 'id' and code[m_ + 1].s == ':' and code[m_ + 2].s == ':':
                m_ += 3
            if m_ + 1 < n and code[m_].k == 'id' and code[m_ + 1].s == '!':
                it.kind = 'macro_call'
                it.name = code[m_].s
                macro_call = True
            else:
                raise SyntaxError(f'unknown item head {kw!r} at {code[h].a}: {text[code[h].a - 40:code[h].a + 80]!r}')
        if kw == 'macro_rules':
            it.kind = 'macro_rules'
            it.name = code[h + 2].s
        elif kw == 'impl':
            pass
        elif kw in ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'static', 'const', 'mod', 'extern'):
            nm = h + 1
            if code[nm].s == 'mut':
                nm += 1
            it.name = code[nm].s if code[nm].k == 'id' or code[nm].s == '_' else None
            if kw == 'extern' and code[h + 1].s == 'crate':
                it.kind = 'extern_crate'
                it.name = code[h + 2].s
        depth = 0
        k = h
        angle = 0
        end_tok = None
        while k < n:
            t = code[k].s
            if code[k].k == 'p':
                if t in '([':
                    depth += 1
                elif t in ')]':
                    depth -= 1
                elif t == '{':
                    if depth == 0 and it.kind in ('fn', 'struct', 'enum', 'union', 'trait', 'mod', 'impl', 'extern', 'macro_rules', 'macro_call', 'unsafe') and not (k > 0 and code[k - 1].s in ('<', ',') and angle > 0):
                        d2 = 0
                        m = k
                        while True:
                            if code[m].s == '{' and code[m].k == 'p':
                                d2 += 1
                            elif code[m].s == '}' and code[m].k == 'p':
                                d2 -= 1
                                if d2 == 0:
                                    break
                            m += 1
                        end_tok = m
                        break
                    depth += 1
                elif t == '}':
                    depth -= 1
                elif t == ';' and depth == 0:
                    end_tok = k
                    break
                elif t == '<' and depth == 0:
                    angle += 1
                elif t == '>' and depth == 0 and angle > 0 and not (k > 0 and code[k - 1].s == '-'):
                    angle -= 1
            k += 1
        assert end_tok is not None, (it.kind, it.name, s0)
        it.start = s0
        it.end = code[end_tok].b
        it.text = text[it.start:it.end]
        it._tok_range = (i, end_tok)
        if it.kind == 'unsafe':
            if code[h + 1].s == 'impl':
                it.kind = 'impl'
        if it.kind == 'impl':
            head_toks = []
            for m in range(h + 1, end_tok + 1):
                if code[m].s == '{' and code[m].k == 'p':
                    break
                head_toks.append(code[m])
            hs = [t for t in head_toks]
            idx_for = None
            d = 0
            for m, t in enumerate(hs):
                if t.s == '<':
                    d += 1
                elif t.s == '>' and not (m > 0 and hs[m - 1].s == '-'):
                    d -= 1
                elif t.s == 'for' and t.k == 'id' and d == 0:
                    idx_for = m
                elif t.s == 'where' and t.k == 'id' and d == 0:
                    hs = hs[:m]
                    break
            if idx_for is not None:
                trait_toks, ty_toks = hs[:idx_for], hs[idx_for + 1:]
            else:
                trait_toks, ty_toks = [], hs
            def skip_generics(ts):
                if ts and ts[0].s == '<':
                    d = 0
                    for m, t in enumerate(ts):
                        if t.s == '<':
                            d += 1
                        elif t.s == '>' and not (m > 0 and ts[m - 1].s == '-'):
                            d -= 1
                            if d == 0:
                                return ts[m + 1:]
                return ts
            if idx_for is None:
                ty_toks = skip_generics(ty_toks)
            else:
                trait_toks = skip_generics(trait_toks)
            def last_path_ident(ts):
                depth = 0
                last = None
                for t in ts:
                    if t.s == '<':
                        depth += 1
                    elif t.s == '>':
                        depth -= 1
                    elif depth == 0 and t.k == 'id' and t.s not in ('dyn', 'mut', 'const', 'unsafe'):
                        last = t.s
                    elif depth == 0 and t.s in ('(', '[', '&') :
                        if t.s != '&':
                            return ''.join(x.s for x in ts) if not last else last
                return last
            it.impl_ty = last_path_ident(ty_toks)
            it.impl_trait = last_path_ident(trait_toks) if idx_for is not None else None
            it.impl_ty_text = ''.join(t.s + ' ' if t.k == 'id' else t.s for t in ty_toks)
            it.impl_trait_text = ''.join(t.s + ' ' if t.k == 'id' else t.s for t in trait_toks)
            it.name = None
        ids = set()
        plain = set()
        dots = set()
        calls = set()
        fields = set()
        assoc = set()
        for m in range(i, end_tok + 1):
            t = code[m]
            if t.k == 'id':
                ids.add(t.s)
                prev = code[m - 1].s if m > i else ''
                nxt = code[m + 1].s if m + 1 <= end_tok else ''
                if prev == '.' and not (m - 2 >= i and code[m - 2].s == '.'):
                    if nxt == '(' or (nxt == ':' and m + 3 <= end_tok and code[m + 2].s == ':' and code[m + 3].s == '<'):
                        dots.add(t.s)
                    else:
                        fields.add(t.s)
                else:
                    qualified = prev == ':' and m - 2 >= i and code[m - 2].s == ':'
                    seg = code[m - 3].s if qualified and m - 3 >= i and code[m - 3].k == 'id' else ''
                    root = seg
                    if qualified and seg:
                        q = m
                        while q - 3 >= i and code[q - 1].s == ':' and code[q - 2].s == ':' and code[q - 3].k == 'id':
                            q -= 3
                        root = code[q].s
                    if (not qualified) or (seg and (seg[0].islower() or seg[0] == '_') and (LOCAL_ROOTS is None or root in LOCAL_ROOTS)):
                        plain.add(t.s)
                    if nxt == '(' or (nxt == ':' and m + 3 <= end_tok and code[m + 2].s == ':' and code[m + 3].s == '<') or nxt == '!':
                        calls.add(t.s)
                    if prev == ':' and m - 2 >= i and code[m - 2].s == ':' and m - 3 >= i and code[m - 3].k == 'id':
                        assoc.add((code[m - 3].s, t.s))
        m = i
        while m <= end_tok:
            if code[m].s == '#' and code[m].k == 'p':
                q = m + 1
                if q <= end_tok and code[q].s == '!': q += 1
                if q <= end_tok and code[q].s == '[':
                    d = 0
                    while q <= end_tok:
                        if code[q].s == '[': d += 1
                        elif code[q].s == ']':
                            d -= 1
                            if d == 0: break
                        elif code[q].k == 'str':
                            lit = code[q].s.strip('"')
                            if ID_RE.fullmatch(lit): attr_strs.add(lit)
                        q += 1
                    m = q
            m += 1
        it.attr_strs = attr_strs
        it.idents = ids
        it.plain = plain
        it.dots = dots
        it.calls = calls
        it.fields = fields
        it.assoc = assoc
        it.tok_slice = (i, end_tok)
        items.append(it)
        i = end_tok + 1
    return items, toks, code


def line_start(text, pos):
    p = text.rfind('\n', 0, pos)
    return p + 1


def chunk_file(text):
    """Split a file into chunks. Chunk = leading trivia + item (+ trailing `//#endregion` lines). Returns (head_lines_text, chunks, tail) where head is the `//!` crate-doc lines at file start."""
    items, toks, code = split_items(text)
    comments = [t for t in toks if t.k == 'comment']
    # inner doc / inner attributes at the very top belong to head
    chunks = []
    pos = 0
    n = len(text)
    for idx, it in enumerate(items):
        s = line_start(text, it.start)
        it.line_start = s
        e = text.find('\n', it.end)
        e = n if e < 0 else e
        it.line_end = e
    # leading trivia = text between previous item's line_end(+1) and this item's line_start
    prev = 0
    for it in items:
        it.lead_start = prev
        prev = it.line_end + 1 if it.line_end < n else n
    # attach trailing //#endregion lines (right after item, contiguous)
    for idx, it in enumerate(items):
        nxt_start = items[idx + 1].line_start if idx + 1 < len(items) else n
        trail_end = it.line_end + 1 if it.line_end < n else n
        p = trail_end
        while p < nxt_start:
            q = text.find('\n', p)
            q = n if q < 0 else q
            line = text[p:q]
            if re.match(r'\s*//\s*#endregion\b', line):
                trail_end = q + 1 if q < n else n
                p = trail_end
            else:
                break
        it.trail_end = trail_end
    for idx, it in enumerate(items):
        it.lead_start = items[idx - 1].trail_end if idx else 0
    for idx, it in enumerate(items):
        it.full_start = it.lead_start
        it.full_end = it.trail_end
    return items, text


if __name__ == '__main__':
    path = sys.argv[1]
    text = open(path, encoding='utf-8').read()
    items, _ = chunk_file(text)
    for it in items:
        col = it.start - line_start(text, it.start)
        print(it.kind, it.name, it.impl_ty, it.impl_trait, it.vis, 'col', col, text.count('\n', 0, it.start) + 1, text.count('\n', 0, it.end) + 1)
