from specs.lib import *
F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs'
c = Cut(F, 'pz5w.rs')
c.child('🍓', 'config-diffs', 'config_diffs', [(402, 505), (510, 669)], 'Config diffs: the board and world window config diff records, their cold diff algebra and mutation diff impls.')
c.child('🥦', 'transient-diff', 'transient_diff', [(693, 793)], 'Transient diff: the window transient diff record, its diff algebra and the preflight and transfer hooks.')
c.child('🌽', 'resolution', 'resolution', [(845, 1039)], 'Resolution: the window config and transient resolution from views, snapshots and runtimes.')
SPEC = c.spec(keep_empty=('TaxonomyMounts',))
