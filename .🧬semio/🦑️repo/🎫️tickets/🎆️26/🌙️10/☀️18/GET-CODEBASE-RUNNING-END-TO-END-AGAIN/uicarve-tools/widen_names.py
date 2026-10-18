"""🔧 uicarve widen_names: widen definitions by name inside a directory set. usage: widen_names.py <dir>[,<dir>] name[:kind] ..."""
import os, re, sys
sys.path.insert(0, 'lib')
import carve, rs
from carve import R, rd, wr
dirs = [R + '/' + d if not d.startswith('/') else d for d in sys.argv[1].split(',')]
names = sys.argv[2:]
KW = {'fn': r'fn', 'const': r'const', 'static': r'static', 'struct': r'struct', 'enum': r'enum', 'type': r'type', 'trait': r'trait', 'mod': r'mod'}
files = []
for d in dirs:
    for dp, dn, fn in os.walk(d):
        for f in fn:
            if f.endswith('.rs'): files.append(os.path.join(dp, f))
for spec in names:
    name, _, kind = spec.partition(':')
    kinds = [kind] if kind else list(KW)
    hits = 0
    for f in files:
        s = rd(f)
        if name not in s: continue
        new = s
        for k in kinds:
            rx = re.compile(r'(?m)^(?P<ind>[ \t]*)(?P<attrs>(?:#\[[^\n]*\]\s*\n[ \t]*)*)(?P<vis>pub\s*\([^)]*\)\s+|pub\s+)?(?P<mods>(?:unsafe\s+|async\s+|const\s+|extern\s+"[^"]*"\s+)*)(?P<kw>%s)\s+%s\b' % (KW[k], re.escape(name)))
            def rep(m):
                if m.group('vis') and m.group('vis').strip() == 'pub': return m.group(0)
                return m.group('ind') + m.group('attrs') + 'pub ' + m.group('mods') + m.group('kw') + ' ' + name
            n2 = rx.sub(rep, new)
            new = n2
        if new != s:
            wr(f, new); hits += 1; print('widened', name, os.path.relpath(f, R)[-70:])
    if not hits: print('NOT FOUND/ALREADY PUB', name)
