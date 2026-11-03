"""Independent recomputation of the EN 1993 / EN 1994 spec worked examples and of the defects in review-en1993-en1994.md (stdlib only, python3 -I)."""
import math

PI = math.pi
E, G = 210000.0, 81000.0


def chi(lam, a, l0=0.2, b=1.0):
    phi = 0.5 * (1 + a * (lam - l0) + b * lam * lam)
    return min(1.0, 1 / (phi + math.sqrt(max(phi * phi - b * lam * lam, 0))))


def sec(title):
    print(f"\n== {title}")


sec("E01 HEB240 S355 flexural buckling")
A, Iy, Iz, fy = 10600, 11260e4, 3923e4, 355
for name, I, al in (("z", Iz, 0.49), ("y", Iy, 0.34)):
    ncr = PI**2 * E * I / 4000**2
    lam = math.sqrt(A * fy / ncr)
    x = chi(lam, al)
    print(name, f"Ncr={ncr/1e3:.1f} lam={lam:.4f} chi={x:.4f} NbRd EN={x*A*fy/1e3:.1f} DE={x*A*fy/1.1/1e3:.1f}")

sec("E02 IPE300 S235 Mcr / chi_LT")
Iz2, It, Iw, Wpl, L = 603.8e4, 20.12e4, 125900e6, 628.4e3, 5000
c1 = 1.127
mcr = c1 * PI**2 * E * Iz2 / L**2 * math.sqrt(Iw / Iz2 + L**2 * G * It / (PI**2 * E * Iz2))
lam = math.sqrt(Wpl * 235 / mcr)
x = chi(lam, 0.34)
print(f"Mcr={mcr/1e6:.2f} kNm lam_LT={lam:.4f} chi(6.56)={x:.4f} Mb={x*Wpl*235/1e6:.2f}/{x*Wpl*235/1.1/1e6:.2f}")
x2 = min(chi(lam, 0.34, 0.4, 0.75), 1 / lam**2)
kc = 0.94
f = min(1, 1 - 0.5 * (1 - kc) * (1 - 2 * (lam - 0.8) ** 2))
print(f"6.57 chi={x2:.4f} f={f:.4f} chi_mod={min(1,x2/f):.4f} Mb={min(1,x2/f)*Wpl*235/1e6:.2f}")

sec("E03/E04 classification + effective width")
eps = math.sqrt(235 / 355)
print("web 1000x8 psi=-1 class3 limit 62 eps (1-psi) sqrt(-psi) =", 62 * eps * 2, " c/t=125")
def kinternal(psi):
    return 23.9 if psi == -1 else (7.81 - 6.29 * psi + 9.78 * psi**2 if -1 < psi < 0 else 4)
k = kinternal(-1)
lp = 1000 / 8 / (28.4 * eps * math.sqrt(k))
rho = (lp - 0.055 * (3 - 1)) / lp**2
bc = 1000 / 2
beff = rho * bc
print(f"lam_p={lp:.4f} rho={rho:.4f} beff={beff:.1f} be1={0.4*beff:.1f} be2={0.6*beff:.1f}")

sec("DEFECT 1-5 web be2 placement (i_section_bending) 300x20 flanges, web 1000x8, S355")
def web_I(correct):
    h, b, tw, tf = 1040.0, 300.0, 8.0, 20.0
    hw = 1000.0
    fl = 1.0
    cf = (b - tw) / 2
    lpf = cf / tf / (28.4 * eps * math.sqrt(0.43))
    rf = 1.0 if lpf <= 0.748 else (lpf - 0.188) / lpf**2
    bcw = hw / 2
    be1, be2 = 0.4 * beff, 0.6 * beff
    items = []
    zt = h / 2 - tf / 2
    items.append(((tw + 2 * rf * cf) * tf, zt, (tw + 2 * rf * cf) * tf**3 / 12))
    items.append((b * tf, -zt, b * tf**3 / 12))
    items.append((be1 * tw, bcw - be1 / 2, tw * be1**3 / 12))
    z2 = (be2 / 2) if correct else (bcw - be1 - be2 / 2)
    items.append((be2 * tw, z2, tw * be2**3 / 12))
    items.append((bcw * tw, -bcw / 2, tw * bcw**3 / 12))
    a = sum(i[0] for i in items)
    en = sum(i[0] * i[1] for i in items) / a
    I = sum(i[2] + i[0] * (i[1] - en) ** 2 for i in items)
    zmax = max(h / 2 - en, h / 2 + en)
    return a, en, I / zmax
for tag, c in (("code (be2 directly under be1)", False), ("EN Table 4.1 (be2 at stress-zero point)", True)):
    a, en, w = web_I(c)
    print(f"{tag}: Aeff={a:.0f} eN={en:.2f} Weff={w/1e3:.0f} cm3")

sec("E05 N + biaxial bending")
Av = 5381; b_, tf_ = 150, 10.7
Npl = 5381 * 235 / 1e3; n = 300 / Npl; a = (5381 - 2 * b_ * tf_) / 5381
Mpl_y, Mpl_z = 147.67, 29.42
MNy = min(Mpl_y * (1 - n) / (1 - 0.5 * a), Mpl_y)
beta = max(5 * n, 1)
print(f"Npl={Npl:.1f} n={n:.4f} a={a:.4f} MNy={MNy:.2f} util={(80/MNy)**2+(10/Mpl_z)**beta:.4f}")

sec("E06 Annex B (HEB240 S355, NEd=800, My=100 psi=0)")
for gm1 in (1.0, 1.1):
    NRk, MyRk = 3763.0, 373.8
    ny = 800 / (0.8807 * NRk / gm1); nz = 800 / (0.6243 * NRk / gm1)
    kyy = min(0.6 * (1 + (0.5079 - 0.2) * ny), 0.6 * (1 + 0.8 * ny))
    d = 0.6 - 0.25
    kzy = max(1 - 0.1 * 0.8605 * nz / d, 1 - 0.1 * nz / d)
    print(f"gM1={gm1}: kyy={kyy:.4f} kzy={kzy:.4f} eta61={ny+kyy*100/(1.0*MyRk/gm1):.4f}")
    # eta62 with kzz class 1/2 I
    Mz = 0
    print("  eta62 (Mz=0) =", round(nz + kzy * 100 / (MyRk / gm1), 4))

sec("E07 shear buckling plate girder")
hw, tw_, a_ = 1000, 8, 2000
kt = 5.34 + 4 / (a_ / hw) ** 2
lw = hw / (37.4 * tw_ * eps * math.sqrt(kt))
cw = 1.37 / (0.7 + lw)
V = cw * 355 * hw * tw_ / math.sqrt(3)
print(f"k_tau={kt:.2f} lam_w={lw:.4f} chi_w={cw:.4f} Vbw={V/1e3:.1f} /1.1 = {V/1.1/1e3:.1f}")

sec("E08/E09 bolts and welds")
print("Fv,Rd", 0.6 * 800 * 245 / 1.25 / 1e3, "Ft,Rd", 0.9 * 800 * 245 / 1.25 / 1e3, "Fb,Rd", 2.5 * (40 / 66) * 490 * 20 * 12 / 1.25 / 1e3)
print("fvw,d", 490 / (math.sqrt(3) * 0.9 * 1.25))

sec("E10 T-stub (spec) vs code Lb* direction")
leff, tf2, m, ew = 210.0, 15.0, 40.0, 0.0
Mpl = 0.25 * leff * tf2**2 * 355 / 1e3
Ft3 = 2 * 141.12
Ft1 = 4 * Mpl / m
n_ = min(40, 1.25 * m)
Ft2 = (2 * Mpl + n_ * Ft3) / (m + n_)
Lbs = 8.8 * m**3 * 245 * 2 / (leff * tf2**3)
print(f"Mpl={Mpl:.1f} FT1={Ft1:.1f} FT2={Ft2:.2f} FT3={Ft3:.2f} Lb*={Lbs:.1f}; Lb=60 -> EN: prying (Lb<=Lb*) FT,Rd={min(Ft1,Ft2,Ft3):.2f}")
print(f"code: prying = Lb > Lb* = {60 > Lbs} -> no prying -> FT,Rd = 2Mpl1/m = {2*Mpl/m:.2f} (ft_2=inf)")
print("thick plate tf=40 Lb=100: Lb*=", round(8.8 * m**3 * 245 * 2 / (leff * 40**3), 1))
Mpl40 = 0.25 * leff * 40**2 * 355 / 1e3
print(f"  EN no-prying FT,1-2 = {2*Mpl40/m:.1f} ; code (prying flag true) FT1 = 4Mpl/m = {4*Mpl40/m:.1f} (x2 unconservative), FT3={Ft3}")
k5 = 0.9 * leff * 15**3 / 40**3; k10 = 1.6 * 245 / 60
print(f"k5 EN Table 6.11 (0.9) = {k5:.3f} (spec 0.85 -> {0.85*leff*15**3/40**3:.3f}); k10 per bolt-row spec = {k10:.3f}, code n_b*... = {2*k10:.3f}")

sec("E11 fatigue")
dc = 71
dD = dc * (2 / 5) ** (1 / 3); dL = dD * (5 / 100) ** 0.2
print(f"dD={dD:.2f} dL={dL:.2f} N(60)={2e6*(71/60)**3:.3e} N_R(gMf 1.15)={2e6*(71/1.15/60)**3:.3e} D={1e6/(2e6*(71/1.15/60)**3):.4f}")
print("gamma_Mf Table 3.1 (1993-1-9): damage tolerant low/high 1.00/1.15; safe life low/high 1.15/1.35. code: DamageTolerant=1.0, SafeLife=1.15, LowConsequence=1.35 (mislabelled)")

sec("E12 fire")
am = 1160.05 / 5381 * 1000
print("Am/V", am, "ksh", 0.9 * 167.26 / am)
mu0 = 0.6
print("theta_cr", 39.19 * math.log(1 / (0.9674 * mu0**3.833) - 1) + 482)

sec("E13/E14/E15")
print("alpha_cr", (50 / 1200) * (6000 / 12), (50 / 1200) * (6000 / 40), "amp", 1 / (1 - 1 / 6.25))
ah, am_ = 2 / math.sqrt(6), math.sqrt(0.5 * (1 + 1 / 2))
print("phi", 1 / 200 * ah * am_)
print("Nu,Rd", 0.9 * (200 - 44) * 12 * 360 / 1.25 / 1e3)
Avz = 5381 - 2 * 150 * 10.7 + (7.1 + 30) * 10.7
Vpl = Avz * 235 / math.sqrt(3) / 1e3
rho = (2 * 200 / Vpl - 1) ** 2
Aw = 278.6 * 7.1
print(f"Av={Avz:.2f} Vpl={Vpl:.2f} rho={rho:.5f} MyV={(628.4e3 - rho*Aw**2/(4*7.1))*235/1e6:.2f}")
print("code shear_reduced_m_rd: M*(1-rho) =", 147.67 * (1 - rho), " (EN 146.96); at V=0.9Vpl: code", 147.67 * (1 - (2 * .9 - 1) ** 2), "EN", (628.4e3 - (2 * .9 - 1) ** 2 * Aw**2 / (4 * 7.1)) * 235 / 1e6, " signed V=-0.9Vpl: code", 147.67)

sec("DEFECT Annex A eps_y at N=0 (IEEE)")
eps_y = math.inf
s = math.sqrt(eps_y) * 0.9
print("s/(1+s) with inf:", "nan" if math.isinf(s) else s / (1 + s), "(inf/inf = NaN in Rust f64)")

sec("DEFECT class 3 signed sum")
N, My, A_, W = 500e3, -60e6, 8000.0, 400e3
print("signed", abs(N / A_ + My / W) / 355, " envelope", (abs(N / A_) + abs(My / W)) / 355)

sec("EN 1994")
sec_h, b, tf, tw, Aa = 300.0, 150.0, 10.7, 7.1, 5381.0
fcc = 0.85 * 30 / 1.5
Npl_a = Aa * 355
x = Npl_a / (fcc * 2000)
print(f"E1-02 x={x:.2f} Mpl,Rd={Npl_a*(150+100-x/2)/1e6:.2f} kNm")
print("E1-03 etamin Le=8:", 1 - (355 / 355) * (0.75 - 0.03 * 8))
d, fu, hsc = 19.0, 450.0, 100.0
p1 = 0.8 * fu * PI * d * d / 4 / 1.25 / 1e3
p2 = 0.29 * 1.0 * d * d * math.sqrt(30 * 33000) / 1.25 / 1e3
print(f"E1-04 PRd1={p1:.3f} PRd2={p2:.3f}")
kt = 0.7 * (100 / 50) * (95 / 50 - 1)
print(f"kt={kt:.2f}")
print("E1-05 vEd", 81656 / 150 / 200, "As/s", 81656 / 150 / 200 * 100 / (500 / 1.15) * 1000, "mm2/m")
print("E1-06 Av", Avz, "Vpl", Avz * 355 / math.sqrt(3) / 1e3, "rho@400", (2 * 400 / (Avz * 355 / math.sqrt(3) / 1e3) - 1) ** 2)
EIeff = 210000 * 121.583e6 + 0.6 * 33000 * 418.690e6
Ncr = PI**2 * EIeff / 4000**2 / 1e3
lam = math.sqrt(5676.9 / Ncr)
print(f"E1-07 EIeff={EIeff:.4e} Ncr={Ncr:.0f} lam={lam:.4f} chi={chi(lam,0.21):.4f} NRd={chi(lam,0.21)*4951.5:.1f}")
Ap, fyp, ep, e_, h, hc = 1100.0, 350.0, 20.0, 25.0, 150.0, 100.0
Ncf = Ap * fyp / 1e3
for nc in (Ncf, 0.6 * Ncf):
    xx = nc * 1e3 / (0.85 * 20 * 1000)
    z = h - xx / 2 - ep + (ep - e_) * nc / Ncf
    mpr = 0 if nc >= Ncf else min(1.25 * 4.0 * (1 - nc / Ncf), 4.0)
    print(f"E1-09 Nc={nc:.1f} x={xx:.2f} z={z:.2f} M={nc*z/1e3+mpr:.2f} kNm/m")
print("E1-09 Vl,Rd", 125 / 1.25 * (200 * 1100 / (1000 * 1000) + 0.04) * 1000 / 1000, "kN/m;", "Vv,Rd", 400 * 125 * (0.25 * 2.0 / 1.5) * 1.475 * (1.2 + 40 * 0.02) / 1000)
n0 = 210000 / 33000
Iy300 = 8356e4
def trans(n):
    ac = 2000 * 100 / n
    ya = 100 + 150.0
    A = Aa + ac
    yna = (Aa * ya + ac * 50) / A
    I = Iy300 + Aa * (ya - yna) ** 2 + 2000 * 100**3 / 12 / n + ac * (50 - yna) ** 2
    return I, yna, ac
nL = n0 * (1 + 1.1 * 2.5)
ns = n0 * (1 + 0.55 * 2.5)
I0, _, _ = trans(n0); IL, _, _ = trans(nL); Is, ynas, acs = trans(ns)
dl = 5 * 8000**4 / (384 * 210000) * (6 / IL + 4 / I0)
kap = 0.3e-3 * acs * (ynas - 50) / Is
print(f"E1-10 I0={I0/1e6:.2f} IL={IL/1e6:.2f} delta={dl:.2f} shr={kap*8000**2/8:.2f} (zc={ynas-50:.2f})")
kc_ = min(1, 1 / (1 + 100 / 500) + 0.3)
print("E1-11 As,min", 0.9 * kc_ * 0.8 * 2.9 * 200000 / 500)
print("E1-12", 20 + 345 * math.log10(8 * 60 + 1), 20 + 345 * math.log10(8 * 30 + 1))

sec("DEFECT EN1994 two-span continuous coefficients")
print("UDL span M: exact 9/128 =", 9 / 128, " code 1/16 =", 1 / 16, " -> sagging underestimated", round((9/128)/(1/16)-1, 3))
print("point load at midspan of both spans: M_span exact 5/32 =", 5/32, " code 1/8 =", 1/8, "; M_support exact 3/32 =", 3/32, " code 1/8; V inner support exact 11/16 =", 11/16, " code 1.25/2 =", 0.625)

sec("DEFECT EN1994 chi_LT alpha (6.3.2.2 Table 6.4: rolled a/b, welded c/d) for IPE300 h/b=2")
lamlt = 1.0
for al in (0.21, 0.34):
    print("alpha", al, "chi", round(chi(lamlt, al), 4))

sec("DEFECT EN1994 stud transverse spacing rule")
print("EN 1994-1-1 6.6.5.7(4): transverse >= 2.5d solid slab, >= 4d otherwise; code: solid->4d, else 2.5d")

sec("DEFECT EN1994 crack control: alpha_st / Delta sigma_s (cat. 7.4.3) IPE300, beff 2000 x hc 100, As = 2000 mm2 at 40 mm below slab top, C30")
Ea, Es_en, Es_code = 210000.0, 210000.0, 200000.0
def cracked_I(Es):
    ar = 2000 * Es / Ea
    ya = 100 + 150.0
    A = Aa + ar
    yna = (Aa * ya + ar * 40) / A
    I = Iy300 + Aa * (ya - yna) ** 2 + ar * (40 - yna) ** 2
    return A, I
n0v = 210000 / 33000
A_unc, I_unc = 0, 0
ac = 2000 * 100 / n0v
ya = 250.0
A_unc = Aa + ac
yn = (Aa * ya + ac * 50) / A_unc
I_unc = Iy300 + Aa * (ya - yn) ** 2 + 2000 * 100**3 / 12 / n0v + ac * (50 - yn) ** 2
Acr, Icr = cracked_I(Es_en)
rho_s = 2000 / (2000 * 100)
fctm = 2.9
a_code = A_unc * I_unc / (Aa * Iy300)
a_en = Acr * Icr / (Aa * Iy300)
print(f"alpha_st code (uncracked incl. concrete) = {a_code:.2f}, EN (cracked, concrete in tension ignored) = {a_en:.2f}")
print(f"delta sigma_s code = {0.4*fctm/(a_code*rho_s):.1f} MPa, EN = {0.4*fctm/(a_en*rho_s):.1f} MPa")
print("E_s: EN 1994-1-1 3.2(2) -> 210 GPa; code E_S = 200 GPa (catalogue constants) ->", Es_code / Ea)
