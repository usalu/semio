import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p37 as P
RX = re.compile(r'#\[test\]((?:\s*#\[[^\]]*(?:\][^\]]*)*?\])*)\s*(?:pub\s+)?fn\s+(\w+)')
def names(t):
    return sorted(m.group(2) for m in RX.finditer(t))
bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()
    root = os.path.join(P.SCR, 'stage', tag) if len(sys.argv) < 0 else os.environ.get('C4P37_ROOT_' + tag.upper(), os.path.join(P.SCR, 'stage', tag))
    new = open(os.path.join(root, '🦀️.rs'), encoding='utf-8').read() + ''.join(open(os.path.join(root, c['folder'], '🦀️.rs'), encoding='utf-8').read() for c in sp['children'])
    a, b = names(orig), names(new)
    ok = a == b
    bad += not ok
    print(tag, 'tests orig', len(a), 'new', len(b), 'identical' if ok else 'DIFFERENT')
    if not ok:
        print(set(a) ^ set(b))
sys.exit(1 if bad else 0)
