import sys, time, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cases import CASES
import proto
from sp_ref import reference

hf = float(sys.argv[1]) if len(sys.argv) > 1 else 4.0
only = sys.argv[2:] 
for name, (outer, holes) in CASES.items():
    if only and name not in only:
        continue
    t = time.time()
    m = proto.analyse(outer, holes, h_factor=hf)
    r = reference(outer, holes)
    def rel(a, b):
        return (a - b) / abs(b) if abs(b) > 1e-30 else 0.0
    print(f"{name}: tris={m['tris']} ({time.time()-t:.1f}s) J {rel(m['j'], r['j']):+.4%} gamma {rel(m['gamma'], r['gamma']):+.4%} As_x {rel(m['a_s'][0], r['a_s'][0]):+.4%} As_y {rel(m['a_s'][1], r['a_s'][1]):+.4%} sc_dx {m['sc'][0]-r['sc'][0]:+.2e} sc_dy {m['sc'][1]-r['sc'][1]:+.2e} Ixx {rel(m['ic'][0], r['ic'][0]):+.1e} ", flush=True)
