"""Independent numpy re-implementation of the EN 1994 column fire heat-conduction model (quadrant finite volumes, ISO 834, harmonic-mean conductances) used to cross-check the Rust catalogue module `column_fire`.

Prints cell temperatures and the fire resistance of the reference column at 60 minutes; the values are hard-coded in the Rust tests.
"""
import math
import numpy as np

SIGMA, ALPHA_C, EPS = 5.67e-8, 25.0, 0.7
RHO_C, RHO_S = 2300.0, 7850.0


def iso(t):
    return 20 + 345 * math.log10(8 * t / 60 + 1)


def lam_c(th):
    t = np.clip(th, 20, 1200) / 100
    return 1.36 - 0.136 * t + 0.0057 * t * t


def cp_c(th):
    return np.where(th <= 100, 900.0, np.where(th <= 200, 900 + (th - 100), np.where(th <= 400, 1000 + (th - 200) / 2, 1100.0)))


def lam_s(th):
    return np.where(th < 800, 54 - 3.33e-2 * th, 27.3)


def cp_s(th):
    return np.where(th < 600, 425 + 0.773 * th - 1.69e-3 * th ** 2 + 2.22e-6 * th ** 3,
                    np.where(th < 735, 666 + 13002 / (738 - th), np.where(th < 900, 545 + 17820 / (th - 731), 650.0)))


def run(g, t_end):
    h, b, tw, tf, bc, hc = g
    wx, hy = max(bc, b) / 2, max(hc, h) / 2
    nx, ny = int(np.ceil(wx / 0.01)), int(np.ceil(hy / 0.01))
    dx, dy = wx / nx, hy / ny
    xc = (np.arange(nx) + 0.5) * dx
    yc = (np.arange(ny) + 0.5) * dy
    X, Y = np.meshgrid(xc, yc)
    lim = h / 2 - max(tf, dy / 2)
    steel = ((Y >= lim) & (X <= b / 2) & (Y <= h / 2)) | ((X <= tw / 2) & (Y < lim))
    th = np.full((ny, nx), 20.0)
    dt = 1.0
    t = 0.0
    while t < t_end - 1e-9:
        step = min(dt, t_end - t)
        gas = iso(t + step / 2)
        lam = np.where(steel, lam_s(th), lam_c(th))
        cap = np.where(steel, RHO_S * cp_s(th), RHO_C * cp_c(th))
        q = np.zeros_like(th)

        def link(a, bb, face, d):
            k = 2 * a * bb / (a + bb)
            return k * face / d

        # x direction
        kx = link(lam[:, :-1], lam[:, 1:], dy, dx) * (th[:, 1:] - th[:, :-1])
        q[:, :-1] += kx
        q[:, 1:] -= kx
        ky = link(lam[:-1, :], lam[1:, :], dx, dy) * (th[1:, :] - th[:-1, :])
        q[:-1, :] += ky
        q[1:, :] -= ky
        flux = lambda T: ALPHA_C * (gas - T) + EPS * SIGMA * ((gas + 273) ** 4 - (T + 273) ** 4)
        q[:, -1] += flux(th[:, -1]) * dy
        q[-1, :] += flux(th[-1, :]) * dx
        th = th + q * step / (cap * dx * dy)
        t += step
    return th, steel, (nx, ny, dx, dy)


if __name__ == '__main__':
    g = (0.2, 0.2, 0.0085, 0.013, 0.4, 0.4)
    th, steel, (nx, ny, dx, dy) = run(g, 3600.0)
    for (i, j) in [(0, 0), (4, 4), (0, 9), (19, 0), (19, 19), (0, 19)]:
        print(f'cell({i},{j}) steel={bool(steel[j, i])} theta={th[j, i]:.4f}')
    print('max', th.max(), 'steel_mean', th[steel].mean())
