"""🔗 C6UI string-path co-location check: serde-style attribute string paths (`skip_serializing_if = "f"`, `default = "f"`, `with = "m"`, `bound`, ...) and other bare identifier strings inside attributes name functions that the compiler resolves at the attribute's module. The ident scan of the splitter ignores them, so this checks that every plain-fn name used that way is defined in the SAME staged file as the attribute (or is not a fn defined anywhere in the split scope).
usage: serdecheck.py <tag>... (stage dir) | C6_LIVE=1 for the live tree"""
import importlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import modsplit as M
ATTR = re.compile(r'(?:skip_serializing_if|default|with|serialize_with|deserialize_with|getter|from|into|try_from|bound)\s*=\s*"([A-Za-z_][A-Za-z0-9_:]*)"')
FN = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*fn\s+([A-Za-z_]\w*)', re.M)
bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.join(M.REPO, os.path.dirname(sp['file'])) if os.environ.get('C6_LIVE') else os.path.join(M.SCR, 'stage', tag)
    files = {}
    for dp, dn, fn in os.walk(base):
        for f in fn:
            if f == '🦀️.rs':
                files[os.path.relpath(os.path.join(dp, f), base)] = open(os.path.join(dp, f), encoding='utf-8').read()
    defs = {}
    for rel, t in files.items():
        for m in FN.finditer(t):
            defs.setdefault(m.group(1), set()).add(rel)
    for rel, t in files.items():
        for m in ATTR.finditer(t):
            path = m.group(1)
            if '::' in path:
                continue
            if path in defs and rel not in defs[path]:
                print('MISPLACED', tag, rel, path, 'defined in', sorted(defs[path]))
                bad += 1
print('serdecheck', 'OK' if not bad else 'FAIL %d' % bad)
sys.exit(1 if bad else 0)
