# 🧪 Finds items gated by cfg(test) (fn/const/struct/enum/trait/type/static, also impl members) in the files of the carved crates that other crates or the kernel mention by name: a dependency is compiled without cfg(test), so such uses cannot resolve. usage: cfgtest.py [crates-file]
import os, re, sys, collections
OKD = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, OKD + '/../okwp0-tools')
import okcrates as C
crates, model, cargo = C.build()
keys = open(sys.argv[1] if len(sys.argv) > 1 else OKD + '/b1.crates').read().split()
extra = [k for k in C.RANK if k not in keys and os.path.exists(C.node_abs(k, model) + '/🦀️.rs')]
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]'); INC = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')
def mounted(root):
    seen = []; st = [root]
    while st:
        f = st.pop()
        if f in seen or not os.path.exists(f): continue
        seen.append(f)
        t = open(f, encoding='utf8').read()
        for m in list(PATH.finditer(t)) + list(INC.finditer(t)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith('.rs'): st.append(p)
    return seen
def is_test(p): return '/🧪️tests/' in p
files = {k: mounted(C.node_abs(k, model) + '/🦀️.rs') for k in keys + extra}
text = {f: open(f, encoding='utf8').read() for fl in files.values() for f in fl}
S = C.M + '/🏪️store'
kernel_files = [S + '/🦀️.rs', C.K + '/🦀️.rs'] + [os.path.join(dp, f) for dp, dn, fn in os.walk(C.K + '/📥️mounts') for f in fn]
for f in kernel_files:
    if os.path.exists(f): text[f] = open(f, encoding='utf8').read()
RX = re.compile(r'#\[cfg\((?:any\()?test\b[^\n]*\]\s*(?:#\[[^\n]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(fn|const|struct|enum|trait|type|static)\s+(\w+)')
defs = collections.defaultdict(list)
for k in keys:
    for f in files[k]:
        if is_test(f): continue
        for m in RX.finditer(text[f]):
            defs[m.group(2)].append((k, f))
owner_of = {f: k for k, fl in files.items() for f in fl}
for name, lst in sorted(defs.items()):
    for k, f in lst:
        users = []
        for g, t in text.items():
            ok = owner_of.get(g)
            if ok == k: continue
            if re.search(r'(?<![\w])' + re.escape(name) + r'(?![\w])', t): users.append(os.path.relpath(g, C.R)[-60:] if ok is None else ok)
        if users: print(f'{name} [{k}] used by', sorted(set(users))[:6])
print('done', sum(len(v) for v in defs.values()), 'cfg(test) items')
