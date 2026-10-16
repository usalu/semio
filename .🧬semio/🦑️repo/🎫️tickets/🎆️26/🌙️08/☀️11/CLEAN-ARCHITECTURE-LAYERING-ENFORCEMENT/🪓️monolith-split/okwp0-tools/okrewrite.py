# ✍️ Token-level rewrite of `crate::...` / `$crate::...` paths: never touches comments or string literals.
import re, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import tokenize

def rewrite(src, resolve, dollar=True):
    """resolve(segments) -> (consumed, replacement) | None. `segments` are the ids after `crate::` (up to the first non-path token)."""
    toks = tokenize(src)
    out = []; i = 0; n = len(toks); count = 0
    def nxt(j):
        while j < n and toks[j][0] in ('ws',): j += 1
        return j
    while i < n:
        kd, tx, a, b = toks[i]
        if kd == 'id' and tx == 'crate' and i + 2 < n and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            prev_dollar = bool(out) and out[-1].endswith('$')
            j = i + 3; segs = []; ends = []
            while j < n and toks[j][0] == 'id':
                segs.append(toks[j][1]); ends.append(j); j += 1
                if j + 1 < n and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                else: break
            brace = j < n and False
            r = resolve(segs, prev_dollar) if segs else None
            if r:
                consumed, repl = r
                # tokens covering crate :: seg1 :: ... :: seg_consumed
                last_tok = ends[consumed - 1] if consumed >= 1 else i
                out.append(repl)
                i = last_tok + 1
                count += 1
                continue
        out.append(tx)
        i += 1
    return ''.join(out), count

def strip_dollar_ok(s): return s
