"""A law whose independent Python oracle is missing must fail, not return green: rewrites `if(!python)return;` style guards in test files into a thrown refusal. usage: oracle-required.py [--dry]"""
import os, re, sys
R = '/Users/ueli/Documents/semio/'
rx = re.compile(r'if\s*\(\s*(!\s*(?P<a>python\w*|oracleProcess)|(?P<b>python\w*)\s*===\s*null)\s*\)\s*return\s*;')
changed = []
for base in ('🧰️framework', '✏️s', '🚀️launcher'):
    for root, dirs, names in os.walk(R + base):
        dirs[:] = [d for d in dirs if d not in ('node_modules', '.cache', '🗑️generated', 'target', 'dist') and not d.startswith('.')]
        if '/🧪️tests' not in root + '/' and not root.endswith('🧪️tests'): continue
        for n in names:
            if not n.endswith('.ts'): continue
            f = os.path.join(root, n)
            t = open(f, encoding='utf-8').read()
            def sub(m):
                v = m.group('a') or m.group('b')
                return 'if(!%s)throw Error("Independent Python oracle required");' % v
            new, k = rx.subn(sub, t)
            if k:
                changed.append((f[len(R):], k))
                if '--dry' not in sys.argv: open(f, 'w', encoding='utf-8').write(new)
for f, k in changed: print(k, f[-90:])
print(len(changed), 'files', sum(k for _, k in changed), 'sites')
