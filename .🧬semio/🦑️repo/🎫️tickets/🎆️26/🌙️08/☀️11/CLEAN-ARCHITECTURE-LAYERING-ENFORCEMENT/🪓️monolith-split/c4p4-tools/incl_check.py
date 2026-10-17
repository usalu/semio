import os, re, sys, importlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
RX = re.compile(r'(?:include_str|include_bytes|include)!\(\s*"([^"]*)"')
mode = sys.argv[1]
bad = 0
for tag in sys.argv[2:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(pfa.T, 'orig', sp['orig']), encoding='utf-8').read()
    base_orig = os.path.dirname(os.path.join(pfa.REPO, sp['file']))
    want = collections.Counter()
    for m in RX.finditer(orig):
        if m.group(1).startswith(('/', '$')):
            continue
        want[os.path.normpath(os.path.join(base_orig, m.group(1)))] += 1
    got = collections.Counter()
    if mode == 'stage':
        root = os.path.join(pfa.SCR, 'stage', tag)
        files = [(os.path.join(root, '🦀️.rs'), base_orig)] + [(os.path.join(root, c['folder'], '🦀️.rs'), os.path.join(base_orig, c['folder'])) for c in sp['children']]
    else:
        files = [(os.path.join(pfa.REPO, sp['file']), base_orig)] + [(os.path.join(base_orig, c['folder'], '🦀️.rs'), os.path.join(base_orig, c['folder'])) for c in sp['children']]
    for f, d in files:
        for m in RX.finditer(open(f, encoding='utf-8').read()):
            if m.group(1).startswith(('/', '$')):
                continue
            got[os.path.normpath(os.path.join(d, m.group(1)))] += 1
    missing = [p for p in got if not os.path.exists(p)]
    print(tag, 'includes orig', sum(want.values()), 'new', sum(got.values()), 'same', want == got, 'nonexistent', len(missing))
    if want != got or missing:
        bad += 1
        for p in (want - got):
            print('  only-orig', p[-100:])
        for p in (got - want):
            print('  only-new', p[-100:])
sys.exit(1 if bad else 0)
