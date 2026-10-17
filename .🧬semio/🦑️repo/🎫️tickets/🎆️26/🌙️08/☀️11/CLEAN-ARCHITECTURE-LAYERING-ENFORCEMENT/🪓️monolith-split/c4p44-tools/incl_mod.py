"""🧭 c4p44 include resolution proof for the six module-style splits on the LIVE tree: the multiset of absolute include_str!/include_bytes!/include! targets of the snapshot (resolved against the original folder) versus root plus children (resolved per file); every target must exist. usage: incl_mod.py"""
import collections, importlib, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p44 as P
RX = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
ok = True
for t in 'pac gwb inf cat sch ses'.split():
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.join(P.REPO, os.path.dirname(sp['file']))
    orig = collections.Counter(os.path.normpath(os.path.join(base, m)) for m in RX.findall(open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()))
    new = collections.Counter()
    files = [(base, os.path.join(P.REPO, sp['file']))] + [(os.path.join(base, c['folder']), os.path.join(base, c['folder'], '🦀️.rs')) for c in sp['children']]
    for d, f in files:
        for m in RX.findall(open(f, encoding='utf-8').read()):
            new[os.path.normpath(os.path.join(d, m))] += 1
    missing = [p for p in new if not os.path.exists(p)]
    only_new = new - orig
    only_orig = orig - new
    print(t, 'orig', sum(orig.values()), 'new', sum(new.values()), 'missing-targets', len(missing), 'only-orig', len(only_orig), 'only-new', len(only_new))
    for p in only_orig: print('   ORIG-ONLY', p[-80:])
    for p in only_new: print('   NEW-ONLY', p[-80:])
    if missing: ok = False
print('INCLUDE-TARGETS', 'OK (only the declared own-tree concat differs)' if ok else 'FAILED')
