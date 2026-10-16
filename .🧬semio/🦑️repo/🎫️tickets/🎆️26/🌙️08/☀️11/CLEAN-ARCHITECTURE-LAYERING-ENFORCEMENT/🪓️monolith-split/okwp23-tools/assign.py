# 🧷 Overrides of the planner's item assignment found by the item-level edge analysis (each is a defect of the plan; RESULT-OS-KERNEL-WP2-3.md lists them).
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import model
S = model.S
PUB = 'os-store-one-item-publication'
OVERRIDES = {
    (S + '/🧵️canonical-edit/🧳️source/🦀️.rs', 'Identities'): PUB,
    (S + '/🧵️canonical-edit/🧳️source/🦀️.rs', 'Returned'): PUB,
    (S + '/🔗️read/🦀️.rs', 'ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS'): 'os-store-replay',
}
SS = 'os-store-space-schema'
SH = 'os-store-space-history'
_SNAP = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs'
_BORROWED = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs'
OVERRIDES.update({
    (S + '/🚪️io/📝️text/📸️snapshot/🦀️.rs', 'record_tree'): SS,
    (S + '/🚪️io/📝️text/🧬️mutations/🦀️.rs', 'record_tree'): SS,
    (_SNAP, 'schema'): SS, (_SNAP, 'rows'): SS, (_SNAP, 'word'): SS, (_SNAP, 'pair'): SS,
    (_BORROWED, "< 'a > Node < 'a >"): SH,
    (_BORROWED, 'Node'): SH,
})
model.OVERRIDE.update(OVERRIDES)

_SNAPDIR = S + '/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot'
for _sub in ('📏️preflight', '📏️preflight/🫳️borrowed', '🚦️native', '🛂️admission', '🛂️validation', '🫴️receiving'):
    model.FILE_OVERRIDE[_SNAPDIR + '/' + _sub + '/🦀️.rs'] = SH

import re
_RET = S + '/♻️retirement/🦀️.rs'
model.OVERRIDE_RX += [
    (_RET, re.compile(r'artifact_retire_struct!\s*\(\s*super::(SpaceMemberPin|SpaceCheckpoint|SpaceAlternative|SpaceHistorySnapshot|CommitSpaceCheckpoint|CreateSpaceAlternative|SwitchSpaceAlternative|RemoveSpaceCheckpoint|RemoveSpaceAlternative|SetActiveSpaceAlternative)\b'), SS),
    (_RET, re.compile(r'RetireOwned for super::SpaceHistoryMutation'), SS),
]

TI = 'os-store-text-io'
for _f in ('/🚪️io/💾️binary/📸️snapshot/🦀️.rs', '/🚪️io/📝️text/📸️snapshot/🦀️.rs', '/🚪️io/📝️text/🧬️mutations/🦀️.rs'):
    model.FILE_OVERRIDE[S + _f] = TI
model.FILE_OVERRIDE[_SNAP] = SH
