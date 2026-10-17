from specs.lib import *

CH = [
    ch('🌉', 'json-value-bridges', 'json_value_bridges', [(144, 241)],
       'JSON value bridges of the shell schema: the open JSON value and staged argument aliases and the four value bridge modules the wire types name through `value(with)`.'),
    ch('🧭', 'anchors-and-layout', 'anchors_and_layout', [(245, 351)],
       'Anchors and layout of the shell schema: panel anchors, per-anchor values, the dock layout tree, dock state, extra window instances and icon names.'),
    ch('🔌', 'plugin-runtime-and-overlays', 'plugin_runtime_and_overlays', [(355, 444)],
       'Plugin runtime and overlays of the shell schema: loaded plugins and their supervisor state, active sessions, dialogs, notices and the app role.'),
    ch('🔄', 'sync-merge-and-scope', 'sync_merge_and_scope', [(452, 516)],
       'Sync, merge and scope of the shell schema: sync cards, artifact sync status, inference port status, merge policy and conflicts and the storage scope.'),
    ch('📋', 'shell-state', 'shell_state', [(520, 687)],
       'Shell state of the shell schema: the one shell state record with its default.'),
    ch('🎮', 'shell-command', 'shell_command', [(691, 778)],
       'Shell command of the shell schema: the command vocabulary every host dispatches.'),
    ch('🪢', 'shell-command-value-bridge', 'shell_command_value_bridge', [(782, 1400)],
       'Shell command value bridge of the shell schema: the hand-written `ToValue` and `FromValue` impls of the shell command.'),
    ch('📣', 'events-and-errors', 'events_and_errors', [(1405, 1477)],
       'Events and errors of the shell schema: the shell event vocabulary and the shell error with its display.'),
    ch('🛰', 'capabilities', 'capabilities', [(1481, 1655)],
       'Capabilities of the shell schema: the capability record, the command catalog and the capability listing an MCP gateway advertises.'),
]

SPEC = dict(
    orig='shs.rs',
    file=SHS + '/🦀️.rs',
    children=CH,
    external=mounted(SHS + '/🦀️.rs', orig='shs.rs'),
)
