"""🔎 C6DB2 moved-literal reader census: every string literal (>= 12 chars) of a tracked ts/tsx/mjs file that occurs in a snapshot of a split root, is absent from the live root file alone, and present in the live module tree. usage: moved_literals.py <tag>... -> per ts file the tags and example literals"""
import os, re, subprocess, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
R = '/Users/ueli/Documents/semio'
ft = dict(l.rstrip('\n').split('\t') for l in open(HERE + '/files.tsv', encoding='utf-8'))
tags = sys.argv[1:]
orig = {t: open(f'{HERE}/orig/{t}.rs', encoding='utf-8').read() for t in tags}
def tree(path):
    out = []
    seen = set()
    def walk(p):
        if p in seen or not os.path.isfile(p): return
        seen.add(p)
        t = open(p, encoding='utf-8').read()
        out.append(t)
        d = os.path.dirname(p)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', t):
            if m.group(1).endswith('.rs'): walk(os.path.normpath(os.path.join(d, m.group(1))))
        for m in re.finditer(r'include!\("([^"]+)"\)', t):
            walk(os.path.normpath(os.path.join(d, m.group(1))))
    walk(path)
    return '\n'.join(out)
root = {t: open(os.path.join(R, ft[t]), encoding='utf-8').read() for t in tags}
live = {t: tree(os.path.join(R, ft[t])) for t in tags}
files = subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'ls-files', '*.ts', '*.tsx', '*.mjs'], cwd=R, capture_output=True, text=True).stdout.split('\n')
LIT = re.compile(r'"((?:[^"\\\n]|\\.)*)"|\'((?:[^\'\\\n]|\\.)*)\'|`((?:[^`\\]|\\.)*)`', re.S)
def unesc(s):
    return s.replace('\\n', '\n').replace('\\"', '"').replace("\\'", "'").replace('\\`', '`').replace('\\\\', '\\').replace('\\$', '$')
res = collections.defaultdict(lambda: collections.defaultdict(list))
for f in files:
    if not f or f.startswith('.🧬semio') or f.startswith('.cache'): continue
    try: t = open(os.path.join(R, f), encoding='utf-8').read()
    except Exception: continue
    if not any(x in t for x in ('🦀', 'rs', 'Rust', 'rust')): pass
    for m in LIT.finditer(t):
        s = unesc(next(g for g in m.groups() if g is not None))
        if len(s) < 12: continue
        for tag in tags:
            if s in orig[tag] and s not in root[tag] and s in live[tag]:
                res[f][tag].append(s)
for f, d in sorted(res.items()):
    print(f'=== {f}')
    for tag, lits in d.items():
        print(f'   {tag}: {len(lits)} moved literal(s), e.g. {lits[0][:110]!r}')
