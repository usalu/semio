import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🔁️recap\🐍️.py")
src = ROOT.read_text(encoding="utf-8")


def sub(old, new, count=None):
    global src
    n = src.count(old)
    if n == 0 or (count is not None and n != count):
        sys.exit(f"MISMATCH {n} != {count}: {old[:70]!r}")
    src = src.replace(old, new)


def block(start, end, new):
    global src
    i = src.index(start)
    j = src.index(end, i) + len(end)
    src = src[:i] + new + src[j:]


CONST = r'''#region Example house
# One set of numbers for the whole chapter, taken from the running example (areas, U-values, volume,
# air change, heat capacity of air), so every bar and readout below is a product of drawn quantities.
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
SURFACE_SLUGS = ("wall", "roof", "windows", "floor")
_ENVELOPE = {name: (area, u) for name, area, u, _delta in _example.ENVELOPE}
_GROUND_DELTA = {name: delta for name, _a, _u, delta in _example.ENVELOPE}["Boden"]
_SURFACE_COLORS = (("Außenwand", LOSS_BLUE), ("Dach", semio_color("info")), ("Fenster", P_CYAN), ("Boden", P_TEAL))


def _surfaces_at(d_theta: float):
    """🧱 The four envelope surfaces at an indoor-outdoor difference; the floor always sees the ground at its own difference."""
    return [(n, _ENVELOPE[n][0], _ENVELOPE[n][1], _GROUND_DELTA if n == "Boden" else d_theta, c) for n, c in _SURFACE_COLORS]


def _eta_heat(gamma: float, a: float = 2.0) -> float:
    """🔋 Utilisation factor of the gains η = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_g / Q_l (DIN V 4108-6, 6.5)."""
    return a / (a + 1.0) if abs(gamma - 1.0) < 1e-9 else (1.0 - gamma ** a) / (1.0 - gamma ** (a + 1.0))


def _eta_cool(gamma: float, a: float = 2.0) -> float:
    """❄️ Utilisation factor of the losses η_c = (1 − γ⁻ᵃ) / (1 − γ⁻⁽ᵃ⁺¹⁾) with γ = Q_g / Q_l."""
    return a / (a + 1.0) if abs(gamma - 1.0) < 1e-9 else (1.0 - gamma ** -a) / (1.0 - gamma ** -(a + 1.0))


D_THETA_MONTH, D_THETA_JAN, D_THETA_JUL = 10.0, _example.D_THETA_WINTER_K, 6.0
D_THETA = D_THETA_MONTH
SURFACES = _surfaces_at(D_THETA_MONTH)
A_HUELLE = sum(a for _n, a, _u, _d, _c in SURFACES)
DU_WB = 0.05
V_NET, N_AIR, RHO_C = _example.V_NET_M3, _example.N_AIR_PER_H, _example.C_LUFT_WH_M3_K
T_MONTH = 744.0
PHI_SURFACES = [u * a * d for _n, a, u, d, _c in SURFACES]
PHI_WB = DU_WB * A_HUELLE * D_THETA
PHI_V = V_NET * N_AIR * RHO_C * D_THETA
BAR_UNIT = 3.0 / (sum(PHI_SURFACES) + PHI_WB + PHI_V)


def _loss_kwh(d_theta: float) -> float:
    """📉 Monthly losses Q_l = Q_T + Q_V in kWh."""
    phi = sum(u * a * d for _n, a, u, d, _c in _surfaces_at(d_theta)) + DU_WB * A_HUELLE * d_theta + V_NET * N_AIR * RHO_C * d_theta
    return phi * T_MONTH / 1000.0


A_WINDOW, G_VALUE, F_S = _ENVELOPE["Fenster"][0], 0.6, 0.7
I_S, I_S_JAN, I_S_JUL = 65.0, 26.0, 54.0
Q_I_SPEC, A_NET = 5.0, 60.0
Q_INTERN = Q_I_SPEC * A_NET * T_MONTH / 1000.0


def _solar_kwh(i_s: float) -> float:
    """☀️ Solar gains Q_S = A · I_s · g⊥ · F_S in kWh."""
    return A_WINDOW * G_VALUE * F_S * i_s


Q_SOLAR = _solar_kwh(I_S)
Q_L, Q_G = _loss_kwh(D_THETA_MONTH), Q_SOLAR + Q_INTERN
ETA_H = _eta_heat(Q_G / Q_L)
Q_USABLE = ETA_H * Q_G
Q_L_JAN, Q_G_JAN = _loss_kwh(D_THETA_JAN), _solar_kwh(I_S_JAN) + Q_INTERN
ETA_JAN = _eta_heat(Q_G_JAN / Q_L_JAN)
Q_USABLE_JAN = ETA_JAN * Q_G_JAN
Q_L_JUL, Q_G_JUL = _loss_kwh(D_THETA_JUL), _solar_kwh(I_S_JUL) + Q_INTERN
ETA_C = _eta_cool(Q_G_JUL / Q_L_JUL)
#endregion'''

block("#region Example house", "Q_QUELLE_JUL = A_WINDOW * G_VALUE * F_S * I_S_JUL + Q_INTERN\n#endregion", CONST)

head, sep, tail = src.partition("class Beat4_WinterUndSommer")
for a, b in (("Q_SENKE_JUL", "Q_L_JUL"), ("Q_QUELLE_JUL", "Q_G_JUL"), ("Q_SENKE", "Q_L"), ("Q_QUELLE", "Q_G")):
    head = re.sub(rf"\b{a}\b", b, head)
    tail = re.sub(rf"\b{a}\b", b, tail)
tail = re.sub(r"\bQ_L\b", "Q_L_JAN", tail)
tail = re.sub(r"\bQ_USABLE\b", "Q_USABLE_JAN", tail)
tail = re.sub(r"\bETA_H\b", "ETA_JAN", tail)
tail = re.sub(r"\bETA_K\b", "ETA_C", tail)
tail = re.sub(r"\bD_THETA\b", "D_THETA_JAN", tail)
tail = re.sub(r"\bI_S\b", "I_S_JAN", tail)
src = head + sep + tail
head2, sep2, tail2 = src.partition("class Beat1_Bilanzgrenze")
head2 = head2.replace("D_THETA_JAN, D_THETA_JUL = 10.0", "D_THETA_JAN, D_THETA_JUL = 10.0")
src = head2 + sep2 + tail2.replace("ETA_K", "ETA_C")

sub(r"Q_{\mathrm{Senke}}", r"Q_{l}")
sub(r"Q_{\mathrm{Quelle}}", r"Q_{g}")
sub(r"Q_{{\mathrm{{Quelle}}}}", r"Q_{{g}}")
sub(r"\eta_{h}", r"\eta")
sub(r"\eta_{{h}}", r"\eta")
sub(r"\eta_{k}", r"\eta_{c}")
sub(r'r"Q_{I}"', r'r"Q_{i}"')
sub(r"Q_{{I}}", r"Q_{{i}}")
sub(r'r"q_{I}"', r'r"q_{i}"')
sub(r"q_{{I}}", r"q_{{i}}")
sub(r'r"Q_{K}"', r'r"Q_{c}"')
sub(r"I_{S}", r"I_{s}")
sub(r"I_{{S}}", r"I_{{s}}")
sub(r"A_{W}", r"A")
sub(r"A_{{W}}", r"A")
sub(r"\;[\mathrm{kWh/a}]", r"\quad Q\ \mathrm{in}\ \frac{\mathrm{kWh}}{\mathrm{a}}")
sub(r"(None, r\"\;[\mathrm{kWh}]\", P_PRIMARY),\n        ])\n        self.add(_veil(row_s))", "XX", 0) if False else None
sub(r"\;[\mathrm{kWh}]", r"\quad Q\ \mathrm{in\ kWh}")
sub(r"\mathrm{{kWh/m^{{2}}}}", r"\frac{{\mathrm{{kWh}}}}{{\mathrm{{m^{{2}}}}}}", 2)
sub(r"\mathrm{{W/(m^{{2}}K)}}", r"\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}\,K}}}}", 2)
sub(r"\mathrm{{W/m^{{2}}}}", r"\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}}}}}", 1)
sub(r"\mathrm{{Wh/m^{{3}}K}}", r"\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}\,K}}}}", 1)
sub(r"\rho c = ", r"\rho\,c_{{p}} = ", 1)
sub(r'("rc", r"\rho\,c", P_GREEN)', r'("rc", r"\rho\,c_{p}", P_GREEN)', 1)
sub(r"\Phi_{{V}}", r"\dot{{Q}}_{{V}}", 1)
sub('_gauge("g", P_ORANGE', r'_gauge(r"g_{\perp}", P_ORANGE', 1)
sub(r'("g", r"g", P_ORANGE)', r'("g", r"g_{\perp}", P_ORANGE)', 1)
sub(r'rf"g = {label_number(g_t.get_value(), 2)}"', r'rf"g_{{\perp}} = {label_number(g_t.get_value(), 2)}"', 1)
sub(r'''(None, rf"\approx {label_number(ETA_H, 1)}\cdot {label_number(Q_G)} \approx {label_number(Q_USABLE)}\;\mathrm{{kWh}}", P_WHITE),''',
    r'''(None, rf"\approx {label_number(ETA_H, 2)}\cdot {label_number(Q_G)}\,\mathrm{{kWh}} \approx {label_number(Q_USABLE)}\,\mathrm{{kWh}}", P_WHITE),''', 1)
sub('("f", r"F_{S}", P_TEAL), (None, r"\quad Q\ \mathrm{in\ kWh}", P_PRIMARY),',
    '("f", r"F_{S}", P_TEAL), (None, r"\\quad (F_{C}=F_{F}=F_{W}=1)\\quad Q\\ \\mathrm{in\\ kWh}", P_PRIMARY),', 1)
sub('("f", r"F_{S}", P_TEAL), (None, r"\\quad Q\\ \\mathrm{in\\ kWh}", P_PRIMARY),',
    '("f", r"F_{S}", P_TEAL), (None, r"\\quad (F_{C}=F_{F}=F_{W}=1)\\quad Q\\ \\mathrm{in\\ kWh}", P_PRIMARY),') if False else None
sub('th["out"]["level"].animate.set_value(0.25)', 'th["out"]["level"].animate.set_value(0.5)', 1)
sub('stack = stacked_bar(', 'stack = stacked_bar(', 1)
sub('"Boden": P_TEAL', '"Boden": P_TEAL', 0) if False else None

sub('din = _din_ref("DIN V 18599-1")', 'din = _din_ref("DIN V 4108-6:2003-06, 3.2 · 5.5")', 1)
sub('din = _din_ref("DIN V 18599-2")\n        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)\n\n        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))\n        self.play(FadeIn(caption), run_time=0.3)\n\n        H = _home(np.array([-3.7',
    'din = _din_ref("DIN V 4108-6:2003-06, 6.1 · 6.2")\n        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)\n\n        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))\n        self.play(FadeIn(caption), run_time=0.3)\n\n        H = _home(np.array([-3.7', 1)
sub('din = _din_ref("DIN V 18599-2")', 'din = _din_ref("DIN V 4108-6:2003-06, 6.3 · 6.4 · 6.5")', 1)
sub('din = _din_ref("DIN V 18599-2 · VDI 2078")', 'din = _din_ref("DIN V 4108-6:2003-06, 6.5 · DIN V 18599-2:2018-09, 5.5.3")', 1)
sub('chip(tr("Winter: losses > gains", "Winter: Verluste > Gewinne"), LOSS_BLUE', 'chip(tr("Winter: losses > usable gains", "Winter: Verluste > nutzbare Gewinne"), LOSS_BLUE', 1)
sub('chip(tr("Summer: gains > losses", "Sommer: Gewinne > Verluste"), P_RED', 'chip(tr("Summer: gains > usable losses", "Sommer: Gewinne > nutzbare Verluste"), P_RED', 1)
sub('body_text("außen", font_size=LABEL_FONT_SIZE, color=LOSS_BLUE)', 'body_text(tr("outside", "außen"), font_size=LABEL_FONT_SIZE, color=LOSS_BLUE)', 1)
ROOT.write_text(src, encoding="utf-8")
print("ok")
