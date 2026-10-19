"""Visibility planner of slice c4p6 (nest-aware successor of pfa-tools/widen.py).

A moved definition keeps the reach it had:
  * private (no visibility) -> `pub(super)` when its name occurs in another file of the split scope;
  * `pub(super)` -> `pub(in super::super)` when the child sits one level deeper than the original file (`nest`), because the
    original `pub(super)` meant "visible in the parent of the original file" and that module is now two levels up;
  * every other visibility is untouched.
Edits are (start, end, replacement, label); an insertion has start == end."""
import os, sys, re

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex, items, name_kind

TOPKINDS = ('fn', 'const', 'static', 'struct', 'enum', 'trait', 'type', 'union', 'mod')
MEMBERKINDS = ('fn', 'const', 'type')
NEST = 'pub(in super::super) '


def ident_set(text):
    s = set()
    for k, x, sl, el, si in lex(text):
        if k == 'id':
            s.add(x)
        elif k == 'str':
            for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*)', x):
                s.add(m.group(1))
            m = re.fullmatch(r'b?"([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)"', x)
            if m:
                s.update(m.group(1).split('::'))
    return s


def vis_span(toks, i):
    """toks[i] is the first token of a (possibly absent) visibility; returns (text, first_si, end_si_exclusive, next_index)."""
    if i >= len(toks) or toks[i][1] != 'pub':
        return '', None, None, i
    j = i + 1
    if j < len(toks) and toks[j][1] == '(':
        k = j
        while toks[k][1] != ')':
            k += 1
        text = 'pub' + ''.join(t[1] for t in toks[j:k + 1])
        return text, toks[i][4], toks[k][4] + 1, k + 1
    return 'pub', toks[i][4], toks[i][4] + 3, j


def field_segments(toks, start, open_tok):
    depth = 0
    angle = 0
    i = start
    n = len(toks)
    segs = []
    cur = []
    while i < n:
        k, x, sl, el, si = toks[i]
        if k == 'p':
            if x in '{([':
                depth += 1
                if depth == 1 and i == start:
                    i += 1
                    continue
            elif x in '})]':
                depth -= 1
                if depth == 0:
                    segs.append(cur)
                    break
            elif x == '<' and depth == 1:
                angle += 1
            elif x == '>' and depth == 1 and not (i > 0 and toks[i - 1][1] == '-'):
                angle = max(0, angle - 1)
            elif x == ',' and depth == 1 and angle == 0:
                segs.append(cur)
                cur = []
                i += 1
                continue
        cur.append(toks[i])
        i += 1
    out = []
    for s in segs:
        s = [t for t in s if t[0] != 'comment']
        j = 0
        while j < len(s) and s[j][1] == '#' and s[j][0] == 'p':
            d = 0
            j += 1
            while j < len(s):
                if s[j][1] == '[':
                    d += 1
                elif s[j][1] == ']':
                    d -= 1
                    if d == 0:
                        j += 1
                        break
                j += 1
        if j >= len(s):
            continue
        vtext, vs, ve, nxt = vis_span(s, j)
        name = None
        if open_tok == '{' and nxt < len(s) and s[nxt][0] == 'id':
            name = s[nxt][1]
        out.append((name, vtext, vs, ve, s[j][4]))
    return out


def decide(vtext, name, others, nest):
    """returns 'insert-super', 'nest' or None"""
    if vtext == '':
        return 'insert-super' if name in others else None
    if nest and vtext == 'pub(super)':
        return 'nest'
    return None


def plan(text, others, nest=False):
    edits = []
    for it in items(text):
        v, k, nm = name_kind(it['toks'])
        toks = it['toks']
        vtext, vs, ve, nxt = vis_span(toks, 0)
        if k in TOPKINDS:
            d = decide(vtext, nm, others, nest)
            if k == 'trait' and vtext == '' and d is None:
                br_ = next((q for q, t_ in enumerate(toks) if t_[0] == 'p' and t_[1] == '{'), None)
                if br_ is not None:
                    base_ = toks[br_][4] + 1
                    sub_ = text[base_:toks[-1][4]]
                    if any(name_kind(m_['toks'])[2] in others for m_ in items(sub_) if name_kind(m_['toks'])[1] in MEMBERKINDS):
                        d = 'insert-super'
            if d == 'insert-super':
                edits.append((toks[0][4], toks[0][4], 'pub(super) ', ('item', k, nm)))
            elif d == 'nest':
                edits.append((vs, ve, NEST.rstrip(), ('item-nest', k, nm)))
            if k == 'struct':
                idx = None
                sk = [q for q, t in enumerate(toks) if t[0] == 'id' and t[1] == 'struct'][0]
                for q, t in enumerate(toks):
                    if q <= sk:
                        continue
                    if t[0] == 'p' and t[1] in '{(':
                        idx = q
                        break
                    if t[0] == 'p' and t[1] == ';':
                        break
                if idx is not None:
                    ot = toks[idx][1]
                    for fname, fvt, fvs, fve, fpos in field_segments(toks, idx, ot):
                        key = fname if ot == '{' else nm
                        d = decide(fvt, key, others, nest)
                        label = 'field' if ot == '{' else 'tuplefield'
                        if d == 'insert-super':
                            edits.append((fpos, fpos, 'pub(super) ', (label, nm, key or '')))
                        elif d == 'nest':
                            edits.append((fvs, fve, NEST.rstrip(), (label + '-nest', nm, key or '')))
        elif k == 'impl':
            br = None
            for q, t in enumerate(toks):
                if t[0] == 'p' and t[1] == '{':
                    br = q
                    break
            if br is None:
                continue
            hdr = toks[:br]
            if any(t[0] == 'id' and t[1] == 'for' for t in hdr):
                continue
            depth = 0
            endq = None
            for q in range(br, len(toks)):
                t = toks[q]
                if t[0] == 'p' and t[1] in '{([':
                    depth += 1
                elif t[0] == 'p' and t[1] in '})]':
                    depth -= 1
                    if depth == 0:
                        endq = q
                        break
            base = toks[br][4] + 1
            sub = text[base:toks[endq][4]]
            tname = ''.join(t[1] for t in hdr if t[0] != 'comment')
            for mit in items(sub):
                mv, mk, mn = name_kind(mit['toks'])
                if mk not in MEMBERKINDS:
                    continue
                mt = mit['toks']
                mvt, mvs, mve, mnxt = vis_span(mt, 0)
                d = decide(mvt, mn, others, nest)
                if d == 'insert-super':
                    edits.append((mt[0][4] + base, mt[0][4] + base, 'pub(super) ', ('member', tname, mn)))
                elif d == 'nest':
                    edits.append((mvs + base, mve + base, NEST.rstrip(), ('member-nest', tname, mn)))
    return edits


def apply(text, edits):
    for s, e, rep, _ in sorted(edits, key=lambda x: (-x[0], -x[1])):
        text = text[:s] + rep + text[e:]
    return text
