import math, warnings
warnings.filterwarnings("ignore")
from sectionproperties.pre.library import i_section, channel_section, mono_i_section
from sectionproperties.analysis import Section
import numpy as np
from shapely.geometry import Polygon

E, G = 70e9, 27e9

def solve(geom, n):
    geom.create_mesh(mesh_sizes=[n])
    s = Section(geom)
    s.calculate_geometric_properties(); s.calculate_warping_properties()
    return s

def cubic(geom):
    poly = geom.geom
    cx, cy = poly.centroid.x, poly.centroid.y
    def tri_int(f, a, b):
        t = abs(a[0]*b[1]-b[0]*a[1])/2
        pts = [(.6,.2,.2),(.2,.6,.2),(.2,.2,.6)]
        mid = f((a[0]+b[0])/3,(a[1]+b[1])/3)
        return t*(-27/48*mid+25/48*sum(f(w[1]*a[0]+w[2]*b[0], w[1]*a[1]+w[2]*b[1]) for w in pts))
    tot = 0
    for ring, w in [(poly.exterior, 1)] + [(r, -1) for r in poly.interiors]:
        c = [(x-cx, y-cy) for x, y in ring.coords][:-1]
        sg = 1 if Polygon(c).exterior.is_ccw else -1
        for i in range(len(c)):
            a, b = c[i], c[(i+1) % len(c)]
            sgn = np.sign(a[0]*b[1]-b[0]*a[1])
            tot += w*sg*sgn*(1 if True else 1)*tri_int(lambda y, z: z*(y*y+z*z), a, b)
    return tot

def mcr(c1, iz, iw, it, L, zj):
    x = -zj
    return c1*math.pi**2*E*iz/L**2*(math.sqrt(iw/iz+L**2*G*it/(math.pi**2*E*iz)+x*x)-x)

def mcr_energy(iz, iw, it, L, zj):
    k = math.pi/L
    a, b = 1.0, -2*E*iz*zj*k**2
    c = -E*iz*(E*iw*k**4+G*it*k**2)
    return (-b+math.sqrt(b*b-4*a*c))/2

def report(name, geom, n, L):
    s = solve(geom, n)
    c = s.get_c(); sc = s.get_sc()
    ic = s.get_ic()
    iz_ = ic[1]
    iyy = ic[0]
    j = s.get_j(); gam = s.get_gamma()
    area = s.get_area()
    zs = sc[1]
    cub = cubic(geom)
    zj = (zs - c[1]) - 0.5*cub/iyy
    print(name, dict(A=area, Iy=iyy, Iz=iz_, J=j, Iw=gam, cy=c[1], cx=c[0], scx=sc[0]-c[0], scz=sc[1]-c[1], zj=zj))
    print("  Mcr L=%g: closed=%.6g energy=%.6g sym=%.6g" % (L, mcr(1, iz_, gam, j, L, zj), mcr_energy(iz_, gam, j, L, zj), mcr(1, iz_, gam, j, L, 0)))
    return dict(A=area, Iy=iyy, Iz=iz_, J=j, Iw=gam, scx=sc[0]-c[0], zj=zj)

report("E2-07 I 120x80x10/6", i_section(d=0.12, b=0.08, t_f=0.01, t_w=0.006, r=0, n_r=1), 1e-6, 2.0)
report("I 200x100x10/6", i_section(d=0.2, b=0.1, t_f=0.01, t_w=0.006, r=0, n_r=1), 1e-6, 4.0)
report("channel 100x50x7/5", channel_section(d=0.1, b=0.05, t_f=0.007, t_w=0.005, r=0, n_r=1), 1e-6, 3.0)
report("mono-I 200 top 100x12 bot 60x8 tw6", mono_i_section(d=0.2, b_t=0.1, b_b=0.06, t_ft=0.012, t_fb=0.008, t_w=0.006, r=0, n_r=1), 1e-6, 4.0)
