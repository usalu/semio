# 🗺️ WP4 crate map: owner prefixes and entry shape per crate for the staged census (writes 🗑️generated/pwp4-crates4.json).
import json
from spec4 import *
R = '⚛️reactor/'
TURN_TESTS = [R + '🔄️turn/🧪️tests/', R + '🔄️turn/📥️receiving/🧪️tests/', R + '🔄️turn/📥️receiving/📦️completed/🧪️tests/', R + '🔄️turn/🎟️receipt/🧪️tests/', R + '🧪️tests/⚛️reactor-driver/', R + '🧪️tests/🔬️m1-m2-reactor/', R + '🧪️tests/🔬️reconcile-budget/', R + '🧪️tests/🔬️reconcile-spin/', R + '🧪️tests/🚚️more-work-drive/', R + '🧪️tests/🧺️turn-patch-batch/', R + '🧪️tests/🧫️fixtures/🔁️reconcile-spin.json', R + '🧪️tests/🧫️fixtures/🚚️more-work-drive.json', R + '🧪️tests/🧫️fixtures/🧺️turn-patch-batch.json']
OWN = {
 'reactor-app': [R + '🔌️app/', R + '🏃️runtime/📥️staging/🔌️reactor-app/'],
 'reactor-tasks': [R + '⏲️tasks/', R + '🧪️tests/🔬️job-render-binding/'],
 'reactor-lifecycle': [R + '📨️pending/', R + '🚪️lifetime/', '🚪️lifetime/', R + '🏃️runtime/📥️staging/♻️reactor-lifecycle/'],
 'runtime-instances': [R + '🏃️runtime/📦️instances/'],
 'runtime-ops': [R + '🏃️runtime/🧰️ops/', R + '🏃️runtime/🎞️command/'],
 'runtime-exchange': [R + '🏃️runtime/🔄️exchange/'],
 'reactor-patches': [R + '🩹️patches/'],
 'reactor-surfaces': [R + '📥️cold-pair/', R + '📸️checkpoint/'],
 'reactor-core': [R + '🦀️.rs', R + '🧪️tests/'],
 'reactor-jobs': [R + '💼️jobs/'],
 'reactor-turn': [R + '🔄️turn/'],
 'reactor-wit-bridge': [R + '🧬️wit-bridge/'],
 'reactor-context': [R + '🎟️context/'],
 'facade': [R + '🏃️runtime/🦀️.rs', '📣️macros/⚛️wp4-reactor/'],
 'tests-07': TURN_TESTS,
}
ENTRY = {   # class B: the module file is the crate root; class A: virtual root in the stub
 'reactor-app': (R + '🔌️app/🦀️.rs', 'B'), 'reactor-tasks': (R + '⏲️tasks/🦀️.rs', 'A'),
 'reactor-lifecycle': (R + '📨️pending/🦀️.rs', 'B'), 'runtime-instances': (R + '🏃️runtime/📦️instances/🦀️.rs', 'A'),
 'runtime-ops': (R + '🏃️runtime/🧰️ops/🦀️.rs', 'A'), 'runtime-exchange': (R + '🏃️runtime/🔄️exchange/🦀️.rs', 'A'),
 'reactor-patches': (R + '🩹️patches/🦀️.rs', 'B'), 'reactor-surfaces': (R + '📥️cold-pair/🦀️.rs', 'B'),
 'reactor-core': (R + '🦀️.rs', 'B'), 'reactor-jobs': (R + '💼️jobs/🦀️.rs', 'B'), 'reactor-turn': (R + '🔄️turn/🦀️.rs', 'B'),
 'reactor-wit-bridge': (R + '🧬️wit-bridge/🦀️.rs', 'A'), 'reactor-context': (R + '🎟️context/🦀️.rs', 'B'),
 'tests-07': ('🧷️companions/🍇️tests-07/🦀️.rs', 'A'),
}
# extra module files a class-B / class-A crate roots must mount virtually (their physical parent is another crate's file)
EXTRA = {
 'reactor-lifecycle': [R + '🚪️lifetime/🦀️.rs'],
 'reactor-surfaces': [R + '📸️checkpoint/🦀️.rs'],
}
out = {'own': OWN, 'entry': ENTRY, 'mounts': {}, 'extra': EXTRA}
out['mounts']['tests-07'] = [x for x in TURN_TESTS if x.endswith('/') is False and x.endswith('.rs')] + [R + '🔄️turn/🧪️tests/' + d + '/🦀️.rs' for d in ['⏱️execution','⚖️invocation-failure','📏️future-size','📡️wire-effect-round-trip','📥️inbound-request','🕹️deferred-render-drain']] + [R + '🔄️turn/📥️receiving/🧪️tests/🦀️.rs', R + '🔄️turn/📥️receiving/📦️completed/🧪️tests/🦀️.rs', R + '🔄️turn/🎟️receipt/🧪️tests/🦀️.rs'] + [R + '🧪️tests/' + d + '/🦀️.rs' for d in ['⚛️reactor-driver','🔬️m1-m2-reactor','🔬️reconcile-budget','🔬️reconcile-spin','🚚️more-work-drive','🧺️turn-patch-batch']]
for _, pieces, _r in CUTS:
    for p in pieces:
        if p.existing or p.crate in ('facade',): continue
        out['mounts'].setdefault(p.crate, []).append(p.path)
json.dump(out, open(GEN + '/pwp4-crates4.json', 'w'), ensure_ascii=False, indent=1)
print(json.dumps(out['mounts'], ensure_ascii=False)[:600])
