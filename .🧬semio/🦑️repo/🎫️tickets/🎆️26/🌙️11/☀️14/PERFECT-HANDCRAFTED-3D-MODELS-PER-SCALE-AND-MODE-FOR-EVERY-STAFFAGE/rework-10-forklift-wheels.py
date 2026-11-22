"""🛞 Forklift: the wheels of 1:100, 1:200 and 1:500 were the 1:50 wheels (stepped 10 mm rim disc, five or six hexagon nuts of 19 mm per wheel, 50 mm hub cap) at every scale. They are turned anew for each scale: 23x9-10 drive tyres (0.5915 m, 0.225 m wide), 18x7-8 steer tyres (0.455 m, 0.18 m wide)."""
exec(open('rework-10-lib.py').read())
s = Source('forklift')
WHEELS = {'f': (0.29575, 0.1125, 0.139, 0.4875, 6), 'r': (0.2276, 0.09, 0.1136, 0.47, 5)}
for axle, (R, h, bead, z, nuts) in WHEELS.items():
    for side, sign in (('l', 1), ('r', -1)):
        tag = f'{axle}-{side}'
        nut_ids = [f'nut-{tag}-{k}' for k in range(1, nuts + 1)]
        rim = s.get(100, f'rim-{tag}')
        rim['profile'] = [[0.02 if abs(a - 0.025) < 1e-9 else a, r] for a, r in rim['profile']]
        s.drop(200, nut_ids)
        s.get(200, f'tyre-{tag}')['profile'] = [[-h, bead], [-h, R - 0.03], [-h + 0.03, R], [h - 0.03, R], [h, R - 0.03], [h, bead], [-h, bead]]
        s.get(200, f'rim-{tag}')['profile'] = [[-h + 0.01, bead - 0.03], [-h + 0.01, bead + 0.004], [h - 0.01, bead + 0.004], [h - 0.01, bead - 0.03], [-h + 0.01, bead - 0.03]]
        s.get(200, f'hub-{tag}')['profile'] = [[0, 0], [0, bead - 0.026], [0.035, bead - 0.026], [0.035, 0.06], [0.07, 0.045], [0.07, 0]]
        s.drop(500, nut_ids + [f'hub-{tag}'])
        s.get(500, f'tyre-{tag}')['profile'] = [[-h, bead], [-h, R - 0.04], [-h + 0.04, R], [h - 0.04, R], [h, R - 0.04], [h, bead], [-h, bead]]
        s.get(500, f'rim-{tag}')['profile'] = [[-h + 0.015, 0], [-h + 0.015, bead + 0.004], [h - 0.015, bead + 0.004], [h - 0.015, 0]]
s.decide(100, "Rework 10: the rim disc is 15 mm at its thinnest for this scale; the wheel nuts (19 mm across the flats) stay.", "Überarbeitung 10: die Radscheibe ist für diesen Maßstab an der dünnsten Stelle 15 mm; die Radmuttern (Schlüsselweite 19 mm) bleiben.")
s.decide(200, "Rework 10: wheels turned anew for the 30 mm feature: tyre with one 30 mm shoulder chamfer, rim as a 34 mm barrel, wheel disc and hub as one 35 mm dish with its boss, no nuts.", "Überarbeitung 10: Räder für das 30-mm-Merkmal neu gedreht: Reifen mit einer 30-mm-Schulterfase, Felge als 34-mm-Ring, Radscheibe und Nabe als eine 35-mm-Schüssel mit Nabenbund, keine Muttern.")
s.decide(500, "Rework 10: wheels turned anew for the 75 mm feature: a tyre ring with one 40 mm shoulder chamfer on a solid wheel disc, no hub cap, no nuts.", "Überarbeitung 10: Räder für das 75-mm-Merkmal neu gedreht: ein Reifenring mit einer 40-mm-Schulterfase auf einer vollen Radscheibe, ohne Nabendeckel und Muttern.")
s.save()
