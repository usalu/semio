from specs.lib import *

CH = [
    ch('🗺', 'type-vocabulary', 'type_vocabulary', [(19, 223)],
       'Type vocabulary of the repo GraphQL schema: the type reference, argument, field, object, interface, union, enum, input and named types, the schema and its scalars.'),
    ch('🧱', 'common-schemas', 'common_schemas', [(227, 302)],
       'Common schemas of the repo GraphQL schema: the shared enums and metrics.'),
    ch('🗃', 'repo-and-codebase-schemas', 'repo_and_codebase_schemas', [(306, 451)],
       'Repo and codebase schemas of the repo GraphQL schema: the repo, technology, bundle, folder and file types.'),
    ch('📜', 'section-and-statute-schemas', 'section_and_statute_schemas', [(453, 579)],
       'Section and statute schemas of the repo GraphQL schema: the section, definition, statute, breach and policy types.'),
    ch('🤝', 'interaction-goal-and-todo-schemas', 'interaction_goal_and_todo_schemas', [(581, 686)],
       'Interaction, goal and todo schemas of the repo GraphQL schema: the interaction, goal, draft and todo types.'),
    ch('🎫', 'ticket-and-contributor-schemas', 'ticket_and_contributor_schemas', [(688, 829)],
       'Ticket and contributor schemas of the repo GraphQL schema: the ticket, checkpoint and contributor types.'),
    ch('⌨', 'inputs-and-operations', 'inputs_and_operations', [(831, 1080)],
       'Inputs and operations of the repo GraphQL schema: the input object types and the query and mutation root types.'),
    ch('🧾', 'build-and-sdl', 'build_and_sdl', [(1082, 1254)],
       'Build and SDL of the repo GraphQL schema: the schema assembly, the reachable type walk, the SDL rendering and the schema inventory.'),
]

SPEC = dict(
    orig='rsc.rs',
    file=RSC + '/🦀️.rs',
    children=CH,
    external=[RSC + '/🧪️tests/🦀️.rs'],
)
