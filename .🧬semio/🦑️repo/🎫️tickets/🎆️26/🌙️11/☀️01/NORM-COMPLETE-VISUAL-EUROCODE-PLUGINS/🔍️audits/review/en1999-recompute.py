"""EN 1999 review recomputation (stdlib only, run with python3 -I). Section A: spec worked examples E2-01..E2-12. Section B: transcriptions of reviewed Rust code vs the intended clause."""
import math
ok_all = True
def chk(name, got, exp, tol=1e-3):
    global ok_all
    good = abs(got - exp) <= tol * max(1.0, abs(exp)); ok_all &= good
    print(f"{'OK  ' if good else 'DIFF'} {name}: got {got:.6g} expected {exp:.6g}")
E, G, g1, g2 = 70000.0, 27000.0, 1.1, 1.25
print("== A. spec worked examples ==")
chk("E2-01 fo,haz", 0.48*260, 124.8); chk("E2-01 fu,haz", 0.60*310, 186.0)
# E2-02
rho = 32/28 - 220/28**2; chk("E2-02 rho_c", rho, 0.86224, 1e-4)
A = 150**2 - 140**2; Aeff = A - 4*140*5*(1-rho); chk("E2-02 Aeff", Aeff, 2514.3, 1e-3); chk("E2-02 Nc,Rd kN", Aeff*250/g1/1e3, 571.4, 1e-3)
eps = math.sqrt(250/260); chk("E2-02 eps(260)", eps, 0.98058, 1e-4)
print("   I-section: flange beta=3.7, beta1*eps=%.2f, beta2*eps=%.2f -> class 2; web 0.4*100/6=%.2f <= 11*eps=%.2f -> class 1" % (3*eps, 4.5*eps, 0.4*100/6, 11*eps))
# E2-03
chk("E2-03 No,haz", 1000*0.48*260/g1/1e3, 113.45, 1e-3); chk("E2-03 Nu,haz", 1000*0.60*310/g2/1e3, 148.80, 1e-3)
chk("E2-03 No", 1000*260/g1/1e3, 236.36, 1e-3); chk("E2-03 Nu", 0.9*1000*310/g2/1e3, 223.2, 1e-3)
# E2-04
Iy = (80*120**3 - 74*100**3)/12; Wel = Iy/60; Wpl = 80*10*110 + 6*100**2/4
chk("E2-04 Iy", Iy, 5.3533e6, 1e-3); chk("E2-04 Wel", Wel, 89222, 1e-3); chk("E2-04 Wpl", Wpl, 103000); chk("E2-04 alpha", Wpl/Wel, 1.1544, 1e-3)
chk("E2-04 MRd kNm", Wpl*260/g1/1e6, 24.345, 1e-3)
# E2-05
chk("E2-05 VRd", 600*260/(math.sqrt(3)*g1)/1e3, 81.88, 1e-3)
# E2-06
I = (100**4 - 90**4)/12; Ncr = math.pi**2*E*I/2500**2; chk("E2-06 Ncr kN", Ncr/1e3, 316.79, 1e-3)
Ar = 100**2 - 90**2; lam = math.sqrt(Ar*250/Ncr); chk("E2-06 A", Ar, 1900); chk("E2-06 lam", lam, 1.2245, 1e-3)
def chi(lam, a, l0):
    if lam <= l0: return 1.0
    ph = 0.5*(1+a*(lam-l0)+lam**2); return min(1.0, 1/(ph+math.sqrt(ph*ph-lam*lam)))
chk("E2-06 chi", chi(lam, 0.2, 0.1), 0.5105, 1e-3); chk("E2-06 NRd", chi(lam, 0.2, 0.1)*1900*250/g1/1e3, 220.44, 1e-3)
# E2-07
Iz = 2*10*80**3/12 + 100*6**3/12; It = (2*80*10**3 + 100*6**3)/3; Iw = 10*80**3/24*(110)**2
Iw_spec = 10*80**3*(120-10)**2/24
L = 2000; Mcr = math.pi**2*E*Iz/L**2*math.sqrt(Iw_spec/Iz + L**2*G*It/(math.pi**2*E*Iz))
chk("E2-07 Iz", Iz, 855133, 1e-3); chk("E2-07 It", It, 60533, 1e-3); chk("E2-07 Iw", Iw_spec, 2.5813e9, 1e-3); chk("E2-07 Mcr kNm", Mcr/1e6, 17.53, 1e-3)
lamlt = math.sqrt(Wpl*260/Mcr); chk("E2-07 lamLT", lamlt, 1.2360, 1e-3)
chk("E2-07 chiLT", chi(lamlt, 0.10, 0.6), 0.5937, 1e-3); chk("E2-07 Mb,Rd", chi(lamlt, 0.1, 0.6)*Wpl*260/g1/1e6, 14.45, 2e-3)
# E2-08
Fv = 0.6*800*157/g2/1e3; chk("E2-08 Fv,Rd", Fv, 60.29, 1e-3)
ab = min(40/54, 60/54-0.25, 800/310, 1.0); k1 = min(2.8*30/18-1.7, 2.5); Fb = k1*ab*310*16*8/g2/1e3
chk("E2-08 alpha_b", ab, 0.7407, 1e-3); chk("E2-08 Fb,Rd", Fb, 58.79, 1e-3); chk("E2-08 Ft,Rd", 0.9*800*157/g2/1e3, 90.4, 2e-3)
# E2-09
dsD = 71*(2/5)**(1/3.4); dsL = dsD*(5e6/1e8)**(1/5.4)
chk("E2-09 dsD", dsD, 54.23, 1e-3); chk("E2-09 dsL", dsL, 31.14, 2e-3)
N1 = 2e6*(71/60)**3.4; N2 = 5e6*(dsD/40)**5.4
chk("E2-09 N1", N1, 3.545e6, 1e-3); chk("E2-09 D1", 1e6/N1, 0.2821, 1e-3); chk("E2-09 N2", N2, 2.586e7, 2e-3); chk("E2-09 D2", 2e6/N2, 0.0773, 2e-3); chk("E2-09 D", 1e6/N1+2e6/N2, 0.3594, 1e-3)
# E2-10
tg = 20+345*math.log10(8*(5/60)+1); chk("E2-10 theta_g", tg, 96.54, 1e-3)
ca = 0.41*20+903; chk("E2-10 ca", ca, 911.2, 1e-3)
hn = 25*(tg-20) + 0.3*5.67e-8*((tg+273)**4-293**4); chk("E2-10 hnet", hn, 2105.3, 1e-3)
chk("E2-10 dtheta", 200/(ca*2700)*hn*5, 0.856, 2e-3)
# E2-11
sx = 0.605*E*3/500; chk("E2-11 sigma_cr", sx, 254.1, 1e-3); chk("E2-11 lam_x", math.sqrt(200/sx), 0.8872, 1e-3)
# E2-12
Aeff12 = 960 - 480*(1-0.48); chk("E2-12 Aeff", Aeff12, 710.4); chk("E2-12 NRd", Aeff12*260/g1/1e3, 167.91, 1e-3)
Ah = 2*39.9*8; Aeff12b = 960 - Ah*0.52; chk("E2-12 T-junction Aeff", Aeff12b, 628.0, 2e-3); chk("E2-12 T NRd", Aeff12b*260/g1/1e3, 148.4, 2e-3)

print("\n== B. code transcriptions vs clause ==")
# B1 internal flange of RHS gets web eta under bending
def eta_g(psi): return 0.7+0.3*psi if psi > -1 else 0.8/(1-psi)
def cls(be, lim): return 1 if be<=lim[0] else 2 if be<=lim[1] else 3 if be<=lim[2] else 4
A_int = (11,16,22); eps250 = 1.0
for psi in (1.0, 0.0, -1.0):
    eta = eta_g(psi); beta_flange = 140/5
    print(f"B1 RHS150x150x5 bending psi={psi}: eta={eta:.2f}; code flange class {cls(eta*beta_flange/eps250, A_int)} (beta/eps={eta*beta_flange:.1f}); EN flange (uniform compression, eta=1) class {cls(beta_flange, A_int)} (28.0)")
# B2 Class 1/2 N-M exponents for I section strong axis: code (psi=2, eta=1) vs plastic interaction
a = 600*0.0+ (100*6)/(2*80*10+100*6)
for n in (0.1, 0.3, 0.5, 0.7):
    m_code = 1 - n**2
    m_true = 1.0 if n <= a else (1-n)/(1-0.5*a)
    print(f"B2 I-section n={n}: code allows M/M_Rd <= {m_code:.3f}, plastic M_N,y/M_pl = {m_true:.3f}{'  <-- code unconservative' if m_code > m_true+1e-9 else ''}")
# B3 connection HAZ plate check
t, Lw, bh, ro = 8.0, 200.0, 30.0, 0.48
print(f"B3 connection HAZ plate width: code L-2*bhaz*(1-rho) = {Lw-2*bh*(1-ro):.1f} mm vs transverse weld HAZ rho*L = {ro*Lw:.1f} mm (spec E2-03 uses rho*A): N_code={t*(Lw-2*bh*(1-ro))*260/g1/1e3:.1f} kN vs {t*ro*Lw*260/g1/1e3:.1f} kN")
# B4 6082 band by max thickness
print("B4 6082-T6 section flange 10 mm / web 4 mm: code uses band of max thickness fo=260, fu=310 for the 4 mm web whose own band is fo=250, fu=290 (fo +4 %, fu +7 %)")
# B5 deflection combination
g_, q_ = 2.0, 3.0
print(f"B5 SS beam g=2, q=3 kN/m (office psi2=0.3): w_qp ~ {g_+0.3*q_:.2f} vs w_char ~ {g_+q_:.2f} (qp understates by {(1-(g_+0.3*q_)/(g_+q_))*100:.0f} %)")
# B6 fatigue m1=0
dc=71.0; m1=1e-6
dD = dc*(0.4)**(1/m1); print(f"B6 m1->0 clamp 1e-6: Delta_sigma_D = {dD}; N(60 MPa) = 2e6*(71/60)**0 = {2e6*(dc/60)**0:.3g} cycles for any stress range (true m1=3.4: {2e6*(71/60)**3.4:.4g})")
# B7 torsional buckling constants
print("B7 torsional buckling curve: code alpha(BC) lambda0=0.20; EN 1999-1-1 Table 6.6 [V]: alpha=0.35, lambda0=0.40 for both BC")
for lam in (0.5, 1.0):
    print(f"   lam_T={lam}: chi(code BC A 0.2/0.2)={chi(lam,0.2,0.2):.4f}, chi([V] 0.35/0.40)={chi(lam,0.35,0.40):.4f}")
# B8 shell: spec check chi_x for fo=200
def alpha_x(r,t,Q): dw=math.sqrt(r/t)*t/Q; return 0.62/(1+1.91*(dw/t)**1.44)
for Q, nm in ((40,'A'),(25,'B'),(16,'C')):
    ax=alpha_x(500,3,Q); lam=0.8872; lp=math.sqrt(ax/(1-0.6)); chi_=1-0.6*((lam-0.2)/(lp-0.2)) if lam<lp else ax/lam**2
    print(f"B8 shell r/t=167 fabrication class {nm} (Q={Q}): alpha_x={ax:.3f}, chi_x(lam=0.887)={chi_:.3f} -> class B hard-coded in code")
# B9 weld filler values
print("B9 filler f_w: code 4043=190, 5356=240, 5183=270 MPa; spec A-32 gives 110-210 MPa [T] -> code exceeds the spec range by up to 29 %")
# B10 bolt: connection fu/fo by thickest band
print("B10 connection uses resolve_alloy (thickest band): 6082-T6 plate t=4 mm gets fu=310 not 290: bearing +6.9 %, block tearing +6.9 %/+4 %")
# B11 species E2-06 flexural buckling class4 effect with welds omitted, skip
print("\nALL SPEC EXAMPLES REPRODUCED" if ok_all else "\nSOME SPEC EXAMPLES DIFFER")
