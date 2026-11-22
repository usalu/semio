"""🚴 Cyclist: hip bones and shoulder blades of the rider's skeleton as drawn plates (female build: factor 0.95 and 0.90, hip joints 0.176 m apart); the bicycle under her was measured and kept."""
exec(open('rework-06-skeleton.py').read())
A = 'humans/cyclist'
d = load(A)
for sc, thick, coarse in (('50', 0.012, False), ('100', 0.016, False), ('200', 0.030, True)):
    for side in 'lr':
        base = find(d, sc, 'structure', 'ilium-' + side)
        ala, wing = hip_bone(side, 0.95, 0.088, 0.0, 0.004, thick, coarse, base)
        replace(d, sc, 'structure', 'ilium-' + side, ala)
        insert_after(d, sc, 'structure', 'ilium-' + side, wing)
        sb = find(d, sc, 'structure', 'scapula-' + side)
        if sb: replace(d, sc, 'structure', 'scapula-' + side, shoulder_blade(side, 0.90, thick, coarse, sb, 'zx', True))
    d['volumes'][sc]['decisions'].append(NOTE_BONES)
save(A, d)
