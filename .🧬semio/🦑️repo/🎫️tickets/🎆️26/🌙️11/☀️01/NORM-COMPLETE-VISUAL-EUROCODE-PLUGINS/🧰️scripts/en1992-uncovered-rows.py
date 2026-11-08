"""Independent expected values for the nine EN 1992 ids that had no test (M-03, M-09, M-14, D-02, P-01, PR-01, PR-02)."""
import math
import numpy as np
from shapely.geometry import box, Point
from structuralcodes.codes.ec2_2004 import fctm as ec2_fctm

def cube(fck):
    xs = [12,16,20,25,30,35,40,45,50,55,60,70,80,90]
    ys = [15,20,25,30,37,45,50,55,60,67,75,85,95,105]
    return float(np.interp(fck, xs, ys))

rows = []
rows.append(("m-03-cube-30", "M-03", "De", "fck=30", "fck_cube", cube(30), 1e-6))
rows.append(("m-03-cube-interp", "M-03", "De", "fck=32", "fck_cube", cube(32), 1e-6))
g = 1.15
def sigma_p(fp01k, fpk, eps, ep=195000.0, eps_ud=20.0):
    fpd = fp01k / g
    if eps / 1000 * ep <= fpd:
        return ep * eps / 1000
    return fpd + (fpk / g - fpd) * (eps / 1000 - fpd / ep) / (eps_ud / 1000 - fpd / ep)
rows.append(("m-09-elastic", "M-09", "De", "fp01k=1500;fpk=1700;eps=5", "sigma_p", sigma_p(1500, 1700, 5), 1e-6))
rows.append(("m-09-hardening", "M-09", "De", "fp01k=1500;fpk=1700;eps=15", "sigma_p", sigma_p(1500, 1700, 15), 1e-6))
rows.append(("m-09-fpd", "M-09", "De", "fp01k=1500;fpk=1700;eps=5", "fpd", 1500 / g, 1e-6))
b, h, fck = 300.0, 600.0, 30.0
fl = max((1.6 - h / 1000) * ec2_fctm(fck), ec2_fctm(fck))
rows.append(("m-14-mcr", "M-14", "De", f"fck=30;b={b:g};h={h:g}", "mcr", fl * b * h * h / 6, 5e-3))
p, e = 500000.0, 150.0
rows.append(("m-14-mcr-p", "M-14", "De", f"fck=30;b={b:g};h={h:g};p={p:g};e_p={e:g}", "mcr_p", fl * b * h * h / 6 + p * (e + b * h * h / 6 / (b * h)), 5e-3))
need = max(16, 25)
rows.append(("d-02-pass", "D-02", "De", "fck=30;classes:XC1,XC4", "fck_min", 25.0, 1e-6))
rows.append(("d-02-frost-air", "D-02", "De", "fck=30;classes:XF2,XC1;air_entrained=1", "fck_min", 25.0, 1e-6))
rows.append(("d-02-frost-noair", "D-02", "De", "fck=30;classes:XF2,XC1", "fck_min", 35.0, 1e-6))
c1, c2, d = 400.0, 400.0, 210.0
rect = box(-c1/2, -c2/2, c1/2, c2/2)
u0 = rect.length
u1 = rect.buffer(2*d, resolution=2048).length
rows.append(("p-01-u0", "P-01", "De", f"c1={c1:g};c2={c2:g};d={d:g}", "u0", u0, 1e-6))
rows.append(("p-01-u1", "P-01", "De", f"c1={c1:g};c2={c2:g};d={d:g}", "u1", u1, 5e-5))
cd = Point(0, 0).buffer(250 + 2*d, resolution=2048).length
rows.append(("p-01-u1-circ", "P-01", "De", f"c1=500;d={d:g};circular=1", "u1", cd, 5e-5))
rows.append(("p-01-kd", "P-01", "De", f"c1={c1:g};c2={c2:g};d={d:g}", "uout_dist", 1.5 * d, 1e-6))
fpk, fp01k, ap = 1860.0, 1640.0, 1500.0
smax = min(0.8*fpk, 0.9*fp01k)
rows.append(("pr-01-smax", "PR-01", "De", f"ap={ap:g};fpk={fpk:g};fp01k={fp01k:g}", "sigma_p_max", smax, 1e-6))
rows.append(("pr-01-pmax", "PR-01", "De", f"ap={ap:g};fpk={fpk:g};fp01k={fp01k:g}", "p_max", ap*smax, 1e-6))
lim = min(0.75*fpk, 0.85*fp01k)
rows.append(("pr-02-lim", "PR-02", "De", f"ap={ap:g};fpk={fpk:g};fp01k={fp01k:g};pm0=1.8e6", "sigma_pm0_max", lim, 1e-6))
rows.append(("pr-02-util", "PR-02", "De", f"ap={ap:g};fpk={fpk:g};fp01k={fp01k:g};pm0=1.8e6", "util", 1.8e6/ap/lim, 1e-6))
for r in rows:
    print("      | %-18s | %-5s | %-5s | %-50s | %-13s | %-14.9g | %-9g |" % (r[0], r[1], r[2], r[3], r[4], r[5], r[6]))
