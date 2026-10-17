from specs.common import ch, V
E = '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation'

SPEC = dict(
    orig='animation.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    modules=[
        dict(
            name='animation',
            folder='🎯' + V + 'core-animations',
            doc='//! 🎯' + V + ' Core animations of the animate engine: the animation trait with its enum dispatch and the leaf, composite, builder and effect animations. See [`super`].',
            inner=dict(
                nest=False,
                children=[
                    ch('🍃', 'leaf-animations', 'leaf_animations', [(73, 350)], 'Leaf animations of the core animation set: create, fade in and out, transform, rotate and move along a path.'),
                    ch('🪢', 'composite-animations', 'composite_animations', [(352, 652)], 'Composite animations of the core animation set: groups, successions, lagged starts and the wait.'),
                    ch('🪄', 'builder-effects', 'builder_and_effect_animations', [(654, 981)], 'Builder and effect animations of the core animation set: the animate builder and the shift, apply-method, focus, blink, traced path and change-speed animations.'),
                ],
                external=[E + '/🧪️tests/🔬️animation-unit/🦀️.rs'],
            ),
        ),
        dict(
            name='animations_catalog',
            folder='🗃' + V + 'animations-catalog',
            doc='//! 🗃' + V + ' Extended animation catalog of the animate engine: the transform, grow, wave, swap and write animations beyond the core set. See [`super`].',
            inner=dict(
                nest=False,
                children=[
                    ch('🎆', 'transform-highlight', 'transform_and_highlight_animations', [(1081, 1498)], 'Transform and highlight animations of the extended catalog: draw-border, fade and replacement transforms, move to target, restore, flash and circumscribe.'),
                    ch('🌊', 'grow-wave-cycle', 'grow_wave_and_cycle_animations', [(1500, 1936)], 'Grow, wave and cycle animations of the extended catalog: grow from a point, shrink, spin in, decimal change, broadcast, wave, wiggle and cyclic replace.'),
                    ch('✍', 'swap-flash-write', 'swap_flash_and_write_animations', [(1938, 2420)], 'Swap, flash and write animations of the extended catalog: swap, matching shapes, homotopy, passing flash, spiral in, uncreate, write, grow from center, indicate and rotating.'),
                ],
                external=[E + '/🧪️tests/🔬️animations-catalog-unit/🦀️.rs'],
            ),
        ),
    ],
    external=[],
)
