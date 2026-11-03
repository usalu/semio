import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import profiles_proto as P, proto
from sectionproperties.pre.library import angle_section, channel_section, elliptical_hollow_section, i_section, mono_i_section, rectangular_hollow_section, tee_section
from shapely.geometry import Polygon
SP = {
 "ipe300": lambda: i_section(d=0.3, b=0.15, t_f=0.0107, t_w=0.0071, r=0.015, n_r=16),
 "pfc200": lambda: channel_section(d=0.2, b=0.075, t_f=0.0115, t_w=0.0085, r=0.0115, n_r=16),
 "l100x10": lambda: angle_section(d=0.1, b=0.1, t=0.01, r_r=0.012, r_t=0.006, n_r=16),
 "t150": lambda: tee_section(d=0.15, b=0.15, t_f=0.0107, t_w=0.0071, r=0.012, n_r=16),
 "plate-girder": lambda: mono_i_section(d=0.8, b_t=0.3, b_b=0.4, t_ft=0.02, t_fb=0.03, t_w=0.012, r=0.0, n_r=1),
 "rhs-200x100x8": lambda: rectangular_hollow_section(d=0.2, b=0.1, t=0.008, r_out=0.016, n_r=16),
 "ehs-300x200x10": lambda: elliptical_hollow_section(d_x=0.3, d_y=0.2, t=0.01, n=128),
}
orc = {}
for l in open(sys.argv[1]):
    k, v = l.split(); orc[k] = float(v)
for name, mk in P.LIB.items():
    o, h = mk()
    g = SP[name]().geom
    mine = Polygon(o, h)
    print(name, "area diff", abs(mine.area - g.area) / g.area, "sym diff", mine.symmetric_difference(g).area / g.area, flush=True)
    r = proto.analyse(o, h, h_factor=float(sys.argv[2]))
    rel = lambda a, b: (a - b) / abs(b)
    print("   J %+.3f%% Iw %+.3f%% avy %+.3f%% avz %+.3f%%  sc %+.2e %+.2e tris %d" % (100*rel(r['j'], orc[name+'.j']), 100*rel(r['gamma'], orc[name+'.iw']), 100*rel(r['a_s'][0], orc[name+'.avy']), 100*rel(r['a_s'][1], orc[name+'.avz']), r['sc'][0]-orc[name+'.scy'], r['sc'][1]-orc[name+'.scz'], r['tris']), flush=True)
