"""🛢️ Oil pumpjack: the stuffing box (14 mm packing wall, 13 mm follower) and the 15 mm casing wall were the 1:50 parts at 1:100, 1:200 and 1:500; they are turned for each scale. Everything else at the coarse scales was already modelled per scale (ropes and polished rod are declared slender)."""
exec(open('rework-10-lib.py').read())
s = Source('oil-pumpjack')
s.get(100, 'rod-packing')['profile'] = [[0, 0.025], [0, 0.04], [0.04, 0.04], [0.04, 0.025], [0, 0.025]]
s.get(100, 'packing-follower')['profile'] = [[0, 0.0255], [0, 0.0405], [0.018, 0.0405], [0.018, 0.0255], [0, 0.0255]]
for scale, r, h in ((200, 0.04, 0.04), (500, 0.0375, 0.075)):
    s.get(scale, 'rod-packing')['profile'] = [[0, 0], [0, r], [h, r], [h, 0]]
    s.get(scale, 'packing-follower')['profile'] = [[0, 0], [0, r], [max(h, 0.03), r], [max(h, 0.03), 0]]
    s.get(scale, 'well-casing')['profile'] = [[0, 0], [0, 0.085], [1.3, 0.085], [1.3, 0]]
s.decide(100, "Rework 10: stuffing box packing and follower with 15 mm walls.", "Überarbeitung 10: Stopfbuchspackung und Brille mit 15 mm Wand.")
s.decide(200, "Rework 10: stuffing box as two solid 80 mm turned parts on the rod, well casing as a solid 170 mm bar (its 15 mm wall is below the 30 mm feature).", "Überarbeitung 10: Stopfbuchse als zwei volle 80-mm-Drehteile auf der Stange, Bohrlochverrohrung als voller 170-mm-Stab (ihre 15-mm-Wand liegt unter dem 30-mm-Merkmal).")
s.decide(500, "Rework 10: stuffing box as two solid 75 mm turned parts, well casing as a solid 170 mm bar.", "Überarbeitung 10: Stopfbuchse als zwei volle 75-mm-Drehteile, Bohrlochverrohrung als voller 170-mm-Stab.")
s.save()
