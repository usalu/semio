"""🦴 Hand-drawn hip bone and shoulder blade of author 06: the outlines are stated here once in bone-local metres and placed per figure by its own numbers (size factor, hip-joint half spacing, scapula seat)."""
exec(open('rework-06-lib.py').read())

ALA = [(0.000, -0.030), (0.030, -0.040), (0.050, -0.045), (0.082, -0.068), (0.105, -0.055), (0.122, -0.030), (0.130, 0.000), (0.130, 0.010), (-0.020, 0.010), (-0.018, -0.018)]
ALA_COARSE = [(0.000, -0.030), (0.050, -0.045), (0.082, -0.068), (0.122, -0.030), (0.130, 0.010), (-0.020, 0.010)]
WING = [(0.130, 0.000), (0.127, 0.022), (0.115, 0.046), (0.085, 0.068), (0.060, 0.062), (0.035, 0.040), (0.020, 0.028), (-0.005, 0.024), (-0.020, 0.000)]
WING_COARSE = [(0.130, 0.000), (0.115, 0.046), (0.085, 0.068), (0.035, 0.040), (-0.020, 0.000)]
BLADE = [(0.070, -0.036), (0.062, 0.010), (0.058, 0.046), (0.036, 0.052), (0.020, 0.040), (-0.030, 0.012), (-0.078, -0.010), (-0.070, -0.024), (-0.010, -0.046), (0.045, -0.050)]
BLADE_COARSE = [(0.070, -0.036), (0.058, 0.046), (0.036, 0.052), (-0.078, -0.010), (-0.070, -0.024), (0.045, -0.050)]

def hip_bone(side, k, hip_w, origin_u, origin_v, thick, coarse, base):
    """Two plates hinged on the line through the iliac tubercle: the ala (flared 14 degrees, facing sideways) and the posterior wing (turned 42 degrees towards the sacrum, flared 10 degrees)."""
    s = 1 if side == 'l' else -1
    out = []
    for name, prof, toe, flare, dv in (('ilium', ALA_COARSE if coarse else ALA, 0, 14, 0.0), ('ilium-wing', WING_COARSE if coarse else WING, 42, 10, 0.010)):
        q = qmul(qaxis('v', flare), qaxis('u', toe))
        if s < 0: q = qmirror(q)
        pts = [[round(u * k, 4), round((v - (0.010 if name == 'ilium' else 0.0)) * k, 4)] for u, v in prof]
        o = {k2: base[k2] for k2 in ('material', 'role', 'tone', 'member', 'part') if k2 in base}
        o.update({'id': '%s-%s' % (name, side), 'kind': 'extrude', 'bone': 'pelvis', 'plane': 'xy', 'profile': pts, 'holes': [], 'from': -thick / 2, 'to': thick / 2,
                  'transform': {'rotation': rq(q), 'translation': [origin_u, round(origin_v + 0.010 * k, 4), round(s * (hip_w + 0.0085 * k), 4)]}})
        out.append(o)
    return out

def shoulder_blade(side, k, thick, coarse, base, plane, flip, shift=(0, 0, 0)):
    """Triangular blade with superior, inferior and glenoid angle in the frame of the disc it replaces (thin axis, long axis up, short axis lateral)."""
    s = 1 if side == 'l' else -1
    pts = [[round(a * k, 4), round(s * b * k, 4)] for a, b in (BLADE_COARSE if coarse else BLADE)]
    if flip: pts = [[p[1], p[0]] for p in pts]
    o = {k2: base[k2] for k2 in ('id', 'bone', 'material', 'role', 'tone', 'member', 'part', 'transform') if k2 in base}
    o.update({'kind': 'extrude', 'plane': plane, 'profile': pts, 'holes': [], 'from': -thick / 2, 'to': thick / 2})
    t = dict(o.get('transform', {}))
    t['translation'] = [round(a + b * (s if i == 2 else 1), 5) for i, (a, b) in enumerate(zip(t.get('translation', [0, 0, 0]), shift))]
    o['transform'] = t
    return o
