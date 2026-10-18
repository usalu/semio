from specs.lib import *
F = FILES['pe']
def pre(t):
    old = '\nconst PUZZLE2D_RETAINED_PAYLOAD_SCHEMA:'
    assert t.count(old) == 1
    return t.replace(old, '\n//#region 🧵️RetainedCommands' + old)
CH = [
    ch('🔧', 'play-app-retained-commands-and-tool-proofs', 'play_app_retained_commands_and_tool_proofs', [(125, 402)],
       'Play app, retained commands and tool proofs: the puzzle2d play app with its render body, the retained command job factory and the bounded first-step tool proofs.'),
    ch('🧠', 'artifact-editor', 'artifact_editor', [(407, 854)],
       'Artifact editor: the `ArtifactEditor` implementation of the puzzle2d play app and its remaining operations.'),
    ch('🚀', 'app-assembly', 'app_assembly', [(859, 1143)],
       'App assembly: the internal action, fault notice and flag value helpers and the app creation with its manifest stitch.'),
]
SPEC = dict(orig='pe.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='pe.rs'), pre=pre, expect_extra=['//#region 🧵️RetainedCommands'], keep_empty=empties('pe'))
