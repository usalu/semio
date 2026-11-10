"""Independent physics shared by the DIN/ISO/VDI cross-checks: IAPWS saturation pressure, ISO 13788 curves, closed-form helpers. No repo oracle is imported."""
import math

N = [0.11670521452767e4, -0.72421316703206e6, -0.17073846940092e2, 0.12020824702470e5, -0.32325550322333e7, 0.14915108613530e2, -0.48232657361591e4, 0.40511340542057e6, -0.23855557567849, 0.65017534844798e3]


def psat_iapws(t_c):
    """IAPWS-IF97 region 4 over water, IAPWS R14-08 sublimation over ice below 0 C; Pa."""
    t = t_c + 273.15
    if t_c >= 0.0:
        th = t + N[8] / (t - N[9])
        a = th * th + N[0] * th + N[1]
        b = N[2] * th * th + N[3] * th + N[4]
        c = N[5] * th * th + N[6] * th + N[7]
        return 1e6 * (2 * c / (-b + math.sqrt(b * b - 4 * a * c))) ** 4
    q = t / 273.16
    s = -0.212144006e2 * q ** 0.333333333e-2 + 0.273203819e2 * q ** 1.20666667 - 0.610598130e1 * q ** 1.70333333
    return 611.657 * math.exp(s / q)


def psat_iso13788(t_c):
    return 610.5 * math.exp(17.269 * t_c / (237.3 + t_c)) if t_c >= 0 else 610.5 * math.exp(21.875 * t_c / (265.5 + t_c))


def psat_magnus_rust(t_c):
    return 611.2 * math.exp(17.67 * t_c / (t_c + 243.5))


def psat_iso7730(t_c):
    return 1000.0 * math.exp(16.6536 - 4030.183 / (t_c + 235.0))


assert abs(psat_iapws(100.0) - 101418.0) < 20.0
assert abs(psat_iapws(0.01) - 611.657) < 0.05
