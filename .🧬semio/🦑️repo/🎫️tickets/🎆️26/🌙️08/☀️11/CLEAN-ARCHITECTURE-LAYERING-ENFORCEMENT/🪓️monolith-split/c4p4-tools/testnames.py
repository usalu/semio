import os, re, sys, importlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
RX = re.compile(r'((?:#\[[^\]\n]*(?:test)[^\]\n]*\]\s*)+)(?:async\s+)?fn\s+([A-Za-z0-9_]+)', re.M)
tot = 0
bad = 0
for tag in ['rma', 'editor_unit', 'owned', 'diff', 'schema', 'arena', 'digest', 'raster_editor']:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(pfa.T, 'orig', sp['orig']), encoding='utf-8').read()
    base = os.path.dirname(os.path.join(pfa.REPO, sp['file']))
    files = [os.path.join(pfa.REPO, sp['file'])] + [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    a = collections.Counter(m.group(2) for m in RX.finditer(orig))
    b = collections.Counter()
    for f in files:
        b.update(m.group(2) for m in RX.finditer(open(f, encoding='utf-8').read()))
    print(tag, 'test fns orig', sum(a.values()), 'new', sum(b.values()), 'identical', a == b)
    tot += sum(a.values())
    bad += a != b
print('TOTAL test fns', tot, 'mismatching files', bad)
