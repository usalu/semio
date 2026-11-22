"""👷 Worker carrying a box, pass A: shoulder girdle of the garment (acromion line), deltoid cap of the sleeves, hip bones and shoulder blades of the skeleton."""
exec(open('rework-06-skeleton.py').read())
A = 'humans/worker-carrying-box'
d = load(A)
YOKE = [(0.399, 0.0), (0.411, 0.002), (0.421, 0.008), (0.431, 0.018), (0.441, 0.029), (0.451, 0.037), (0.461, 0.036), (0.470, 0.025), (0.479, 0.013), (0.487, 0.005), (0.493, 0.002)]
YOKE_IDS = {'50': {'body': ['shirt'], 'thermal': ['th-shirt-yoke-air', 'th-shirt-yoke-tee', 'th-shirt-yoke', 'th-vest-air', 'th-vest-tee']}, '100': {'body': ['shirt-yoke'], 'thermal': []}, '200': {'body': ['shirt-collar', 'vest'], 'thermal': []}, '500': {'body': ['vest', 'collar'], 'thermal': []}}
for sc, lists in YOKE_IDS.items():
    for lst, ids in lists.items():
        for i in ids:
            s = find(d, sc, lst, i)
            for q in s['sections']:
                u = q['at'][0]
                if u > 0.399:
                    q['rx'] = round(min(q['rx'] + lerp_table(YOKE, u), max(q['rx'], 0.200)), 4)
CAP = {
 '50': [(-0.046, 0, -0.022, 0.022, 0.030), (-0.042, 0, -0.018, 0.036, 0.044), (-0.034, 0, -0.012, 0.047, 0.054), (-0.022, 0, -0.006, 0.054, 0.061), (-0.008, -0.001, 0.0, 0.059, 0.066), (0.012, -0.002, 0.006, 0.062, 0.069), (0.040, -0.002, 0.009, 0.063, 0.070), (0.075, -0.002, 0.009, 0.061, 0.067), (0.110, -0.001, 0.006, 0.057, 0.062), (0.150, 0.002, 0.004, 0.054, 0.060), (0.200, 0.002, 0.002, 0.052, 0.056), (0.250, 0.001, 0, 0.049, 0.050), (0.290, 0, 0, 0.047, 0.047), (0.330, 0, 0, 0.046, 0.045)],
 '100': [(-0.046, 0, -0.022, 0.024, 0.032), (-0.036, 0, -0.013, 0.046, 0.053), (-0.016, 0, -0.003, 0.057, 0.064), (0.012, -0.002, 0.006, 0.062, 0.069), (0.045, -0.002, 0.009, 0.063, 0.070), (0.110, -0.001, 0.006, 0.057, 0.062), (0.200, 0.002, 0.002, 0.052, 0.056), (0.270, 0, 0, 0.048, 0.048), (0.330, 0, 0, 0.046, 0.045)],
 '200': [(-0.046, 0, -0.022, 0.030, 0.036), (-0.030, 0, -0.010, 0.050, 0.056), (0.005, -0.001, 0.004, 0.061, 0.068), (0.045, -0.002, 0.009, 0.063, 0.070), (0.120, 0, 0.005, 0.056, 0.061), (0.230, 0.001, 0, 0.050, 0.052), (0.330, 0, 0, 0.046, 0.045)],
 '500': [(-0.046, 0, -0.020, 0.040, 0.046), (0.000, -0.001, 0.002, 0.061, 0.067), (0.045, -0.002, 0.008, 0.063, 0.070), (0.160, 0.001, 0.003, 0.054, 0.058), (0.320, 0, 0, 0.048, 0.046)]}
for sc, rows in CAP.items():
    set_loft(find(d, sc, 'body', 'sleeve-l'), rows)
    set_loft(find(d, sc, 'body', 'sleeve-r'), mirror_rows(rows))
air = [(r[0] + (0.002 if i == 0 else 0), r[1], r[2], r[3] - 0.002, r[4] - 0.002) for i, r in enumerate(CAP['50'])]
set_loft(find(d, '50', 'thermal', 'th-sleeve-l-air'), air)
set_loft(find(d, '50', 'thermal', 'th-sleeve-r-air'), mirror_rows(air))
for sc, thick, coarse in (('50', 0.012, False), ('100', 0.016, False), ('200', 0.030, True)):
    for side in 'lr':
        base = find(d, sc, 'structure', 'ilium-' + side)
        ala, wing = hip_bone(side, 1.0, 0.0895, 0.0, 0.004, thick, coarse, base)
        replace(d, sc, 'structure', 'ilium-' + side, ala)
        insert_after(d, sc, 'structure', 'ilium-' + side, wing)
        sb = find(d, sc, 'structure', 'scapula-' + side)
        replace(d, sc, 'structure', 'scapula-' + side, shoulder_blade(side, 1.0, thick, coarse, sb, 'yz', False, (0.006, -0.010, 0)))
NOTE = {'en': 'Rework 2026-11-22: the garment carries the shoulder girdle out to the acromion (0.20 m from the midline, 35 mm above the shoulder joint) and slopes along the trapezius to the collar, so the deltoid cap of the sleeve grows out of the trunk instead of sitting beside it.', 'de': 'Überarbeitung 2026-11-22: Das Kleidungsstück führt den Schultergürtel bis zum Akromion (0,20 m von der Mitte, 35 mm über dem Schultergelenk) und fällt entlang des Trapezmuskels zum Kragen ab, sodass die Deltakappe des Ärmels aus dem Rumpf wächst, statt daneben zu sitzen.'}
BONES = {'en': 'Rework 2026-11-22: each hip bone is two plates with the drawn outline of the ilium (anterior superior and inferior spine, crest with tubercle, posterior spines, greater sciatic notch), the ala flared 14 degrees and the posterior wing turned 42 degrees onto the sacrum; each shoulder blade is a triangular plate with superior, inferior and glenoid angle lying on the ribs.', 'de': 'Überarbeitung 2026-11-22: Jedes Hüftbein besteht aus zwei Platten mit dem gezeichneten Umriss des Darmbeins (vorderer oberer und unterer Darmbeinstachel, Kamm mit Höcker, hintere Stachel, große Sitzbeinkerbe), die Schaufel um 14 Grad ausgestellt, der hintere Flügel um 42 Grad zum Kreuzbein gedreht; jedes Schulterblatt ist eine dreieckige Platte mit oberem, unterem und Gelenkwinkel auf den Rippen.'}
for sc in ('50', '100', '200', '500'):
    d['volumes'][sc]['decisions'].append(NOTE)
    if sc != '500': d['volumes'][sc]['decisions'].append(BONES)
v200 = find(d, '200', 'body', 'vest')
v200['sections'][-2]['rx'] = 0.200
v200['sections'][-1]['rx'] = 0.187
save(A, d)

def redraw(reps, asset):
    """Front elevations: the garment follows the acromion line of the model (shoulder point 0.20 m from the midline at 1.47 m, trapezius slope to the collar)."""
    g = reps['50']['geometry']
    shape(g, 'front', 'torso')['points'] = sym([(0.1688, 0.0896), (0.164, 0.2174), (0.165, 0.3134), (0.174, 0.3964), (0.178, 0.4204), (0.196, 0.4854), (0.199, 0.5000), (0.200, 0.5210), (0.200, 0.5310), (0.197, 0.5410), (0.187, 0.5510), (0.166, 0.5610), (0.137, 0.5700), (0.109, 0.5790), (0.0748, 0.5931), (0.0713, 0.5936), (0.0658, 0.6151)])
    g = reps['100']['geometry']
    shape(g, 'front', 'torso')['points'] = sym([(0.175, 0.4394), (0.198, 0.4984), (0.200, 0.5300), (0.196, 0.5420), (0.186, 0.5510), (0.137, 0.5700), (0.074, 0.5934)])
    g = reps['200']['geometry']
    shape(g, 'front', 'torso')['points'] = sym([(0.150, 0.4999), (0.136, 0.5590), (0.068, 0.5969)])
    shape(g, 'front', 'vest')['points'] = sym([(0.1895, 0.0899), (0.170, 0.4614), (0.200, 0.5294), (0.187, 0.5514)])
    g = reps['500']['geometry']
    shape(g, 'front', 'torso')['points'] = [[-0.1882, 0.0901], [0.1883, 0.0901], [0.1883, 0.4900], [0.1995, 0.5210], [0.191, 0.5460], [0.176, 0.5604], [-0.176, 0.5604], [-0.191, 0.5460], [-0.1995, 0.5210], [-0.1912, 0.4900], [-0.18, 0.3684]]

root_edit(A, 'representations', redraw, out=sys.argv[1] if len(sys.argv) > 1 else None)
