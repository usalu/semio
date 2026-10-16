# 🧪 A1 for the monolith-mounted tests: variables typed `ArtifactStore` gain the component hop on direct field reads.
import sys, os, re, json, collections
import em
from okrs import tokenize
from a1ext import hop

def store_vars(src):
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    n = len(toks); names = set()
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] == 'ArtifactStore':
            j = i - 1
            while j >= 0 and (toks[j][1] in ('&', 'mut', "'", '(', 'Option', '<', 'Box', 'Arc', 'Vec', 'Result') or toks[j][0] == 'life'):
                j -= 1
            if j >= 1 and toks[j][1] == ':' and toks[j - 1][0] == 'id' and toks[j - 1][1] != 'self': names.add(toks[j - 1][1])
        if t[1] == 'let':
            k = i + 1
            if toks[k][1] == 'mut': k += 1
            if toks[k][0] == 'id' and i + 6 < n:
                window = ' '.join(x[1] for x in toks[k: k + 14])
                if re.search(r'ArtifactStore\s*:\s*:\s*(<[^>]*>\s*:\s*:\s*)?(new|construct|from_)', window): names.add(toks[k][1])
    return names

def convert_tests(files):
    out = {}; report = {}
    for f in files:
        src = open(f, encoding='utf8').read()
        names = store_vars(src)
        if not names: continue
        new, n = hop(src, names)
        if n:
            out[f] = new; report[os.path.relpath(f, em.S)] = (n, sorted(names)[:6])
    return out, report

TEST_FILES = ['🧪️tests/🔬️unit/🦀️.rs', '🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs', '🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs', '🧪️tests/🧪️supersede-replay/🦀️.rs', '🧪️tests/🧪️deferred-reprojection/🦀️.rs']
if __name__ == '__main__':
    out, rep = convert_tests([em.S + '/' + f for f in TEST_FILES])
    for k, v in rep.items(): print(k[-60:], v)
