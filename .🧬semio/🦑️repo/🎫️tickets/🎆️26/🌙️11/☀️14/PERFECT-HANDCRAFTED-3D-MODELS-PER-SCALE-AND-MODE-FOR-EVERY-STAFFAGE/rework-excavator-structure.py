"""🦴 Closes the load path of the excavator structure at every scale by hand: main girders under the boom feet, counterweight carriers and engine bearers on the girders, cab platform under a closed ROPS frame, slew bearing between the rings, pistons and cap ends in the hollow cylinders, lugs under the cylinder pins, the whole bucket."""
import json, sys
P = 'volumes/machinery/excavator.json'
doc = json.load(open(P))
V = doc['volumes']
STEEL = {'material': 'paint-steel', 'role': 'body', 'tone': 0.45}

def box(i, bone, member, x, y, z, **kw):
    return {'id': i, 'kind': 'box', 'bone': bone, **STEEL, **kw, 'member': member, 'part': 'member', 'size': [round(x[1] - x[0], 4), round(y[1] - y[0], 4), round(z[1] - z[0], 4)], 'transform': {'translation': [round((x[0] + x[1]) / 2, 4), round((y[0] + y[1]) / 2, 4), round((z[0] + z[1]) / 2, 4)]}}

def span(b, k):
    return b['transform']['translation'][k] - b['size'][k] / 2, b['transform']['translation'][k] + b['size'][k] / 2

def put(lst, s):
    for n, x in enumerate(lst):
        if x['id'] == s['id']:
            lst[n] = s
            return
    lst.append(s)

for d in ('50', '100', '200', '500'):
    body = {x['id']: x for x in V[d]['body']}
    st = V[d]['structure']
    S = {x['id']: x for x in st}
    fr, fl = body['boom-foot-right'], body['boom-foot-left']
    zr, zl = (fr['from'] + fr['to']) / 2, (fl['from'] + fl['to']) / 2
    half = max(0.04, (fr['to'] - fr['from']) / 2)
    for rid, z in (('frame-rail-1', zr), ('frame-rail-2', zl)):
        r = S[rid]
        r['transform']['translation'][2] = round(z, 4)
        r['size'][2] = round(2 * half, 4)
    for cid, z in (('counterweight-carrier-1', zr), ('counterweight-carrier-2', zl)):
        S[cid]['transform']['translation'][2] = round(z, 4)
    beds = [x for x in st if x['id'].startswith('engine-bed')]
    if len(beds) == 2:
        for b, x in zip(beds, (0.08, 0.64)):
            b['size'] = [0.08, 0.1, 1.0]
            b['transform'] = {'translation': [x, -0.075, round((zr + zl) / 2, 4)]}
    else:
        beds[0]['transform']['translation'][1] = -0.075
    if d == '50':
        S['joint-slew']['profile'] = [[0, 0.54], [0, 0.66], [0.18, 0.66], [0.18, 0.54], [0, 0.54]]
        S['joint-slew']['origin'] = [-0.03, 0, 0]
    y0, y1 = (0.02, 0.22) if d in ('50', '100') else (0.0, 0.2)
    rear, rail = S['rops-post-rear-right'], S['rops-rail-right']
    top = span(rear, 1)[1]
    xr = rear['transform']['translation'][0]
    w = rear['size'][2]
    for side, z in (('right', S['rops-post-rear-right']['transform']['translation'][2]), ('left', S['rops-post-rear-left']['transform']['translation'][2])):
        on_side = side == 'left' and 'frame-side-2' in S
        foot = (span(S['frame-side-2'], 1)[1] if on_side else y1) - 0.02
        p = S[f'rops-post-rear-{side}']
        p['size'][1] = round(top - foot, 4)
        p['transform']['translation'][1] = round((top + foot) / 2, 4)
        f = S[f'rops-post-front-{side}']
        if f['kind'] == 'extrude':
            low = min(q[1] for q in f['profile'])
            f['profile'] = [[q[0], round(foot, 4) if q[1] == low else q[1]] for q in f['profile']]
            xf = (min(q[0] for q in f['profile']), max(q[0] for q in f['profile'] if q[1] < 0.5))
        else:
            b, t = span(f, 1)
            f['size'][1] = round(t - foot, 4)
            f['transform']['translation'][1] = round((t + foot) / 2, 4)
            xf = span(f, 0)
        if not on_side:
            put(st, box(f'frame-cab-sill-{side}', 'upper', 'upper', (xr - 0.08, max(xf) - 0.02), (y0, y1), (z - 0.03, z + 0.05)))
    zR, zL = S['rops-post-rear-right']['transform']['translation'][2], S['rops-post-rear-left']['transform']['translation'][2]
    for name, x in (('rear', xr), ('mid', 0.66)):
        put(st, box(f'frame-cab-{name}', 'upper', 'upper', (x - 0.05, x + 0.05), (y0, y1), (zl - half, zL + w / 2)))
    xa, xb = span(rail, 0)
    yr = rail['transform']['translation'][1]
    h = rail['size'][1]
    for name, x in (('front', xb - w / 2), ('rear', xa + w / 2)):
        put(st, box(f'rops-bow-{name}', 'upper', 'upper', (x - w / 2, x + w / 2), (yr - h / 2, yr + h / 2), (zR, zL), tone=0.5))
    for c in ('boom-cyl-left', 'boom-cyl-right', 'stick-cyl', 'bucket-cyl'):
        tube, shaft = S['tube-' + c], S['shaft-' + c]
        if len(tube['profile']) != 5:
            continue
        ri, ro = tube['profile'][0][1], tube['profile'][1][1]
        z = tube['origin'][2]
        put(st, {'id': 'piston-' + c, 'kind': 'revolve', 'bone': shaft['bone'], 'material': 'paint-steel', 'role': 'detail', 'tone': 0.35, 'member': shaft['member'], 'part': 'member', 'profile': [[0, 0], [0, round((ri + ro) / 2, 4)], [0.1, round((ri + ro) / 2, 4)], [0.1, 0]], 'axis': [1, 0, 0], 'origin': [0, 0, z]})
    for b in ('stick-cyl-lug-right', 'stick-cyl-lug-left', 'stick-cyl-lug', 'bucket-cyl-lug-right', 'bucket-cyl-lug-left', 'bucket-cyl-lug', 'boom-rod-boss', 'boom-cyl-eye', 'stick-cyl-eye', 'bucket-cyl-eye', 'bucket-shell', 'bucket-cheek-right', 'bucket-cheek-left', 'bucket-lip', 'bucket-ear-right', 'bucket-ear-left', 'idler-hub'):
        if b in body and not (b == 'idler-hub' and d == '500') and not any(x.get('of') == b for x in st):
            put(st, {'id': ('st-' if b == 'idler-hub' else 'member-') + b, 'of': b, 'member': 'undercarriage' if b == 'idler-hub' else body[b]['bone'], 'part': 'member'})
    for g in ('grouser-bottom-l', 'grouser-bottom-r'):
        if g in body and not any(x.get('of') == g for x in st):
            put(st, {'id': 'st-' + g, 'of': g, 'member': 'undercarriage', 'part': 'member'})
    if d == '200':
        for rid, z in (('roller-l', 1.075), ('roller-r', -1.305)):
            body[rid]['profile'] = [[0, 0], [0, 0.09], [0.23, 0.09], [0.23, 0]]
            body[rid]['origin'][2] = z
json.dump(doc, open(P, 'w'), ensure_ascii=False, indent=1)
print('[DEBUG] written', {d: len(V[d]['structure']) for d in V})
