"""🛢️ Author 17, indirect hot-water cylinder (about 200 l): the mesh dumps (jacket quarters, vessel courses, cover, coil) replaced by hand-authored revolves and a lofted helix; the charging coil moved to the lower two thirds where it is built (return 0.26 m, flow 0.76 m), nine turns of 28 mm tube at 340 mm; magnesium anode, inlet baffle, long sensor pocket and the hand-hole neck added; a hidden thermal region for the coil and the internals. Bones are stacked along the height: zone-4 from 0.06 m, zone-3 0.39, zone-2 0.72, zone-1 1.05, cap 1.38; the connections bone is in world metres."""
import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rework-17-kit.py')).read())

SLUG = 'hot-water-cylinder'
V = load(SLUG)
UP = [0, 1, 0]
O = [0.0, 0.0, 0.0]
FOAM, STEEL, CAP = 'foam-jacket', 'enamelled-steel', 'cap-plastic'
RETURN_Y, FLOW_Y = 0.26, 0.76
SHIFT = {'3': FLOW_Y - 0.9227, '4': RETURN_Y - 0.5102}
COIL_R, TUBE_R, TURNS = 0.17, 0.014, 9

HEAD = {
    50: [[0.047, 0.0], [0.0508, 0.06], [0.0623, 0.12], [0.0781, 0.17], [0.0907, 0.2], [0.105, 0.222], [0.122, 0.235], [0.14, 0.24]],
    100: [[0.047, 0.0], [0.0553, 0.09], [0.0781, 0.17], [0.098, 0.213], [0.118, 0.234], [0.14, 0.24]],
    200: [[0.047, 0.0], [0.0623, 0.12], [0.0907, 0.2], [0.115, 0.232], [0.14, 0.24]],
    500: [[0.047, 0.0], [0.0781, 0.17], [0.11, 0.228], [0.14, 0.24]],
}
DOME_STEP = {50: 10, 100: 15, 200: 22.5, 500: 30}
PER_TURN = {50: 12, 100: 8, 200: 6}
WALL = 0.003


def inner(points):
    """🪞 The same head 3 mm inside (shifted along the height at the apex, along the radius at the wall)."""
    n = len(points) - 1
    return [[a + WALL * (1 - k / n), max(0.0, r - WALL * k / n)] for k, (a, r) in enumerate(points)]


def tank_bottom(scale):
    out = HEAD[scale]
    return rev('tank-4', 'zone-4', STEEL, UP, O, out + [[0.33, 0.24], [0.33, 0.237]] + inner(out)[::-1] + [out[0]], tone=0.5)


def tank_top(scale):
    out = [[0.29 - a, r] for a, r in HEAD[scale]]
    ins = [[0.29 - a, r] for a, r in inner(HEAD[scale])]
    return rev('tank-1', 'zone-1', STEEL, UP, O, [[0.0, 0.237], [0.0, 0.24]] + out[::-1] + ins + [[0.0, 0.237]], tone=0.5)


def jacket_bottom(scale):
    cavity = [[a, r + 0.0005 if r else 0.0] for a, r in HEAD[scale]]
    return rev('zone-4', 'zone-4', FOAM, UP, O, [[0.0, 0.0], [0.0, 0.3], [0.33, 0.3], [0.33, 0.2405]] + cavity[::-1] + [[0.0, 0.0]], tone=0.7)


def jacket_top(scale):
    cavity = [[0.29 - a, r + 0.0005 if r else 0.0] for a, r in HEAD[scale]]
    return rev('zone-1', 'zone-1', FOAM, UP, O, [[0.0, 0.2405], [0.0, 0.3], [0.33, 0.3], [0.33, 0.0]] + cavity + [[0.0, 0.2405]], tone=0.7)


def sleeve(k):
    return rev('zone-%d' % k, 'zone-%d' % k, FOAM, UP, O, [[0.0, 0.2405], [0.0, 0.3], [0.33, 0.3], [0.33, 0.2405], [0.0, 0.2405]], tone=0.7)


def course(k):
    return rev('tank-%d' % k, 'zone-%d' % k, STEEL, UP, O, [[0.0, 0.237], [0.0, 0.24], [0.33, 0.24], [0.33, 0.237], [0.0, 0.237]], tone=0.5)


def dome(scale):
    step = DOME_STEP[scale]
    n = int(round(90 / step))
    outer = [[0.1 * math.sin(math.radians(step * k)), 0.3 * math.cos(math.radians(step * k))] for k in range(n + 1)]
    ins = [[0.097 * math.sin(math.radians(step * k)), 0.297 * math.cos(math.radians(step * k))] for k in range(n + 1)]
    outer[-1][1] = 0.0
    ins[-1][1] = 0.0
    return rev('cap', 'cap', CAP, UP, O, outer + ins[::-1] + [outer[0]], tone=0.78)


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def helix(per_turn):
    """🌀 The coil as one loft: a circle of 28 mm swept along a helix of 340 mm about the vessel axis from the return tapping up to the flow tapping; the sections carry no rotation, so the kernel sets each normal to the path."""
    pitch = (FLOW_Y - RETURN_Y) / TURNS
    sections = []
    for k in range(TURNS * per_turn + 1):
        phi = 2 * math.pi * k / per_turn
        sections.append({'at': [r5(COIL_R * math.cos(phi)), r5(RETURN_Y + pitch * k / per_turn), r5(COIL_R * math.sin(phi))], 'rx': TUBE_R, 'ry': TUBE_R, 'exponent': 2})
    s = base('charging-coil', 'connections', STEEL, role='detail', tone=0.37)
    s.update(kind='loft', sections=sections, capStart=True, capEnd=True)
    return s


def nut(k, y, side):
    lo, hi = (-0.33, -0.318) if side < 0 else (0.318, 0.33)
    return ext('nut-%d' % k, 'connections', 'brass', 'yz', hexagon(y, 0.0, 0.0329, math.pi / 2), lo, hi, holes=[circle(y, 0.0, 0.012, 16)], tone=0.45)


def shift_solids(vol):
    for lst in ('body', 'thermal', 'structure'):
        for s in vol[lst]:
            sid = s['id']
            if sid[-1] in SHIFT and sid[:-1] in ('stub-', 'th-stub-', 'weld-pipe-') and 'origin' in s:
                s['origin'] = [s['origin'][0], r5(s['origin'][1] + SHIFT[sid[-1]]), s['origin'][2]]


STUB_Y = {0: 1.1977, 1: 0.7852, 2: 0.1893, 3: FLOW_Y, 4: RETURN_Y}
INT = dict(region='internals', layer='tube')

for scale in (50, 100, 200, 500):
    L = Lists(V, scale)
    shift_solids(L.v)
    L.put('body', jacket_bottom(scale), after='zone-4')
    L.put('body', tank_bottom(scale), after='tank-4')
    for k in (3, 2):
        L.put('body', sleeve(k), after='zone-%d' % k)
        L.put('body', course(k), after='tank-%d' % k)
    L.put('body', jacket_top(scale), after='zone-1')
    L.put('body', tank_top(scale), after='tank-1')
    L.put('body', dome(scale), after='cap')
    for k in range(5):
        L.put('body', nut(k, STUB_Y[k], -1 if k < 3 else 1), after='nut-%d' % k)
    if scale == 500:
        coil = rev('charging-coil', 'connections', STEEL, UP, O, [[RETURN_Y - TUBE_R, COIL_R - TUBE_R], [RETURN_Y - TUBE_R, COIL_R + TUBE_R], [FLOW_Y + TUBE_R, COIL_R + TUBE_R], [FLOW_Y + TUBE_R, COIL_R - TUBE_R], [RETURN_Y - TUBE_R, COIL_R - TUBE_R]], role='detail', tone=0.37)
    else:
        coil = helix(PER_TURN[scale])
    legs = cap('coil-legs', 'connections', STEEL, [0.237, RETURN_Y, 0.0], [COIL_R, RETURN_Y, 0.0], TUBE_R, role='detail', tone=0.37, repeat={'count': 2, 'translation': [0, FLOW_Y - RETURN_Y, 0]})
    anode = cyl('anode', 'connections', 'magnesium', UP, [-0.1, 0.0, 0.0], 0.83, 1.285, 0.013, role='detail', tone=0.6)
    pocket = rev('sensor-pocket', 'connections', STEEL, [1, 0, 0], [-0.235, 0.7852, 0], [[0, 0.0085], [0, 0.012], [0.15, 0.012], [0.15, 0], [0.147, 0], [0.147, 0.0085], [0, 0.0085]], role='detail', tone=0.52)
    L.put('body', coil, after='charging-coil')
    L.put('body', pocket, after='sensor-pocket')
    extra = [legs, anode]
    if scale <= 100:
        extra += [cyl('anode-plug', 'connections', 'brass', UP, [-0.1, 0.0, 0.0], 1.27, 1.3, 0.02, role='detail', tone=0.45),
                  cyl('inlet-baffle', 'connections', STEEL, [1, 0, 0], [0.0, 0.1893, 0.0], -0.195, -0.18, 0.045, role='detail', tone=0.5),
                  cap('inlet-nozzle', 'connections', STEEL, [-0.237, 0.1893, 0.0], [-0.195, 0.1893, 0.0], 0.012, role='detail', tone=0.52)]
    if scale == 50:
        extra += [tube('hand-hole-neck', 'zone-4', STEEL, [0, 0, -1], [0.0, 0.17, 0.0], 0.225, 0.27, 0.055, 0.06, role='detail', tone=0.5),
                  cyl('hand-hole-flange', 'zone-4', STEEL, [0, 0, -1], [0.0, 0.17, 0.0], 0.262, 0.272, 0.075, role='detail', tone=0.45)]
    L.put('body', *extra, after='charging-coil')
    L.v['thermal'] = [s for s in L.v['thermal'] if s.get('region') != 'internals']
    L.put('thermal', own(coil, **INT), own(legs, **INT), own(anode, **INT), own(pocket, **INT))
    if scale <= 100:
        L.put('thermal', own(L.get('body', 'inlet-baffle'), **INT), own(L.get('body', 'inlet-nozzle'), **INT))
    L.decide('The right tappings',
             'The right tappings feed the charging coil where it is built, in the lower two thirds: return at 0.26 m, flow at 0.76 m, ' + ('nine turns of 28 mm tube on a 340 mm helix, one lofted solid with a circular section normal to the helix at %d stations a turn' % PER_TURN[scale] if scale != 500 else 'the nine turns generalised to one 28 mm sleeve of 340 mm') + ', and two radial legs to the tappings. A magnesium anode of 26 mm hangs from the top head' + (', the cold inlet ends at a baffle disc' if scale <= 100 else '') + ', and the left middle tapping carries a closed immersion pocket 150 mm into the water. Jacket quarters, vessel courses with their dished heads and the cover are turned parts (revolves) with the 3 mm wall and the 60 mm foam; the pipes pass through the foam.',
             'Die rechten Stutzen speisen die Ladewendel dort, wo sie gebaut wird, in den unteren zwei Dritteln: Rücklauf auf 0,26 m, Vorlauf auf 0,76 m, ' + ('neun Windungen aus 28-mm-Rohr auf einer Wendel von 340 mm, ein gelofteter Körper mit Kreisquerschnitt, an %d Stationen je Windung senkrecht zur Wendel gedreht' % PER_TURN[scale] if scale != 500 else 'die neun Windungen zu einer 28 mm starken Hülse von 340 mm verallgemeinert') + ', und zwei radiale Schenkel zu den Stutzen. Eine Magnesiumanode von 26 mm hängt am oberen Boden' + (', der Kaltwasserzulauf endet an einer Prallscheibe' if scale <= 100 else '') + ', und der linke mittlere Stutzen trägt eine geschlossene Tauchhülse, die 150 mm ins Wasser reicht. Mantelviertel, Behälterschüsse mit ihren Klöpperböden und die Abdeckung sind Drehteile mit 3 mm Wand und 60 mm Schaum; die Rohre durchstoßen den Schaum.')

save(SLUG, V)
print('saved', {s: tuple(len(V[s][l]) for l in ('body', 'thermal', 'structure')) for s in V}, os.path.getsize('%s/volumes/appliances/%s.json' % (T, SLUG)))


def root(asset):
    if not any(m['id'] == 'magnesium' for m in asset['materials']):
        asset['materials'].append({'id': 'magnesium', 'title': {'en': 'Magnesium sacrificial anode', 'de': 'Magnesium-Opferanode'}, 'emissivity': 0.6, 'solarAbsorptivity': 0.5, 'densityKgM3': 1740, 'conductivityWmK': 156, 'specificHeatJKgK': 1020, 'source': 'literature', 'reference': 'incropera-2018'})
    regions = asset['thermal']['regions']
    if not any(r['id'] == 'internals' for r in regions):
        regions.append({
            'id': 'internals', 'title': {'en': 'Charging coil and internals', 'de': 'Ladewendel und Einbauten'}, 'kind': 'surface', 'model': 'passive', 'material': 'enamelled-steel',
            'areaM2': 0.9, 'characteristicLengthM': 0.028, 'geometry': 'cylinder', 'skyViewFactor': 0,
            'layers': [{'id': 'tube', 'title': {'en': 'Enamelled steel tube wall 2.6 mm', 'de': 'Emaillierte Stahlrohrwand 2,6 mm'}, 'kind': 'metal', 'material': 'enamelled-steel', 'thicknessM': 0.0026, 'resistanceM2KW': 5.2e-05, 'heatCapacityJM2K': 9390, 'source': 'authored-estimate', 'reference': 'incropera-2018'}],
            'heatCapacityJM2K': 9390, 'source': 'authored-estimate', 'reference': 'viessmann-vitocell-100-v',
            'hidden': {'en': 'The charging coil of nine turns of 28 mm tube (0.85 m2) between the return tapping at 0.26 m and the flow tapping at 0.76 m, the magnesium anode, the inlet baffle and the sensor pocket stand in the water inside the vessel; no drawing shows them. The heat the coil gives to the water is carried by the four quarters, the region itself is passive.', 'de': 'Die Ladewendel aus neun Windungen 28-mm-Rohr (0,85 m2) zwischen Rücklaufstutzen auf 0,26 m und Vorlaufstutzen auf 0,76 m, die Magnesiumanode, die Prallscheibe am Zulauf und die Tauchhülse stehen im Wasser im Behälter; keine Zeichnung zeigt sie. Die Wärme, die die Wendel ans Wasser abgibt, tragen die vier Viertel, die Region selbst ist passiv.'}})
    for pose in asset['poses']:
        pose['thermal']['regions'].setdefault('internals', {'exposedFraction': 1, 'sunlitFraction': 0})
    done = abs(shape(asset, 50, 'elevation', 'stub-3')['y'] + 0.012 - FLOW_Y) < 1e-3
    if not done:
        for rep in asset['representations'].values():
            if rep['status'] != 'authored':
                continue
            for view in ('elevation', 'front'):
                for s in rep['geometry'].get(view, []):
                    sid = s['id']
                    if sid[-1] in SHIFT and sid[:-1] in ('stub-', 'nut-', 'bore-stub-'):
                        dy = SHIFT[sid[-1]]
                        if 'points' in s:
                            s['points'] = [[p[0], r5(p[1] + dy)] for p in s['points']]
                        elif 'cy' in s:
                            s['cy'] = r5(s['cy'] + dy)
                        elif s['kind'] == 'rect':
                            s['y'] = r5(s['y'] + dy)
                        else:
                            raise SystemExit('shape %s not shifted' % sid)


edit_catalog(SLUG, ['materials', 'representations', 'poses', 'thermal'], root)
