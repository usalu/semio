#!/usr/bin/env python3
"""EN 1995 review recomputation (stdlib only). Run: python3 -I en1995-recompute.py
Section A reproduces the spec worked examples E1..E11 from first principles; section B demonstrates the findings of review-en1995-en1996.md.
Values tagged [V] are my reading of the printed standard from memory, not a verified quote."""
import math

PI = math.pi
ok = lambda c: "OK " if c else "BAD"


def show(name, got, want, tol=2e-3):
    rel = abs(got - want) / max(abs(want), 1e-12)
    print(f"  {ok(rel <= tol)} {name}: got {got:.6g} want {want:.6g} (rel {rel:.2e})")


print("== A. spec worked examples ==")
# E1 GL28h 200x400 L=5 m, DE
qd = 1.35 * 2.5 + 1.5 * 3.0
Md = qd * 5.0**2 / 8
W = 200 * 400**2 / 6
sm = Md * 1e6 / W
kh = min((600 / 400) ** 0.1, 1.1)
fmd = 0.8 * kh * 28 / 1.25
show("E1 qd", qd, 7.875); show("E1 Md kNm", Md, 24.609); show("E1 sigma_m", sm, 4.614); show("E1 kh", kh, 1.0414); show("E1 fmd", fmd, 18.66)
Vd = qd * 5 / 2
tau = 1.5 * Vd * 1e3 / ((2.5 / 3.5) * 200 * 400)
show("E1 tau", tau, 0.5168); show("E1 fvd", 0.8 * 3.5 / 1.25, 2.24)
I = 200 * 400**3 / 12
wg = 5 * 2.5 * 5000**4 / (384 * 12600 * I); wq = 5 * 3.0 * 5000**4 / (384 * 12600 * I)
show("E1 winst,G", wg, 1.514); show("E1 winst,Q", wq, 1.817); show("E1 wfin (psi2=0)", wg * 1.6 + wq, 4.239)
# E2 column
i = 100 / math.sqrt(12); lam = 3000 / i; lrel = lam / PI * math.sqrt(21 / 7400)
k = 0.5 * (1 + 0.2 * (lrel - 0.3) + lrel**2); kc = 1 / (k + math.sqrt(k * k - lrel**2))
show("E2 lrel", lrel, 1.7622); show("E2 kc", kc, 0.28457); show("E2 NRd kN", kc * 0.8 * 21 / 1.3 * 10000 / 1e3, 36.77)
# E3
show("E3 solid", 1.25 * 0.8 * 2.5 / 1.3, 1.923); show("E3 glulam", 1.5 * 0.8 * 3.0 / 1.25, 2.880)
# E4
sc = 0.78 * 120**2 * 10500 / (360 * 8000); lr = math.sqrt(28 / sc)
show("E4 sigma_crit", sc, 40.95); show("E4 lrel", lr, 0.8269); show("E4 kcrit", 1.56 - 0.75 * lr, 0.9398)
# E5 notch
a = 200 / 240; kn = 5
kv = kn / (math.sqrt(240) * (math.sqrt(a * (1 - a)) + 0.8 * (100 / 240) * math.sqrt(1 / a - a * a)))
show("E5 kv", kv, 0.5294); show("E5 eta", 1.5 * 10000 / (120 * 200) / (kv * 0.8 * 4.0 / 1.3), 0.4796)
# E6 Johansen
fh = 0.082 * (1 - 0.12) * 350; My = 0.3 * 400 * 12**2.6; t = 60; d = 12; b = 1.0
c = fh * t * d / (1 + b) * (math.sqrt(b + 2 * b * b * (1 + 1 + 1) + b**3) - b * 2)
dm = 1.05 * fh * t * d / (2 + b) * (math.sqrt(2 * b * (1 + b) + 4 * b * (2 + b) * My / (fh * d * t * t)) - b)
f_ = 1.15 * math.sqrt(2 * b / (1 + b)) * math.sqrt(2 * My * fh * d)
show("E6 fh", fh, 25.256); show("E6 My", My, 76745, 5e-3); show("E6 a", fh * t * d, 18184); show("E6 c", c, 7532); show("E6 d", dm, 7643, 5e-3); show("E6 f", f_, 7844)
show("E6 FvRd kN", 0.8 * c / 1.3 / 1e3, 4.635)
thick_d = fh * t * d * (math.sqrt(2 + 4 * My / (fh * d * t * t)) - 1); thick_e = 2.3 * math.sqrt(My * fh * d)
show("E6 steel d", thick_d, 9282); show("E6 steel e", thick_e, 11092); show("E6 steel FvRd kN", 0.8 * thick_d / 1.3 / 1e3, 5.712)
# E7
nef = lambda n, a1, dd: min(n, n**0.9 * (a1 / (13 * dd)) ** 0.25)
show("E7 nef a1=90", nef(4, 90, 12), 3.035); show("E7 nef a1=60", nef(4, 60, 12), 2.742); show("E7 nef 8 in row", nef(8, 120, 12), 6.085)
# E8
fax = 0.52 * 8**-0.5 * 80**-0.1 * 350**0.8
show("E8 fax,k", fax, 12.865); show("E8 FaxRk kN", fax * 8 * 80 / 1e3, 8.233); show("E8 FaxRd kN", 0.8 * fax * 8 * 80 / 1.3 / 1e3, 5.067)
# E9 fire
for tmin, bexp, hexp in ((60, 62, 271), (30, 104, 292)):
    dch = 0.7 * tmin; def_ = dch + min(tmin / 20, 1) * 7
    bb, hh = 160 - 2 * def_, 320 - def_
    print(f"  E9 R{tmin}: def {def_:.1f} b {bb:.0f} h {hh:.0f}  ({ok(abs(bb-bexp)<1 and abs(hh-hexp)<1)})")
bb, hh = 62.0, 271.0; Wf = bb * hh * hh / 6; khf = (600 / 271) ** 0.1
show("E9 W", Wf, 758890, 1e-3); show("E9 Mfi,Rd with kh kNm", Wf * 1.15 * 28 * khf / 1e6, 26.46, 3e-3)
print(f"  NOTE code (fire_utilisation/assess_fire) omits kh: M_fi,Rd = {Wf*1.15*28/1e6:.2f} kNm ({Wf*1.15*28/1e6/26.46-1:+.1%} vs spec)")
# E10 gamma method
E = 11000; Kser = 350**1.5 * 3.1**0.8 / 30; A = 10000; I1 = 200 * 50**3 / 12
g = 1 / (1 + PI**2 * E * A * 150 / (Kser * 3000**2)); EIef = 3 * E * I1 + 2 * g * E * A * 50**2
show("E10 Kser", Kser, 539.6, 2e-3); show("E10 gamma", g, 0.02896, 3e-3); show("E10 EIef kNm2", EIef / 1e9, 84.68, 3e-3)
print(f"  NOTE spec E10 uses rho_m = 350 (= C24 rho_k). Table 7.1 needs the MEAN density 420 -> Kser {420**1.5*3.1**0.8/30:.1f} N/mm, gamma {1/(1+PI**2*E*A*150/(420**1.5*3.1**0.8/30*3000**2)):.4f}")
# E11 vibration: the completion report claims the spec is a factor 10 too small. It is not.
EI_l = E * (80 * 220**3 / 12) / 625.0  # N mm^2 per mm of width? no: per (spacing 625 mm)
EI_l_Nm2_per_m = E * (80 * 220**3 / 12) * 1e-6 / 0.625  # N mm2 -> N m2, per metre width
f1 = PI / (2 * 4.5**2) * math.sqrt(EI_l_Nm2_per_m / 150)
show("E11 (EI)l N m2/m  [spec value is RIGHT]", EI_l_Nm2_per_m, 1.249e6, 2e-3); show("E11 f1 Hz", f1, 7.08, 3e-3)
print("  en1995-completion.md claims E11 is a factor 10 too small (correct 1.25e7, f1 ~ 22 Hz). That is wrong: E*I = 11000*80*220^3/12 = 7.81e11 N mm^2 = 7.81e5 N m^2, divided by 0.625 m = 1.249e6 N m^2/m, f1 = 7.08 Hz as in the spec.")
print("\n== B. findings ==")
print("B1 EN 338:2016 ft,0,k / fc,0,k ([V] from memory) versus the code table (strength-tables:58-69)")
ref = {"C14": (7.2, 16), "C16": (8.5, 17), "C18": (10, 18), "C20": (11.5, 19), "C22": (13, 20), "C24": (14.5, 21), "C27": (16.5, 22),
       "C30": (19, 24), "C35": (22.5, 25), "C40": (26, 27), "C45": (30, 29), "C50": (33, 30)}
code = {"C14": (8, 16), "C16": (10, 17), "C18": (11, 18), "C20": (12, 19), "C22": (13, 20), "C24": (14.5, 21), "C27": (16.5, 22),
        "C30": (18, 23), "C35": (21, 25), "C40": (24, 26), "C45": (27, 27), "C50": (30, 29)}
for k_, (ft, fc) in ref.items():
    cft, cfc = code[k_]
    flag = "" if (ft, fc) == (cft, cfc) else f"  <-- ft0 {cft} vs {ft} ({cft/ft-1:+.0%}), fc0 {cfc} vs {fc}"
    print(f"  {k_}: ref ft0 {ft} fc0 {fc}{flag}")
print("B2 EN 14080:2013 homogeneous glulam ([V]): ft,0,g,k = 0.8 fm, fc,0,g,k = fm, ft,90 = 0.5, fc,90 = 2.5, G = 650")
for n, fm_ in (("GL20h", 20), ("GL22h", 22), ("GL24h", 24), ("GL26h", 26), ("GL28h", 28), ("GL30h", 30), ("GL32h", 32)):
    print(f"  {n}: ft0 {0.8*fm_:.1f} (22.3 for GL28h) fc0 {fm_} ft90 0.5 fc90 2.5")
print("  code GL24h/28h/32h: ft0 16.5/19.5/22.5 (EN 1194), fc90 2.7/3.0/3.3 (EN 1194) -> fc90 is unconservative by +8/+20/+32 %")
print("B3 ContinuousTwoSpan internals (load-combinations:29-31) versus exact two-span beam")
q, L, F = 10.0, 5.0, 10.0
print(f"  UDL: code M = qL^2/14 = {q*L*L/14:.3f}, exact hogging qL^2/8 = {q*L*L/8:.3f} (x{14/8:.2f}); code V = 0.6 qL = {0.6*q*L:.2f}, exact 0.625 qL = {0.625*q*L:.2f}")
print(f"  Point at midspan of both spans: code M = FL/6 = {F*L/6:.3f}, exact hogging 3FL/16 = {3*F*L/16:.3f}, sagging 13FL/64 = {13*F*L/64:.3f}; code V = 0.6 F, exact 11F/16 = {11*F/16:.3f}")
print("B4 missing permanent-only ULS combination when variable actions exist (enumerate_combos:171-204)")
G, Q = 5.0, 1.0
kperm, kmed = 0.6, 0.8
print(f"  G=5 Q=1 (kN/m): G-only 1.35G/kmod,perm = {1.35*G/kperm:.2f};  G+Q (the only combination built) (1.35G+1.5Q)/kmod,med = {(1.35*G+1.5*Q)/kmed:.2f}  -> G-only governs by {(1.35*G/kperm)/((1.35*G+1.5*Q)/kmed)-1:+.1%} and is never checked")
print("B5 K_ser (kser_value:405,407) versus Table 7.1 [V]: staples rho^1.5 d^0.8/80, toothed plate C1-C9 1.5 rho^1.5 dc/4")
print(f"  staple d=1.8 rho=420: code /30 = {420**1.5*1.8**0.8/30:.0f}, EN /80 = {420**1.5*1.8**0.8/80:.0f} N/mm (x{80/30:.2f})")
print("B6 n_ef for bolts/dowels at load angle alpha (n_ef_value:241): EN 8.5.1.1(?) linear interpolation in alpha, code uses sin(alpha)")
par = nef(4, 90, 12)
for al in (15, 30, 45, 60):
    print(f"  alpha {al}: code {par+abs(math.sin(math.radians(al)))*(4-par):.3f}  linear-in-angle {par+al/90*(4-par):.3f}")
print("B7 predrilled nails n_ef (n_ef_value:231) returns n for any a1; Table 8.1 predrilled column is 0.7/0.85/1.0/1.0 at 4d/7d/10d/14d [V]")
print(f"  n=10, a1=4d: code {10.0:.2f}, [V] {10**0.7:.2f}")
print("B8 notch k_v for h_ef <= 0 or x < 0 is NaN -> Rust f64::min(NaN, 1.0) = 1.0 (notch_kv:112); alpha=-0.1 gives sqrt(alpha(1-alpha)) = sqrt(", -0.1*1.1, ")")
print("B9 member deflection ignores shear (deflection_m:378): E1 beam")
ws = 1.2 * (1.0) * 5000**2 / (8 * 650 * 80000)  # kappa q L^2/(8 G A) per N/mm
wb = 5 * 1.0 * 5000**4 / (384 * 12600 * I)
print(f"  per 1 N/mm: bending {wb:.3f} mm, shear {ws:.3f} mm ({ws/wb:+.1%}); L/h = 12.5")
print("B10 combined compression + LTB (6.35) is absent in assess_member:273-279: sigma_m uses kh*fm without kcrit")
print("B11 stability in fire absent (fire_utilisation:94-95): (sc/(kfi fc))^2 without kc,fi")
print("B12 default kmod = 1.0 in T-76/T-80/T-81/T-82 (c.o(\"kmod\", 1.0)): permanent loading needs 0.6")
