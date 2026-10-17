from specs.lib import *

CH = [
    ch('🗨', 'dialogs', 'dialogs', [(39, 229)],
       'Dialogs of the app surface: the declared modal form dialog, its choices and the neutral tone rule.'),
    ch('🎛', 'modes-and-non-empty-vec', 'modes_and_non_empty_vec', [(233, 390)],
       'Modes of the app surface: the mode definition, the non-empty vector with its conversions and the mode and window kind collections.'),
    ch('🪟', 'window-kinds-and-panels', 'window_kinds_and_panels', [(392, 539)],
       'Window kinds and panels of the app surface: the window kind definition, the panel groups, the panel tab kinds and the panel tab definition.'),
    ch('🧬', 'app-definition', 'app_definition', [(542, 688)],
       'App definition of the app surface: the app reference, the surface app id and the declared app with its builders.'),
    ch('🧭', 'layout-and-action-resolution', 'layout_and_action_resolution', [(690, 837)],
       'Layout and action resolution of an app: the layout of a mode, the effective, missing, unresolved and invalid action arguments and the window and mode tool actions.'),
    ch('📚', 'breadcrumbs-examples-and-command-grammar', 'breadcrumbs_examples_and_command_grammar', [(841, 976)],
       'Breadcrumbs, examples and command grammar of an app: the app breadcrumb and window label, the example definitions, the config spec and the command grammar.'),
    ch('🧩', 'reserved-contributions', 'reserved_contributions', [(981, 1061)],
       'Reserved contributions of the app surface: the contribution a framework reserves in every app and its audience rule.'),
    ch('🕹', 'interaction-window-bodies', 'interaction_window_bodies', [(1066, 1144)],
       'Interaction window bodies of the app surface: the window bodies by domain, the panel leaf body keys and the interaction action definitions.'),
]

TESTS = R + '/🛂️manifest/🧪️tests'

SPEC = dict(
    orig='app.rs',
    file=APP + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[TESTS + '/🧰️testkit/🦀️.rs', TESTS + '/🔬️app-label/🦀️.rs', TESTS + '/🔬️example-picker/🦀️.rs', TESTS + '/🧪️dialog-choices/🦀️.rs', TESTS + '/🧩️reserved-contributions/🦀️.rs'],
)
