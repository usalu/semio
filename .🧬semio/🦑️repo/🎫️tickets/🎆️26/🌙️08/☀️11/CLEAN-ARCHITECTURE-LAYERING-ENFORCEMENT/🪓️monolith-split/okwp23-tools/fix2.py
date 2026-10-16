import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ns, apply as A
S = ns.S
def edit(path, pairs):
    t = open(path, encoding='utf8').read()
    for old, new, n in pairs:
        assert t.count(old) == n, (path[-60:], old[:70], t.count(old), n)
        t = t.replace(old, new)
    A.write(path, t)
edit(S + '/🦀️.rs', [('\n\n\n\n\nuse semio_framework_os_store_replay::raw_replay_retirement;\n//#endregion 🔖️EditReplay', '\n//#endregion 🔖️PairMerge\n//#region 🔖️EffectiveForwards\n//#endregion 🔖️EffectiveForwards\n//#region 🔖️EditReplay\nuse semio_framework_os_store_replay::raw_replay_retirement;\n//#endregion 🔖️EditReplay', 1)])
edit(S + '/📒️ledger/📥️lifted/🏪️store/🦀️.rs', [('use semio_framework_os_store_replay::FoldProjection;', 'use crate::lifted_replay::FoldProjection;\nuse semio_framework_os_store_projection::retire_shared_projection;', 1), ('//#endregion 🔖️PairMerge\n//#region 🔖️EffectiveForwards\n', '', 1)])
edit(S + '/🔁️replay/📥️lifted/🏪️store/🦀️.rs', [('//#endregion 🔖️EffectiveForwards\n//#region 🔖️EditReplay\n', '', 1)])
print('ok')
