import math

for h, to, ti in ((2.5, 18, 26), (4.4, 18, 26), (4.4, 0, 20)):
    r = lambda c: 101325 / (287.05 * (273.15 + c))
    d = r(to) - r(ti)
    dp = h * 9.81 * d
    v = math.sqrt(2 * dp / 1.2)
    q = 0.6 * 0.1 * v * 3600
    print(h, to, ti, round(r(to), 4), round(r(ti), 4), round(d, 4), round(dp, 3), round(v, 3), round(q, 1), round(0.6 * 0.1 * v, 4))
