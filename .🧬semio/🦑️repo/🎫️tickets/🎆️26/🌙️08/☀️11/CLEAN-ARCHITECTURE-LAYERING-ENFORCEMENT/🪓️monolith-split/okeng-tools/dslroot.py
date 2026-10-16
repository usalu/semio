# 🧬 Lists carved crates whose mounted files use the dsl_derive family (DslArtifact, MutationLeaf, Mutations, diff_text, diff_binary or #[dsl(...)]) and what root path they name; these expansions emit `::semio_framework_os_kernel::...`.
import os, re, sys, collections
OKD = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, OKD + '/../okwp0-tools')
import okcrates as C
crates, model, cargo = C.build()
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]'); INC = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')
def mounted(root):
    seen = []; st = [root]
    while st:
        f = st.pop()
        if f in seen or not os.path.exists(f): continue
        seen.append(f); t = open(f, encoding='utf8').read()
        for m in list(PATH.finditer(t)) + list(INC.finditer(t)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith('.rs'): st.append(p)
    return seen
RX = re.compile(r'(?<![\w])((?:(?:dsl|semio_framework_os_dsl|crate::os_dsl|crate::dsl|semio_framework_os_kernel::os_dsl)::)(?:DslArtifact|MutationLeaf|Mutations|diff_text|diff_binary))|#\[dsl\(|#\[derive\([^)]*\b(?:DslArtifact|MutationLeaf|Mutations)\b')
for k in C.RANK:
    root = C.node_abs(k, model) + '/🦀️.rs'
    if not os.path.exists(root): continue
    hits = collections.Counter(); files = collections.Counter()
    for f in mounted(root):
        t = re.sub(r'(?m)^\s*//.*$', '', open(f, encoding='utf8').read())
        for m in RX.finditer(t):
            hits[m.group(0)[:48]] += 1; files[os.path.relpath(f, C.M)[-60:]] += 1
    if hits: print(k, dict(hits), len(files), 'files')
