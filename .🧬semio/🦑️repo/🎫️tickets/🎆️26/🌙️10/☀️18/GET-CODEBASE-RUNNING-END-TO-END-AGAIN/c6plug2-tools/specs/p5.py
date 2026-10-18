from specs.lib import *
F = FILES['p5']
CH = [
    ch('🔧', 'play-app-retained-commands-and-tool-proofs', 'play_app_retained_commands_and_tool_proofs', [(151, 445)],
       'Play app, retained commands and tool proofs: the puzzle5d play app with its render body, the retained command job factory and the bounded first-step tool proofs.'),
    ch('✍', 'artifact-editor', 'artifact_editor', [(448, 964)],
       'Artifact editor: the `ArtifactEditor` implementation of the puzzle5d play app.'),
    ch('🚀', 'app-assembly', 'app_assembly', [(972, 1357)],
       'App assembly: the fault notice, flag value, part kind and interaction definition helpers and the app creation with its manifest stitch.'),
]
SPEC = dict(orig='p5.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='p5.rs'), keep_empty=empties('p5'))
