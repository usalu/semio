from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'fsc.rs'), encoding='utf-8').read()
CUTS = [(146, 146, [224, 369]), (428, 428, [571, 620, 763, 884])]

CH = [
    ch('📡', 'emitter-and-management', 'emitter_and_management', [(29, 83)],
       'Emitter and management of the production context: the coordinator event emitter and the provider-backed goal management.'),
    ch('🧭', 'analyze-scope', 'analyze_scope', [(85, 137)],
       'Analyze scope of the production context: the scope kinds an analyze request names and their parsing.'),
    ch('🏠', 'context-core', 'context_core', [(139, 223)],
       'Context core of the production context: the filesystem repo context, its construction, accessors, wiring of the ticket, goal and todo aggregates and its readers.'),
    ch('🎯', 'goal-and-file-support', 'goal_and_file_support', [(225, 370)],
       'Goal and file support of the production context: the goal retitle and milestone support, the file record, the file header and the workspace loading.'),
    ch('🔧', 'refactor-operations', 'refactor_operations', [(372, 430)],
       'Refactor operations of the production context: the move, integrate, extract and rename refactors, the cached breaches and the autofix.'),
    ch('🔎', 'context-queries', 'context_queries', [(432, 574)],
       'Context queries of the production context: the read side of the repo context port, from technologies and bundles to interactions and analyze.'),
    ch('✏', 'goal-todo-and-draft-mutations', 'goal_todo_and_draft_mutations', [(576, 625)],
       'Goal, todo and draft mutations of the production context: the goal, todo and draft commands of the repo context port.'),
    ch('🎫', 'ticket-mutations', 'ticket_mutations', [(627, 770)],
       'Ticket mutations of the production context: the ticket open, close, reopen, change and delete commands of the repo context port.'),
    ch('🧱', 'contributor-and-structure-mutations', 'contributor_and_structure_mutations', [(772, 893)],
       'Contributor and structure mutations of the production context: the contributor, folder, file and section commands and the integrate and extract commands.'),
    ch('🔄', 'management-sync', 'management_sync', [(895, 1102)],
       'Management sync of the production context: the management synchronisation, the section record and label pruning and the goal sync.'),
]

SPEC = dict(
    orig='fsc.rs',
    file=FSC + '/🦀️.rs',
    children=CH,
    pre=split_impls_h(CUTS),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[FSC + '/🧪️tests/🦀️.rs'],
)
