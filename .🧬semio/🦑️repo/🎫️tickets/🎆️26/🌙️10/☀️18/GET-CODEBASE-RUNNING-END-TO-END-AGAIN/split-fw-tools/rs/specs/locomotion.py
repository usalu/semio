import os
REPO = '/Users/ueli/Documents/semio'
F = '🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🦀️.rs'
D = os.path.join(REPO, os.path.dirname(F))
NAMES = {n.split('️', 1)[-1] if '️' in n else n: n for n in os.listdir(D)}
def folder(kebab):
    return NAMES[kebab]
def ch(kebab, mod, ranges, doc, **kw):
    f = folder(kebab)
    emoji = f[:-len(kebab)]
    return dict(folder=f, mod=mod, ranges=ranges, doc='//! %s %s See [`super`].' % (emoji, doc), **kw)
EXTERNAL = [
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🧪️tests/🔬️unit/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/🎯️choice/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/👥️population/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/💞️sociability/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/🕰️clock/🦀️.rs',
    '🧰️framework/🛍️products/🐾️pets/🔨️modules/👀️attention/🧪️tests/🔬️unit/🦀️.rs',
]
SPEC = dict(orig='locomotion.rs', file=F, external=EXTERNAL, children=[
    ch('footing', 'footing', [(26, 97)], 'Footing: a complete course and its reserved corridor, installing a validated course, vanishing, dropping and releasing a footing before planning the way down.'),
    ch('courses', 'courses', [(99, 210)], 'Courses: charting a fall, bounce or parachute descent through its exact landing and trying the ways down in contract order.'),
    ch('departure', 'departure', [(212, 335)], 'Departure: setting out for a goal on a perch, attending a partner and the hop that launches a pet to another perch.'),
    ch('motion', 'motion', [(337, 555)], 'Motion: one tick of a stride, slipping, scooting, plunging, flying, arriving and the dust of a disappearance.'),
    ch('hand', 'hand', [(557, 660)], 'Hand: lifting a pet by its scruff, holding, letting go, giving back and tossing it.'),
    ch('outings', 'outings', [(662, 1141)], 'Outings: planning a gear trip over walls, ladders and grappling ropes and committing it only when its corridor is clear.'),
    ch('trips', 'trips', [(1143, 1310)], 'Trips: resting, halting and travelling an installed gear trip, alighting and moving on from a wall or ladder.'),
    ch('mischief', 'mischief', [(1312, 1411)], 'Mischief: reaching a post on foot or with gear, slipping to a distant post and pushing from it.'),
    ch('leaving', 'leaving', [(1413, 1442)], 'Leaving: an actor that has nowhere left to be walks off its perch.'),
])
