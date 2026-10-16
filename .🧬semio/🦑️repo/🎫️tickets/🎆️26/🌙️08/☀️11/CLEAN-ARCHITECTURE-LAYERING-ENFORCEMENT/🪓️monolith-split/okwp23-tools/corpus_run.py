# usage: corpus_run.py <batch> dry|write   (runs the census through closure.ts, derives shim lines from the kernel root, writes producer + cut rows)
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, okcrates as C, corpus

name, mode = sys.argv[1], sys.argv[2]
crates = open(HERE + f'/../🗑️generated/okwp23-{name}.crates').read().split()
model_ = C.load()[0]
census = {}
for k in crates:
    pkg = C.pkg_abs(k, model_); entry = C.node_abs(k, model_) + '/🦀️.rs'
    best = 0
    for line in subprocess.run(['bun', HERE + '/../okwp0-tools/closure.ts'], env=dict(os.environ, PKG=pkg, ENTRY=entry, FEATURES=''), capture_output=True, text=True, cwd=C.R).stdout.split('\n'):
        m = re.search(r'lines\t(\d+)', line)
        if m: best = max(best, int(m.group(1)))
    census[k] = best
print('census', census)
root = ns.S + '/🦀️.rs'
rt = open(root, encoding='utf8').read()
shims = {}
for k in crates:
    ident = 'semio_framework_' + k.replace('-', '_')
    for l in rt.split('\n'):
        s = l.strip()
        if re.match(r'^(pub(\([a-z]+\))? )?use ' + ident + r'(::\*| as \w+);$', s):
            shims[k] = (root, s); break
rows = corpus.producer_rows(crates, census)
cuts = corpus.cut_rows(crates, shims)
print('shims', {k: v[1] for k, v in shims.items()})
corpus.write_rows(rows, cuts, mode)
