"""💧 Author 17, wet-rotor circulation pump: the one triangle dump of the model, the suction divider of the volute, replaced by the turned part it is: a cast disc of 94 mm and 5 mm wall that stands obliquely in the volute (70 degrees to the shaft) and parts the suction channel behind the impeller from the pressure side. Housing bone: u along the pipe, v across, w along the shaft."""
import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'circulation-pump'
V = load(SLUG)
DU, DW = 0.092, 0.0334
N = math.hypot(DU, DW)
AXIS = [round(-DW / N, 6), 0, round(DU / N, 6)]
RADIUS = {50: 0.047, 100: 0.047, 200: 0.0465, 500: 0.046}
for scale in (50, 100, 200, 500):
    L = Lists(V, scale)
    L.put('body', cyl('suction-divider', 'housing', 'cast-iron', AXIS, [0.1055, -0.003, 0.0355], -0.0025, 0.0025, RADIUS[scale], tone=0.37))
    # thermal: volute and stator housing as shells, the wet rotor inside as a hidden region
    L.drop('thermal', 'th-casing', 'th-motor-head')
    L.put('thermal',
          rev('th-casing', 'housing', 'cast-iron', [0, 0, 1], [0.1055, -0.003, 0], [[0.012, 0.02], [0.012, 0.048], [0.064, 0.048], [0.067, 0.045], [0.067, 0.0], [0.063, 0.0], [0.063, 0.042], [0.061, 0.044], [0.016, 0.044], [0.016, 0.02], [0.012, 0.02]], tone=0.4, region='casing', layer='shell'),
          after=None)
    L.put('thermal', th('motor-head', 'motor', 'head'), after='th-motor-flange')
    L.v['thermal'] = [s for s in L.v['thermal'] if s['region'] != 'rotor']
    for sid in ('rotor-drum', 'drive-shaft', 'impeller-back', 'impeller-vane', 'impeller-eye', 'wet-rotor-can'):
        L.put('thermal', own(L.get('body', sid), 'rotor', 'drum'))
    L.decide('Thermal: the wet rotor',
             'Thermal: the wet rotor is a hidden region of its own: the volute and the stator housing are shells, and inside them stand the rotor drum on its ceramic shaft, the impeller with back plate, six vanes and eye ring, and the stainless can that parts the water from the stator; the oblique suction divider of the volute is one turned disc.',
             'Thermik: der Nassläufer ist eine eigene verdeckte Region: Spiralgehäuse und Statorgehäuse sind Schalen, und darin stehen die Läufertrommel auf ihrer Keramikwelle, das Laufrad mit Rückscheibe, sechs Schaufeln und Saugring und das Edelstahl-Spaltrohr, das das Wasser vom Stator trennt; die schräge Saugtrennwand des Spiralgehäuses ist eine gedrehte Scheibe.')
save(SLUG, V)
print('saved', os.path.getsize('%s/volumes/appliances/%s.json' % (T, SLUG)))


def root(asset):
    regions = asset['thermal']['regions']
    if not any(r['id'] == 'rotor' for r in regions):
        regions.append({
            'id': 'rotor', 'title': {'en': 'Wet rotor with impeller and can', 'de': 'Nassläufer mit Laufrad und Spaltrohr'}, 'kind': 'surface', 'model': 'passive', 'material': 'rotor-stainless',
            'areaM2': AREA, 'characteristicLengthM': 0.034, 'geometry': 'cylinder', 'skyViewFactor': 0,
            'layers': [{'id': 'drum', 'title': {'en': 'Stainless rotor sleeve 0.7 mm', 'de': 'Edelstahl-Läuferhülse 0,7 mm'}, 'kind': 'metal', 'material': 'rotor-stainless', 'thicknessM': 0.0007, 'resistanceM2KW': 4.7e-05, 'heatCapacityJM2K': 2770, 'source': 'authored-estimate', 'reference': asset['thermal']['regions'][0]['reference']}],
            'heatCapacityJM2K': 2770, 'source': 'authored-estimate', 'reference': asset['thermal']['regions'][0]['reference'],
            'hidden': {'en': 'The rotor drum on its ceramic shaft, the impeller of 62 mm with six vanes and the stainless can stand in the pumped water inside the volute and the stator housing; only the shaft end shows behind the window. The region is passive: the losses of the motor leave through the motor and casing regions.', 'de': 'Die Läufertrommel auf ihrer Keramikwelle, das Laufrad von 62 mm mit sechs Schaufeln und das Edelstahl-Spaltrohr stehen im Förderwasser im Spiral- und im Statorgehäuse; nur das Wellenende zeigt sich hinter dem Fenster. Die Region ist passiv: die Verluste des Motors gehen über die Regionen Motor und Gehäuse ab.'}})
    for r in regions:
        if r['id'] == 'rotor':
            r['areaM2'] = AREA
    for pose in asset['poses']:
        pose['thermal']['regions'].setdefault('rotor', {'exposedFraction': 1, 'sunlitFraction': 0})


AREA = 0.038
edit_catalog(SLUG, ['poses', 'thermal'], root)
