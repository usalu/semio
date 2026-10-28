"""For each test file of a list, finds every project/script file in the repo whose text names the file's test directory with its last three path segments (streams the files once, low memory). usage: routed-anywhere.py <list-file>"""
import os, sys
R = '/Users/ueli/Documents/semio/'
files = [p for p in open(sys.argv[1], encoding='utf-8').read().split('\n') if p]
needles = {}
for p in files:
    d = p.split('/')[:-1]
    k = max(i for i, x in enumerate(d) if x == '🧪️tests')
    leaf = d[k:]
    keep = d[max(0, k - 2):]
    needles[p] = ('/'.join(keep), '", "'.join(keep), '/'.join(d[-3:]))
hits = {p: [] for p in files}
for base in ('🧰️framework', '✏️s', '🚀️launcher', '.'):
    for root, dirs, names in os.walk(R + base if base != '.' else R, followlinks=False):
        dirs[:] = [x for x in dirs if x not in ('node_modules', '.cache', '.git', '🗑️generated', 'target', 'dist', '.🧬semio', 'bazel-out') and not x.startswith('.')]
        if base == '.' and root != R:
            dirs[:] = []
        for n in names:
            if n in ('📜️script.ts', '📋️project.json') or (n.endswith('.json') and n.startswith('🔣️')) or n == 'package.json':
                f = os.path.join(root, n)
                try:
                    if os.path.getsize(f) > 3_000_000: continue
                    t = open(f, encoding='utf-8', errors='replace').read()
                except OSError: continue
                for p, (a, b, c) in needles.items():
                    if a in t or b in t:
                        hits[p].append(f[len(R):])
for p in files:
    h = [x for x in hits[p] if x != '']
    print(('R ' if h else 'U ') + '/'.join(p.split('/')[-4:-1]) + ('  <- ' + ' | '.join(x[-60:] for x in h[:2]) if h else ''))
