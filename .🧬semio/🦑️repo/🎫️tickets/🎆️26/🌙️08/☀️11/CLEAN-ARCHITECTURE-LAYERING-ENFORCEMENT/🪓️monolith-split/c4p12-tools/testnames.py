import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p12 as P
RX = re.compile(r'#\[(?:test|[\w:]*async_test[^\]]*)\]((?:\s*#\[[^\]]*(?:\][^\]]*)*?\])*)\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)')
def names(t):
    return sorted(m.group(2) for m in RX.finditer(t))
bad = 0
root0 = os.environ.get('C4P12_ROOT')
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()
    root = os.path.join(root0, tag) if root0 else os.path.join(P.SCR, 'stage', tag)
    new = open(os.path.join(root, '🦀️.rs'), encoding='utf-8').read() + ''.join(open(os.path.join(root, f, '🦀️.rs'), encoding='utf-8').read() for f in P.folders_of(sp))
    a, b = names(orig), names(new)
    ok = a == b
    bad += not ok
    print(tag, 'test fns orig', len(a), 'new', len(b), 'identical' if ok else 'DIFFERENT')
    if not ok:
        print(set(a) ^ set(b))
sys.exit(1 if bad else 0)
