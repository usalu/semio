"""✂️ Scissor lift: the scissor stack hardware was the 1:50 hardware at every scale (3.25 mm bushing walls, 4 mm tube walls, 10 mm pin heads, 208-vertex arms) and the 1:500 guardrails were 40 mm tube. Each scale gets the stack and the rails its smallest feature allows."""
import re
exec(open('rework-10-lib.py').read())
s = Source('scissor-lift')
ST = 'scissor-steel'


def ids(scale, pattern):
    return [i for i in s.ids(scale) if re.fullmatch(pattern, i)]


def rod(solid, r=None):
    """Hollow turned part (closed-loop profile) to a solid one of the same outer radius, or of radius r."""
    a = [p[0] for p in solid['profile']]
    rr = r if r is not None else max(p[1] for p in solid['profile'])
    solid['profile'] = [[min(a), 0], [min(a), rr], [max(a), rr], [max(a), 0]]


ARM = [[-0.045, -0.02], [-0.02, -0.045], [1.48, -0.045], [1.505, -0.02], [1.505, 0.02], [1.48, 0.045], [-0.02, 0.045], [-0.045, 0.02]]

for i in ids(50, r'sleeve-[ab]\d-\d'):
    p = s.get(50, i)['profile']
    s.get(50, i)['profile'] = [[q[0], 0.02 if q[1] < 0.021 else 0.0275] for q in p]
for i in ids(50, r'tube-b\d-(root|mid|tip|rod)'):
    rod(s.get(50, i))

for i in ids(100, r'sleeve-[ab]\d-\d'):
    p = s.get(100, i)['profile']
    s.get(100, i)['profile'] = [[q[0], 0.015 if q[1] < 0.021 else 0.03] for q in p]
for i in ids(100, r'tube-b\d-(root|mid|tip|rod)'):
    rod(s.get(100, i))
for i in ids(100, r'pin-a\d-(root|mid|tip)'):
    s.get(100, i)['profile'] = [[0, 0], [0, 0.032], [0.015, 0.032], [0.015, 0]]

s.drop(200, ids(200, r'sleeve-[ab]\d-\d') + ids(200, r'pin-a\d-(root|mid|tip)'))
for i in ids(200, r'tube-b\d-(root|mid|tip|rod)'):
    rod(s.get(200, i))
s.get(200, 'roller-a4')['profile'] = [[0, 0], [0, 0.033], [0.03, 0.033], [0.03, 0]]
for n in '1234':
    s.put(200, [ext(f'arm-a{n}', f'a{n}', ST, ARM, 'xy', 0.27, 0.32, tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.59]}),
                ext(f'arm-b{n}', f'b{n}', ST, ARM, 'xy', 0.215, 0.265, tone=0.45, repeat={"count": 2, "translation": [0, 0, -0.48]})])

s.drop(500, ids(500, r'sleeve-[ab]\d-\d') + ids(500, r'pin-a\d-(root|mid|tip)') + ids(500, r'pin-shaft-a\d-\d'))
for i in ids(500, r'tube-b\d-(root|mid|tip|rod)'):
    rod(s.get(500, i), 0.0375)
s.get(500, 'roller-a4')['profile'] = [[-0.047, 0], [-0.047, 0.0375], [0.028, 0.0375], [0.028, 0]]
s.get(500, 'roller-slide')['profile'] = [[0, 0], [0, 0.0375], [0.075, 0.0375], [0.075, 0]]
s.get(500, 'cylinder-rod')['profile'] = [[0, 0], [0, 0.0375], [0.47, 0.0375], [0.47, 0]]
BAR = [[-0.045, -0.045], [1.505, -0.045], [1.505, 0.045], [-0.045, 0.045]]
for n in '1234':
    s.put(500, [ext(f'arm-a{n}', f'a{n}', ST, BAR, 'xy', 0.27, 0.345, tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.615]}),
                ext(f'arm-b{n}', f'b{n}', ST, BAR, 'xy', 0.19, 0.265, tone=0.45, repeat={"count": 2, "translation": [0, 0, -0.455]})])
    s.put(500, [dict(cyl(f'joint-a{n}', f'a{n}', ST, [0, 0, 0.27], [0, 0, 1], 0.045, 0.075, repeat={"count": 3, "translation": [0.73, 0, 0]}), member=f'a{n}', part='joint')], lst='structure')
for i in ('post-rear', 'post-front', 'top-rail', 'mid-rail', 'toe-board', 'rear-top-rail', 'gate', 'gate-toe-board', 'ext-post-rear', 'ext-post-front', 'ext-top-rail', 'ext-mid-rail', 'front-top-rail', 'front-mid-rail', 'front-toe-board'):
    b = s.get(500, i)
    b['size'] = [max(x, 0.075) for x in b['size']]
ext_deck = s.get(500, 'deck-ext')
ext_deck['size'] = [0.92, 0.075, 0.66]
ext_deck['transform']['translation'] = [-0.46, 0.0975, 0]

s.decide(50, "Rework 10: pivot bushings have a 7.5 mm wall and the cross tubes are solid rods of their 50 mm diameter (their 4 mm wall is below the feature and their ends are closed by the bushings).", "Überarbeitung 10: die Gelenkbuchsen haben 7,5 mm Wand und die Querrohre sind volle Stäbe ihres 50-mm-Durchmessers (ihre 4-mm-Wand liegt unter dem Merkmal, ihre Enden sind von den Buchsen verschlossen).")
s.decide(100, "Rework 10: bushings with 15 mm wall, pin heads 15 mm, cross tubes as solid rods.", "Überarbeitung 10: Buchsen mit 15 mm Wand, Bolzenköpfe 15 mm, Querrohre als volle Stäbe.")
s.decide(200, "Rework 10: scissor stack modelled anew for the 30 mm feature: arms as 90 x 50 mm bars with chamfered ends, 40 mm pivot pins without bushings and heads, cross tubes as 50 mm rods, 30 mm platform roller.", "Überarbeitung 10: Scherenpaket für das 30-mm-Merkmal neu modelliert: Arme als 90 x 50 mm Stäbe mit gefasten Enden, 40-mm-Gelenkbolzen ohne Buchsen und Köpfe, Querrohre als 50-mm-Stäbe, 30-mm-Plattformrolle.")
s.decide(500, "Rework 10: modelled anew for the 75 mm feature: arms as 90 x 75 mm bars, cross tubes, rollers and the cylinder rod at 75 mm, no separate pins; guardrail posts, rails, gate and toe boards at 75 mm instead of the real 40 mm tube and 30 mm board, the extension deck 75 mm deep.", "Überarbeitung 10: für das 75-mm-Merkmal neu modelliert: Arme als 90 x 75 mm Stäbe, Querrohre, Rollen und Kolbenstange mit 75 mm, keine einzelnen Bolzen; Geländerpfosten, Holme, Tor und Fußleisten mit 75 mm statt des realen 40-mm-Rohrs und 30-mm-Bretts, das Ausschubdeck 75 mm stark.")
s.save()
