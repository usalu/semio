"""⚓ C6DB2 anchor census: every string literal (>= 8 chars) of the audited ts files that occurs in a snapshot of a split root must occur the same number of times in the live production module tree of that root (the text the policy readers produce). usage: anchor_counts.py <tag>..."""
import os, re, subprocess, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
R = '/Users/ueli/Documents/semio'
ft = dict(l.rstrip('\n').split('\t') for l in open(HERE + '/files.tsv', encoding='utf-8'))
tags = sys.argv[1:]
orig = {t: open(f'{HERE}/orig/{t}.rs', encoding='utf-8').read() for t in tags}
def tree(path):
    out, seen = [], set()
    def walk(p):
        if p in seen or not os.path.isfile(p): return
        seen.add(p)
        t = open(p, encoding='utf-8').read()
        out.append(t)
        d = os.path.dirname(p)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', t):
            q = os.path.normpath(os.path.join(d, m.group(1)))
            if m.group(1).endswith('.rs') and '🧪️tests' not in q.split(os.sep)[-3:] and '🧪️tests' not in m.group(1): walk(q)
        for m in re.finditer(r'include!\("([^"]+)"\)', t):
            q = os.path.normpath(os.path.join(d, m.group(1)))
            if '🧪️tests' not in m.group(1): walk(q)
    walk(path)
    return '\n'.join(out)
live = {t: tree(os.path.join(R, ft[t])) for t in tags}
DB = '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/'
scope = (DB, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/', '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/')
files = [f for f in subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'ls-files', '*.ts'], cwd=R, capture_output=True, text=True).stdout.split('\n') if f.startswith(scope)]
LIT = re.compile(r'"((?:[^"\\\n]|\\.)*)"|\'((?:[^\'\\\n]|\\.)*)\'|`((?:[^`\\]|\\.)*)`', re.S)
def unesc(s):
    return s.replace('\\n', '\n').replace('\\"', '"').replace("\\'", "'").replace('\\`', '`').replace('\\\\', '\\').replace('\\$', '$')
bad = collections.defaultdict(list)
for f in files:
    t = open(os.path.join(R, f), encoding='utf-8').read()
    for m in LIT.finditer(t):
        s = unesc(next(g for g in m.groups() if g is not None))
        if len(s) < 8: continue
        for tag in tags:
            c0 = orig[tag].count(s)
            if c0 and c0 != live[tag].count(s):
                bad[f].append((tag, c0, live[tag].count(s), s))
for f, rows in sorted(bad.items()):
    print('===', f.replace(DB, 'DB/'))
    seen = set()
    for tag, a, b, s in rows:
        if (tag, s) in seen: continue
        seen.add((tag, s))
        print(f'   {tag} orig={a} live={b} {s[:130]!r}')
print('files', len(files), 'mismatching files', len(bad))
