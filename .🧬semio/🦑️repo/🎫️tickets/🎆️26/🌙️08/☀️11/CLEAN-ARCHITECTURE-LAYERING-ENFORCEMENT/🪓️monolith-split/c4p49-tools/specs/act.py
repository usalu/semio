from specs.lib import *

CH = [
    ch('🧰', 'utilities-commands-and-tools', 'utilities_commands_and_tools', [(26, 186), (191, 221)],
       'Declared utilities, commands and tools of the manifest: the utility, command, OS command catalog and mode tool definitions.'),
    ch('🗂', 'action-kind-and-capability-vocabulary', 'action_kind_and_capability_vocabulary', [(225, 397)],
       'Action kind and capability vocabulary: the history classification of an action, the resource selector, the capability effects, approval, policy, preview, undo, idempotency and execution class.'),
    ch('🎯', 'audience-and-execution', 'audience_and_execution', [(399, 493)],
       'Audience and execution of a capability: the capability audience with its derivation, the interactive job classification and the capability execution shape.'),
    ch('🧬', 'action-semantics', 'action_semantics', [(495, 597)],
       'Action semantics: what an action or command means to an agent, the classification error and the catalog classification validator.'),
    ch('📇', 'action-definition', 'action_definition', [(601, 760)],
       'Action definition: the declared action with its builders and the revert and switch alternative action ids.'),
    ch('⏪', 'history-and-host-event-actions', 'history_and_host_event_actions', [(762, 931)],
       'History, host event and clipboard actions: the framework-owned history verbs, the history command filter, the shell note, the host event input verb and the clipboard actions.'),
    ch('🕹', 'interaction-verbs', 'interaction_verbs', [(936, 1058)],
       'Interaction verbs: the framework-owned selection, hover and granularity action ids, the closed interaction verb type and its refresh scope derivation.'),
    ch('🗃', 'utility-tool-and-document-actions', 'utility_tool_and_document_actions', [(1063, 1129)],
       'Utility, tool and document actions: the active utility and tool actions, the document transfer actions and the default action icon.'),
]

SPEC = dict(
    orig='act.rs',
    file=ACT + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[R + '/🛂️manifest/🪛️utilities/🌅️initial/🦀️.rs'],
)
