# one-off fixes found by the b1 union run (test and lib configs); every edit asserts a unique anchor and logs the file
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ns, widen, apply as A
S = ns.S
def edit(path, pairs):
    t = open(path, encoding='utf8').read()
    for old, new, n in pairs:
        assert t.count(old) == n, (path[-60:], old[:60], t.count(old), n)
        t = t.replace(old, new)
    A.write(path, t)
edit(S + '/📒️ledger/🦀️.rs', [('#[cfg(any(test, feature = "hot-path-census"))] use crate::hot_path_census;\n', '', 1)])
edit(S + '/📨️messages/✂️clamp/🧪️tests/🦀️.rs', [('super::raw_replay_retirement::tests::close', 'super::raw_replay_retirement_tests::close', 2)])
edit(S + '/🧪️tests/🧪️supersede-replay/🦀️.rs', [('super::super::raw_replay_retirement::tests::close', 'super::super::raw_replay_retirement_tests::close', 2)])
K = ns.K
edit(K + '/📜️script.ts', [('os_store::component::canonical_edit::tests::', 'os_store::component::canonical_edit_tests::', 2)])
for rel, names in (('/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs', {'TransferTask'}), ('/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs', {'ArtifactEphemeralTaskPreparation'})):
    p = S + rel
    t = open(p, encoding='utf8').read()
    t2, done = widen.widen_items(t, names)
    assert done == names, (rel, done)
    A.write(p, t2)
print('fixed')
