from . import _ext
F = '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
SPEC = dict(
    orig='draw-editor.rs.txt',
    file=F,
    wiring='each',
    children=[
        dict(folder='🖐️gesture-owners', mod='gesture_owners',
             ranges=[(175, 531)],
             doc='//! 🖐️ Drawing gesture owners: the gesture tool ids and limits and the fixed operation owners of the gesture and instance operations.'),
        dict(folder='⚙️gesture-jobs', mod='gesture_jobs',
             ranges=[(533, 840)],
             doc='//! ⚙️ Drawing gesture jobs: the gesture operation payload, the retained command decoder, the interactive gesture job and its tool job factory.'),
        dict(folder='🧾️bounded-commands', mod='bounded_commands',
             ranges=[(848, 1140)],
             doc='//! 🧾️ Drawing bounded commands: the bounded, export, image and path tool ids, their publication contracts and the bounded command job factory.'),
        dict(folder='💾️store-preparation', mod='store_preparation',
             ranges=[(1145, 1341)],
             doc='//! 💾️ Drawing store preparation: the one item publication preparation factory and its preparation state.'),
        dict(folder='🔏️proof-catalogs', mod='proof_catalogs', glob=False,
             ranges=[(1346, 1422)],
             doc='//! 🔏️ Drawing proof catalogs: the gesture, path, image, export and bounded command proof records.'),
        dict(folder='🖥️play-app', mod='play_app',
             ranges=[(1427, 1778)],
             doc='//! 🖥️ Drawing play app: the `DrawingPlayApp` editor, its body render and its `ArtifactEditor` implementation.'),
    ],
    external=_ext.mounted(F),
)
