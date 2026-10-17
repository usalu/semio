F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs'
SPEC = dict(
    orig='brush.rs.txt',
    file=F,
    nest=False,
    children=[
        dict(folder='🌀️vortex-compatibility', mod='vortex_compatibility', ranges=[(32, 226)],
             doc='//! 🌀️ Vortex compatibility of the brush: the vortex context, the port shape rules, the attraction compatibility and the stack pairs.'),
        dict(folder='🎯️candidates-and-placement', mod='candidates_and_placement', ranges=[(229, 606)],
             doc='//! 🎯️ Brush candidates and placement: the candidate ranking, the kind weights and the placement of one brush candidate on a snapshot.'),
        dict(folder='⏯️suggestions-run', mod='suggestions_run', ranges=[(610, 1147)],
             doc='//! ⏯️ Brush suggestions run: the interactive job that searches brush suggestions around a drag.'),
    ],
    external=['✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs'],
)
