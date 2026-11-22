"""🔌 Diesel generator: 1:200 and 1:500 were the 1:50 list (8 mm hood sheets, fifteen 2 mm louvre blades per opening, 30 mm louvre frames, 50 mm uprights) and both lists were the same. The discharge hood, the louvres and the frame parts are modelled for each scale; the 2 mm wall and roof skins with their 50 mm wool stay as the thermal sandwich the feature rule keeps at its gauge."""
exec(open('rework-10-lib.py').read())
s = Source('diesel-generator')
CS, AL = 'container-steel', 'aluminium'
HOOD = [[0.0, 0.0], [1.13, 0.0], [1.13, 0.674], [0.3, 0.674], [0.0, 0.394]]

s.get(50, 'exhaust-stack')['profile'] = [[-0.04, 0.0925], [0.33, 0.0925], [0.33, 0.1], [-0.04, 0.1], [-0.04, 0.0925]]
s.get(50, 'rain-cap-skirt')['profile'] = [[-0.012, 0.1425], [0.035, 0.1425], [0.035, 0.15], [-0.012, 0.15], [-0.012, 0.1425]]
s.get(100, 'rain-cap-skirt')['profile'] = [[-0.012, 0.135], [0.035, 0.135], [0.035, 0.15], [-0.012, 0.15], [-0.012, 0.135]]


def louvres(scale, half_depth, half_gauge, count, pitch, first):
    for k in ('1', '2'):
        for side in ('r', 'l'):
            blade = s.get(scale, f'inlet-louvre-{k}-{side}-slat')
            blade['profile'] = [[-half_depth, -0.65], [half_depth, -0.65], [half_depth, 0.65], [-half_depth, 0.65]]
            blade['from'], blade['to'] = -half_gauge, half_gauge
            blade['repeat'] = {"count": count, "translation": [0.0, pitch, 0.0]}
            blade['transform']['translation'][1] = first
            blade.pop('slender', None)


louvres(200, 0.03, 0.015, 8, 0.2, 0.6)
s.get(200, 'hood-side-l').update({'from': 0.87, 'to': 0.9})
s.get(200, 'hood-side-r').update({'from': -0.9, 'to': -0.87})
s.get(200, 'hood-top').update(size=[0.83, 0.03, 1.74], transform={"translation": [0.715, 0.659, 0.0]})
s.get(200, 'hood-back').update(size=[0.03, 0.394, 1.74], transform={"translation": [0.015, 0.197, 0.0]})
s.get(200, 'hood-frame').update({'from': 1.04, 'to': 1.07})
s.get(200, 'hood-slope').update(profile=[[0.0, 0.394], [0.3, 0.674], [0.3205, 0.6521], [0.0205, 0.3721]], **{'from': -0.87, 'to': 0.87})

louvres(500, 0.06, 0.0375, 5, 0.3, 0.7)
for k in '12':
    for side, z in (('r', -1.14), ('l', 1.14)):
        s.get(500, f'inlet-louvre-{k}-{side}-slat')['transform']['translation'][2] = z
s.drop(500, ['hood-side-l', 'hood-side-r', 'hood-top', 'hood-slope', 'hood-back', 'hood-frame', 'louvre-frame-r', 'louvre-frame-l'])
s.put(500, [ext('hood', 'outlet-hood', CS, HOOD, 'xy', -0.9, 0.9, tone=0.4)], region='discharge-hood')
s.get(500, 'wall-end-a').update(size=[0.075, 2.27, 2.276], transform={"translation": [0.0675, 1.335, 0]})
s.get(500, 'wall-end-b').update(size=[0.075, 2.27, 2.276], transform={"translation": [5.9925, 1.335, 0]})
for i, z in (('top-rail-r', -1.1825), ('top-rail-l', 1.1825)):
    s.get(500, i).update(size=[5.704, 0.112, 0.075], transform={"translation": [3.03, 2.526, z]})
for i, x in (('top-end-rail-a', 0.0375), ('top-end-rail-b', 6.0225)):
    s.get(500, i).update(size=[0.075, 0.152, 2.116], transform={"translation": [x, 2.506, 0.0]})
for side, z0, z1 in (('l', 1.115, 1.19), ('r', -1.19, -1.115)):
    for k, x0 in (('1', 0.155), ('2', 1.53)):
        s.get(500, f'intake-upright-{side}-{k}')['profile'] = [[z0, x0], [z1, x0], [z1, x0 + 0.075], [z0, x0 + 0.075]]
s.get(500, 'silencer-saddle').update({'from': 0.4325, 'to': 0.5075})

s.decide(50, "Rework 10: exhaust stack wall and rain cap skirt at 7.5 mm.", "Überarbeitung 10: Auspuffrohrwand und Regenkappenrand mit 7,5 mm.")
s.decide(100, "Rework 10: rain cap skirt at 15 mm.", "Überarbeitung 10: Regenkappenrand mit 15 mm.")
s.decide(200, "Rework 10: modelled for the 30 mm feature instead of the 1:50 parts: discharge hood of 30 mm plates, eight louvre blades of 60 x 30 mm per opening at 200 mm pitch.", "Überarbeitung 10: für das 30-mm-Merkmal statt der 1:50-Teile modelliert: Ausblashaube aus 30-mm-Platten, je Öffnung acht Lamellen 60 x 30 mm im Abstand von 200 mm.")
s.decide(500, "Rework 10: modelled for the 75 mm feature: discharge hood as one closed block, five louvre blades of 120 x 75 mm per opening without frames, end walls, top rails, intake uprights and silencer saddles at 75 mm.", "Überarbeitung 10: für das 75-mm-Merkmal modelliert: Ausblashaube als ein geschlossener Block, je Öffnung fünf Lamellen 120 x 75 mm ohne Rahmen, Stirnwände, Obergurte, Ansaugpfosten und Schalldämpfersättel mit 75 mm.")
s.save()
