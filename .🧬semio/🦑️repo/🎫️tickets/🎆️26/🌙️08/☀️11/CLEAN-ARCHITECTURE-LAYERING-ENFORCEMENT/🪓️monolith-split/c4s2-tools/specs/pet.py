from specs.lib import *

CH = [
    ch('🧱', 'wall-and-hoist-laws', 'wall_and_hoist_laws', [(158, 295)],
       'The stage cut, line of sight, the hoist and the wall holds, crowns, takes and surveys of the climbing unit laws.', reexport=False),
    ch('🫳', 'grip-climb-slide-and-mantle', 'grip_climb_slide_and_mantle', [(297, 418)],
       'Grip, the climbing speed and clip, sliding and the mantle of the climbing unit laws.', reexport=False),
    ch('🪜', 'ladder-laws', 'ladder_laws', [(420, 604)],
       'The ladder lean, rise, passes, generated stages, climb, carry, toppling and following of the climbing unit laws.', reexport=False),
    ch('🪝', 'grapple-shot-and-haul', 'grapple_shot_and_haul', [(606, 846)],
       'The grapple shot, its perches and ropes, the flying hook, the haul, the swing and the landing of the climbing unit laws.', reexport=False),
    ch('🗺', 'routes-and-wall-lines', 'routes_and_wall_lines', [(848, 1032)],
       'The routes up, over and down, and the lunges, pitches and ways of the wall lines of the climbing unit laws.', reexport=False),
    ch('📜', 'committed-vectors-and-source', 'committed_vectors_and_source', [(1034, 1228)],
       'The constants, the committed wall, ladder and grapple vectors and the exact operations of the design of the climbing unit laws.', reexport=False),
]

SPEC = dict(
    orig='pet.rs',
    file=PET + '/🦀️.rs',
    children=CH,
    external=[],
)
