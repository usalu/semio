"""🔗 Excavator, diesel generator and oil pumpjack: the floating body parts that `volumes-connected` names are joined to what carries them (pins, arms, shafts, saddles that reach the roof). The structure load paths of these three are only partly closed by this; see the report."""
import sys
exec(open('rework-10-lib.py').read())
which = sys.argv[1]

if which == 'excavator':
    s = Source('excavator')
    PS = 'paint-steel'
    fine = {x['id']: x for x in s.v['50']['body']}
    s.put(50, [
        {"id": "monitor-arm", "kind": "capsule", "bone": "upper", "material": PS, "role": "detail", "tone": 0.25, "p0": [1.25, 1.15, 0.58], "p1": [1.375, 1.1, 0.47], "r0": 0.012, "r1": 0.012},
        {"id": "mirror-arm-left", "kind": "capsule", "bone": "upper", "material": PS, "role": "detail", "tone": 0.25, "p0": [1.3, 1.45, 1.395], "p1": [1.3, 1.45, 1.355], "r0": 0.012, "r1": 0.012},
    ], region='house')
    s.get(50, 'console-right').update(size=[0.62, 0.2, 0.15], transform={"translation": [0.48, 0.8, 0.615]})
    s.get(50, 'console-left').update(size=[0.62, 0.2, 0.15], transform={"translation": [0.48, 0.8, 1.215]})
    for pipe in ('boom-pipe-right', 'boom-pipe-left'):
        for c in s.get(50, pipe)['sections']:
            c['at'][1] = round(c['at'][1] - 0.004, 4)
    for scale in (50, 100):
        for i in s.ids(scale):
            if i.startswith('grouser-'):
                s.get(scale, i)['size'][1] = 0.036
    s.put(100, [dict(fine['idler-pin']), dict(fine['link-pin'])], region='attachment')
    s.put(200, [dict(fine['boom-pin'])], region='attachment')
    s.put(500, [cyl('boom-cyl-pin', 'upper', PS, [0.552, 0.14, -0.56], [0, 0, 1], 0.0375, 0.96, tone=0.3)], region='hydraulics')
    note = ("Rework 10: floating parts joined: monitor and left mirror on arms, consoles against the seat, boom lines on the boom, grouser ribs seated in the shoe band, link pins at 1:100, the boom foot pin at 1:200 and the boom cylinder foot pin at 1:500.", "Überarbeitung 10: schwebende Teile verbunden: Monitor und linker Spiegel an Armen, Konsolen am Sitz, Auslegerleitungen auf dem Ausleger, Stollen im Plattenband, Laschenbolzen bei 1:100, Auslegerfußbolzen bei 1:200 und Fußbolzen der Auslegerzylinder bei 1:500.")
elif which == 'diesel-generator':
    s = Source('diesel-generator')
    for scale in (50, 100, 200, 500):
        for i in s.ids(scale):
            if i.startswith('attenuator-'):
                s.get(scale, i)['profile'] = [[(1.141 if z > 0 else -1.141) if abs(abs(z) - 1.135) < 1e-9 else z, x] for z, x in s.get(scale, i)['profile']]
        saddle = s.get(scale, 'silencer-saddle')
        low = min(p[0] for p in saddle['profile'])
        saddle['profile'] = [[-0.395 if abs(p[0] - low) < 1e-9 else p[0], p[1]] for p in saddle['profile']]
        s.put(scale, [dict(box('member-silencer-bearer', 'container', 'container-steel', [0.1, 0.06, 2.44], [1.49, 2.55, 0.0], role='body', repeat={"count": 2, "translation": [1.1, 0, 0]}), member='container', part='member')], lst='structure')
    for scale in (200,):
        for i in ('member-hood-side-l', 'member-hood-side-r', 'member-hood-top'):
            s.put(scale, [{"id": i, "of": i.replace('member-', ''), "member": "outlet-hood", "part": "member"}], lst='structure')
    s.drop(500, ['member-hood-side-l', 'member-hood-side-r', 'member-hood-top'], lists=('structure',))
    s.put(500, [{"id": "member-hood", "of": "hood", "member": "outlet-hood", "part": "member"}], lst='structure')
    note = ("Rework 10: the attenuator splitters reach the wall lining, the silencer saddles reach into the roof and bear on two cross bearers of the structure, the hood members are the hood plates themselves.", "Überarbeitung 10: die Kulissen reichen bis an die Wandauskleidung, die Schalldämpfersättel greifen ins Dach und liegen im Tragwerk auf zwei Querträgern, die Haubenstäbe sind die Haubenbleche selbst.")
else:
    s = Source('oil-pumpjack')
    for scale in (200, 500):
        s.put(scale, [cyl('saddle-shaft', 'post', 'painted-steel', [3.85, 0.0, -0.38], [0, 0, 1], 0.06, 0.76, tone=0.3)], region='steel')
        s.put(scale, [cyl('crank-shaft', 'crank', 'cast-iron', [0, 0, -0.76], [0, 0, 1], 0.08, 1.52, tone=0.3)], region='crank')
    note = ("Rework 10: the saddle bearing shaft and the crank shaft are modelled at this scale too, so the walking beam sits on the post and the cranks on the gear reducer.", "Überarbeitung 10: Sattellagerwelle und Kurbelwelle sind auch in diesem Maßstab modelliert, damit der Schwengel auf dem Bock und die Kurbeln am Getriebe sitzen.")
for scale in (50, 100, 200, 500):
    s.decide(scale, *note)
s.save()
