import re
p = r'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️C1\venetian_blind.py'
s = open(p, encoding='utf8').read()
s = s.replace('*slats_lines(0.1),', '*slats(0.02, 0.1),').replace('*slats_lines(0.13),', '*slats(0.013, 0.13),').replace('*slats_rects(),', '*slats(0.012, 0.216),')
pens = iter([0.18, 0.13, 0.296])
s = re.sub(r"        rect\('bottom-rail', BR,[^\n]*\n", lambda m: '        rail(%s),\n' % next(pens), s)
extra = iter(['*slats(0.02, 0.1, 3), rail(0.18, 3.5),', '*slats(0.013, 0.13, 3), rail(0.13, 3.5),', '*slats(0.012, 0.216, 3), rail(0.296, 3.5),'])
s = re.sub(r"    'plan': \[\n", lambda m: "    'plan': [\n        %s\n" % next(extra), s)
s = s.replace('with the lamellae hidden below the box in every pose (they hang within the 0.15 m depth of the box)', 'with the lamellae and the bottom rail drawn as extruded solids below the box, where they stay hidden in every pose (they hang within the 0.15 m depth of the box)')
s = s.replace('die Lamellen liegen in jeder Pose verdeckt unter dem Kasten (sie hängen innerhalb der 0,15 m Tiefe des Kastens)', 'die Lamellen und die Unterschiene sind als extrudierte Körper unter dem Kasten gezeichnet und bleiben in jeder Pose verdeckt (sie hängen innerhalb der 0,15 m Tiefe des Kastens)')
s = s.replace('17 lamella lines', '17 extruded lamella bars (the tilt reads in the front view)').replace('17 Lamellenlinien', '17 extrudierte Lamellenbalken (die Kippung liest sich in der Vorderansicht)')
s = s.replace('17 lamella bars of 0.012 m', '17 extruded lamella bars of 0.012 m thickness (their apparent height grows with the tilt up to the 0.09 m chord)').replace('17 Lamellenbalken von 0,012 m', '17 extrudierte Lamellenbalken von 0,012 m Dicke (ihre scheinbare Höhe wächst mit der Kippung bis zur Sehne von 0,09 m)')
open(p, 'w', encoding='utf8').write(s)
