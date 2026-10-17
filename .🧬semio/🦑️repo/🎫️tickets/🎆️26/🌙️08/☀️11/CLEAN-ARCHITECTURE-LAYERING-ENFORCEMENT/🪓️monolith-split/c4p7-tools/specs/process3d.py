D = '✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests'


def pre(t):
    a = '//#region 🔖️MediaTests'
    assert t.count(a) == 1
    return t.replace(a, '//#endregion 🔖️CrossCutting\n\n' + a)


SPEC = dict(
    orig='process3d-editor-unit.rs.txt',
    file=D + '/🔬️unit/🦀️.rs',
    pre=pre,
    pre_extra=['//#endregion 🔖️CrossCutting'],
    wiring='each',
    children=[
        dict(folder='🏭️production-envelope', mod='production_envelope', reexport=False,
             ranges=[(86, 291)],
             doc='//! 🏭️ Process3d editor unit tests: the production envelope helpers and the maintenance swap law.'),
        dict(folder='🧵️command-surface', mod='command_surface', reexport=False,
             ranges=[(294, 746)],
             doc='//! 🧵️ Process3d editor unit tests: the retained command routes, the resumable work, the command round trips and the context menu.'),
        dict(folder='📋️manifest-and-io', mod='manifest_and_io', reexport=False,
             ranges=[(750, 804), (1016, 1049)],
             doc='//! 📋️ Process3d editor unit tests: the manifest stitch, the utilities, the typed media io and the media export and import.'),
        dict(folder='🎮️interaction-behavior', mod='interaction_behavior', reexport=False,
             ranges=[(808, 1011), (1053, 1108)],
             doc='//! 🎮️ Process3d editor unit tests: labels, undo, world pointer and face drag gestures, replay cursor, sun, window bodies and the contribution envelope limits.'),
        dict(folder='🧩️contributions-and-examples', mod='contributions_and_examples', reexport=False,
             ranges=[(1112, 1185), (1189, 1216), (1220, 1295)],
             doc='//! 🧩️ Process3d editor unit tests: the real contribution pack admission, the host contribution output cap and the member-less example archive door.'),
    ],
    external=[D + '/🧰️support/🦀️.rs'],
)
