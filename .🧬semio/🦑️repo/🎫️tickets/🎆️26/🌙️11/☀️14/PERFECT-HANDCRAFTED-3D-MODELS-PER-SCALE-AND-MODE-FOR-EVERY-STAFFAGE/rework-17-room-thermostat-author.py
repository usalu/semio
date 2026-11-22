"""🎚️ Author 17, electronic room thermostat: the empty housing shell of the thermal model filled with what makes it a thermostat: the circuit board at the back, the NTC sensor bead at the lower edge where the room air enters, the relay, the encoder under the dial and the terminal block. Housing bone: u up (the housing spans -0.015 to 0.075), v lateral, w depth (face -0.0015, wall 0.0135; the cavity is 10 mm deep)."""
import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'room-thermostat'
V = load(SLUG)
H = 'housing'
E = dict(region='electronics', layer='board')
pcb = lambda: box('th-circuit-board', H, 'circuit-board', [0.074, 0.074, 0.0016], [0.03, 0.0, 0.009], tone=0.35, **E)
L = Lists(V, 50)
L.put('thermal', pcb(),
      ell('th-sensor', H, 'abs-plastic', [0.003, 0.003, 0.0025], [-0.006, 0.031, 0.005], tone=0.2, **E),
      cap('th-sensor-leads', H, 'circuit-board', [-0.006, 0.031, 0.005], [0.0, 0.031, 0.0082], 0.0006, tone=0.5, **E),
      box('th-relay', H, 'abs-plastic', [0.019, 0.01, 0.007], [0.036, 0.026, 0.0047], r=0.001, tone=0.25, **E),
      cyl('th-encoder', 'dial', 'abs-plastic', [0, 0, 1], [0, 0, 0], 0.001, 0.0082, 0.006, tone=0.3, **E),
      box('th-terminal-block', H, 'abs-plastic', [0.008, 0.036, 0.007], [-0.006, -0.012, 0.0047], r=0.001, tone=0.3, **E))
L.decide('Thermal: inside', 'Thermal: inside the 10 mm housing shell stand, as one hidden region, the 74 mm circuit board at the back, the NTC bead on its two leads at the lower edge where the room air enters through the slots, the relay of 19 x 10 x 7 mm, the encoder under the dial and the terminal block; these are the parts whose self-heating (0.14 to 0.7 W) the casing region releases.', 'Thermik: in der 10 mm tiefen Gehäuseschale stehen als eine verdeckte Region die 74 mm große Leiterplatte an der Rückwand, die NTC-Perle an ihren zwei Drähten am unteren Rand, wo die Raumluft durch die Schlitze eintritt, das Relais von 19 x 10 x 7 mm, der Drehgeber unter dem Stellrad und die Klemmleiste; das sind die Teile, deren Eigenerwärmung (0,14 bis 0,7 W) die Gehäuseregion abgibt.')
L = Lists(V, 100)
L.put('thermal', pcb(), box('th-relay', H, 'abs-plastic', [0.019, 0.015, 0.007], [0.036, 0.024, 0.0047], tone=0.25, **E))
L.decide('Thermal: inside', 'Thermal: inside the housing shell the circuit board and the relay stand as one hidden region.', 'Thermik: in der Gehäuseschale stehen Leiterplatte und Relais als eine verdeckte Region.')
for scale in (200, 500):
    L = Lists(V, scale)
    L.put('thermal', box('th-circuit-board', H, 'circuit-board', [0.074 if scale == 200 else 0.07, 0.074 if scale == 200 else 0.07, 0.0016], [0.03, 0.0, 0.009], tone=0.35, **E))
    L.decide('Thermal: inside', 'Thermal: the circuit board stands inside the housing as the one hidden body.', 'Thermik: die Leiterplatte steht als der eine verdeckte Körper im Gehäuse.')
save(SLUG, V)
print('saved', {s: tuple(len(V[s][l]) for l in ('body', 'thermal', 'structure')) for s in V})
AREA = 0.0125


def root(asset):
    if not any(m['id'] == 'circuit-board' for m in asset['materials']):
        asset['materials'].append({'id': 'circuit-board', 'title': {'en': 'Glass-epoxy circuit board', 'de': 'Glasfaser-Epoxid-Leiterplatte'}, 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 1850, 'conductivityWmK': 0.3, 'specificHeatJKgK': 1100, 'source': 'authored-estimate', 'reference': 'incropera-2018'})
    regions = asset['thermal']['regions']
    if not any(r['id'] == 'electronics' for r in regions):
        regions.append({
            'id': 'electronics', 'title': {'en': 'Circuit board, sensor and relay', 'de': 'Leiterplatte, Fühler und Relais'}, 'kind': 'surface', 'model': 'passive', 'material': 'circuit-board',
            'areaM2': AREA, 'characteristicLengthM': 0.074, 'geometry': 'vertical-plate', 'skyViewFactor': 0,
            'layers': [{'id': 'board', 'title': {'en': 'Glass-epoxy board 1.6 mm', 'de': 'Glasfaser-Epoxid-Platte 1,6 mm'}, 'kind': 'polymer', 'material': 'circuit-board', 'thicknessM': 0.0016, 'resistanceM2KW': 0.0053, 'heatCapacityJM2K': 3256, 'source': 'authored-estimate', 'reference': 'incropera-2018'}],
            'heatCapacityJM2K': 3256, 'source': 'authored-estimate', 'reference': 'incropera-2018',
            'hidden': {'en': 'The circuit board with the NTC sensor bead at its lower edge, the relay, the encoder of the dial and the terminal block fill the 10 mm cavity of the housing; no drawing shows them. The region is passive: their 0.14 to 0.7 W are released by the casing region.', 'de': 'Die Leiterplatte mit der NTC-Fühlerperle am unteren Rand, das Relais, der Drehgeber des Stellrads und die Klemmleiste füllen den 10 mm tiefen Hohlraum des Gehäuses; keine Zeichnung zeigt sie. Die Region ist passiv: ihre 0,14 bis 0,7 W gibt die Gehäuseregion ab.'}})
    for r in regions:
        if r['id'] == 'electronics':
            r['areaM2'] = AREA
    for pose in asset['poses']:
        pose['thermal']['regions'].setdefault('electronics', {'exposedFraction': 1, 'sunlitFraction': 0})


edit_catalog(SLUG, ['materials', 'poses', 'thermal'], root)
