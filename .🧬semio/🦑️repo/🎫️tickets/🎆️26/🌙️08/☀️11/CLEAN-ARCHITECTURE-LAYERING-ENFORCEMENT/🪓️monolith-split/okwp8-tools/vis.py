"""🔓 Visibility widening of test-double items moved into a dependency crate: items, fields and inherent methods become `pub`."""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize
SIG = lambda t: t[0] not in ('ws', 'comment')
def sig_tokens(text):
    return [t for t in tokenize(text) if SIG(t)]
def skip_attrs(toks, i):
    n = len(toks)
    while i < n and toks[i][1] == '#':
        j = i + 1
        if j < n and toks[j][1] == '!': j += 1
        if j < n and toks[j][1] == '[':
            d = 0
            while j < n:
                if toks[j][1] == '[': d += 1
                elif toks[j][1] == ']':
                    d -= 1
                    if d == 0: j += 1; break
                j += 1
            i = j
        else: break
    return i
def match_close(toks, i):
    op = toks[i][1]; cl = {'{': '}', '(': ')', '[': ']'}[op]
    d = 0
    for j in range(i, len(toks)):
        if toks[j][1] == op: d += 1
        elif toks[j][1] == cl:
            d -= 1
            if d == 0: return j
    return len(toks) - 1
def vis_span(toks, i):
    """if toks[i] is `pub` return index after the visibility (with optional (..)), else i"""
    if i < len(toks) and toks[i][1] == 'pub':
        j = i + 1
        if j < len(toks) and toks[j][1] == '(':
            return match_close(toks, j) + 1
        return j
    return i
def split_top(toks, a, b, sep=','):
    """split toks[a:b] at top-level separators (angle depth aware, '->' safe). returns list of (s,e)"""
    parts = []; d = 0; s = a
    for j in range(a, b):
        t = toks[j][1]
        if t in ('(', '[', '{'): d += 1
        elif t in (')', ']', '}'): d -= 1
        elif t == '<': d += 1
        elif t == '>':
            if j > a and toks[j - 1][1] == '-': pass
            else: d -= 1
        elif t == sep and d == 0:
            parts.append((s, j)); s = j + 1
    if s < b: parts.append((s, b))
    return parts
def widen(text, kind, inherent=False):
    toks = sig_tokens(text)
    edits = []   # (offset, delete_len, insert)
    i = skip_attrs(toks, 0)
    def set_vis(idx):
        """make the visibility starting at toks[idx] `pub`; idx is the first token after attrs"""
        j = vis_span(toks, idx)
        if j != idx:
            if text[toks[idx][2]:toks[j - 1][3]] != 'pub': edits.append((toks[idx][2], toks[j - 1][3] - toks[idx][2], 'pub'))
        else:
            edits.append((toks[idx][2], 0, 'pub '))
    if kind == 'impl':
        if not inherent: return text
        # inherent impl: walk body
        k = i
        while k < len(toks) and toks[k][1] != '{': k += 1
        end = match_close(toks, k)
        p = k + 1
        while p < end:
            p = skip_attrs(toks, p)
            if p >= end: break
            q = vis_span(toks, p)
            # item keyword run: const/async/unsafe/fn/type
            w = toks[q][1] if q < end else ''
            if w in ('fn', 'const', 'async', 'unsafe', 'extern', 'type', 'static'):
                set_vis(p)
            # skip to end of this member
            r = q; d = 0
            while r < end:
                t = toks[r][1]
                if t in ('{', '(', '['):
                    r = match_close(toks, r) + 1
                    if t == '{' : break
                    continue
                if t == ';': r += 1; break
                r += 1
            p = r
        return apply(text, edits)
    if kind == 'macro_rules':
        return text
    if kind in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type', 'union'):
        # macro-wrapped statics: `std::thread_local! { static X: T = ...; }` -> widen the inner static
        k = i
        if k < len(toks) and toks[k][0] == 'id' and k + 1 < len(toks) and toks[k + 1][1] == ':':
            j = k
            while j + 2 < len(toks) and toks[j + 1][1] == ':' and toks[j + 2][1] == ':': j += 3
            if j + 1 < len(toks) and toks[j + 1][1] == '!' and toks[k][1] in ('std', 'thread_local') or (j + 1 < len(toks) and toks[j][1] == 'thread_local' and toks[j + 1][1] == '!'):
                b = j + 2
                while b < len(toks) and toks[b][1] != '{': b += 1
                end = match_close(toks, b)
                p = b + 1
                while p < end:
                    p = skip_attrs(toks, p)
                    if toks[p][1] == 'static' or toks[p][1] == 'pub':
                        set_vis(p)
                    r = p
                    while r < end and toks[r][1] != ';': r += 1
                    p = r + 1
                return apply(text, edits)
        set_vis(i)
        if kind in ('struct',):
            # find body
            k = vis_span(toks, i) + 2
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
            if k < len(toks) and toks[k][1] == 'where': pass
            # `where` clause before body: advance to the body brace
            kk = k
            while kk < len(toks) and toks[kk][1] not in ('{', ';') and not (toks[kk][1] == '(' and kk == k): kk += 1
            if k < len(toks) and toks[k][1] in ('{', '('): kk = k
            if kk < len(toks) and toks[kk][1] in ('{', '('):
                close = match_close(toks, kk)
                for s, e in split_top(toks, kk + 1, close):
                    p = skip_attrs(toks, s)
                    if p >= e: continue
                    set_vis(p)
        return apply(text, edits)
    return text
def apply(text, edits):
    for off, dl, ins in sorted(edits, key=lambda e: -e[0]):
        text = text[:off] + ins + text[off + dl:]
    return text
if __name__ == '__main__':
    t = "/// doc\n#[derive(Clone)]\nstruct A<T: X> where T: Y { n: Option<i32>, pub(super) m: T, #[x] k: fn(u8) -> u8 }\n"
    print(widen(t, 'struct'))
    t = "impl<P> Foo<P> {\n    fn a(&self) {}\n    pub(crate) fn b() {}\n    const X: usize = 1;\n    async fn c() {}\n}\n"
    print(widen(t, 'impl', True))
    print(widen("struct T(u8, Vec<(u8, u8)>);", 'struct'))
    print(widen("pub(super) fn x(){}", 'fn'))
