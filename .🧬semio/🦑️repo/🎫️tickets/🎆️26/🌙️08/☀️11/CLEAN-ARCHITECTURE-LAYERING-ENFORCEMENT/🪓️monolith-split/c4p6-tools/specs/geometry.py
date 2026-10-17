F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs'
SPEC = dict(
    orig='geometry.rs.txt',
    file=F,
    nest=False,
    children=[
        dict(folder='🧱️fixed-owners', mod='fixed_owners', ranges=[(27, 684)],
             doc='//! 🧱️ Fixed owners of the precompute geometry layer: the bookkeeping slot constants, the sub-page reservation and the fixed-capacity vector, map and set owners.'),
        dict(folder='🧮️pose-math', mod='pose_math', ranges=[(687, 1091)],
             doc='//! 🧮️ Pose math of the precompute geometry layer: the rigid vector, point, rotation and pose adapter, the collision mesh shape and the brush placement pose solver.'),
        dict(folder='🧊️collision-index', mod='collision_index', ranges=[(1126, 1993)],
             doc='//! 🧊️ Collision index of the precompute geometry layer: the collision bodies, bounds and the spatial broad-phase index with its incremental mutation and query cursors.'),
        dict(folder='💥️overlap-state-machine', mod='overlap_state_machine', ranges=[(1997, 2233)],
             doc='//! 💥️ Overlap state machine of the precompute geometry layer: the incremental penetration test between two collision bodies.'),
    ],
    external=['✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs'],
)
