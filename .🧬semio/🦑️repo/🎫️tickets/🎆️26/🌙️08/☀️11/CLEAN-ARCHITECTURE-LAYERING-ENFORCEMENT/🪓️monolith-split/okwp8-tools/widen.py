"""🔓 Targeted widening of struct fields and inherent members to `pub` in the carved Store crates (definition scan result)."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import items as parse_items
import vis as V, unitgraph, wp8lib as L
def widen_field(text, owner, field):
    edits = []
    for it in parse_items(text):
        if it['kind'] not in ('struct', 'union') or it['name'] != owner: continue
        toks = V.sig_tokens(it['text'])
        i = V.skip_attrs(toks, 0)
        k = V.vis_span(toks, i) + 2
        while k < len(toks) and toks[k][1] not in ('{', '(', ';'):
            if toks[k][1] == '<':
                d = 0
                while k < len(toks):
                    if toks[k][1] == '<': d += 1
                    elif toks[k][1] == '>' and not (k > 0 and toks[k - 1][1] == '-'):
                        d -= 1
                        if d == 0: break
                    k += 1
            k += 1
        kk = k
        while kk < len(toks) and toks[kk][1] not in ('{', ';'):
            if toks[kk][1] == '(' and kk == k: break
            kk += 1
        if k < len(toks) and toks[k][1] in ('{', '('): kk = k
        if kk >= len(toks) or toks[kk][1] not in ('{', '('): continue
        close = V.match_close(toks, kk)
        for idx, (s, e) in enumerate(V.split_top(toks, kk + 1, close)):
            p = V.skip_attrs(toks, s)
            if p >= e: continue
            q = V.vis_span(toks, p)
            name = toks[q][1] if toks[kk][1] == '{' else str(idx)
            if name != field: continue
            vtxt = it['text'][toks[p][2]:toks[q - 1][3]] if q != p else ''
            if vtxt == 'pub': return None
            if q != p: edits.append((it['start'] + toks[p][2], toks[q - 1][3] - toks[p][2], 'pub'))
            else: edits.append((it['start'] + toks[p][2], 0, 'pub '))
    return edits
def widen_method(text, owner, member):
    edits = []
    for it in parse_items(text):
        if it['kind'] != 'impl': continue
        tr, st = unitgraph.impl_parts(it['text'])
        if tr is not None or st != owner: continue
        toks = V.sig_tokens(it['text'])
        i = V.skip_attrs(toks, 0)
        k = i
        while k < len(toks) and toks[k][1] != '{': k += 1
        end = V.match_close(toks, k)
        p = k + 1
        while p < end:
            p = V.skip_attrs(toks, p)
            if p >= end: break
            q = V.vis_span(toks, p)
            r = q
            while r < end and toks[r][1] in ('const', 'async', 'unsafe', 'extern'): r += 1
            nm = toks[r + 1][1] if r < end and toks[r][1] in ('fn', 'static', 'type') and r + 1 < end else (toks[q + 1][1] if toks[q][1] == 'const' and q + 1 < end else None)
            if nm == member:
                vtxt = it['text'][toks[p][2]:toks[q - 1][3]] if q != p else ''
                if vtxt != 'pub':
                    if q != p: edits.append((it['start'] + toks[p][2], toks[q - 1][3] - toks[p][2], 'pub'))
                    else: edits.append((it['start'] + toks[p][2], 0, 'pub '))
            r = q
            while r < end:
                t = toks[r][1]
                if t in ('{', '(', '['):
                    r = V.match_close(toks, r) + 1
                    if t == '{': break
                    continue
                if t == ';': r += 1; break
                r += 1
            p = r
    return edits
def apply_edits(text, edits):
    for off, dl, ins in sorted(edits, key=lambda e: -e[0]):
        text = text[:off] + ins + text[off + dl:]
    return text
