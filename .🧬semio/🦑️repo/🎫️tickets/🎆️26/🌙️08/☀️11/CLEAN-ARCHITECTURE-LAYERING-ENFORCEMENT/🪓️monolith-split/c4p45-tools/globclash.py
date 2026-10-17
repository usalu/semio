"""🕳️ c4p45 glob ambiguity scan: names defined at the top level of a split child that an external glob import of the parent also exports (two globs with one name are ambiguous, a local item used to shadow the external glob).
usage: globclash.py stage|live <tag> <external crate dir relative to repo> ..."""
import importlib, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p45 as P

mode, tag, dirs = sys.argv[1], sys.argv[2], sys.argv[3:]
sp = importlib.import_module('specs.' + tag).SPEC
base = os.path.join(P.SCR, 'stage', tag) if mode == 'stage' else os.path.join(P.REPO, os.path.dirname(sp['file']))
files = [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
inn = sp.get('inner')
if inn:
    files.append(os.path.join(base, inn['folder'], '🦀️.rs'))
names = {}
for f in files:
    doc = P.Doc(open(f, encoding='utf-8').read())
    for it in doc.items:
        if it['name'] and it['kind'] not in ('use', 'impl', 'crate'):
            names.setdefault(it['name'].split('<')[0], set()).add(os.path.basename(os.path.dirname(f)))
DEF = re.compile(r'^\s*pub(?:\([^)]*\))?\s+(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:struct|enum|fn|trait|type|mod|const|static|union)\s+(\w+)', re.M)
MAC = re.compile(r'macro_rules!\s+(\w+)')
USE = re.compile(r'^\s*pub(?:\([^)]*\))?\s+use\s+([^;]+);', re.M)
ext = {}
for d in dirs:
    root = os.path.join(P.REPO, d)
    for dp, dn, fn in os.walk(root):
        dn[:] = [x for x in dn if x not in ('target', 'node_modules', 'dist', '🧪️tests', '🧪️testing', '.git', '🗑️generated')]
        for f in fn:
            if not f.endswith('.rs'):
                continue
            t = open(os.path.join(dp, f), encoding='utf-8', errors='ignore').read()
            for m in DEF.finditer(t):
                ext.setdefault(m.group(1), set()).add(os.path.join(dp, f))
            for m in MAC.finditer(t):
                ext.setdefault(m.group(1), set()).add(os.path.join(dp, f))
            for m in USE.finditer(t):
                for n in re.findall(r'(?:as\s+)?(\w+)\s*(?:,|\}|$)', m.group(1).replace('\n', ' ')):
                    ext.setdefault(n, set()).add(os.path.join(dp, f))
clash = sorted(set(names) & set(ext))
print(tag, 'child top-level names', len(names), 'external exported names', len(ext), 'clashes', len(clash))
for n in clash:
    print('  CLASH', n, sorted(names[n]), [os.path.relpath(x, P.REPO)[-60:] for x in sorted(ext[n])][:2])
sys.exit(1 if clash else 0)
