"""Find every call of a wrapper function outside the world crates and classify its first argument."""
import re, sys, collections, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import lex
R = "/Users/ueli/Documents/semio/"
GEN = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/oinf-ext-users.txt"
API = R + "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🔌️api/🦀️.rs"
RES = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/♻️resources/🧪️tests/🦀️.rs"

def wrapper_names():
    return re.findall(r'^pub fn (\w+)', open(API, encoding='utf-8').read(), re.M)

def files():
    fs = [l.rstrip('\n') for l in open(GEN, encoding='utf-8')]
    if RES not in fs: fs.append(RES)
    return fs

def calls(names):
    out = []
    ns = set(names)
    for f in files():
        t = open(R + f, encoding='utf-8').read()
        toks = [x for x in lex(t) if x.k != 'comment']
        for i, x in enumerate(toks):
            if x.k == 'id' and x.s in ns and i + 1 < len(toks) and toks[i+1].s == '(' and toks[i-1].s != 'fn':
                # first arg
                d = 0; j = i + 1
                start = j + 1
                end = None
                while True:
                    s = toks[j].s
                    if toks[j].k == 'p':
                        if s in '([{': d += 1
                        elif s in ')]}':
                            d -= 1
                            if d == 0: end = j; break
                        elif s == ',' and d == 1:
                            end = j; break
                    j += 1
                arg0 = t[toks[start].a:toks[end - 1].b] if end > start else ''
                out.append(dict(file=f, line=t.count('\n', 0, x.a) + 1, name=x.s, arg0=arg0, a=toks[start].a, b=toks[end-1].b if end > start else toks[start].a, end_sym=toks[end].s))
    return out

if __name__ == '__main__':
    names = wrapper_names()
    cs = calls(names)
    print(len(cs), 'call sites')
    forms = collections.Counter()
    for c in cs:
        a = c['arg0']
        if re.fullmatch(r'&mut [A-Za-z_][A-Za-z0-9_.]*', a): k = 'ref-mut-path'
        elif re.fullmatch(r'&[A-Za-z_][A-Za-z0-9_.]*', a): k = 'ref-path'
        elif re.fullmatch(r'[A-Za-z_][A-Za-z0-9_.]*', a): k = 'bare-path'
        else: k = 'complex'
        c['form'] = k; forms[k] += 1
    print(forms)
    for c in cs:
        if c['form'] == 'complex': print(c['file'].split('/')[-2][:24], c['line'], c['name'], '|', c['arg0'][:100])
