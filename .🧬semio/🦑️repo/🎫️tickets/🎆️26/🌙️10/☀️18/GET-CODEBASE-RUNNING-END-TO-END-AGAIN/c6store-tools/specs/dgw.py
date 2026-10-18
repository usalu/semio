from specs.lib import *
F = STORE + '/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs'
CH = [
    ch('🧾', 'group-schema-constants-anchors-and-decision-errors', 'group_schema_constants_anchors_and_decision_errors', [(16, 150)],
       'Group schema constants, anchors and decision errors: the durable group schema, role and size constants, the anchor, member and prepared outcome records, the three member unsigned record and the decision error.'),
    ch('📋', 'outcome-and-journal-receipt-types', 'outcome_and_journal_receipt_types', [(152, 305)],
       'Outcome and journal receipt types: the unbound and bound one item outcomes, the prepared and verified outcomes, the journal receipt, the verified three edits, the journal advance, commit and sink and the durable group root.'),
    ch('🏗', 'staged-root-retirement-and-commit-phases', 'staged_root_retirement_and_commit_phases', [(307, 593)],
       'Staged root retirement and commit phases: the staged root retirement, the retained stacks, the three store commit phases and advances, the map recovery and the map assembly phases.'),
    ch('📦', 'outcome-codecs-json-budget-and-map-handles', 'outcome_codecs_json_budget_and_map_handles', [(595, 1008)],
       'Outcome codecs, JSON budget and map handles: the outcome parsing and verification, the canonical json parse and budget, the dsl, pack and snapshot codecs of the outcomes and the map handles and frontiers.'),
    ch('🧪', 'member-identity-and-group-testing-helpers', 'member_identity_and_group_testing_helpers', [(1010, 1125)],
       'Member identity and group testing helpers: the unsigned member, the member identity matches and the durable group testing outcome and member helpers.'),
]
SPEC = dict(orig='dgw.rs', file=F, children=CH, wiring='block', external=crate_ext(STORE + '/🧩️composition/🗄️durable-group/📜️wire/🦀️.rs', F, orig='dgw.rs', children=CH))
