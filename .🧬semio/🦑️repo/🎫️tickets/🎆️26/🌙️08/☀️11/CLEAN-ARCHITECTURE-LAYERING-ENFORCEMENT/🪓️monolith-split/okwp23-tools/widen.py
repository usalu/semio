# 🔓 Visibility widening of the carved crates: items, module mounts, fields and inherent methods that other crates (or the kernel) reach after the split become `pub`.
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import tokenize, items

DEF_KINDS = ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'union', 'mod', 'macro_rules')

def _code_start(toks):
    """index of the first code token after leading comments and attributes"""
    k = 0; n = len(toks)
    while k < n:
        kd, tx, a, b = toks[k]
        if kd in ('ws', 'comment'): k += 1; continue
        if kd == 'punct' and tx == '#':
            k += 1
            while k < n and toks[k][1] != '[': k += 1
            d = 0
            while k < n:
                if toks[k][1] == '[': d += 1
                elif toks[k][1] == ']':
                    d -= 1
                    if d == 0: k += 1; break
                k += 1
            continue
        return k
    return n

def widen_span(toks, k):
    """(start, end, replacement) making the item whose first code token is toks[k] `pub`, or None when already pub"""
    kd, tx, a, b = toks[k]
    if tx == 'pub':
        j = k + 1
        while j < len(toks) and toks[j][0] == 'ws': j += 1
        if j < len(toks) and toks[j][1] == '(':
            d = 0; e = j
            while e < len(toks):
                if toks[e][1] == '(': d += 1
                elif toks[e][1] == ')':
                    d -= 1
                    if d == 0: break
                e += 1
            return (a, toks[e][3], 'pub')
        return None
    return (a, a, 'pub ')

def widen_items(text, names, kinds=DEF_KINDS):
    """widen the top-level items called `names` (any kind in `kinds`); returns (text, widened names)"""
    its = items(text)
    edits = []; done = set()
    for it in its:
        if it['kind'] in kinds and it['name'] in names:
            toks = tokenize(it['text'])
            k = _code_start(toks)
            sp = widen_span(toks, k)
            if sp:
                off = it['start']
                edits.append((off + sp[0], off + sp[1], sp[2])); done.add(it['name'])
    for s, e, r in sorted(edits, reverse=True): text = text[:s] + r + text[e:]
    return text, done

def widen_mod_decls(text, names):
    its = items(text)
    edits = []
    for it in its:
        if it['kind'] == 'mod' and it['name'] in names:
            toks = tokenize(it['text']); k = _code_start(toks)
            sp = widen_span(toks, k)
            if sp: edits.append((it['start'] + sp[0], it['start'] + sp[1], sp[2]))
    for s, e, r in sorted(edits, reverse=True): text = text[:s] + r + text[e:]
    return text

def member_uses(text):
    """identifiers used as members: `.name`, `name:` after `{`/`,` (field init / pattern), `name(` after `.`"""
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = set(); n = len(toks)
    for i, t in enumerate(toks):
        if t[0] != 'id': continue
        prev = toks[i - 1][1] if i else ''
        nxt = toks[i + 1][1] if i + 1 < n else ''
        nxt2 = toks[i + 2][1] if i + 2 < n else ''
        if prev == '.': out.add(t[1]); continue
        if nxt == ':' and nxt2 != ':' and prev in ('{', ',', '('): out.add(t[1]); continue
        if prev in ('{', ',') and nxt in (',', '}') : out.add(t[1])
    return out

def _members(toks, sig, start_body):
    """index ranges (lo, hi) into `sig` of the depth-1 members of the group opened at sig[start_body]"""
    members = []; d = 0; cur = None; j = start_body
    while j < len(sig):
        tx = toks[sig[j]][1]
        if tx in ('(', '{', '['):
            d += 1
            if d == 1 and j == start_body: cur = j + 1
        elif tx in (')', '}', ']'):
            d -= 1
            if d == 0:
                if cur is not None and cur < j: members.append((cur, j))
                break
            if d == 1 and tx == '}' and cur is not None:
                members.append((cur, j + 1)); cur = j + 1
        elif tx in (',', ';') and d == 1 and cur is not None:
            members.append((cur, j + 1)); cur = j + 1
        j += 1
    return members

def _find_body(toks, sig, kind):
    depth = 0
    for j in range(len(sig)):
        tx = toks[sig[j]][1]
        if tx == '<': depth += 1
        elif tx == '>' and depth: depth -= 1
        elif depth == 0 and tx == '{': return j, '{'
        elif depth == 0 and kind == 'struct' and tx == '(' and j > 0 and toks[sig[j - 1]][0] == 'id' and toks[sig[j - 1]][1] not in ('struct', 'pub'):
            return j, '('
        elif depth == 0 and tx == ';': return None, None
    return None, None

def widen_members(text, used_outside, tuple_access=False):
    """widen non-pub named fields (and tuple fields when `.0`-style access is used outside) of structs and the methods / consts of inherent impls whose name is used outside the crate"""
    its = items(text)
    edits = []; count = 0
    for it in its:
        kind = it['kind']
        if kind not in ('struct', 'impl'): continue
        if kind == 'impl' and re.search(r'\bfor\b', it['name'] or ''): continue
        toks = tokenize(it['text'])
        sig = [i for i, t in enumerate(toks) if t[0] not in ('ws', 'comment')]
        sb, opener = _find_body(toks, sig, kind)
        if sb is None: continue
        for lo, hi in _members(toks, sig, sb):
            m = lo
            while m < hi and toks[sig[m]][1] == '#':
                m += 1
                while m < hi and toks[sig[m]][1] != '[': m += 1
                dd = 0
                while m < hi:
                    t2 = toks[sig[m]][1]
                    if t2 == '[': dd += 1
                    elif t2 == ']':
                        dd -= 1
                        if dd == 0: m += 1; break
                    m += 1
            if m >= hi: continue
            first = m
            has_vis = toks[sig[m]][1] == 'pub'; vis_end = m
            is_pub = False
            if has_vis:
                vis_end = m + 1
                if vis_end < hi and toks[sig[vis_end]][1] == '(':
                    dd = 0
                    while vis_end < hi:
                        t2 = toks[sig[vis_end]][1]
                        if t2 == '(': dd += 1
                        elif t2 == ')':
                            dd -= 1
                            if dd == 0: vis_end += 1; break
                        vis_end += 1
                else: is_pub = True
            r = vis_end
            while r < hi and toks[sig[r]][0] == 'id' and toks[sig[r]][1] in ('const', 'unsafe', 'async', 'extern', 'default') and not (toks[sig[r]][1] == 'const' and r + 2 < hi and toks[sig[r + 1]][0] == 'id' and toks[sig[r + 2]][1] in (':',)): r += 1
            if r >= hi: continue
            name = None
            if kind == 'impl':
                if toks[sig[r]][1] in ('fn', 'const', 'type') and r + 1 < hi: name = toks[sig[r + 1]][1]
            elif opener == '{':
                if r + 2 < hi and toks[sig[r + 1]][1] == ':' and toks[sig[r + 2]][1] != ':': name = toks[sig[r]][1]
            else:
                name = None
                if tuple_access: name = '<tuple>'
            if name is None or is_pub: continue
            if name == '<tuple>':
                pass
            elif name not in used_outside: continue
            a = toks[sig[first]][2]
            if has_vis:
                b = toks[sig[vis_end - 1]][3]
                edits.append((it['start'] + a, it['start'] + b, 'pub'))
            else:
                edits.append((it['start'] + a, it['start'] + a, 'pub '))
            count += 1
    for s, e, rr in sorted(set(edits), reverse=True): text = text[:s] + rr + text[e:]
    return text, count


def vis_text(item_text):
    """the visibility prefix of an item as written (`pub`, `pub(crate)`, ... or empty)"""
    toks = tokenize(item_text)
    k = _code_start(toks)
    if k < len(toks) and toks[k][1] == 'pub':
        j = k + 1
        while j < len(toks) and toks[j][0] == 'ws': j += 1
        if j < len(toks) and toks[j][1] == '(':
            d = 0; e = j
            while e < len(toks):
                if toks[e][1] == '(': d += 1
                elif toks[e][1] == ')':
                    d -= 1
                    if d == 0: break
                e += 1
            return item_text[toks[k][2]:toks[e][3]] + ' '
        return 'pub '
    return ''


def widen_use_vis(text):
    """`pub(crate) use` / `pub(super) use` re-exports of a carved crate become `pub use` (the kernel and sibling crates import through them)"""
    edits = []
    for it in items(text):
        if it['kind'] != 'use': continue
        toks = tokenize(it['text']); k = _code_start(toks)
        if k < len(toks) and toks[k][1] == 'pub':
            sp = widen_span(toks, k)
            if sp: edits.append((it['start'] + sp[0], it['start'] + sp[1], sp[2]))
    for s, e, r in sorted(edits, reverse=True): text = text[:s] + r + text[e:]
    return text
