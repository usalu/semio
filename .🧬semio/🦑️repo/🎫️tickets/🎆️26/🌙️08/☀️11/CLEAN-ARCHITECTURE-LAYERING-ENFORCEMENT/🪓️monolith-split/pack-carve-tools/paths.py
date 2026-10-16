"""🧭 Token-level `crate::` path rewriting for moved sources (comments and strings are never touched)."""
import re
from rs import tokenize
import usetree

def rewrite_crate_paths(text, rule):
    """rule(segments:list[str]) -> None | (consumed:int, replacement:str). Segments are the identifiers after `crate::`."""
    toks = tokenize(text)
    out = []; i = 0; n = len(toks)
    def sig(k):
        while k < n and toks[k][0] in ("ws", "comment"): k += 1
        return k
    while i < n:
        kd, tx, a, b = toks[i]
        if kd == "id" and tx == "crate":
            prev = i - 1
            while prev >= 0 and toks[prev][0] == "ws": prev -= 1
            if prev >= 0 and toks[prev][1] == "$":
                out.append(text[a:b]); i += 1; continue
            if prev >= 0 and toks[prev][0] == "id" and toks[prev][1] == "in":
                pass
            j = i + 1
            if j < n and toks[j][1] == ":" and j + 1 < n and toks[j + 1][1] == ":":
                segs = []; k = j + 2; ends = []
                while True:
                    if k < n and toks[k][0] == "id":
                        segs.append(toks[k][1]); ends.append(k)
                        if k + 2 < n and toks[k + 1][1] == ":" and toks[k + 2][1] == ":" and toks[k + 3][0] == "id":
                            k += 3; continue
                    break
                if segs:
                    r = rule(segs)
                    if r is not None:
                        consumed, repl = r
                        out.append(repl)
                        i = ends[consumed - 1] + 1
                        continue
        out.append(text[a:b]); i += 1
    return "".join(out)
