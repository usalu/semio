"""📦 Courier with hand truck: acromion line of the jacket, deltoid cap of the sleeves with their fleece and shirt shells, hip bones and shoulder blades of the skeleton, front elevations redrawn."""
exec(open('rework-06-skeleton.py').read())
A = 'humans/courier-with-hand-truck'
d = load(A)
YOKE = [(0.425, 0.0), (0.452, 0.008), (0.474, 0.020), (0.492, 0.010), (0.506, 0.002)]
for sc in ('50', '100', '200', '500'):
    widen(d, sc, 'body', 'jacket', YOKE, cap=0.200, lo=0.425)
for i in ('th-jacket-fleece', 'th-jacket-shirt'):
    widen(d, '50', 'thermal', i, YOKE, cap=0.198, lo=0.425)
CAP = {'50': [(-0.046, -0.004, -0.022, 0.024, 0.030), (-0.040, -0.004, -0.017, 0.038, 0.043), (-0.030, -0.003, -0.011, 0.048, 0.051), (-0.016, -0.002, -0.004, 0.055, 0.056)],
       '100': [(-0.046, -0.004, -0.022, 0.026, 0.032), (-0.034, -0.003, -0.013, 0.045, 0.049), (-0.016, -0.002, -0.004, 0.055, 0.056)],
       '200': [(-0.046, -0.004, -0.021, 0.030, 0.036), (-0.022, -0.002, -0.007, 0.052, 0.054)],
       '500': [(-0.044, -0.004, -0.018, 0.040, 0.044)]}
for sc, rows in CAP.items():
    for side, sg in (('l', 1), ('r', -1)):
        recap(d, sc, 'body', 'sleeve-' + side, rows, 0.0, 0.0, sg)
for side, sg in (('l', 1), ('r', -1)):
    recap(d, '50', 'thermal', 'th-sleeve-%s-fleece' % side, CAP['50'], 0.002, 0.0, sg)
    recap(d, '50', 'thermal', 'th-sleeve-%s-shirt' % side, CAP['50'], 0.006, 0.0, sg)
for sc, thick, coarse in (('50', 0.012, False), ('100', 0.016, False), ('200', 0.030, True)):
    for side in 'lr':
        base = find(d, sc, 'structure', 'ilium-' + side)
        ala, wing = hip_bone(side, 1.0, 0.095, 0.0, 0.004, thick, coarse, base)
        replace(d, sc, 'structure', 'ilium-' + side, ala)
        insert_after(d, sc, 'structure', 'ilium-' + side, wing)
        sb = find(d, sc, 'structure', 'scapula-' + side)
        if sb: replace(d, sc, 'structure', 'scapula-' + side, shoulder_blade(side, 1.0, thick, coarse, sb, 'zx', True, (0, -0.006, 0)))
for sc in ('50', '100', '200', '500'):
    d['volumes'][sc]['decisions'].append(NOTE_SHOULDER)
    if sc != '500': d['volumes'][sc]['decisions'].append(NOTE_BONES)
save(A, d)

R = [(0.200, 1.5121), (0.178, 1.5341), (0.118, 1.5521)]
L = [(-x, y) for x, y in reversed(R)]
def redraw(reps, asset):
    """Front elevation of the jacket: acromion at 0.20 m and 1.51 m, trapezius slope to the collar."""
    t = shape(reps['50']['geometry'], 'front', 'torso')
    t['points'] = repoint(t['points'], [((0.1917, 1.5128), R), ((0.1567, 1.5348), []), ((0.1122, 1.5503), []), ((-0.1123, 1.5503), L), ((-0.1568, 1.5348), []), ((-0.1928, 1.5108), [])])
    t = shape(reps['100']['geometry'], 'front', 'torso')
    t['points'] = repoint(t['points'], [((0.1917, 1.5128), R), ((0.1587, 1.5338), []), ((-0.1568, 1.5348), L), ((-0.1918, 1.5128), [])])
    t = shape(reps['200']['geometry'], 'front', 'torso')
    t['points'] = repoint(t['points'], [((0.1935, 1.5096), R[:2]), ((0.1595, 1.5336), []), ((-0.1555, 1.5356), L[1:]), ((-0.1935, 1.5096), [])])
    t = shape(reps['500']['geometry'], 'front', 'torso')
    t['points'] = repoint(t['points'], [((0.1555, 1.5356), [R[1]]), ((-0.1555, 1.5356), [L[1]])])

root_edit(A, 'representations', redraw, out=sys.argv[1] if len(sys.argv) > 1 else None)
