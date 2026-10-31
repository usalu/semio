"""NW-F scratch library on top of lib.py: thermal layer helpers, pose/clip builders, an own validator that mirrors the official check
(references, bindings, extents, mass, region coverage, loop closing) and an own print-suitability estimate (ink gap + feature width,
same definitions as the representation module) so drawings can be iterated before the official CLIs are free. Every shape is authored
by hand in the per-asset scripts; nothing here simplifies or generates geometry."""
import json, math, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib
from lib import *  # noqa

PENS = {
    '500': dict(body=0.18, detail=None, texture=None, glazing=None, ground=0.25),
    '200': dict(body=0.25, detail=0.13, texture=None, glazing=0.13, ground=0.35),
    '100': dict(body=0.35, detail=0.18, texture=0.13, glazing=0.13, ground=0.5),
    '50': dict(body=0.5, detail=0.25, texture=0.13, glazing=0.18, ground=0.7),
}
lib.PENS = PENS
RANGES = {'500': (351, 1000), '200': (151, 350), '100': (76, 150), '50': (20, 75)}
OUT = f'{S}/🗂️catalog/🛋️furniture'


def asset_path(aid):
    os.makedirs(f'{OUT}/{aid}', exist_ok=True)
    return f'{OUT}/{aid}/🔣️.json'


#region references
REFS = {
    'iso-10456-2007': 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (conductivity, specific heat capacity, density of timber, wood-based boards, steel, aluminium, flexible polyurethane foam, rubber).',
    'en-1991-1-1': 'EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: General actions - densities, self-weight, imposed loads for buildings (Table 6.2 category A floors 1.5-2.0 kN/m2, category C tables 3.0 kN/m2, Table 6.7/6.8 storage, Annex A densities of stored materials).',
    'en-1993-1-1': 'EN 1993-1-1:2005 Eurocode 3: Design of steel structures, Part 1-1 (E = 210 GPa, density 7850 kg/m3).',
    'en-1995-1-1': 'EN 1995-1-1:2004 Eurocode 5: Design of timber structures, Part 1-1 (characteristic strength and stiffness classes of solid timber and wood-based panels).',
    'en-312': 'EN 312:2010 Particleboards - Specifications (type P5 load-bearing board in humid conditions: bending strength 15 MPa, modulus of elasticity 2.55-3.2 GPa).',
    'en-12520': 'EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating (static seat and back test loads are defined through EN 1728).',
    'en-1728': 'EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (static load on seat and back).',
    'en-1730': 'EN 1730:2012 Furniture - Tables - Test methods for the determination of stability, strength and durability (vertical and horizontal static loads on the table top).',
    'en-12521': 'EN 12521:2015 Furniture - Strength, durability and safety - Requirements for domestic tables.',
    'en-14749': 'EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (shelf load tests with a uniformly distributed load).',
    'en-1725': 'EN 1725:1998 Domestic furniture - Beds and mattresses - Safety requirements and test methods (static load tests of bed bases).',
    'en-1957': 'EN 1957:2012 Furniture - Beds and mattresses - Test methods for the determination of functional characteristics.',
    'en-581-1': 'EN 581-1:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements (stability, folding mechanisms).',
    'en-581-2': 'EN 581-2:2015 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 2: Mechanical safety requirements and test methods for seating (static seat load test).',
    'en-581-3': 'EN 581-3:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 3: Mechanical safety requirements and test methods for tables.',
    'iso-7730-2005': 'ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (outer clothing surface temperatures of seated persons around 28-32 degC in comfort conditions; 30 degC used for the contact surface).',
    'iso-9920-2007': 'ISO 9920:2007 Ergonomics of the thermal environment - Estimation of thermal insulation and water vapour resistance of a clothing ensemble (light summer clothing 0.5 clo, 1 clo = 0.155 m2K/W).',
    'incropera-2018': 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (surface radiative properties, convection correlations, thermal penetration depth).',
    'wood-handbook-2021': 'Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (density and elastic modulus of oak, beech and pine; air-dry rounded values).',
    'iso-3386-1': 'ISO 3386-1:1986 Polymeric materials, cellular flexible - Determination of stress-strain characteristics in compression, Part 1 (compression stress at 40 % strain of 3-6 kPa typical for upholstery foams of 30-40 kg/m3).',
    'iso-9227-din-en-12150': 'DIN 4102 / ISO 10456 table note: matt painted and powder-coated metal surfaces have emissivity of about 0.9; chromium-plated polished steel about 0.1 (Incropera, Table A.11).',
    'system-32': 'System 32 (Lochreihen-System): 32 mm hole pitch, 37 mm to the front edge, 5 mm pin diameter, the European standard for adjustable shelf pins in carcass furniture (DIN 68861-8 / industry standard).',
    'semio-nwf': 'Semio staffage content NW-F: dimensions and masses are authored from typical market products (see provenance notes); parameters tagged authored-estimate are not taken from a datasheet.',
}


def refs(*ids):
    return [{'id': i, 'citation': REFS[i]} for i in ids]
#endregion


#region materials and layers
def mat(mid, en, de, eps, absorb, rho, lam, cp, E=None, strength=None, source='literature', ref='iso-10456-2007'):
    d = {'id': mid, 'title': T(en, de), 'emissivity': eps, 'solarAbsorptivity': absorb, 'densityKgM3': rho, 'conductivityWmK': lam, 'specificHeatJKgK': cp}
    if E:
        d['elasticModulusPa'] = E
    if strength:
        d['strengthPa'] = strength
    d['source'] = source
    d['reference'] = ref
    return d


def cap(rho, cp, lam, t):
    return rho * cp * min(t, math.sqrt(lam / (rho * cp) * 3600.0))


def lay(lid, en, de, kind, m, t, M, source='literature', ref='iso-10456-2007', vap=None, resistance=None):
    c = cap(M['densityKgM3'], M['specificHeatJKgK'], M['conductivityWmK'], t)
    d = {'id': lid, 'title': T(en, de), 'kind': kind, 'material': m, 'thicknessM': t,
         'resistanceM2KW': r6(resistance if resistance is not None else t / M['conductivityWmK']), 'heatCapacityJM2K': round(c), 'source': source}
    if vap is not None:
        d['vapourResistanceM2PaW'] = vap
    if ref:
        d['reference'] = ref
    return d


def reg(rid, en, de, kind, material, area, L, geom, svf, layers, source='authored-estimate', ref='iso-10456-2007', hidden=None):
    c = sum(l['heatCapacityJM2K'] for l in layers)
    d = {'id': rid, 'title': T(en, de), 'kind': kind, 'model': 'passive', 'material': material, 'areaM2': area, 'characteristicLengthM': L,
         'geometry': geom, 'skyViewFactor': svf, 'layers': layers, 'heatCapacityJM2K': c, 'source': source, 'reference': ref}
    if hidden:
        d['hidden'] = hidden
    return d
#endregion


#region poses, clips, loads
class Ctx:
    def __init__(self, regions, base):
        self.regions = regions
        self.base = base

    def bind(self, over=None, contact=None):
        out = {}
        for rid in self.regions:
            v = self.base[rid]
            out[rid] = {'exposedFraction': v[0], 'sunlitFraction': v[1]} if isinstance(v, tuple) else dict(v)
        for rid, o in (over or {}).items():
            if isinstance(o, tuple):
                out[rid] = {'exposedFraction': o[0], 'sunlitFraction': o[1]}
            else:
                out[rid].update(o)
        for rid, c in (contact or {}).items():
            out[rid]['contact'] = {'fraction': c[0], 'tempC': 30}
            out[rid]['exposedFraction'] = round(max(0.0, out[rid]['exposedFraction'] - c[0]), 4)
        return out


def pose(ctx, pid, en, de, den, dde, bones=None, over=None, contact=None, contacts=(), loads=(), key=False, root=None, emitters=None):
    p = {'id': pid, 'title': T(en, de), 'description': T(den, dde)}
    if key:
        p['key'] = True
    p.update({'bones': bones or {}})
    if root:
        p['root'] = root
    p.update({'ground': 'auto', 'contacts': list(contacts), 'thermal': {'regions': ctx.bind(over, contact), 'emitters': emitters or {}}, 'loads': list(loads)})
    return p


def load(lid, en, de, kind, fy, bone=None, anchor=None, x=0.0, y=0.0, fx=0.0, source='authored-estimate', ref=None, dist=None):
    d = {'id': lid, 'kind': kind, 'title': T(en, de)}
    if anchor:
        d['anchor'] = anchor
    elif dist:
        d['distributed'] = dist
    else:
        d.update({'bone': bone, 'x': x, 'y': y})
    d.update({'fxN': fx, 'fyN': fy, 'source': source})
    if ref:
        d['reference'] = ref
    return d


def clip(cid, en, de, den, dde, dur, keys, contacts, phases, events, tracks=(), fps=30, loop=False):
    return {'id': cid, 'title': T(en, de), 'description': T(den, dde), 'durationS': dur, 'fps': fps, 'loop': loop, 'rootMotion': 'none',
            'keys': keys, 'contacts': contacts, 'phases': [{'id': i, 'fromS': a, 'toS': b, 'title': T(e, d)} for i, a, b, e, d in phases],
            'events': [{'t': t, 'kind': k, 'title': T(e, d), **({'target': tg} if tg else {})} for t, k, e, d, tg in events],
            'tracks': [{'target': t, 'ease': e, 'keys': [{'t': a, 'v': v} for a, v in ks]} for t, e, ks in tracks]}


def structure(supports, notes):
    return {'gravityMS2': 9.80665, 'dynamics': 'quasi-static', 'supports': supports, 'assumptions': [T(e, d) for e, d in notes], 'clinical': False}


def D(sil, ret, mer, omi, thin, hat, sty, ove):
    keys = ['silhouette', 'retained', 'merged', 'omitted', 'thinElements', 'hatching', 'styles', 'overlays']
    return {k: T(*v) for k, v in zip(keys, (sil, ret, mer, omi, thin, hat, sty, ove))}


def rbox(x0, y0, x1, y1, rt=0.0, rb=0.0):
    c = [('M', x0, y0 + rb)]
    if rb:
        pass
    c += [('L', x0, y1 - rt)]
    if rt:
        c += [('Q', x0, y1, x0 + rt, y1)]
    c += [('L', x1 - rt, y1)]
    if rt:
        c += [('Q', x1, y1, x1, y1 - rt)]
    c += [('L', x1, y0 + rb)]
    if rb:
        c += [('Q', x1, y0, x1 - rb, y0)]
        c += [('L', x0 + rb, y0), ('Q', x0, y0, x0, y0 + rb)]
    c += [('Z',)]
    return c


def anc(rig, aid, bone, wx, wy, kind, en, de, **kw):
    lx, ly = rig.local(bone, wx, wy)
    d = {'id': aid, 'bone': bone, 'x': r6(lx), 'y': r6(ly), 'kind': kind, 'title': T(en, de)}
    d.update(kw)
    return d


def bone(bid, parent, en, de, x, y, ang, length, joint, mass=None, section=None):
    d = {'id': bid, 'parent': parent, 'title': T(en, de), 'x': x, 'y': y, 'angleDeg': ang, 'lengthM': length, 'joint': joint}
    if mass:
        kg, cx, cy, dist, src, ref = mass
        d['mass'] = {'kg': kg, 'comX': cx, 'comY': cy, 'distribution': dist, 'source': src}
        if ref:
            d['mass']['reference'] = ref
    if section:
        d['section'] = section
    return d


FIXED = {'type': 'fixed'}


def prism(axis, lo, hi):
    return {'type': 'prismatic', 'axisDeg': axis, 'minM': lo, 'maxM': hi}


def rev(lo, hi):
    return {'type': 'revolute', 'minDeg': lo, 'maxDeg': hi}


def ref_asset(den):
    return den


def assemble(aid, title, summary, tags, prov_notes, dims, materials, rig, anchors, reps, symbols, poses, clips, thermal, struct, presentation, reference_ids):
    return sorted_keys({'id': aid, 'category': 'furniture', 'title': title, 'summary': summary, 'tags': tags,
                        'provenance': {'author': 'Semio staffage content (NW-F)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs(*reference_ids), 'notes': prov_notes},
                        'dimensions': dims, 'materials': materials, 'rig': rig, 'anchors': anchors, 'representations': reps, 'symbols': symbols,
                        'poses': poses, 'clips': clips, 'thermal': {**thermal, 'passengers': []}, 'structure': struct, 'presentation': presentation,
                        'exports': ['svg', 'pdf', 'dxf', 'dwg', 'png-sequence', 'mp4', 'webm', 'component'], 'completion': 'authored'})
#endregion


#region validation
def validate(A, verbose=True):
    errs = []
    E = errs.append
    bones = {b['id']: b for b in A['rig']['bones']}
    regs = {r['id']: r for r in A['thermal']['regions']}
    mats = {m['id'] for m in A['materials']}
    anchors = {a['id'] for a in A['anchors']}
    refids = {r['id'] for r in A['provenance']['references']}
    for key, ids in (('bones', [b['id'] for b in A['rig']['bones']]), ('regions', list(regs)), ('anchors', [a['id'] for a in A['anchors']]),
                     ('poses', [p['id'] for p in A['poses']]), ('clips', [c['id'] for c in A['clips']])):
        if len(ids) != len(set(ids)):
            E(f'duplicate {key}')
    roots = [b for b in bones.values() if b['parent'] is None]
    if len(roots) != 1 or roots[0]['id'] != A['rig']['root']:
        E('rig root')
    for b in bones.values():
        if b['parent'] and b['parent'] not in bones:
            E(f'bone parent {b["id"]}')
        if 'mass' in b and not b['mass'].get('source'):
            E('mass source')
        if 'mass' in b and b['mass'].get('reference') and b['mass']['reference'] not in refids:
            E(f'mass ref {b["mass"]["reference"]}')
    for m in A['materials']:
        if m.get('reference') and m['reference'] not in refids:
            E(f'material ref {m["reference"]}')
    for a in A['anchors']:
        if a['bone'] not in bones:
            E(f'anchor bone {a["id"]}')
    total = sum(b['mass']['kg'] for b in bones.values() if 'mass' in b)
    if abs(total - A['dimensions']['massKg']) > 0.05:
        E(f'mass sum {total:.2f} vs {A["dimensions"]["massKg"]}')
    for r in regs.values():
        if r['material'] not in mats:
            E(f'region material {r["id"]}')
        for l in r['layers']:
            if l['material'] not in mats:
                E(f'layer material {l["id"]}')
            if l.get('reference') and l['reference'] not in refids:
                E(f'layer ref {l["reference"]}')
        if r.get('reference') and r['reference'] not in refids:
            E(f'region ref {r["reference"]}')
    allpose = {p['id']: p for p in A['poses']}
    if len([p for p in A['poses'] if not p.get('key')]) < 3:
        E('poses < 3')
    if len(A['clips']) < 2:
        E('clips < 2')
    emitters = {e['id'] for e in A['thermal']['emitters']}
    for p in A['poses']:
        for bid, o in p['bones'].items():
            if bid not in bones:
                E(f'pose {p["id"]} bone {bid}')
            else:
                j = bones[bid]['joint']
                if ('dx' in o or 'dy' in o) and j['type'] != 'prismatic':
                    E(f'pose {p["id"]} translates non-prismatic {bid}')
                if 'angleDeg' in o and j['type'] != 'revolute':
                    E(f'pose {p["id"]} rotates non-revolute {bid}')
                if j['type'] == 'revolute' and 'angleDeg' in o and not (j.get('minDeg', -1e9) - 1e-6 <= o['angleDeg'] <= j.get('maxDeg', 1e9) + 1e-6):
                    E(f'pose {p["id"]} {bid} angle {o["angleDeg"]} outside limits')
                if j['type'] == 'prismatic':
                    ax = math.radians(j.get('axisDeg', 0))
                    v = o.get('dx', 0) * math.cos(ax) + o.get('dy', 0) * math.sin(ax)
                    if not (j.get('minM', -1e9) - 1e-6 <= v <= j.get('maxM', 1e9) + 1e-6):
                        E(f'pose {p["id"]} {bid} travel {v:.4f} outside limits')
        for rid in regs:
            if rid not in p['thermal']['regions']:
                E(f'pose {p["id"]} region {rid} unbound')
        for rid in p['thermal']['regions']:
            if rid not in regs:
                E(f'pose {p["id"]} binds unknown region {rid}')
        for c in p['contacts']:
            if c not in anchors:
                E(f'pose contact {c}')
        for ld in p['loads']:
            for k in ('anchor',):
                if ld.get(k) and ld[k] not in anchors:
                    E(f'load anchor {ld["id"]}')
            if ld.get('bone') and ld['bone'] not in bones:
                E(f'load bone {ld["id"]}')
            if ld.get('reference') and ld['reference'] not in refids:
                E(f'load ref {ld["reference"]}')
        for e in emitters:
            if e not in p['thermal']['emitters']:
                E(f'pose emitter {e}')
    loadids = {ld['id'] for p in A['poses'] for ld in p['loads']}
    for c in A['clips']:
        ts = [k['t'] for k in c['keys']]
        if ts != sorted(set(ts)) or ts[0] < 0 or ts[-1] > c['durationS']:
            E(f'clip {c["id"]} key times')
        for k in c['keys']:
            if k['pose'] not in allpose:
                E(f'clip {c["id"]} pose {k["pose"]}')
            for bid in k.get('bones', {}):
                if bid not in bones:
                    E(f'clip bone {bid}')
        for s in c['contacts']:
            if s['anchor'] not in anchors or not (0 <= s['fromS'] < s['toS'] <= c['durationS']):
                E(f'clip {c["id"]} contact span')
        if c['loop']:
            a, b = allpose[c['keys'][0]['pose']], allpose[c['keys'][-1]['pose']]
            for bid in bones:
                if a['bones'].get(bid, {}) != b['bones'].get(bid, {}):
                    E(f'clip {c["id"]} loop closing {bid}')
        for t in c['tracks']:
            tg = t['target']
            ok = tg in ('activityMet', 'externalWorkW', 'breathing') or (tg.startswith('load:') and tg[5:] in loadids) or (tg.startswith('emitter:') and tg[8:] in emitters) \
                or (tg.startswith('region:') and tg.split(':')[1] in regs)
            if not ok:
                E(f'clip track {tg}')
            ts = [k['t'] for k in t['keys']]
            if ts != sorted(ts):
                E(f'track order {tg}')
        for ev in c['events']:
            if not 0 <= ev['t'] <= c['durationS']:
                E('event range')
            if ev.get('target') and ev['kind'] in ('load-on', 'load-off') and ev['target'] not in loadids:
                E(f'event load {ev["target"]}')
    dims = A['dimensions']
    rig0 = Rig(A['rig']['bones'])
    for k in ('500', '200', '100', '50'):
        r = A['representations'][k]
        if r['status'] == 'unavailable':
            continue
        if not (r['validRange']['minDenominator'] <= r['denominator'] <= r['validRange']['maxDenominator']):
            E(f'{k} validRange')
        shapes = r['geometry']['elevation']
        ids = [s['id'] for s in shapes]
        if len(ids) != len(set(ids)):
            E(f'{k} duplicate shape ids')
        hs = {h['id'] for h in r['hatches']}
        drawn = set()
        for s in shapes:
            if s['bone'] not in bones:
                E(f'{k} shape {s["id"]} bone')
            if s.get('region'):
                drawn.add(s['region'])
                if s['region'] not in regs:
                    E(f'{k} shape {s["id"]} region')
            for rr in s.get('regions', []):
                drawn.add(rr)
                if rr not in regs:
                    E(f'{k} shape {s["id"]} regions')
            if not s.get('region') and not s.get('regions') and s['role'] not in ('ground',) and s['filled']:
                E(f'{k} filled shape {s["id"]} has no region')
            if s.get('material') and s['material'] not in mats:
                E(f'{k} shape material')
            if s.get('hatch') and s['hatch'] not in hs:
                E(f'{k} hatch {s["hatch"]}')
            if s.get('penMm') is None and r['pens'].get(s['role']) is None:
                E(f'{k} shape {s["id"]} role {s["role"]} has no pen')
        for rid, rg in regs.items():
            if rid not in drawn and not rg.get('hidden'):
                E(f'{k} region {rid} undrawn')
        x0, y0, x1, y1 = bbox_world(rig0, [s for s in shapes if s['role'] != 'ground'])
        w, h = x1 - x0, y1 - y0
        for nm, got, want in (('width', w, dims['widthM']), ('height', h, dims['heightM'])):
            if abs(got - want) > max(0.02 * want, 0.015):
                E(f'{k} {nm} {got:.3f} vs {want}')
        if verbose:
            print(f'  {k}: {len(shapes)} shapes, bbox {w:.3f} x {h:.3f} (y0 {y0:.3f}), regions drawn {len(drawn)}/{len(regs)}')
    dd = A['presentation']
    if dd['defaultPose'] not in allpose or dd['defaultClip'] not in {c['id'] for c in A['clips']}:
        E('presentation')
    for s in A['symbols']:
        for sh in s['geometry'].get('plan', []) + s['geometry']['elevation']:
            if sh['bone'] != A['rig']['root']:
                E(f'symbol {s["id"]} shape bone')
    for e in errs:
        print('  ERROR', e)
    return errs
#endregion


#region print suitability estimate
def _seg_inter(p, q, r, s):
    d1 = (q[0] - p[0], q[1] - p[1])
    d2 = (s[0] - r[0], s[1] - r[1])
    den = d1[0] * d2[1] - d1[1] * d2[0]
    if abs(den) < 1e-14:
        return False
    t = ((r[0] - p[0]) * d2[1] - (r[1] - p[1]) * d2[0]) / den
    u = ((r[0] - p[0]) * d1[1] - (r[1] - p[1]) * d1[0]) / den
    return 0 <= t <= 1 and 0 <= u <= 1


def _pt_seg(x, a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    l2 = dx * dx + dy * dy
    t = 0 if l2 == 0 else max(0, min(1, ((x[0] - a[0]) * dx + (x[1] - a[1]) * dy) / l2))
    return math.hypot(x[0] - (a[0] + t * dx), x[1] - (a[1] + t * dy))


def _inside(pt, ring):
    ins = False
    j = len(ring) - 1
    for i in range(len(ring)):
        a, b = ring[i], ring[j]
        if (a[1] > pt[1]) != (b[1] > pt[1]) and pt[0] < (b[0] - a[0]) * (pt[1] - a[1]) / (b[1] - a[1]) + a[0]:
            ins = not ins
        j = i
    return ins


def _edt1(f):
    n = len(f)
    INF = 1e20
    v = [0] * n
    z = [0.0] * (n + 1)
    k = 0
    z[0], z[1] = -INF, INF
    for q in range(1, n):
        while True:
            s = ((f[q] + q * q) - (f[v[k]] + v[k] * v[k])) / (2 * q - 2 * v[k])
            if s <= z[k] and k > 0:
                k -= 1
            else:
                break
        if s <= z[k]:
            s = z[k]
        k += 1
        v[k] = q
        z[k] = s
        z[k + 1] = INF
    out = [0.0] * n
    k = 0
    for q in range(n):
        while z[k + 1] < q:
            k += 1
        out[q] = (q - v[k]) ** 2 + f[v[k]]
    return out


def _edt(mask, w, h):
    INF = 1e20
    g = [[0.0 if mask[y][x] else INF for x in range(w)] for y in range(h)]
    cols = []
    for x in range(w):
        col = _edt1([g[y][x] for y in range(h)])
        cols.append(col)
    out = [[0.0] * w for _ in range(h)]
    for y in range(h):
        row = _edt1([cols[x][y] for x in range(w)])
        out[y] = row
    return out


def feature_width_ok(ring_mm, min_feature=0.2):
    """True when the opening of the filled ring with a disk of min_feature removes no more than the official tolerance."""
    xs = [p[0] for p in ring_mm]
    ys = [p[1] for p in ring_mm]
    h = min_feature / 6.0
    x0, y0 = min(xs) - 2 * h, min(ys) - 2 * h
    w = int((max(xs) - x0) / h) + 3
    hh = int((max(ys) - y0) / h) + 3
    if w * hh > 600000:
        return True, None
    mask = [[_inside((x0 + (i + 0.5) * h, y0 + (j + 0.5) * h), ring_mm) for i in range(w)] for j in range(hh)]
    area = sum(sum(1 for v in row if v) for row in mask) * h * h
    if area == 0:
        return True, 0
    dist = _edt([[not v for v in row] for row in mask], w, hh)
    r = min_feature / 2.0 / h
    core = [[mask[j][i] and dist[j][i] >= r * r for i in range(w)] for j in range(hh)]
    if not any(any(r_) for r_ in core):
        return False, area
    dcore = _edt(core, w, hh)
    removed = sum(1 for j in range(hh) for i in range(w) if mask[j][i] and dcore[j][i] > r * r) * h * h
    tol = max(0.01 * area, (min_feature / 2) ** 2) + 4 * (1 - math.pi / 4) * (min_feature / 2) ** 2
    return removed <= tol, removed


def suitability(A, key, poses=None, feature=True, gap_min=0.15, verbose=True):
    rep_ = A['representations'][key]
    den = int(key)
    k = 1000.0 / den
    pens = rep_['pens']
    problems = []
    names = poses or [p['id'] for p in A['poses'] if not p.get('key')]
    for pid in names:
        P = next(p for p in A['poses'] if p['id'] == pid)
        rig = Rig(A['rig']['bones'], P['bones'])
        items = []
        for s in rep_['geometry']['elevation']:
            pen = s.get('penMm', pens.get(s['role']))
            if pen is None or s['role'] == 'ground':
                continue
            rings = [([(x * k, -y * k) for x, y in pts], closed) for pts, closed in world_outline(rig, s)]
            items.append((s, pen, rings))
            if feature and s['filled'] and pid == names[0]:
                for pts, closed in rings:
                    if closed:
                        ok, rem = feature_width_ok(pts)
                        if not ok:
                            problems.append(('feature', pid, s['id'], rem))
        for i in range(len(items)):
            si, pi, ri = items[i]
            bi = [(min(p[0] for r, _ in ri for p in r), min(p[1] for r, _ in ri for p in r), max(p[0] for r, _ in ri for p in r), max(p[1] for r, _ in ri for p in r))][0]
            for j in range(i + 1, len(items)):
                sj, pj, rj = items[j]
                bj = (min(p[0] for r, _ in rj for p in r), min(p[1] for r, _ in rj for p in r), max(p[0] for r, _ in rj for p in r), max(p[1] for r, _ in rj for p in r))
                m = 1.5 + pi + pj
                if bi[2] + m < bj[0] or bj[2] + m < bi[0] or bi[3] + m < bj[1] or bj[3] + m < bi[1]:
                    continue
                best = 1e9
                joined = False
                for a, ca in ri:
                    ea = [(a[n], a[n + 1]) for n in range(len(a) - 1)] + ([(a[-1], a[0])] if ca else [])
                    for b, cb in rj:
                        eb = [(b[n], b[n + 1]) for n in range(len(b) - 1)] + ([(b[-1], b[0])] if cb else [])
                        for p, q in ea:
                            for r_, s_ in eb:
                                if _seg_inter(p, q, r_, s_):
                                    joined = True
                                    break
                                best = min(best, _pt_seg(p, r_, s_), _pt_seg(q, r_, s_), _pt_seg(r_, p, q), _pt_seg(s_, p, q))
                            if joined:
                                break
                        if joined:
                            break
                        if ca and a and _inside(b[0], a) and all(_inside(x, a) for x in b):
                            joined = True
                        if cb and b and _inside(a[0], b) and all(_inside(x, b) for x in a):
                            joined = True
                    if joined:
                        break
                if joined or best >= 1e8:
                    continue
                if best < 1e-4:
                    continue
                gap = max(0.0, best - pi / 2 - pj / 2)
                if gap < gap_min and gap < 1.0:
                    problems.append(('gap', pid, si['id'] + ' | ' + sj['id'], round(gap, 3)))
    seen = {}
    for p in problems:
        kk = (p[0], p[2])
        if kk not in seen or p[3] < seen[kk][3]:
            seen[kk] = p
    if verbose:
        print(f'  1:{key} suitability: {len(seen)} problems')
        for p in sorted(seen.values(), key=lambda t: (t[0], t[2])):
            print('    ', p)
    return list(seen.values())
#endregion


def write(A, path):
    doc = {'$schema': '../../../🧬️schema/🔣️.json#/$defs/AssetDocument', 'version': 'semio.staffage/v2', 'asset': A}
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write('\n')
    print('wrote', path, os.path.getsize(path), 'bytes')


def preview(A, key, pose_id=None, name=None, dpi=600, zoom=1.0):
    rep_ = A['representations'][key]
    P = next(p for p in A['poses'] if p['id'] == (pose_id or A['presentation']['defaultPose']))
    rig = Rig(A['rig']['bones'], P['bones'])
    return render(rep_, rig, int(key), name or f'{A["id"]}-{key}-{P["id"]}', dpi=dpi, zoom=zoom)


def main(A, aid, suit_poses=None):
    errs = validate(A)
    if '--suit' in sys.argv:
        for k in ('500', '200', '100', '50'):
            suitability(A, k, poses=suit_poses, feature='--nofeat' not in sys.argv)
    if '--write' in sys.argv and not errs:
        write(A, asset_path(aid))
    for k in sys.argv:
        if k.startswith('--prev='):
            den, _, pid = k[7:].partition(':')
            print(preview(A, den, pid or None, zoom=8))
