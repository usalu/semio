# 🔬 Static lints of the A1 output that the resolution oracle cannot see: component receivers only name their own fields and methods, every call carries the declared argument count, no composer code still reads a field through `self` without the component hop.
import re, sys, json, collections
import em, rs, a1lib
from okrs import tokenize
from a1lib import *

def toks(text): return [(t[1], t[0]) for t in tokenize(text) if t[0] not in ('ws', 'comment')]

def count_args(T, i):
    """T[i] == '(' : number of top-level arguments up to the matching ')'"""
    d = 0; n = 0; nonempty = False; ang = 0
    j = i
    while j < len(T):
        tx = T[j][0]
        if tx in '([{': d += 1
        elif tx in ')]}':
            d -= 1
            if d == 0: break
        elif tx == ',' and d == 1: n += 1; nonempty = True
        elif d >= 1 and j > i and tx not in ',': nonempty = True
        j += 1
    # trailing comma
    trailing = T[j - 1][0] == ',' if j > i + 1 else False
    return (n + (0 if trailing else 1)) if nonempty else 0

def lint(root_text, ms):
    issues = []
    L = root_text.split('\n')
    its = rs.outline(root_text)
    comp_of_struct = {v: k for k, v in STRUCT.items()}
    # definition table per component: name -> (param count excluding receiver, receiver)
    defs = {}
    for it in its:
        if it['kind'] != 'impl': continue
        m = re.match(r'impl<[^>]*>\s*(Store\w+|ArtifactStore)<P, Mutation>', it['name'])
        if not m: continue
        comp = comp_of_struct.get(m.group(1), 'ops')
        for me in rs.methods(root_text, it['h'], it['e']):
            if me['kind'] != 'fn': continue
            text = '\n'.join(L[me['s'] - 1: me['e']])
            sig = parse_signature(text)
            nparams = len(sig['ptoks']) - (1 if sig['recv'] else 0)
            defs[(comp, me['name'])] = (nparams, sig['recv'], me['s'])
    # scan component and composer impls
    for it in its:
        if it['kind'] != 'impl': continue
        m = re.match(r'impl<[^>]*>\s*(Store\w+|ArtifactStore)<P, Mutation>', it['name'])
        if not m: continue
        comp = comp_of_struct.get(m.group(1), 'ops')
        body = '\n'.join(L[it['s'] - 1: it['e']])
        T = toks(body)
        for i in range(len(T) - 4):
            tx = T[i][0]
            # store_c . name (
            if tx in PNAME.values() or (tx == 'self' and False):
                c = [k for k, v in PNAME.items() if v == tx][0]
                if T[i + 1][0] == '.' and T[i + 2][1] == 'id':
                    nm = T[i + 2][0]
                    if T[i + 3][0] == '(':
                        d = defs.get((c, nm))
                        if d is None: issues.append(('no such method on component', comp, c, nm)); continue
                        want = d[0]; got = count_args(T, i + 3)
                        if want != got: issues.append(('arity', comp, c, nm, want, got))
                    elif nm in FIELD_COMP and FIELD_COMP[nm] != c:
                        issues.append(('field of another component', comp, c, nm))
            if tx == 'self' and T[i + 1][0] == '.' and T[i + 2][1] == 'id':
                nm = T[i + 2][0]
                if comp == 'ops' and nm in COMPS and T[i + 3][0] == '.' and T[i + 4][1] == 'id':
                    c = nm; n2 = T[i + 4][0]
                    if i + 5 < len(T) and T[i + 5][0] == '(':
                        d = defs.get((c, n2))
                        if d is None: issues.append(('no such method on component (composer)', c, n2)); continue
                        got = count_args(T, i + 5)
                        # composer passes the needs explicitly: want = params + needs
                        r = ms.get(n2)
                        want = d[0]
                        if want != got: issues.append(('arity (composer)', c, n2, want, got))
                    elif n2 in FIELD_COMP and FIELD_COMP[n2] != c:
                        issues.append(('field of another component (composer)', c, n2))
                elif comp == 'ops' and nm in FIELD_COMP and not (T[i + 3][0] in ('(',)):
                    issues.append(('composer reads a field without the hop', nm))
                elif comp != 'ops' and nm in FIELD_COMP and FIELD_COMP[nm] != comp and T[i + 3][0] != '(':
                    issues.append(('component reads a foreign field through self', comp, nm))
                elif comp != 'ops' and T[i + 3][0] == '(' and nm in ms:
                    h = ms[nm]
                    d = defs.get((comp, nm))
                    if h.home != comp and nm not in ('new',): issues.append(('self.call of a foreign-homed method', comp, nm, h.home))
                    elif d:
                        got = count_args(T, i + 3)
                        if d[0] != got: issues.append(('arity (self)', comp, nm, d[0], got))
    return issues, defs

if __name__ == '__main__':
    d = json.load(open(em.HERE + '/../🗑️generated/okwp56-a1-overlay.json'))
    text = d['out'][em.ROOT]
    t = em.read()
    ms = a1lib.load_methods(t)
    issues, defs = lint(text, ms)
    print(len(defs), 'method definitions;', len(issues), 'issues')
    for i in issues[:60]: print(i)
