"""Author 32: city-car thermal, structure and decisions for all four scales (body lists are the repaired bodies in the source)."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *

A = 'vehicles/city-car'
CH = 'chassis'
EXH = 'exhaust-steel'
RQ = [0, -0.70711, 0, 0.70711]

def wheels(sc, parts):
    out = []
    for sgn, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            out.append((sgn, n, bone, p))
    return out

def brakes(sc, thick, rin, rout, caliper):
    out = []
    for sgn, n, bone, p in wheels(sc, None):
        z = sgn * 0.62
        out.append(th(rev(f'th-brake-{p}{n}', bone, 'cast-iron', 'detail', 0.35, [(-thick / 2, rin), (-thick / 2, rout), (thick / 2, rout), (thick / 2, rin)], (0, 0, z)), 'brakes'))
        if caliper:
            out.append(th(box(f'th-caliper-{p}{n}', bone, 'cast-iron', 'detail', 0.3, (0.07, 0.1, 0.05), (0.0, 0.13, z - sgn * 0.0), corner=0.01), 'brakes'))
    return out

def thermal50(v):
    T = []
    B = lambda i: body_of(v, i)
    for i, sf in (('body-lower', ['-v']), ('bumper-front', None), ('bumper-rear', None)):
        n = {'body-lower': 'lower', 'bumper-front': 'bumper-f', 'bumper-rear': 'bumper-r'}[i]
        T.append(ref(f'th-bp-{n}-paint', i, 'body-panels', 'paint', sf))
        T.append(shell(B(i), 0.00008, f'th-bp-{n}-sheet', 'body-panels', 'sheet'))
    for i, rg in (('roof-skin', 'roof'), ('bonnet-slab', 'bonnet'), ('hatch-slab', 'tailgate')):
        T.append(ref(f'th-{rg}-paint', i, rg, 'paint'))
        T.append(shell(B(i), 0.00008, f'th-{rg}-sheet', rg, 'sheet'))
    for i in ('windscreen', 'side-glass-front-l', 'side-glass-front-r', 'side-glass-rear-l', 'side-glass-rear-r', 'hatch-glass'):
        T.append(ref('th-' + i, i, 'glazing'))
    for i in ('exhaust-pipe', 'silencer', 'tail-pipe'):
        T.append(ref('th-' + i, i, 'exhaust-system', 'pipe'))
    T.append(th(cap('th-tailpipe-nozzle', CH, EXH, 'detail', 0.5, (-1.9, 0.231, -0.3), (-1.925, 0.245, -0.3), 0.02, 0.02), 'exhaust-system', 'pipe', ['tailpipe']))
    for i in ('fl', 'fr', 'rl', 'rr'):
        T.append(ref(f'th-tread-{i}', f'{i}-tyre', 'tyres', 'tread'))
    T += brakes(50, 0.02, 0.06, 0.1745, True)
    # hidden interior: seat foam, dash and floor liner as real solids inside the cabin
    for s in v['body']:
        if s['id'].startswith('seat-'):
            c = copy.deepcopy(s); c['id'] = 'th-foam-' + s['id'][5:]; c['material'] = 'interior-trim'
            T.append(th(c, 'interior'))
    T.append(th(box('th-dash-liner', CH, 'interior-trim', 'detail', 0.3, (0.24, 0.12, 0.8), (0.55, 1.0, 0), corner=0.02), 'interior'))
    T.append(th(box('th-floor-liner', CH, 'interior-trim', 'detail', 0.3, (1.0, 0.04, 0.8), (-0.4, 0.46, 0), corner=0.01), 'interior'))
    T.append(th(rev('th-steering-rim', CH, 'interior-trim', 'detail', 0.2, [(-0.012, 0.075), (-0.012, 0.09), (0.012, 0.09), (0.012, 0.075)], (0.56, 1.09, 0.29)), 'interior'))
    # hidden engine bay: block, gearbox and manifold inside the lower body below the bonnet
    T.append(th(box('th-engine-block', CH, 'engine-metal', 'detail', 0.3, (0.36, 0.4, 0.46), (1.3, 0.55, 0), corner=0.03), 'engine-bay'))
    T.append(th(cap('th-gearbox', CH, 'engine-metal', 'detail', 0.25, (1.02, 0.5, 0), (1.12, 0.5, 0), 0.1), 'engine-bay'))
    T.append(th(cap('th-exhaust-manifold', CH, EXH, 'detail', 0.35, (1.45, 0.4, -0.2), (1.45, 0.4, 0.2), 0.03), 'engine-bay'))
    return T

def thermal_coarse(v, sc):
    T = []
    ids = {s['id'] for s in v['body']}
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    T.append(ref('th-bp-lower', 'body-lower', 'body-panels', None, ['-v']))
    if 'roof-skin' in ids:
        T.append(ref('th-roof', 'roof-skin', 'roof'))
    else:
        cab = body_of(v, 'cabin')
        top = max(q['at'][1] + q['ry'] if abs(q['rotation'][0]) < 0.1 else q['at'][1] for q in cab['sections'])
        t = f
        T.append(th(loft('th-roof-panel', CH, 'paint', 'body', 0.5, [sec((0.18, 1.455, 0), 0.5, t / 2 + 0.002, 8, RQ), sec((-0.5, 1.487 - t / 2 + 0.004, 0), 0.52, t / 2 + 0.002, 8, RQ), sec((-1.0, 1.485 - t / 2 + 0.004, 0), 0.52, t / 2 + 0.002, 8, RQ), sec((-1.38, 1.44 - t / 2 + 0.004, 0), 0.46, t / 2 + 0.002, 8, RQ)]), 'roof'))
    T.append(ref('th-bonnet', 'bonnet-panel', 'bonnet'))
    T.append(ref('th-tailgate', 'hatch-panel', 'tailgate', None, ['+v']))
    for i in ('windscreen', 'side-glass-front-l', 'side-glass-front-r', 'side-glass-rear-l', 'side-glass-rear-r', 'side-glass-l', 'side-glass-r', 'hatch-glass'):
        if i in ids: T.append(ref('th-' + i, i, 'glazing'))
    for i in ('exhaust-pipe', 'silencer', 'tail-pipe'):
        if i in ids: T.append(ref('th-' + i, i, 'exhaust-system'))
    if 'exhaust-pipe' not in ids:
        T.append(th(cap('th-exhaust-line', CH, EXH, 'detail', 0.35, (-1.0, 0.14, -0.35), (0.35, 0.14, -0.35), 0.0375), 'exhaust-system'))
        T.append(th(cap('th-silencer-can', CH, EXH, 'detail', 0.4, (-1.75, 0.15, -0.35), (-1.0, 0.15, -0.35), 0.0525, lateral=1.4), 'exhaust-system'))
    if 'windscreen' not in ids:
        t = f / 2
        T.append(th(loft('th-windscreen', CH, 'glass', 'glazing', 0.85, [sec((0.78, 1.1, 0), 0.69, t, 8, [-0.20245, -0.6775, -0.20245, 0.6775]), sec((0.38, 1.35, 0), 0.62, t, 8, [-0.20245, -0.6775, -0.20245, 0.6775])]), 'glazing'))
        for sg, n in ((1, 'l'), (-1, 'r')):
            T.append(th(loft(f'th-side-glass-{n}', CH, 'glass', 'glazing', 0.85, [sec((0.3, 1.24, sg * 0.69), 0.15, t, 8, [0.42995, 0.56137, 0.42995, 0.56137] if sg > 0 else [0.56137, 0.42995, 0.56137, 0.42995]), sec((-1.0, 1.24, sg * 0.685), 0.15, t, 8, [0.42995, 0.56137, 0.42995, 0.56137] if sg > 0 else [0.56137, 0.42995, 0.56137, 0.42995])]), 'glazing'))
    T.append(th(cap('th-tailpipe-nozzle', CH, EXH, 'detail', 0.5, (-1.86, 0.2, -0.3), (-1.9, 0.222, -0.3), f * 0.6), 'exhaust-system', None, ['tailpipe']))
    for i in ('fl-tyre', 'fr-tyre', 'rl-tyre', 'rr-tyre', 'fl-wheel', 'fr-wheel', 'rl-wheel', 'rr-wheel'):
        if i in ids: T.append(ref('th-tread-' + i[:2], i, 'tyres'))
    return T

def interior_hidden(sc):
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    T = []
    if sc == 100:
        for sg, n in ((1, 'l'), (-1, 'r')):
            T.append(th(box(f'th-foam-front-{n}', CH, 'interior-trim', 'detail', 0.3, (0.5, 0.4, 0.4), (0.2, 1.1, sg * 0.25), corner=0.05), 'interior'))
        T.append(th(box('th-foam-rear', CH, 'interior-trim', 'detail', 0.3, (0.3, 0.4, 1.1), (-0.62, 1.1, 0), corner=0.05), 'interior'))
        T.append(th(box('th-dash-liner', CH, 'interior-trim', 'detail', 0.3, (0.24, 0.12, 0.8), (0.55, 1.0, 0), corner=0.02), 'interior'))
    elif sc == 200:
        T.append(th(box('th-foam-seats', CH, 'interior-trim', 'detail', 0.3, (1.0, 0.4, 0.8), (-0.2, 1.1, 0), corner=0.05), 'interior'))
    else:
        T.append(th(box('th-cabin-trim', CH, 'interior-trim', 'detail', 0.3, (0.9, 0.4, 0.8), (-0.3, 1.1, 0), corner=0.075), 'interior'))
    return T

def engine_hidden(sc):
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    T = []
    if sc == 100:
        T.append(th(box('th-engine-block', CH, 'engine-metal', 'detail', 0.3, (0.36, 0.4, 0.46), (1.3, 0.55, 0), corner=0.03), 'engine-bay'))
        T.append(th(cap('th-gearbox', CH, 'engine-metal', 'detail', 0.25, (1.02, 0.5, 0), (1.12, 0.5, 0), 0.1), 'engine-bay'))
    elif sc == 200:
        T.append(th(box('th-engine-unit', CH, 'engine-metal', 'detail', 0.3, (0.5, 0.4, 0.46), (1.25, 0.55, 0), corner=0.05), 'engine-bay'))
    else:
        T.append(th(box('th-engine-unit', CH, 'engine-metal', 'detail', 0.3, (0.5, 0.4, 0.46), (1.25, 0.55, 0), corner=0.075), 'engine-bay'))
    return T

def brake_coarse(sc):
    f = {100: 0.02, 200: 0.03, 500: 0.075}[sc]
    return brakes(sc, f, 0.06 if sc < 500 else 0.0, 0.1745, sc == 100)

def thermal100(v): return thermal_coarse(v, 100) + interior_hidden(100) + engine_hidden(100) + brake_coarse(100)
def thermal200(v): return thermal_coarse(v, 200) + interior_hidden(200) + engine_hidden(200) + brake_coarse(200)
def thermal500(v): return thermal_coarse(v, 500) + interior_hidden(500) + engine_hidden(500) + brake_coarse(500)

def structure50(v):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    FE = 'cast-iron'
    for sg, n in ((1, 'l'), (-1, 'r')):
        M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (3.0, 0.12, 0.1), (0.0, 0.34, sg * 0.6)), CH)
        M(box(f'front-rail-{n}', CH, FE, 'detail', 0.5, (0.9, 0.1, 0.08), (1.4, 0.5, sg * 0.3)), CH)
        M(box(f'rear-rail-{n}', CH, FE, 'detail', 0.5, (0.7, 0.1, 0.08), (-1.5, 0.5, sg * 0.42)), CH)
        M(cap(f'a-pillar-{n}', CH, FE, 'detail', 0.5, (0.8, 1.04, sg * 0.64), (0.26, 1.42, sg * 0.52), 0.03), CH)
        M(cap(f'b-pillar-{n}', CH, FE, 'detail', 0.5, (-0.35, 1.02, sg * 0.64), (-0.35, 1.43, sg * 0.54), 0.03), CH)
        M(cap(f'c-pillar-{n}', CH, FE, 'detail', 0.5, (-1.3, 1.02, sg * 0.64), (-1.36, 1.4, sg * 0.5), 0.03), CH)
        M(cap(f'roof-rail-{n}', CH, FE, 'detail', 0.5, (-1.28, 1.43, sg * 0.5), (0.18, 1.43, sg * 0.5), 0.028), CH)
    for x, nm in ((1.25, 'front-crossmember'), (0.1, 'seat-crossmember-front'), (-0.55, 'seat-crossmember-rear'), (-1.2, 'rear-crossmember')):
        M(box(nm, CH, FE, 'detail', 0.5, (0.1, 0.1, 1.2), (x, 0.34, 0)), CH)
    for x, nm in ((0.2, 'roof-bow-front'), (-0.6, 'roof-bow-mid'), (-1.3, 'roof-bow-rear')):
        M(cap(nm, CH, FE, 'detail', 0.5, (x, 1.44, -0.5), (x, 1.44, 0.5), 0.025), CH)
    M(box('bumper-beam-front', CH, FE, 'detail', 0.5, (0.08, 0.1, 1.1), (1.86, 0.45, 0)), CH)
    M(box('bumper-beam-rear', CH, FE, 'detail', 0.5, (0.08, 0.1, 1.1), (-1.86, 0.45, 0)), CH)
    S.append(st(ell('com-chassis', CH, FE, 'detail', 0.5, (0.3, 0.12, 0.3), (0.38, 0.55, 0)), CH, 'mass'))
    M(box('bonnet-rib', 'bonnet', FE, 'detail', 0.5, (0.85, 0.03, 0.6), (0.5, -0.02, 0)), 'bonnet')
    M(box('hatch-rib', 'hatch', FE, 'detail', 0.5, (0.7, 0.03, 0.8), (0.4, 0.02, 0)), 'hatch')
    for z, n in ((0.5, 'l'), (-0.5, 'r')):
        M(ell(f'bonnet-hinge-{n}', 'bonnet', FE, 'detail', 0.5, (0.03, 0.02, 0.03), (0.02, -0.01, z)), 'bonnet', 'joint')
        M(ell(f'hatch-hinge-{n}', 'hatch', FE, 'detail', 0.5, (0.03, 0.02, 0.03), (0.05, 0.012, z)), 'hatch', 'joint')
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -0.64), (0, 0, 0.64), 0.04), ax)
        S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.09, 0.09, 0.09), (0, 0, 0)), ax, 'mass'))
        for z, n in ((0.5, 'l'), (-0.5, 'r')):
            M(ell(f'{ax}-spring-{n}', ax, FE, 'detail', 0.5, (0.06, 0.1, 0.06), (0, 0.1, z)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            M(rev(f'{bone}-rim-{n}', bone, 'alloy', 'detail', 0.55, [(-0.075, 0.19), (0.075, 0.19)], (0, 0, sg * 0.72)), bone)
            M(rev(f'{bone}-hub-{n}', bone, 'alloy', 'detail', 0.7, [(-0.09, 0.045), (0.09, 0.045)], (0, 0, sg * 0.72)), bone, 'joint')
        S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.1, 0.1, 0.1), (0, 0, 0.72)), bone, 'mass'))
    return S

def structure_coarse(v, sc):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    FE = 'cast-iron'
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    for sg, n in ((1, 'l'), (-1, 'r')):
        if sc == 100:
            M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (3.0, 0.13, 0.12), (0.0, 0.34, sg * 0.6)), CH)
            M(box(f'front-rail-{n}', CH, FE, 'detail', 0.5, (0.95, 0.12, 0.09), (1.4, 0.5, sg * 0.3)), CH)
            M(box(f'rear-rail-{n}', CH, FE, 'detail', 0.5, (0.75, 0.12, 0.09), (-1.5, 0.5, sg * 0.42)), CH)
            M(cap(f'a-pillar-{n}', CH, FE, 'detail', 0.5, (0.8, 1.04, sg * 0.64), (0.26, 1.42, sg * 0.52), 0.035), CH)
            M(cap(f'b-pillar-{n}', CH, FE, 'detail', 0.5, (-0.35, 1.02, sg * 0.64), (-0.35, 1.43, sg * 0.54), 0.035), CH)
            M(cap(f'c-pillar-{n}', CH, FE, 'detail', 0.5, (-1.3, 1.02, sg * 0.64), (-1.36, 1.4, sg * 0.5), 0.035), CH)
            M(cap(f'roof-rail-{n}', CH, FE, 'detail', 0.5, (-1.28, 1.43, sg * 0.5), (0.18, 1.43, sg * 0.5), 0.035), CH)
        elif sc == 200:
            M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (3.0, 0.15, 0.14), (0.0, 0.34, sg * 0.6)), CH)
            M(box(f'front-rail-{n}', CH, FE, 'detail', 0.5, (1.0, 0.14, 0.1), (1.4, 0.5, sg * 0.3)), CH)
            M(cap(f'a-pillar-{n}', CH, FE, 'detail', 0.5, (0.8, 1.04, sg * 0.62), (0.26, 1.4, sg * 0.5), 0.04), CH)
            M(cap(f'c-pillar-{n}', CH, FE, 'detail', 0.5, (-0.6, 1.04, sg * 0.62), (-1.3, 1.4, sg * 0.5), 0.04), CH)
            M(cap(f'roof-rail-{n}', CH, FE, 'detail', 0.5, (-1.28, 1.42, sg * 0.5), (0.18, 1.42, sg * 0.5), 0.04), CH)
        else:
            M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (3.0, 0.18, 0.16), (0.0, 0.36, sg * 0.58)), CH)
            M(cap(f'pillar-{n}', CH, FE, 'detail', 0.5, (0.7, 1.04, sg * 0.58), (-1.2, 1.38, sg * 0.46), 0.045), CH)
    cross = {100: ((1.25, 'front-crossmember'), (-0.25, 'floor-crossmember'), (-1.2, 'rear-crossmember')), 200: ((1.25, 'front-crossmember'), (-1.2, 'rear-crossmember')), 500: ((1.25, 'front-crossmember'), (-1.2, 'rear-crossmember'))}[sc]
    ft = max(0.1, f * 1.2) if sc < 500 else 0.12
    for x, nm in cross:
        M(box(nm, CH, FE, 'detail', 0.5, (ft, ft, 1.2), (x, 0.34, 0)), CH)
    if sc == 100:
        M(cap('roof-bow', CH, FE, 'detail', 0.5, (-0.6, 1.43, -0.5), (-0.6, 1.43, 0.5), 0.03), CH)
    if sc <= 200:
        M(box('bumper-beam-front', CH, FE, 'detail', 0.5, (0.1, 0.12, 1.1), (1.85, 0.45, 0)), CH)
        M(box('bumper-beam-rear', CH, FE, 'detail', 0.5, (0.1, 0.12, 1.1), (-1.85, 0.45, 0)), CH)
    S.append(st(ell('com-chassis', CH, FE, 'detail', 0.5, (0.3, 0.12, 0.3) if sc < 500 else (0.3, 0.1, 0.3), (0.38, 0.55, 0)), CH, 'mass'))
    bt = {100: 0.045, 200: 0.06, 500: 0.1}[sc]
    M(box('bonnet-rib', 'bonnet', FE, 'detail', 0.5, (0.85, bt, 0.6), (0.5, -0.02, 0)), 'bonnet')
    M(box('hatch-rib', 'hatch', FE, 'detail', 0.5, (0.7, bt, 0.8), (0.4, 0.03, 0)), 'hatch')
    if sc == 100:
        M(ell('bonnet-hinge', 'bonnet', FE, 'detail', 0.5, (0.03, 0.025, 0.5), (0.02, -0.01, 0)), 'bonnet', 'joint')
        M(ell('hatch-hinge', 'hatch', FE, 'detail', 0.5, (0.03, 0.025, 0.5), (0.05, 0.012, 0)), 'hatch', 'joint')
    ab = {100: 0.045, 200: 0.055, 500: 0.0425}[sc]
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -0.64), (0, 0, 0.64), ab), ax)
        if sc <= 200:
            S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.1, 0.1, 0.1), (0, 0, 0)), ax, 'mass'))
        if sc == 100:
            M(ell(ax + '-spring', ax, FE, 'detail', 0.5, (0.07, 0.1, 0.5), (0, 0.1, 0)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            if sc == 100:
                M(rev(f'{bone}-rim-{n}', bone, 'alloy', 'detail', 0.55, [(-0.09, 0.045), (-0.075, 0.045), (-0.075, 0.19), (0.075, 0.19), (0.075, 0.045), (0.09, 0.045)], (0, 0, sg * 0.72)), bone)
            else:
                M(rev(f'{bone}-rim-{n}', bone, 'alloy', 'detail', 0.55, [(-0.0825 if sc == 500 else -0.075, 0.19), (0.0825 if sc == 500 else 0.075, 0.19)], (0, 0, sg * 0.72)), bone)
        if sc <= 200:
            S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.1, 0.1, 0.1), (0, 0, 0.72)), bone, 'mass'))
    return S

DEC = {
    50: [("Body: a real car body. The lower body is a loft along the car with superelliptic sections read from the 1:50 elevation and plan (wheel arches, sill step, nose), the cabin a loft of horizontal slices so that the tumblehome of the front view is exact; the windscreen and the four side panes are 9 mm glass shells that stand 4 mm proud of the cabin surface, the tailgate glass sits on the tailgate slab, the engine fill ends in the bonnet slab, wheels are revolved tyres, rims, hubs and five spokes, the exhaust is pipe, silencer and tail pipe.",
          "Körper: ein echter Fahrzeugkörper. Der Unterbau ist ein Loft entlang des Fahrzeugs mit superelliptischen Querschnitten aus Seitenriss und Grundriss 1:50 (Radläufe, Schwellerstufe, Front), die Kabine ein Loft aus horizontalen Schnitten, damit der Einzug der Vorderansicht stimmt; Windschutz- und vier Seitenscheiben sind 9 mm dicke Glasschalen, die 4 mm über der Kabinenfläche stehen, das Heckglas liegt auf der Heckklappe, die Motorfüllung endet in der Haubenplatte, die Räder sind Rotationsreifen, Felgen, Naben und fünf Speichen, der Abgasstrang besteht aus Rohr, Schalldämpfer und Endrohr."),
         ("Thermal: body-panels, roof, bonnet and tailgate are nested shells (paint film as the outer reference to the body solid, steel sheet as the inset shell below it); the underside of the lower body is not a painted panel and is excluded with sectionFaces; glazing, exhaust and tyres reference the body solids; the hidden interior (seat foam, dash, floor liner, steering rim) and the hidden engine bay (block, gearbox, manifold) are real solids inside the body; brake discs with calipers sit behind the rims; the tail pipe outlet carries the exhaust emitter.",
          "Thermik: Karosserieblech, Dach, Haube und Heckklappe sind verschachtelte Schalen (der Lackfilm als äussere Referenz auf den Körper, das Stahlblech als eingerückte Schale darunter); die Unterseite des Unterbaus ist kein lackiertes Blech und wird mit sectionFaces ausgenommen; Verglasung, Abgas und Reifen verweisen auf Körper, der verdeckte Innenraum (Sitzschaum, Armaturenbrett, Bodenbelag, Lenkradkranz) und der verdeckte Motorraum (Block, Getriebe, Krümmer) sind eigene Körper im Inneren; Bremsscheiben mit Sätteln sitzen hinter den Felgen; der Endrohrauslass trägt den Abgasemitter."),
         ("Structure: sills, front and rear rails, four cross members, bumper beams, A, B and C pillars, roof rails and three roof bows form the unibody load path inside the shell; rib frames carry bonnet and tailgate on hinge joints, each axle is a beam with spring joints and a mass, each wheel a rim, hub joint and mass; the centre of mass sits at the body mass ellipsoid. Everything lies inside the body silhouette.",
          "Tragwerk: Schweller, vordere und hintere Längsträger, vier Querträger, Stossfängerträger, A-, B- und C-Säulen, Dachholme und drei Dachspriegel bilden den Lastpfad der selbsttragenden Karosserie innerhalb der Hülle; Rippenrahmen tragen Haube und Heckklappe an Scharniergelenken, jede Achse ist ein Balken mit Federgelenken und einer Masse, jedes Rad aus Felge, Nabengelenk und Masse; der Schwerpunkt liegt im Massenellipsoid des Aufbaus. Alles liegt innerhalb der Körpersilhouette.")],
    100: [("Remodelled against the 1:100 drawing: hull and cabin lofts use fewer stations, side glass is one pane per door window and again 8 mm shells proud of the cabin, the roof skin is a four-section loft, seats are two blocks, wheels are tyre, rim disc and hub without spokes, the bonnet and tailgate panels are the thicker outline the 1:100 drawing shows with the engine fill overlapping the panel from below.",
           "Neu modelliert gegen die 1:100-Zeichnung: Unterbau- und Kabinen-Loft mit weniger Stationen, eine Seitenscheibe je Türfenster, wieder als 8 mm dicke Schalen über der Kabine, die Dachhaut als Loft aus vier Schnitten, Sitze als zwei Blöcke, Räder aus Reifen, Felgenscheibe und Nabe ohne Speichen, Haube und Heckklappe als dickere Kontur, wie sie die 1:100-Zeichnung zeigt, mit der Motorfüllung von unten in der Platte."),
          ("Thermal: single surface solids per region instead of layer shells (sheet and paint merged); body-panels reference the lower body without its underside, bonnet, roof and glazing reference body solids, the tailgate references the outer side of the thick panel; hidden interior and engine bay are coarser real solids (three seat blocks and dash; block and gearbox); brake discs are 20 mm thick.",
           "Thermik: je Region ein einzelner Oberflächenkörper statt Schichtschalen (Blech und Lack zusammengefasst); das Karosserieblech verweist auf den Unterbau ohne Unterseite, Haube, Dach und Verglasung verweisen auf Körper, die Heckklappe auf die Aussenseite der dicken Platte; Innenraum und Motorraum sind gröbere eigene Körper (drei Sitzblöcke und Armaturenbrett; Block und Getriebe); Bremsscheiben sind 20 mm dick."),
          ("Structure: the unibody keeps sills, rails, three cross members, A, B and C pillars, roof rails, one roof bow and bumper beams at heavier sections; the hinge joints are merged to one per panel, each axle has one wide spring joint, the wheel rims are single revolves with hub boss, the hubs are merged into them.",
           "Tragwerk: die selbsttragende Karosserie behält Schweller, Längsträger, drei Querträger, A-, B- und C-Säulen, Dachholme, einen Dachspriegel und Stossfängerträger in kräftigeren Querschnitten; die Scharniergelenke sind zu einem je Platte zusammengefasst, jede Achse hat ein breites Federgelenk, die Felgen sind einzelne Rotationskörper mit Nabenkörper, die Naben sind darin aufgegangen.")],
    200: [("Remodelled against the 1:200 drawing: merged side glass per side, one seat block, exhaust as pipe and silencer, wheels as tyre and rim; mirrors kept because the 1:200 drawing shows them; engine fill re-fitted under the bonnet panel so that the silhouette has no slit.",
           "Neu modelliert gegen die 1:200-Zeichnung: eine Seitenscheibe je Seite, ein Sitzblock, Abgas als Rohr und Schalldämpfer, Räder aus Reifen und Felge; Spiegel bleiben, weil die 1:200-Zeichnung sie zeigt; die Motorfüllung ist unter der Haubenplatte neu angepasst, damit die Silhouette keinen Schlitz hat."),
          ("Thermal: one solid per region; the roof, which this scale does not draw separately, is a 30 mm roof panel on the cabin top; the interior is one foam block, the engine bay one engine unit (both hidden, inside the body), brake discs are 30 mm; only the total of the exposed areas is compared at this scale.",
           "Thermik: ein Körper je Region; das Dach, das dieser Massstab nicht gesondert zeichnet, ist eine 30 mm dicke Dachplatte auf der Kabine; der Innenraum ist ein Schaumblock, der Motorraum eine Motoreinheit (beide verdeckt, im Inneren), Bremsscheiben sind 30 mm; in diesem Massstab wird nur die Summe der freien Flächen verglichen."),
          ("Structure: sills, front rails, A pillars, one C-pillar strut per side, roof rails, two cross members and the bumper beams carry the shell; the hinge joints are absorbed into the rib frames, axles are beam plus mass, wheels are rim plus mass.",
           "Tragwerk: Schweller, vordere Längsträger, A-Säulen, eine C-Säulenstrebe je Seite, Dachholme, zwei Querträger und die Stossfängerträger tragen die Hülle; die Scharniergelenke sind in die Rippenrahmen aufgegangen, Achsen bestehen aus Balken und Masse, Räder aus Felge und Masse.")],
    500: [("Minimal model: hull, engine fill and greenhouse lofts, bonnet and tailgate panels and four wheel cylinders; glass is part of the greenhouse, no interior or exhaust bodies.",
           "Minimales Modell: Unterbau-, Motorfüllungs- und Glashaus-Loft, Haube und Heckklappe sowie vier Radzylinder; das Glas gehört zum Glashaus, keine Innenraum- und Abgaskörper."),
          ("Thermal: regions the body does not carry get their own compact solids at the smallest feature: 75 mm glass shells for windscreen and side glass, a 75 mm exhaust line and silencer under the floor, a roof panel, one hidden interior block and one hidden engine unit; brake discs are 75 mm thick pucks; only the total exposed area counts.",
           "Thermik: Regionen, die der Körper nicht trägt, bekommen eigene kompakte Körper in der kleinsten Detailgrösse: 75 mm dicke Glasschalen für Windschutz- und Seitenscheiben, Abgasstrang und Schalldämpfer von 75 mm unter dem Boden, eine Dachplatte, einen verdeckten Innenraumblock und eine verdeckte Motoreinheit; Bremsscheiben sind 75 mm dicke Scheiben; nur die Summe der freien Fläche zählt."),
          ("Structure: sills, two cross members, one pillar strut per side with the roof line, bonnet and tailgate ribs, axle beams and wheel rims at 75 mm sections; masses merged into the body mass.",
           "Tragwerk: Schweller, zwei Querträger, eine Säulenstrebe je Seite entlang der Dachlinie, Rippen von Haube und Heckklappe, Achsbalken und Felgen in 75-mm-Querschnitten; die Massen sind in der Aufbaumasse zusammengefasst.")],
}

if __name__ == '__main__':
    scales = [int(x) for x in sys.argv[1:]] or [50, 100, 200, 500]
    d = load(A)
    for sc in scales:
        v = d['volumes'][str(sc)]
        v['thermal'] = {50: thermal50, 100: thermal100, 200: thermal200, 500: thermal500}[sc](v)
        v['structure'] = structure50(v) if sc == 50 else structure_coarse(v, sc)
        v['decisions'] = [{"en": a, "de": b} for a, b in DEC[sc]]
    save(A, d)
    for sc in scales:
        fails(A, sc)
