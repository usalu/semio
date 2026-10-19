from specs.lib import *
F_ = FW = '✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
def pre(text):
    old = '}//#endregion 🧵️RetainedCommands'
    assert text.count(old) == 1
    return text.replace(old, '}\n//#endregion 🧵️RetainedCommands')
CH = [
    ch('🧵', 'command-admission-emission-and-retained-work', 'command_admission_emission_and_retained_work', [(99, 134), (171, 223), (227, 437), (439, 559), (563, 638)],
       'Command admission, emission and retained work: the hidden writer operation and view, the AST topology, the context menu items, the command tool ids and capacities, the command admission and emission, the retained command work with its job factory and the artifact store preparation.'),
    ch('🎛', 'writer-play-app-editor-and-constructor', 'writer_play_app_editor_and_constructor', [(643, 1024), (1028, 1150)],
       'Writer play app editor and constructor: the writer play app with its artifact editor implementation and the writer app constructor with its manifest stitch.'),
]
SPEC = dict(orig='fwr.rs', file=FW, children=CH, wiring='block', external=mounted(FW, orig='fwr.rs'), pre=pre,
            expect_missing=['}//#endregion 🧵️RetainedCommands'], pre_extra=['}', '//#endregion 🧵️RetainedCommands'], external_prod=family_prod(F_, up=5, exclude=[c['folder'] for c in CH]), keep_empty=('TaxonomyMounts', 'Interaction'))
