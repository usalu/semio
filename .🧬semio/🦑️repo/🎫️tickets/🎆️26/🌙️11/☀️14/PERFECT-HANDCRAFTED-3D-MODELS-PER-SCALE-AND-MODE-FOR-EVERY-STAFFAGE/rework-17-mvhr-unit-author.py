"""🌬️ Author 17, MVHR unit: the extract fan rebuilt as a real radial fan (hollow scroll with tangential outlet under the exhaust spigot, forward-curved wheel, motor), a condensate tray with drain under the plate pack. Casing bone: u = height, v = lateral (left +), w = depth (front negative)."""
import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'mvhr-unit'
V = load(SLUG)
C = 'casing'
W = [0, 0, 1]
CU, CV = 0.49, 0.17
EF = dict(region='extract-fan', layer='scroll')


def scroll(r0, r1, length, step, inset=0.0):
    """🐚 Scroll outline in (u, v): the radius grows from r0 at the tongue to r1 over 360 degrees, then the outlet runs up along the left side."""
    n = int(360 / step)
    pts = []
    for k in range(n):
        phi = math.radians(-90 + step * k)
        r = r0 + (r1 - r0) * k / n - inset
        pts.append([CU + r * math.cos(phi), CV - r * math.sin(phi)])
    if inset == 0:
        return pts + [[CU, CV + r1], [CU + length, CV + r1], [CU + length, CV + r0]]
    return pts + [[CU, CV + r1 - inset], [CU + length - inset, CV + r1 - inset], [CU + length - inset, CV + r0 + inset], [CU - inset, CV + r0 + inset]]


L = Lists(V, 50)
L.drop('thermal', 'th-extract-fan')
L.put('thermal',
      ext('th-extract-fan', C, 'pp-impeller', 'xy', scroll(0.045, 0.062, 0.085, 30), 0.0, 0.105, holes=[scroll(0.045, 0.062, 0.085, 30, 0.003)], tone=0.5, **EF),
      tube('th-extract-wheel', C, 'pp-impeller', W, [CU, CV, 0.0], 0.012, 0.08, 0.024, 0.038, tone=0.75, **EF),
      cyl('th-extract-motor', C, 'painted-steel', W, [CU, CV, 0.0], 0.012, 0.105, 0.02, tone=0.4, **EF),
      after='th-core')
L.put('thermal',
      box('th-condensate-tray', C, 'epp-foam', [0.015, 0.3, 0.195], [0.1575, 0.0, 0.0125], r=0.004, tone=0.3, region='casing', layer='liner'),
      cap('th-condensate-drain', C, 'pp-impeller', [0.15, 0.1, 0.09], [0.03, 0.1, 0.09], 0.008, tone=0.5, region='casing', layer='liner'),
      after='th-liner-back')
L.decide('Thermal:', *[t for t in [next(x for x in L.v['decisions'] if x['en'].startswith('Thermal:'))] for t in (t['en'] + ' The extract fan is the real radial fan: a hollow scroll of 3 mm wall that grows from 45 to 62 mm with its tangential outlet under the exhaust spigot, the forward-curved wheel of 76 mm and the motor on its axis; under the plate pack lies the condensate tray with its drain down to the casing floor.', t['de'] + ' Der Abluftventilator ist der wirkliche Radialventilator: eine hohle Spirale mit 3 mm Wand, die von 45 auf 62 mm wächst, mit tangentialem Auslass unter dem Fortluftstutzen, das vorwärtsgekrümmte Laufrad von 76 mm und der Motor auf seiner Achse; unter dem Plattenpaket liegt die Kondensatwanne mit ihrem Ablauf zum Gehäuseboden.')])

L = Lists(V, 100)
L.drop('thermal', 'th-extract-fan')
L.put('thermal',
      ext('th-extract-fan', C, 'pp-impeller', 'xy', scroll(0.046, 0.062, 0.085, 45), 0.0, 0.105, holes=[scroll(0.046, 0.062, 0.085, 45, 0.003)], tone=0.5, **EF),
      box('th-condensate-tray', C, 'epp-foam', [0.015, 0.3, 0.195], [0.1575, 0.0, 0.0125], tone=0.3, region='casing'),
      after='th-core')
L.decide('Thermal:', *[t for t in [next(x for x in L.v['decisions'] if x['en'].startswith('Thermal:'))] for t in (t['en'] + ' The extract fan is a scroll with its tangential outlet under the exhaust spigot (hollow, the wheel inside is below the feature), and the condensate tray lies under the plate pack.', t['de'] + ' Der Abluftventilator ist eine Spirale mit tangentialem Auslass unter dem Fortluftstutzen (hohl, das Laufrad darin liegt unter dem Merkmal), und die Kondensatwanne liegt unter dem Plattenpaket.')])

save(SLUG, V)
print('saved', {s: tuple(len(V[s][l]) for l in ('body', 'thermal', 'structure')) for s in V})

set_area(SLUG, {'extract-fan': 0.15})
patch_catalog(SLUG, [
    ('The extract fan and its scroll sit in the upper left of the lining under the exhaust spigot; no drawing shows them.', 'The extract fan sits in the upper left of the lining under the exhaust spigot: a hollow scroll with tangential outlet, the forward-curved wheel and the motor, 0.15 m2 of surface inside and out; no drawing shows them.'),
    ('Der Abluftventilator und seine Spirale sitzen oben links in der Auskleidung unter dem Fortluftstutzen; keine Zeichnung zeigt sie.', 'Der Abluftventilator sitzt oben links in der Auskleidung unter dem Fortluftstutzen: eine hohle Spirale mit tangentialem Auslass, das vorwärtsgekrümmte Laufrad und der Motor, 0,15 m2 Oberfläche innen und außen; keine Zeichnung zeigt sie.'),
])
