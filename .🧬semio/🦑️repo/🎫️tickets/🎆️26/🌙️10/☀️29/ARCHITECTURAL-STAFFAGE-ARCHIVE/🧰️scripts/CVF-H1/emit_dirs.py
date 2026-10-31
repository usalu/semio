import sys, math
sys.path.insert(0, '/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-H1/h1')
from lib import *
for aid in sys.argv[1:]:
    a = load(aid)['asset']
    print('==', aid)
    em = {e['id']: e for e in a['thermal']['emitters'] if e['kind'] == 'breath'}
    for p in a['poses']:
        W = fk(a, p)
        m = W['head']
        row = []
        for eid, e in em.items():
            th = math.radians(e['directionDeg'])
            dx, dy = math.cos(th), math.sin(th)
            wx, wy = m[0]*dx + m[2]*dy, m[1]*dx + m[3]*dy
            row.append(f"{eid}:({wx:+.2f},{wy:+.2f})")
        # facing sign: face dir local -y -> world
        fx, fy = m[2]*-1, m[3]*-1
        print(f"  {p['id']:22s} face({fx:+.2f},{fy:+.2f}) ", ' '.join(row))
