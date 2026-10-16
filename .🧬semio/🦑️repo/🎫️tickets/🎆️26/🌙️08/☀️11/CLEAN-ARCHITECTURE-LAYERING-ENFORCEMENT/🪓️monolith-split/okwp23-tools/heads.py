# 🔎 Lists the `crate::`, `super::`, `self::` path heads (and `use super::*` globs) of the files of the given store node(s), token level.
import os, sys, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mt import R, M, S
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import tokenize

def heads(path):
    src = open(path, encoding='utf8').read()
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    out = collections.Counter(); n = len(toks)
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] in ('crate', 'super', 'self', 'Self') and i + 2 < n and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            if t[1] == 'Self': continue
            prev = toks[i - 1][1] if i else ''
            if prev == ':' and i >= 2 and toks[i - 2][1] == ':':
                if i >= 3 and toks[i - 3][0] == 'id': continue
            j = i
            segs = [t[1]]
            j = i + 3
            while j < n:
                if toks[j][0] == 'id':
                    segs.append(toks[j][1]); j += 1
                    if j + 1 < n and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                    else: break
                elif toks[j][1] == '{':
                    segs.append('{'); break
                elif toks[j][1] == '*':
                    segs.append('*'); break
                else: break
            out['::'.join(segs[:3 if segs[0] != 'super' else 4])] += 1
    return out

if __name__ == '__main__':
    for p in sys.argv[1:]:
        h = heads(p)
        print('#', p.replace(R + '/', '').replace('🧰️framework/🛍️products/💻️os/🔨️modules/', ''))
        for k, v in sorted(h.items()): print('   ', v, k)
