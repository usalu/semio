"""🔬 Scans bell-crank pivot, lever arm, link length and bucket link pin of the Z-bar for the widest transmission angles over the working poses; the chosen values are then written by hand into r3_wheel_loader_kin.py."""
import itertools, math
import r3_wheel_loader_kin as k

NEED = [(-43.4, 0), (-43.4, 40), (-36.5, 45), (5, 50), (45, 55), (45, -45), (5, -20)]
best = []
for fu, fv, r2, ln, kx, ky in itertools.product([1.8, 1.9, 2.0, 2.1], [0.4, 0.5, 0.6, 0.7], [0.5, 0.6, 0.7, 0.8], [0.5, 0.6, 0.7, 0.8, 0.9], [0.0, 0.06, 0.12], [0.40, 0.46, 0.52]):
    worst = 180
    ok = True
    for th, be in NEED:
        arm = lambda p: k.add(k.A, k.rot(p, th))
        B = arm((k.L, 0)); K = k.add(B, k.rot((kx, ky), be)); F = arm((fu, fv))
        d = k.sub(K, F); dist = k.norm(d)
        if dist > r2 + ln - 0.02 or dist < abs(r2 - ln) + 0.02:
            ok = False; break
        a = (r2 * r2 - ln * ln + dist * dist) / (2 * dist); h = math.sqrt(max(0, r2 * r2 - a * a))
        ex = (d[0] / dist, d[1] / dist)
        J = (F[0] + a * ex[0] - h * ex[1], F[1] + a * ex[1] + h * ex[0])
        J2 = (F[0] + a * ex[0] + h * ex[1], F[1] + a * ex[1] - h * ex[0])
        Jl = k.rot(k.sub(J, k.A), -th); J2l = k.rot(k.sub(J2, k.A), -th)
        J = J if Jl[1] < J2l[1] else J2
        tj = abs((k.ang(k.sub(K, J)) - k.ang(k.sub(F, J)) + 180) % 360 - 180)
        tk = abs((k.ang(k.sub(J, K)) - k.ang(k.sub(B, K)) + 180) % 360 - 180)
        worst = min(worst, tj, 180 - tj, tk, 180 - tk)
    if ok:
        best.append((worst, fu, fv, r2, ln, kx, ky))
best.sort(reverse=True)
for b in best[:25]:
    print("worst %.1f F (%.2f %.2f) r2 %.2f link %.2f K (%.2f %.2f)" % b)
