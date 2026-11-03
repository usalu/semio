import sys
sys.path.insert(0, r'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️C1')
from lib import *

def by_id(shapes, i):
    return next(s for s in shapes if s['id'] == i)

doc, ind = load('masonry-wall-with-lintel')
rep = doc['asset']['representations']['500']
rep['geometry']['front'].insert(0, rect('reveal', 'wall', 'body', 0.5, -0.2225, 0.91667, 0.445, 1.33333, 0.8, region='masonry-field', material='brick'))
d = rep['decisions']['silhouette']
d['en'] = d['en'].replace('section A-A is two filled blocks (0.9 x 2.1 mm below, 0.9 x 1.0 mm above the opening)', 'section A-A is two filled blocks (0.9 x 2.1 mm below, 0.9 x 1.0 mm above the opening) joined by a light block for the reveal behind the cut (0.9 x 2.7 mm), so the 2.75 m wall reads as one section')
d['de'] = d['de'].replace('ist der Schnitt A-A zwei gefüllte Blöcke (0,9 x 2,1 mm unter, 0,9 x 1,0 mm über der Öffnung)', 'ist der Schnitt A-A zwei gefüllte Blöcke (0,9 x 2,1 mm unter, 0,9 x 1,0 mm über der Öffnung), verbunden durch einen hellen Block für die Laibung hinter dem Schnitt (0,9 x 2,7 mm), sodass die 2,75 m hohe Wand als ein Schnitt lesbar ist')
om = rep['decisions']['omitted']
om['en'] = om['en'].replace('and the reveal behind the cut.', 'and the layers of the reveal behind the cut.')
om['de'] = om['de'].replace('und die Laibung hinter dem Schnitt.', 'und die Schichten der Laibung hinter dem Schnitt.')
save('masonry-wall-with-lintel', doc, ind)

doc, ind = load('glulam-beam')
reps = doc['asset']['representations']
f = by_id(reps['500']['geometry']['front'], 'beam-end')
f['tone'] = 0.2
f['penMm'] = 0.1
d = reps['500']['decisions']['silhouette']
d['en'] += ' The end face is a solid dark block, the honest section of the bar at this size, as high as the side view (0.62 m).'
d['de'] += ' Die Stirnfläche ist ein dunkler Vollblock, der ehrliche Schnitt des Balkens in dieser Größe, so hoch wie die Seitenansicht (0,62 m).'
for sc in ('200', '100'):
    g = reps[sc]['geometry']
    if sc == '200':
        for i in ('bearing-left', 'bearing-right'):
            for p in by_id(g['elevation'], i)['points']:
                p[1] = -0.32
        for p in by_id(g['front'], 'bearing-end')['points']:
            p[1] = 0.0
    else:
        for i in ('bearing-plate-left', 'bearing-plate-right'):
            by_id(g['elevation'], i)['y'] = -0.32
        by_id(g['front'], 'bearing-plate-end')['y'] = 0.0
    k = reps[sc]['decisions']['retained']
    k['en'] += ' Overall height 0.62 m (beam 0.60 m plus 0.02 m plate level) in side and front view, as at 1:50 and 1:500.'
    k['de'] += ' Gesamthöhe 0,62 m (Balken 0,60 m plus 0,02 m Plattenlage) in Seiten- und Vorderansicht, wie bei 1:50 und 1:500.'
save('glulam-beam', doc, ind)
