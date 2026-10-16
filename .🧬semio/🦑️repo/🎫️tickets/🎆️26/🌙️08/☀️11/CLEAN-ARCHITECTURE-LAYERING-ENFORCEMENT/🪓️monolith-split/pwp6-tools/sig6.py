import sys; sys.path.insert(0, '.')
from lib6 import *

def match_close(M, i, open_ch, close_ch):
    d = 0
    for j in range(i, len(M)):
        c = M[j]
        if c == open_ch: d += 1
        elif c == close_ch:
            d -= 1
            if d == 0: return j
    return -1

def match_angle(M, i):
    d = 0
    for j in range(i, len(M)):
        c = M[j]
        if c == '<': d += 1
        elif c == '>' and M[j - 1] != '-' and M[j - 1] != '=':
            d -= 1
            if d == 0: return j
    return -1

def split_args(M, raw, lo, hi):
    """top-level comma split of raw[lo:hi] using the mask; returns [(a, b)] spans (untrimmed) - angle brackets are NOT tracked (value expressions)"""
    parts = []; d = 0; s = lo
    for i in range(lo, hi):
        c = M[i]
        if c in '([{': d += 1
        elif c in ')]}': d -= 1
        elif c == ',' and d == 0: parts.append((s, i)); s = i + 1
    parts.append((s, hi))
    return [(a, b) for a, b in parts if raw[a:b].strip()]

def split_params(M, raw, lo, hi):
    """parameter list split: tracks angle brackets too (types)"""
    parts = []; d = 0; ang = 0; s = lo
    for i in range(lo, hi):
        c = M[i]
        if c in '([{': d += 1
        elif c in ')]}': d -= 1
        elif c == '<': ang += 1
        elif c == '>' and M[i - 1] not in '-=': ang -= 1
        elif c == ',' and d == 0 and ang == 0: parts.append((s, i)); s = i + 1
    parts.append((s, hi))
    return [(a, b) for a, b in parts if raw[a:b].strip()]

def parse_sig(raw):
    """signature of the first fn in raw: spans are absolute offsets into raw"""
    M = mask(raw)
    m = re.search(r'\bfn\s+(\w+)', M)
    name, name_end, fn_start = m.group(1), m.end(), m.start()
    pos = name_end
    while M[pos].isspace(): pos += 1
    gen = None
    if M[pos] == '<':
        e = match_angle(M, pos); gen = (pos, e + 1); pos = e + 1
        while M[pos].isspace(): pos += 1
    assert M[pos] == '(', (raw[:80], M[pos - 10:pos + 10])
    pe = match_close(M, pos, '(', ')')
    params = split_params(M, raw, pos + 1, pe)
    # body or semicolon
    j = pe + 1; d = 0
    while j < len(M):
        c = M[j]
        if c in '([': d += 1
        elif c in ')]': d -= 1
        elif c == '{' and d == 0: break
        elif c == ';' and d == 0: break
        j += 1
    recv = None
    if params:
        t = raw[params[0][0]:params[0][1]].strip()
        mm = re.match(r'(&\s*(?:\'\w+\s+)?mut\s+self|&\s*(?:\'\w+\s+)?self|mut\s+self|self)\b(\s*:.*)?$', t, re.S)
        if mm:
            kind = 'mut' if re.match(r'&\s*(?:\'\w+\s+)?mut', t) else ('ref' if t.startswith('&') else 'val')
            recv = dict(kind=kind, span=params[0], text=t)
    vis_m = re.match(r'((?:\s*(?:///[^\n]*|//[^\n]*|#\[[^\]]*\]))*\s*)((?:pub(?:\([^)]*\))?\s+)?)', raw)
    return dict(name=name, fn_start=fn_start, name_span=(fn_start, name_end), gen=gen, paren=(pos, pe), params=params, recv=recv, body_open=j, is_async=bool(re.search(r'\basync\s+(?:unsafe\s+)?(?:const\s+)?fn\b|\bconst\s+async\s+fn\b', M[:fn_start + 3])), prefix_end=vis_m.end(), ret=raw[pe + 1:j].strip())

if __name__ == '__main__':
    s = "    /// doc\n    pub(super) async fn run_framework_reserved_job<J: Foo + Send + 'static>(\n        &mut self, verb: &str, raw: &[u8],\n     cx:&mut Ctx<'_>) -> Result<(), E> where J: Bar {\n        body\n    }"
    p = parse_sig(s)
    print({k: (s[v[0]:v[1]] if isinstance(v, tuple) else v) for k, v in p.items() if k not in ('params',)}, [s[a:b].strip() for a, b in p['params']])
