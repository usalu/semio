# ✅ Round-trip verification of the A1 method rewrite: the inverse of the rewrite applied to every generated method must reproduce the original token stream exactly (nothing but receiver hops, lower parameters and callee arguments changed), and every generated call must carry exactly the arguments its callee declares.
import re, collections
import em, rs, a1lib
from a1lib import *
from okrs import tokenize

def toks_of(text):
    return [(t[0], t[1]) for t in tokenize(text) if t[0] not in ('ws', 'comment')]

PN = {PNAME[c]: c for c in COMPS}

def inverse_tokens(new_text, r, ms):
    """token stream of the generated method with the A1 changes undone"""
    T = toks_of(new_text)
    out = []; i = 0; n = len(T)
    # signature: drop `, store_x : & [mut] StoreX < P , Mutation >` groups
    while i < n:
        k, tx = T[i]
        if tx == ',' and i + 1 < n and T[i + 1][1] in PN and i + 2 < n and T[i + 2][1] == ':':
            j = i + 3
            depth = 0
            # consume `& mut? StoreX < P , Mutation >`
            while j < n:
                if T[j][1] == '<': depth += 1
                elif T[j][1] == '>':
                    depth -= 1
                    if depth == 0: j += 1; break
                j += 1
            i = j; continue
        if k == 'id' and tx in PN and i + 2 < n and T[i + 1][1] == '.' and T[i + 2][0] == 'id':
            name = T[i + 2][1]
            nxt = T[i + 3][1] if i + 3 < n else ''
            callee = ms.get(name)
            is_call = nxt == '(' or (nxt == ':' and T[i + 4][1] == ':')
            if not is_call:
                out += [('id', 'self'), ('punct', '.')]; i += 2; continue
            out += [('id', 'self'), ('punct', '.')]; i += 2; continue
        if k == 'id' and tx == 'self' and i + 2 < n and T[i + 1][1] == '.' and T[i + 2][0] == 'id' and T[i + 2][1] in PN.values() and i + 4 < n and T[i + 3][1] == '.' and T[i + 4][0] == 'id':
            # composer hop `self . comp . field`
            if T[i + 4][1] in FIELD_COMP and FIELD_COMP[T[i + 4][1]] == T[i + 2][1]:
                out += [('id', 'self'), ('punct', '.')]; i += 4; continue
        if k == 'id' and tx in STRUCT.values() and i + 6 < n and T[i + 1][1] == ':' and T[i + 2][1] == ':' and T[i + 3][1] == '<' :
            # StoreX::<P, Mutation>::name -> Self::name
            j = i + 3; d = 0
            while True:
                if T[j][1] == '<': d += 1
                elif T[j][1] == '>':
                    d -= 1
                    if d == 0: break
                j += 1
            if T[j + 1][1] == ':' and T[j + 2][1] == ':':
                out += [('id', 'Self'), ('punct', ':'), ('punct', ':')]; i = j + 3; continue
        out.append((k, tx)); i += 1
    # callee arguments: after `self . m (` drop the leading need names
    res = []; i = 0; n = len(out)
    while i < n:
        res.append(out[i])
        if out[i][1] == 'self' and i + 3 < n and out[i + 1][1] == '.' and out[i + 2][0] == 'id' and out[i + 2][1] in ms:
            m = ms[out[i + 2][1]]
            j = i + 3
            if out[j][1] == ':':
                d = 0; j += 2
                while True:
                    if out[j][1] == '<': d += 1
                    elif out[j][1] == '>' and out[j - 1][1] != '-':
                        d -= 1
                        if d == 0: break
                    j += 1
                j += 1
            if out[j][1] == '(' and r.home != 'ops' and m.home != 'ops':
                res += out[i + 1: j + 1]
                i = j + 1
                for c in m.need:
                    assert out[i][1] == PNAME[c], ('callee argument missing', m.name, c, out[i])
                    i += 1
                    if out[i][1] == ',': i += 1
                continue
        i += 1
    return res

def verify_method(r, new_text, ms):
    orig = toks_of(strip_markers_leading(r.text))
    inv = inverse_tokens(new_text, r, ms)
    return orig == inv, orig, inv

def strip_markers_leading(t):
    import a1gen
    return a1gen.strip_leading_markers(t)
