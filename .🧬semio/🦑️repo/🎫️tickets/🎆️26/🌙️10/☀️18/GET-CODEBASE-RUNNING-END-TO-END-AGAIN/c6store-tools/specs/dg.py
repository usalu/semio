from specs.lib import *
F = STORE + '/🧩️composition/🗄️durable-group/🦀️.rs'
CH = [
    ch('📒', 'group-decision-journal-record-and-snapshot-capture', 'group_decision_journal_record_and_snapshot_capture', [(85, 374)],
       'Group decision, journal record and snapshot capture: the three member decision, the prepared, bound and recovery store records, the journal record with its operations and the owned three snapshot capture.'),
    ch('🛣', 'map-commit-recovery-hosts-and-assembly-owners', 'map_commit_recovery_hosts_and_assembly_owners', [(376, 641)],
       'Map commit, recovery hosts and assembly owners: the three store commit, the map commit operation, host and owners, the map recovery admission, owners, operation, host and start and the map assembly owners and assembly.'),
    ch('🧱', 'map-assembly-open-and-recovery-steps', 'map_assembly_open_and_recovery_steps', [(647, 1053)],
       'Map assembly open and recovery steps: the map assembly operations that open, admit and recover the three member stores.'),
    ch('🚢', 'map-assembly-commit-and-publication-steps', 'map_assembly_commit_and_publication_steps', [(1055, 1474)],
       'Map assembly commit and publication steps: the map assembly operations that commit, publish and verify the three member stores.'),
    ch('🛠', 'map-assembly-recovery-completion', 'map_assembly_recovery_completion', [(1476, 1797)],
       'Map assembly recovery completion: the map assembly operations that complete the recovery, retire the owners and report the phases.'),
    ch('🪪', 'three-member-decision-operations-codecs-and-testing', 'three_member_decision_operations_codecs_and_testing', [(1825, 2136)],
       'Three member decision operations, codecs and testing: the three member decision operations, its dsl, pack and snapshot codecs and the journal record testing helpers.'),
]
_L = 'struct DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {'
SPEC = dict(orig='dg.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='dg.rs'),
            patch={CH[1]['folder']: [('pub(super) ' + _L, 'pub(crate) ' + _L)]},
            expect_missing=['pub(super) ' + _L], expect_extra=['pub(crate) ' + _L])
