#!/usr/bin/env python3
"""EN 1996 review recomputation (stdlib only). Run: python3 -I en1996-recompute.py
Section A reproduces the spec worked examples X1..X12 from first principles; section B demonstrates the findings of review-en1995-en1996.md.
[V] = my reading of the printed standard from memory, not a verified quote."""
import math

ok = lambda c: "OK " if c else "BAD"


def show(name, got, want, tol=2e-3):
    rel = abs(got - want) / max(abs(want), 1e-12)
    print(f"  {ok(rel <= tol)} {name}: got {got:.6g} want {want:.6g} (rel {rel:.2e})")


print("== A. spec worked examples ==")
fb, fm, K = 20.0, 10.0, 0.55
fk = K * fb**0.7 * fm**0.3
fd = fk / 1.5
show("X1 fk", fk, 8.935); show("X1 fd", fd, 5.957); show("X1 Elt", 1000 * fk / 2.5, 3574, 2e-3)
t, h = 0.365, 2.75
hef = 0.75 * h; show("X2 hef", hef, 2.0625); show("X2 hef/t", hef / t, 5.65, 3e-3); show("X2 einit mm", hef / 450 * 1e3, 4.583)
rho3 = 0.75 / (1 + (0.75 * h / (3 * 3.0)) ** 2); rho4 = 0.75 / (1 + (0.75 * h / 3.0) ** 2)
show("X2 rho3", rho3, 0.7126); show("X2 rho4", rho4, 0.5093)
ei = 0.020 + hef / 450; phi_top = 1 - 2 * ei / t
show("X3 ei", ei, 0.024583); show("X3 Phi_top", phi_top, 0.8653); show("X3 NRd,top kN/m", phi_top * t * fd * 1e3, 1881, 3e-3)
em = 0.0125 + hef / 450; ek = 0.002 * 1.5 * (hef / t) * math.sqrt(t * em); emk = max(em + ek, 0.05 * t)
A1 = 1 - 2 * emk / t; lam = (hef / t) * math.sqrt(fk / (1000 * fk)) if False else (hef / t) * math.sqrt(8.935 / 8935)
u = (lam - 0.063) / (0.73 - 1.17 * emk / t); phim = A1 * math.exp(-u * u / 2)
show("X4 ek", ek, 1.34e-3, 1e-2); show("X4 emk (spec writes 0.01825 = 0.05 t, but em+ek = 0.01842 governs)", emk, 0.01842, 1e-3); show("X4 lambda", lam, 0.17869); show("X4 u", u, 0.17229); show("X4 Phi_m", phim, 0.8867, 3e-3)
show("X4 NRd,mid kN/m", phim * t * fd * 1e3, 1928, 3e-3)
Ab, a1, hc = 0.073, 0.5, 2.75; Aef = 1.0 * t
beta = (1 + 0.3 * a1 / hc) * (1.5 - 1.1 * Ab / Aef); cap = min(1.25 + a1 / (2 * hc), 1.5)
show("X5 beta raw", beta, 1.3498); show("X5 cap", cap, 1.3409); show("X5 NRdc kN", min(beta, cap) * Ab * fd * 1e3, 583.1, 2e-3)
fvk = 0.30 + 0.4 * 0.8; show("X6 fvk", fvk, 0.62); show("X6 fvd", fvk / 1.5, 0.4133); show("X6 VRd kN", fvk / 1.5 * 365 * 2000 / 1e3, 301.7, 2e-3)
MEd = 0.96 * 2.75**2 / 8; Z = 0.175**2 / 6; MRd = 0.2 / 1.5 * 1e3 * Z
show("X7 MEd", MEd, 0.9075); show("X7 MRd", MRd, 0.6806); show("X7 eta", MEd / MRd, 1.333, 2e-3)
show("X7 MRd with sigma 0.2", (0.2 / 1.5 + 0.2) * 1e3 * Z, 1.70, 5e-3)
z = 300 * (1 - 0.5 * 600 * 435 / (1000 * 300 * 5.957)); show("X9 z", z, 278.1); show("X9 MRd kNm", 600 * 435 * z / 1e6, 72.58, 2e-3)
hef10 = 0.75 * 2.75; lam10 = hef10 / 0.24; phis = 0.85 - 0.0011 * lam10**2; fk10 = 0.55 * 14**0.7 * 5**0.3
show("X10 lambda", lam10, 8.594, 1e-3); show("X10 Phi_s", phis, 0.7688, 1e-3); show("X10 fk", fk10, 5.654, 1e-3); show("X10 NRd kN/m", phis * 0.24 * fk10 / 1.5 * 1e3, 695.4, 2e-3)
show("X11 Nmin", 18 * 2.5 * 4 / (20 * 0.365), 24.66, 1e-3)

print("\n== B. findings ==")
print("B1 spec X7 uses f_xk1 = 0.2 N/mm2; Table 3.6 (code, normative-tables:230-235) has f_xk1 <= 0.10 for every unit material -> X7 cannot be reproduced; eta would be", round(MEd / (0.10 / 1.5 * 1e3 * Z), 3), "with 0.10")
print("B2 K for thin-layer mortar (fk_factors_en:124-134): code gives 0.80 for calcium silicate group 2 / concrete / AAC; public check EN6-33: calcium silicate G2 0.65")
for tag, Kc, Kref in (("CS G2 thin", 0.80, 0.65),):
    print(f"  {tag}: fk(fb=20) code {Kc*20**0.85:.2f} vs {Kref*20**0.85:.2f} MPa ({Kc/Kref-1:+.0%})")
print("B3 f_vk0 (f_vk0_mpa:204-214) is material independent. EN Table 3.4 [V]/public EN6-34: clay GP M10-M20 0.30; calcium silicate, concrete, AAC GP M10-M20 0.20, M2.5-M9 0.15; thin layer clay 0.30, calcium silicate 0.40")
for sigma in (0.2, 0.8):
    for m, c0, r0 in (("CS M10", 0.30, 0.20), ("CS M5", 0.20, 0.15)):
        print(f"  sigma {sigma}: {m} code fvk {c0+0.4*sigma:.2f} vs {r0+0.4*sigma:.2f} MPa ({(c0+0.4*sigma)/(r0+0.4*sigma)-1:+.0%})")
print("B4 NA-DE f_vlt (f_vlt_mpa:221): f_bt,cal = 0.026 fb for every unit; public ZAPF: 0.020 hollow, 0.026 perforated, 0.032 solid")
fb_, sig = 20.0, 0.8
for tag, fc in (("hollow", 0.020), ("perforated", 0.026), ("solid", 0.032)):
    fbt = fc * fb_
    print(f"  {tag}: f_vlt = {0.45*fbt*math.sqrt(1+sig/fbt):.3f} MPa (code always {0.45*0.026*fb_*math.sqrt(1+sig/(0.026*fb_)):.3f})")
print("  X6 in the DE annex: fvk = min(0.62, f_vlt) =", round(min(0.62, 0.45 * 0.026 * 20 * math.sqrt(1 + 0.8 / (0.026 * 20))), 3), "MPa, i.e. the spec X6 value 0.62 is the EN cap result only")
print("B5 beta in the wall check (concentrated_beta(0.0, ...), load-case-checks:360): a1 = 0 gives beta <= 1.25; X5 needs a1 = 0.5 m -> 1.3409 (+7.3 %); no a1 input exists")
print("B6 shear wall in check_wall_shear (load-case-checks:204-206) uses the full net area and sigma = N/(l t), not l_c = 3(l/2 - e) as M-20 does")
Ned, M, l, tt, f0 = 300e3, 275e3, 3.0, 0.365, 0.20e6
e = M / Ned; lc = 3 * (l / 2 - e)
v_wall = (f0 + 0.4 * Ned / (l * tt)) / 1.5 * l * tt
v_m20 = (f0 + 0.4 * Ned / (lc * tt)) / 1.5 * lc * tt
print(f"  N 300 kN, overturning 275 kNm (wind 100 kN x 2.75 m): e = {e:.3f} m, l_c = {lc:.3f} m; wall check V_Rd = {v_wall/1e3:.0f} kN, M-20 V_Rd = {v_m20/1e3:.0f} kN (wall check x{v_wall/v_m20:.2f}, unconservative)")
print("B7 rho_n for support_sides in {0,1}: rho_n() falls into the 4-sided branch -> rho = 0.75 (h_ef = 0.75 h) for a free-standing wall (EN: h_ef = 2 h)")
print("B8 yield-line collapse (numerical-models) treats X (fixed) edges with hogging = sagging capacity; unreinforced masonry has no tension reserve at a restrained edge")
print("  square panel, all edges fixed, isotropic m: yield-line w = 48 m/l^2 vs simply supported 24 m/l^2 (x2.0)")
print("B9 H-category psi1 = 0.2 in the masonry tables (psi_imposed:278 and oracle tables.json); EN 1990 Table A1.1 category H = (0, 0, 0); timber psi_factors has (0,0,0)")
print("B10 fire table: fire_mu_factor 0.9..1.0 and +10 mm (groups 2-4) / +5 mm (thin layer) are invented scalings (m90.fire is on the unverified list but drives a verdict)")
print("B11 Phi_2 (phi_2_slab_span) and e_theta (eccentricity_from_slab_rotation_m) are NA-DE rules but are applied in compression() for the EN annex too")
