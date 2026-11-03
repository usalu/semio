"""EN 1998 review recomputation (stdlib only, run with python3 -I). Section A: spec worked examples. Section B: faithful transcriptions of the Rust code under review, compared to the correct clause formula."""
import math
ok_all = True
def chk(name, got, exp, tol=1e-3):
    global ok_all
    good = abs(got - exp) <= tol * max(1.0, abs(exp))
    ok_all &= good
    print(f"{'OK  ' if good else 'DIFF'} {name}: got {got:.6g} expected {exp:.6g}")

g = 9.81
def se(ag, S, TB, TC, TD, T, eta=1.0):
    if T <= TB: return ag*S*(1+T/TB*(2.5*eta-1))
    if T <= TC: return ag*S*2.5*eta
    if T <= TD: return ag*S*2.5*eta*TC/T
    return ag*S*2.5*eta*TC*TD/T**2
def sd(ag, S, TB, TC, TD, q, T, beta=0.2):
    if T <= TB: return ag*S*(2/3+T/TB*(2.5/q-2/3))
    if T <= TC: return ag*S*2.5/q
    if T <= TD: return max(ag*S*2.5/q*TC/T, beta*ag)
    return max(ag*S*2.5/q*TC*TD/T**2, beta*ag)
print("== A. spec worked examples (site: zone 2, C-R) ==")
ag, S, TB, TC, TD = 0.6, 1.5, 0.05, 0.30, 2.0
for T, e in [(0, 0.9), (0.05, 2.25), (0.3, 2.25), (0.5, 1.35), (1.0, 0.675), (2.0, 0.3375), (3.0, 0.15)]:
    chk(f"EX98-01 Se({T})", se(ag, S, TB, TC, TD, T), e)
ag2 = 1.2*0.25*g
chk("EX98-02 ag", ag2, 2.943, 1e-3); chk("EX98-02 plateau", ag2*1.15*2.5, 8.461, 1e-3)
chk("EX98-02 vs30", 30/(5/150+10/250+15/400), 270.68, 1e-3)
chk("EX98-03 q0", 3.0*1.3, 3.9); chk("EX98-03 irregular", 0.8*3.9, 3.12); chk("EX98-03 kw a0=1", (1+1)/3, 0.6667, 1e-3)
q = 3.9
chk("EX98-04 plateau", sd(ag,S,TB,TC,TD,q,0.2), 0.5769); chk("EX98-04 Sd(0.5)", sd(ag,S,TB,TC,TD,q,0.5), 0.3462)
chk("EX98-04 Sd(3)", sd(ag,S,TB,TC,TD,q,3.0), 0.12)
chk("EX98-05 mass", (3000+0.24*1000)/g, 330.28, 1e-3)
T1 = 0.075*12**0.75; chk("EX98-06 T1", T1, 0.4836, 1e-3)
Sd1 = sd(ag,S,TB,TC,TD,q,T1); chk("EX98-06 Sd(T1)", Sd1, 0.3579, 1e-3)
Fb = Sd1*1600*0.85; chk("EX98-06 Fb", Fb, 486.78, 1e-3)
z = [3,6,9,12]; F = [Fb*zi/sum(z) for zi in z]; print("  Fi", [round(f,2) for f in F])
V = [sum(F[i:]) for i in range(4)]; print("  Vi", [round(v,2) for v in V])
chk("EX98-06 V4", V[3], 194.71, 1e-3)
dr = q*1.2e-3; P = 1600*g; th = P*dr/(Fb*3)
chk("EX98-07 theta", th, 0.0503, 2e-3); chk("EX98-07 drift ratio", dr*0.5/(0.005*3), 0.156, 2e-3)
dr2 = q*4e-3; th2 = P*dr2/(Fb*3); chk("EX98-07 fail theta", th2, 0.1677, 2e-3); chk("EX98-07 1/(1-th)", 1/(1-th2), 1.2015, 1e-3)
chk("EX98-08 delta", 1+0.6*12/24, 1.30); chk("EX98-08 eai", 0.05*24, 1.2)
# EX98-09 two-DOF
m = 100e3; k = 40e6
w2 = [(3-math.sqrt(5))/2*k/m, (3+math.sqrt(5))/2*k/m]
w = [math.sqrt(x) for x in w2]; T = [2*math.pi/x for x in w]
chk("EX98-09 w1", w[0], 12.3607, 1e-4); chk("EX98-09 w2", w[1], 32.3607, 1e-4)
chk("EX98-09 T1", T[0], 0.50832, 1e-4); chk("EX98-09 T2", T[1], 0.19416, 1e-4)
phi1 = (0.618034, 1.0); phi2 = (-1.618034, 1.0)
def gam(ph): return sum(ph)/sum(p*p for p in ph)
def meff(ph): return sum(ph)**2/sum(p*p for p in ph)*m
chk("EX98-09 Gamma1", gam(phi1), 1.17082, 1e-4)
chk("EX98-09 Meff1 %", meff(phi1)/(2*m)*100, 94.72, 1e-3); chk("EX98-09 Meff2 %", meff(phi2)/(2*m)*100, 5.28, 2e-3)
sd1 = sd(ag,S,TB,TC,TD,2.0,T[0]); sd2 = sd(ag,S,TB,TC,TD,2.0,T[1])
chk("EX98-09 Sd1", sd1, 0.6640, 1e-3); chk("EX98-09 Sd2", sd2, 1.125, 1e-3)
Vb1 = sd1*meff(phi1)/1e3; Vb2 = sd2*meff(phi2)/1e3
chk("EX98-09 Vb1", Vb1, 125.78, 1e-3); chk("EX98-09 Vb2", Vb2, 11.88, 2e-3); chk("EX98-09 SRSS", math.hypot(Vb1,Vb2), 126.34, 1e-3)
chk("EX98-09 F modal1 (1st storey)", gam(phi1)*sd1*m*phi1[0]/1e3, 48.04, 2e-3); chk("EX98-09 F modal1 (2nd)", gam(phi1)*sd1*m*phi1[1]/1e3, 77.74, 2e-3)
def rho(xi, r): return 8*xi*xi*(1+r)*r**1.5/((1-r*r)**2+4*xi*xi*r*(1+r)**2)
r = w[1]/w[0]; chk("EX98-09 r", r, 2.618, 1e-3); chk("EX98-09 rho12", rho(0.05, r), 0.0088, 2e-2)
chk("EX98-10 ratio", 800/(1.3*600), 1.0256, 1e-3); chk("EX98-10 fail", 760/780, 0.974, 2e-3)
chk("EX98-10 beam V", (300+200)/5+60, 160); chk("EX98-10 col V", 1.1*800/2.6, 338.46, 1e-3)
phiy, Lv, a_v, z_, h, db, fy, fc = 0.0060, 1.5, 1, 0.414, 0.5, 0.020, 500, 25
thy = phiy*(Lv+a_v*z_)/3 + 0.0013*(1+1.5*h/Lv) + 0.13*phiy*db*fy/math.sqrt(fc)
chk("EX98-11 theta_y", thy, 0.007338, 1e-3)
Rt, H = 10, 8
chk("EX98-12 tanh", math.tanh(1.84*0.8), 0.89996, 1e-4)
wc = math.sqrt(1.84*g/Rt*math.tanh(1.84*H/Rt)); chk("EX98-12 wc", wc, 1.2745, 1e-3); chk("EX98-12 Tc", 2*math.pi/wc, 4.930, 1e-3)
chk("EX98-12 mc/m", 0.230*(Rt/H)*math.tanh(1.84*H/Rt), 0.2587, 1e-3)
eta_c = math.sqrt(10/5.5); chk("EX98-12 eta_c", eta_c, 1.3484, 1e-3)
chk("EX98-12 Se(Tc)", se(0.6,1.5,0.05,0.30,2.0,2*math.pi/wc,eta_c), 0.0749, 2e-2)
def kae(phi, kh, psi=0, beta=0, dl=0, kv=0):
    th = math.atan(kh/(1+kv)); p = math.radians(phi); ps = math.radians(psi); b = math.radians(beta); d = math.radians(dl)
    num = math.cos(p-th-ps)**2
    root = math.sqrt(math.sin(p+d)*math.sin(p-th-b)/(math.cos(d+ps+th)*math.cos(b-ps)))
    den = math.cos(th)*math.cos(ps)**2*math.cos(d+ps+th)*(1+root)**2
    return num/den
K = kae(30, 0.1); chk("EX98-13 K_AE", K, 0.39655, 1e-3); Ed = 0.5*19*25*K; chk("EX98-13 Ed", Ed, 94.18, 1e-3)
Ka = (1-math.sin(math.radians(30)))/(1+math.sin(math.radians(30))); chk("EX98-13 static", 0.5*19*25*Ka, 79.17, 1e-3); chk("EX98-13 dE", Ed-0.5*19*25*Ka, 15.02, 2e-2)
sv = 6*18 - 0 ; sv = 1*18 + 5*19; u = 5*9.81; spr = sv-u
chk("EX98-14 sv0", sv, 113); chk("EX98-14 s'v0", spr, 63.95, 1e-3)
csr = 0.65*0.9/g*sv/spr; chk("EX98-14 CSR", csr, 0.1054, 2e-3)
def crr(N): return 1/(34-N)+N/135+50/(10*N+45)**2-1/200
chk("EX98-14 CRR14", crr(14), 0.1502, 2e-3); chk("EX98-14 FS", crr(14)/csr, 1.425, 5e-3); chk("EX98-14 CRR10", crr(10), 0.1132, 3e-3); chk("EX98-14 FS10", crr(10)/csr, 1.074, 5e-3)
sd12 = sd(0.6,1.5,0.05,0.30,2.0,1.5,1.2); chk("EX98-15 Sd", sd12, 0.375, 1e-3); Fb15 = sd12*2000/g; chk("EX98-15 Fb", Fb15, 76.45, 2e-3); chk("EX98-15 Mb", Fb15*2*40/3, 2038.7, 2e-3)
chk("EX98-16 dE", se(0.6,1.5,0.05,0.30,2.0,1.0)*(1/(2*math.pi))**2, 0.01710, 2e-3); chk("EX98-16 q short", 3.5*math.sqrt(2/3), 2.858, 1e-3)
chk("EX98-17", 0.7*math.hypot(40,30), 35)
aS = 0.6*1.5/g; Sa = aS*(3*2/(1+0)-0.5); chk("EX98-18 Sa", Sa, 0.5046, 2e-3); chk("EX98-18 Fa/W", Sa*1.5/2, 0.378, 2e-3)

print("\n== B. code transcriptions vs correct formula ==")
# B1 design_spectrum_sd (used for base shear): Se*gI/q floored at 0.2*ag (agR, no gI)
def code_sd(se_, gI, q, ag): return max(se_*gI/max(q,1.0), 0.2*ag)
for T in (0.02, 0.05):
    print(f"T={T}: code Sd={code_sd(se(ag,S,TB,TC,TD,T),1.0,3.9,ag):.4f} vs EN Eq3.13 Sd={sd(ag,S,TB,TC,TD,3.9,T):.4f}")
gI=1.4; Tlong=3.0
print(f"B1b lower bound at T=3 s, gI=1.4: code floor={0.2*ag:.3f}, EN floor beta*ag(design)={0.2*ag*gI:.3f}, code Sd={code_sd(se(ag,S,TB,TC,TD,Tlong),gI,3.9,ag):.4f}, EN Sd={sd(ag*gI,S,TB,TC,TD,3.9,Tlong):.4f}")
# B2 Rayleigh with normalised displacement shape
for n, mass in [(4, [400e3]*4), (10, [400e3]*10), (2, [400e3]*2)]:
    zs = [i+1 for i in range(n)]; d = [zi/n for zi in zs]
    num = sum(mi*di*di for mi, di in zip(mass, d)); den = sum(mi*g*di for mi, di in zip(mass, d))
    print(f"B2 code Rayleigh T1 for n={n} equal masses = {2*math.pi*math.sqrt(num/den):.3f} s (independent of any stiffness)")
# B3 phi Table 4.2 (cat E storage)
psi2 = {"A":0.3,"B":0.3,"C":0.6,"D":0.6,"E":0.8,"F":0.6}
for c in "BDE":
    phi_code = 0.8 if c in "BDE" else None
    phi_en = 0.8 if c in "ABC" else 1.0
    print(f"B3 cat {c}: code psi_E={phi_code*psi2[c]:.3f}, EN psi_E={phi_en*psi2[c]:.3f}")
# B4 Mononobe-Okabe code
def kae_code(phi_deg, kh):
    p = math.radians(phi_deg); th = math.atan(kh)
    br = 1 + math.sqrt(math.sin(p)*math.sin(p-th)/math.cos(th))
    return math.cos(p-th)**2/(math.cos(th)*br*br)
for kh in (0.1, 0.2, 0.3):
    print(f"B4 kh={kh}: code K_AE={kae_code(30,kh):.5f} correct={kae(30,kh):.5f} ratio={kae_code(30,kh)/kae(30,kh):.4f}")
print("B4b kh=0.7 (theta>phi): ", end="")
try: print(kae_code(30,0.7))
except ValueError as e: print("sqrt of negative ->", e, "(Rust f64::sqrt returns NaN)")
# B5 theta with delta-scaled forces subtracted from f_b
Fraw = [Fb*zi/sum(z) for zi in z]
for dlt in (1.0, 1.3, 1.6, 1.8):
    forces = [f*dlt for f in Fraw]; sh = Fb; vs = []
    for f in forces: vs.append(sh); sh = max(sh-f, 0)
    print(f"B5 delta={dlt}: code V_i per storey = {[round(v,1) for v in vs]} (true {[round(v,1) for v in V]})")
# B6 q_max dual DCH
print("B6 dual DCH: 4.5*1.2=%.2f (wall-equivalent), 4.5*1.3=%.2f (frame-equivalent); code 5.2" % (4.5*1.2, 4.5*1.3))
# B7 envelope p-delta
for th_ in (0.15, 0.25, 0.31): print(f"B7 theta={th_}: code amplification {1/(1-th_) if th_<=0.3 else None}")
# B8 foundation 'bearing pressure'
Vb=486.78e3; A=25.0
print(f"B8 code p_Ed = Vb/A = {Vb/A:.0f} Pa from a horizontal shear of {Vb:.0f} N over {A} m2 (a shear stress, no N, M)")
# B9 p_Ed assessments with gamma_el=0
print("B9 r_d = Rk/(cf*max(gamma_el,1e-9)) with gamma_el=0, Rk=1e6, cf=1.35 ->", 1e6/(1.35*1e-9))
# B10 NA-DE table
print("B10 NA-DE B-T/C-T: code (S=1.25/1.50) vs spec (1.00/1.25); at T in plateau Se ratio =", 1.25/1.0, 1.5/1.25)
print("\nALL SPEC EXAMPLES REPRODUCED" if ok_all else "\nSOME SPEC EXAMPLES DIFFER")
