"""🧮️ Test-function inventory: every fn carrying a test attribute, with the module chain, per file. usage: testnames.py out.tsv file..."""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize
TESTATTR = re.compile(r'#\s*\[\s*(?:[A-Za-z_0-9]+::)*(?:test|async_test|tokio_test|wasm_bindgen_test|quickcheck)\b')
def names(src):
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    out = []
    n = len(toks)
    i = 0
    while i < n:
        k, tx, a, b = toks[i]
        if k == 'punct' and tx == '#':
            j = i + 1
            if j < n and toks[j][1] == '!': j += 1
            if j < n and toks[j][1] == '[':
                d = 0; e = j
                while e < n:
                    if toks[e][0] == 'punct' and toks[e][1] == '[': d += 1
                    elif toks[e][0] == 'punct' and toks[e][1] == ']':
                        d -= 1
                        if d == 0: break
                    e += 1
                attr = src[toks[i][2]:toks[e][3]]
                if TESTATTR.match(attr) and toks[i + 1][1] != '!':
                    m = e + 1
                    while m < n and toks[m][1] == '#':
                        d = 0; m += 1
                        while m < n:
                            if toks[m][0] == 'punct' and toks[m][1] == '[': d += 1
                            elif toks[m][0] == 'punct' and toks[m][1] == ']':
                                d -= 1
                                if d == 0: m += 1; break
                            m += 1
                    while m < n and toks[m][1] in ('pub', 'async', 'unsafe', 'extern', 'const') or (m < n and toks[m][1] == '(' ):
                        m += 1
                    if m < n and toks[m][1] == 'fn' and m + 1 < n:
                        out.append(toks[m + 1][1])
                i = e
        i += 1
    return out
if __name__ == '__main__':
    out = open(sys.argv[1], 'w', encoding='utf8')
    total = 0
    for f in sys.argv[2:]:
        s = open(f, encoding='utf8').read()
        ns = names(s)
        for nme in ns: out.write(nme + '\t' + os.path.basename(os.path.dirname(f)) + '\n')
        total += len(ns)
    print(total)
