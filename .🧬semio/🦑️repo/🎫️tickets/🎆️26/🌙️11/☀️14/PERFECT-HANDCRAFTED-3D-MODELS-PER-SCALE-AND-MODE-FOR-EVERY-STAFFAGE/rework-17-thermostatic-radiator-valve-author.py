"""🌡️ Author 17, thermostatic radiator valve: the thermal model made hollow where the product is hollow (valve chamber, head, hand wheel) and given the parts that make it a thermostat: the liquid-filled sensor element behind the cap, the bellows, the pressure pin through the bonnet, the spindle with return spring, the valve disc and its seat. Valve axis = w, the head points to negative w; inlet from below (v), outlet along u."""
import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'thermostatic-radiator-valve'
V = load(SLUG)
W = [0, 0, 1]
O = [0, 0, 0]
S = dict(region='sensor', layer='element')

L = Lists(V, 50)
grip = L.get('body', 'grip')
L.drop('thermal', 'th-valve-chamber', 'th-inlet-neck', 'th-outlet-boss', 'th-base-ring', 'th-scale-band', 'th-grip', 'th-sensor-cap')
L.put('thermal',
      rev('th-valve-chamber', 'body', 'brass', W, O, [[0.0155, 0.0], [0.0155, 0.009], [0.0147, 0.0124], [0.0132, 0.014], [-0.009, 0.014], [-0.0096, 0.015], [-0.0227, 0.015], [-0.0235, 0.0142], [-0.0235, 0.004], [-0.0205, 0.004], [-0.0205, 0.0115], [0.012, 0.0115], [0.0125, 0.008], [0.0125, 0.0], [0.0155, 0.0]], tone=0.6, region='body', layer='wall'),
      tube('th-inlet-neck', 'body', 'brass', [0, 1, 0], O, -0.02, -0.007, 0.0065, 0.011, tone=0.6, region='body', layer='wall'),
      tube('th-outlet-boss', 'body', 'brass', [1, 0, 0], O, 0.004, 0.02, 0.0065, 0.0132, tone=0.6, region='body', layer='wall'),
      after='th-inlet-socket')
L.put('thermal',
      rev('th-base-ring', 'head', 'abs-plastic', W, O, [[-0.025, 0.009], [-0.025, 0.0205], [-0.0265, 0.022], [-0.041, 0.022], [-0.041, 0.0195], [-0.0275, 0.0195], [-0.0275, 0.009], [-0.025, 0.009]], tone=0.85, region='head', layer='shell'),
      rev('th-scale-band', 'knob', 'abs-plastic', W, O, [[-0.035, 0.0225], [-0.035, 0.025], [-0.0365, 0.0265], [-0.046, 0.0265], [-0.046, 0.0252], [-0.048, 0.0252], [-0.048, 0.0225], [-0.035, 0.0225]], tone=0.85, region='head', layer='shell'),
      ext('th-grip', 'knob', 'abs-plastic', 'xy', grip['profile'], grip['from'], grip['to'], holes=[circle(0, 0, 0.021, 24)], tone=0.85, region='head', layer='shell'),
      th('sensor-cap', 'knob', 'shell'),
      after='th-coupling-nut')
L.put('thermal',
      rev('th-sensor-element', 'knob', 'brass', W, O, [[-0.067, 0.0], [-0.067, 0.0105], [-0.065, 0.0125], [-0.052, 0.0125], [-0.05, 0.0105], [-0.05, 0.0]], tone=0.55, **S),
      rev('th-bellows', 'knob', 'brass', W, O, [[-0.05, 0.0], [-0.05, 0.008], [-0.0485, 0.011], [-0.047, 0.008], [-0.0455, 0.011], [-0.044, 0.008], [-0.0425, 0.011], [-0.041, 0.008], [-0.0395, 0.011], [-0.038, 0.008], [-0.038, 0.0]], tone=0.5, **S),
      cyl('th-pressure-pin', 'head', 'brass', W, O, -0.038, -0.016, 0.00225, tone=0.45, **S),
      cyl('th-spindle', 'body', 'brass', W, O, -0.019, 0.0, 0.002, tone=0.45, **S),
      rev('th-return-spring', 'body', 'brass', W, O, [[-0.0185, 0.0035], [-0.0185, 0.006], [-0.016, 0.0045], [-0.0135, 0.006], [-0.011, 0.0045], [-0.0085, 0.006], [-0.006, 0.0045], [-0.006, 0.0035], [-0.0185, 0.0035]], tone=0.4, **S),
      cyl('th-valve-disc', 'body', 'brass', W, O, -0.001, 0.003, 0.0075, tone=0.3, **S),
      tube('th-valve-seat', 'body', 'brass', W, O, 0.003, 0.0065, 0.006, 0.0095, tone=0.5, **S),
      )
L.decide('Thermal:',
         'Thermal: valve chamber, inlet neck, outlet boss, base ring, scale band and hand wheel are shells as the parts are, so that the thermostat shows in section: the liquid-filled sensor element of 25 mm diameter in the hand wheel behind the cap, the bellows with four folds behind it, the 4.5 mm pressure pin through the bonnet, the spindle with its return spring, the valve disc of 15 mm and its seat ring facing the inlet. The inner parts are one hidden region; pipe, nuts and tailpiece stay solid as water-filled brass.',
         'Thermik: Ventilkammer, Zulaufhals, Abgangsstutzen, Sockelring, Skalenband und Handrad sind Schalen wie die Teile selbst, sodass der Thermostat im Schnitt lesbar ist: das flüssigkeitsgefüllte Fühlerelement von 25 mm Durchmesser im Handrad hinter der Kappe, dahinter der Faltenbalg mit vier Falten, der Druckstift von 4,5 mm durch das Oberteil, die Spindel mit Rückstellfeder, der Ventilteller von 15 mm und sein Sitzring zum Zulauf. Die Innenteile sind eine verdeckte Region; Rohr, Muttern und Anschlussstück bleiben massiv als wassergefülltes Messing.')

L = Lists(V, 100)
L.put('thermal',
      cyl('th-sensor-element', 'knob', 'brass', W, O, -0.067, -0.05, 0.0125, tone=0.55, **S),
      cyl('th-bellows', 'knob', 'brass', W, O, -0.05, -0.038, 0.0095, tone=0.5, **S),
      cyl('th-valve-disc', 'body', 'brass', W, O, -0.016, 0.003, 0.0075, tone=0.3, **S))
L.decide('Thermal: the sensor', 'Thermal: the sensor element, the bellows behind it and the valve disc on its spindle stand inside the head and the chamber as one hidden region.', 'Thermik: das Fühlerelement, der Faltenbalg dahinter und der Ventilteller auf seiner Spindel stehen als eine verdeckte Region in Kopf und Kammer.')
for scale in (200, 500):
    L = Lists(V, scale)
    L.put('thermal', cyl('th-sensor-element', 'knob', 'brass', W, O, -0.067, -0.038 if scale == 200 else -0.04, 0.0125, tone=0.55, **S))
    L.decide('Thermal: the sensor', 'Thermal: the sensor element with its bellows stands inside the head as one hidden body.', 'Thermik: das Fühlerelement mit seinem Faltenbalg steht als ein verdeckter Körper im Kopf.')
save(SLUG, V)
print('saved', {s: tuple(len(V[s][l]) for l in ('body', 'thermal', 'structure')) for s in V})

AREA = 0.0064


def root(asset):
    regions = asset['thermal']['regions']
    if not any(r['id'] == 'sensor' for r in regions):
        regions.append({
            'id': 'sensor', 'title': {'en': 'Sensor element, pin and valve disc', 'de': 'Fühlerelement, Stift und Ventilteller'}, 'kind': 'surface', 'model': 'passive', 'material': 'brass',
            'areaM2': AREA, 'characteristicLengthM': 0.025, 'geometry': 'cylinder', 'skyViewFactor': 0,
            'layers': [{'id': 'element', 'title': {'en': 'Brass capsule wall 0.5 mm', 'de': 'Messing-Kapselwand 0,5 mm'}, 'kind': 'metal', 'material': 'brass', 'thicknessM': 0.0005, 'resistanceM2KW': 4.2e-06, 'heatCapacityJM2K': 1615, 'source': 'authored-estimate', 'reference': 'incropera-2018'}],
            'heatCapacityJM2K': 1615, 'source': 'authored-estimate', 'reference': 'incropera-2018',
            'hidden': {'en': 'The liquid-filled sensor element sits in the hand wheel behind the cap, its bellows pushes the pressure pin through the bonnet onto the spindle, and the valve disc closes on its seat against the return spring; all of it is inside the head and the brass chamber and no drawing shows it. The region is passive: the room air reaches the element through the cap, which the knob region models.', 'de': 'Das flüssigkeitsgefüllte Fühlerelement sitzt im Handrad hinter der Kappe, sein Faltenbalg drückt den Druckstift durch das Oberteil auf die Spindel, und der Ventilteller schließt gegen die Rückstellfeder auf seinem Sitz; alles liegt im Kopf und in der Messingkammer, und keine Zeichnung zeigt es. Die Region ist passiv: die Raumluft erreicht das Element durch die Kappe, die die Region des Knopfs abbildet.'}})
    for r in regions:
        if r['id'] == 'sensor':
            r['areaM2'] = AREA
    for pose in asset['poses']:
        pose['thermal']['regions'].setdefault('sensor', {'exposedFraction': 1, 'sunlitFraction': 0})


edit_catalog(SLUG, ['poses', 'thermal'], root)
