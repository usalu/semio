from specs.lib import *
BA = PL + '/🏗️builder/🧩️app'
H = 'impl AppBuilder {'
B = [472, 631, 834]
CH = [
    ch('📐', 'specs-and-validators', 'specs_and_validators', [(82, 268)],
       'Specs and validators of the app builder: the reserved contributions, the mode, window kind, panel tab and keybinding specs, their definition conversions and the argument and element id validators.'),
    ch('🧱', 'builder-identity-and-modes', 'builder_identity_and_modes', [(301, 472)],
       'Builder identity and modes: the app builder constructor and the artifact kind, config, io, command grammar, media port, terminology, onboarding, dialog, icon, document and mode setters.'),
    ch('🪟', 'window-kinds-and-panels', 'window_kinds_and_panels', [(474, 634)],
       'Window kinds and panels of the app builder: the window kind setters with engagement, measures, actions and utilities, the named and default layouts, the panel tabs and the keybinding.'),
    ch('🎬', 'actions-commands-and-tools', 'actions_commands_and_tools', [(636, 840)],
       'Actions, commands and tools of the app builder: the mutation, view and shell actions with their descriptions, audience, destructive and interactive job flags, the commands, utilities, interactions and tools.'),
    ch('🏗', 'try-build-definition', 'try_build_definition', [(842, 1405)],
       'Try build definition of the app builder: the validating assembly of the app definition and the panicking build wrapper.'),
    ch('📚', 'example-source-and-app', 'example_source_and_app', [(1413, 1530), (1535, 1581)],
       'Example source and app: the example source body and leaf, the built app with its examples and the asynchronous builder entry.'),
]
SPEC = dict(orig='ba.rs', file=BA + '/🦀️.rs', children=CH, pre=split_impl(H, B), expect_extra=split_extra(H, B),
            external=mounted(BA + '/🦀️.rs', orig='ba.rs') + [PL + '/🧪️tests/🔬️app-merge-ui-values/🦀️.rs', PL + '/🧪️tests/🔬️app-example-source/🦀️.rs'])
