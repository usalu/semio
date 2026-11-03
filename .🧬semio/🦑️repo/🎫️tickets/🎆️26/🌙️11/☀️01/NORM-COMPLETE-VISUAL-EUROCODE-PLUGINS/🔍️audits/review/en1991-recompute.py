import math

rho = 1.25
def qb(v): return 0.5 * rho * v * v

print("== wind: NA-DE profiles")
WZ = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}
def na_inland(q0, z):
    z = max(z, 1.0)
    f = 1.5 if z <= 7 else 1.7 * (z / 10) ** 0.37 if z <= 50 else 2.1 * (z / 10) ** 0.24
    return f * q0
def na_coast(q0, z):
    z = max(z, 1.0)
    f = 1.8 if z <= 4 else 2.3 * (z / 10) ** 0.27 if z <= 50 else 2.6 * (z / 10) ** 0.19
    return f * q0
SIMPL_IN = {1: [0.50, 0.65, 0.75], 2: [0.65, 0.80, 0.90], 3: [0.80, 0.95, 1.10], 4: [0.95, 1.15, 1.30]}
SIMPL_CO_CODE = {2: [0.85, 1.00, 1.10], 3: [1.05, 1.20, 1.30], 4: [1.25, 1.40, 1.50]}
for zone in (2, 3, 4):
    q0 = qb(WZ[zone]) / 1000
    for h, band in ((10, 0), (18, 1), (25, 2)):
        pi = na_inland(q0, h)
        pc = na_coast(q0, h)
        print(f"WZ{zone} h={h}: inland profile {pi:.3f} simplified {SIMPL_IN[zone][band]:.2f} | coast profile {pc:.3f} simplified(code) {SIMPL_CO_CODE[zone][band]:.2f}")
print("coast WZ4 18-25 m band: code 1.50, ndp-public-verification 1.55 (printed value still to verify); profile at 25 m =", round(na_coast(qb(30) / 1000, 25), 3))

print("== wind: EN profile vs orography (NA-DE profile multiplied linearly by c_o in code)")
def prof(z0, vb, z, co=1.0, kI=1.0):
    kr = 0.19 * (z0 / 0.05) ** 0.07
    zmin = {0.003: 1, 0.01: 1, 0.05: 2, 0.3: 5, 1.0: 10}[z0]
    zz = max(z, zmin)
    cr = kr * math.log(zz / z0)
    vm = cr * co * vb
    iv = kI / (co * math.log(zz / z0))
    return (1 + 7 * iv) * 0.5 * rho * vm * vm / 1000, iv
q_flat, _ = prof(0.05, 25, 10)
q_oro, iv = prof(0.05, 25, 10, co=1.3)
print("EN cat II z=10 qp flat", round(q_flat, 4), "with c_o=1.3 (EN, c_o inside I_v and v_m):", round(q_oro, 4), "ratio", round(q_oro / q_flat, 4), "; linear c_o=1.3 would give", 1.3)

print("== wind: pitched roof cpe interpolation 45-60 deg zone F (code table, theta=0)")
rows = {45: ((0.0, 0.0), (0.7, 0.7)), 60: ((0.7, 0.7), None)}
a = 52.5
t = (a - 45) / 15
primary = tuple(rows[45][0][i] + (rows[60][0][i] - rows[45][0][i]) * t for i in range(2))
print("code returns only the primary pair at 52.5 deg:", primary, "; EN printed Table 7.4a has +0.7 at 45, 60 and 75 deg (F), so +0.7 must be a load case at 52.5 deg")

print("== wind: dominant opening c_pi")
for ratio in (1.0, 2.0, 2.5, 2.9, 3.0):
    code = 0.9 * -1.2 if ratio >= 3 else 0.75 * -1.2
    interp = None if ratio < 2 else (0.75 + (0.90 - 0.75) * (min(ratio, 3) - 2)) * -1.2
    print(f"area ratio {ratio}: code c_pi {code:.3f}, EN 7.2.9(5) interpolated {interp}")

print("== wind: Annex B structural factor (code vs spec E14-12 vs standard)")
def annexB(h, b, z0, vb, delta, n1=None, spec_variant=False):
    zmin = {0.003: 1, 0.01: 1, 0.05: 2, 0.3: 5, 1.0: 10}[z0]
    zs = max(0.6 * h, zmin)
    alpha = 0.67 + 0.05 * math.log(z0)
    L = 300 * (zs / 200) ** alpha
    B2 = 1 / (1 + 0.9 * ((b + h) / L) ** 0.63)
    kr = 0.19 * (z0 / 0.05) ** 0.07
    vm = kr * math.log(zs / z0) * vb
    n1 = n1 or 46 / h
    fL = n1 * L / vm
    SL = 6.8 * fL / (1 + 10.2 * fL) ** (5 / 3)
    Rh = lambda e: 1 if e <= 0 else 1 / e - (1 - math.exp(-2 * e)) / (2 * e * e)
    mult = SL if spec_variant else 1.0
    eh, eb = 4.6 * h / L * fL * mult, 4.6 * b / L * fL * mult
    R2 = math.pi ** 2 / (2 * delta) * SL * Rh(eh) * Rh(eb)
    nu = max(n1 * math.sqrt(R2 / (B2 + R2)), 0.08)
    r = math.sqrt(2 * math.log(nu * 600))
    kp = max(r + 0.6 / r, 3)
    iv = 1 / math.log(zs / z0)
    return zs, L, B2, n1, SL, R2, nu, kp, (1 + 2 * kp * iv * math.sqrt(B2 + R2)) / (1 + 7 * iv)
print("standard/code:", [round(x, 5) for x in annexB(40, 30, 0.3, 25, 0.1)])
print("spec E14-12  :", [round(x, 5) for x in annexB(40, 30, 0.3, 25, 0.1, spec_variant=True)], "<- spec multiplies eta by S_L; wrong")

print("== snow: NA-DE ground snow")
def sk(zone, A):
    t = ((max(A, 0) + 140) / 760) ** 2
    base = {"1": max(0.65, 0.19 + 0.91 * t), "2": max(0.85, 0.25 + 1.91 * t), "3": max(1.10, 0.31 + 2.91 * t)}[zone[0]]
    return base * (1.25 if zone.endswith("a") else 1)
print("zone2 A=300", sk("2", 300), "zone3 A=800", sk("3", 800), "zone1a A=50", sk("1a", 50), "zone3 A=1500 (no domain guard in code)", sk("3", 1500))
print("return period D.1 V=0.4 n=100 :", (1 - 0.4 * (0.45 + 0.78 * math.log(-math.log(1 - 0.01)))) / (1 + 2.59 * 0.4))
print("exceptional s_Ad = 2.3 sk(zone1,NGL,A=50) :", 2.3 * sk("1", 50))
print("overhang s=0.9 : d,k,se", 0.9 / 3, min(3 / 0.3, 0.3 * 3), min(3 / 0.3, 0.3 * 3) * 0.81 / 3)

print("== imposed: alpha_A/alpha_n")
psi0 = 0.7
print("EN alphaA A=40", 5 / 7 * psi0 + 10 / 40, "A=200 C/D floor", max(0.6, 5 / 7 * psi0 + 10 / 200), "alphaN n=5", (2 + 3 * psi0) / 5)
print("DE code alpha_A(C,A=40) = clamp(0.5+10/sqrt(A),0.7,1)=", min(1, max(0.7, 0.5 + 10 / math.sqrt(40))), "(spec says area reduction NOT permitted in NA-DE: 1.0)")
print("combined on a floor (code multiplies): A=40 n=5 cat C EN:", (5 / 7 * psi0 + 10 / 40) * ((2 + 3 * psi0) / 5))

print("== fire")
iso = lambda t: 20 + 345 * math.log10(8 * t + 1)
print("ISO", [round(iso(t), 1) for t in (15, 30, 60, 90, 120)])
def parametric(Af, At, Av, heq, b, qfd, tlim=1 / 3):
    O = Av * math.sqrt(heq) / At
    G = lambda o: (o / b / (0.04 / 1160)) ** 2
    qtd = qfd * Af / At
    tmax_v = 0.2e-3 * qtd / O
    fuel = tmax_v < tlim
    if fuel:
        Olim = 0.1e-3 * qtd / tlim
        Gh = G(Olim)
        if O > 0.04 and qtd < 75 and b < 1160:
            Gh *= 1 + (O - 0.04) / 0.04 * (qtd - 75) / 75 * (1160 - b) / 1160
        tmax = tlim
    else:
        Gh, tmax = G(O), tmax_v
    th = lambda ts: 20 + 1325 * (1 - 0.324 * math.exp(-0.2 * ts) - 0.204 * math.exp(-1.7 * ts) - 0.472 * math.exp(-19 * ts))
    return O, G(O), qtd, tmax, th(tmax * Gh), fuel
print("E12-04 O,Gamma,qtd,tmax,theta_max,fuel:", parametric(50, 2 * 50 + 4 * 7.07 * 3, 5, 2.0, 1160, 420 * .8 * 1.5))
print("alpha_c for parametric: EN 1991-1-2 3.1(6): 35 W/m2K; code alpha_c_for_curve returns 25 for FireCurve::Parametric")
print("q_fk office: Annex E Table E.4 80% fractile 511, mean 420; code uses mean for AnnexChoice::De -> q_fd", 420 * .8 * 1.5, "vs", 511 * .8 * 1.5)

print("== traffic")
w = 14.0
n = int(w // 3)
print("w=14 m: n1 =", n, "lane UDL entries produced by E2.02 =", min(n, 3), "-> lane 4 (2.5 kN/m2 x 3 m) missing; remaining width", w - 3 * n)
print("LM71 phi2/phi3 L=20:", 1.44 / (math.sqrt(20) - .2) + .82, 2.16 / (math.sqrt(20) - .2) + .73)
v, Lf = 160, 30
print("rail f:", 1 - (v - 120) / 1000 * (814 / v + 1.75) * (1 - math.sqrt(2.88 / Lf)), "; f for Lf=2 m (EN: f=1):", 1 - (v - 120) / 1000 * (814 / v + 1.75) * (1 - math.sqrt(2.88 / 2)))
print("footbridge qfk L=50:", 2 + 120 / 80, "braking Qlk L=40:", 0.6 * 600 + 0.1 * 27 * 40)

print("== cranes")
print("phi2 HC2 vh=0.25:", 1.10 + 0.34 * 0.25, "skew f(0.015):", 0.3 * (1 - math.exp(-250 * 0.015)))
print("test loads: practice (EN 13001/ISO 4310): static test 1.25 x SWL, dynamic test 1.10 x SWL; code E3.04 returns static 1.1 and dynamic 1.25")

print("== silo")
phi = 30
K = 1.1 * (1 - math.sin(math.radians(phi)))
mu = 0.4
d = 6.0
a = d / 4
zo = a / (K * mu)
pho = 9 * a / mu
for z in (2, 10, 20):
    print("Janssen z", z, pho * (1 - math.exp(-z / zo)))
print("K_u = 1.2 K_m =", 1.2 * K, "mu_l = mu/1.15 =", mu / 1.15, "-> p_hf,max (z=20) using extremes:", (9 * a / (mu / 1.15)) * (1 - math.exp(-20 / (a / (1.2 * K * mu / 1.15)))), "vs mean", pho * (1 - math.exp(-20 / zo)))

print("== thermal")
print("Te,min/max steel/comp/conc:", [(-24 + s, 37 + e) for s, e in ((-3, 16), (4, 4), (8, 2))])
print("E15-05 restrained bowing:", 210000 * 1e9 * 12e-6 * 18 / 1000 / 1e6)
